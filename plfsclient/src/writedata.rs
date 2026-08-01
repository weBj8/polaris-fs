pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn writev(
        __fd: ::core::ffi::c_int,
        __iovec: *const iovec,
        __count: ::core::ffi::c_int,
    ) -> ssize_t;
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
    unsafe fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
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
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn __sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    unsafe fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn pthread_sigmask(
        __how: ::core::ffi::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_detach(__th: pthread_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn mycrc32(crc: uint32_t, block: *const ::core::ffi::c_void, leng: uint32_t)
    -> uint32_t;
    unsafe fn queue_new(size: uint32_t) -> *mut ::core::ffi::c_void;
    unsafe fn queue_delete(que: *mut ::core::ffi::c_void);
    unsafe fn queue_close(que: *mut ::core::ffi::c_void);
    unsafe fn queue_put(
        que: *mut ::core::ffi::c_void,
        id: uint32_t,
        op: uint32_t,
        data: *mut uint8_t,
        leng: uint32_t,
    );
    unsafe fn queue_get(
        que: *mut ::core::ffi::c_void,
        id: *mut uint32_t,
        op: *mut uint32_t,
        data: *mut *mut uint8_t,
        leng: *mut uint32_t,
    );
    unsafe fn univmakestrip(strip: *mut ::core::ffi::c_char, ip: uint32_t);
    unsafe fn tcpsocket() -> ::core::ffi::c_int;
    unsafe fn tcpnodelay(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnumbind(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpnumtoconnect(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
        msecto: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpgetpeer(
        sock: ::core::ffi::c_int,
        ip: *mut uint32_t,
        port: *mut uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpgetmyaddr(
        sock: ::core::ffi::c_int,
        ip: *mut uint32_t,
        port: *mut uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn conncache_insert(ip: uint32_t, port: uint16_t, fd: ::core::ffi::c_int);
    unsafe fn conncache_get(ip: uint32_t, port: uint16_t) -> ::core::ffi::c_int;
    unsafe fn csorder_sort(
        chain: *mut cspri,
        csdataver: uint8_t,
        csdata: *const uint8_t,
        csdatasize: uint32_t,
        writeflag: uint8_t,
    ) -> uint32_t;
    unsafe fn csdb_writeinc(ip: uint32_t, port: uint16_t);
    unsafe fn csdb_writedec(ip: uint32_t, port: uint16_t);
    unsafe fn delay_run(
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        udata: *mut ::core::ffi::c_void,
        useconds: uint64_t,
    );
    unsafe fn fs_getsrcip() -> uint32_t;
    unsafe fn fs_writechunk(
        inode: uint32_t,
        indx: uint32_t,
        chunkopflags: uint8_t,
        csdataver: *mut uint8_t,
        length: *mut uint64_t,
        chunkid: *mut uint64_t,
        version: *mut uint32_t,
        csdata: *mut *const uint8_t,
        csdatasize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_writeend(
        chunkid: uint64_t,
        inode: uint32_t,
        indx: uint32_t,
        length: uint64_t,
        chunkopflags: uint8_t,
        offset: uint32_t,
        size: uint32_t,
    ) -> uint8_t;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn read_inode_clear_cache(inode: uint32_t, offset: uint64_t, leng: uint64_t);
    unsafe fn chunkrwlock_wlock(inode: uint32_t, indx: uint32_t);
    unsafe fn chunkrwlock_wunlock(inode: uint32_t, indx: uint32_t);
    unsafe fn chunksdatacache_insert(
        inode: uint32_t,
        chindx: uint32_t,
        chunkid: uint64_t,
        version: uint32_t,
        csdataver: uint8_t,
        csdata: *const uint8_t,
        csdatasize: uint32_t,
    );
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type ssize_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __glibc_reserved: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
    pub __unused_initialized_1: ::core::ffi::c_uint,
    pub __unused_initialized_2: ::core::ffi::c_uint,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cspri {
    pub ip: uint32_t,
    pub port: uint16_t,
    pub version: uint32_t,
    pub labelmask: uint32_t,
    pub priority: uint32_t,
}
pub type cspri = _cspri;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cblock_s {
    pub data: [uint8_t; 65536],
    pub pos: uint16_t,
    pub writeid: uint32_t,
    pub from: uint32_t,
    pub to: uint32_t,
    pub next: *mut cblock_s,
    pub prev: *mut cblock_s,
}
pub type cblock = cblock_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inodedata_s {
    pub inode: uint32_t,
    pub maxfleng: uint64_t,
    pub cacheblockcount: uint32_t,
    pub status: ::core::ffi::c_int,
    pub flushwaiting: uint16_t,
    pub writewaiting: uint16_t,
    pub chunkwaiting: uint16_t,
    pub lcnt: uint16_t,
    pub chunkscnt: uint16_t,
    pub chunks: *mut chunkdata,
    pub chunkstail: *mut *mut chunkdata,
    pub chunksnext: *mut chunkdata,
    pub flushcond: pthread_cond_t,
    pub writecond: pthread_cond_t,
    pub chunkcond: pthread_cond_t,
    pub lock: pthread_mutex_t,
    pub next: *mut inodedata_s,
}
pub type chunkdata = chunkdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunkdata_s {
    pub chindx: uint32_t,
    pub trycnt: uint16_t,
    pub waitingworker: uint8_t,
    pub chunkready: uint8_t,
    pub unbreakable: uint8_t,
    pub continueop: uint8_t,
    pub superuser: uint8_t,
    pub wakeup_fd: ::core::ffi::c_int,
    pub datachainhead: *mut cblock,
    pub datachaintail: *mut cblock,
    pub parent: *mut inodedata_s,
    pub next: *mut chunkdata_s,
    pub prev: *mut *mut chunkdata_s,
}
pub type inodedata = inodedata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker_s {
    pub thread_id: pthread_t,
}
pub type worker = worker_s;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const EDQUOT: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ENXIO: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const EBADF: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ENOSPC: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const EROFS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const __SC_THREAD_STACK_MIN_VALUE: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const MFSCHUNKMASK: ::core::ffi::c_int = 0x3ffffff as ::core::ffi::c_int;
pub const MFSCHUNKBITS: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const MFSBLOCKSIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const MFSBLOCKMASK: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;
pub const MFSBLOCKBITS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_ERROR_CHUNKLOST: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MFS_ERROR_INDEXTOOBIG: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MFS_ERROR_LOCKED: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const MFS_ERROR_NOCHUNKSERVERS: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const MFS_ERROR_NOCHUNK: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const MFS_ERROR_NOTDONE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFS_ERROR_NOSPACE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const MFS_ERROR_IO: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const MFS_ERROR_EROFS: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const MFS_ERROR_QUOTA: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const MFS_ERROR_CSNOTPRESENT: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const MFS_ERROR_EAGAIN: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const MFS_ERROR_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHUNKOPFLAG_CONTINUEOP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CHUNKOPFLAG_CANUSERESERVESPACE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLTOCS_WRITE: ::core::ffi::c_int = PROTO_BASE + 210 as ::core::ffi::c_int;
pub const CSTOCL_WRITE_STATUS: ::core::ffi::c_int = PROTO_BASE + 211 as ::core::ffi::c_int;
pub const CLTOCS_WRITE_DATA: ::core::ffi::c_int = PROTO_BASE + 212 as ::core::ffi::c_int;
pub const CLTOCS_WRITE_FINISH: ::core::ffi::c_int = PROTO_BASE + 213 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PROTO_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn get8bit(mut ptr: *mut *const uint8_t) -> uint8_t {
    unsafe {
        let mut t8: uint8_t = 0;
        t8 = *(*ptr).offset(0 as isize);
        *ptr = (*ptr).offset(1);
        return t8;
    }
}
#[inline]
unsafe extern "C" fn mfsstrerr(mut status: uint8_t) -> *const ::core::ffi::c_char {
    unsafe {
        static mut errtab: [*const ::core::ffi::c_char; 65] = [
            b"OK\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation not permitted\0".as_ptr() as *const ::core::ffi::c_char,
            b"Not a directory\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such file or directory\0".as_ptr() as *const ::core::ffi::c_char,
            b"Permission denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"File exists\0".as_ptr() as *const ::core::ffi::c_char,
            b"Invalid argument\0".as_ptr() as *const ::core::ffi::c_char,
            b"Directory not empty\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunk lost\0".as_ptr() as *const ::core::ffi::c_char,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            b"Index too big\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunk locked\0".as_ptr() as *const ::core::ffi::c_char,
            b"No chunk servers\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such chunk\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunk is busy\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect register BLOB\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation not completed\0".as_ptr() as *const ::core::ffi::c_char,
            b"File not opened\0".as_ptr() as *const ::core::ffi::c_char,
            b"Write not started\0".as_ptr() as *const ::core::ffi::c_char,
            b"Wrong chunk version\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunk already exists\0".as_ptr() as *const ::core::ffi::c_char,
            b"No space left\0".as_ptr() as *const ::core::ffi::c_char,
            b"IO error\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect block number\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect size\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect offset\0".as_ptr() as *const ::core::ffi::c_char,
            b"Can't connect\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect chunk id\0".as_ptr() as *const ::core::ffi::c_char,
            b"Disconnected\0".as_ptr() as *const ::core::ffi::c_char,
            b"CRC error\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation delayed\0".as_ptr() as *const ::core::ffi::c_char,
            b"Can't create path\0".as_ptr() as *const ::core::ffi::c_char,
            b"Data mismatch\0".as_ptr() as *const ::core::ffi::c_char,
            b"Read-only file system\0".as_ptr() as *const ::core::ffi::c_char,
            b"Quota exceeded\0".as_ptr() as *const ::core::ffi::c_char,
            b"Bad session id\0".as_ptr() as *const ::core::ffi::c_char,
            b"Password is needed\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect password\0".as_ptr() as *const ::core::ffi::c_char,
            b"Attribute not found\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation not supported\0".as_ptr() as *const ::core::ffi::c_char,
            b"Result too large\0".as_ptr() as *const ::core::ffi::c_char,
            b"Entity not found\0".as_ptr() as *const ::core::ffi::c_char,
            b"Entity is active\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunkserver not present\0".as_ptr() as *const ::core::ffi::c_char,
            b"Waiting on lock\0".as_ptr() as *const ::core::ffi::c_char,
            b"Resource temporarily unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            b"Interrupted system call\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation canceled\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such file or directory (not cacheable)\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation not permitted (mfs admin only)\0".as_ptr() as *const ::core::ffi::c_char,
            b"Class name already in use\0".as_ptr() as *const ::core::ffi::c_char,
            b"Maximum number of classes reached\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such class\0".as_ptr() as *const ::core::ffi::c_char,
            b"Class in use\0".as_ptr() as *const ::core::ffi::c_char,
            b"One of MFS instance components is too old to perform this operation\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"Pattern already defined\0".as_ptr() as *const ::core::ffi::c_char,
            b"Maximum number of patterns reached\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such pattern\0".as_ptr() as *const ::core::ffi::c_char,
            b"File name too long\0".as_ptr() as *const ::core::ffi::c_char,
            b"Too many links\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation timed out\0".as_ptr() as *const ::core::ffi::c_char,
            b"Bad file descriptor\0".as_ptr() as *const ::core::ffi::c_char,
            b"File too large\0".as_ptr() as *const ::core::ffi::c_char,
            b"Is a directory\0".as_ptr() as *const ::core::ffi::c_char,
            b"Unknown MFS error\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        if status as ::core::ffi::c_int > MFS_ERROR_MAX {
            status = MFS_ERROR_MAX as uint8_t;
        }
        return errtab[status as usize];
    }
}
#[inline]
unsafe extern "C" fn close_pipe(mut handles: *mut ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if close(*handles.offset(0 as isize)) < 0 as ::core::ffi::c_int {
            res = -1 as ::core::ffi::c_int;
        }
        if close(*handles.offset(1 as isize)) < 0 as ::core::ffi::c_int {
            res = -1 as ::core::ffi::c_int;
        }
        return res;
    }
}
pub const NEXT_BLOCK_DELAY: ::core::ffi::c_double = 0.05f64;
pub const CHUNKSERVER_ACTIVITY_TIMEOUT: ::core::ffi::c_double = 5.0f64;
pub const WORKER_IDLE_TIMEOUT: ::core::ffi::c_double = 0.1f64;
pub const WORKER_BUSY_LAST_SEND_TIMEOUT: ::core::ffi::c_double = 5.0f64;
pub const WORKER_BUSY_WAIT_FOR_STATUS: ::core::ffi::c_double = 15.0f64;
pub const WORKER_BUSY_NOJOBS_INCREASE_TIMEOUT: ::core::ffi::c_double = 60.0f64;
pub const WORKER_NOP_INTERVAL: ::core::ffi::c_double = 1.0f64;
pub const MAX_SIM_CHUNKS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SUSTAIN_WORKERS: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const HEAVYLOAD_WORKERS: ::core::ffi::c_int = 150 as ::core::ffi::c_int;
pub const MAX_WORKERS: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
pub const IDHASHSIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
static mut fcblock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __glibc_reserved: 0,
        __list: __pthread_list_t {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut fcbcond: pthread_cond_t = pthread_cond_t {
    __data: __pthread_cond_s {
        __wseq: __atomic_wide_counter { __value64: 0 },
        __g1_start: __atomic_wide_counter { __value64: 0 },
        __g_size: [0; 2],
        __g1_orig_size: 0,
        __wrefs: 0,
        __g_signals: [0; 2],
        __unused_initialized_1: 0,
        __unused_initialized_2: 0,
    },
};
static mut fcbwaiting: uint16_t = 0;
static mut cacheblocks: *mut cblock = ::core::ptr::null_mut::<cblock>();
static mut freecblockshead: *mut cblock = ::core::ptr::null_mut::<cblock>();
static mut freecacheblocks: uint32_t = 0;
static mut cacheblockcount: uint32_t = 0;
static mut optimeout: ::core::ffi::c_double = 0.;
static mut maxretries: uint32_t = 0;
static mut minlogretry: uint32_t = 0;
static mut erroronlostchunk: uint8_t = 0;
static mut erroronnospace: uint8_t = 0;
static mut idhash: *mut *mut inodedata = ::core::ptr::null_mut::<*mut inodedata>();
static mut hashlock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __glibc_reserved: 0,
        __list: __pthread_list_t {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut workerslock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __glibc_reserved: 0,
        __list: __pthread_list_t {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut workers_avail: uint32_t = 0;
static mut workers_total: uint32_t = 0;
static mut worker_term_waiting: uint32_t = 0;
static mut worker_term_cond: pthread_cond_t = pthread_cond_t {
    __data: __pthread_cond_s {
        __wseq: __atomic_wide_counter { __value64: 0 },
        __g1_start: __atomic_wide_counter { __value64: 0 },
        __g_size: [0; 2],
        __g1_orig_size: 0,
        __wrefs: 0,
        __g_signals: [0; 2],
        __unused_initialized_1: 0,
        __unused_initialized_2: 0,
    },
};
static mut worker_thattr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
static mut jqueue: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn write_increase_total_bytes(mut v: uint32_t) {
    unsafe {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut total_bytes_sent,
            v as uint64_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_get_total_bytes() -> uint64_t {
    unsafe {
        let mut v: uint64_t = 0;
        v = ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut total_bytes_sent,
            0 as uint64_t,
        );
        return v;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_cb_release(mut ind: *mut inodedata, mut cb: *mut cblock) {
    unsafe {
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut fcblock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    275 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    275 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    275 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    275 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    275 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    275 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        (*cb).next = freecblockshead as *mut cblock_s;
        freecblockshead = cb;
        freecacheblocks = freecacheblocks.wrapping_add(1);
        (*ind).cacheblockcount = (*ind).cacheblockcount.wrapping_sub(1);
        if fcbwaiting != 0 {
            let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_cond_signal(&raw mut fcbcond);
            if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        281 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&fcbcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        281 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&fcbcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        281 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&fcbcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        281 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&fcbcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                } else {
                    let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        281 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&fcbcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        281 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&fcbcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut fcblock);
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    286 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    286 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    286 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    286 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    286 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    286 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_cb_acquire(mut ind: *mut inodedata) -> *mut cblock {
    unsafe {
        let mut ret: *mut cblock = ::core::ptr::null_mut::<cblock>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut fcblock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    291 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    291 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    291 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    291 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    291 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    291 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        fcbwaiting = fcbwaiting.wrapping_add(1);
        while freecblockshead.is_null() {
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_cond_wait(&raw mut fcbcond, &raw mut fcblock);
            if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        295 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&fcbcond,&fcblock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        295 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&fcbcond,&fcblock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        295 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&fcbcond,&fcblock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        295 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&fcbcond,&fcblock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                } else {
                    let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        295 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&fcbcond,&fcblock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        295 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&fcbcond,&fcblock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
        }
        fcbwaiting = fcbwaiting.wrapping_sub(1);
        ret = freecblockshead;
        freecblockshead = (*ret).next as *mut cblock;
        (*ret).pos = 0 as uint16_t;
        (*ret).writeid = 0 as uint32_t;
        (*ret).from = 0 as uint32_t;
        (*ret).to = 0 as uint32_t;
        (*ret).next = ::core::ptr::null_mut::<cblock_s>();
        (*ret).prev = ::core::ptr::null_mut::<cblock_s>();
        freecacheblocks = freecacheblocks.wrapping_sub(1);
        (*ind).cacheblockcount = (*ind).cacheblockcount.wrapping_add(1);
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut fcblock);
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    312 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    312 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    312 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    312 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    312 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    312 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_cache_almost_full() -> uint8_t {
    unsafe {
        let mut r: uint8_t = 0;
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut fcblock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        r = (if freecacheblocks < cacheblockcount.wrapping_div(3 as uint32_t) {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut fcblock);
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        return r;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_find_inodedata(mut inode: uint32_t) -> *mut inodedata {
    unsafe {
        let mut indh: uint32_t = inode
            .wrapping_mul(0xb239fb71 as uint32_t)
            .wrapping_rem(IDHASHSIZE as uint32_t);
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut hashlock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        ind = *idhash.offset(indh as isize);
        while !ind.is_null() {
            if (*ind).inode == inode {
                (*ind).lcnt = (*ind).lcnt.wrapping_add(1);
                let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut hashlock);
                if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            333 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            333 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                    } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_0);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            333 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            333 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                    } else {
                        let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_0);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            333 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_err_0,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            333 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_err_0,
                        );
                    }
                    abort();
                }
                return ind;
            }
            ind = (*ind).next as *mut inodedata;
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut hashlock);
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    337 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    337 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    337 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    337 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    337 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    337 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        return ::core::ptr::null_mut::<inodedata>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_get_inodedata(
    mut inode: uint32_t,
    mut fleng: uint64_t,
) -> *mut inodedata {
    unsafe {
        let mut indh: uint32_t = inode
            .wrapping_mul(0xb239fb71 as uint32_t)
            .wrapping_rem(IDHASHSIZE as uint32_t);
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut hashlock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    346 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    346 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    346 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    346 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    346 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    346 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        ind = *idhash.offset(indh as isize);
        while !ind.is_null() {
            if (*ind).inode == inode {
                (*ind).lcnt = (*ind).lcnt.wrapping_add(1);
                let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut hashlock);
                if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            350 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            350 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                    } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_0);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            350 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            350 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                    } else {
                        let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_0);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            350 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_err_0,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            350 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_err_0,
                        );
                    }
                    abort();
                }
                return ind;
            }
            ind = (*ind).next as *mut inodedata;
        }
        ind = malloc(::core::mem::size_of::<inodedata>()) as *mut inodedata;
        if ind.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                356 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ind\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                356 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ind\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if ind
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut inodedata
        {
            let mut _mfs_errorstring_3: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                356 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ind\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                356 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ind\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_3,
            );
            abort();
        }
        (*ind).inode = inode;
        (*ind).cacheblockcount = 0 as uint32_t;
        (*ind).maxfleng = fleng;
        (*ind).status = 0 as ::core::ffi::c_int;
        (*ind).chunkscnt = 0 as uint16_t;
        (*ind).chunks = ::core::ptr::null_mut::<chunkdata>();
        (*ind).chunksnext = ::core::ptr::null_mut::<chunkdata>();
        (*ind).chunkstail = &raw mut (*ind).chunks;
        (*ind).flushwaiting = 0 as uint16_t;
        (*ind).chunkwaiting = 0 as uint16_t;
        (*ind).writewaiting = 0 as uint16_t;
        (*ind).lcnt = 1 as uint16_t;
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_cond_init(
            &raw mut (*ind).flushcond,
            ::core::ptr::null::<pthread_condattr_t>(),
        );
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    370 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->flushcond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    370 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->flushcond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_4,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_5: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    370 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->flushcond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    370 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->flushcond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_5,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    370 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->flushcond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    370 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->flushcond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_cond_init(
            &raw mut (*ind).writecond,
            ::core::ptr::null::<pthread_condattr_t>(),
        );
        if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_6: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    371 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->writecond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    371 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->writecond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_6,
                );
            } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_7: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    371 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->writecond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    371 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->writecond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_7,
                );
            } else {
                let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    371 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->writecond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    371 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->writecond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_3: ::core::ffi::c_int = pthread_cond_init(
            &raw mut (*ind).chunkcond,
            ::core::ptr::null::<pthread_condattr_t>(),
        );
        if _mfs_assert_ret_3 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_3 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_8: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    372 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->chunkcond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_8,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    372 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->chunkcond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_8,
                );
            } else if _mfs_assert_ret_3 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_9: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_3);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    372 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->chunkcond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_9,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    372 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->chunkcond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_9,
                );
            } else {
                let mut _mfs_errorstring_err_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_3: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_3);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    372 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->chunkcond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    372 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ind->chunkcond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_4: ::core::ffi::c_int = pthread_mutex_init(
            &raw mut (*ind).lock,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
        if _mfs_assert_ret_4 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_4 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_10: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    373 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(ind->lock),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_10,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    373 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(ind->lock),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_10,
                );
            } else if _mfs_assert_ret_4 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_11: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_4);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    373 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(ind->lock),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_11,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    373 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(ind->lock),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_11,
                );
            } else {
                let mut _mfs_errorstring_err_4: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_4: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_4);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    373 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(ind->lock),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    373 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(ind->lock),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
            }
            abort();
        }
        (*ind).next = *idhash.offset(indh as isize) as *mut inodedata_s;
        *idhash.offset(indh as isize) = ind;
        let mut _mfs_assert_ret_5: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut hashlock);
        if _mfs_assert_ret_5 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_5 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_12: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    376 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_12,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    376 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_12,
                );
            } else if _mfs_assert_ret_5 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_13: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_5);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    376 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_13,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    376 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_13,
                );
            } else {
                let mut _mfs_errorstring_err_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_5: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_5);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    376 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_err_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    376 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_err_5,
                );
            }
            abort();
        }
        return ind;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_free_inodedata(mut fid: *mut inodedata) {
    unsafe {
        let mut indh: uint32_t = (*fid)
            .inode
            .wrapping_mul(0xb239fb71 as uint32_t)
            .wrapping_rem(IDHASHSIZE as uint32_t);
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        let mut indp: *mut *mut inodedata = ::core::ptr::null_mut::<*mut inodedata>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut hashlock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    383 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    383 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    383 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    383 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    383 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    383 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        indp = idhash.offset(indh as isize);
        loop {
            ind = *indp;
            if ind.is_null() {
                break;
            }
            if ind == fid {
                (*ind).lcnt = (*ind).lcnt.wrapping_sub(1);
                if (*ind).lcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    *indp = (*ind).next as *mut inodedata;
                    let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                        pthread_mutex_lock(&raw mut (*ind).lock);
                    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                            && *__errno_location() != 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                390 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_1,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                390 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_1,
                            );
                        } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                            && *__errno_location() == 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_0);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                390 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_2,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                390 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_2,
                            );
                        } else {
                            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_0);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                390 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_err_0,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                390 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_err_0,
                            );
                        }
                        abort();
                    }
                    if (*ind).chunkscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && (*ind).flushwaiting as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && (*ind).writewaiting as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                    } else {
                        fprintf(
                            stderr,
                            b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            391 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"ind->chunkscnt==0 && ind->flushwaiting==0 && ind->writewaiting==0\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"inode structure not clean\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            391 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"ind->chunkscnt==0 && ind->flushwaiting==0 && ind->writewaiting==0\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"inode structure not clean\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    };
                    let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                        pthread_mutex_unlock(&raw mut (*ind).lock);
                    if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
                        if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                            && *__errno_location() != 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                392 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_1,
                                *__errno_location(),
                                _mfs_errorstring_3,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                392 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_1,
                                *__errno_location(),
                                _mfs_errorstring_3,
                            );
                        } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                            && *__errno_location() == 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_4: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_1);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                392 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_1,
                                _mfs_errorstring_4,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                392 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_1,
                                _mfs_errorstring_4,
                            );
                        } else {
                            let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_1);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                392 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_1,
                                _mfs_errorstring_ret_1,
                                *__errno_location(),
                                _mfs_errorstring_err_1,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                392 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_1,
                                _mfs_errorstring_ret_1,
                                *__errno_location(),
                                _mfs_errorstring_err_1,
                            );
                        }
                        abort();
                    }
                    let mut _mfs_assert_ret_2: ::core::ffi::c_int =
                        pthread_cond_destroy(&raw mut (*ind).flushcond);
                    if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
                        if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                            && *__errno_location() != 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                393 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->flushcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_2,
                                *__errno_location(),
                                _mfs_errorstring_5,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                393 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->flushcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_2,
                                *__errno_location(),
                                _mfs_errorstring_5,
                            );
                        } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                            && *__errno_location() == 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_6: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_2);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                393 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->flushcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_2,
                                _mfs_errorstring_6,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                393 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->flushcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_2,
                                _mfs_errorstring_6,
                            );
                        } else {
                            let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_2);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                393 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->flushcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_2,
                                _mfs_errorstring_ret_2,
                                *__errno_location(),
                                _mfs_errorstring_err_2,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                393 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->flushcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_2,
                                _mfs_errorstring_ret_2,
                                *__errno_location(),
                                _mfs_errorstring_err_2,
                            );
                        }
                        abort();
                    }
                    let mut _mfs_assert_ret_3: ::core::ffi::c_int =
                        pthread_cond_destroy(&raw mut (*ind).writecond);
                    if _mfs_assert_ret_3 != 0 as ::core::ffi::c_int {
                        if _mfs_assert_ret_3 < 0 as ::core::ffi::c_int
                            && *__errno_location() != 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_7: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                394 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->writecond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_3,
                                *__errno_location(),
                                _mfs_errorstring_7,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                394 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->writecond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_3,
                                *__errno_location(),
                                _mfs_errorstring_7,
                            );
                        } else if _mfs_assert_ret_3 > 0 as ::core::ffi::c_int
                            && *__errno_location() == 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_8: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_3);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                394 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->writecond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_3,
                                _mfs_errorstring_8,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                394 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->writecond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_3,
                                _mfs_errorstring_8,
                            );
                        } else {
                            let mut _mfs_errorstring_err_3: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            let mut _mfs_errorstring_ret_3: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_3);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                394 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->writecond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_3,
                                _mfs_errorstring_ret_3,
                                *__errno_location(),
                                _mfs_errorstring_err_3,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                394 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->writecond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_3,
                                _mfs_errorstring_ret_3,
                                *__errno_location(),
                                _mfs_errorstring_err_3,
                            );
                        }
                        abort();
                    }
                    let mut _mfs_assert_ret_4: ::core::ffi::c_int =
                        pthread_cond_destroy(&raw mut (*ind).chunkcond);
                    if _mfs_assert_ret_4 != 0 as ::core::ffi::c_int {
                        if _mfs_assert_ret_4 < 0 as ::core::ffi::c_int
                            && *__errno_location() != 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_9: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                395 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->chunkcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_4,
                                *__errno_location(),
                                _mfs_errorstring_9,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                395 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->chunkcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_4,
                                *__errno_location(),
                                _mfs_errorstring_9,
                            );
                        } else if _mfs_assert_ret_4 > 0 as ::core::ffi::c_int
                            && *__errno_location() == 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_10: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_4);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                395 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->chunkcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_4,
                                _mfs_errorstring_10,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                395 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->chunkcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_4,
                                _mfs_errorstring_10,
                            );
                        } else {
                            let mut _mfs_errorstring_err_4: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            let mut _mfs_errorstring_ret_4: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_4);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                395 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->chunkcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_4,
                                _mfs_errorstring_ret_4,
                                *__errno_location(),
                                _mfs_errorstring_err_4,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                395 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ind->chunkcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_4,
                                _mfs_errorstring_ret_4,
                                *__errno_location(),
                                _mfs_errorstring_err_4,
                            );
                        }
                        abort();
                    }
                    let mut _mfs_assert_ret_5: ::core::ffi::c_int =
                        pthread_mutex_destroy(&raw mut (*ind).lock);
                    if _mfs_assert_ret_5 != 0 as ::core::ffi::c_int {
                        if _mfs_assert_ret_5 < 0 as ::core::ffi::c_int
                            && *__errno_location() != 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_11: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_destroy(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_5,
                                *__errno_location(),
                                _mfs_errorstring_11,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_destroy(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_5,
                                *__errno_location(),
                                _mfs_errorstring_11,
                            );
                        } else if _mfs_assert_ret_5 > 0 as ::core::ffi::c_int
                            && *__errno_location() == 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_12: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_5);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_destroy(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_5,
                                _mfs_errorstring_12,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_destroy(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_5,
                                _mfs_errorstring_12,
                            );
                        } else {
                            let mut _mfs_errorstring_err_5: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            let mut _mfs_errorstring_ret_5: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_5);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_destroy(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_5,
                                _mfs_errorstring_ret_5,
                                *__errno_location(),
                                _mfs_errorstring_err_5,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_destroy(&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_5,
                                _mfs_errorstring_ret_5,
                                *__errno_location(),
                                _mfs_errorstring_err_5,
                            );
                        }
                        abort();
                    }
                    free(ind as *mut ::core::ffi::c_void);
                }
                let mut _mfs_assert_ret_6: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut hashlock);
                if _mfs_assert_ret_6 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_6 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_13: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            399 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_6,
                            *__errno_location(),
                            _mfs_errorstring_13,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            399 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_6,
                            *__errno_location(),
                            _mfs_errorstring_13,
                        );
                    } else if _mfs_assert_ret_6 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_14: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_6);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            399 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_6,
                            _mfs_errorstring_14,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            399 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_6,
                            _mfs_errorstring_14,
                        );
                    } else {
                        let mut _mfs_errorstring_err_6: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_6: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_6);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            399 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_6,
                            _mfs_errorstring_ret_6,
                            *__errno_location(),
                            _mfs_errorstring_err_6,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            399 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&hashlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_6,
                            _mfs_errorstring_ret_6,
                            *__errno_location(),
                            _mfs_errorstring_err_6,
                        );
                    }
                    abort();
                }
                return;
            }
            indp = &raw mut (*ind).next as *mut *mut inodedata;
        }
        let mut _mfs_assert_ret_7: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut hashlock);
        if _mfs_assert_ret_7 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_7 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_15: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    404 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_7,
                    *__errno_location(),
                    _mfs_errorstring_15,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    404 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_7,
                    *__errno_location(),
                    _mfs_errorstring_15,
                );
            } else if _mfs_assert_ret_7 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_16: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_7);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    404 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_7,
                    _mfs_errorstring_16,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    404 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_7,
                    _mfs_errorstring_16,
                );
            } else {
                let mut _mfs_errorstring_err_7: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_7: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_7);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    404 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_7,
                    _mfs_errorstring_ret_7,
                    *__errno_location(),
                    _mfs_errorstring_err_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    404 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_7,
                    _mfs_errorstring_ret_7,
                    *__errno_location(),
                    _mfs_errorstring_err_7,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_test_chunkdata(mut ind: *mut inodedata) {
    unsafe {
        let mut chd: *mut chunkdata = ::core::ptr::null_mut::<chunkdata>();
        if ((*ind).chunkscnt as ::core::ffi::c_int) < MAX_SIM_CHUNKS {
            if !(*ind).chunksnext.is_null() {
                chd = (*ind).chunksnext;
                (*ind).chunksnext = (*chd).next as *mut chunkdata;
                (*ind).chunkscnt = (*ind).chunkscnt.wrapping_add(1);
                write_enqueue(chd);
            }
        } else {
            chd = (*ind).chunks;
            while !chd.is_null() {
                if (*chd).waitingworker != 0 {
                    if write(
                        (*chd).wakeup_fd,
                        b" \0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        1 as size_t,
                    ) != 1 as ssize_t
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"can't write to pipe !!!\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    (*chd).waitingworker = 0 as uint8_t;
                    (*chd).wakeup_fd = -1 as ::core::ffi::c_int;
                }
                chd = (*chd).next as *mut chunkdata;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_new_chunkdata(
    mut ind: *mut inodedata,
    mut chindx: uint32_t,
) -> *mut chunkdata {
    unsafe {
        let mut chd: *mut chunkdata = ::core::ptr::null_mut::<chunkdata>();
        chd = malloc(::core::mem::size_of::<chunkdata>()) as *mut chunkdata;
        if chd.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                436 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chd\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                436 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chd\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if chd
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut chunkdata
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                436 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chd\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                436 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chd\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*chd).chindx = chindx;
        (*chd).wakeup_fd = -1 as ::core::ffi::c_int;
        (*chd).datachainhead = ::core::ptr::null_mut::<cblock>();
        (*chd).datachaintail = ::core::ptr::null_mut::<cblock>();
        (*chd).waitingworker = 0 as uint8_t;
        (*chd).chunkready = 0 as uint8_t;
        (*chd).unbreakable = 0 as uint8_t;
        (*chd).continueop = 0 as uint8_t;
        (*chd).superuser = 0 as uint8_t;
        (*chd).trycnt = 0 as uint16_t;
        (*chd).parent = ind as *mut inodedata_s;
        (*chd).next = ::core::ptr::null_mut::<chunkdata_s>();
        (*chd).prev = (*ind).chunkstail as *mut *mut chunkdata_s;
        *(*ind).chunkstail = chd;
        (*ind).chunkstail = &raw mut (*chd).next as *mut *mut chunkdata;
        if (*ind).chunksnext.is_null() {
            (*ind).chunksnext = chd;
        }
        return chd;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_free_chunkdata(mut chd: *mut chunkdata) {
    unsafe {
        *(*chd).prev = (*chd).next;
        if !(*chd).next.is_null() {
            (*(*chd).next).prev = (*chd).prev;
        } else {
            (*(*chd).parent).chunkstail = (*chd).prev as *mut *mut chunkdata;
        }
        (*(*chd).parent).chunkscnt = (*(*chd).parent).chunkscnt.wrapping_sub(1);
        write_test_chunkdata((*chd).parent as *mut inodedata);
        free(chd as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_enqueue(mut chd: *mut chunkdata) {
    unsafe {
        queue_put(
            jqueue,
            0 as uint32_t,
            0 as uint32_t,
            chd as *mut uint8_t,
            0 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_delayrun_enqueue(mut udata: *mut ::core::ffi::c_void) {
    unsafe {
        queue_put(
            jqueue,
            0 as uint32_t,
            0 as uint32_t,
            udata as *mut uint8_t,
            0 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_delayed_enqueue(mut chd: *mut chunkdata, mut usecs: uint32_t) {
    unsafe {
        if usecs > 0 as uint32_t {
            delay_run(
                Some(
                    write_delayrun_enqueue as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
                chd as *mut ::core::ffi::c_void,
                usecs as uint64_t,
            );
        } else {
            queue_put(
                jqueue,
                0 as uint32_t,
                0 as uint32_t,
                chd as *mut uint8_t,
                0 as uint32_t,
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_job_end(
    mut chd: *mut chunkdata,
    mut status: ::core::ffi::c_int,
    mut delay: uint32_t,
) {
    unsafe {
        let mut cb: *mut cblock = ::core::ptr::null_mut::<cblock>();
        let mut fcb: *mut cblock = ::core::ptr::null_mut::<cblock>();
        let mut ind: *mut inodedata = (*chd).parent as *mut inodedata;
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*ind).lock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    531 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    531 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    531 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    531 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    531 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    531 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        if status != 0 as ::core::ffi::c_int {
            *__errno_location() = status;
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"error writing file number %u: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*ind).inode,
                strerr(*__errno_location()),
            );
            (*ind).status = status;
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_cond_broadcast(&raw mut (*ind).chunkcond);
            if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        537 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(ind->chunkcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        537 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(ind->chunkcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        537 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(ind->chunkcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        537 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(ind->chunkcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                } else {
                    let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        537 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(ind->chunkcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        537 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(ind->chunkcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
        }
        if status == 0 as ::core::ffi::c_int && delay == 0 as uint32_t {
            (*chd).trycnt = 0 as uint16_t;
        }
        status = (*ind).status;
        if !(*chd).datachainhead.is_null() && status == 0 as ::core::ffi::c_int {
            cb = (*chd).datachainhead;
            while !cb.is_null() {
                (*cb).writeid = 0 as uint32_t;
                cb = (*cb).next as *mut cblock;
            }
            write_delayed_enqueue(chd, delay);
        } else {
            cb = (*chd).datachainhead;
            while !cb.is_null() {
                fcb = cb;
                cb = (*cb).next as *mut cblock;
                write_cb_release(ind, fcb);
            }
            if (*ind).flushwaiting as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                    pthread_cond_broadcast(&raw mut (*ind).flushcond);
                if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_broadcast(&(ind->flushcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_3,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_broadcast(&(ind->flushcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_3,
                        );
                    } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_4: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_1);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_broadcast(&(ind->flushcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_4,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_broadcast(&(ind->flushcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_4,
                        );
                    } else {
                        let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_1);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_broadcast(&(ind->flushcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_err_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_broadcast(&(ind->flushcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_err_1,
                        );
                    }
                    abort();
                }
            }
            write_free_chunkdata(chd);
        }
        let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*ind).lock);
        if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    564 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    564 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
            } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_6: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    564 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    564 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
            } else {
                let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    564 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    564 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
    }
}
static mut lastnotify: uint32_t = 0 as uint32_t;
#[inline]
unsafe extern "C" fn write_data_spawn_worker() {
    unsafe {
        let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
        let mut newset: sigset_t = sigset_t { __val: [0; 16] };
        let mut w: *mut worker = ::core::ptr::null_mut::<worker>();
        let mut res: ::core::ffi::c_int = 0;
        w = malloc(::core::mem::size_of::<worker>()) as *mut worker;
        if w.is_null() {
            return;
        }
        sigemptyset(&raw mut newset);
        sigaddset(&raw mut newset, SIGTERM);
        sigaddset(&raw mut newset, SIGINT);
        sigaddset(&raw mut newset, SIGHUP);
        sigaddset(&raw mut newset, SIGQUIT);
        let mut _mfs_assert_ret: ::core::ffi::c_int =
            pthread_sigmask(0 as ::core::ffi::c_int, &raw mut newset, &raw mut oldset);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    591 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_BLOCK, &newset, &oldset)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    591 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_BLOCK, &newset, &oldset)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    591 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_BLOCK, &newset, &oldset)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    591 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_BLOCK, &newset, &oldset)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    591 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_BLOCK, &newset, &oldset)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    591 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_BLOCK, &newset, &oldset)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        res = pthread_create(
            &raw mut (*w).thread_id,
            &raw mut worker_thattr,
            Some(
                write_worker
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            w as *mut ::core::ffi::c_void,
        );
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_sigmask(
            2 as ::core::ffi::c_int,
            &raw mut oldset,
            ::core::ptr::null_mut::<__sigset_t>(),
        );
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_SETMASK, &oldset, NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_SETMASK, &oldset, NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_SETMASK, &oldset, NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_SETMASK, &oldset, NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_SETMASK, &oldset, NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_SETMASK, &oldset, NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        if res < 0 as ::core::ffi::c_int {
            return;
        }
        workers_avail = workers_avail.wrapping_add(1);
        workers_total = workers_total.wrapping_add(1);
        if workers_total.wrapping_rem(10 as uint32_t) == 0 as uint32_t
            && workers_total != lastnotify
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"write workers: %u+\n\0".as_ptr() as *const ::core::ffi::c_char,
                workers_total,
            );
            lastnotify = workers_total;
        }
    }
}
#[inline]
unsafe extern "C" fn write_data_close_worker(mut w: *mut worker) {
    unsafe {
        workers_avail = workers_avail.wrapping_sub(1);
        workers_total = workers_total.wrapping_sub(1);
        if workers_total == 0 as uint32_t && worker_term_waiting != 0 {
            let mut _mfs_assert_ret: ::core::ffi::c_int =
                pthread_cond_signal(&raw mut worker_term_cond);
            if _mfs_assert_ret != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        616 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&worker_term_cond)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        616 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&worker_term_cond)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        616 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&worker_term_cond)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        616 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&worker_term_cond)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                } else {
                    let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        616 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&worker_term_cond)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        616 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&worker_term_cond)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                }
                abort();
            }
            worker_term_waiting = worker_term_waiting.wrapping_sub(1);
        }
        pthread_detach((*w).thread_id);
        free(w as *mut ::core::ffi::c_void);
        if workers_total.wrapping_rem(10 as uint32_t) == 0 as uint32_t
            && workers_total != lastnotify
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"write workers: %u-\n\0".as_ptr() as *const ::core::ffi::c_char,
                workers_total,
            );
            lastnotify = workers_total;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_worker(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut z1: uint32_t = 0;
        let mut z2: uint32_t = 0;
        let mut z3: uint32_t = 0;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut fd: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut pfd: [pollfd; 2] = [pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        }; 2];
        let mut sent: uint32_t = 0;
        let mut rcvd: uint32_t = 0;
        let mut hdrtosend: uint32_t = 0;
        let mut sending_mode: uint8_t = 0;
        let mut wants_pollout: uint8_t = 0;
        let mut recvbuff: [uint8_t; 21] = [0; 21];
        let mut sendbuff: [uint8_t; 32] = [0; 32];
        let mut siov: [iovec; 2] = [iovec {
            iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            iov_len: 0,
        }; 2];
        let mut pipebuff: [uint8_t; 1024] = [0; 1024];
        let mut pipefd: [::core::ffi::c_int; 2] = [0; 2];
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut reccmd: uint32_t = 0;
        let mut recleng: uint32_t = 0;
        let mut recchunkid: uint64_t = 0;
        let mut recwriteid: uint32_t = 0;
        let mut recstatus: uint8_t = 0;
        let mut cpw: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut chain: [cspri; 100] = [cspri {
            ip: 0,
            port: 0,
            version: 0,
            labelmask: 0,
            priority: 0,
        }; 100];
        let mut chainminver: uint32_t = 0;
        let mut chainelements: uint16_t = 0;
        let mut cschain: [uint8_t; 594] = [0; 594];
        let mut cschainsize: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut chindx: uint32_t = 0;
        let mut ip: uint32_t = 0;
        let mut port: uint16_t = 0;
        let mut srcip: uint32_t = 0;
        let mut mfleng: uint64_t = 0;
        let mut maxwroffset: uint64_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut nextwriteid: uint32_t = 0;
        let mut csdata: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut csdatasize: uint32_t = 0;
        let mut csdataver: uint8_t = 0;
        let mut westatus: uint8_t = 0;
        let mut wrstatus: uint8_t = 0;
        let mut chunkready: uint8_t = 0;
        let mut unbreakable: uint8_t = 0;
        let mut chunkopflags: uint8_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut csstrip: [::core::ffi::c_char; 16] = [0; 16];
        let mut waitforstatus: uint8_t = 0;
        let mut donotstayidle: uint8_t = 0;
        let mut opbegin: ::core::ffi::c_double = 0.;
        let mut start: ::core::ffi::c_double = 0.;
        let mut now: ::core::ffi::c_double = 0.;
        let mut lastrcvd: ::core::ffi::c_double = 0.;
        let mut lastblock: ::core::ffi::c_double = 0.;
        let mut lastsent: ::core::ffi::c_double = 0.;
        let mut workingtime: ::core::ffi::c_double = 0.;
        let mut lrdiff: ::core::ffi::c_double = 0.;
        let mut lbdiff: ::core::ffi::c_double = 0.;
        let mut wtotal: uint32_t = 0;
        let mut cnt: uint8_t = 0;
        let mut firsttime: uint8_t = 1 as uint8_t;
        let mut w: *mut worker = arg as *mut worker;
        let mut valid_offsets: uint8_t = 0;
        let mut min_offset: uint64_t = 0;
        let mut max_offset: uint64_t = 0;
        let mut min_from: uint32_t = 0;
        let mut max_to: uint32_t = 0;
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        let mut chd: *mut chunkdata = ::core::ptr::null_mut::<chunkdata>();
        let mut cb: *mut cblock = ::core::ptr::null_mut::<cblock>();
        let mut ncb: *mut cblock = ::core::ptr::null_mut::<cblock>();
        let mut rcb: *mut cblock = ::core::ptr::null_mut::<cblock>();
        chainelements = 0 as uint16_t;
        chindx = 0 as uint32_t;
        if pipe(&raw mut pipefd as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"pipe error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerr(*__errno_location()),
            );
            return NULL;
        }
        loop {
            i = 0 as ::core::ffi::c_int;
            while i < chainelements as ::core::ffi::c_int {
                csdb_writedec(chain[i as usize].ip, chain[i as usize].port);
                i += 1;
            }
            chainelements = 0 as uint16_t;
            if firsttime as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let mut _mfs_assert_ret: ::core::ffi::c_int =
                    pthread_mutex_lock(&raw mut workerslock);
                if _mfs_assert_ret != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            *__errno_location(),
                            _mfs_errorstring,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            *__errno_location(),
                            _mfs_errorstring,
                        );
                    } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            _mfs_errorstring_0,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            _mfs_errorstring_0,
                        );
                    } else {
                        let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            _mfs_errorstring_ret,
                            *__errno_location(),
                            _mfs_errorstring_err,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            _mfs_errorstring_ret,
                            *__errno_location(),
                            _mfs_errorstring_err,
                        );
                    }
                    abort();
                }
                workers_avail = workers_avail.wrapping_add(1);
                if workers_avail > SUSTAIN_WORKERS as uint32_t {
                    write_data_close_worker(w);
                    let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                        pthread_mutex_unlock(&raw mut workerslock);
                    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                            && *__errno_location() != 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                734 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_1,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                734 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_1,
                            );
                        } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                            && *__errno_location() == 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_0);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                734 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_2,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                734 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_2,
                            );
                        } else {
                            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_0);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                734 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_err_0,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                734 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_err_0,
                            );
                        }
                        abort();
                    }
                    close_pipe(&raw mut pipefd as *mut ::core::ffi::c_int);
                    return NULL;
                }
                let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut workerslock);
                if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            738 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_3,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            738 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_3,
                        );
                    } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_4: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_1);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            738 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_4,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            738 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_4,
                        );
                    } else {
                        let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_1);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            738 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_err_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            738 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_err_1,
                        );
                    }
                    abort();
                }
            }
            firsttime = 0 as uint8_t;
            queue_get(jqueue, &raw mut z1, &raw mut z2, &raw mut data, &raw mut z3);
            let mut _mfs_assert_ret_2: ::core::ffi::c_int =
                pthread_mutex_lock(&raw mut workerslock);
            if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        745 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_5,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        745 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_5,
                    );
                } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_6: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_2);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        745 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_6,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        745 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_6,
                    );
                } else {
                    let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_2);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        745 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_err_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        745 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_err_2,
                    );
                }
                abort();
            }
            if data.is_null() {
                write_data_close_worker(w);
                let mut _mfs_assert_ret_3: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut workerslock);
                if _mfs_assert_ret_3 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_3 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_7: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            749 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_7,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            749 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_7,
                        );
                    } else if _mfs_assert_ret_3 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_8: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_3);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            749 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_8,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            749 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_8,
                        );
                    } else {
                        let mut _mfs_errorstring_err_3: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_3: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_3);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            749 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_err_3,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            749 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_err_3,
                        );
                    }
                    abort();
                }
                close_pipe(&raw mut pipefd as *mut ::core::ffi::c_int);
                return NULL;
            }
            workers_avail = workers_avail.wrapping_sub(1);
            if workers_avail == 0 as uint32_t && workers_total < MAX_WORKERS as uint32_t {
                write_data_spawn_worker();
            }
            let mut _mfs_assert_ret_4: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut workerslock);
            if _mfs_assert_ret_4 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_4 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_9: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        758 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        *__errno_location(),
                        _mfs_errorstring_9,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        758 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        *__errno_location(),
                        _mfs_errorstring_9,
                    );
                } else if _mfs_assert_ret_4 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_10: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_4);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        758 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        _mfs_errorstring_10,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        758 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        _mfs_errorstring_10,
                    );
                } else {
                    let mut _mfs_errorstring_err_4: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_4: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_4);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        758 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        _mfs_errorstring_ret_4,
                        *__errno_location(),
                        _mfs_errorstring_err_4,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        758 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        _mfs_errorstring_ret_4,
                        *__errno_location(),
                        _mfs_errorstring_err_4,
                    );
                }
                abort();
            }
            chd = data as *mut chunkdata;
            ind = (*chd).parent as *mut inodedata;
            let mut _mfs_assert_ret_5: ::core::ffi::c_int =
                pthread_mutex_lock(&raw mut (*ind).lock);
            if _mfs_assert_ret_5 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_5 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_11: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        763 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_5,
                        *__errno_location(),
                        _mfs_errorstring_11,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        763 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_5,
                        *__errno_location(),
                        _mfs_errorstring_11,
                    );
                } else if _mfs_assert_ret_5 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_12: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_5);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        763 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_5,
                        _mfs_errorstring_12,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        763 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_5,
                        _mfs_errorstring_12,
                    );
                } else {
                    let mut _mfs_errorstring_err_5: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_5: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_5);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        763 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_5,
                        _mfs_errorstring_ret_5,
                        *__errno_location(),
                        _mfs_errorstring_err_5,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        763 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_5,
                        _mfs_errorstring_ret_5,
                        *__errno_location(),
                        _mfs_errorstring_err_5,
                    );
                }
                abort();
            }
            if !(*chd).datachainhead.is_null() {
                chindx = (*chd).chindx;
                status = (*ind).status;
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"writeworker got inode with no data to write !!!\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                status = EINVAL;
            }
            chunkready = (*chd).chunkready;
            chunkopflags = ((if (*chd).continueop as ::core::ffi::c_int != 0 {
                CHUNKOPFLAG_CONTINUEOP
            } else {
                0 as ::core::ffi::c_int
            }) | (if (*chd).superuser as ::core::ffi::c_int != 0 {
                CHUNKOPFLAG_CANUSERESERVESPACE
            } else {
                0 as ::core::ffi::c_int
            })) as uint8_t;
            let mut _mfs_assert_ret_6: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut (*ind).lock);
            if _mfs_assert_ret_6 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_6 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_13: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        775 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_6,
                        *__errno_location(),
                        _mfs_errorstring_13,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        775 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_6,
                        *__errno_location(),
                        _mfs_errorstring_13,
                    );
                } else if _mfs_assert_ret_6 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_14: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_6);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        775 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_6,
                        _mfs_errorstring_14,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        775 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_6,
                        _mfs_errorstring_14,
                    );
                } else {
                    let mut _mfs_errorstring_err_6: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_6: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_6);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        775 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_6,
                        _mfs_errorstring_ret_6,
                        *__errno_location(),
                        _mfs_errorstring_err_6,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        775 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_6,
                        _mfs_errorstring_ret_6,
                        *__errno_location(),
                        _mfs_errorstring_err_6,
                    );
                }
                abort();
            }
            if status != 0 {
                write_job_end(chd, status, 0 as uint32_t);
            } else {
                inode = (*ind).inode;
                chunkrwlock_wlock(inode, chindx);
                opbegin = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                if optimeout > 0.0f64 {
                    opbegin = monotonic_seconds();
                }
                valid_offsets = 0 as uint8_t;
                min_offset = 0 as uint64_t;
                max_offset = 0 as uint64_t;
                min_from = 0 as uint32_t;
                max_to = 0 as uint32_t;
                version = 0 as uint32_t;
                chunkid = 0 as uint64_t;
                csdataver = 0 as uint8_t;
                wrstatus = fs_writechunk(
                    inode,
                    chindx,
                    chunkopflags,
                    &raw mut csdataver,
                    &raw mut mfleng,
                    &raw mut chunkid,
                    &raw mut version,
                    &raw mut csdata,
                    &raw mut csdatasize,
                );
                if wrstatus as ::core::ffi::c_int != MFS_STATUS_OK {
                    if wrstatus as ::core::ffi::c_int != MFS_ERROR_LOCKED
                        && wrstatus as ::core::ffi::c_int != MFS_ERROR_EAGAIN
                    {
                        if wrstatus as ::core::ffi::c_int == MFS_ERROR_ENOENT
                            || wrstatus as ::core::ffi::c_int == MFS_ERROR_EPERM
                            || wrstatus as ::core::ffi::c_int == MFS_ERROR_NOCHUNK
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_writechunk returned status: %s\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(wrstatus),
                            );
                            write_job_end(chd, EBADF, 0 as uint32_t);
                        } else if wrstatus as ::core::ffi::c_int == MFS_ERROR_INDEXTOOBIG {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_writechunk returned status: %s\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(wrstatus),
                            );
                            write_job_end(chd, EINVAL, 0 as uint32_t);
                        } else if wrstatus as ::core::ffi::c_int == MFS_ERROR_QUOTA {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_writechunk returned status: %s\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(wrstatus),
                            );
                            write_job_end(chd, EDQUOT, 0 as uint32_t);
                        } else if wrstatus as ::core::ffi::c_int == MFS_ERROR_NOSPACE
                            && erroronnospace as ::core::ffi::c_int != 0
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_writechunk returned status: %s\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(wrstatus),
                            );
                            write_job_end(chd, ENOSPC, 0 as uint32_t);
                        } else if wrstatus as ::core::ffi::c_int == MFS_ERROR_CHUNKLOST
                            && erroronlostchunk as ::core::ffi::c_int != 0
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_writechunk returned status: %s\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(wrstatus),
                            );
                            write_job_end(chd, ENXIO, 0 as uint32_t);
                        } else if wrstatus as ::core::ffi::c_int == MFS_ERROR_IO {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_writechunk returned status: %s\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(wrstatus),
                            );
                            write_job_end(chd, EIO, 0 as uint32_t);
                        } else if wrstatus as ::core::ffi::c_int == MFS_ERROR_EROFS {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_writechunk returned status: %s\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(wrstatus),
                            );
                            write_job_end(chd, EROFS, 0 as uint32_t);
                        } else {
                            if (*chd).trycnt as uint32_t >= minlogretry {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"file: %u, index: %u - fs_writechunk returned status: %s\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    inode,
                                    chindx,
                                    mfsstrerr(wrstatus),
                                );
                            }
                            (*chd).trycnt = (*chd).trycnt.wrapping_add(1);
                            if (*chd).trycnt as uint32_t >= maxretries {
                                if wrstatus as ::core::ffi::c_int == MFS_ERROR_NOCHUNKSERVERS
                                    || wrstatus as ::core::ffi::c_int == MFS_ERROR_NOSPACE
                                {
                                    write_job_end(chd, ENOSPC, 0 as uint32_t);
                                } else if wrstatus as ::core::ffi::c_int == MFS_ERROR_CSNOTPRESENT
                                    || wrstatus as ::core::ffi::c_int == MFS_ERROR_CHUNKLOST
                                {
                                    write_job_end(chd, ENXIO, 0 as uint32_t);
                                } else {
                                    write_job_end(chd, EIO, 0 as uint32_t);
                                }
                            } else {
                                write_delayed_enqueue(
                                    chd,
                                    (1000 as ::core::ffi::c_int
                                        + (if ((*chd).trycnt as ::core::ffi::c_int)
                                            < 30 as ::core::ffi::c_int
                                        {
                                            ((*chd).trycnt as ::core::ffi::c_int
                                                - 1 as ::core::ffi::c_int)
                                                * 300000 as ::core::ffi::c_int
                                        } else {
                                            10000000 as ::core::ffi::c_int
                                        })) as uint32_t,
                                );
                            }
                        }
                    } else if (*chd).trycnt as ::core::ffi::c_int <= 2 as ::core::ffi::c_int {
                        (*chd).trycnt = (*chd).trycnt.wrapping_add(1);
                        write_delayed_enqueue(chd, 1000 as uint32_t);
                    } else if (*chd).trycnt as ::core::ffi::c_int <= 6 as ::core::ffi::c_int {
                        (*chd).trycnt = (*chd).trycnt.wrapping_add(1);
                        write_delayed_enqueue(chd, 100000 as uint32_t);
                    } else {
                        write_delayed_enqueue(chd, 500000 as uint32_t);
                    }
                    chunkrwlock_wunlock(inode, chindx);
                } else {
                    chunksdatacache_insert(
                        inode, chindx, chunkid, version, csdataver, csdata, csdatasize,
                    );
                    if !csdata.is_null() && csdatasize > 0 as uint32_t {
                        chainelements = csorder_sort(
                            &raw mut chain as *mut cspri,
                            csdataver,
                            csdata,
                            csdatasize,
                            1 as uint8_t,
                        ) as uint16_t;
                    } else {
                        chainelements = 0 as uint16_t;
                    }
                    if csdata.is_null()
                        || csdatasize == 0 as uint32_t
                        || chainelements as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        chainelements = 0 as uint16_t;
                        let mut _mfs_assert_ret_7: ::core::ffi::c_int =
                            pthread_mutex_lock(&raw mut (*ind).lock);
                        if _mfs_assert_ret_7 != 0 as ::core::ffi::c_int {
                            if _mfs_assert_ret_7 < 0 as ::core::ffi::c_int
                                && *__errno_location() != 0 as ::core::ffi::c_int
                            {
                                let mut _mfs_errorstring_15: *const ::core::ffi::c_char =
                                    strerr(*__errno_location());
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_7,
                                    *__errno_location(),
                                    _mfs_errorstring_15,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_7,
                                    *__errno_location(),
                                    _mfs_errorstring_15,
                                );
                            } else if _mfs_assert_ret_7 > 0 as ::core::ffi::c_int
                                && *__errno_location() == 0 as ::core::ffi::c_int
                            {
                                let mut _mfs_errorstring_16: *const ::core::ffi::c_char =
                                    strerr(_mfs_assert_ret_7);
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_7,
                                    _mfs_errorstring_16,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_7,
                                    _mfs_errorstring_16,
                                );
                            } else {
                                let mut _mfs_errorstring_err_7: *const ::core::ffi::c_char =
                                    strerr(*__errno_location());
                                let mut _mfs_errorstring_ret_7: *const ::core::ffi::c_char =
                                    strerr(_mfs_assert_ret_7);
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_7,
                                    _mfs_errorstring_ret_7,
                                    *__errno_location(),
                                    _mfs_errorstring_err_7,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_7,
                                    _mfs_errorstring_ret_7,
                                    *__errno_location(),
                                    _mfs_errorstring_err_7,
                                );
                            }
                            abort();
                        }
                        (*chd).trycnt = ((*chd).trycnt as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int)
                            as uint16_t;
                        unbreakable = (*chd).unbreakable;
                        if (*chd).trycnt as uint32_t >= maxretries {
                            unbreakable = 0 as uint8_t;
                        }
                        (*chd).continueop = unbreakable;
                        let mut _mfs_assert_ret_8: ::core::ffi::c_int =
                            pthread_mutex_unlock(&raw mut (*ind).lock);
                        if _mfs_assert_ret_8 != 0 as ::core::ffi::c_int {
                            if _mfs_assert_ret_8 < 0 as ::core::ffi::c_int
                                && *__errno_location() != 0 as ::core::ffi::c_int
                            {
                                let mut _mfs_errorstring_17: *const ::core::ffi::c_char =
                                    strerr(*__errno_location());
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    900 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_8,
                                    *__errno_location(),
                                    _mfs_errorstring_17,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    900 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_8,
                                    *__errno_location(),
                                    _mfs_errorstring_17,
                                );
                            } else if _mfs_assert_ret_8 > 0 as ::core::ffi::c_int
                                && *__errno_location() == 0 as ::core::ffi::c_int
                            {
                                let mut _mfs_errorstring_18: *const ::core::ffi::c_char =
                                    strerr(_mfs_assert_ret_8);
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    900 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_8,
                                    _mfs_errorstring_18,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    900 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_8,
                                    _mfs_errorstring_18,
                                );
                            } else {
                                let mut _mfs_errorstring_err_8: *const ::core::ffi::c_char =
                                    strerr(*__errno_location());
                                let mut _mfs_errorstring_ret_8: *const ::core::ffi::c_char =
                                    strerr(_mfs_assert_ret_8);
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    900 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_8,
                                    _mfs_errorstring_ret_8,
                                    *__errno_location(),
                                    _mfs_errorstring_err_8,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    900 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_8,
                                    _mfs_errorstring_ret_8,
                                    *__errno_location(),
                                    _mfs_errorstring_err_8,
                                );
                            }
                            abort();
                        }
                        if unbreakable as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            fs_writeend(
                                chunkid,
                                inode,
                                chindx,
                                0 as uint64_t,
                                0 as uint8_t,
                                min_from,
                                max_to.wrapping_sub(min_from),
                            );
                        }
                        if (*chd).trycnt as uint32_t >= minlogretry {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u, chunk: %016lX, version: %u - there are no valid copies\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                chunkid,
                                version,
                            );
                        }
                        if (*chd).trycnt as uint32_t >= maxretries {
                            write_job_end(chd, ENXIO, 0 as uint32_t);
                        } else {
                            write_delayed_enqueue(chd, 60000000 as uint32_t);
                        }
                        chunkrwlock_wunlock(inode, chindx);
                    } else {
                        ip = chain[0 as usize].ip;
                        port = chain[0 as usize].port;
                        chainminver = chain[0 as usize].version;
                        csdb_writeinc(ip, port);
                        csstrip[0 as usize] = 0 as ::core::ffi::c_char;
                        cpw = &raw mut cschain as *mut uint8_t;
                        cschainsize = 0 as uint32_t;
                        i = 1 as ::core::ffi::c_int;
                        while i < chainelements as ::core::ffi::c_int {
                            csdb_writeinc(chain[i as usize].ip, chain[i as usize].port);
                            if chain[i as usize].version < chainminver {
                                chainminver = chain[i as usize].version;
                            }
                            put32bit(&raw mut cpw, chain[i as usize].ip);
                            put16bit(&raw mut cpw, chain[i as usize].port);
                            cschainsize = cschainsize.wrapping_add(6 as uint32_t);
                            i += 1;
                        }
                        start = monotonic_seconds();
                        srcip = fs_getsrcip();
                        fd = conncache_get(ip, port);
                        if fd < 0 as ::core::ffi::c_int {
                            let mut connmaxtry: uint32_t = 0;
                            let mut _mfs_assert_ret_9: ::core::ffi::c_int =
                                pthread_mutex_lock(&raw mut (*ind).lock);
                            if _mfs_assert_ret_9 != 0 as ::core::ffi::c_int {
                                if _mfs_assert_ret_9 < 0 as ::core::ffi::c_int
                                    && *__errno_location() != 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_19: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1005 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_9,
                                        *__errno_location(),
                                        _mfs_errorstring_19,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1005 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_9,
                                        *__errno_location(),
                                        _mfs_errorstring_19,
                                    );
                                } else if _mfs_assert_ret_9 > 0 as ::core::ffi::c_int
                                    && *__errno_location() == 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_20: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_9);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1005 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_9,
                                        _mfs_errorstring_20,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1005 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_9,
                                        _mfs_errorstring_20,
                                    );
                                } else {
                                    let mut _mfs_errorstring_err_9: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    let mut _mfs_errorstring_ret_9: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_9);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1005 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_9,
                                        _mfs_errorstring_ret_9,
                                        *__errno_location(),
                                        _mfs_errorstring_err_9,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1005 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_9,
                                        _mfs_errorstring_ret_9,
                                        *__errno_location(),
                                        _mfs_errorstring_err_9,
                                    );
                                }
                                abort();
                            }
                            connmaxtry = ((*chd).trycnt as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int)
                                as uint32_t;
                            if connmaxtry > 10 as uint32_t {
                                connmaxtry = 10 as uint32_t;
                            }
                            let mut _mfs_assert_ret_10: ::core::ffi::c_int =
                                pthread_mutex_unlock(&raw mut (*ind).lock);
                            if _mfs_assert_ret_10 != 0 as ::core::ffi::c_int {
                                if _mfs_assert_ret_10 < 0 as ::core::ffi::c_int
                                    && *__errno_location() != 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_21: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1010 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_10,
                                        *__errno_location(),
                                        _mfs_errorstring_21,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1010 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_10,
                                        *__errno_location(),
                                        _mfs_errorstring_21,
                                    );
                                } else if _mfs_assert_ret_10 > 0 as ::core::ffi::c_int
                                    && *__errno_location() == 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_22: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_10);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1010 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_10,
                                        _mfs_errorstring_22,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1010 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_10,
                                        _mfs_errorstring_22,
                                    );
                                } else {
                                    let mut _mfs_errorstring_err_10: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    let mut _mfs_errorstring_ret_10: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_10);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1010 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_10,
                                        _mfs_errorstring_ret_10,
                                        *__errno_location(),
                                        _mfs_errorstring_err_10,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1010 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_10,
                                        _mfs_errorstring_ret_10,
                                        *__errno_location(),
                                        _mfs_errorstring_err_10,
                                    );
                                }
                                abort();
                            }
                            cnt = 0 as uint8_t;
                            while (cnt as uint32_t) < connmaxtry {
                                fd = tcpsocket();
                                if fd < 0 as ::core::ffi::c_int {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"writeworker: can't create tcp socket: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        strerr(*__errno_location()),
                                    );
                                    break;
                                } else {
                                    if srcip != 0 {
                                        if tcpnumbind(fd, srcip, 0 as uint16_t)
                                            < 0 as ::core::ffi::c_int
                                        {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"writeworker: can't bind socket to given ip: %s\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                strerr(*__errno_location()),
                                            );
                                            tcpclose(fd);
                                            fd = -1 as ::core::ffi::c_int;
                                            break;
                                        }
                                    }
                                    if tcpnumtoconnect(
                                        fd,
                                        ip,
                                        port,
                                        (if cnt as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
                                        {
                                            300 as ::core::ffi::c_int
                                                * ((1 as ::core::ffi::c_int)
                                                    << (cnt as ::core::ffi::c_int
                                                        >> 1 as ::core::ffi::c_int))
                                        } else {
                                            200 as ::core::ffi::c_int
                                                * ((1 as ::core::ffi::c_int)
                                                    << (cnt as ::core::ffi::c_int
                                                        >> 1 as ::core::ffi::c_int))
                                        }) as uint32_t,
                                    ) < 0 as ::core::ffi::c_int
                                    {
                                        cnt = cnt.wrapping_add(1);
                                        if cnt as uint32_t >= connmaxtry {
                                            let mut err: ::core::ffi::c_int = *__errno_location();
                                            let mut _mfs_assert_ret_11: ::core::ffi::c_int =
                                                pthread_mutex_lock(&raw mut (*ind).lock);
                                            if _mfs_assert_ret_11 != 0 as ::core::ffi::c_int {
                                                if _mfs_assert_ret_11 < 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        != 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_23: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1030 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_11,
                                                        *__errno_location(),
                                                        _mfs_errorstring_23,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1030 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_11,
                                                        *__errno_location(),
                                                        _mfs_errorstring_23,
                                                    );
                                                } else if _mfs_assert_ret_11
                                                    > 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        == 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_24: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_11,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1030 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_11,
                                                        _mfs_errorstring_24,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1030 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_11,
                                                        _mfs_errorstring_24,
                                                    );
                                                } else {
                                                    let mut _mfs_errorstring_err_11: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    let mut _mfs_errorstring_ret_11: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_11,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1030 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_11,
                                                        _mfs_errorstring_ret_11,
                                                        *__errno_location(),
                                                        _mfs_errorstring_err_11,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1030 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_11,
                                                        _mfs_errorstring_ret_11,
                                                        *__errno_location(),
                                                        _mfs_errorstring_err_11,
                                                    );
                                                }
                                                abort();
                                            }
                                            if (*chd).trycnt as uint32_t >= minlogretry {
                                                univmakestrip(
                                                    &raw mut csstrip as *mut ::core::ffi::c_char,
                                                    ip,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_WARNING,
                                                    b"writeworker: can't connect to (%s:%hu): %s\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    &raw mut csstrip as *mut ::core::ffi::c_char,
                                                    port as ::core::ffi::c_int,
                                                    strerr(err),
                                                );
                                            }
                                            let mut _mfs_assert_ret_12: ::core::ffi::c_int =
                                                pthread_mutex_unlock(&raw mut (*ind).lock);
                                            if _mfs_assert_ret_12 != 0 as ::core::ffi::c_int {
                                                if _mfs_assert_ret_12 < 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        != 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_25: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1035 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_12,
                                                        *__errno_location(),
                                                        _mfs_errorstring_25,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1035 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_12,
                                                        *__errno_location(),
                                                        _mfs_errorstring_25,
                                                    );
                                                } else if _mfs_assert_ret_12
                                                    > 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        == 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_26: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_12,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1035 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_12,
                                                        _mfs_errorstring_26,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1035 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_12,
                                                        _mfs_errorstring_26,
                                                    );
                                                } else {
                                                    let mut _mfs_errorstring_err_12: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    let mut _mfs_errorstring_ret_12: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_12,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1035 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_12,
                                                        _mfs_errorstring_ret_12,
                                                        *__errno_location(),
                                                        _mfs_errorstring_err_12,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1035 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_12,
                                                        _mfs_errorstring_ret_12,
                                                        *__errno_location(),
                                                        _mfs_errorstring_err_12,
                                                    );
                                                }
                                                abort();
                                            }
                                        }
                                        tcpclose(fd);
                                        fd = -1 as ::core::ffi::c_int;
                                    } else {
                                        let mut mip: uint32_t = 0;
                                        let mut pip: uint32_t = 0;
                                        let mut mport: uint16_t = 0;
                                        let mut pport: uint16_t = 0;
                                        tcpgetpeer(fd, &raw mut pip, &raw mut pport);
                                        tcpgetmyaddr(fd, &raw mut mip, &raw mut mport);
                                        cnt = connmaxtry as uint8_t;
                                    }
                                }
                            }
                        }
                        if fd < 0 as ::core::ffi::c_int {
                            let mut _mfs_assert_ret_13: ::core::ffi::c_int =
                                pthread_mutex_lock(&raw mut (*ind).lock);
                            if _mfs_assert_ret_13 != 0 as ::core::ffi::c_int {
                                if _mfs_assert_ret_13 < 0 as ::core::ffi::c_int
                                    && *__errno_location() != 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_27: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1052 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_13,
                                        *__errno_location(),
                                        _mfs_errorstring_27,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1052 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_13,
                                        *__errno_location(),
                                        _mfs_errorstring_27,
                                    );
                                } else if _mfs_assert_ret_13 > 0 as ::core::ffi::c_int
                                    && *__errno_location() == 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_28: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_13);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1052 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_13,
                                        _mfs_errorstring_28,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1052 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_13,
                                        _mfs_errorstring_28,
                                    );
                                } else {
                                    let mut _mfs_errorstring_err_13: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    let mut _mfs_errorstring_ret_13: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_13);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1052 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_13,
                                        _mfs_errorstring_ret_13,
                                        *__errno_location(),
                                        _mfs_errorstring_err_13,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1052 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_13,
                                        _mfs_errorstring_ret_13,
                                        *__errno_location(),
                                        _mfs_errorstring_err_13,
                                    );
                                }
                                abort();
                            }
                            (*chd).trycnt = (*chd).trycnt.wrapping_add(1);
                            unbreakable = (*chd).unbreakable;
                            if (*chd).trycnt as uint32_t >= maxretries {
                                unbreakable = 0 as uint8_t;
                            }
                            (*chd).continueop = unbreakable;
                            let mut _mfs_assert_ret_14: ::core::ffi::c_int =
                                pthread_mutex_unlock(&raw mut (*ind).lock);
                            if _mfs_assert_ret_14 != 0 as ::core::ffi::c_int {
                                if _mfs_assert_ret_14 < 0 as ::core::ffi::c_int
                                    && *__errno_location() != 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_29: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1059 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_14,
                                        *__errno_location(),
                                        _mfs_errorstring_29,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1059 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_14,
                                        *__errno_location(),
                                        _mfs_errorstring_29,
                                    );
                                } else if _mfs_assert_ret_14 > 0 as ::core::ffi::c_int
                                    && *__errno_location() == 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_30: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_14);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1059 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_14,
                                        _mfs_errorstring_30,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1059 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_14,
                                        _mfs_errorstring_30,
                                    );
                                } else {
                                    let mut _mfs_errorstring_err_14: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    let mut _mfs_errorstring_ret_14: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_14);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1059 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_14,
                                        _mfs_errorstring_ret_14,
                                        *__errno_location(),
                                        _mfs_errorstring_err_14,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1059 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_14,
                                        _mfs_errorstring_ret_14,
                                        *__errno_location(),
                                        _mfs_errorstring_err_14,
                                    );
                                }
                                abort();
                            }
                            if unbreakable as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                fs_writeend(
                                    chunkid,
                                    inode,
                                    chindx,
                                    0 as uint64_t,
                                    0 as uint8_t,
                                    min_from,
                                    max_to.wrapping_sub(min_from),
                                );
                            }
                            if (*chd).trycnt as uint32_t >= maxretries {
                                write_job_end(chd, EIO, 0 as uint32_t);
                            } else {
                                write_delayed_enqueue(
                                    chd,
                                    (1000 as ::core::ffi::c_int
                                        + (if ((*chd).trycnt as ::core::ffi::c_int)
                                            < 30 as ::core::ffi::c_int
                                        {
                                            ((*chd).trycnt as ::core::ffi::c_int
                                                - 1 as ::core::ffi::c_int)
                                                * 300000 as ::core::ffi::c_int
                                        } else {
                                            10000000 as ::core::ffi::c_int
                                        })) as uint32_t,
                                );
                            }
                            chunkrwlock_wunlock(inode, chindx);
                        } else {
                            if tcpnodelay(fd) < 0 as ::core::ffi::c_int {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_NOTICE,
                                    b"writeworker: can't set TCP_NODELAY: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    strerr(*__errno_location()),
                                );
                            }
                            if chunkready as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                let mut _mfs_assert_ret_15: ::core::ffi::c_int =
                                    pthread_mutex_lock(&raw mut (*ind).lock);
                                if _mfs_assert_ret_15 != 0 as ::core::ffi::c_int {
                                    if _mfs_assert_ret_15 < 0 as ::core::ffi::c_int
                                        && *__errno_location() != 0 as ::core::ffi::c_int
                                    {
                                        let mut _mfs_errorstring_31: *const ::core::ffi::c_char =
                                            strerr(*__errno_location());
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1076 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_15,
                                            *__errno_location(),
                                            _mfs_errorstring_31,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1076 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_15,
                                            *__errno_location(),
                                            _mfs_errorstring_31,
                                        );
                                    } else if _mfs_assert_ret_15 > 0 as ::core::ffi::c_int
                                        && *__errno_location() == 0 as ::core::ffi::c_int
                                    {
                                        let mut _mfs_errorstring_32: *const ::core::ffi::c_char =
                                            strerr(_mfs_assert_ret_15);
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1076 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_15,
                                            _mfs_errorstring_32,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1076 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_15,
                                            _mfs_errorstring_32,
                                        );
                                    } else {
                                        let mut _mfs_errorstring_err_15: *const ::core::ffi::c_char = strerr(
                                            *__errno_location(),
                                        );
                                        let mut _mfs_errorstring_ret_15: *const ::core::ffi::c_char = strerr(
                                            _mfs_assert_ret_15,
                                        );
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1076 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_15,
                                            _mfs_errorstring_ret_15,
                                            *__errno_location(),
                                            _mfs_errorstring_err_15,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1076 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_15,
                                            _mfs_errorstring_ret_15,
                                            *__errno_location(),
                                            _mfs_errorstring_err_15,
                                        );
                                    }
                                    abort();
                                }
                                if (*chd).chunkready as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    (*chd).chunkready = 1 as uint8_t;
                                    let mut _mfs_assert_ret_16: ::core::ffi::c_int =
                                        pthread_cond_broadcast(&raw mut (*ind).chunkcond);
                                    if _mfs_assert_ret_16 != 0 as ::core::ffi::c_int {
                                        if _mfs_assert_ret_16 < 0 as ::core::ffi::c_int
                                            && *__errno_location() != 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_33: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1080 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_cond_broadcast(&(ind->chunkcond))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_16,
                                                *__errno_location(),
                                                _mfs_errorstring_33,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1080 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_cond_broadcast(&(ind->chunkcond))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_16,
                                                *__errno_location(),
                                                _mfs_errorstring_33,
                                            );
                                        } else if _mfs_assert_ret_16 > 0 as ::core::ffi::c_int
                                            && *__errno_location() == 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_34: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_16,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1080 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_cond_broadcast(&(ind->chunkcond))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_16,
                                                _mfs_errorstring_34,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1080 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_cond_broadcast(&(ind->chunkcond))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_16,
                                                _mfs_errorstring_34,
                                            );
                                        } else {
                                            let mut _mfs_errorstring_err_16: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            let mut _mfs_errorstring_ret_16: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_16,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1080 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_cond_broadcast(&(ind->chunkcond))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_16,
                                                _mfs_errorstring_ret_16,
                                                *__errno_location(),
                                                _mfs_errorstring_err_16,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1080 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_cond_broadcast(&(ind->chunkcond))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_16,
                                                _mfs_errorstring_ret_16,
                                                *__errno_location(),
                                                _mfs_errorstring_err_16,
                                            );
                                        }
                                        abort();
                                    }
                                }
                                let mut _mfs_assert_ret_17: ::core::ffi::c_int =
                                    pthread_mutex_unlock(&raw mut (*ind).lock);
                                if _mfs_assert_ret_17 != 0 as ::core::ffi::c_int {
                                    if _mfs_assert_ret_17 < 0 as ::core::ffi::c_int
                                        && *__errno_location() != 0 as ::core::ffi::c_int
                                    {
                                        let mut _mfs_errorstring_35: *const ::core::ffi::c_char =
                                            strerr(*__errno_location());
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1083 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_17,
                                            *__errno_location(),
                                            _mfs_errorstring_35,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1083 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_17,
                                            *__errno_location(),
                                            _mfs_errorstring_35,
                                        );
                                    } else if _mfs_assert_ret_17 > 0 as ::core::ffi::c_int
                                        && *__errno_location() == 0 as ::core::ffi::c_int
                                    {
                                        let mut _mfs_errorstring_36: *const ::core::ffi::c_char =
                                            strerr(_mfs_assert_ret_17);
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1083 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_17,
                                            _mfs_errorstring_36,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1083 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_17,
                                            _mfs_errorstring_36,
                                        );
                                    } else {
                                        let mut _mfs_errorstring_err_17: *const ::core::ffi::c_char = strerr(
                                            *__errno_location(),
                                        );
                                        let mut _mfs_errorstring_ret_17: *const ::core::ffi::c_char = strerr(
                                            _mfs_assert_ret_17,
                                        );
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1083 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_17,
                                            _mfs_errorstring_ret_17,
                                            *__errno_location(),
                                            _mfs_errorstring_err_17,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1083 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_17,
                                            _mfs_errorstring_ret_17,
                                            *__errno_location(),
                                            _mfs_errorstring_err_17,
                                        );
                                    }
                                    abort();
                                }
                            }
                            nextwriteid = 1 as uint32_t;
                            pfd[0 as usize].fd = fd;
                            pfd[1 as usize].fd = pipefd[0 as usize];
                            rcvd = 0 as uint32_t;
                            sent = 0 as uint32_t;
                            waitforstatus = 1 as uint8_t;
                            wptr = &raw mut sendbuff as *mut uint8_t;
                            put32bit(&raw mut wptr, CLTOCS_WRITE as uint32_t);
                            if chainminver
                                >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                    + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                    + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                        32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                    } else {
                                        32 as ::core::ffi::c_int
                                    })) as uint32_t
                            {
                                put32bit(&raw mut wptr, (13 as uint32_t).wrapping_add(cschainsize));
                                put8bit(&raw mut wptr, 1 as uint8_t);
                                hdrtosend = 21 as uint32_t;
                            } else {
                                put32bit(&raw mut wptr, (12 as uint32_t).wrapping_add(cschainsize));
                                hdrtosend = 20 as uint32_t;
                            }
                            put64bit(&raw mut wptr, chunkid);
                            put32bit(&raw mut wptr, version);
                            sending_mode = 1 as uint8_t;
                            wants_pollout = 1 as uint8_t;
                            cb = ::core::ptr::null_mut::<cblock>();
                            status = 0 as ::core::ffi::c_int;
                            wrstatus = MFS_STATUS_OK as uint8_t;
                            lastrcvd = 0.0f64;
                            lastsent = 0.0f64;
                            lastblock = 0.0f64;
                            donotstayidle = 0 as uint8_t;
                            loop {
                                now = monotonic_seconds();
                                let mut _mfs_assert_ret_18: ::core::ffi::c_int =
                                    pthread_mutex_lock(&raw mut workerslock);
                                if _mfs_assert_ret_18 != 0 as ::core::ffi::c_int {
                                    if _mfs_assert_ret_18 < 0 as ::core::ffi::c_int
                                        && *__errno_location() != 0 as ::core::ffi::c_int
                                    {
                                        let mut _mfs_errorstring_37: *const ::core::ffi::c_char =
                                            strerr(*__errno_location());
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_18,
                                            *__errno_location(),
                                            _mfs_errorstring_37,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_18,
                                            *__errno_location(),
                                            _mfs_errorstring_37,
                                        );
                                    } else if _mfs_assert_ret_18 > 0 as ::core::ffi::c_int
                                        && *__errno_location() == 0 as ::core::ffi::c_int
                                    {
                                        let mut _mfs_errorstring_38: *const ::core::ffi::c_char =
                                            strerr(_mfs_assert_ret_18);
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_18,
                                            _mfs_errorstring_38,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_18,
                                            _mfs_errorstring_38,
                                        );
                                    } else {
                                        let mut _mfs_errorstring_err_18: *const ::core::ffi::c_char = strerr(
                                            *__errno_location(),
                                        );
                                        let mut _mfs_errorstring_ret_18: *const ::core::ffi::c_char = strerr(
                                            _mfs_assert_ret_18,
                                        );
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_18,
                                            _mfs_errorstring_ret_18,
                                            *__errno_location(),
                                            _mfs_errorstring_err_18,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_lock(&workerslock)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_18,
                                            _mfs_errorstring_ret_18,
                                            *__errno_location(),
                                            _mfs_errorstring_err_18,
                                        );
                                    }
                                    abort();
                                }
                                wtotal = workers_total;
                                let mut _mfs_assert_ret_19: ::core::ffi::c_int =
                                    pthread_mutex_unlock(&raw mut workerslock);
                                if _mfs_assert_ret_19 != 0 as ::core::ffi::c_int {
                                    if _mfs_assert_ret_19 < 0 as ::core::ffi::c_int
                                        && *__errno_location() != 0 as ::core::ffi::c_int
                                    {
                                        let mut _mfs_errorstring_39: *const ::core::ffi::c_char =
                                            strerr(*__errno_location());
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_19,
                                            *__errno_location(),
                                            _mfs_errorstring_39,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_19,
                                            *__errno_location(),
                                            _mfs_errorstring_39,
                                        );
                                    } else if _mfs_assert_ret_19 > 0 as ::core::ffi::c_int
                                        && *__errno_location() == 0 as ::core::ffi::c_int
                                    {
                                        let mut _mfs_errorstring_40: *const ::core::ffi::c_char =
                                            strerr(_mfs_assert_ret_19);
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_19,
                                            _mfs_errorstring_40,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_19,
                                            _mfs_errorstring_40,
                                        );
                                    } else {
                                        let mut _mfs_errorstring_err_19: *const ::core::ffi::c_char = strerr(
                                            *__errno_location(),
                                        );
                                        let mut _mfs_errorstring_ret_19: *const ::core::ffi::c_char = strerr(
                                            _mfs_assert_ret_19,
                                        );
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_19,
                                            _mfs_errorstring_ret_19,
                                            *__errno_location(),
                                            _mfs_errorstring_err_19,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&workerslock)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_19,
                                            _mfs_errorstring_ret_19,
                                            *__errno_location(),
                                            _mfs_errorstring_err_19,
                                        );
                                    }
                                    abort();
                                }
                                if optimeout > 0.0f64 && now - opbegin > optimeout {
                                    status = EIO;
                                    break;
                                } else {
                                    let mut _mfs_assert_ret_20: ::core::ffi::c_int =
                                        pthread_mutex_lock(&raw mut (*ind).lock);
                                    if _mfs_assert_ret_20 != 0 as ::core::ffi::c_int {
                                        if _mfs_assert_ret_20 < 0 as ::core::ffi::c_int
                                            && *__errno_location() != 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_41: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_20,
                                                *__errno_location(),
                                                _mfs_errorstring_41,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_20,
                                                *__errno_location(),
                                                _mfs_errorstring_41,
                                            );
                                        } else if _mfs_assert_ret_20 > 0 as ::core::ffi::c_int
                                            && *__errno_location() == 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_42: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_20,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_20,
                                                _mfs_errorstring_42,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_20,
                                                _mfs_errorstring_42,
                                            );
                                        } else {
                                            let mut _mfs_errorstring_err_20: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            let mut _mfs_errorstring_ret_20: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_20,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_20,
                                                _mfs_errorstring_ret_20,
                                                *__errno_location(),
                                                _mfs_errorstring_err_20,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_20,
                                                _mfs_errorstring_ret_20,
                                                *__errno_location(),
                                                _mfs_errorstring_err_20,
                                            );
                                        }
                                        abort();
                                    }
                                    if lastrcvd == 0.0f64 {
                                        lastrcvd = now;
                                    } else {
                                        lrdiff = now - lastrcvd;
                                        if lrdiff >= CHUNKSERVER_ACTIVITY_TIMEOUT {
                                            if (*chd).trycnt as uint32_t >= minlogretry {
                                                univmakestrip(
                                                    &raw mut csstrip as *mut ::core::ffi::c_char,
                                                    ip,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_WARNING,
                                                    b"file: %u, index: %u, chunk: %016lX, version: %u - writeworker: connection with (%s:%hu) was timed out (unfinished writes: %hhu; try counter: %u)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    inode,
                                                    chindx,
                                                    chunkid,
                                                    version,
                                                    &raw mut csstrip as *mut ::core::ffi::c_char,
                                                    port as ::core::ffi::c_int,
                                                    waitforstatus as ::core::ffi::c_int,
                                                    (*chd).trycnt as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int,
                                                );
                                            }
                                            let mut _mfs_assert_ret_21: ::core::ffi::c_int =
                                                pthread_mutex_unlock(&raw mut (*ind).lock);
                                            if _mfs_assert_ret_21 != 0 as ::core::ffi::c_int {
                                                if _mfs_assert_ret_21 < 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        != 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_43: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_21,
                                                        *__errno_location(),
                                                        _mfs_errorstring_43,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_21,
                                                        *__errno_location(),
                                                        _mfs_errorstring_43,
                                                    );
                                                } else if _mfs_assert_ret_21
                                                    > 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        == 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_44: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_21,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_21,
                                                        _mfs_errorstring_44,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_21,
                                                        _mfs_errorstring_44,
                                                    );
                                                } else {
                                                    let mut _mfs_errorstring_err_21: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    let mut _mfs_errorstring_ret_21: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_21,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_21,
                                                        _mfs_errorstring_ret_21,
                                                        *__errno_location(),
                                                        _mfs_errorstring_err_21,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_21,
                                                        _mfs_errorstring_ret_21,
                                                        *__errno_location(),
                                                        _mfs_errorstring_err_21,
                                                    );
                                                }
                                                abort();
                                            }
                                            break;
                                        }
                                    }
                                    if lastblock == 0.0f64 {
                                        lbdiff = NEXT_BLOCK_DELAY;
                                    } else {
                                        lbdiff = now - lastblock;
                                    }
                                    workingtime = now - start;
                                    (*chd).waitingworker = 1 as uint8_t;
                                    (*chd).wakeup_fd = pipefd[1 as usize];
                                    if lastsent == 0.0f64 {
                                        lastsent = now;
                                    }
                                    if sending_mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                        && lastsent + WORKER_NOP_INTERVAL < now
                                        && chainminver
                                            >= (1 as ::core::ffi::c_int
                                                * 0x10000 as ::core::ffi::c_int
                                                + 7 as ::core::ffi::c_int
                                                    * 0x100 as ::core::ffi::c_int
                                                + (if 1 as ::core::ffi::c_int
                                                    > 1 as ::core::ffi::c_int
                                                {
                                                    32 as ::core::ffi::c_int
                                                        * 2 as ::core::ffi::c_int
                                                } else {
                                                    32 as ::core::ffi::c_int
                                                }))
                                                as uint32_t
                                    {
                                        wptr = &raw mut sendbuff as *mut uint8_t;
                                        put32bit(&raw mut wptr, ANTOAN_NOP as uint32_t);
                                        put32bit(&raw mut wptr, 0 as uint32_t);
                                        sent = 0 as uint32_t;
                                        sending_mode = 3 as uint8_t;
                                        lastsent = now;
                                    }
                                    if sending_mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                        && workingtime
                                            < WORKER_BUSY_LAST_SEND_TIMEOUT
                                                + (if wtotal > HEAVYLOAD_WORKERS as uint32_t {
                                                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                                                } else {
                                                    WORKER_BUSY_NOJOBS_INCREASE_TIMEOUT
                                                })
                                        && (waitforstatus as ::core::ffi::c_int)
                                            < 64 as ::core::ffi::c_int
                                    {
                                        if cb.is_null() {
                                            ncb = (*chd).datachainhead;
                                        } else {
                                            ncb = (*cb).next as *mut cblock;
                                        }
                                        if !ncb.is_null() {
                                            if (*ncb).to.wrapping_sub((*ncb).from)
                                                == MFSBLOCKSIZE as uint32_t
                                                || lbdiff >= NEXT_BLOCK_DELAY
                                                || !(*ncb).next.is_null()
                                                || (*ind).flushwaiting as ::core::ffi::c_int != 0
                                            {
                                                cb = ncb;
                                                sending_mode = 2 as uint8_t;
                                            } else {
                                                (*chd).waitingworker = 2 as uint8_t;
                                                (*chd).wakeup_fd = pipefd[1 as usize];
                                            }
                                        }
                                        if sending_mode as ::core::ffi::c_int
                                            == 2 as ::core::ffi::c_int
                                        {
                                            let mut offset_from: uint64_t = 0;
                                            let mut offset_to: uint64_t = 0;
                                            let mut chunk_offset_from: uint32_t = 0;
                                            let mut chunk_offset_to: uint32_t = 0;
                                            let c2rust_fresh0 = nextwriteid;
                                            nextwriteid = nextwriteid.wrapping_add(1);
                                            (*cb).writeid = c2rust_fresh0;
                                            waitforstatus = waitforstatus.wrapping_add(1);
                                            wptr = &raw mut sendbuff as *mut uint8_t;
                                            put32bit(&raw mut wptr, CLTOCS_WRITE_DATA as uint32_t);
                                            put32bit(
                                                &raw mut wptr,
                                                (24 as uint32_t).wrapping_add(
                                                    (*cb).to.wrapping_sub((*cb).from),
                                                ),
                                            );
                                            put64bit(&raw mut wptr, chunkid);
                                            put32bit(&raw mut wptr, (*cb).writeid);
                                            put16bit(&raw mut wptr, (*cb).pos);
                                            put16bit(&raw mut wptr, (*cb).from as uint16_t);
                                            put32bit(
                                                &raw mut wptr,
                                                (*cb).to.wrapping_sub((*cb).from),
                                            );
                                            put32bit(
                                                &raw mut wptr,
                                                mycrc32(
                                                    0 as uint32_t,
                                                    (&raw mut (*cb).data as *mut uint8_t)
                                                        .offset((*cb).from as isize)
                                                        as *const ::core::ffi::c_void,
                                                    (*cb).to.wrapping_sub((*cb).from),
                                                ),
                                            );
                                            sent = 0 as uint32_t;
                                            lastblock = now;
                                            lastsent = now;
                                            offset_from = chindx as uint64_t;
                                            offset_from <<= MFSCHUNKBITS;
                                            offset_to = offset_from;
                                            chunk_offset_from = ((*cb).pos as ::core::ffi::c_int
                                                * MFSBLOCKSIZE)
                                                as uint32_t;
                                            chunk_offset_to = chunk_offset_from;
                                            chunk_offset_from =
                                                chunk_offset_from.wrapping_add((*cb).from);
                                            chunk_offset_to =
                                                chunk_offset_to.wrapping_add((*cb).to);
                                            offset_from = offset_from
                                                .wrapping_add(chunk_offset_from as uint64_t);
                                            offset_to =
                                                offset_to.wrapping_add(chunk_offset_to as uint64_t);
                                            if valid_offsets != 0 {
                                                if offset_from < min_offset {
                                                    min_offset = offset_from;
                                                }
                                                if offset_to > max_offset {
                                                    max_offset = offset_to;
                                                }
                                                if chunk_offset_from < min_from {
                                                    min_from = chunk_offset_from;
                                                }
                                                if chunk_offset_to > max_to {
                                                    max_to = chunk_offset_to;
                                                }
                                            } else {
                                                min_offset = offset_from;
                                                max_offset = offset_to;
                                                min_from = chunk_offset_from;
                                                max_to = chunk_offset_to;
                                                valid_offsets = 1 as uint8_t;
                                            }
                                        }
                                    }
                                    if waitforstatus as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                                    {
                                        if workingtime
                                            > WORKER_BUSY_LAST_SEND_TIMEOUT
                                                + WORKER_BUSY_WAIT_FOR_STATUS
                                                + (if wtotal > HEAVYLOAD_WORKERS as uint32_t {
                                                    0 as ::core::ffi::c_int as ::core::ffi::c_double
                                                } else {
                                                    WORKER_BUSY_NOJOBS_INCREASE_TIMEOUT
                                                })
                                        {
                                            (*chd).waitingworker = 0 as uint8_t;
                                            (*chd).wakeup_fd = -1 as ::core::ffi::c_int;
                                            if (*chd).trycnt as uint32_t >= minlogretry {
                                                univmakestrip(
                                                    &raw mut csstrip as *mut ::core::ffi::c_char,
                                                    ip,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_WARNING,
                                                    b"file: %u, index: %u, chunk: %016lX, version: %u - writeworker: connection with (%s:%hu) was busy too long (unfinished writes: %hhu; try counter: %u)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    inode,
                                                    chindx,
                                                    chunkid,
                                                    version,
                                                    &raw mut csstrip as *mut ::core::ffi::c_char,
                                                    port as ::core::ffi::c_int,
                                                    waitforstatus as ::core::ffi::c_int,
                                                    (*chd).trycnt as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int,
                                                );
                                            }
                                            let mut _mfs_assert_ret_22: ::core::ffi::c_int =
                                                pthread_mutex_unlock(&raw mut (*ind).lock);
                                            if _mfs_assert_ret_22 != 0 as ::core::ffi::c_int {
                                                if _mfs_assert_ret_22 < 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        != 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_45: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1263 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_22,
                                                        *__errno_location(),
                                                        _mfs_errorstring_45,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1263 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_22,
                                                        *__errno_location(),
                                                        _mfs_errorstring_45,
                                                    );
                                                } else if _mfs_assert_ret_22
                                                    > 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        == 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_46: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_22,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1263 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_22,
                                                        _mfs_errorstring_46,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1263 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_22,
                                                        _mfs_errorstring_46,
                                                    );
                                                } else {
                                                    let mut _mfs_errorstring_err_22: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    let mut _mfs_errorstring_ret_22: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_22,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1263 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_22,
                                                        _mfs_errorstring_ret_22,
                                                        *__errno_location(),
                                                        _mfs_errorstring_err_22,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        1263 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_22,
                                                        _mfs_errorstring_ret_22,
                                                        *__errno_location(),
                                                        _mfs_errorstring_err_22,
                                                    );
                                                }
                                                abort();
                                            }
                                            break;
                                        }
                                    } else if lbdiff >= WORKER_IDLE_TIMEOUT
                                        || donotstayidle as ::core::ffi::c_int != 0
                                        || wtotal > HEAVYLOAD_WORKERS as uint32_t
                                    {
                                        (*chd).waitingworker = 0 as uint8_t;
                                        (*chd).wakeup_fd = -1 as ::core::ffi::c_int;
                                        let mut _mfs_assert_ret_23: ::core::ffi::c_int =
                                            pthread_mutex_unlock(&raw mut (*ind).lock);
                                        if _mfs_assert_ret_23 != 0 as ::core::ffi::c_int {
                                            if _mfs_assert_ret_23 < 0 as ::core::ffi::c_int
                                                && *__errno_location() != 0 as ::core::ffi::c_int
                                            {
                                                let mut _mfs_errorstring_47: *const ::core::ffi::c_char = strerr(
                                                    *__errno_location(),
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    1270 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_23,
                                                    *__errno_location(),
                                                    _mfs_errorstring_47,
                                                );
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    1270 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_23,
                                                    *__errno_location(),
                                                    _mfs_errorstring_47,
                                                );
                                            } else if _mfs_assert_ret_23 > 0 as ::core::ffi::c_int
                                                && *__errno_location() == 0 as ::core::ffi::c_int
                                            {
                                                let mut _mfs_errorstring_48: *const ::core::ffi::c_char = strerr(
                                                    _mfs_assert_ret_23,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    1270 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_23,
                                                    _mfs_errorstring_48,
                                                );
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    1270 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_23,
                                                    _mfs_errorstring_48,
                                                );
                                            } else {
                                                let mut _mfs_errorstring_err_23: *const ::core::ffi::c_char = strerr(
                                                    *__errno_location(),
                                                );
                                                let mut _mfs_errorstring_ret_23: *const ::core::ffi::c_char = strerr(
                                                    _mfs_assert_ret_23,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    1270 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_23,
                                                    _mfs_errorstring_ret_23,
                                                    *__errno_location(),
                                                    _mfs_errorstring_err_23,
                                                );
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    1270 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_23,
                                                    _mfs_errorstring_ret_23,
                                                    *__errno_location(),
                                                    _mfs_errorstring_err_23,
                                                );
                                            }
                                            abort();
                                        }
                                        break;
                                    }
                                    let mut _mfs_assert_ret_24: ::core::ffi::c_int =
                                        pthread_mutex_unlock(&raw mut (*ind).lock);
                                    if _mfs_assert_ret_24 != 0 as ::core::ffi::c_int {
                                        if _mfs_assert_ret_24 < 0 as ::core::ffi::c_int
                                            && *__errno_location() != 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_49: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1276 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_24,
                                                *__errno_location(),
                                                _mfs_errorstring_49,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1276 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_24,
                                                *__errno_location(),
                                                _mfs_errorstring_49,
                                            );
                                        } else if _mfs_assert_ret_24 > 0 as ::core::ffi::c_int
                                            && *__errno_location() == 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_50: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_24,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1276 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_24,
                                                _mfs_errorstring_50,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1276 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_24,
                                                _mfs_errorstring_50,
                                            );
                                        } else {
                                            let mut _mfs_errorstring_err_24: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            let mut _mfs_errorstring_ret_24: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_24,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1276 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_24,
                                                _mfs_errorstring_ret_24,
                                                *__errno_location(),
                                                _mfs_errorstring_err_24,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1276 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_24,
                                                _mfs_errorstring_ret_24,
                                                *__errno_location(),
                                                _mfs_errorstring_err_24,
                                            );
                                        }
                                        abort();
                                    }
                                    wants_pollout = (if sending_mode as ::core::ffi::c_int != 0 {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    })
                                        as uint8_t;
                                    match sending_mode as ::core::ffi::c_int {
                                        1 => {
                                            if sent < hdrtosend {
                                                if cschainsize > 0 as uint32_t {
                                                    siov[0 as usize].iov_base = (&raw mut sendbuff
                                                        as *mut uint8_t)
                                                        .offset(sent as isize)
                                                        as *mut ::core::ffi::c_void;
                                                    siov[0 as usize].iov_len =
                                                        hdrtosend.wrapping_sub(sent) as size_t;
                                                    siov[1 as usize].iov_base = &raw mut cschain
                                                        as *mut uint8_t
                                                        as *mut ::core::ffi::c_void;
                                                    siov[1 as usize].iov_len =
                                                        cschainsize as size_t;
                                                    i = writev(
                                                        fd,
                                                        &raw mut siov as *mut iovec,
                                                        2 as ::core::ffi::c_int,
                                                    )
                                                        as ::core::ffi::c_int;
                                                } else {
                                                    i = write(
                                                        fd,
                                                        (&raw mut sendbuff as *mut uint8_t)
                                                            .offset(sent as isize)
                                                            as *const ::core::ffi::c_void,
                                                        hdrtosend.wrapping_sub(sent) as size_t,
                                                    )
                                                        as ::core::ffi::c_int;
                                                }
                                            } else {
                                                i = write(
                                                    fd,
                                                    (&raw mut cschain as *mut uint8_t).offset(
                                                        sent.wrapping_sub(hdrtosend) as isize,
                                                    )
                                                        as *const ::core::ffi::c_void,
                                                    cschainsize
                                                        .wrapping_sub(sent.wrapping_sub(hdrtosend))
                                                        as size_t,
                                                )
                                                    as ::core::ffi::c_int;
                                            }
                                            if i >= 0 as ::core::ffi::c_int {
                                                sent = sent.wrapping_add(i as uint32_t);
                                                if sent == hdrtosend.wrapping_add(cschainsize) {
                                                    sending_mode = 0 as uint8_t;
                                                }
                                            }
                                        }
                                        2 => {
                                            if sent < 32 as uint32_t {
                                                siov[0 as usize].iov_base = (&raw mut sendbuff
                                                    as *mut uint8_t)
                                                    .offset(sent as isize)
                                                    as *mut ::core::ffi::c_void;
                                                siov[0 as usize].iov_len =
                                                    (32 as uint32_t).wrapping_sub(sent) as size_t;
                                                siov[1 as usize].iov_base = (&raw mut (*cb).data
                                                    as *mut uint8_t)
                                                    .offset((*cb).from as isize)
                                                    as *mut ::core::ffi::c_void;
                                                siov[1 as usize].iov_len =
                                                    (*cb).to.wrapping_sub((*cb).from) as size_t;
                                                i = writev(
                                                    fd,
                                                    &raw mut siov as *mut iovec,
                                                    2 as ::core::ffi::c_int,
                                                )
                                                    as ::core::ffi::c_int;
                                            } else {
                                                i = write(
                                                    fd,
                                                    (&raw mut (*cb).data as *mut uint8_t)
                                                        .offset((*cb).from as isize)
                                                        .offset(sent.wrapping_sub(32 as uint32_t)
                                                            as isize)
                                                        as *const ::core::ffi::c_void,
                                                    (*cb).to.wrapping_sub((*cb).from).wrapping_sub(
                                                        sent.wrapping_sub(32 as uint32_t),
                                                    )
                                                        as size_t,
                                                )
                                                    as ::core::ffi::c_int;
                                            }
                                            if i >= 0 as ::core::ffi::c_int {
                                                sent = sent.wrapping_add(i as uint32_t);
                                                if sent
                                                    == (32 as uint32_t)
                                                        .wrapping_add((*cb).to)
                                                        .wrapping_sub((*cb).from)
                                                {
                                                    sending_mode = 0 as uint8_t;
                                                    write_increase_total_bytes(
                                                        (*cb).to.wrapping_sub((*cb).from),
                                                    );
                                                }
                                            }
                                        }
                                        3 => {
                                            i = write(
                                                fd,
                                                (&raw mut sendbuff as *mut uint8_t)
                                                    .offset(sent as isize)
                                                    as *const ::core::ffi::c_void,
                                                (8 as uint32_t).wrapping_sub(sent) as size_t,
                                            )
                                                as ::core::ffi::c_int;
                                            if i >= 0 as ::core::ffi::c_int {
                                                sent = sent.wrapping_add(i as uint32_t);
                                                if sent == 8 as uint32_t {
                                                    sending_mode = 0 as uint8_t;
                                                }
                                            }
                                        }
                                        _ => {
                                            i = 0 as ::core::ffi::c_int;
                                        }
                                    }
                                    if i < 0 as ::core::ffi::c_int {
                                        if *__errno_location() != EAGAIN
                                            && *__errno_location() != EWOULDBLOCK
                                            && *__errno_location() != EINTR
                                        {
                                            if (*chd).trycnt as uint32_t >= minlogretry {
                                                univmakestrip(
                                                    &raw mut csstrip as *mut ::core::ffi::c_char,
                                                    ip,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_WARNING,
                                                    b"file: %u, index: %u, chunk: %016lX, version: %u - writeworker: write to (%s:%hu) error: %s / NEGWRITE (unfinished writes: %hhu; try counter: %u)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    inode,
                                                    chindx,
                                                    chunkid,
                                                    version,
                                                    &raw mut csstrip as *mut ::core::ffi::c_char,
                                                    port as ::core::ffi::c_int,
                                                    strerr(*__errno_location()),
                                                    waitforstatus as ::core::ffi::c_int,
                                                    (*chd).trycnt as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int,
                                                );
                                            }
                                            status = EIO;
                                            break;
                                        }
                                    }
                                    pfd[0 as usize].events = (POLLIN
                                        | (if wants_pollout as ::core::ffi::c_int != 0 {
                                            POLLOUT
                                        } else {
                                            0 as ::core::ffi::c_int
                                        }))
                                        as ::core::ffi::c_short;
                                    pfd[0 as usize].revents = 0 as ::core::ffi::c_short;
                                    pfd[1 as usize].events = POLLIN as ::core::ffi::c_short;
                                    pfd[1 as usize].revents = 0 as ::core::ffi::c_short;
                                    if poll(
                                        &raw mut pfd as *mut pollfd,
                                        2 as nfds_t,
                                        100 as ::core::ffi::c_int,
                                    ) < 0 as ::core::ffi::c_int
                                    {
                                        if *__errno_location() != EINTR {
                                            if (*chd).trycnt as uint32_t >= minlogretry {
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_WARNING,
                                                    b"file: %u, index: %u, chunk: %016lX, version: %u - writeworker: poll error: %s (unfinished writes: %hhu; try counter: %u)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    inode,
                                                    chindx,
                                                    chunkid,
                                                    version,
                                                    strerr(*__errno_location()),
                                                    waitforstatus as ::core::ffi::c_int,
                                                    (*chd).trycnt as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int,
                                                );
                                            }
                                            status = EIO;
                                            break;
                                        }
                                    }
                                    let mut _mfs_assert_ret_25: ::core::ffi::c_int =
                                        pthread_mutex_lock(&raw mut (*ind).lock);
                                    if _mfs_assert_ret_25 != 0 as ::core::ffi::c_int {
                                        if _mfs_assert_ret_25 < 0 as ::core::ffi::c_int
                                            && *__errno_location() != 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_51: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_25,
                                                *__errno_location(),
                                                _mfs_errorstring_51,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_25,
                                                *__errno_location(),
                                                _mfs_errorstring_51,
                                            );
                                        } else if _mfs_assert_ret_25 > 0 as ::core::ffi::c_int
                                            && *__errno_location() == 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_52: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_25,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_25,
                                                _mfs_errorstring_52,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_25,
                                                _mfs_errorstring_52,
                                            );
                                        } else {
                                            let mut _mfs_errorstring_err_25: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            let mut _mfs_errorstring_ret_25: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_25,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_25,
                                                _mfs_errorstring_ret_25,
                                                *__errno_location(),
                                                _mfs_errorstring_err_25,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_25,
                                                _mfs_errorstring_ret_25,
                                                *__errno_location(),
                                                _mfs_errorstring_err_25,
                                            );
                                        }
                                        abort();
                                    }
                                    (*chd).waitingworker = 0 as uint8_t;
                                    (*chd).wakeup_fd = -1 as ::core::ffi::c_int;
                                    donotstayidle = (if (*ind).flushwaiting as ::core::ffi::c_int
                                        > 0 as ::core::ffi::c_int
                                        || (*ind).status != 0 as ::core::ffi::c_int
                                        || (*ind).chunkscnt as ::core::ffi::c_int >= MAX_SIM_CHUNKS
                                    {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    })
                                        as uint8_t;
                                    let mut _mfs_assert_ret_26: ::core::ffi::c_int =
                                        pthread_mutex_unlock(&raw mut (*ind).lock);
                                    if _mfs_assert_ret_26 != 0 as ::core::ffi::c_int {
                                        if _mfs_assert_ret_26 < 0 as ::core::ffi::c_int
                                            && *__errno_location() != 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_53: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1369 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_26,
                                                *__errno_location(),
                                                _mfs_errorstring_53,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1369 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_26,
                                                *__errno_location(),
                                                _mfs_errorstring_53,
                                            );
                                        } else if _mfs_assert_ret_26 > 0 as ::core::ffi::c_int
                                            && *__errno_location() == 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_54: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_26,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1369 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_26,
                                                _mfs_errorstring_54,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1369 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_26,
                                                _mfs_errorstring_54,
                                            );
                                        } else {
                                            let mut _mfs_errorstring_err_26: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            let mut _mfs_errorstring_ret_26: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_26,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1369 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_26,
                                                _mfs_errorstring_ret_26,
                                                *__errno_location(),
                                                _mfs_errorstring_err_26,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1369 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_26,
                                                _mfs_errorstring_ret_26,
                                                *__errno_location(),
                                                _mfs_errorstring_err_26,
                                            );
                                        }
                                        abort();
                                    }
                                    if pfd[1 as usize].revents as ::core::ffi::c_int & POLLIN != 0 {
                                        i = read(
                                            pipefd[0 as usize],
                                            &raw mut pipebuff as *mut uint8_t
                                                as *mut ::core::ffi::c_void,
                                            1024 as size_t,
                                        )
                                            as ::core::ffi::c_int;
                                        if i < 0 as ::core::ffi::c_int {
                                            if (*chd).trycnt as uint32_t >= minlogretry {
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_WARNING,
                                                    b"file: %u, index: %u, chunk: %016lX, version: %u - writeworker: read pipe error: %s (unfinished writes: %hhu; try counter: %u)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    inode,
                                                    chindx,
                                                    chunkid,
                                                    version,
                                                    strerr(*__errno_location()),
                                                    waitforstatus as ::core::ffi::c_int,
                                                    (*chd).trycnt as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int,
                                                );
                                            }
                                        }
                                    }
                                    if pfd[0 as usize].revents as ::core::ffi::c_int & POLLHUP != 0
                                    {
                                        if (*chd).trycnt as uint32_t >= minlogretry {
                                            univmakestrip(
                                                &raw mut csstrip as *mut ::core::ffi::c_char,
                                                ip,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"file: %u, index: %u, chunk: %016lX, version: %u - writeworker: connection with (%s:%hu) was reset by peer / POLLHUP (unfinished writes: %hhu; try counter: %u)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                inode,
                                                chindx,
                                                chunkid,
                                                version,
                                                &raw mut csstrip as *mut ::core::ffi::c_char,
                                                port as ::core::ffi::c_int,
                                                waitforstatus as ::core::ffi::c_int,
                                                (*chd).trycnt as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int,
                                            );
                                        }
                                        status = EIO;
                                        break;
                                    } else if pfd[0 as usize].revents as ::core::ffi::c_int
                                        & POLLERR
                                        != 0
                                    {
                                        if (*chd).trycnt as uint32_t >= minlogretry {
                                            univmakestrip(
                                                &raw mut csstrip as *mut ::core::ffi::c_char,
                                                ip,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"file: %u, index: %u, chunk: %016lX, version: %u - writeworker: connection with (%s:%hu) got error status / POLLERR (unfinished writes: %hhu; try counter: %u)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                inode,
                                                chindx,
                                                chunkid,
                                                version,
                                                &raw mut csstrip as *mut ::core::ffi::c_char,
                                                port as ::core::ffi::c_int,
                                                waitforstatus as ::core::ffi::c_int,
                                                (*chd).trycnt as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int,
                                            );
                                        }
                                        status = EIO;
                                        break;
                                    } else {
                                        if pfd[0 as usize].revents as ::core::ffi::c_int & POLLIN
                                            == 0
                                        {
                                            continue;
                                        }
                                        i = read(
                                            fd,
                                            (&raw mut recvbuff as *mut uint8_t)
                                                .offset(rcvd as isize)
                                                as *mut ::core::ffi::c_void,
                                            (21 as uint32_t).wrapping_sub(rcvd) as size_t,
                                        )
                                            as ::core::ffi::c_int;
                                        if i == 0 as ::core::ffi::c_int {
                                            if (*chd).trycnt as uint32_t >= minlogretry {
                                                univmakestrip(
                                                    &raw mut csstrip as *mut ::core::ffi::c_char,
                                                    ip,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_WARNING,
                                                    b"file: %u, index: %u, chunk: %016lX, version: %u - writeworker: connection with (%s:%hu) was reset by peer / ZEROREAD (unfinished writes: %hhu; try counter: %u)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    inode,
                                                    chindx,
                                                    chunkid,
                                                    version,
                                                    &raw mut csstrip as *mut ::core::ffi::c_char,
                                                    port as ::core::ffi::c_int,
                                                    waitforstatus as ::core::ffi::c_int,
                                                    (*chd).trycnt as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int,
                                                );
                                            }
                                            status = EIO;
                                            break;
                                        } else {
                                            if i < 0 as ::core::ffi::c_int {
                                                if *__errno_location() != EINTR {
                                                    if (*chd).trycnt as uint32_t >= minlogretry {
                                                        univmakestrip(
                                                            &raw mut csstrip
                                                                as *mut ::core::ffi::c_char,
                                                            ip,
                                                        );
                                                        mfs_log(
                                                            MFSLOG_SYSLOG,
                                                            MFSLOG_WARNING,
                                                            b"file: %u, index: %u, chunk: %016lX, version: %u - writeworker: read from (%s:%hu) error: %s (unfinished writes: %hhu; try counter: %u)\0"
                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                            inode,
                                                            chindx,
                                                            chunkid,
                                                            version,
                                                            &raw mut csstrip as *mut ::core::ffi::c_char,
                                                            port as ::core::ffi::c_int,
                                                            strerr(*__errno_location()),
                                                            waitforstatus as ::core::ffi::c_int,
                                                            (*chd).trycnt as ::core::ffi::c_int
                                                                + 1 as ::core::ffi::c_int,
                                                        );
                                                    }
                                                    status = EIO;
                                                    break;
                                                } else {
                                                    i = 0 as ::core::ffi::c_int;
                                                }
                                            }
                                            lastrcvd = monotonic_seconds();
                                            rcvd = rcvd.wrapping_add(i as uint32_t);
                                            if rcvd >= 8 as uint32_t
                                                && recvbuff[7 as usize] as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                                && recvbuff[6 as usize] as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                                && recvbuff[5 as usize] as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                                && recvbuff[4 as usize] as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                                && recvbuff[3 as usize] as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                                && recvbuff[2 as usize] as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                                && recvbuff[1 as usize] as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                                && recvbuff[0 as usize] as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                            {
                                                if rcvd > 8 as uint32_t {
                                                    memmove(
                                                        &raw mut recvbuff as *mut uint8_t
                                                            as *mut ::core::ffi::c_void,
                                                        (&raw mut recvbuff as *mut uint8_t).offset(
                                                            8 as ::core::ffi::c_int as isize,
                                                        )
                                                            as *const ::core::ffi::c_void,
                                                        rcvd.wrapping_sub(8 as uint32_t) as size_t,
                                                    );
                                                    rcvd = rcvd.wrapping_sub(8 as uint32_t);
                                                }
                                            }
                                            if rcvd != 21 as uint32_t {
                                                continue;
                                            }
                                            rptr = &raw mut recvbuff as *mut uint8_t;
                                            reccmd = get32bit(&raw mut rptr);
                                            recleng = get32bit(&raw mut rptr);
                                            recchunkid = get64bit(&raw mut rptr);
                                            recwriteid = get32bit(&raw mut rptr);
                                            recstatus = get8bit(&raw mut rptr);
                                            if reccmd != CSTOCL_WRITE_STATUS as uint32_t
                                                || recleng != 13 as uint32_t
                                            {
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_WARNING,
                                                    b"writeworker: got unrecognized packet from chunkserver (cmd:%u,leng:%u)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    reccmd,
                                                    recleng,
                                                );
                                                status = EIO;
                                                break;
                                            } else if recchunkid != chunkid {
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_WARNING,
                                                    b"writeworker: got unexpected packet (expected chunkdid:%016lX,packet chunkid:%016lX)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    chunkid,
                                                    recchunkid,
                                                );
                                                status = EIO;
                                                break;
                                            } else if recstatus as ::core::ffi::c_int
                                                != MFS_STATUS_OK
                                            {
                                                if (*chd).trycnt as uint32_t >= minlogretry {
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_WARNING,
                                                        b"writeworker: write error: %s\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        mfsstrerr(recstatus),
                                                    );
                                                }
                                                wrstatus = recstatus;
                                                break;
                                            } else {
                                                if recwriteid > 0 as uint32_t {
                                                    let mut _mfs_assert_ret_27: ::core::ffi::c_int =
                                                        pthread_mutex_lock(&raw mut (*ind).lock);
                                                    if _mfs_assert_ret_27 != 0 as ::core::ffi::c_int
                                                    {
                                                        if _mfs_assert_ret_27
                                                            < 0 as ::core::ffi::c_int
                                                            && *__errno_location()
                                                                != 0 as ::core::ffi::c_int
                                                        {
                                                            let mut _mfs_errorstring_55: *const ::core::ffi::c_char = strerr(
                                                                *__errno_location(),
                                                            );
                                                            mfs_log(
                                                                MFSLOG_SYSLOG,
                                                                MFSLOG_ERR,
                                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                1451 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                _mfs_assert_ret_27,
                                                                *__errno_location(),
                                                                _mfs_errorstring_55,
                                                            );
                                                            fprintf(
                                                                stderr,
                                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                1451 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                _mfs_assert_ret_27,
                                                                *__errno_location(),
                                                                _mfs_errorstring_55,
                                                            );
                                                        } else if _mfs_assert_ret_27
                                                            > 0 as ::core::ffi::c_int
                                                            && *__errno_location()
                                                                == 0 as ::core::ffi::c_int
                                                        {
                                                            let mut _mfs_errorstring_56: *const ::core::ffi::c_char = strerr(
                                                                _mfs_assert_ret_27,
                                                            );
                                                            mfs_log(
                                                                MFSLOG_SYSLOG,
                                                                MFSLOG_ERR,
                                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                1451 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                _mfs_assert_ret_27,
                                                                _mfs_errorstring_56,
                                                            );
                                                            fprintf(
                                                                stderr,
                                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                1451 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                _mfs_assert_ret_27,
                                                                _mfs_errorstring_56,
                                                            );
                                                        } else {
                                                            let mut _mfs_errorstring_err_27: *const ::core::ffi::c_char = strerr(
                                                                *__errno_location(),
                                                            );
                                                            let mut _mfs_errorstring_ret_27: *const ::core::ffi::c_char = strerr(
                                                                _mfs_assert_ret_27,
                                                            );
                                                            mfs_log(
                                                                MFSLOG_SYSLOG,
                                                                MFSLOG_ERR,
                                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                1451 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                _mfs_assert_ret_27,
                                                                _mfs_errorstring_ret_27,
                                                                *__errno_location(),
                                                                _mfs_errorstring_err_27,
                                                            );
                                                            fprintf(
                                                                stderr,
                                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                1451 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                _mfs_assert_ret_27,
                                                                _mfs_errorstring_ret_27,
                                                                *__errno_location(),
                                                                _mfs_errorstring_err_27,
                                                            );
                                                        }
                                                        abort();
                                                    }
                                                    rcb = (*chd).datachainhead;
                                                    while !rcb.is_null()
                                                        && (*rcb).writeid != recwriteid
                                                    {
                                                        rcb = (*rcb).next as *mut cblock;
                                                    }
                                                    if rcb.is_null() {
                                                        mfs_log(
                                                            MFSLOG_SYSLOG,
                                                            MFSLOG_WARNING,
                                                            b"writeworker: got unexpected status (writeid:%u)\0"
                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                            recwriteid,
                                                        );
                                                        let mut _mfs_assert_ret_28: ::core::ffi::c_int = pthread_mutex_unlock(
                                                            &raw mut (*ind).lock,
                                                        );
                                                        if _mfs_assert_ret_28
                                                            != 0 as ::core::ffi::c_int
                                                        {
                                                            if _mfs_assert_ret_28
                                                                < 0 as ::core::ffi::c_int
                                                                && *__errno_location()
                                                                    != 0 as ::core::ffi::c_int
                                                            {
                                                                let mut _mfs_errorstring_57: *const ::core::ffi::c_char = strerr(
                                                                    *__errno_location(),
                                                                );
                                                                mfs_log(
                                                                    MFSLOG_SYSLOG,
                                                                    MFSLOG_ERR,
                                                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    1455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    _mfs_assert_ret_28,
                                                                    *__errno_location(),
                                                                    _mfs_errorstring_57,
                                                                );
                                                                fprintf(
                                                                    stderr,
                                                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    1455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    _mfs_assert_ret_28,
                                                                    *__errno_location(),
                                                                    _mfs_errorstring_57,
                                                                );
                                                            } else if _mfs_assert_ret_28
                                                                > 0 as ::core::ffi::c_int
                                                                && *__errno_location()
                                                                    == 0 as ::core::ffi::c_int
                                                            {
                                                                let mut _mfs_errorstring_58: *const ::core::ffi::c_char = strerr(
                                                                    _mfs_assert_ret_28,
                                                                );
                                                                mfs_log(
                                                                    MFSLOG_SYSLOG,
                                                                    MFSLOG_ERR,
                                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    1455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    _mfs_assert_ret_28,
                                                                    _mfs_errorstring_58,
                                                                );
                                                                fprintf(
                                                                    stderr,
                                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    1455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    _mfs_assert_ret_28,
                                                                    _mfs_errorstring_58,
                                                                );
                                                            } else {
                                                                let mut _mfs_errorstring_err_28: *const ::core::ffi::c_char = strerr(
                                                                    *__errno_location(),
                                                                );
                                                                let mut _mfs_errorstring_ret_28: *const ::core::ffi::c_char = strerr(
                                                                    _mfs_assert_ret_28,
                                                                );
                                                                mfs_log(
                                                                    MFSLOG_SYSLOG,
                                                                    MFSLOG_ERR,
                                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    1455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    _mfs_assert_ret_28,
                                                                    _mfs_errorstring_ret_28,
                                                                    *__errno_location(),
                                                                    _mfs_errorstring_err_28,
                                                                );
                                                                fprintf(
                                                                    stderr,
                                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    1455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    _mfs_assert_ret_28,
                                                                    _mfs_errorstring_ret_28,
                                                                    *__errno_location(),
                                                                    _mfs_errorstring_err_28,
                                                                );
                                                            }
                                                            abort();
                                                        }
                                                        status = EIO;
                                                        break;
                                                    } else {
                                                        if rcb == cb {
                                                            if sending_mode as ::core::ffi::c_int
                                                                == 2 as ::core::ffi::c_int
                                                            {
                                                                mfs_log(
                                                                    MFSLOG_SYSLOG,
                                                                    MFSLOG_WARNING,
                                                                    b"writeworker: got status OK before all data have been sent\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                );
                                                                let mut _mfs_assert_ret_29: ::core::ffi::c_int = pthread_mutex_unlock(
                                                                    &raw mut (*ind).lock,
                                                                );
                                                                if _mfs_assert_ret_29
                                                                    != 0 as ::core::ffi::c_int
                                                                {
                                                                    if _mfs_assert_ret_29 < 0 as ::core::ffi::c_int
                                                                        && *__errno_location() != 0 as ::core::ffi::c_int
                                                                    {
                                                                        let mut _mfs_errorstring_59: *const ::core::ffi::c_char = strerr(
                                                                            *__errno_location(),
                                                                        );
                                                                        mfs_log(
                                                                            MFSLOG_SYSLOG,
                                                                            MFSLOG_ERR,
                                                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            1463 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            _mfs_assert_ret_29,
                                                                            *__errno_location(),
                                                                            _mfs_errorstring_59,
                                                                        );
                                                                        fprintf(
                                                                            stderr,
                                                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            1463 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            _mfs_assert_ret_29,
                                                                            *__errno_location(),
                                                                            _mfs_errorstring_59,
                                                                        );
                                                                    } else if _mfs_assert_ret_29 > 0 as ::core::ffi::c_int
                                                                        && *__errno_location() == 0 as ::core::ffi::c_int
                                                                    {
                                                                        let mut _mfs_errorstring_60: *const ::core::ffi::c_char = strerr(
                                                                            _mfs_assert_ret_29,
                                                                        );
                                                                        mfs_log(
                                                                            MFSLOG_SYSLOG,
                                                                            MFSLOG_ERR,
                                                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            1463 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            _mfs_assert_ret_29,
                                                                            _mfs_errorstring_60,
                                                                        );
                                                                        fprintf(
                                                                            stderr,
                                                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            1463 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            _mfs_assert_ret_29,
                                                                            _mfs_errorstring_60,
                                                                        );
                                                                    } else {
                                                                        let mut _mfs_errorstring_err_29: *const ::core::ffi::c_char = strerr(
                                                                            *__errno_location(),
                                                                        );
                                                                        let mut _mfs_errorstring_ret_29: *const ::core::ffi::c_char = strerr(
                                                                            _mfs_assert_ret_29,
                                                                        );
                                                                        mfs_log(
                                                                            MFSLOG_SYSLOG,
                                                                            MFSLOG_ERR,
                                                                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            1463 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            _mfs_assert_ret_29,
                                                                            _mfs_errorstring_ret_29,
                                                                            *__errno_location(),
                                                                            _mfs_errorstring_err_29,
                                                                        );
                                                                        fprintf(
                                                                            stderr,
                                                                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            1463 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                                as *const ::core::ffi::c_char,
                                                                            _mfs_assert_ret_29,
                                                                            _mfs_errorstring_ret_29,
                                                                            *__errno_location(),
                                                                            _mfs_errorstring_err_29,
                                                                        );
                                                                    }
                                                                    abort();
                                                                }
                                                                status = EIO;
                                                                break;
                                                            } else {
                                                                cb = ::core::ptr::null_mut::<cblock>(
                                                                );
                                                            }
                                                        }
                                                        if !(*rcb).prev.is_null() {
                                                            (*(*rcb).prev).next = (*rcb).next;
                                                        } else {
                                                            (*chd).datachainhead =
                                                                (*rcb).next as *mut cblock;
                                                        }
                                                        if !(*rcb).next.is_null() {
                                                            (*(*rcb).next).prev = (*rcb).prev;
                                                        } else {
                                                            (*chd).datachaintail =
                                                                (*rcb).prev as *mut cblock;
                                                        }
                                                        maxwroffset = ((chindx as uint64_t)
                                                            << MFSCHUNKBITS)
                                                            .wrapping_add(
                                                                (((*rcb).pos as uint32_t)
                                                                    << MFSBLOCKBITS)
                                                                    as uint64_t,
                                                            )
                                                            .wrapping_add((*rcb).to as uint64_t);
                                                        if maxwroffset > mfleng {
                                                            mfleng = maxwroffset;
                                                        }
                                                        write_cb_release(ind, rcb);
                                                        let mut _mfs_assert_ret_30: ::core::ffi::c_int = pthread_mutex_unlock(
                                                            &raw mut (*ind).lock,
                                                        );
                                                        if _mfs_assert_ret_30
                                                            != 0 as ::core::ffi::c_int
                                                        {
                                                            if _mfs_assert_ret_30
                                                                < 0 as ::core::ffi::c_int
                                                                && *__errno_location()
                                                                    != 0 as ::core::ffi::c_int
                                                            {
                                                                let mut _mfs_errorstring_61: *const ::core::ffi::c_char = strerr(
                                                                    *__errno_location(),
                                                                );
                                                                mfs_log(
                                                                    MFSLOG_SYSLOG,
                                                                    MFSLOG_ERR,
                                                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    1485 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    _mfs_assert_ret_30,
                                                                    *__errno_location(),
                                                                    _mfs_errorstring_61,
                                                                );
                                                                fprintf(
                                                                    stderr,
                                                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    1485 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    _mfs_assert_ret_30,
                                                                    *__errno_location(),
                                                                    _mfs_errorstring_61,
                                                                );
                                                            } else if _mfs_assert_ret_30
                                                                > 0 as ::core::ffi::c_int
                                                                && *__errno_location()
                                                                    == 0 as ::core::ffi::c_int
                                                            {
                                                                let mut _mfs_errorstring_62: *const ::core::ffi::c_char = strerr(
                                                                    _mfs_assert_ret_30,
                                                                );
                                                                mfs_log(
                                                                    MFSLOG_SYSLOG,
                                                                    MFSLOG_ERR,
                                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    1485 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    _mfs_assert_ret_30,
                                                                    _mfs_errorstring_62,
                                                                );
                                                                fprintf(
                                                                    stderr,
                                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    1485 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    _mfs_assert_ret_30,
                                                                    _mfs_errorstring_62,
                                                                );
                                                            } else {
                                                                let mut _mfs_errorstring_err_30: *const ::core::ffi::c_char = strerr(
                                                                    *__errno_location(),
                                                                );
                                                                let mut _mfs_errorstring_ret_30: *const ::core::ffi::c_char = strerr(
                                                                    _mfs_assert_ret_30,
                                                                );
                                                                mfs_log(
                                                                    MFSLOG_SYSLOG,
                                                                    MFSLOG_ERR,
                                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    1485 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    _mfs_assert_ret_30,
                                                                    _mfs_errorstring_ret_30,
                                                                    *__errno_location(),
                                                                    _mfs_errorstring_err_30,
                                                                );
                                                                fprintf(
                                                                    stderr,
                                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    1485 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                                                        as *const ::core::ffi::c_char,
                                                                    _mfs_assert_ret_30,
                                                                    _mfs_errorstring_ret_30,
                                                                    *__errno_location(),
                                                                    _mfs_errorstring_err_30,
                                                                );
                                                            }
                                                            abort();
                                                        }
                                                    }
                                                }
                                                waitforstatus = waitforstatus.wrapping_sub(1);
                                                rcvd = 0 as uint32_t;
                                            }
                                        }
                                    }
                                }
                            }
                            if waitforstatus as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                && chainminver
                                    >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                        + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                        + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                            32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                        } else {
                                            32 as ::core::ffi::c_int
                                        })) as uint32_t
                            {
                                wptr = &raw mut sendbuff as *mut uint8_t;
                                put32bit(&raw mut wptr, CLTOCS_WRITE_FINISH as uint32_t);
                                put32bit(&raw mut wptr, 12 as uint32_t);
                                put64bit(&raw mut wptr, chunkid);
                                put32bit(&raw mut wptr, version);
                                if write(
                                    fd,
                                    &raw mut sendbuff as *mut uint8_t as *const ::core::ffi::c_void,
                                    20 as size_t,
                                ) == 20 as ssize_t
                                {
                                    conncache_insert(ip, port, fd);
                                } else {
                                    tcpclose(fd);
                                }
                            } else {
                                tcpclose(fd);
                            }
                            if status != 0 as ::core::ffi::c_int
                                || wrstatus as ::core::ffi::c_int != MFS_STATUS_OK
                            {
                                if wrstatus as ::core::ffi::c_int != MFS_STATUS_OK {
                                    if wrstatus as ::core::ffi::c_int == MFS_ERROR_NOSPACE {
                                        status = ENOSPC;
                                    } else {
                                        status = EIO;
                                    }
                                }
                            } else if nextwriteid.wrapping_sub(1 as uint32_t)
                                == waitforstatus as uint32_t
                            {
                                if (*chd).trycnt as uint32_t >= minlogretry {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"nothing has been written (unfinished writes: %hhu; try counter: %u)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        waitforstatus as ::core::ffi::c_int,
                                        (*chd).trycnt as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int,
                                    );
                                }
                                status = EIO;
                            }
                            let mut _mfs_assert_ret_31: ::core::ffi::c_int =
                                pthread_mutex_lock(&raw mut (*ind).lock);
                            if _mfs_assert_ret_31 != 0 as ::core::ffi::c_int {
                                if _mfs_assert_ret_31 < 0 as ::core::ffi::c_int
                                    && *__errno_location() != 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_63: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1537 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_31,
                                        *__errno_location(),
                                        _mfs_errorstring_63,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1537 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_31,
                                        *__errno_location(),
                                        _mfs_errorstring_63,
                                    );
                                } else if _mfs_assert_ret_31 > 0 as ::core::ffi::c_int
                                    && *__errno_location() == 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_64: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_31);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1537 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_31,
                                        _mfs_errorstring_64,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1537 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_31,
                                        _mfs_errorstring_64,
                                    );
                                } else {
                                    let mut _mfs_errorstring_err_31: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    let mut _mfs_errorstring_ret_31: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_31);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1537 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_31,
                                        _mfs_errorstring_ret_31,
                                        *__errno_location(),
                                        _mfs_errorstring_err_31,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1537 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_31,
                                        _mfs_errorstring_ret_31,
                                        *__errno_location(),
                                        _mfs_errorstring_err_31,
                                    );
                                }
                                abort();
                            }
                            unbreakable = (*chd).unbreakable;
                            if optimeout > 0.0f64 && monotonic_seconds() - opbegin > optimeout {
                                unbreakable = 0 as uint8_t;
                            } else if status != 0 as ::core::ffi::c_int {
                                if wrstatus as ::core::ffi::c_int != MFS_ERROR_NOTDONE {
                                    (*chd).trycnt = (*chd).trycnt.wrapping_add(1);
                                }
                                if (*chd).trycnt as uint32_t >= maxretries {
                                    unbreakable = 0 as uint8_t;
                                }
                            } else {
                                unbreakable = 0 as uint8_t;
                            }
                            (*chd).continueop = unbreakable;
                            let mut _mfs_assert_ret_32: ::core::ffi::c_int =
                                pthread_mutex_unlock(&raw mut (*ind).lock);
                            if _mfs_assert_ret_32 != 0 as ::core::ffi::c_int {
                                if _mfs_assert_ret_32 < 0 as ::core::ffi::c_int
                                    && *__errno_location() != 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_65: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1553 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_32,
                                        *__errno_location(),
                                        _mfs_errorstring_65,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1553 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_32,
                                        *__errno_location(),
                                        _mfs_errorstring_65,
                                    );
                                } else if _mfs_assert_ret_32 > 0 as ::core::ffi::c_int
                                    && *__errno_location() == 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_66: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_32);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1553 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_32,
                                        _mfs_errorstring_66,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1553 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_32,
                                        _mfs_errorstring_66,
                                    );
                                } else {
                                    let mut _mfs_errorstring_err_32: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    let mut _mfs_errorstring_ret_32: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_32);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1553 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_32,
                                        _mfs_errorstring_ret_32,
                                        *__errno_location(),
                                        _mfs_errorstring_err_32,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1553 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_32,
                                        _mfs_errorstring_ret_32,
                                        *__errno_location(),
                                        _mfs_errorstring_err_32,
                                    );
                                }
                                abort();
                            }
                            if unbreakable as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                westatus = fs_writeend(
                                    chunkid,
                                    inode,
                                    chindx,
                                    mfleng,
                                    0 as uint8_t,
                                    min_from,
                                    max_to.wrapping_sub(min_from),
                                );
                            } else {
                                westatus = MFS_STATUS_OK as uint8_t;
                            }
                            if optimeout > 0.0f64 && monotonic_seconds() - opbegin > optimeout {
                                write_job_end(chd, EIO, 0 as uint32_t);
                            } else if westatus as ::core::ffi::c_int == MFS_ERROR_ENOENT {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"file: %u, index: %u - fs_writeend returned status: %s\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    inode,
                                    chindx,
                                    mfsstrerr(westatus),
                                );
                                write_job_end(chd, EBADF, 0 as uint32_t);
                            } else if westatus as ::core::ffi::c_int == MFS_ERROR_QUOTA {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"file: %u, index: %u - fs_writeend returned status: %s\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    inode,
                                    chindx,
                                    mfsstrerr(westatus),
                                );
                                write_job_end(chd, EDQUOT, 0 as uint32_t);
                            } else if westatus as ::core::ffi::c_int == MFS_ERROR_IO {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"file: %u, index: %u - fs_writeend returned status: %s\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    inode,
                                    chindx,
                                    mfsstrerr(westatus),
                                );
                                write_job_end(chd, EIO, 0 as uint32_t);
                            } else if westatus as ::core::ffi::c_int != MFS_STATUS_OK {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"file: %u, index: %u - fs_writeend returned status: %s\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    inode,
                                    chindx,
                                    mfsstrerr(westatus),
                                );
                                write_job_end(chd, ENXIO, 0 as uint32_t);
                            } else if status != 0 as ::core::ffi::c_int {
                                if (*chd).trycnt as uint32_t >= maxretries {
                                    write_job_end(chd, status, 0 as uint32_t);
                                } else if wrstatus as ::core::ffi::c_int == MFS_ERROR_NOTDONE {
                                    write_job_end(chd, 0 as ::core::ffi::c_int, 300000 as uint32_t);
                                } else {
                                    write_job_end(
                                        chd,
                                        0 as ::core::ffi::c_int,
                                        (1000 as ::core::ffi::c_int
                                            + (if ((*chd).trycnt as ::core::ffi::c_int)
                                                < 30 as ::core::ffi::c_int
                                            {
                                                ((*chd).trycnt as ::core::ffi::c_int
                                                    - 1 as ::core::ffi::c_int)
                                                    * 300000 as ::core::ffi::c_int
                                            } else {
                                                10000000 as ::core::ffi::c_int
                                            })) as uint32_t,
                                    );
                                }
                            } else {
                                if valid_offsets != 0 {
                                    read_inode_clear_cache(
                                        inode,
                                        min_offset,
                                        max_offset.wrapping_sub(min_offset),
                                    );
                                }
                                write_job_end(chd, 0 as ::core::ffi::c_int, 0 as uint32_t);
                            }
                            chunkrwlock_wunlock(inode, chindx);
                        }
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_data_init(
    mut cachesize: uint32_t,
    mut retries: uint32_t,
    mut timeout: uint32_t,
    mut logretry: uint32_t,
    mut erronlostchunk: uint8_t,
    mut erronnospace: uint8_t,
) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut mystacksize: size_t = 0;
        erroronlostchunk = erronlostchunk;
        erroronnospace = erronnospace;
        cacheblockcount = cachesize.wrapping_div(MFSBLOCKSIZE as uint32_t);
        maxretries = retries;
        if optimeout > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            optimeout = timeout as ::core::ffi::c_double;
        } else {
            optimeout = 0.0f64;
        }
        minlogretry = logretry;
        if cacheblockcount < 10 as uint32_t {
            cacheblockcount = 10 as uint32_t;
        }
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_init(
            &raw mut hashlock,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1632 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&hashlock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1632 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&hashlock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1632 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&hashlock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1632 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&hashlock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1632 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&hashlock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1632 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&hashlock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_init(
            &raw mut workerslock,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1633 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&workerslock,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1633 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&workerslock,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1633 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&workerslock,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1633 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&workerslock,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1633 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&workerslock,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1633 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&workerslock,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_cond_init(
            &raw mut worker_term_cond,
            ::core::ptr::null::<pthread_condattr_t>(),
        );
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1634 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&worker_term_cond,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1634 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&worker_term_cond,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1634 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&worker_term_cond,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1634 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&worker_term_cond,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1634 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&worker_term_cond,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1634 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&worker_term_cond,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        worker_term_waiting = 0 as uint32_t;
        let mut _mfs_assert_ret_2: ::core::ffi::c_int =
            pthread_mutex_init(&raw mut fcblock, ::core::ptr::null::<pthread_mutexattr_t>());
        if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1637 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&fcblock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1637 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&fcblock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
            } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_6: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1637 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&fcblock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1637 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&fcblock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
            } else {
                let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1637 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&fcblock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1637 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&fcblock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_3: ::core::ffi::c_int =
            pthread_cond_init(&raw mut fcbcond, ::core::ptr::null::<pthread_condattr_t>());
        if _mfs_assert_ret_3 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_3 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_7: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1638 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&fcbcond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1638 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&fcbcond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_7,
                );
            } else if _mfs_assert_ret_3 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_8: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_3);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1638 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&fcbcond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_8,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1638 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&fcbcond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_8,
                );
            } else {
                let mut _mfs_errorstring_err_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_3: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_3);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1638 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&fcbcond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1638 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&fcbcond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
            }
            abort();
        }
        fcbwaiting = 0 as uint16_t;
        cacheblocks =
            malloc(::core::mem::size_of::<cblock>().wrapping_mul(cacheblockcount as size_t))
                as *mut cblock;
        if cacheblocks.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                1641 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"cacheblocks\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                1641 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"cacheblocks\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if cacheblocks
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut cblock
        {
            let mut _mfs_errorstring_9: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                1641 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"cacheblocks\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_9,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                1641 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"cacheblocks\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_9,
            );
            abort();
        }
        i = 0 as uint32_t;
        while i < cacheblockcount.wrapping_sub(1 as uint32_t) {
            (*cacheblocks.offset(i as isize)).next =
                cacheblocks.offset(i.wrapping_add(1 as uint32_t) as isize) as *mut cblock_s;
            i = i.wrapping_add(1);
        }
        (*cacheblocks.offset(cacheblockcount.wrapping_sub(1 as uint32_t) as isize)).next =
            ::core::ptr::null_mut::<cblock_s>();
        freecblockshead = cacheblocks;
        freecacheblocks = cacheblockcount;
        idhash = malloc(::core::mem::size_of::<*mut inodedata>().wrapping_mul(IDHASHSIZE as size_t))
            as *mut *mut inodedata;
        if idhash.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                1650 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"idhash\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                1650 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"idhash\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if idhash
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut inodedata
        {
            let mut _mfs_errorstring_10: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                1650 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"idhash\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_10,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr() as *const ::core::ffi::c_char,
                1650 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"idhash\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_10,
            );
            abort();
        }
        i = 0 as uint32_t;
        while i < IDHASHSIZE as uint32_t {
            *idhash.offset(i as isize) = ::core::ptr::null_mut::<inodedata>();
            i = i.wrapping_add(1);
        }
        jqueue = queue_new(0 as uint32_t);
        let mut _mfs_assert_ret_4: ::core::ffi::c_int = pthread_attr_init(&raw mut worker_thattr);
        if _mfs_assert_ret_4 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_4 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_11: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1658 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&worker_thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_11,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1658 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&worker_thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_11,
                );
            } else if _mfs_assert_ret_4 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_12: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_4);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1658 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&worker_thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_12,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1658 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&worker_thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_12,
                );
            } else {
                let mut _mfs_errorstring_err_4: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_4: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_4);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1658 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&worker_thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1658 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&worker_thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
            }
            abort();
        }
        mystacksize = __sysconf(__SC_THREAD_STACK_MIN_VALUE) as size_t;
        if mystacksize < 0x20000 as ::core::ffi::c_int as size_t {
            mystacksize = 0x20000 as ::core::ffi::c_int as size_t;
        }
        let mut _mfs_assert_ret_5: ::core::ffi::c_int =
            pthread_attr_setstacksize(&raw mut worker_thattr, mystacksize);
        if _mfs_assert_ret_5 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_5 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_13: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1667 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&worker_thattr,mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_13,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1667 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&worker_thattr,mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_13,
                );
            } else if _mfs_assert_ret_5 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_14: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_5);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1667 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&worker_thattr,mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_14,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1667 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&worker_thattr,mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_14,
                );
            } else {
                let mut _mfs_errorstring_err_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_5: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_5);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1667 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&worker_thattr,mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_err_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1667 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&worker_thattr,mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_err_5,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_6: ::core::ffi::c_int = pthread_mutex_lock(&raw mut workerslock);
        if _mfs_assert_ret_6 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_6 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_15: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1678 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    *__errno_location(),
                    _mfs_errorstring_15,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1678 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    *__errno_location(),
                    _mfs_errorstring_15,
                );
            } else if _mfs_assert_ret_6 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_16: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_6);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1678 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    _mfs_errorstring_16,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1678 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    _mfs_errorstring_16,
                );
            } else {
                let mut _mfs_errorstring_err_6: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_6: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_6);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1678 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    _mfs_errorstring_ret_6,
                    *__errno_location(),
                    _mfs_errorstring_err_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1678 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    _mfs_errorstring_ret_6,
                    *__errno_location(),
                    _mfs_errorstring_err_6,
                );
            }
            abort();
        }
        workers_avail = 0 as uint32_t;
        workers_total = 0 as uint32_t;
        write_data_spawn_worker();
        let mut _mfs_assert_ret_7: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut workerslock);
        if _mfs_assert_ret_7 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_7 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_17: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1682 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_7,
                    *__errno_location(),
                    _mfs_errorstring_17,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1682 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_7,
                    *__errno_location(),
                    _mfs_errorstring_17,
                );
            } else if _mfs_assert_ret_7 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_18: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_7);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1682 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_7,
                    _mfs_errorstring_18,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1682 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_7,
                    _mfs_errorstring_18,
                );
            } else {
                let mut _mfs_errorstring_err_7: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_7: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_7);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1682 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_7,
                    _mfs_errorstring_ret_7,
                    *__errno_location(),
                    _mfs_errorstring_err_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1682 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_7,
                    _mfs_errorstring_ret_7,
                    *__errno_location(),
                    _mfs_errorstring_err_7,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_data_term() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        let mut indn: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        let mut chd: *mut chunkdata = ::core::ptr::null_mut::<chunkdata>();
        let mut chdn: *mut chunkdata = ::core::ptr::null_mut::<chunkdata>();
        queue_close(jqueue);
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut workerslock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1695 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1695 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1695 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1695 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1695 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1695 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        while workers_total > 0 as uint32_t {
            worker_term_waiting = worker_term_waiting.wrapping_add(1);
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_cond_wait(&raw mut worker_term_cond, &raw mut workerslock);
            if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1698 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&worker_term_cond,&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1698 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&worker_term_cond,&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1698 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&worker_term_cond,&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1698 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&worker_term_cond,&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                } else {
                    let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1698 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&worker_term_cond,&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1698 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&worker_term_cond,&workerslock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut workerslock);
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1700 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1700 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1700 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1700 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1700 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1700 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        queue_delete(jqueue);
        let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_lock(&raw mut hashlock);
        if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1704 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1704 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
            } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_6: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1704 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1704 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
            } else {
                let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1704 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1704 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
        i = 0 as uint32_t;
        while i < IDHASHSIZE as uint32_t {
            ind = *idhash.offset(i as isize);
            while !ind.is_null() {
                indn = (*ind).next as *mut inodedata;
                let mut _mfs_assert_ret_3: ::core::ffi::c_int =
                    pthread_mutex_lock(&raw mut (*ind).lock);
                if _mfs_assert_ret_3 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_3 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_7: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1708 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_7,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1708 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_7,
                        );
                    } else if _mfs_assert_ret_3 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_8: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_3);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1708 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_8,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1708 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_8,
                        );
                    } else {
                        let mut _mfs_errorstring_err_3: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_3: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_3);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1708 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_err_3,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1708 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_err_3,
                        );
                    }
                    abort();
                }
                chd = (*ind).chunks;
                while !chd.is_null() {
                    chdn = (*chd).next as *mut chunkdata;
                    write_free_chunkdata(chd);
                    chd = chdn;
                }
                let mut _mfs_assert_ret_4: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut (*ind).lock);
                if _mfs_assert_ret_4 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_4 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_9: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1715 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_4,
                            *__errno_location(),
                            _mfs_errorstring_9,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1715 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_4,
                            *__errno_location(),
                            _mfs_errorstring_9,
                        );
                    } else if _mfs_assert_ret_4 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_10: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_4);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1715 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_4,
                            _mfs_errorstring_10,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1715 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_4,
                            _mfs_errorstring_10,
                        );
                    } else {
                        let mut _mfs_errorstring_err_4: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_4: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_4);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1715 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_4,
                            _mfs_errorstring_ret_4,
                            *__errno_location(),
                            _mfs_errorstring_err_4,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1715 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_4,
                            _mfs_errorstring_ret_4,
                            *__errno_location(),
                            _mfs_errorstring_err_4,
                        );
                    }
                    abort();
                }
                let mut _mfs_assert_ret_5: ::core::ffi::c_int =
                    pthread_cond_destroy(&raw mut (*ind).flushcond);
                if _mfs_assert_ret_5 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_5 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_11: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1716 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(ind->flushcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_5,
                            *__errno_location(),
                            _mfs_errorstring_11,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1716 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(ind->flushcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_5,
                            *__errno_location(),
                            _mfs_errorstring_11,
                        );
                    } else if _mfs_assert_ret_5 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_12: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_5);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1716 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(ind->flushcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_5,
                            _mfs_errorstring_12,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1716 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(ind->flushcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_5,
                            _mfs_errorstring_12,
                        );
                    } else {
                        let mut _mfs_errorstring_err_5: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_5: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_5);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1716 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(ind->flushcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_5,
                            _mfs_errorstring_ret_5,
                            *__errno_location(),
                            _mfs_errorstring_err_5,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1716 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(ind->flushcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_5,
                            _mfs_errorstring_ret_5,
                            *__errno_location(),
                            _mfs_errorstring_err_5,
                        );
                    }
                    abort();
                }
                let mut _mfs_assert_ret_6: ::core::ffi::c_int =
                    pthread_cond_destroy(&raw mut (*ind).writecond);
                if _mfs_assert_ret_6 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_6 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_13: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1717 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(ind->writecond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_6,
                            *__errno_location(),
                            _mfs_errorstring_13,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1717 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(ind->writecond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_6,
                            *__errno_location(),
                            _mfs_errorstring_13,
                        );
                    } else if _mfs_assert_ret_6 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_14: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_6);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1717 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(ind->writecond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_6,
                            _mfs_errorstring_14,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1717 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(ind->writecond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_6,
                            _mfs_errorstring_14,
                        );
                    } else {
                        let mut _mfs_errorstring_err_6: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_6: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_6);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1717 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(ind->writecond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_6,
                            _mfs_errorstring_ret_6,
                            *__errno_location(),
                            _mfs_errorstring_err_6,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1717 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(ind->writecond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_6,
                            _mfs_errorstring_ret_6,
                            *__errno_location(),
                            _mfs_errorstring_err_6,
                        );
                    }
                    abort();
                }
                let mut _mfs_assert_ret_7: ::core::ffi::c_int =
                    pthread_mutex_destroy(&raw mut (*ind).lock);
                if _mfs_assert_ret_7 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_7 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_15: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1718 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_destroy(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_7,
                            *__errno_location(),
                            _mfs_errorstring_15,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1718 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_destroy(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_7,
                            *__errno_location(),
                            _mfs_errorstring_15,
                        );
                    } else if _mfs_assert_ret_7 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_16: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_7);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1718 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_destroy(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_7,
                            _mfs_errorstring_16,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1718 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_destroy(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_7,
                            _mfs_errorstring_16,
                        );
                    } else {
                        let mut _mfs_errorstring_err_7: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_7: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_7);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1718 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_destroy(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_7,
                            _mfs_errorstring_ret_7,
                            *__errno_location(),
                            _mfs_errorstring_err_7,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1718 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_destroy(&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_7,
                            _mfs_errorstring_ret_7,
                            *__errno_location(),
                            _mfs_errorstring_err_7,
                        );
                    }
                    abort();
                }
                free(ind as *mut ::core::ffi::c_void);
                ind = indn;
            }
            i = i.wrapping_add(1);
        }
        free(idhash as *mut ::core::ffi::c_void);
        let mut _mfs_assert_ret_8: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut hashlock);
        if _mfs_assert_ret_8 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_8 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_17: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1723 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_8,
                    *__errno_location(),
                    _mfs_errorstring_17,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1723 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_8,
                    *__errno_location(),
                    _mfs_errorstring_17,
                );
            } else if _mfs_assert_ret_8 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_18: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_8);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1723 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_8,
                    _mfs_errorstring_18,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1723 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_8,
                    _mfs_errorstring_18,
                );
            } else {
                let mut _mfs_errorstring_err_8: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_8: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_8);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1723 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_8,
                    _mfs_errorstring_ret_8,
                    *__errno_location(),
                    _mfs_errorstring_err_8,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1723 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_8,
                    _mfs_errorstring_ret_8,
                    *__errno_location(),
                    _mfs_errorstring_err_8,
                );
            }
            abort();
        }
        free(cacheblocks as *mut ::core::ffi::c_void);
        let mut _mfs_assert_ret_9: ::core::ffi::c_int =
            pthread_attr_destroy(&raw mut worker_thattr);
        if _mfs_assert_ret_9 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_9 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_19: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1725 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&worker_thattr)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_9,
                    *__errno_location(),
                    _mfs_errorstring_19,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1725 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&worker_thattr)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_9,
                    *__errno_location(),
                    _mfs_errorstring_19,
                );
            } else if _mfs_assert_ret_9 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_20: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_9);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1725 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&worker_thattr)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_9,
                    _mfs_errorstring_20,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1725 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&worker_thattr)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_9,
                    _mfs_errorstring_20,
                );
            } else {
                let mut _mfs_errorstring_err_9: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_9: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_9);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1725 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&worker_thattr)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_9,
                    _mfs_errorstring_ret_9,
                    *__errno_location(),
                    _mfs_errorstring_err_9,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1725 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&worker_thattr)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_9,
                    _mfs_errorstring_ret_9,
                    *__errno_location(),
                    _mfs_errorstring_err_9,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_10: ::core::ffi::c_int =
            pthread_cond_destroy(&raw mut worker_term_cond);
        if _mfs_assert_ret_10 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_10 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_21: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1726 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&worker_term_cond)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_10,
                    *__errno_location(),
                    _mfs_errorstring_21,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1726 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&worker_term_cond)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_10,
                    *__errno_location(),
                    _mfs_errorstring_21,
                );
            } else if _mfs_assert_ret_10 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_22: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_10);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1726 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&worker_term_cond)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_10,
                    _mfs_errorstring_22,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1726 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&worker_term_cond)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_10,
                    _mfs_errorstring_22,
                );
            } else {
                let mut _mfs_errorstring_err_10: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_10: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_10);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1726 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&worker_term_cond)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_10,
                    _mfs_errorstring_ret_10,
                    *__errno_location(),
                    _mfs_errorstring_err_10,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1726 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&worker_term_cond)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_10,
                    _mfs_errorstring_ret_10,
                    *__errno_location(),
                    _mfs_errorstring_err_10,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_11: ::core::ffi::c_int = pthread_cond_destroy(&raw mut fcbcond);
        if _mfs_assert_ret_11 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_11 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_23: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1727 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&fcbcond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_11,
                    *__errno_location(),
                    _mfs_errorstring_23,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1727 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&fcbcond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_11,
                    *__errno_location(),
                    _mfs_errorstring_23,
                );
            } else if _mfs_assert_ret_11 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_24: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_11);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1727 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&fcbcond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_11,
                    _mfs_errorstring_24,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1727 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&fcbcond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_11,
                    _mfs_errorstring_24,
                );
            } else {
                let mut _mfs_errorstring_err_11: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_11: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_11);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1727 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&fcbcond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_11,
                    _mfs_errorstring_ret_11,
                    *__errno_location(),
                    _mfs_errorstring_err_11,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1727 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&fcbcond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_11,
                    _mfs_errorstring_ret_11,
                    *__errno_location(),
                    _mfs_errorstring_err_11,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_12: ::core::ffi::c_int = pthread_mutex_destroy(&raw mut fcblock);
        if _mfs_assert_ret_12 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_12 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_25: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1728 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_12,
                    *__errno_location(),
                    _mfs_errorstring_25,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1728 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_12,
                    *__errno_location(),
                    _mfs_errorstring_25,
                );
            } else if _mfs_assert_ret_12 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_26: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_12);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1728 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_12,
                    _mfs_errorstring_26,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1728 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_12,
                    _mfs_errorstring_26,
                );
            } else {
                let mut _mfs_errorstring_err_12: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_12: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_12);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1728 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_12,
                    _mfs_errorstring_ret_12,
                    *__errno_location(),
                    _mfs_errorstring_err_12,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1728 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&fcblock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_12,
                    _mfs_errorstring_ret_12,
                    *__errno_location(),
                    _mfs_errorstring_err_12,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_13: ::core::ffi::c_int =
            pthread_mutex_destroy(&raw mut workerslock);
        if _mfs_assert_ret_13 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_13 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_27: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_13,
                    *__errno_location(),
                    _mfs_errorstring_27,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_13,
                    *__errno_location(),
                    _mfs_errorstring_27,
                );
            } else if _mfs_assert_ret_13 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_28: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_13);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_13,
                    _mfs_errorstring_28,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_13,
                    _mfs_errorstring_28,
                );
            } else {
                let mut _mfs_errorstring_err_13: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_13: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_13);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_13,
                    _mfs_errorstring_ret_13,
                    *__errno_location(),
                    _mfs_errorstring_err_13,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&workerslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_13,
                    _mfs_errorstring_ret_13,
                    *__errno_location(),
                    _mfs_errorstring_err_13,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_14: ::core::ffi::c_int = pthread_mutex_destroy(&raw mut hashlock);
        if _mfs_assert_ret_14 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_14 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_29: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1730 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_14,
                    *__errno_location(),
                    _mfs_errorstring_29,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1730 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_14,
                    *__errno_location(),
                    _mfs_errorstring_29,
                );
            } else if _mfs_assert_ret_14 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_30: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_14);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1730 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_14,
                    _mfs_errorstring_30,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1730 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_14,
                    _mfs_errorstring_30,
                );
            } else {
                let mut _mfs_errorstring_err_14: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_14: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_14);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1730 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_14,
                    _mfs_errorstring_ret_14,
                    *__errno_location(),
                    _mfs_errorstring_err_14,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1730 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&hashlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_14,
                    _mfs_errorstring_ret_14,
                    *__errno_location(),
                    _mfs_errorstring_err_14,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_cb_expand(
    mut chd: *mut chunkdata,
    mut cb: *mut cblock,
    mut from: uint32_t,
    mut to: uint32_t,
    mut data: *const uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        if (*cb).writeid > 0 as uint32_t || from > (*cb).to || to < (*cb).from {
            return -1 as ::core::ffi::c_int;
        }
        memcpy(
            (&raw mut (*cb).data as *mut uint8_t).offset(from as isize) as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            to.wrapping_sub(from) as size_t,
        );
        if from < (*cb).from {
            (*cb).from = from;
        }
        if to > (*cb).to {
            (*cb).to = to;
        }
        if (*cb).to.wrapping_sub((*cb).from) == MFSBLOCKSIZE as uint32_t
            && (*cb).next.is_null()
            && (*chd).waitingworker as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        {
            if write(
                (*chd).wakeup_fd,
                b" \0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                1 as size_t,
            ) != 1 as ssize_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"can't write to pipe !!!\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            (*chd).waitingworker = 0 as uint8_t;
            (*chd).wakeup_fd = -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_block(
    mut ind: *mut inodedata,
    mut chindx: uint32_t,
    mut pos: uint16_t,
    mut from: uint32_t,
    mut to: uint32_t,
    mut data: *const uint8_t,
    mut superuser: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut cb: *mut cblock = ::core::ptr::null_mut::<cblock>();
        let mut ncb: *mut cblock = ::core::ptr::null_mut::<cblock>();
        let mut chd: *mut chunkdata = ::core::ptr::null_mut::<chunkdata>();
        let mut newchunk: uint8_t = 0;
        ncb = write_cb_acquire(ind);
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*ind).lock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1760 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1760 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1760 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1760 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1760 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1760 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        chd = (*ind).chunks;
        while !chd.is_null() {
            if (*chd).chindx == chindx {
                if superuser != 0 {
                    (*chd).superuser = 1 as uint8_t;
                }
                cb = (*chd).datachaintail;
                while !cb.is_null() {
                    if (*cb).pos as ::core::ffi::c_int == pos as ::core::ffi::c_int {
                        if write_cb_expand(chd, cb, from, to, data) == 0 as ::core::ffi::c_int {
                            write_cb_release(ind, ncb);
                            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                                pthread_mutex_unlock(&raw mut (*ind).lock);
                            if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                                if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                                    && *__errno_location() != 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1770 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_0,
                                        *__errno_location(),
                                        _mfs_errorstring_1,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1770 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_0,
                                        *__errno_location(),
                                        _mfs_errorstring_1,
                                    );
                                } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                                    && *__errno_location() == 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_0);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1770 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_0,
                                        _mfs_errorstring_2,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1770 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_0,
                                        _mfs_errorstring_2,
                                    );
                                } else {
                                    let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_0);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1770 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_0,
                                        _mfs_errorstring_ret_0,
                                        *__errno_location(),
                                        _mfs_errorstring_err_0,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1770 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_0,
                                        _mfs_errorstring_ret_0,
                                        *__errno_location(),
                                        _mfs_errorstring_err_0,
                                    );
                                }
                                abort();
                            }
                            return 0 as ::core::ffi::c_int;
                        }
                        break;
                    } else {
                        cb = (*cb).prev as *mut cblock;
                    }
                }
                break;
            } else {
                chd = (*chd).next as *mut chunkdata;
            }
        }
        (*ncb).pos = pos;
        (*ncb).from = from;
        (*ncb).to = to;
        memcpy(
            (&raw mut (*ncb).data as *mut uint8_t).offset(from as isize)
                as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            to.wrapping_sub(from) as size_t,
        );
        if chd.is_null() {
            chd = write_new_chunkdata(ind, chindx);
            if superuser != 0 {
                (*chd).superuser = 1 as uint8_t;
            }
            newchunk = 1 as uint8_t;
        } else {
            newchunk = 0 as uint8_t;
        }
        (*ncb).prev = (*chd).datachaintail as *mut cblock_s;
        (*ncb).next = ::core::ptr::null_mut::<cblock_s>();
        if !(*chd).datachaintail.is_null() {
            (*(*chd).datachaintail).next = ncb as *mut cblock_s;
        } else {
            (*chd).datachainhead = ncb;
        }
        (*chd).datachaintail = ncb;
        if newchunk != 0 {
            write_test_chunkdata(ind);
        } else if (*chd).waitingworker != 0 {
            if write(
                (*chd).wakeup_fd,
                b" \0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                1 as size_t,
            ) != 1 as ssize_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"can't write to pipe !!!\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            (*chd).waitingworker = 0 as uint8_t;
            (*chd).wakeup_fd = -1 as ::core::ffi::c_int;
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*ind).lock);
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1811 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1811 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1811 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1811 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1811 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1811 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_data(
    mut vid: *mut ::core::ffi::c_void,
    mut offset: uint64_t,
    mut size: uint32_t,
    mut data: *const uint8_t,
    mut superuser: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut chindx: uint32_t = 0;
        let mut pos: uint16_t = 0;
        let mut from: uint32_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut ind: *mut inodedata = vid as *mut inodedata;
        if ind.is_null() {
            return EIO;
        }
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*ind).lock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        status = (*ind).status;
        if status == 0 as ::core::ffi::c_int {
            if offset.wrapping_add(size as uint64_t) > (*ind).maxfleng {
                (*ind).maxfleng = offset.wrapping_add(size as uint64_t);
            }
            (*ind).writewaiting = (*ind).writewaiting.wrapping_add(1);
            while (*ind).flushwaiting as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                    pthread_cond_wait(&raw mut (*ind).writecond, &raw mut (*ind).lock);
                if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1837 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&(ind->writecond),&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1837 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&(ind->writecond),&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                    } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_0);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1837 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&(ind->writecond),&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1837 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&(ind->writecond),&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                    } else {
                        let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_0);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1837 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&(ind->writecond),&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_err_0,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1837 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&(ind->writecond),&(ind->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_err_0,
                        );
                    }
                    abort();
                }
            }
            (*ind).writewaiting = (*ind).writewaiting.wrapping_sub(1);
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*ind).lock);
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1841 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1841 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1841 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1841 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1841 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1841 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        if status != 0 as ::core::ffi::c_int {
            return status;
        }
        chindx = (offset >> MFSCHUNKBITS) as uint32_t;
        pos = ((offset & MFSCHUNKMASK as uint64_t) >> MFSBLOCKBITS) as uint16_t;
        from = (offset & MFSBLOCKMASK as uint64_t) as uint32_t;
        while size > 0 as uint32_t {
            if size > (MFSBLOCKSIZE as uint32_t).wrapping_sub(from) {
                if write_block(
                    ind,
                    chindx,
                    pos,
                    from,
                    MFSBLOCKSIZE as uint32_t,
                    data,
                    superuser,
                ) < 0 as ::core::ffi::c_int
                {
                    return EIO;
                }
                size = size.wrapping_sub((MFSBLOCKSIZE as uint32_t).wrapping_sub(from));
                data = data.offset((MFSBLOCKSIZE as uint32_t).wrapping_sub(from) as isize);
                from = 0 as uint32_t;
                pos = pos.wrapping_add(1);
                if pos as ::core::ffi::c_int == 1024 as ::core::ffi::c_int {
                    pos = 0 as uint16_t;
                    chindx = chindx.wrapping_add(1);
                }
            } else {
                if write_block(
                    ind,
                    chindx,
                    pos,
                    from,
                    from.wrapping_add(size),
                    data,
                    superuser,
                ) < 0 as ::core::ffi::c_int
                {
                    return EIO;
                }
                size = 0 as uint32_t;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_data_new(
    mut inode: uint32_t,
    mut fleng: uint64_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        ind = write_get_inodedata(inode, fleng);
        if ind.is_null() {
            return NULL;
        }
        return ind as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn write_data_do_chunk_wait(mut ind: *mut inodedata) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0;
        let mut chd: *mut chunkdata = ::core::ptr::null_mut::<chunkdata>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*ind).lock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1891 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1891 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1891 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1891 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1891 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1891 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        loop {
            chd = ::core::ptr::null_mut::<chunkdata>();
            if (*ind).status == 0 as ::core::ffi::c_int {
                chd = (*ind).chunks;
                while !chd.is_null() && (*chd).chunkready as ::core::ffi::c_int != 0 {
                    chd = (*chd).next as *mut chunkdata;
                }
                if !chd.is_null() {
                    let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                        pthread_cond_wait(&raw mut (*ind).chunkcond, &raw mut (*ind).lock);
                    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                            && *__errno_location() != 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1901 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_wait(&(ind->chunkcond),&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_1,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1901 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_wait(&(ind->chunkcond),&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_1,
                            );
                        } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                            && *__errno_location() == 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_0);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1901 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_wait(&(ind->chunkcond),&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_2,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1901 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_wait(&(ind->chunkcond),&(ind->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_2,
                            );
                        } else {
                            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                                strerr(_mfs_assert_ret_0);
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1901 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_wait(&(ind->chunkcond),&(ind->lock))\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_err_0,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1901 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_wait(&(ind->chunkcond),&(ind->lock))\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_err_0,
                            );
                        }
                        abort();
                    }
                }
            }
            if !((*ind).status == 0 as ::core::ffi::c_int && !chd.is_null()) {
                break;
            }
        }
        chd = (*ind).chunks;
        while !chd.is_null() {
            (*chd).unbreakable = 1 as uint8_t;
            chd = (*chd).next as *mut chunkdata;
        }
        ret = (*ind).status;
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*ind).lock);
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1913 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1913 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1913 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1913 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1913 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1913 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        return ret;
    }
}
unsafe extern "C" fn write_data_will_flush_wait(mut ind: *mut inodedata) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0;
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*ind).lock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1923 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1923 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1923 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1923 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1923 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1923 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        ret = (*ind).chunkscnt as ::core::ffi::c_int;
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*ind).lock);
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1925 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1925 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1925 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1925 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1925 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1925 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        return ret;
    }
}
unsafe extern "C" fn write_data_do_flush(
    mut ind: *mut inodedata,
    mut releaseflag: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0;
        let mut chd: *mut chunkdata = ::core::ptr::null_mut::<chunkdata>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*ind).lock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1937 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1937 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1937 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1937 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1937 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1937 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        (*ind).flushwaiting = (*ind).flushwaiting.wrapping_add(1);
        while (*ind).chunkscnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            chd = (*ind).chunks;
            while !chd.is_null() {
                if (*chd).waitingworker != 0 {
                    if write(
                        (*chd).wakeup_fd,
                        b" \0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        1 as size_t,
                    ) != 1 as ssize_t
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"can't write to pipe !!!\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    (*chd).waitingworker = 0 as uint8_t;
                    (*chd).wakeup_fd = -1 as ::core::ffi::c_int;
                }
                chd = (*chd).next as *mut chunkdata;
            }
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_cond_wait(&raw mut (*ind).flushcond, &raw mut (*ind).lock);
            if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1952 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(ind->flushcond),&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1952 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(ind->flushcond),&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1952 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(ind->flushcond),&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1952 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(ind->flushcond),&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                } else {
                    let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1952 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(ind->flushcond),&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1952 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(ind->flushcond),&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
        }
        (*ind).flushwaiting = (*ind).flushwaiting.wrapping_sub(1);
        if (*ind).flushwaiting as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*ind).writewaiting as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                pthread_cond_broadcast(&raw mut (*ind).writecond);
            if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1959 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(ind->writecond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_3,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1959 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(ind->writecond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_3,
                    );
                } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_4: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_1);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1959 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(ind->writecond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_4,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1959 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(ind->writecond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_4,
                    );
                } else {
                    let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_1);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1959 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(ind->writecond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_err_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1959 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(ind->writecond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_err_1,
                    );
                }
                abort();
            }
        }
        ret = (*ind).status;
        let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*ind).lock);
        if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1962 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1962 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
            } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_6: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1962 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1962 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
            } else {
                let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1962 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1962 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
        if releaseflag != 0 {
            write_free_inodedata(ind);
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_data_flush(mut vid: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    unsafe {
        if vid.is_null() {
            return EIO;
        }
        return write_data_do_flush(vid as *mut inodedata, 0 as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_data_chunk_wait(
    mut vid: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        if vid.is_null() {
            return EIO;
        }
        return write_data_do_chunk_wait(vid as *mut inodedata);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_data_inode_setmaxfleng(mut inode: uint32_t, mut maxfleng: uint64_t) {
    unsafe {
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        ind = write_find_inodedata(inode);
        if !ind.is_null() {
            let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*ind).lock);
            if _mfs_assert_ret != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1991 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1991 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1991 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1991 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                } else {
                    let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1991 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1991 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                }
                abort();
            }
            (*ind).maxfleng = maxfleng;
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut (*ind).lock);
            if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1993 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1993 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1993 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1993 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                } else {
                    let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1993 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1993 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
            write_free_inodedata(ind);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_data_inode_getmaxfleng(mut inode: uint32_t) -> uint64_t {
    unsafe {
        let mut maxfleng: uint64_t = 0;
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        ind = write_find_inodedata(inode);
        if !ind.is_null() {
            let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*ind).lock);
            if _mfs_assert_ret != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2003 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2003 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2003 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2003 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                } else {
                    let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2003 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2003 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                }
                abort();
            }
            maxfleng = (*ind).maxfleng;
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut (*ind).lock);
            if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2005 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2005 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2005 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2005 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                } else {
                    let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2005 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2005 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
            write_free_inodedata(ind);
        } else {
            maxfleng = 0 as uint64_t;
        }
        return maxfleng;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_data_getmaxfleng(mut vid: *mut ::core::ffi::c_void) -> uint64_t {
    unsafe {
        let mut maxfleng: uint64_t = 0;
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        if vid.is_null() {
            return 0 as uint64_t;
        }
        ind = vid as *mut inodedata;
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*ind).lock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2020 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2020 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2020 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2020 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2020 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2020 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        maxfleng = (*ind).maxfleng;
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*ind).lock);
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2022 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2022 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2022 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2022 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2022 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/writedata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2022 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ind->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        return maxfleng;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_data_flush_inode(mut inode: uint32_t) -> ::core::ffi::c_int {
    unsafe {
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        let mut ret: ::core::ffi::c_int = 0;
        ind = write_find_inodedata(inode);
        if ind.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        ret = write_data_do_flush(ind, 1 as uint8_t);
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_data_will_end_wait(
    mut vid: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        if !vid.is_null() {
            return write_data_will_flush_wait(vid as *mut inodedata);
        } else {
            return 0 as ::core::ffi::c_int;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_data_end(mut vid: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0;
        if vid.is_null() {
            return EIO;
        }
        ret = write_data_do_flush(vid as *mut inodedata, 1 as uint8_t);
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_init() {
    unsafe {
        write_get_total_bytes();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_term() {}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

// ponytail: c2rust dropped the _Atomic static (see mastercomm.rs note)
static mut total_bytes_sent: uint64_t = 0;
