pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use crate::pcqueue::{OwnedJob, QueueSlot};
use ::c2rust_bitfields;
unsafe extern "C" {
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn readv(
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
    unsafe fn abort() -> !;
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn __sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    unsafe fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn pthread_sigmask(
        __how: ::core::ffi::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
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
    unsafe fn univmakestrip(strip: *mut ::core::ffi::c_char, ip: uint32_t);
    unsafe fn tcpsocket() -> ::core::ffi::c_int;
    unsafe fn tcpnonblock(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpgetstatus(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnodelay(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnumbind(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpnumconnect(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
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
    unsafe fn delay_run(
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        udata: *mut ::core::ffi::c_void,
        useconds: uint64_t,
    );
    unsafe fn fs_getsrcip() -> uint32_t;
    unsafe fn fs_readchunk(
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
    unsafe fn master_version() -> uint32_t;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub type rrequest = rrequest_s;
// std::sync primitives replace pthread_cond_t: futex-based, no heap
// resources. Struct is Box-allocated in read_new_request (payload `data`
// is a Box<[u8]> leaked as a raw ptr); freed via Box::from_raw in
// read_delete_request.
#[repr(C)]
pub struct rrequest_s {
    pub ind: *mut inodedata_s,
    pub wakeup_fd: ::core::ffi::c_int,
    pub waitingworker: uint8_t,
    pub data: *mut uint8_t,
    pub offset: uint64_t,
    pub leng: uint32_t,
    pub rleng: uint32_t,
    pub currentpos: uint32_t,
    pub splitcurrpos: [uint32_t; 8],
    pub chindx: uint32_t,
    pub trycnt: uint32_t,
    pub modified: ::core::ffi::c_double,
    pub refresh: uint8_t,
    pub mode: uint8_t,
    pub lcnt: uint16_t,
    pub cond: std::sync::Condvar,
    // C: struct rrequest_s *next, **prev (per-inode reqhead/reqtail chain,
    // readdata.c:173) — replaced by inodedata_s.reqs below. Raw rrequest
    // handles escape (JQUEUE jobs, delay_run udata, rlist_s.rreq, the lcnt
    // protocol), so nodes stay Box::into_raw'd and reqs only enumerates.
}
// std::sync primitives replace pthread_cond_t/pthread_mutex_t (see
// rrequest_s note). Struct is Box-allocated in read_data_new; freed via
// Box::from_raw in read_inode_free / read_data_term.
#[repr(C)]
pub struct inodedata_s {
    pub inode: uint32_t,
    pub seqdata: uint32_t,
    pub fleng: uint64_t,
    pub status: ::core::ffi::c_int,
    pub closing: uint8_t,
    pub inqueue: uint8_t,
    pub readahead: uint8_t,
    pub lastoffset: uint64_t,
    pub waiting_writers: uint16_t,
    pub readers_cnt: uint16_t,
    pub lcnt: uint16_t,
    // C: rrequest *reqhead, **reqtail (readdata.c:196). Tail-append FIFO of
    // raw rrequest handles; push == C's `*(ind->reqtail) = rreq` tail link,
    // iteration from index 0 == walking from reqhead.
    pub reqs: Vec<*mut rrequest>,
    pub closecond: std::sync::Condvar,
    pub readerscond: std::sync::Condvar,
    pub writerscond: std::sync::Condvar,
    pub lock: std::sync::Mutex<()>,
    // C: struct inodedata_s *next (indhash bucket chain, readdata.c:202) —
    // replaced by the indhash Vec buckets below. Raw inodedata handles
    // escape (read_data_new returns the node as the opaque void* file
    // handle consumed by mfs_fuse), so nodes stay Box::into_raw'd and
    // buckets only enumerate.
}
pub type inodedata = inodedata_s;
pub const INQUEUE: C2Rust_Unnamed_0 = 1;
pub const NEW: C2Rust_Unnamed_0 = 0;
pub const READY: C2Rust_Unnamed_0 = 6;
pub const REFRESH: C2Rust_Unnamed_0 = 3;
pub const NOTNEEDED: C2Rust_Unnamed_0 = 7;
pub const BREAK: C2Rust_Unnamed_0 = 4;
pub const FILLED: C2Rust_Unnamed_0 = 5;
pub type data_source = _data_source;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _data_source {
    pub fd: ::core::ffi::c_int,
    pub startpos: uint32_t,
    pub currpos: uint32_t,
    pub endpos: uint32_t,
    pub port: uint16_t,
    pub ip: uint32_t,
    pub csver: uint32_t,
    pub gotstatus: uint8_t,
    pub sent: uint32_t,
    pub tosend: uint32_t,
    pub received: uint32_t,
    pub lastrcvd: ::core::ffi::c_double,
    pub lastsend: ::core::ffi::c_double,
    pub recvbuff: [uint8_t; 20],
    pub sendbuff: [uint8_t; 29],
    pub reccmd: uint32_t,
    pub recleng: uint32_t,
    pub state: data_source_state,
}
pub type data_source_state = _data_source_state;
pub type _data_source_state = ::core::ffi::c_uint;
pub const STATE_ERROR: _data_source_state = 3;
pub const STATE_CONNECTED: _data_source_state = 2;
pub const STATE_CONNECTING: _data_source_state = 1;
pub const STATE_IDLE: _data_source_state = 0;
pub const BUSY: C2Rust_Unnamed_0 = 2;
pub type rlist = rlist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlist_s {
    pub rreq: *mut rrequest,
    pub offsetadd: uint32_t,
    pub reqleng: uint32_t,
    // C: struct rlist_s *next (readdata.c:2137) — replaced by the Vec<rlist>
    // built in read_data and passed across the vrhead void* ABI as a boxed
    // Vec. Tail-append order preserved (push == C's *rtail link).
}
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
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
pub const __SC_THREAD_STACK_MIN_VALUE: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const SIG_BLOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIG_SETMASK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSCHUNKSIZE: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const MFSCHUNKMASK: ::core::ffi::c_int = 0x3ffffff as ::core::ffi::c_int;
pub const MFSCHUNKBITS: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
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
pub const MFS_ERROR_QUOTA: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const MFS_ERROR_CSNOTPRESENT: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const MFS_ERROR_EAGAIN: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const MFS_ERROR_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLTOCS_READ: ::core::ffi::c_int = PROTO_BASE + 200 as ::core::ffi::c_int;
pub const CSTOCL_READ_STATUS: ::core::ffi::c_int = PROTO_BASE + 201 as ::core::ffi::c_int;
pub const CSTOCL_READ_DATA: ::core::ffi::c_int = PROTO_BASE + 202 as ::core::ffi::c_int;
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
unsafe extern "C" fn get16bit(mut ptr: *mut *const uint8_t) -> uint16_t {
    unsafe {
        let mut t16: uint16_t = 0;
        memcpy(
            &raw mut t16 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            2 as size_t,
        );
        *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
        return t16.swap_bytes();
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
pub const CHUNKSERVER_ACTIVITY_TIMEOUT: ::core::ffi::c_double = 5.0f64;
pub const WORKER_BUSY_LAST_REQUEST_TIMEOUT: ::core::ffi::c_double = 5.0f64;
pub const WORKER_BUSY_WAIT_FOR_FINISH: ::core::ffi::c_double = 5.0f64;
pub const WORKER_BUSY_NOJOBS_INCREASE_TIMEOUT: ::core::ffi::c_double = 20.0f64;
pub const BUFFER_VALIDITY_TIMEOUT: ::core::ffi::c_double = 60.0f64;
pub const SUSTAIN_WORKERS: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const HEAVYLOAD_WORKERS: ::core::ffi::c_int = 150 as ::core::ffi::c_int;
pub const MAX_WORKERS: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
pub const IDHASHSIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const READAHEAD_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAXREQINQUEUE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_data_modename(mut mode: uint8_t) -> *mut ::core::ffi::c_char {
    match mode as ::core::ffi::c_int {
        0 => {
            return b"NEW\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        1 => {
            return b"INQUEUE\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        2 => {
            return b"BUSY\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3 => {
            return b"REFRESH\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        4 => {
            return b"BREAK\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        5 => {
            return b"FILLED\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        6 => {
            return b"READY\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        7 => {
            return b"NOTNEEDED\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        _ => {
            return b"<unknown>\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
    };
}
// C rangesstorage pthread_key (destructor: read_data_ranges_free = free).
// thread_local Vec frees itself at thread exit — same effect. Element 0
// mirrors C ranges[0] (edge capacity); len == ranges[0]+1, so etab ==
// element 1.. . C key_delete in read_data_term has no Rust equivalent:
// thread_locals die with their threads, the main thread's at process exit.
thread_local! {
    static RANGES: std::cell::RefCell<Vec<u64>> = const { std::cell::RefCell::new(Vec::new()) };
}
// C: pthread_getspecific(rangesstorage); if NULL malloc 11 u64 + ranges[0]=10.
// Rust alloc failure aborts, matching C passert. Returned pointer is used
// only until the next RANGES access — the same realloc-invalidation protocol
// as C, which re-fetched etab after every grow.
fn get_ranges() -> *mut uint64_t {
    RANGES.with(|r| {
        let mut r = r.borrow_mut();
        if r.is_empty() {
            r.resize(11, 0);
            r[0] = 10;
        }
        r.as_mut_ptr()
    })
}
// C: ranges[0] += 10; mfsrealloc(ranges, (ranges[0]+1)*8); setspecific.
fn grow_ranges() -> *mut uint64_t {
    RANGES.with(|r| {
        let mut r = r.borrow_mut();
        r[0] = r[0].wrapping_add(10);
        let len = (r[0] as usize).wrapping_add(1);
        r.resize(len, 0);
        r.as_mut_ptr()
    })
}
static mut readahead_leng: uint32_t = 0;
static mut readahead_trigger: uint32_t = 0;
static mut usectimeout: uint64_t = 0;
static mut maxretries: uint32_t = 0;
static mut minlogretry: uint32_t = 0;
static mut maxreadaheadsize: uint64_t = 0;
static mut erroronlostchunk: uint8_t = 0;
static mut erroronnospace: uint8_t = 0;
static mut reqbufftotalsize: uint64_t = 0;
// Owned fixed-size bucket table; replaces malloc'd `*mut *mut inodedata`.
// Buckets enumerate raw handles (nodes escape as the void* file handle);
// insert(0, ..) preserves C's head-insert in read_data_new. All bucket
// mutation happens under INODE_LOCK.
static mut indhash: [Vec<*mut inodedata>; IDHASHSIZE as usize] =
    [const { Vec::new() }; IDHASHSIZE as usize];
// Global inode-table lock. std::sync::Mutex is RAII-only, so emulate
// pthread-style manual lock/unlock by stashing the guard in a thread_local
// slot: lock() parks the guard, unlock() drops it.
// INVARIANT: every lock/unlock pair runs on the same thread (verified 9/9
// sites against readdata.c); unlock with no parked guard panics, catching
// cross-thread misuse. Poisoning is ignored (into_inner) for pthread parity.
// ponytail: guard-slot emulates pthread_mutex_t; upgrade path is a full RAII
// restructure of every lock region (large diff, separate wave).
static INODE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
thread_local! {
    static INODE_LOCK_GUARD: std::cell::RefCell<Option<std::sync::MutexGuard<'static, ()>>> =
        const { std::cell::RefCell::new(None) };
}
fn inode_global_lock() {
    let guard = INODE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    INODE_LOCK_GUARD.with(|slot| {
        let mut slot = slot.borrow_mut();
        assert!(slot.is_none(), "inode_global_lock: guard already held");
        *slot = Some(guard);
    });
}
fn inode_global_unlock() {
    let guard = INODE_LOCK_GUARD
        .with(|slot| slot.borrow_mut().take())
        .expect("inode_global_unlock: no guard held on this thread");
    drop(guard);
}
// Per-inode lock: C ind->lock (pthread_mutex_t) with readerscond/closecond/
// writerscond/rrequest.cond all waiting on it. Same guard-slot emulation as
// INODE_LOCK, but the slot also carries the mutex address so unlock/wait
// with a mismatched ind panics (pthread would UB).
// HOLD-OVERLAP AUDIT (all 28 lock sites vs readdata.c): no thread ever holds
// two different inodedata locks at once. read_worker/read_job_end/read_data/
// read_data_end/read_data_free_buff/read_data_set_length_active each touch
// one ind; read_inode_clear_cache/read_inode_set_length_passive hold
// INODE_LOCK + one ind->lock (different slots) and release ind->lock before
// moving to the next hash entry; read_data_term locks/unlocks each ind
// sequentially. Single slot suffices.
// SAFETY (guard lifetime): the guard borrows (*ind).lock inside malloc'd
// inodedata; transmuted to 'static. Sound because the guard is only dropped
// via ind_unlock/ind_cond_* on the same thread, and inodedata is freed
// (read_inode_free/read_data_term) only under the C refcount protocol (lcnt
// under INODE_LOCK) which guarantees no holder or waiter survives.
// Poisoning is ignored (into_inner) for pthread parity.
thread_local! {
    static IND_LOCK_GUARD: std::cell::RefCell<
        Option<(*const std::sync::Mutex<()>, std::sync::MutexGuard<'static, ()>)>,
    > = const { std::cell::RefCell::new(None) };
}
unsafe fn ind_lock(ind: *mut inodedata) {
    unsafe {
        let guard = (*ind).lock.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: see IND_LOCK_GUARD invariant above.
        let guard: std::sync::MutexGuard<'static, ()> = std::mem::transmute(guard);
        IND_LOCK_GUARD.with(|slot| {
            let mut slot = slot.borrow_mut();
            assert!(slot.is_none(), "ind_lock: guard already held");
            *slot = Some((&raw const (*ind).lock, guard));
        });
    }
}
unsafe fn ind_unlock(ind: *mut inodedata) {
    unsafe {
        let entry = IND_LOCK_GUARD
            .with(|slot| slot.borrow_mut().take())
            .expect("ind_unlock: no guard held on this thread");
        assert!(
            entry.0 == &raw const (*ind).lock,
            "ind_unlock: ind pointer mismatch"
        );
        drop(entry.1);
    }
}
// Caller must hold ind->lock; wait releases and reacquires it, exactly like
// pthread_cond_wait(cond, &ind->lock). C uses while-predicate loops at every
// site, so spurious wakeups are already handled there.
unsafe fn ind_cond_wait(cond: *const std::sync::Condvar, ind: *mut inodedata) {
    unsafe {
        let entry = IND_LOCK_GUARD
            .with(|slot| slot.borrow_mut().take())
            .expect("ind_cond_wait: no guard held on this thread");
        assert!(
            entry.0 == &raw const (*ind).lock,
            "ind_cond_wait: ind pointer mismatch"
        );
        // SAFETY: cond is a live inodedata/rrequest field, kept alive by the
        // same refcount protocol as the lock.
        let guard = (*cond).wait(entry.1).unwrap_or_else(|e| e.into_inner());
        IND_LOCK_GUARD.with(|slot| {
            *slot.borrow_mut() = Some((entry.0, guard));
        });
    }
}
// Returns true on timeout (C: pthread_cond_timedwait == ETIMEDOUT). C
// computes abstime = gettimeofday + usectimeout usecs; a relative Duration
// of the same length is the identical timeout.
unsafe fn ind_cond_timedwait(
    cond: *const std::sync::Condvar,
    ind: *mut inodedata,
    dur: std::time::Duration,
) -> bool {
    unsafe {
        let entry = IND_LOCK_GUARD
            .with(|slot| slot.borrow_mut().take())
            .expect("ind_cond_timedwait: no guard held on this thread");
        assert!(
            entry.0 == &raw const (*ind).lock,
            "ind_cond_timedwait: ind pointer mismatch"
        );
        // SAFETY: see ind_cond_wait.
        let (guard, res) = (*cond)
            .wait_timeout(entry.1, dur)
            .unwrap_or_else(|e| e.into_inner());
        IND_LOCK_GUARD.with(|slot| {
            *slot.borrow_mut() = Some((entry.0, guard));
        });
        res.timed_out()
    }
}
// Worker-thread lifecycle pool: replaces the C workers_lock mutex,
// workers_avail/workers_total counters, worker_term_cond and worker_thattr
// with a Rust Mutex+Condvar and std::thread JoinHandles. C detached workers
// instead of joining; we keep handles and join in read_data_term after the
// pool has drained (workers exit on a null job from the closed queue).
struct ReadWorkerPoolState {
    avail: u32,
    total: u32,
    lastnotify: u32,
    stack_size: usize,
    handles: Vec<std::thread::JoinHandle<()>>,
}

struct ReadWorkerPool {
    state: std::sync::Mutex<ReadWorkerPoolState>,
    term_cond: std::sync::Condvar,
}

static READ_WORKER_POOL: ReadWorkerPool = ReadWorkerPool {
    state: std::sync::Mutex::new(ReadWorkerPoolState {
        avail: 0,
        total: 0,
        lastnotify: 0,
        stack_size: 0,
        handles: Vec::new(),
    }),
    term_cond: std::sync::Condvar::new(),
};

impl ReadWorkerPool {
    fn lock_state(&self) -> std::sync::MutexGuard<'_, ReadWorkerPoolState> {
        // pthread mutexes have no poisoning; mirror that.
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    // C: read_data_spawn_worker (called with the pool state locked).
    fn spawn_worker(&self, st: &mut ReadWorkerPoolState) {
        unsafe {
            let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
            let mut newset: sigset_t = sigset_t { __val: [0; 16] };
            sigemptyset(&raw mut newset);
            sigaddset(&raw mut newset, SIGTERM);
            sigaddset(&raw mut newset, SIGINT);
            sigaddset(&raw mut newset, SIGHUP);
            sigaddset(&raw mut newset, SIGQUIT);
            // Worker inherits the blocked signal mask, as with pthread_create.
            pthread_sigmask(SIG_BLOCK, &raw mut newset, &raw mut oldset);
            let res = std::thread::Builder::new()
                .stack_size(st.stack_size)
                .spawn(move || {
                    read_worker(::core::ptr::null_mut::<::core::ffi::c_void>());
                });
            pthread_sigmask(
                SIG_SETMASK,
                &raw mut oldset,
                ::core::ptr::null_mut::<__sigset_t>(),
            );
            // C: on pthread_create failure, return without touching counters.
            let handle = match res {
                Ok(handle) => handle,
                Err(_) => return,
            };
            st.handles.push(handle);
            st.avail = st.avail.wrapping_add(1);
            st.total = st.total.wrapping_add(1);
            if st.total.wrapping_rem(10 as u32) == 0 as u32 && st.total != st.lastnotify {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"read workers: %u+\0".as_ptr() as *const ::core::ffi::c_char,
                    st.total,
                );
                st.lastnotify = st.total;
            }
        }
    }

    // C: read_data_close_worker (called with the pool state locked).
    fn close_worker(&self, st: &mut ReadWorkerPoolState) {
        unsafe {
            st.avail = st.avail.wrapping_sub(1);
            st.total = st.total.wrapping_sub(1);
            if st.total == 0 as u32 {
                self.term_cond.notify_one();
            }
            if st.total.wrapping_rem(10 as u32) == 0 as u32 && st.total != st.lastnotify {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"read workers: %u-\0".as_ptr() as *const ::core::ffi::c_char,
                    st.total,
                );
                st.lastnotify = st.total;
            }
        }
    }
}
static JQUEUE: QueueSlot<OwnedJob<rrequest>> = QueueSlot::new();

unsafe fn read_queue_put(rreq: *mut rrequest) {
    // SAFETY: every enqueue receives one live Box allocation. Queue owns it
    // until dequeue; on closed/missing queue it is dropped here, matching C where
    // an unbounded queue always enqueued and queue_delete freed leftovers.
    let job = unsafe { OwnedJob::from_raw(rreq) };
    if let Some(queue) = JQUEUE.get() {
        if let Err(job) = queue.put(job) {
            drop(job);
        }
    } else {
        drop(job);
    }
}

#[inline]
unsafe extern "C" fn read_increase_total_bytes(mut v: uint32_t) {
    unsafe {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut total_bytes_rcvd,
            v as uint64_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_get_total_bytes() -> uint64_t {
    unsafe {
        let mut v: uint64_t = 0;
        v = ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut total_bytes_rcvd,
            0 as uint64_t,
        );
        return v;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_enqueue(rreq: *mut rrequest) {
    // SAFETY: caller transfers one live request to queue protocol.
    unsafe { read_queue_put(rreq) };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_delayrun_enqueue(udata: *mut ::core::ffi::c_void) {
    // SAFETY: delayrun invokes callback once with request supplied below.
    unsafe { read_queue_put(udata.cast()) };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_delayed_enqueue(mut rreq: *mut rrequest, mut usecs: uint32_t) {
    unsafe {
        if usecs > 0 as uint32_t {
            delay_run(
                Some(read_delayrun_enqueue as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
                rreq as *mut ::core::ffi::c_void,
                usecs as uint64_t,
            );
        } else {
            // SAFETY: caller transfers one live request directly to queue protocol.
            read_queue_put(rreq);
        };
    }
}
#[inline]
unsafe extern "C" fn read_new_request(
    mut ind: *mut inodedata,
    mut offset: *mut uint64_t,
    mut blockend: uint64_t,
) -> *mut rrequest {
    unsafe {
        let mut chunkoffset: uint64_t = 0;
        let mut chunkend: uint64_t = 0;
        let mut chunkleng: uint32_t = 0;
        let mut chindx: uint32_t = 0;
        if blockend > *offset {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr() as *const ::core::ffi::c_char,
                312 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"blockend>*offset\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr() as *const ::core::ffi::c_char,
                312 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"blockend>*offset\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        chunkoffset = *offset;
        chindx = (chunkoffset >> MFSCHUNKBITS) as uint32_t;
        chunkend = chindx as uint64_t;
        chunkend <<= MFSCHUNKBITS;
        chunkend = chunkend.wrapping_add(MFSCHUNKSIZE as uint64_t);
        if blockend > chunkend {
            chunkleng = chunkend.wrapping_sub(chunkoffset) as uint32_t;
            *offset = chunkend;
        } else {
            chunkleng = blockend.wrapping_sub(*offset) as uint32_t;
            *offset = blockend;
        }
        // C: malloc(chunkleng) + passert. Box<[u8]> leaked as raw ptr;
        // new_uninit_slice matches C's uninitialized payload (workers fill
        // it from the network before any read) and skips the zero-fill.
        let data: *mut uint8_t =
            Box::into_raw(Box::<[u8]>::new_uninit_slice(chunkleng as usize).assume_init())
                as *mut uint8_t;
        let mut rreq: *mut rrequest = Box::into_raw(Box::new(rrequest_s {
            ind: ind as *mut inodedata_s,
            wakeup_fd: -1 as ::core::ffi::c_int,
            waitingworker: 0 as uint8_t,
            data,
            offset: chunkoffset,
            leng: chunkleng,
            rleng: 0 as uint32_t,
            currentpos: (chunkoffset & MFSCHUNKMASK as uint64_t) as uint32_t,
            splitcurrpos: [0 as uint32_t; 8],
            chindx,
            trycnt: 0 as uint32_t,
            modified: monotonic_seconds(),
            refresh: 0 as uint8_t,
            mode: NEW as ::core::ffi::c_int as uint8_t,
            lcnt: 0 as uint16_t,
            cond: std::sync::Condvar::new(),
        }));
        if ((*ind).inqueue as ::core::ffi::c_int) < MAXREQINQUEUE {
            (*rreq).mode = INQUEUE as ::core::ffi::c_int as uint8_t;
            read_enqueue(rreq);
            (*ind).inqueue = (*ind).inqueue.wrapping_add(1);
        }
        // C: rreq->next = NULL; rreq->prev = ind->reqtail;
        // *(ind->reqtail) = rreq; ind->reqtail = &(rreq->next); — tail append.
        (&mut (*ind).reqs).push(rreq);
        let c2rust_lhs = &raw mut reqbufftotalsize;
        let c2rust_rhs = chunkleng as uint64_t;
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            c2rust_lhs, c2rust_rhs,
        )
        .wrapping_add(c2rust_rhs);
        return rreq;
    }
}
#[inline]
unsafe extern "C" fn read_delete_request(mut rreq: *mut rrequest) {
    unsafe {
        // C: *(rreq->prev) = rreq->next; ... doubly-linked unlink. Scan for
        // the handle instead. ponytail: O(n) position lookup, n bounded by
        // requests-per-inode (readahead keeps it small); index-based links
        // only if profiling flags it.
        let reqs = &raw mut (*(*rreq).ind).reqs;
        let pos = (*reqs)
            .iter()
            .position(|&p| p == rreq)
            .expect("read_delete_request: rreq not in ind->reqs");
        (&mut *reqs).remove(pos);
        let c2rust_lhs = &raw mut reqbufftotalsize;
        let c2rust_rhs = (*rreq).leng as uint64_t;
        ::core::intrinsics::atomic_xsub::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            c2rust_lhs, c2rust_rhs,
        )
        .wrapping_sub(c2rust_rhs);
        // C: free(rreq->data) with leng == chunkleng at alloc time
        // (rreq->leng is never reassigned after read_new_request).
        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
            (*rreq).data,
            (*rreq).leng as usize,
        )));
        drop(Box::from_raw(rreq));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_job_end(
    mut rreq: *mut rrequest,
    mut status: ::core::ffi::c_int,
    mut delay: uint32_t,
) {
    unsafe {
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        let mut breakmode: uint8_t = 0;
        ind = (*rreq).ind as *mut inodedata;
        ind_lock(ind);
        breakmode = 0 as uint8_t;
        if (*rreq).mode as ::core::ffi::c_int == FILLED as ::core::ffi::c_int {
            (*rreq).mode = READY as ::core::ffi::c_int as uint8_t;
            (*rreq).trycnt = 0 as uint32_t;
            (*rreq).cond.notify_all();
        } else if (*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int {
            breakmode = 1 as uint8_t;
            (*rreq).mode = NOTNEEDED as ::core::ffi::c_int as uint8_t;
        } else if (*rreq).mode as ::core::ffi::c_int == REFRESH as ::core::ffi::c_int {
            delay = 0 as uint32_t;
            (*rreq).mode = NEW as ::core::ffi::c_int as uint8_t;
        } else {
            (*rreq).mode = NEW as ::core::ffi::c_int as uint8_t;
        }
        (*ind).inqueue = (*ind).inqueue.wrapping_sub(1);
        if status != 0 as ::core::ffi::c_int {
            if (*ind).closing as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                *__errno_location() = status;
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"error reading file number %u: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*ind).inode,
                    strerr(*__errno_location()),
                );
            }
            (*ind).status = status;
        }
        status = (*ind).status;
        if (*ind).closing as ::core::ffi::c_int != 0
            || status != 0 as ::core::ffi::c_int
            || breakmode as ::core::ffi::c_int != 0
        {
            if (*rreq).lcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                read_delete_request(rreq);
                if (*ind).closing as ::core::ffi::c_int != 0 && (&(*ind).reqs).is_empty() {
                    (*ind).closecond.notify_all();
                }
            } else if breakmode as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (*rreq).mode as ::core::ffi::c_int != READY as ::core::ffi::c_int
            {
                (*rreq).rleng = 0 as uint32_t;
                (*rreq).mode = READY as ::core::ffi::c_int as uint8_t;
                (*rreq).cond.notify_all();
            }
        } else {
            if (*rreq).mode as ::core::ffi::c_int == NEW as ::core::ffi::c_int {
                (*rreq).mode = INQUEUE as ::core::ffi::c_int as uint8_t;
                read_delayed_enqueue(rreq, delay);
                (*ind).inqueue = (*ind).inqueue.wrapping_add(1);
            }
            // C: for (rreq = ind->reqhead ; rreq && ind->inqueue < MAXREQINQUEUE
            // ; rreq=rreq->next) — head-to-tail scan, no removal inside.
            let mut ri: usize = 0;
            while ri < (&(*ind).reqs).len()
                && ((*ind).inqueue as ::core::ffi::c_int) < MAXREQINQUEUE
            {
                let qr: *mut rrequest = (&(*ind).reqs)[ri];
                if (*qr).mode as ::core::ffi::c_int == NEW as ::core::ffi::c_int {
                    (*qr).mode = INQUEUE as ::core::ffi::c_int as uint8_t;
                    read_enqueue(qr);
                    (*ind).inqueue = (*ind).inqueue.wrapping_add(1);
                }
                ri = ri.wrapping_add(1);
            }
        }
        ind_unlock(ind);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_worker(_arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut data: *mut rrequest = ::core::ptr::null_mut::<rrequest>();
        let mut datasrc: [data_source; 8] = [data_source {
            fd: 0,
            startpos: 0,
            currpos: 0,
            endpos: 0,
            port: 0,
            ip: 0,
            csver: 0,
            gotstatus: 0,
            sent: 0,
            tosend: 0,
            received: 0,
            lastrcvd: 0.,
            lastsend: 0.,
            recvbuff: [0; 20],
            sendbuff: [0; 29],
            reccmd: 0,
            recleng: 0,
            state: STATE_IDLE,
        }; 8];
        let mut i: ::core::ffi::c_int = 0;
        let mut pfd: [pollfd; 9] = [pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        }; 9];
        let mut resetpos: uint8_t = 0;
        let mut datacurrpos: uint32_t = 0;
        let mut siov: [iovec; 2] = [iovec {
            iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            iov_len: 0,
        }; 2];
        let mut pipebuff: [uint8_t; 1024] = [0; 1024];
        let mut pipefd: [::core::ffi::c_int; 2] = [0; 2];
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut inode: uint32_t = 0;
        let mut trycnt: uint32_t = 0;
        let mut connmaxtry: uint32_t = 0;
        let mut rleng: uint32_t = 0;
        let mut recchunkid: uint64_t = 0;
        let mut recblocknum: uint16_t = 0;
        let mut recoffset: uint16_t = 0;
        let mut recsize: uint32_t = 0;
        let mut reccrc: uint32_t = 0;
        let mut finished: uint8_t = 0;
        let mut desc: uint8_t = 0;
        let mut recstatus: uint8_t = 0;
        let mut notdone: uint8_t = 0;
        let mut readanything: uint8_t = 0;
        let mut chain: [cspri; 100] = [cspri {
            ip: 0,
            port: 0,
            version: 0,
            labelmask: 0,
            priority: 0,
        }; 100];
        let mut chainelements: uint16_t = 0;
        let mut cnt: uint8_t = 0;
        let mut cpart: uint8_t = 0;
        let mut part: uint8_t = 0;
        let mut parts: uint8_t = 0;
        let mut connect_status: data_source_state = STATE_IDLE;
        let mut chindx: uint32_t = 0;
        let mut srcip: uint32_t = 0;
        let mut mfleng: uint64_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut csdata: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut csdatasize: uint32_t = 0;
        let mut csdataver: uint8_t = 0;
        let mut rdstatus: uint8_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut csstrip: [::core::ffi::c_char; 16] = [0; 16];
        let mut reqsend: uint8_t = 0;
        let mut closing: uint8_t = 0;
        let mut mode: uint8_t = 0;
        let mut start: ::core::ffi::c_double = 0.;
        let mut now: ::core::ffi::c_double = 0.;
        let mut workingtime: ::core::ffi::c_double = 0.;
        let mut lrdiff: ::core::ffi::c_double = 0.;
        let mut timeoutadd: ::core::ffi::c_double = 0.;
        let mut firsttime: uint8_t = 1 as uint8_t;
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        let mut rreq: *mut rrequest = ::core::ptr::null_mut::<rrequest>();
        parts = 0 as uint8_t;
        csstrip[0 as usize] = 0 as ::core::ffi::c_char;
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
            part = 0 as uint8_t;
            while (part as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                if datasrc[part as usize].ip != 0
                    || datasrc[part as usize].port as ::core::ffi::c_int != 0
                {
                    crate::csdb::read_dec(datasrc[part as usize].ip, datasrc[part as usize].port);
                }
                part = part.wrapping_add(1);
            }
            parts = 0 as uint8_t;
            if firsttime as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let mut st = READ_WORKER_POOL.lock_state();
                st.avail = st.avail.wrapping_add(1);
                if st.avail > SUSTAIN_WORKERS as uint32_t {
                    READ_WORKER_POOL.close_worker(&mut st);
                    drop(st);
                    close_pipe(&raw mut pipefd as *mut ::core::ffi::c_int);
                    return NULL;
                }
            }
            firsttime = 0 as uint8_t;
            data = JQUEUE
                .get()
                .and_then(|queue| queue.get())
                .map_or(::core::ptr::null_mut(), OwnedJob::into_raw);
            let mut st = READ_WORKER_POOL.lock_state();
            if data.is_null() {
                READ_WORKER_POOL.close_worker(&mut st);
                drop(st);
                close_pipe(&raw mut pipefd as *mut ::core::ffi::c_int);
                return NULL;
            }
            st.avail = st.avail.wrapping_sub(1);
            if st.avail == 0 as uint32_t && st.total < MAX_WORKERS as uint32_t {
                READ_WORKER_POOL.spawn_worker(&mut st);
            }
            timeoutadd = if st.total > HEAVYLOAD_WORKERS as uint32_t {
                0.0f64
            } else {
                WORKER_BUSY_NOJOBS_INCREASE_TIMEOUT
            };
            drop(st);
            rreq = data;
            ind = (*rreq).ind as *mut inodedata;
            ind_lock(ind);
            if (*rreq).mode as ::core::ffi::c_int == INQUEUE as ::core::ffi::c_int {
                (*rreq).mode = BUSY as ::core::ffi::c_int as uint8_t;
            } else if (*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int {
                ind_unlock(ind);
                read_job_end(rreq, 0 as ::core::ffi::c_int, 0 as uint32_t);
                continue;
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"file: %u, index: %u - bad request state: %s (expected INQUEUE or BREAK)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*ind).inode,
                    (*rreq).chindx,
                    read_data_modename((*rreq).mode),
                );
                (*rreq).mode = BUSY as ::core::ffi::c_int as uint8_t;
            }
            chindx = (*rreq).chindx;
            status = (*ind).status;
            inode = (*ind).inode;
            trycnt = (*rreq).trycnt;
            if status != MFS_STATUS_OK {
                ind_unlock(ind);
                read_job_end(rreq, status, 0 as uint32_t);
            } else if (*ind).closing != 0 {
                ind_unlock(ind);
                read_job_end(rreq, 0 as ::core::ffi::c_int, 0 as uint32_t);
            } else {
                ind_unlock(ind);
                crate::chunkrwlock::read_lock(inode, chindx);
                mfleng = 0 as uint64_t;
                chunkid = 0 as uint64_t;
                version = 0 as uint32_t;
                csdataver = 0 as uint8_t;
                csdatasize = 1024 as uint32_t;
                if master_version()
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            74 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            74 as ::core::ffi::c_int
                        })) as uint32_t
                    && if let Some(cached) = crate::chunksdatacache::find(inode, chindx) {
                        if cached.csdata.len() <= pipebuff.len() {
                            chunkid = cached.chunkid;
                            version = cached.version;
                            csdataver = cached.csdataver;
                            csdatasize = cached.csdata.len() as uint32_t;
                            pipebuff[..cached.csdata.len()].copy_from_slice(&cached.csdata);
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                {
                    rdstatus = MFS_STATUS_OK as uint8_t;
                    csdata = &raw mut pipebuff as *mut uint8_t;
                    ind_lock(ind);
                    if (*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int {
                        ind_unlock(ind);
                        read_job_end(rreq, 0 as ::core::ffi::c_int, 0 as uint32_t);
                        crate::chunkrwlock::read_unlock(inode, chindx);
                        continue;
                    } else {
                        mfleng = (*ind).fleng;
                        ind_unlock(ind);
                    }
                } else {
                    rdstatus = fs_readchunk(
                        inode,
                        chindx,
                        0 as uint8_t,
                        &raw mut csdataver,
                        &raw mut mfleng,
                        &raw mut chunkid,
                        &raw mut version,
                        &raw mut csdata,
                        &raw mut csdatasize,
                    );
                    if rdstatus as ::core::ffi::c_int == MFS_STATUS_OK {
                        crate::chunksdatacache::insert(
                            inode,
                            chindx,
                            chunkid,
                            version,
                            csdataver,
                            ::core::slice::from_raw_parts(csdata, csdatasize as usize),
                        );
                        ind_lock(ind);
                        if (*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int {
                            ind_unlock(ind);
                            read_job_end(rreq, 0 as ::core::ffi::c_int, 0 as uint32_t);
                            crate::chunkrwlock::read_unlock(inode, chindx);
                            continue;
                        } else {
                            (*ind).fleng = mfleng;
                            ind_unlock(ind);
                        }
                    }
                }
                if rdstatus as ::core::ffi::c_int != MFS_STATUS_OK {
                    if rdstatus as ::core::ffi::c_int != MFS_ERROR_LOCKED
                        && rdstatus as ::core::ffi::c_int != MFS_ERROR_EAGAIN
                    {
                        if rdstatus as ::core::ffi::c_int == MFS_ERROR_ENOENT
                            || rdstatus as ::core::ffi::c_int == MFS_ERROR_EPERM
                            || rdstatus as ::core::ffi::c_int == MFS_ERROR_NOCHUNK
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_readchunk returned status: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(rdstatus),
                            );
                            read_job_end(rreq, EBADF, 0 as uint32_t);
                        } else if rdstatus as ::core::ffi::c_int == MFS_ERROR_INDEXTOOBIG {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_readchunk returned status: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(rdstatus),
                            );
                            read_job_end(rreq, EINVAL, 0 as uint32_t);
                        } else if rdstatus as ::core::ffi::c_int == MFS_ERROR_QUOTA {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_readchunk returned status: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(rdstatus),
                            );
                            read_job_end(rreq, EDQUOT, 0 as uint32_t);
                        } else if rdstatus as ::core::ffi::c_int == MFS_ERROR_NOSPACE
                            && erroronnospace as ::core::ffi::c_int != 0
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_readchunk returned status: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(rdstatus),
                            );
                            read_job_end(rreq, ENOSPC, 0 as uint32_t);
                        } else if rdstatus as ::core::ffi::c_int == MFS_ERROR_CHUNKLOST
                            && erroronlostchunk as ::core::ffi::c_int != 0
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_readchunk returned status: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(rdstatus),
                            );
                            read_job_end(rreq, ENXIO, 0 as uint32_t);
                        } else if rdstatus as ::core::ffi::c_int == MFS_ERROR_IO {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"file: %u, index: %u - fs_readchunk returned status: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                inode,
                                chindx,
                                mfsstrerr(rdstatus),
                            );
                            read_job_end(rreq, EIO, 0 as uint32_t);
                        } else {
                            ind_lock(ind);
                            if trycnt >= minlogretry {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"file: %u, index: %u - fs_readchunk returned status: %s\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    inode,
                                    chindx,
                                    mfsstrerr(rdstatus),
                                );
                            }
                            (*rreq).trycnt = (*rreq).trycnt.wrapping_add(1);
                            trycnt = (*rreq).trycnt;
                            if trycnt >= maxretries {
                                ind_unlock(ind);
                                if rdstatus as ::core::ffi::c_int == MFS_ERROR_NOCHUNKSERVERS
                                    || rdstatus as ::core::ffi::c_int == MFS_ERROR_NOSPACE
                                {
                                    read_job_end(rreq, ENOSPC, 0 as uint32_t);
                                } else if rdstatus as ::core::ffi::c_int == MFS_ERROR_CSNOTPRESENT
                                    || rdstatus as ::core::ffi::c_int == MFS_ERROR_CHUNKLOST
                                {
                                    read_job_end(rreq, ENXIO, 0 as uint32_t);
                                } else {
                                    read_job_end(rreq, EIO, 0 as uint32_t);
                                }
                            } else if (*rreq).mode as ::core::ffi::c_int
                                == BREAK as ::core::ffi::c_int
                            {
                                ind_unlock(ind);
                                read_job_end(rreq, 0 as ::core::ffi::c_int, 0 as uint32_t);
                            } else {
                                (*rreq).mode = INQUEUE as ::core::ffi::c_int as uint8_t;
                                ind_unlock(ind);
                                read_delayed_enqueue(
                                    rreq,
                                    (1000 as uint32_t).wrapping_add(if trycnt < 30 as uint32_t {
                                        trycnt
                                            .wrapping_sub(1 as uint32_t)
                                            .wrapping_mul(300000 as uint32_t)
                                    } else {
                                        10000000 as uint32_t
                                    }),
                                );
                            }
                        }
                    } else {
                        ind_lock(ind);
                        if (*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int {
                            ind_unlock(ind);
                            read_job_end(rreq, 0 as ::core::ffi::c_int, 0 as uint32_t);
                        } else {
                            (*rreq).mode = INQUEUE as ::core::ffi::c_int as uint8_t;
                            if (*rreq).trycnt <= 6 as uint32_t {
                                (*rreq).trycnt = (*rreq).trycnt.wrapping_add(1);
                            }
                            trycnt = (*rreq).trycnt;
                            ind_unlock(ind);
                            read_delayed_enqueue(
                                rreq,
                                (if trycnt <= 2 as uint32_t {
                                    1000 as ::core::ffi::c_int
                                } else if trycnt <= 6 as uint32_t {
                                    100000 as ::core::ffi::c_int
                                } else {
                                    500000 as ::core::ffi::c_int
                                }) as uint32_t,
                            );
                        }
                    }
                    crate::chunkrwlock::read_unlock(inode, chindx);
                } else if chunkid == 0 as uint64_t && version == 0 as uint32_t {
                    ind_lock(ind);
                    if (*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int {
                        ind_unlock(ind);
                    } else {
                        if (*rreq).offset > mfleng {
                            (*rreq).rleng = 0 as uint32_t;
                        } else if (*rreq).offset.wrapping_add((*rreq).leng as uint64_t) > mfleng {
                            (*rreq).rleng = mfleng.wrapping_sub((*rreq).offset) as uint32_t;
                        } else {
                            (*rreq).rleng = (*rreq).leng;
                        }
                        if (*rreq).rleng > 0 as uint32_t {
                            memset(
                                (*rreq).data as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                (*rreq).rleng as size_t,
                            );
                        }
                        ind_unlock(ind);
                        if !crate::chunksdatacache::check(inode, chindx, chunkid, version) {
                            ind_lock(ind);
                            (*rreq).currentpos =
                                ((*rreq).offset & MFSCHUNKMASK as uint64_t) as uint32_t;
                            (*rreq).mode = REFRESH as ::core::ffi::c_int as uint8_t;
                            ind_unlock(ind);
                        } else {
                            ind_lock(ind);
                            (*rreq).mode = FILLED as ::core::ffi::c_int as uint8_t;
                            (*rreq).modified = monotonic_seconds();
                            ind_unlock(ind);
                        }
                    }
                    read_job_end(rreq, 0 as ::core::ffi::c_int, 0 as uint32_t);
                    crate::chunkrwlock::read_unlock(inode, chindx);
                } else {
                    if !csdata.is_null() && csdatasize > 0 as uint32_t {
                        chainelements = csorder_sort(
                            &raw mut chain as *mut cspri,
                            csdataver,
                            csdata,
                            csdatasize,
                            0 as uint8_t,
                        ) as uint16_t;
                    } else {
                        chainelements = 0 as uint16_t;
                    }
                    if chainelements as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        ind_lock(ind);
                        if trycnt >= minlogretry {
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
                        (*rreq).trycnt = (*rreq).trycnt.wrapping_add(1);
                        trycnt = (*rreq).trycnt;
                        if trycnt >= maxretries {
                            ind_unlock(ind);
                            read_job_end(rreq, ENXIO, 0 as uint32_t);
                        } else if (*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int
                        {
                            ind_unlock(ind);
                            read_job_end(rreq, 0 as ::core::ffi::c_int, 0 as uint32_t);
                        } else {
                            (*rreq).mode = INQUEUE as ::core::ffi::c_int as uint8_t;
                            ind_unlock(ind);
                            crate::chunksdatacache::invalidate(inode, chindx);
                            read_delayed_enqueue(rreq, 10000000 as uint32_t);
                        }
                        crate::chunkrwlock::read_unlock(inode, chindx);
                    } else {
                        if csdataver as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
                            if chainelements as ::core::ffi::c_int != 8 as ::core::ffi::c_int
                                && chainelements as ::core::ffi::c_int != 4 as ::core::ffi::c_int
                            {
                                ind_lock(ind);
                                if trycnt >= minlogretry {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"file: %u, index: %u, chunk: %016lX, version: %u - split mode with wrong parts counter (%u)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        inode,
                                        chindx,
                                        chunkid,
                                        version,
                                        chainelements as ::core::ffi::c_int,
                                    );
                                }
                                (*rreq).trycnt = (*rreq).trycnt.wrapping_add(1);
                                trycnt = (*rreq).trycnt;
                                if trycnt >= maxretries {
                                    ind_unlock(ind);
                                    read_job_end(rreq, ENXIO, 0 as uint32_t);
                                } else if (*rreq).mode as ::core::ffi::c_int
                                    == BREAK as ::core::ffi::c_int
                                {
                                    ind_unlock(ind);
                                    read_job_end(rreq, 0 as ::core::ffi::c_int, 0 as uint32_t);
                                } else {
                                    (*rreq).mode = INQUEUE as ::core::ffi::c_int as uint8_t;
                                    ind_unlock(ind);
                                    crate::chunksdatacache::invalidate(inode, chindx);
                                    read_delayed_enqueue(rreq, 10000000 as uint32_t);
                                }
                                crate::chunkrwlock::read_unlock(inode, chindx);
                                continue;
                            } else {
                                parts = chainelements as uint8_t;
                            }
                        } else {
                            parts = 1 as uint8_t;
                        }
                        part = 0 as uint8_t;
                        while (part as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                            datasrc[part as usize].ip = chain[part as usize].ip;
                            datasrc[part as usize].port = chain[part as usize].port;
                            datasrc[part as usize].csver = chain[part as usize].version;
                            if datasrc[part as usize].ip == 0 as uint32_t
                                && datasrc[part as usize].port as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                            {
                                parts = 0 as uint8_t;
                            }
                            part = part.wrapping_add(1);
                        }
                        if parts as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            ind_lock(ind);
                            if trycnt >= minlogretry {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"file: %u, index: %u, chunk: %016lX, version: %u - there are no valid copies (bad ip and/or port)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    inode,
                                    chindx,
                                    chunkid,
                                    version,
                                );
                            }
                            (*rreq).trycnt = (*rreq).trycnt.wrapping_add(1);
                            trycnt = (*rreq).trycnt;
                            if trycnt >= maxretries {
                                ind_unlock(ind);
                                read_job_end(rreq, ENXIO, 0 as uint32_t);
                            } else if (*rreq).mode as ::core::ffi::c_int
                                == BREAK as ::core::ffi::c_int
                            {
                                ind_unlock(ind);
                                read_job_end(rreq, 0 as ::core::ffi::c_int, 0 as uint32_t);
                            } else {
                                (*rreq).mode = INQUEUE as ::core::ffi::c_int as uint8_t;
                                ind_unlock(ind);
                                crate::chunksdatacache::invalidate(inode, chindx);
                                read_delayed_enqueue(rreq, 10000000 as uint32_t);
                            }
                            crate::chunkrwlock::read_unlock(inode, chindx);
                        } else {
                            part = 0 as uint8_t;
                            while (part as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                                crate::csdb::read_inc(
                                    datasrc[part as usize].ip,
                                    datasrc[part as usize].port,
                                );
                                part = part.wrapping_add(1);
                            }
                            start = monotonic_seconds();
                            srcip = fs_getsrcip();
                            connect_status = STATE_CONNECTED;
                            part = 0 as uint8_t;
                            while (part as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                                datasrc[part as usize].fd = conncache_get(
                                    datasrc[part as usize].ip,
                                    datasrc[part as usize].port,
                                );
                                if datasrc[part as usize].fd < 0 as ::core::ffi::c_int {
                                    connect_status = STATE_CONNECTING;
                                    datasrc[part as usize].state = STATE_IDLE;
                                } else {
                                    datasrc[part as usize].state = STATE_CONNECTED;
                                }
                                part = part.wrapping_add(1);
                            }
                            connmaxtry = trycnt
                                .wrapping_mul(2 as uint32_t)
                                .wrapping_add(2 as uint32_t);
                            if connmaxtry > 10 as uint32_t {
                                connmaxtry = 10 as uint32_t;
                            }
                            cnt = 0 as uint8_t;
                            while (cnt as uint32_t) < connmaxtry
                                && connect_status as ::core::ffi::c_uint
                                    == STATE_CONNECTING as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                let mut newconnection: uint8_t = 0;
                                newconnection = 0 as uint8_t;
                                part = 0 as uint8_t;
                                while (part as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                                    if datasrc[part as usize].state as ::core::ffi::c_uint
                                        == STATE_IDLE as ::core::ffi::c_int as ::core::ffi::c_uint
                                    {
                                        let mut cres: ::core::ffi::c_int = 0;
                                        datasrc[part as usize].fd = tcpsocket();
                                        if datasrc[part as usize].fd < 0 as ::core::ffi::c_int {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"readworker: can't create tcp socket: %s\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                strerr(*__errno_location()),
                                            );
                                            datasrc[part as usize].state = STATE_ERROR;
                                            connect_status = STATE_ERROR;
                                            break;
                                        } else if tcpnonblock(datasrc[part as usize].fd)
                                            < 0 as ::core::ffi::c_int
                                        {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"readworker: can't set socket to non blocking mode: %s\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                strerr(*__errno_location()),
                                            );
                                            tcpclose(datasrc[part as usize].fd);
                                            datasrc[part as usize].state = STATE_ERROR;
                                            connect_status = STATE_ERROR;
                                            break;
                                        } else {
                                            if srcip != 0 {
                                                if tcpnumbind(
                                                    datasrc[part as usize].fd,
                                                    srcip,
                                                    0 as uint16_t,
                                                ) < 0 as ::core::ffi::c_int
                                                {
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_WARNING,
                                                        b"readworker: can't bind socket to given ip: %s\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        strerr(*__errno_location()),
                                                    );
                                                    tcpclose(datasrc[part as usize].fd);
                                                    datasrc[part as usize].state = STATE_ERROR;
                                                    connect_status = STATE_ERROR;
                                                    break;
                                                }
                                            }
                                            cres = tcpnumconnect(
                                                datasrc[part as usize].fd,
                                                datasrc[part as usize].ip,
                                                datasrc[part as usize].port,
                                            );
                                            if cres < 0 as ::core::ffi::c_int {
                                                let mut err: ::core::ffi::c_int =
                                                    *__errno_location();
                                                ind_lock(ind);
                                                if trycnt >= minlogretry {
                                                    univmakestrip(
                                                        &raw mut csstrip
                                                            as *mut ::core::ffi::c_char,
                                                        datasrc[part as usize].ip,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_WARNING,
                                                        b"readworker: can't connect to (%s:%hu): %s\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        &raw mut csstrip as *mut ::core::ffi::c_char,
                                                        datasrc[part as usize].port as ::core::ffi::c_int,
                                                        strerr(err),
                                                    );
                                                }
                                                ind_unlock(ind);
                                                tcpclose(datasrc[part as usize].fd);
                                                datasrc[part as usize].state = STATE_IDLE;
                                                newconnection = 1 as uint8_t;
                                            } else if cres == 0 as ::core::ffi::c_int {
                                                datasrc[part as usize].state = STATE_CONNECTED;
                                            } else {
                                                datasrc[part as usize].state = STATE_CONNECTING;
                                            }
                                        }
                                    }
                                    part = part.wrapping_add(1);
                                }
                                if connect_status as ::core::ffi::c_uint
                                    == STATE_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    break;
                                }
                                desc = 0 as uint8_t;
                                part = 0 as uint8_t;
                                while (part as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                                    if datasrc[part as usize].state as ::core::ffi::c_uint
                                        == STATE_CONNECTING as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        pfd[desc as usize].fd = datasrc[part as usize].fd;
                                        pfd[desc as usize].events = POLLOUT as ::core::ffi::c_short;
                                        pfd[desc as usize].revents = 0 as ::core::ffi::c_short;
                                        desc = desc.wrapping_add(1);
                                    }
                                    part = part.wrapping_add(1);
                                }
                                if desc as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                    if poll(
                                        &raw mut pfd as *mut pollfd,
                                        desc as nfds_t,
                                        if cnt as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0
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
                                        },
                                    ) < 0 as ::core::ffi::c_int
                                    {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_WARNING,
                                            b"readworker: poll error: %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            strerr(*__errno_location()),
                                        );
                                        connect_status = STATE_ERROR;
                                        break;
                                    } else {
                                        finished = 0 as uint8_t;
                                        desc = 0 as uint8_t;
                                        part = 0 as uint8_t;
                                        while (part as ::core::ffi::c_int)
                                            < parts as ::core::ffi::c_int
                                        {
                                            if datasrc[part as usize].state as ::core::ffi::c_uint
                                                == STATE_CONNECTING as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                if pfd[desc as usize].revents as ::core::ffi::c_int
                                                    & (POLLOUT | POLLERR | POLLHUP)
                                                    != 0
                                                {
                                                    if tcpgetstatus(datasrc[part as usize].fd) != 0
                                                    {
                                                        let mut err_0: ::core::ffi::c_int =
                                                            *__errno_location();
                                                        if trycnt >= minlogretry {
                                                            univmakestrip(
                                                                &raw mut csstrip
                                                                    as *mut ::core::ffi::c_char,
                                                                datasrc[part as usize].ip,
                                                            );
                                                            mfs_log(
                                                                MFSLOG_SYSLOG,
                                                                MFSLOG_WARNING,
                                                                b"readworker: can't connect to (%s:%hu): %s\0".as_ptr()
                                                                    as *const ::core::ffi::c_char,
                                                                &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                datasrc[part as usize].port as ::core::ffi::c_int,
                                                                strerr(err_0),
                                                            );
                                                        }
                                                        tcpclose(datasrc[part as usize].fd);
                                                        datasrc[part as usize].state = STATE_IDLE;
                                                        newconnection = 1 as uint8_t;
                                                    } else {
                                                        datasrc[part as usize].state =
                                                            STATE_CONNECTED;
                                                    }
                                                    finished = 1 as uint8_t;
                                                }
                                                desc = desc.wrapping_add(1);
                                            }
                                            part = part.wrapping_add(1);
                                        }
                                        if finished as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                        {
                                            desc = 0 as uint8_t;
                                            part = 0 as uint8_t;
                                            while (part as ::core::ffi::c_int)
                                                < parts as ::core::ffi::c_int
                                            {
                                                if datasrc[part as usize].state
                                                    as ::core::ffi::c_uint
                                                    == STATE_CONNECTING as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                {
                                                    tcpclose(datasrc[part as usize].fd);
                                                    datasrc[part as usize].state = STATE_IDLE;
                                                    newconnection = 1 as uint8_t;
                                                    desc = desc.wrapping_add(1);
                                                }
                                                part = part.wrapping_add(1);
                                            }
                                        }
                                    }
                                }
                                connect_status = STATE_CONNECTED;
                                part = 0 as uint8_t;
                                while (part as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                                    if datasrc[part as usize].state as ::core::ffi::c_uint
                                        == STATE_CONNECTING as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                        || datasrc[part as usize].state as ::core::ffi::c_uint
                                            == STATE_IDLE as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                    {
                                        connect_status = STATE_CONNECTING;
                                        break;
                                    } else {
                                        part = part.wrapping_add(1);
                                    }
                                }
                                if newconnection != 0 {
                                    cnt = cnt.wrapping_add(1);
                                }
                            }
                            if connect_status as ::core::ffi::c_uint
                                != STATE_CONNECTED as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                part = 0 as uint8_t;
                                while (part as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                                    if datasrc[part as usize].state as ::core::ffi::c_uint
                                        == STATE_CONNECTING as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                        || datasrc[part as usize].state as ::core::ffi::c_uint
                                            == STATE_CONNECTED as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                    {
                                        tcpclose(datasrc[part as usize].fd);
                                    }
                                    part = part.wrapping_add(1);
                                }
                                ind_lock(ind);
                                (*rreq).trycnt = (*rreq).trycnt.wrapping_add(1);
                                trycnt = (*rreq).trycnt;
                                if trycnt >= maxretries {
                                    ind_unlock(ind);
                                    read_job_end(rreq, EIO, 0 as uint32_t);
                                } else if (*rreq).mode as ::core::ffi::c_int
                                    == BREAK as ::core::ffi::c_int
                                {
                                    ind_unlock(ind);
                                    read_job_end(rreq, 0 as ::core::ffi::c_int, 0 as uint32_t);
                                } else {
                                    (*rreq).mode = INQUEUE as ::core::ffi::c_int as uint8_t;
                                    ind_unlock(ind);
                                    crate::chunksdatacache::invalidate(inode, chindx);
                                    read_delayed_enqueue(
                                        rreq,
                                        (1000 as uint32_t).wrapping_add(
                                            if trycnt < 30 as uint32_t {
                                                trycnt
                                                    .wrapping_sub(1 as uint32_t)
                                                    .wrapping_mul(300000 as uint32_t)
                                            } else {
                                                10000000 as uint32_t
                                            },
                                        ),
                                    );
                                }
                                crate::chunkrwlock::read_unlock(inode, chindx);
                            } else {
                                part = 0 as uint8_t;
                                while (part as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                                    if tcpnodelay(datasrc[part as usize].fd)
                                        < 0 as ::core::ffi::c_int
                                    {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_NOTICE,
                                            b"readworker: can't set TCP_NODELAY: %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            strerr(*__errno_location()),
                                        );
                                    }
                                    part = part.wrapping_add(1);
                                }
                                part = 0 as uint8_t;
                                while (part as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                                    datasrc[part as usize].gotstatus = 0 as uint8_t;
                                    datasrc[part as usize].received = 0 as uint32_t;
                                    datasrc[part as usize].sent = 0 as uint32_t;
                                    datasrc[part as usize].tosend = 0 as uint32_t;
                                    datasrc[part as usize].reccmd = 0 as uint32_t;
                                    datasrc[part as usize].recleng = 0 as uint32_t;
                                    datasrc[part as usize].lastrcvd = 0.0f64;
                                    datasrc[part as usize].lastsend = 0.0f64;
                                    datasrc[part as usize].startpos =
                                        0xffffffff as ::core::ffi::c_uint as uint32_t;
                                    datasrc[part as usize].currpos =
                                        0xffffffff as ::core::ffi::c_uint as uint32_t;
                                    datasrc[part as usize].endpos =
                                        0xffffffff as ::core::ffi::c_uint as uint32_t;
                                    part = part.wrapping_add(1);
                                }
                                reqsend = 0 as uint8_t;
                                ind_lock(ind);
                                resetpos = 0 as uint8_t;
                                readanything = 0 as uint8_t;
                                notdone = 0 as uint8_t;
                                ind_unlock(ind);
                                loop {
                                    now = monotonic_seconds();
                                    ind_lock(ind);
                                    mfleng = (*ind).fleng;
                                    if reqsend as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                        let mut startpos: uint32_t = 0;
                                        let mut currpos: uint32_t = 0;
                                        let mut endpos: uint32_t = 0;
                                        if (*rreq).offset > mfleng {
                                            (*rreq).rleng = 0 as uint32_t;
                                        } else if (*rreq)
                                            .offset
                                            .wrapping_add((*rreq).leng as uint64_t)
                                            > mfleng
                                        {
                                            (*rreq).rleng =
                                                mfleng.wrapping_sub((*rreq).offset) as uint32_t;
                                        } else {
                                            (*rreq).rleng = (*rreq).leng;
                                        }
                                        rleng = (*rreq).rleng;
                                        startpos =
                                            ((*rreq).offset & MFSCHUNKMASK as uint64_t) as uint32_t;
                                        currpos = (*rreq).currentpos;
                                        endpos = startpos.wrapping_add(rleng);
                                        if endpos <= 0x4000000 as uint32_t {
                                        } else {
                                            fprintf(
                                                stderr,
                                                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1273 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"endpos<=MFSCHUNKSIZE\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"endpos exceeded chunk size\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1273 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"endpos<=MFSCHUNKSIZE\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"endpos exceeded chunk size\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                            abort();
                                        };
                                        if currpos > endpos {
                                            currpos = endpos;
                                        }
                                        if parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                                            || parts as ::core::ffi::c_int
                                                == 4 as ::core::ffi::c_int
                                        {
                                            let mut firstcluster: uint32_t = 0;
                                            let mut currentcluster: uint32_t = 0;
                                            let mut lastcluster: uint32_t = 0;
                                            let mut firstpartoffset: uint32_t = 0;
                                            let mut currentpartoffset: uint32_t = 0;
                                            let mut lastpartoffset: uint32_t = 0;
                                            let mut firstpart: ::core::ffi::c_int = 0;
                                            let mut currentpart: ::core::ffi::c_int = 0;
                                            let mut lastpart: ::core::ffi::c_int = 0;
                                            let mut clusterbits: uint8_t = 0;
                                            let mut partmask: uint8_t = 0;
                                            if parts as ::core::ffi::c_int
                                                == 8 as ::core::ffi::c_int
                                            {
                                                clusterbits = 21 as uint8_t;
                                                partmask = 0x7 as uint8_t;
                                            } else {
                                                clusterbits = 20 as uint8_t;
                                                partmask = 0x3 as uint8_t;
                                            }
                                            firstcluster =
                                                startpos >> clusterbits as ::core::ffi::c_int;
                                            firstpart = (startpos >> 18 as ::core::ffi::c_int
                                                & partmask as uint32_t)
                                                as ::core::ffi::c_int;
                                            firstpartoffset = startpos & 0x3ffff as uint32_t;
                                            currentcluster =
                                                currpos >> clusterbits as ::core::ffi::c_int;
                                            currentpart = (currpos >> 18 as ::core::ffi::c_int
                                                & partmask as uint32_t)
                                                as ::core::ffi::c_int;
                                            currentpartoffset = currpos & 0x3ffff as uint32_t;
                                            lastcluster =
                                                endpos >> clusterbits as ::core::ffi::c_int;
                                            lastpart = (endpos >> 18 as ::core::ffi::c_int
                                                & partmask as uint32_t)
                                                as ::core::ffi::c_int;
                                            lastpartoffset = endpos & 0x3ffff as uint32_t;
                                            part = 0 as uint8_t;
                                            while (part as ::core::ffi::c_int)
                                                < parts as ::core::ffi::c_int
                                            {
                                                datasrc[part as usize].startpos =
                                                    firstcluster << 18 as ::core::ffi::c_int;
                                                if part as ::core::ffi::c_int == firstpart {
                                                    datasrc[part as usize].startpos = datasrc
                                                        [part as usize]
                                                        .startpos
                                                        .wrapping_add(firstpartoffset);
                                                } else if (part as ::core::ffi::c_int) < firstpart {
                                                    datasrc[part as usize].startpos = datasrc
                                                        [part as usize]
                                                        .startpos
                                                        .wrapping_add(0x40000 as uint32_t);
                                                }
                                                datasrc[part as usize].currpos =
                                                    currentcluster << 18 as ::core::ffi::c_int;
                                                if part as ::core::ffi::c_int == currentpart {
                                                    datasrc[part as usize].currpos = datasrc
                                                        [part as usize]
                                                        .currpos
                                                        .wrapping_add(currentpartoffset);
                                                } else if (part as ::core::ffi::c_int) < currentpart
                                                {
                                                    datasrc[part as usize].currpos = datasrc
                                                        [part as usize]
                                                        .currpos
                                                        .wrapping_add(0x40000 as uint32_t);
                                                }
                                                datasrc[part as usize].endpos =
                                                    lastcluster << 18 as ::core::ffi::c_int;
                                                if part as ::core::ffi::c_int == lastpart {
                                                    datasrc[part as usize].endpos = datasrc
                                                        [part as usize]
                                                        .endpos
                                                        .wrapping_add(lastpartoffset);
                                                } else if (part as ::core::ffi::c_int) < lastpart {
                                                    datasrc[part as usize].endpos = datasrc
                                                        [part as usize]
                                                        .endpos
                                                        .wrapping_add(0x40000 as uint32_t);
                                                }
                                                part = part.wrapping_add(1);
                                            }
                                            part = 0 as uint8_t;
                                            while (part as ::core::ffi::c_int)
                                                < parts as ::core::ffi::c_int
                                            {
                                                if (*rreq).splitcurrpos[part as usize]
                                                    > datasrc[part as usize].currpos
                                                {
                                                    datasrc[part as usize].currpos =
                                                        (*rreq).splitcurrpos[part as usize];
                                                }
                                                part = part.wrapping_add(1);
                                            }
                                        } else if parts as ::core::ffi::c_int
                                            == 1 as ::core::ffi::c_int
                                        {
                                            datasrc[0 as usize].startpos = startpos;
                                            datasrc[0 as usize].currpos = currpos;
                                            datasrc[0 as usize].endpos = endpos;
                                        } else {
                                            part = 0 as uint8_t;
                                            while (part as ::core::ffi::c_int)
                                                < parts as ::core::ffi::c_int
                                            {
                                                datasrc[part as usize].startpos = 0 as uint32_t;
                                                datasrc[part as usize].currpos = 0 as uint32_t;
                                                datasrc[part as usize].endpos = 0 as uint32_t;
                                                part = part.wrapping_add(1);
                                            }
                                        }
                                    }
                                    if (*rreq).mode as ::core::ffi::c_int
                                        == BREAK as ::core::ffi::c_int
                                    {
                                        ind_unlock(ind);
                                        status = EINTR;
                                        (*rreq).currentpos =
                                            ((*rreq).offset & MFSCHUNKMASK as uint64_t) as uint32_t;
                                        break;
                                    } else {
                                        finished = 1 as uint8_t;
                                        part = 0 as uint8_t;
                                        while (part as ::core::ffi::c_int)
                                            < parts as ::core::ffi::c_int
                                            && finished as ::core::ffi::c_int != 0
                                        {
                                            if datasrc[part as usize].gotstatus
                                                as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                            {
                                                finished = 0 as uint8_t;
                                            }
                                            part = part.wrapping_add(1);
                                        }
                                        if finished != 0 {
                                            ind_unlock(ind);
                                            if !crate::chunksdatacache::check(
                                                inode, chindx, chunkid, version,
                                            ) {
                                                ind_lock(ind);
                                                resetpos = 1 as uint8_t;
                                                (*rreq).mode =
                                                    REFRESH as ::core::ffi::c_int as uint8_t;
                                                ind_unlock(ind);
                                            } else {
                                                ind_lock(ind);
                                                (*rreq).mode =
                                                    FILLED as ::core::ffi::c_int as uint8_t;
                                                (*rreq).modified = monotonic_seconds();
                                                ind_unlock(ind);
                                            }
                                            break;
                                        } else {
                                            lrdiff = 0.0f64;
                                            cpart = 0 as uint8_t;
                                            part = 0 as uint8_t;
                                            while (part as ::core::ffi::c_int)
                                                < parts as ::core::ffi::c_int
                                            {
                                                if datasrc[part as usize].lastrcvd == 0.0f64 {
                                                    datasrc[part as usize].lastrcvd = now;
                                                } else {
                                                    let mut diff: ::core::ffi::c_double = 0.;
                                                    diff = now - datasrc[part as usize].lastrcvd;
                                                    if diff > lrdiff {
                                                        lrdiff = diff;
                                                        cpart = part;
                                                    }
                                                }
                                                part = part.wrapping_add(1);
                                            }
                                            if lrdiff >= CHUNKSERVER_ACTIVITY_TIMEOUT {
                                                if trycnt >= minlogretry {
                                                    univmakestrip(
                                                        &raw mut csstrip
                                                            as *mut ::core::ffi::c_char,
                                                        datasrc[cpart as usize].ip,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_WARNING,
                                                        b"file: %u, index: %u, chunk: %016lX, version: %u, part: %hhu - readworker: connection with (%s:%hu) was timed out (lastrcvd:%.6lf,now:%.6lf,lrdiff:%.6lf received: %u/%u, try counter: %u)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        inode,
                                                        chindx,
                                                        chunkid,
                                                        version,
                                                        cpart as ::core::ffi::c_int,
                                                        &raw mut csstrip as *mut ::core::ffi::c_char,
                                                        datasrc[cpart as usize].port as ::core::ffi::c_int,
                                                        datasrc[cpart as usize].lastrcvd,
                                                        now,
                                                        lrdiff,
                                                        ((*rreq).currentpos as uint64_t)
                                                            .wrapping_sub((*rreq).offset & MFSCHUNKMASK as uint64_t)
                                                            as uint32_t,
                                                        (*rreq).rleng,
                                                        trycnt.wrapping_add(1 as uint32_t),
                                                    );
                                                }
                                                status = EIO;
                                                ind_unlock(ind);
                                                break;
                                            } else {
                                                workingtime = now - start;
                                                if workingtime
                                                    > WORKER_BUSY_LAST_REQUEST_TIMEOUT
                                                        + WORKER_BUSY_WAIT_FOR_FINISH
                                                        + timeoutadd
                                                {
                                                    ind_unlock(ind);
                                                    status = EINTR;
                                                    break;
                                                } else {
                                                    if reqsend as ::core::ffi::c_int
                                                        == 0 as ::core::ffi::c_int
                                                    {
                                                        part = 0 as uint8_t;
                                                        while (part as ::core::ffi::c_int)
                                                            < parts as ::core::ffi::c_int
                                                        {
                                                            if datasrc[part as usize].endpos
                                                                > datasrc[part as usize].currpos
                                                            {
                                                                wptr = &raw mut (*(&raw mut datasrc
                                                                    as *mut data_source)
                                                                    .offset(part as isize))
                                                                .sendbuff
                                                                    as *mut uint8_t;
                                                                put32bit(
                                                                    &raw mut wptr,
                                                                    CLTOCS_READ as uint32_t,
                                                                );
                                                                if datasrc[part as usize].csver
                                                                    >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                                                        + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                                                        + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                                                            32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                                                        } else {
                                                                            32 as ::core::ffi::c_int
                                                                        })) as uint32_t
                                                                {
                                                                    put32bit(&raw mut wptr, 21 as uint32_t);
                                                                    put8bit(&raw mut wptr, 1 as uint8_t);
                                                                    datasrc[part as usize].tosend = 29 as uint32_t;
                                                                } else {
                                                                    put32bit(&raw mut wptr, 20 as uint32_t);
                                                                    datasrc[part as usize].tosend = 28 as uint32_t;
                                                                }
                                                                if parts as ::core::ffi::c_int
                                                                    == 1 as ::core::ffi::c_int
                                                                {
                                                                    put64bit(
                                                                        &raw mut wptr,
                                                                        chunkid,
                                                                    );
                                                                } else if parts
                                                                    as ::core::ffi::c_int
                                                                    == 8 as ::core::ffi::c_int
                                                                {
                                                                    put64bit(
                                                                        &raw mut wptr,
                                                                        chunkid & 0xffffffffffffff as uint64_t
                                                                            | ((0x20 as ::core::ffi::c_int | part as ::core::ffi::c_int)
                                                                                as uint64_t) << 56 as ::core::ffi::c_int,
                                                                    );
                                                                } else {
                                                                    put64bit(
                                                                        &raw mut wptr,
                                                                        chunkid & 0xffffffffffffff as uint64_t
                                                                            | ((0x10 as ::core::ffi::c_int | part as ::core::ffi::c_int)
                                                                                as uint64_t) << 56 as ::core::ffi::c_int,
                                                                    );
                                                                }
                                                                put32bit(&raw mut wptr, version);
                                                                put32bit(
                                                                    &raw mut wptr,
                                                                    datasrc[part as usize].currpos,
                                                                );
                                                                put32bit(
                                                                    &raw mut wptr,
                                                                    datasrc[part as usize]
                                                                        .endpos
                                                                        .wrapping_sub(
                                                                            datasrc[part as usize]
                                                                                .currpos,
                                                                        ),
                                                                );
                                                                datasrc[part as usize].sent =
                                                                    0 as uint32_t;
                                                            } else {
                                                                datasrc[part as usize].gotstatus =
                                                                    1 as uint8_t;
                                                            }
                                                            part = part.wrapping_add(1);
                                                        }
                                                        reqsend = 1 as uint8_t;
                                                    }
                                                    (*rreq).waitingworker = 1 as uint8_t;
                                                    (*rreq).wakeup_fd = pipefd[1 as usize];
                                                    ind_unlock(ind);
                                                    part = 0 as uint8_t;
                                                    while (part as ::core::ffi::c_int)
                                                        < parts as ::core::ffi::c_int
                                                    {
                                                        if datasrc[part as usize].tosend
                                                            == 0 as uint32_t
                                                            && now - datasrc[part as usize].lastsend
                                                                > 1.0f64
                                                        {
                                                            wptr = &raw mut (*(&raw mut datasrc
                                                                as *mut data_source)
                                                                .offset(part as isize))
                                                            .sendbuff
                                                                as *mut uint8_t;
                                                            put32bit(
                                                                &raw mut wptr,
                                                                ANTOAN_NOP as uint32_t,
                                                            );
                                                            put32bit(&raw mut wptr, 0 as uint32_t);
                                                            datasrc[part as usize].tosend =
                                                                8 as uint32_t;
                                                            datasrc[part as usize].sent =
                                                                0 as uint32_t;
                                                        }
                                                        if datasrc[part as usize].tosend
                                                            > 0 as uint32_t
                                                        {
                                                            i = write(
                                                                datasrc[part as usize].fd,
                                                                (&raw mut (*(&raw mut datasrc
                                                                    as *mut data_source)
                                                                    .offset(part as isize))
                                                                .sendbuff
                                                                    as *mut uint8_t)
                                                                    .offset(
                                                                        datasrc[part as usize].sent
                                                                            as isize,
                                                                    )
                                                                    as *const ::core::ffi::c_void,
                                                                datasrc[part as usize]
                                                                    .tosend
                                                                    .wrapping_sub(
                                                                        datasrc[part as usize].sent,
                                                                    )
                                                                    as size_t,
                                                            )
                                                                as ::core::ffi::c_int;
                                                            if i < 0 as ::core::ffi::c_int {
                                                                if *__errno_location() != EAGAIN
                                                                    && *__errno_location()
                                                                        != EWOULDBLOCK
                                                                    && *__errno_location() != EINTR
                                                                {
                                                                    if trycnt >= minlogretry {
                                                                        let mut err_1: ::core::ffi::c_int = *__errno_location();
                                                                        univmakestrip(
                                                                            &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                            datasrc[part as usize].ip,
                                                                        );
                                                                        mfs_log(
                                                                            MFSLOG_SYSLOG,
                                                                            MFSLOG_WARNING,
                                                                            b"file: %u, index: %u, chunk: %016lX, version: %u, part: %hhu - readworker: write to (%s:%hu) error: %s (received: %u/%u; try counter: %u)\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                            inode,
                                                                            chindx,
                                                                            chunkid,
                                                                            version,
                                                                            part as ::core::ffi::c_int,
                                                                            &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                            datasrc[part as usize].port as ::core::ffi::c_int,
                                                                            strerr(err_1),
                                                                            datasrc[part as usize]
                                                                                .currpos
                                                                                .wrapping_sub(datasrc[part as usize].startpos),
                                                                            datasrc[part as usize]
                                                                                .endpos
                                                                                .wrapping_sub(datasrc[part as usize].startpos),
                                                                            trycnt.wrapping_add(1 as uint32_t),
                                                                        );
                                                                    }
                                                                    status = EIO;
                                                                    ind_lock(ind);
                                                                    (*rreq).waitingworker =
                                                                        0 as uint8_t;
                                                                    (*rreq).wakeup_fd =
                                                                        -1 as ::core::ffi::c_int;
                                                                    ind_unlock(ind);
                                                                    break;
                                                                } else {
                                                                    i = 0 as ::core::ffi::c_int;
                                                                }
                                                            }
                                                            if i > 0 as ::core::ffi::c_int {
                                                                datasrc[part as usize].sent =
                                                                    datasrc[part as usize]
                                                                        .sent
                                                                        .wrapping_add(
                                                                            i as uint32_t,
                                                                        );
                                                                if datasrc[part as usize].tosend
                                                                    <= datasrc[part as usize].sent
                                                                {
                                                                    datasrc[part as usize].sent =
                                                                        0 as uint32_t;
                                                                    datasrc[part as usize].tosend =
                                                                        0 as uint32_t;
                                                                }
                                                                datasrc[part as usize].lastsend =
                                                                    now;
                                                            }
                                                        }
                                                        part = part.wrapping_add(1);
                                                    }
                                                    if status == EIO {
                                                        break;
                                                    }
                                                    pfd[0 as usize].fd = pipefd[0 as usize];
                                                    pfd[0 as usize].events =
                                                        POLLIN as ::core::ffi::c_short;
                                                    pfd[0 as usize].revents =
                                                        0 as ::core::ffi::c_short;
                                                    desc = 1 as uint8_t;
                                                    part = 0 as uint8_t;
                                                    while (part as ::core::ffi::c_int)
                                                        < parts as ::core::ffi::c_int
                                                    {
                                                        if datasrc[part as usize].tosend
                                                            > 0 as uint32_t
                                                            || datasrc[part as usize].gotstatus
                                                                as ::core::ffi::c_int
                                                                == 0 as ::core::ffi::c_int
                                                        {
                                                            pfd[desc as usize].fd =
                                                                datasrc[part as usize].fd;
                                                            pfd[desc as usize].events =
                                                                POLLIN as ::core::ffi::c_short;
                                                            pfd[desc as usize].revents =
                                                                0 as ::core::ffi::c_short;
                                                            if datasrc[part as usize].tosend
                                                                > 0 as uint32_t
                                                            {
                                                                pfd[desc as usize].events =
                                                                    (pfd[desc as usize].events
                                                                        as ::core::ffi::c_int
                                                                        | POLLOUT)
                                                                        as ::core::ffi::c_short;
                                                            }
                                                            desc = desc.wrapping_add(1);
                                                        }
                                                        part = part.wrapping_add(1);
                                                    }
                                                    if poll(
                                                        &raw mut pfd as *mut pollfd,
                                                        desc as nfds_t,
                                                        100 as ::core::ffi::c_int,
                                                    ) < 0 as ::core::ffi::c_int
                                                    {
                                                        if *__errno_location() != EINTR {
                                                            if trycnt >= minlogretry {
                                                                part = 0 as uint8_t;
                                                                while (part as ::core::ffi::c_int)
                                                                    < parts as ::core::ffi::c_int
                                                                {
                                                                    mfs_log(
                                                                        MFSLOG_SYSLOG,
                                                                        MFSLOG_WARNING,
                                                                        b"file: %u, index: %u, chunk: %016lX, version: %u, part: %hhu - readworker: poll error: %s (received: %u/%u; try counter: %u)\0"
                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                        inode,
                                                                        chindx,
                                                                        chunkid,
                                                                        version,
                                                                        part as ::core::ffi::c_int,
                                                                        strerr(*__errno_location()),
                                                                        datasrc[part as usize]
                                                                            .currpos
                                                                            .wrapping_sub(datasrc[part as usize].startpos),
                                                                        datasrc[part as usize]
                                                                            .endpos
                                                                            .wrapping_sub(datasrc[part as usize].startpos),
                                                                        trycnt.wrapping_add(1 as uint32_t),
                                                                    );
                                                                    part = part.wrapping_add(1);
                                                                }
                                                            }
                                                            status = EIO;
                                                            break;
                                                        }
                                                    }
                                                    ind_lock(ind);
                                                    (*rreq).waitingworker = 0 as uint8_t;
                                                    (*rreq).wakeup_fd = -1 as ::core::ffi::c_int;
                                                    closing = (if (*ind).closing
                                                        as ::core::ffi::c_int
                                                        > 0 as ::core::ffi::c_int
                                                    {
                                                        1 as ::core::ffi::c_int
                                                    } else {
                                                        0 as ::core::ffi::c_int
                                                    })
                                                        as uint8_t;
                                                    mode = (*rreq).mode;
                                                    ind_unlock(ind);
                                                    if pfd[0 as usize].revents as ::core::ffi::c_int
                                                        & POLLIN
                                                        != 0
                                                    {
                                                        i = read(
                                                            pipefd[0 as usize],
                                                            &raw mut pipebuff as *mut uint8_t
                                                                as *mut ::core::ffi::c_void,
                                                            1024 as size_t,
                                                        )
                                                            as ::core::ffi::c_int;
                                                        if i < 0 as ::core::ffi::c_int {
                                                            if trycnt >= minlogretry {
                                                                part = 0 as uint8_t;
                                                                while (part as ::core::ffi::c_int)
                                                                    < parts as ::core::ffi::c_int
                                                                {
                                                                    mfs_log(
                                                                        MFSLOG_SYSLOG,
                                                                        MFSLOG_WARNING,
                                                                        b"file: %u, index: %u, chunk: %016lX, version: %u, part: %hhu - readworker: read pipe error: %s (received: %u/%u; try counter: %u)\0"
                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                        inode,
                                                                        chindx,
                                                                        chunkid,
                                                                        version,
                                                                        part as ::core::ffi::c_int,
                                                                        strerr(*__errno_location()),
                                                                        datasrc[part as usize]
                                                                            .currpos
                                                                            .wrapping_sub(datasrc[part as usize].startpos),
                                                                        datasrc[part as usize]
                                                                            .endpos
                                                                            .wrapping_sub(datasrc[part as usize].startpos),
                                                                        trycnt.wrapping_add(1 as uint32_t),
                                                                    );
                                                                    part = part.wrapping_add(1);
                                                                }
                                                            }
                                                        }
                                                    }
                                                    if mode as ::core::ffi::c_int
                                                        != BUSY as ::core::ffi::c_int
                                                    {
                                                        status = EINTR;
                                                        resetpos = 1 as uint8_t;
                                                        break;
                                                    } else if closing != 0 {
                                                        status = EINTR;
                                                        resetpos = 1 as uint8_t;
                                                        break;
                                                    } else {
                                                        desc = 1 as uint8_t;
                                                        part = 0 as uint8_t;
                                                        while (part as ::core::ffi::c_int)
                                                            < parts as ::core::ffi::c_int
                                                        {
                                                            if datasrc[part as usize].tosend
                                                                > 0 as uint32_t
                                                                || datasrc[part as usize].gotstatus
                                                                    as ::core::ffi::c_int
                                                                    == 0 as ::core::ffi::c_int
                                                            {
                                                                if pfd[desc as usize].revents
                                                                    as ::core::ffi::c_int
                                                                    & POLLHUP
                                                                    != 0
                                                                {
                                                                    if trycnt >= minlogretry {
                                                                        univmakestrip(
                                                                            &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                            datasrc[part as usize].ip,
                                                                        );
                                                                        mfs_log(
                                                                            MFSLOG_SYSLOG,
                                                                            MFSLOG_WARNING,
                                                                            b"file: %u, index: %u, chunk: %016lX, version: %u - readworker: connection with (%s:%hu) was reset by peer / POLLHUP (received: %u/%u; try counter: %u)\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                            inode,
                                                                            chindx,
                                                                            chunkid,
                                                                            version,
                                                                            &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                            datasrc[part as usize].port as ::core::ffi::c_int,
                                                                            datasrc[part as usize]
                                                                                .currpos
                                                                                .wrapping_sub(datasrc[part as usize].startpos),
                                                                            datasrc[part as usize]
                                                                                .endpos
                                                                                .wrapping_sub(datasrc[part as usize].startpos),
                                                                            trycnt.wrapping_add(1 as uint32_t),
                                                                        );
                                                                    }
                                                                    status = EIO;
                                                                    break;
                                                                } else if pfd[desc as usize].revents
                                                                    as ::core::ffi::c_int
                                                                    & POLLERR
                                                                    != 0
                                                                {
                                                                    if trycnt >= minlogretry {
                                                                        univmakestrip(
                                                                            &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                            datasrc[part as usize].ip,
                                                                        );
                                                                        mfs_log(
                                                                            MFSLOG_SYSLOG,
                                                                            MFSLOG_WARNING,
                                                                            b"file: %u, index: %u, chunk: %016lX, version: %u - readworker: connection with (%s:%hu) got error status / POLLERR (received: %u/%u; try counter: %u)\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                            inode,
                                                                            chindx,
                                                                            chunkid,
                                                                            version,
                                                                            &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                            datasrc[part as usize].port as ::core::ffi::c_int,
                                                                            datasrc[part as usize]
                                                                                .currpos
                                                                                .wrapping_sub(datasrc[part as usize].startpos),
                                                                            datasrc[part as usize]
                                                                                .endpos
                                                                                .wrapping_sub(datasrc[part as usize].startpos),
                                                                            trycnt.wrapping_add(1 as uint32_t),
                                                                        );
                                                                    }
                                                                    status = EIO;
                                                                    break;
                                                                } else {
                                                                    if pfd[desc as usize].revents
                                                                        as ::core::ffi::c_int
                                                                        & POLLIN
                                                                        != 0
                                                                    {
                                                                        datasrc[part as usize]
                                                                            .lastrcvd =
                                                                            monotonic_seconds();
                                                                        if datasrc[part as usize]
                                                                            .received
                                                                            < 8 as uint32_t
                                                                        {
                                                                            i = read(
                                                                                datasrc[part as usize].fd,
                                                                                (&raw mut (*(&raw mut datasrc as *mut data_source)
                                                                                    .offset(part as isize))
                                                                                    .recvbuff as *mut uint8_t)
                                                                                    .offset(datasrc[part as usize].received as isize)
                                                                                    as *mut ::core::ffi::c_void,
                                                                                (8 as uint32_t)
                                                                                    .wrapping_sub(datasrc[part as usize].received) as size_t,
                                                                            ) as ::core::ffi::c_int;
                                                                            if i == 0 as ::core::ffi::c_int {
                                                                                if trycnt >= minlogretry {
                                                                                    univmakestrip(
                                                                                        &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                                        datasrc[part as usize].ip,
                                                                                    );
                                                                                    mfs_log(
                                                                                        MFSLOG_SYSLOG,
                                                                                        MFSLOG_WARNING,
                                                                                        b"file: %u, index: %u, chunk: %016lX, version: %u - readworker: connection with (%s:%hu) was reset by peer / ZEROREAD (received: %u/%u; try counter: %u)\0"
                                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                                        inode,
                                                                                        chindx,
                                                                                        chunkid,
                                                                                        version,
                                                                                        &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                                        datasrc[part as usize].port as ::core::ffi::c_int,
                                                                                        datasrc[part as usize]
                                                                                            .currpos
                                                                                            .wrapping_sub(datasrc[part as usize].startpos),
                                                                                        datasrc[part as usize]
                                                                                            .endpos
                                                                                            .wrapping_sub(datasrc[part as usize].startpos),
                                                                                        trycnt.wrapping_add(1 as uint32_t),
                                                                                    );
                                                                                }
                                                                                status = EIO;
                                                                                break;
                                                                            } else if i < 0 as ::core::ffi::c_int
                                                                                && (*__errno_location() != EAGAIN
                                                                                    && *__errno_location() != EWOULDBLOCK)
                                                                            {
                                                                                if trycnt >= minlogretry {
                                                                                    let mut err_2: ::core::ffi::c_int = *__errno_location();
                                                                                    univmakestrip(
                                                                                        &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                                        datasrc[part as usize].ip,
                                                                                    );
                                                                                    mfs_log(
                                                                                        MFSLOG_SYSLOG,
                                                                                        MFSLOG_WARNING,
                                                                                        b"file: %u, index: %u, chunk: %016lX, version: %u, part:%hhu - readworker: read from (%s:%hu) error: %s (received: %u/%u; try counter: %u)\0"
                                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                                        inode,
                                                                                        chindx,
                                                                                        chunkid,
                                                                                        version,
                                                                                        part as ::core::ffi::c_int,
                                                                                        &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                                        datasrc[part as usize].port as ::core::ffi::c_int,
                                                                                        strerr(err_2),
                                                                                        datasrc[part as usize]
                                                                                            .currpos
                                                                                            .wrapping_sub(datasrc[part as usize].startpos),
                                                                                        datasrc[part as usize]
                                                                                            .endpos
                                                                                            .wrapping_sub(datasrc[part as usize].startpos),
                                                                                        trycnt.wrapping_add(1 as uint32_t),
                                                                                    );
                                                                                }
                                                                                status = EIO;
                                                                                break;
                                                                            } else {
                                                                                if i < 0 as ::core::ffi::c_int {
                                                                                    i = 0 as ::core::ffi::c_int;
                                                                                }
                                                                                datasrc[part as usize].received = datasrc[part as usize]
                                                                                    .received
                                                                                    .wrapping_add(i as uint32_t);
                                                                                if datasrc[part as usize].received == 8 as uint32_t {
                                                                                    rptr = &raw mut (*(&raw mut datasrc as *mut data_source)
                                                                                        .offset(part as isize))
                                                                                        .recvbuff as *mut uint8_t;
                                                                                    datasrc[part as usize].reccmd = get32bit(&raw mut rptr);
                                                                                    datasrc[part as usize].recleng = get32bit(&raw mut rptr);
                                                                                    if datasrc[part as usize].reccmd
                                                                                        == CSTOCL_READ_STATUS as uint32_t
                                                                                    {
                                                                                        if datasrc[part as usize].recleng != 9 as uint32_t {
                                                                                            mfs_log(
                                                                                                MFSLOG_SYSLOG,
                                                                                                MFSLOG_WARNING,
                                                                                                b"readworker: got wrong sized status packet from chunkserver (leng:%u)\0"
                                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                                datasrc[part as usize].recleng,
                                                                                            );
                                                                                            status = EIO;
                                                                                            resetpos = 1 as uint8_t;
                                                                                            break;
                                                                                        }
                                                                                    } else if datasrc[part as usize].reccmd
                                                                                        == CSTOCL_READ_DATA as uint32_t
                                                                                    {
                                                                                        if datasrc[part as usize].recleng < 20 as uint32_t {
                                                                                            mfs_log(
                                                                                                MFSLOG_SYSLOG,
                                                                                                MFSLOG_WARNING,
                                                                                                b"readworker: got too short data packet from chunkserver (leng:%u)\0"
                                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                                datasrc[part as usize].recleng,
                                                                                            );
                                                                                            status = EIO;
                                                                                            resetpos = 1 as uint8_t;
                                                                                            break;
                                                                                        } else if datasrc[part as usize].recleng
                                                                                            > (20 as ::core::ffi::c_int + MFSCHUNKSIZE) as uint32_t
                                                                                        {
                                                                                            mfs_log(
                                                                                                MFSLOG_SYSLOG,
                                                                                                MFSLOG_WARNING,
                                                                                                b"readworker: got too long data packet from chunkserver (leng:%u)\0"
                                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                                datasrc[part as usize].recleng,
                                                                                            );
                                                                                            status = EIO;
                                                                                            resetpos = 1 as uint8_t;
                                                                                            break;
                                                                                        } else if datasrc[part as usize]
                                                                                            .recleng
                                                                                            .wrapping_sub(20 as uint32_t)
                                                                                            .wrapping_add(datasrc[part as usize].currpos)
                                                                                            > datasrc[part as usize].endpos
                                                                                        {
                                                                                            mfs_log(
                                                                                                MFSLOG_SYSLOG,
                                                                                                MFSLOG_WARNING,
                                                                                                b"readworker: got too long data packet from chunkserver (leng:%u)\0"
                                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                                datasrc[part as usize].recleng,
                                                                                            );
                                                                                            status = EIO;
                                                                                            resetpos = 1 as uint8_t;
                                                                                            break;
                                                                                        } else if datasrc[part as usize].currpos
                                                                                            & !(0x3ffff as ::core::ffi::c_int) as uint32_t
                                                                                            != datasrc[part as usize]
                                                                                                .recleng
                                                                                                .wrapping_sub(20 as uint32_t)
                                                                                                .wrapping_add(datasrc[part as usize].currpos)
                                                                                                .wrapping_sub(1 as uint32_t)
                                                                                                & !(0x3ffff as ::core::ffi::c_int) as uint32_t
                                                                                        {
                                                                                            mfs_log(
                                                                                                MFSLOG_SYSLOG,
                                                                                                MFSLOG_WARNING,
                                                                                                b"readworker: got data packet from chunkserver (leng:%u) which overlaps two logical blocks\0"
                                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                                datasrc[part as usize].recleng,
                                                                                            );
                                                                                            status = EIO;
                                                                                            resetpos = 1 as uint8_t;
                                                                                            break;
                                                                                        }
                                                                                    } else if datasrc[part as usize].reccmd
                                                                                        == ANTOAN_NOP as uint32_t
                                                                                    {
                                                                                        if datasrc[part as usize].recleng != 0 as uint32_t {
                                                                                            mfs_log(
                                                                                                MFSLOG_SYSLOG,
                                                                                                MFSLOG_WARNING,
                                                                                                b"readworker: got wrong sized nop packet from chunkserver (leng:%u)\0"
                                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                                datasrc[part as usize].recleng,
                                                                                            );
                                                                                            status = EIO;
                                                                                            resetpos = 1 as uint8_t;
                                                                                            break;
                                                                                        } else {
                                                                                            datasrc[part as usize].received = 0 as uint32_t;
                                                                                        }
                                                                                    } else {
                                                                                        let mut myip: uint32_t = 0;
                                                                                        let mut peerip: uint32_t = 0;
                                                                                        let mut myport: uint16_t = 0;
                                                                                        let mut peerport: uint16_t = 0;
                                                                                        tcpgetpeer(
                                                                                            datasrc[part as usize].fd,
                                                                                            &raw mut peerip,
                                                                                            &raw mut peerport,
                                                                                        );
                                                                                        tcpgetmyaddr(
                                                                                            datasrc[part as usize].fd,
                                                                                            &raw mut myip,
                                                                                            &raw mut myport,
                                                                                        );
                                                                                        mfs_log(
                                                                                            MFSLOG_SYSLOG,
                                                                                            MFSLOG_WARNING,
                                                                                            b"readworker: got unrecognized packet from chunkserver (cmd:%u,leng:%u,%u.%u.%u.%u:%u<->%u.%u.%u.%u:%u)\0"
                                                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                                                            datasrc[part as usize].reccmd,
                                                                                            datasrc[part as usize].recleng,
                                                                                            myip >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                                            myip >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                                            myip >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                                            myip & 0xff as uint32_t,
                                                                                            myport as ::core::ffi::c_int,
                                                                                            peerip >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                                            peerip >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                                            peerip >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                                                                            peerip & 0xff as uint32_t,
                                                                                            peerport as ::core::ffi::c_int,
                                                                                        );
                                                                                        status = EIO;
                                                                                        resetpos = 1 as uint8_t;
                                                                                        break;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        if datasrc[part as usize]
                                                                            .received
                                                                            >= 8 as uint32_t
                                                                        {
                                                                            if datasrc
                                                                                [part as usize]
                                                                                .recleng
                                                                                <= 20 as uint32_t
                                                                            {
                                                                                i = read(
                                                                                    datasrc[part as usize].fd,
                                                                                    (&raw mut (*(&raw mut datasrc as *mut data_source)
                                                                                        .offset(part as isize))
                                                                                        .recvbuff as *mut uint8_t)
                                                                                        .offset(
                                                                                            datasrc[part as usize].received.wrapping_sub(8 as uint32_t)
                                                                                                as isize,
                                                                                        ) as *mut ::core::ffi::c_void,
                                                                                    datasrc[part as usize]
                                                                                        .recleng
                                                                                        .wrapping_sub(
                                                                                            datasrc[part as usize].received.wrapping_sub(8 as uint32_t),
                                                                                        ) as size_t,
                                                                                ) as ::core::ffi::c_int;
                                                                            } else {
                                                                                datacurrpos =
                                                                                    datasrc[part
                                                                                        as usize]
                                                                                        .currpos;
                                                                                if parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                                                                                    || parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                                                                                {
                                                                                    datacurrpos &= !(0x3ffff as ::core::ffi::c_int) as uint32_t;
                                                                                    datacurrpos = datacurrpos.wrapping_mul(parts as uint32_t);
                                                                                    datacurrpos = datacurrpos
                                                                                        .wrapping_add(
                                                                                            (part as uint32_t) << 18 as ::core::ffi::c_int,
                                                                                        );
                                                                                    datacurrpos = datacurrpos
                                                                                        .wrapping_add(
                                                                                            datasrc[part as usize].currpos & 0x3ffff as uint32_t,
                                                                                        );
                                                                                }
                                                                                datacurrpos = (datacurrpos as uint64_t)
                                                                                    .wrapping_sub((*rreq).offset & MFSCHUNKMASK as uint64_t)
                                                                                    as uint32_t;
                                                                                if datasrc[part as usize].received
                                                                                    < (8 as ::core::ffi::c_int + 20 as ::core::ffi::c_int)
                                                                                        as uint32_t
                                                                                {
                                                                                    siov[0 as usize].iov_base = (&raw mut (*(&raw mut datasrc
                                                                                        as *mut data_source)
                                                                                        .offset(part as isize))
                                                                                        .recvbuff as *mut uint8_t)
                                                                                        .offset(
                                                                                            datasrc[part as usize].received.wrapping_sub(8 as uint32_t)
                                                                                                as isize,
                                                                                        ) as *mut ::core::ffi::c_void;
                                                                                    siov[0 as usize].iov_len = (20 as uint32_t)
                                                                                        .wrapping_sub(
                                                                                            datasrc[part as usize].received.wrapping_sub(8 as uint32_t),
                                                                                        ) as size_t;
                                                                                    siov[1 as usize].iov_base = (*rreq)
                                                                                        .data
                                                                                        .offset(datacurrpos as isize) as *mut ::core::ffi::c_void;
                                                                                    siov[1 as usize].iov_len = datasrc[part as usize]
                                                                                        .recleng
                                                                                        .wrapping_sub(20 as uint32_t) as size_t;
                                                                                    i = readv(
                                                                                        datasrc[part as usize].fd,
                                                                                        &raw mut siov as *mut iovec,
                                                                                        2 as ::core::ffi::c_int,
                                                                                    ) as ::core::ffi::c_int;
                                                                                } else {
                                                                                    i = read(
                                                                                        datasrc[part as usize].fd,
                                                                                        (*rreq).data.offset(datacurrpos as isize)
                                                                                            as *mut ::core::ffi::c_void,
                                                                                        datasrc[part as usize]
                                                                                            .recleng
                                                                                            .wrapping_sub(
                                                                                                datasrc[part as usize].received.wrapping_sub(8 as uint32_t),
                                                                                            ) as size_t,
                                                                                    ) as ::core::ffi::c_int;
                                                                                }
                                                                            }
                                                                            if i == 0 as ::core::ffi::c_int {
                                                                                if trycnt >= minlogretry {
                                                                                    univmakestrip(
                                                                                        &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                                        datasrc[part as usize].ip,
                                                                                    );
                                                                                    mfs_log(
                                                                                        MFSLOG_SYSLOG,
                                                                                        MFSLOG_WARNING,
                                                                                        b"file: %u, index: %u, chunk: %016lX, version: %u, part: %hhu - readworker: connection with (%s:%hu) was reset by peer (received: %u/%u; try counter: %u)\0"
                                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                                        inode,
                                                                                        chindx,
                                                                                        chunkid,
                                                                                        version,
                                                                                        part as ::core::ffi::c_int,
                                                                                        &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                                        datasrc[part as usize].port as ::core::ffi::c_int,
                                                                                        datasrc[part as usize]
                                                                                            .currpos
                                                                                            .wrapping_sub(datasrc[part as usize].startpos),
                                                                                        datasrc[part as usize]
                                                                                            .endpos
                                                                                            .wrapping_sub(datasrc[part as usize].startpos),
                                                                                        trycnt.wrapping_add(1 as uint32_t),
                                                                                    );
                                                                                }
                                                                                status = EIO;
                                                                                break;
                                                                            } else if i < 0 as ::core::ffi::c_int
                                                                                && (*__errno_location() != EAGAIN
                                                                                    && *__errno_location() != EWOULDBLOCK)
                                                                            {
                                                                                if trycnt >= minlogretry {
                                                                                    let mut err_3: ::core::ffi::c_int = *__errno_location();
                                                                                    univmakestrip(
                                                                                        &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                                        datasrc[part as usize].ip,
                                                                                    );
                                                                                    mfs_log(
                                                                                        MFSLOG_SYSLOG,
                                                                                        MFSLOG_WARNING,
                                                                                        b"file: %u, index: %u, chunk: %016lX, version: %u, part: %hhu - readworker: connection with (%s:%hu) error: %s (received: %u/%u; try counter: %u)\0"
                                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                                        inode,
                                                                                        chindx,
                                                                                        chunkid,
                                                                                        version,
                                                                                        part as ::core::ffi::c_int,
                                                                                        &raw mut csstrip as *mut ::core::ffi::c_char,
                                                                                        datasrc[part as usize].port as ::core::ffi::c_int,
                                                                                        strerr(err_3),
                                                                                        datasrc[part as usize]
                                                                                            .currpos
                                                                                            .wrapping_sub(datasrc[part as usize].startpos),
                                                                                        datasrc[part as usize]
                                                                                            .endpos
                                                                                            .wrapping_sub(datasrc[part as usize].startpos),
                                                                                        trycnt.wrapping_add(1 as uint32_t),
                                                                                    );
                                                                                }
                                                                                status = EIO;
                                                                                break;
                                                                            } else {
                                                                                if i < 0 as ::core::ffi::c_int {
                                                                                    i = 0 as ::core::ffi::c_int;
                                                                                }
                                                                                if datasrc[part as usize].received
                                                                                    < (8 as ::core::ffi::c_int + 20 as ::core::ffi::c_int)
                                                                                        as uint32_t
                                                                                {
                                                                                    if datasrc[part as usize]
                                                                                        .received
                                                                                        .wrapping_add(i as uint32_t)
                                                                                        >= (8 as ::core::ffi::c_int + 20 as ::core::ffi::c_int)
                                                                                            as uint32_t
                                                                                    {
                                                                                        datasrc[part as usize].currpos = datasrc[part as usize]
                                                                                            .currpos
                                                                                            .wrapping_add(
                                                                                                (i as uint32_t)
                                                                                                    .wrapping_sub(
                                                                                                        ((8 as ::core::ffi::c_int + 20 as ::core::ffi::c_int)
                                                                                                            as uint32_t)
                                                                                                            .wrapping_sub(datasrc[part as usize].received),
                                                                                                    ),
                                                                                            );
                                                                                    }
                                                                                } else {
                                                                                    datasrc[part as usize].currpos = datasrc[part as usize]
                                                                                        .currpos
                                                                                        .wrapping_add(i as uint32_t);
                                                                                }
                                                                                datasrc[part as usize].received = datasrc[part as usize]
                                                                                    .received
                                                                                    .wrapping_add(i as uint32_t);
                                                                                if datasrc[part as usize].received
                                                                                    > (8 as uint32_t)
                                                                                        .wrapping_add(datasrc[part as usize].recleng)
                                                                                {
                                                                                    mfs_log(
                                                                                        MFSLOG_SYSLOG,
                                                                                        MFSLOG_WARNING,
                                                                                        b"readworker: internal error - received more bytes than expected\0"
                                                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                                                    );
                                                                                    status = EIO;
                                                                                    resetpos = 1 as uint8_t;
                                                                                    break;
                                                                                } else if datasrc[part as usize].received
                                                                                    == (8 as uint32_t)
                                                                                        .wrapping_add(datasrc[part as usize].recleng)
                                                                                {
                                                                                    if datasrc[part as usize].reccmd
                                                                                        == CSTOCL_READ_STATUS as uint32_t
                                                                                    {
                                                                                        let mut expectedchunkid: uint64_t = 0;
                                                                                        rptr = &raw mut (*(&raw mut datasrc as *mut data_source)
                                                                                            .offset(part as isize))
                                                                                            .recvbuff as *mut uint8_t;
                                                                                        recchunkid = get64bit(&raw mut rptr);
                                                                                        recstatus = get8bit(&raw mut rptr);
                                                                                        if parts as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                                                                                            expectedchunkid = chunkid;
                                                                                        } else if parts as ::core::ffi::c_int
                                                                                            == 8 as ::core::ffi::c_int
                                                                                        {
                                                                                            expectedchunkid = chunkid & 0xffffffffffffff as uint64_t
                                                                                                | ((0x20 as ::core::ffi::c_int | part as ::core::ffi::c_int)
                                                                                                    as uint64_t) << 56 as ::core::ffi::c_int;
                                                                                        } else {
                                                                                            expectedchunkid = chunkid & 0xffffffffffffff as uint64_t
                                                                                                | ((0x10 as ::core::ffi::c_int | part as ::core::ffi::c_int)
                                                                                                    as uint64_t) << 56 as ::core::ffi::c_int;
                                                                                        }
                                                                                        if recchunkid != expectedchunkid {
                                                                                            mfs_log(
                                                                                                MFSLOG_SYSLOG,
                                                                                                MFSLOG_WARNING,
                                                                                                b"readworker: got unexpected status packet (expected chunkdid:%lX,packet chunkid:%lX)\0"
                                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                                expectedchunkid,
                                                                                                recchunkid,
                                                                                            );
                                                                                            status = EIO;
                                                                                            resetpos = 1 as uint8_t;
                                                                                            break;
                                                                                        } else if recstatus as ::core::ffi::c_int != MFS_STATUS_OK {
                                                                                            if trycnt >= minlogretry {
                                                                                                mfs_log(
                                                                                                    MFSLOG_SYSLOG,
                                                                                                    MFSLOG_WARNING,
                                                                                                    b"readworker: read error: %s\0".as_ptr()
                                                                                                        as *const ::core::ffi::c_char,
                                                                                                    mfsstrerr(recstatus),
                                                                                                );
                                                                                            }
                                                                                            status = EIO;
                                                                                            if recstatus as ::core::ffi::c_int == MFS_ERROR_NOTDONE {
                                                                                                notdone = 1 as uint8_t;
                                                                                            } else {
                                                                                                resetpos = 1 as uint8_t;
                                                                                            }
                                                                                            break;
                                                                                        } else if datasrc[part as usize].currpos
                                                                                            != datasrc[part as usize].endpos
                                                                                        {
                                                                                            mfs_log(
                                                                                                MFSLOG_SYSLOG,
                                                                                                MFSLOG_WARNING,
                                                                                                b"readworker: unexpected data block size (requested: %u / received: %u)\0"
                                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                                datasrc[part as usize]
                                                                                                    .endpos
                                                                                                    .wrapping_sub(datasrc[part as usize].startpos),
                                                                                                datasrc[part as usize]
                                                                                                    .currpos
                                                                                                    .wrapping_sub(datasrc[part as usize].startpos),
                                                                                            );
                                                                                            status = EIO;
                                                                                            resetpos = 1 as uint8_t;
                                                                                            break;
                                                                                        } else {
                                                                                            datasrc[part as usize].gotstatus = 1 as uint8_t;
                                                                                        }
                                                                                    } else if datasrc[part as usize].reccmd
                                                                                        == CSTOCL_READ_DATA as uint32_t
                                                                                    {
                                                                                        let mut expectedchunkid_0: uint64_t = 0;
                                                                                        rptr = &raw mut (*(&raw mut datasrc as *mut data_source)
                                                                                            .offset(part as isize))
                                                                                            .recvbuff as *mut uint8_t;
                                                                                        recchunkid = get64bit(&raw mut rptr);
                                                                                        recblocknum = get16bit(&raw mut rptr);
                                                                                        recoffset = get16bit(&raw mut rptr);
                                                                                        recsize = get32bit(&raw mut rptr);
                                                                                        reccrc = get32bit(&raw mut rptr);
                                                                                        datacurrpos = datasrc[part as usize]
                                                                                            .currpos
                                                                                            .wrapping_sub(recsize);
                                                                                        if parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                                                                                            || parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                                                                                        {
                                                                                            datacurrpos &= !(0x3ffff as ::core::ffi::c_int) as uint32_t;
                                                                                            datacurrpos = datacurrpos.wrapping_mul(parts as uint32_t);
                                                                                            datacurrpos = datacurrpos
                                                                                                .wrapping_add(
                                                                                                    (part as uint32_t) << 18 as ::core::ffi::c_int,
                                                                                                );
                                                                                            datacurrpos = datacurrpos
                                                                                                .wrapping_add(
                                                                                                    datasrc[part as usize].currpos.wrapping_sub(recsize)
                                                                                                        & 0x3ffff as uint32_t,
                                                                                                );
                                                                                            if parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                                                                                                expectedchunkid_0 = chunkid & 0xffffffffffffff as uint64_t
                                                                                                    | ((0x20 as ::core::ffi::c_int | part as ::core::ffi::c_int)
                                                                                                        as uint64_t) << 56 as ::core::ffi::c_int;
                                                                                            } else {
                                                                                                expectedchunkid_0 = chunkid & 0xffffffffffffff as uint64_t
                                                                                                    | ((0x10 as ::core::ffi::c_int | part as ::core::ffi::c_int)
                                                                                                        as uint64_t) << 56 as ::core::ffi::c_int;
                                                                                            }
                                                                                        } else {
                                                                                            expectedchunkid_0 = chunkid;
                                                                                        }
                                                                                        datacurrpos = (datacurrpos as uint64_t)
                                                                                            .wrapping_sub((*rreq).offset & MFSCHUNKMASK as uint64_t)
                                                                                            as uint32_t;
                                                                                        if recchunkid != expectedchunkid_0 {
                                                                                            mfs_log(
                                                                                                MFSLOG_SYSLOG,
                                                                                                MFSLOG_WARNING,
                                                                                                b"readworker: got unexpected data packet (expected chunkdid:%016lX,packet chunkid:%016lX)\0"
                                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                                expectedchunkid_0,
                                                                                                recchunkid,
                                                                                            );
                                                                                            status = EIO;
                                                                                            resetpos = 1 as uint8_t;
                                                                                            break;
                                                                                        } else if recsize.wrapping_add(20 as uint32_t)
                                                                                            != datasrc[part as usize].recleng
                                                                                        {
                                                                                            mfs_log(
                                                                                                MFSLOG_SYSLOG,
                                                                                                MFSLOG_WARNING,
                                                                                                b"readworker: got malformed data packet (datasize: %u,packetsize: %u)\0"
                                                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                                                recsize,
                                                                                                datasrc[part as usize].recleng,
                                                                                            );
                                                                                            status = EIO;
                                                                                            resetpos = 1 as uint8_t;
                                                                                            break;
                                                                                        } else if reccrc
                                                                                            != mycrc32(
                                                                                                0 as uint32_t,
                                                                                                (*rreq).data.offset(datacurrpos as isize)
                                                                                                    as *const ::core::ffi::c_void,
                                                                                                recsize,
                                                                                            )
                                                                                        {
                                                                                            mfs_log(
                                                                                                MFSLOG_SYSLOG,
                                                                                                MFSLOG_WARNING,
                                                                                                b"readworker: data checksum error\0".as_ptr()
                                                                                                    as *const ::core::ffi::c_char,
                                                                                            );
                                                                                            status = EIO;
                                                                                            resetpos = 1 as uint8_t;
                                                                                            break;
                                                                                        } else {
                                                                                            read_increase_total_bytes(recsize);
                                                                                            readanything = 1 as uint8_t;
                                                                                        }
                                                                                    }
                                                                                    datasrc[part as usize].received = 0 as uint32_t;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    desc = desc.wrapping_add(1);
                                                                }
                                                            }
                                                            part = part.wrapping_add(1);
                                                        }
                                                        if status == EIO {
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                part = 0 as uint8_t;
                                while (part as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                                    if status == 0 as ::core::ffi::c_int
                                        && datasrc[part as usize].csver
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
                                        conncache_insert(
                                            datasrc[part as usize].ip,
                                            datasrc[part as usize].port,
                                            datasrc[part as usize].fd,
                                        );
                                    } else {
                                        tcpclose(datasrc[part as usize].fd);
                                    }
                                    part = part.wrapping_add(1);
                                }
                                if status == EINTR {
                                    status = 0 as ::core::ffi::c_int;
                                }
                                ind_lock(ind);
                                if readanything != 0 {
                                    if resetpos != 0 {
                                        readanything = 0 as uint8_t;
                                        (*rreq).currentpos =
                                            ((*rreq).offset & MFSCHUNKMASK as uint64_t) as uint32_t;
                                        memset(
                                            &raw mut (*rreq).splitcurrpos as *mut uint32_t
                                                as *mut ::core::ffi::c_void,
                                            0 as ::core::ffi::c_int,
                                            ::core::mem::size_of::<[uint32_t; 8]>(),
                                        );
                                    } else if parts as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                                    {
                                        (*rreq).currentpos = datasrc[0 as usize].currpos;
                                        memset(
                                            &raw mut (*rreq).splitcurrpos as *mut uint32_t
                                                as *mut ::core::ffi::c_void,
                                            0 as ::core::ffi::c_int,
                                            ::core::mem::size_of::<[uint32_t; 8]>(),
                                        );
                                    } else if parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                                        || parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                                    {
                                        let mut minblockpos: uint32_t = 0;
                                        minblockpos = 0xffffffff as ::core::ffi::c_uint as uint32_t;
                                        part = 0 as uint8_t;
                                        while (part as ::core::ffi::c_int)
                                            < parts as ::core::ffi::c_int
                                        {
                                            datacurrpos = datasrc[part as usize].currpos;
                                            datacurrpos &=
                                                !(0x3ffff as ::core::ffi::c_int) as uint32_t;
                                            if part as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                                || datacurrpos < minblockpos
                                            {
                                                minblockpos = datacurrpos;
                                            }
                                            part = part.wrapping_add(1);
                                        }
                                        part = 0 as uint8_t;
                                        while (part as ::core::ffi::c_int)
                                            < parts as ::core::ffi::c_int
                                        {
                                            datacurrpos = datasrc[part as usize].currpos;
                                            datacurrpos &=
                                                !(0x3ffff as ::core::ffi::c_int) as uint32_t;
                                            if datacurrpos
                                                < minblockpos.wrapping_add(0x40000 as uint32_t)
                                            {
                                                datacurrpos =
                                                    datacurrpos.wrapping_mul(parts as uint32_t);
                                                datacurrpos = datacurrpos.wrapping_add(
                                                    (part as uint32_t) << 18 as ::core::ffi::c_int,
                                                );
                                                datacurrpos = datacurrpos.wrapping_add(
                                                    datasrc[part as usize].currpos
                                                        & 0x3ffff as uint32_t,
                                                );
                                                break;
                                            } else {
                                                part = part.wrapping_add(1);
                                            }
                                        }
                                        if (part as ::core::ffi::c_int)
                                            < parts as ::core::ffi::c_int
                                        {
                                        } else {
                                            fprintf(
                                                stderr,
                                                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1970 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"part<parts\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"data mismatch\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1970 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"part<parts\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"data mismatch\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                            abort();
                                        };
                                        if datacurrpos as uint64_t
                                            >= (*rreq).offset & 0x3ffffff as uint64_t
                                            && datacurrpos as uint64_t
                                                <= ((*rreq).offset & 0x3ffffff as uint64_t)
                                                    .wrapping_add((*rreq).leng as uint64_t)
                                        {
                                        } else {
                                            fprintf(
                                                stderr,
                                                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1971 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"(datacurrpos >= (rreq->offset & MFSCHUNKMASK)) && (datacurrpos <= (rreq->offset & MFSCHUNKMASK) + rreq->leng)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"current position mismatch\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                1971 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"(datacurrpos >= (rreq->offset & MFSCHUNKMASK)) && (datacurrpos <= (rreq->offset & MFSCHUNKMASK) + rreq->leng)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"current position mismatch\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                            abort();
                                        };
                                        (*rreq).currentpos = datacurrpos;
                                        part = 0 as uint8_t;
                                        while (part as ::core::ffi::c_int)
                                            < parts as ::core::ffi::c_int
                                        {
                                            (*rreq).splitcurrpos[part as usize] =
                                                datasrc[part as usize].currpos;
                                            part = part.wrapping_add(1);
                                        }
                                    }
                                }
                                if status != 0 as ::core::ffi::c_int {
                                    if readanything as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                    {
                                        if notdone as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                        {
                                            (*rreq).trycnt = (*rreq).trycnt.wrapping_add(1);
                                            trycnt = (*rreq).trycnt;
                                        }
                                    }
                                    if trycnt >= maxretries {
                                        ind_unlock(ind);
                                        crate::chunksdatacache::invalidate(inode, chindx);
                                        read_job_end(rreq, status, 0 as uint32_t);
                                    } else {
                                        ind_unlock(ind);
                                        crate::chunksdatacache::invalidate(inode, chindx);
                                        if notdone != 0 {
                                            read_job_end(
                                                rreq,
                                                0 as ::core::ffi::c_int,
                                                300000 as uint32_t,
                                            );
                                        } else {
                                            read_job_end(
                                                rreq,
                                                0 as ::core::ffi::c_int,
                                                if trycnt < 3 as uint32_t {
                                                    0 as uint32_t
                                                } else {
                                                    (1000 as uint32_t).wrapping_add(
                                                        if trycnt < 30 as uint32_t {
                                                            trycnt
                                                                .wrapping_sub(3 as uint32_t)
                                                                .wrapping_mul(300000 as uint32_t)
                                                        } else {
                                                            10000000 as uint32_t
                                                        },
                                                    )
                                                },
                                            );
                                        }
                                    }
                                } else {
                                    ind_unlock(ind);
                                    read_job_end(rreq, 0 as ::core::ffi::c_int, 0 as uint32_t);
                                }
                                crate::chunkrwlock::read_unlock(inode, chindx);
                            }
                        }
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_data_init(
    mut readaheadsize: uint64_t,
    mut readaheadleng: uint32_t,
    mut readaheadtrigger: uint32_t,
    mut retries: uint32_t,
    mut timeout: uint32_t,
    mut logretry: uint32_t,
    mut erronlostchunk: uint8_t,
    mut erronnospace: uint8_t,
) {
    unsafe {
        let mut mystacksize: size_t = 0;
        maxretries = retries;
        usectimeout = timeout as uint64_t;
        usectimeout = usectimeout.wrapping_mul(1000000 as uint64_t);
        minlogretry = logretry;
        readahead_leng = readaheadleng;
        readahead_trigger = readaheadtrigger;
        maxreadaheadsize = readaheadsize;
        erroronlostchunk = erronlostchunk;
        erroronnospace = erronnospace;
        reqbufftotalsize = 0 as uint64_t;
        // C: pthread_key_create(&rangesstorage, read_data_ranges_free) +
        // pthread_setspecific(NULL) — replaced by the RANGES thread_local
        // (empty until first use; Vec Drop frees at thread exit).
        JQUEUE.init();
        mystacksize = __sysconf(__SC_THREAD_STACK_MIN_VALUE) as size_t;
        if mystacksize < 0x20000 as ::core::ffi::c_int as size_t {
            mystacksize = 0x20000 as ::core::ffi::c_int as size_t;
        }
        let mut st = READ_WORKER_POOL.lock_state();
        st.avail = 0 as uint32_t;
        st.total = 0 as uint32_t;
        st.stack_size = mystacksize as usize;
        READ_WORKER_POOL.spawn_worker(&mut st);
        drop(st);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_data_term() {
    unsafe {
        let mut i: uint32_t = 0;
        JQUEUE.close();
        let mut st = READ_WORKER_POOL.lock_state();
        while st.total > 0 as uint32_t {
            st = READ_WORKER_POOL
                .term_cond
                .wait(st)
                .unwrap_or_else(|e| e.into_inner());
        }
        let handles = ::core::mem::take(&mut st.handles);
        drop(st);
        for handle in handles {
            let _ = handle.join();
        }
        JQUEUE.delete();
        inode_global_lock();
        i = 0 as uint32_t;
        while i < IDHASHSIZE as uint32_t {
            // Drain clears the bucket so a later read_data_init cannot
            // dangle (C freed and re-malloc'd the table, re-zeroing all
            // heads).
            for ind in ::core::mem::take(&mut (*&raw mut indhash)[i as usize]) {
                ind_lock(ind);
                ind_unlock(ind);
                drop(Box::from_raw(ind));
            }
            i = i.wrapping_add(1);
        }
        inode_global_unlock();
        // C: pthread_key_delete(rangesstorage) — nothing to do; RANGES
        // thread_locals are freed when their threads exit.
    }
}
#[inline]
unsafe extern "C" fn read_rreq_invalidate(mut rreq: *mut rrequest) -> *mut rrequest {
    unsafe {
        if !((*rreq).mode as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
            || (*rreq).mode as ::core::ffi::c_int == INQUEUE as ::core::ffi::c_int
            || (*rreq).mode as ::core::ffi::c_int == REFRESH as ::core::ffi::c_int
            || (*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int
            || (*rreq).mode as ::core::ffi::c_int == FILLED as ::core::ffi::c_int)
        {
            if (*rreq).lcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                read_delete_request(rreq);
                return ::core::ptr::null_mut::<rrequest>();
            } else if (*rreq).mode as ::core::ffi::c_int == READY as ::core::ffi::c_int {
                (*rreq).mode = NOTNEEDED as ::core::ffi::c_int as uint8_t;
            }
        } else {
            if (*rreq).lcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*rreq).mode = BREAK as ::core::ffi::c_int as uint8_t;
            } else if (*rreq).mode as ::core::ffi::c_int != INQUEUE as ::core::ffi::c_int {
                (*rreq).mode = REFRESH as ::core::ffi::c_int as uint8_t;
            } else {
                return rreq;
            }
            if (*rreq).waitingworker != 0 {
                if write(
                    (*rreq).wakeup_fd,
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
                (*rreq).waitingworker = 0 as uint8_t;
                (*rreq).wakeup_fd = -1 as ::core::ffi::c_int;
            }
        }
        return rreq;
    }
}
#[inline]
unsafe extern "C" fn read_rreq_not_needed(mut rreq: *mut rrequest) {
    unsafe {
        if !((*rreq).mode as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
            || (*rreq).mode as ::core::ffi::c_int == INQUEUE as ::core::ffi::c_int
            || (*rreq).mode as ::core::ffi::c_int == REFRESH as ::core::ffi::c_int
            || (*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int
            || (*rreq).mode as ::core::ffi::c_int == FILLED as ::core::ffi::c_int)
        {
            if (*rreq).lcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                read_delete_request(rreq);
            } else if (*rreq).mode as ::core::ffi::c_int == READY as ::core::ffi::c_int {
                (*rreq).mode = NOTNEEDED as ::core::ffi::c_int as uint8_t;
            }
        } else if (*rreq).lcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*rreq).mode = BREAK as ::core::ffi::c_int as uint8_t;
            if (*rreq).waitingworker != 0 {
                if write(
                    (*rreq).wakeup_fd,
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
                (*rreq).waitingworker = 0 as uint8_t;
                (*rreq).wakeup_fd = -1 as ::core::ffi::c_int;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_data_offset_cmp(
    mut aa: *const ::core::ffi::c_void,
    mut bb: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut a: uint64_t = *(aa as *const uint64_t);
        let mut b: uint64_t = *(bb as *const uint64_t);
        return if a < b {
            -1 as ::core::ffi::c_int
        } else if a > b {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[inline]
unsafe extern "C" fn read_inode_free(mut indh: uint32_t, mut indf: *mut inodedata) {
    unsafe {
        // C: pointer-to-pointer bucket walk unlinking indf. Pointer-identity
        // scan + remove is the same unlink; the node is then destroyed with
        // the same lock/unlock + Box::from_raw protocol as C.
        let bucket = &mut (*&raw mut indhash)[indh as usize];
        if let Some(pos) = bucket.iter().position(|&p| p == indf) {
            let ind = bucket.remove(pos);
            ind_lock(ind);
            ind_unlock(ind);
            drop(Box::from_raw(ind));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_data(
    mut vid: *mut ::core::ffi::c_void,
    mut offset: uint64_t,
    mut size: *mut uint32_t,
    mut vrhead: *mut *mut ::core::ffi::c_void,
    mut iov: *mut *mut iovec,
    mut iovcnt: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ind: *mut inodedata = vid as *mut inodedata;
        let mut rreq: *mut rrequest = ::core::ptr::null_mut::<rrequest>();
        let mut rreqn: *mut rrequest = ::core::ptr::null_mut::<rrequest>();
        // C: rlist *rhead, **rtail — built tail-append; crosses the vrhead
        // void* ABI as a boxed Vec (see rlist_s).
        let mut rlist_vec: Vec<rlist> = Vec::new();
        let mut rls: *mut Vec<rlist> = ::core::ptr::null_mut::<Vec<rlist>>();
        let mut rbuffsize: uint64_t = 0;
        let mut blockstart: uint64_t = 0;
        let mut blockend: uint64_t = 0;
        let mut firstbyte: uint64_t = 0;
        let mut lastbyte: uint64_t = 0;
        let mut cnt: uint32_t = 0;
        let mut edges: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut reqno: uint32_t = 0;
        let mut added: uint8_t = 0;
        let mut raok: uint8_t = 0;
        let mut etab: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut ranges: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut status: ::core::ffi::c_int = 0;
        let mut now: ::core::ffi::c_double = 0.;
        inode_global_lock();
        (*ind).lcnt = (*ind).lcnt.wrapping_add(1);
        inode_global_unlock();
        ind_lock(ind);
        while (*ind).waiting_writers as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            ind_cond_wait(&raw const (*ind).readerscond, ind);
        }
        (*ind).readers_cnt = (*ind).readers_cnt.wrapping_add(1);
        *vrhead = NULL;
        *iov = ::core::ptr::null_mut::<iovec>();
        *iovcnt = 0 as uint32_t;
        cnt = 0 as uint32_t;
        let c2rust_lhs = &raw mut reqbufftotalsize;
        let c2rust_rhs = 0 as uint64_t;
        rbuffsize =
            ::core::intrinsics::atomic_or::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                c2rust_lhs, c2rust_rhs,
            ) | c2rust_rhs;
        if (*ind).status == 0 as ::core::ffi::c_int
            && (*ind).closing as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            if offset == (*ind).lastoffset {
                if offset == 0 as uint64_t {
                    (*ind).readahead = 1 as uint8_t;
                    (*ind).seqdata = 0 as uint32_t;
                } else if ((*ind).readahead as ::core::ffi::c_int) < READAHEAD_MAX {
                    if (*ind).seqdata >= readahead_trigger {
                        (*ind).readahead = (*ind).readahead.wrapping_add(1);
                        (*ind).seqdata = 0 as uint32_t;
                    }
                }
            } else if offset.wrapping_add(readahead_leng.wrapping_div(2 as uint32_t) as uint64_t)
                < (*ind).lastoffset
                || (*ind)
                    .lastoffset
                    .wrapping_add(readahead_leng.wrapping_div(2 as uint32_t) as uint64_t)
                    < offset
            {
                if (*ind).readahead as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    (*ind).readahead = (*ind).readahead.wrapping_sub(1);
                }
                (*ind).seqdata = 0 as uint32_t;
            }
            if (*ind).readahead as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                && rbuffsize
                    >= maxreadaheadsize.wrapping_div(2 as uint64_t).wrapping_add(
                        maxreadaheadsize.wrapping_mul(1 as uint64_t).wrapping_div(
                            ((*ind).readahead as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as uint64_t,
                        ),
                    )
            {
                (*ind).readahead = (*ind).readahead.wrapping_sub(1);
                (*ind).seqdata = 0 as uint32_t;
            }
            firstbyte = offset;
            lastbyte = offset.wrapping_add(*size as uint64_t);
            if master_version()
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        74 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        74 as ::core::ffi::c_int
                    })) as uint32_t
            {
                if firstbyte > (*ind).fleng {
                    firstbyte = (*ind).fleng;
                }
                if lastbyte > (*ind).fleng {
                    lastbyte = (*ind).fleng;
                }
            }
            now = monotonic_seconds();
            reqno = 0 as uint32_t;
            let mut ri: usize = 0;
            while ri < (&(*ind).reqs).len() {
                rreq = (&(*ind).reqs)[ri];
                if !((*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int
                    || (*rreq).mode as ::core::ffi::c_int == NOTNEEDED as ::core::ffi::c_int)
                {
                    reqno = reqno.wrapping_add(1);
                }
                ri = ri.wrapping_add(1);
            }
            // C: rreq = ind->reqhead; while (rreq) { rreqn = rreq->next; ...
            // rreq = rreqn; } — read_rreq_not_needed may delete the current
            // node, in which case its successor shifted into slot ri.
            ri = 0;
            while ri < (&(*ind).reqs).len() {
                rreq = (&(*ind).reqs)[ri];
                if (*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int
                    || (*rreq).mode as ::core::ffi::c_int == NOTNEEDED as ::core::ffi::c_int
                {
                    read_rreq_not_needed(rreq);
                } else if (*rreq).modified + BUFFER_VALIDITY_TIMEOUT < now {
                    read_rreq_not_needed(rreq);
                    reqno = reqno.wrapping_sub(1);
                } else if (lastbyte <= (*rreq).offset
                    || firstbyte >= (*rreq).offset.wrapping_add((*rreq).leng as uint64_t))
                    && reqno > 3 as uint32_t
                {
                    read_rreq_not_needed(rreq);
                    reqno = reqno.wrapping_sub(1);
                }
                if (&(*ind).reqs).get(ri).copied() == Some(rreq) {
                    ri = ri.wrapping_add(1);
                }
            }
            ranges = get_ranges();
            etab = ranges.offset(1 as ::core::ffi::c_int as isize);
            edges = 0 as uint32_t;
            let c2rust_fresh0 = edges;
            edges = edges.wrapping_add(1);
            *etab.offset(c2rust_fresh0 as isize) = firstbyte;
            let c2rust_fresh1 = edges;
            edges = edges.wrapping_add(1);
            *etab.offset(c2rust_fresh1 as isize) = lastbyte;
            ri = 0;
            while ri < (&(*ind).reqs).len() {
                rreq = (&(*ind).reqs)[ri];
                if !((*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int
                    || (*rreq).mode as ::core::ffi::c_int == NOTNEEDED as ::core::ffi::c_int)
                {
                    if (*rreq).offset > firstbyte && (*rreq).offset < lastbyte {
                        i = 0 as uint32_t;
                        while i < edges && *etab.offset(i as isize) != (*rreq).offset {
                            i = i.wrapping_add(1);
                        }
                        if i >= edges {
                            if i as uint64_t >= *ranges.offset(0 as isize) {
                                ranges = grow_ranges();
                                etab = ranges.offset(1 as ::core::ffi::c_int as isize);
                            }
                            let c2rust_fresh2 = edges;
                            edges = edges.wrapping_add(1);
                            *etab.offset(c2rust_fresh2 as isize) = (*rreq).offset;
                        }
                    }
                    if (*rreq).offset.wrapping_add((*rreq).leng as uint64_t) > firstbyte
                        && (*rreq).offset.wrapping_add((*rreq).leng as uint64_t) < lastbyte
                    {
                        i = 0 as uint32_t;
                        while i < edges
                            && *etab.offset(i as isize)
                                != (*rreq).offset.wrapping_add((*rreq).leng as uint64_t)
                        {
                            i = i.wrapping_add(1);
                        }
                        if i >= edges {
                            if i as uint64_t >= *ranges.offset(0 as isize) {
                                ranges = grow_ranges();
                                etab = ranges.offset(1 as ::core::ffi::c_int as isize);
                            }
                            let c2rust_fresh3 = edges;
                            edges = edges.wrapping_add(1);
                            *etab.offset(c2rust_fresh3 as isize) =
                                (*rreq).offset.wrapping_add((*rreq).leng as uint64_t);
                        }
                    }
                }
                ri = ri.wrapping_add(1);
            }
            if edges > 2 as uint32_t {
                qsort(
                    etab as *mut ::core::ffi::c_void,
                    edges as size_t,
                    ::core::mem::size_of::<uint64_t>(),
                    Some(
                        read_data_offset_cmp
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_void,
                                *const ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
            }
            if *etab.offset(0 as isize) == firstbyte {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2377 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"etab[0]==firstbyte\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2377 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"etab[0]==firstbyte\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            };
            if *etab.offset(edges.wrapping_sub(1 as uint32_t) as isize) == lastbyte {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2378 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"etab[edges-1]==lastbyte\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2378 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"etab[edges-1]==lastbyte\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            };
            i = 0 as uint32_t;
            while i < edges.wrapping_sub(1 as uint32_t) {
                added = 0 as uint8_t;
                ri = 0;
                while ri < (&(*ind).reqs).len()
                    && added as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    rreq = (&(*ind).reqs)[ri];
                    if !((*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int
                        || (*rreq).mode as ::core::ffi::c_int == NOTNEEDED as ::core::ffi::c_int)
                    {
                        if (*rreq).offset <= *etab.offset(i as isize)
                            && (*rreq).offset.wrapping_add((*rreq).leng as uint64_t)
                                >= *etab.offset(i.wrapping_add(1 as uint32_t) as isize)
                        {
                            // C: malloc rlist + *rtail link — tail append.
                            rlist_vec.push(rlist_s {
                                rreq,
                                offsetadd: (*etab.offset(i as isize)).wrapping_sub((*rreq).offset)
                                    as uint32_t,
                                reqleng: (*etab.offset(i.wrapping_add(1 as uint32_t) as isize))
                                    .wrapping_sub(*etab.offset(i as isize))
                                    as uint32_t,
                            });
                            (*rreq).lcnt = (*rreq).lcnt.wrapping_add(1);
                            added = 1 as uint8_t;
                            if (*ind).readahead as ::core::ffi::c_int != 0
                                && i == edges.wrapping_sub(2 as uint32_t)
                            {
                                // C: rreq->next == NULL — rreq is the tail.
                                if ri.wrapping_add(1) == (&(*ind).reqs).len()
                                    && rbuffsize < maxreadaheadsize
                                {
                                    blockstart =
                                        (*rreq).offset.wrapping_add((*rreq).leng as uint64_t);
                                    blockend = blockstart.wrapping_add(
                                        readahead_leng.wrapping_mul(
                                            ((1 as ::core::ffi::c_int)
                                                << ((*ind).readahead as ::core::ffi::c_int
                                                    - 1 as ::core::ffi::c_int)
                                                    * 2 as ::core::ffi::c_int)
                                                as uint32_t,
                                        ) as uint64_t,
                                    );
                                    if blockend > blockstart {
                                    } else {
                                        fprintf(
                                            stderr,
                                            b"%s:%u - failed assertion '%s'\n\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            2409 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"blockend>blockstart\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - failed assertion '%s'\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            2409 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"blockend>blockstart\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        abort();
                                    };
                                    raok = 1 as uint8_t;
                                    let mut rj: usize = 0;
                                    while rj < (&(*ind).reqs).len()
                                        && raok as ::core::ffi::c_int != 0
                                    {
                                        rreqn = (&(*ind).reqs)[rj];
                                        if !((*rreqn).mode as ::core::ffi::c_int
                                            == BREAK as ::core::ffi::c_int
                                            || (*rreqn).mode as ::core::ffi::c_int
                                                == NOTNEEDED as ::core::ffi::c_int)
                                        {
                                            if !(blockend <= (*rreqn).offset
                                                || blockstart
                                                    >= (*rreqn)
                                                        .offset
                                                        .wrapping_add((*rreqn).leng as uint64_t))
                                            {
                                                raok = 0 as uint8_t;
                                            }
                                        }
                                        rj = rj.wrapping_add(1);
                                    }
                                    if raok != 0 {
                                        if blockend <= (*ind).fleng {
                                            read_new_request(ind, &raw mut blockstart, blockend);
                                        } else if blockstart < (*ind).fleng {
                                            read_new_request(
                                                ind,
                                                &raw mut blockstart,
                                                (*ind).fleng,
                                            );
                                        }
                                        // C: rreq->next != NULL &&
                                        // rreq->next->next == NULL — rreq is
                                        // second-to-last. len read live: a
                                        // read-ahead rreq may have just been
                                        // appended above.
                                        if blockstart.wrapping_rem(MFSCHUNKSIZE as uint64_t)
                                            == 0 as uint64_t
                                            && ri.wrapping_add(2) == (&(*ind).reqs).len()
                                            && rbuffsize < maxreadaheadsize
                                        {
                                            blockend = blockstart.wrapping_add(
                                                readahead_leng.wrapping_mul(
                                                    ((1 as ::core::ffi::c_int)
                                                        << ((*ind).readahead as ::core::ffi::c_int
                                                            - 1 as ::core::ffi::c_int)
                                                            * 2 as ::core::ffi::c_int)
                                                        as uint32_t,
                                                )
                                                    as uint64_t,
                                            );
                                            if blockend > blockstart {
                                            } else {
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - failed assertion '%s'\n\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfsclient/readdata.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    2433 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"blockend>blockstart\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - failed assertion '%s'\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfsclient/readdata.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    2433 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"blockend>blockstart\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                abort();
                                            };
                                            raok = 1 as uint8_t;
                                            let mut rj: usize = 0;
                                            while rj < (&(*ind).reqs).len()
                                                && raok as ::core::ffi::c_int != 0
                                            {
                                                rreqn = (&(*ind).reqs)[rj];
                                                if !((*rreqn).mode as ::core::ffi::c_int
                                                    == BREAK as ::core::ffi::c_int
                                                    || (*rreqn).mode as ::core::ffi::c_int
                                                        == NOTNEEDED as ::core::ffi::c_int)
                                                {
                                                    if !(blockend <= (*rreqn).offset
                                                        || blockstart
                                                            >= (*rreqn).offset.wrapping_add(
                                                                (*rreqn).leng as uint64_t,
                                                            ))
                                                    {
                                                        raok = 0 as uint8_t;
                                                    }
                                                }
                                                rj = rj.wrapping_add(1);
                                            }
                                            if raok != 0 {
                                                if blockend <= (*ind).fleng {
                                                    read_new_request(
                                                        ind,
                                                        &raw mut blockstart,
                                                        blockend,
                                                    );
                                                } else if blockstart < (*ind).fleng {
                                                    read_new_request(
                                                        ind,
                                                        &raw mut blockstart,
                                                        (*ind).fleng,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ri = ri.wrapping_add(1);
                }
                if added as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    blockstart = *etab.offset(i as isize);
                    blockend = *etab.offset(i.wrapping_add(1 as uint32_t) as isize);
                    while blockstart < blockend {
                        rreq = read_new_request(ind, &raw mut blockstart, blockend);
                        rlist_vec.push(rlist_s {
                            rreq,
                            offsetadd: 0 as uint32_t,
                            reqleng: (*rreq).leng,
                        });
                        (*rreq).lcnt = (*rreq).lcnt.wrapping_add(1);
                        if !(blockstart == blockend
                            && (*ind).readahead as ::core::ffi::c_int != 0
                            && rbuffsize < maxreadaheadsize
                            && i == edges.wrapping_sub(2 as uint32_t))
                        {
                            continue;
                        }
                        blockend = blockstart.wrapping_add(
                            readahead_leng
                                .wrapping_mul(
                                    ((1 as ::core::ffi::c_int)
                                        << ((*ind).readahead as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int)
                                            * 2 as ::core::ffi::c_int)
                                        as uint32_t,
                                )
                                .wrapping_div(2 as uint32_t)
                                as uint64_t,
                        );
                        if blockend > blockstart {
                        } else {
                            fprintf(
                                stderr,
                                b"%s:%u - failed assertion '%s'\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                2478 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"blockend>blockstart\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - failed assertion '%s'\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                2478 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"blockend>blockstart\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            abort();
                        };
                        raok = 1 as uint8_t;
                        let mut rj: usize = 0;
                        while rj < (&(*ind).reqs).len() && raok as ::core::ffi::c_int != 0 {
                            rreqn = (&(*ind).reqs)[rj];
                            if !((*rreqn).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int
                                || (*rreqn).mode as ::core::ffi::c_int
                                    == NOTNEEDED as ::core::ffi::c_int)
                            {
                                if !(blockend <= (*rreqn).offset
                                    || blockstart
                                        >= (*rreqn).offset.wrapping_add((*rreqn).leng as uint64_t))
                                {
                                    raok = 0 as uint8_t;
                                }
                            }
                            rj = rj.wrapping_add(1);
                        }
                        if raok != 0 {
                            if blockend <= (*ind).fleng {
                                read_new_request(ind, &raw mut blockstart, blockend);
                            } else if blockstart < (*ind).fleng {
                                read_new_request(ind, &raw mut blockstart, (*ind).fleng);
                            }
                        }
                        break;
                    }
                }
                i = i.wrapping_add(1);
            }
            // C: *vrhead = rhead — the list crosses the void* ABI as a
            // boxed Vec; read_data_free_buff reclaims it. Tail-append build
            // order above == C's list order, so the data-wait and iovec
            // walks below see the same sequence.
            rls = Box::into_raw(Box::new(rlist_vec));
            *vrhead = rls as *mut ::core::ffi::c_void;
            cnt = 0 as uint32_t;
            *size = 0 as uint32_t;
            let mut li: usize = 0;
            while li < (&*rls).len() {
                while !((*(&mut *rls)[li].rreq).mode as ::core::ffi::c_int
                    == READY as ::core::ffi::c_int
                    || (*(&mut *rls)[li].rreq).mode as ::core::ffi::c_int
                        == NOTNEEDED as ::core::ffi::c_int)
                    && (*ind).status == 0 as ::core::ffi::c_int
                    && (*ind).closing as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    if usectimeout > 0 as uint64_t {
                        // C: pthread_cond_timedwait on gettimeofday+usectimeout
                        // abstime; relative Duration is the identical timeout.
                        if ind_cond_timedwait(
                            &raw const (*(&mut *rls)[li].rreq).cond,
                            ind,
                            std::time::Duration::from_micros(usectimeout),
                        ) {
                            (*ind).status = EIO;
                        }
                    } else {
                        ind_cond_wait(&raw const (*(&mut *rls)[li].rreq).cond, ind);
                    }
                }
                if (*ind).status != 0 as ::core::ffi::c_int {
                    break;
                }
                if (*(&mut *rls)[li].rreq).rleng
                    < (&mut *rls)[li]
                        .offsetadd
                        .wrapping_add((&mut *rls)[li].reqleng)
                {
                    if (*(&mut *rls)[li].rreq).rleng > (&mut *rls)[li].offsetadd {
                        cnt = cnt.wrapping_add(1);
                        (&mut *rls)[li].reqleng = (*(&mut *rls)[li].rreq)
                            .rleng
                            .wrapping_sub((&mut *rls)[li].offsetadd);
                        *size = (*size).wrapping_add((&mut *rls)[li].reqleng);
                    }
                    break;
                } else {
                    cnt = cnt.wrapping_add(1);
                    *size = (*size).wrapping_add((&mut *rls)[li].reqleng);
                    li = li.wrapping_add(1);
                }
            }
        }
        if (*ind).status == 0 as ::core::ffi::c_int
            && (*ind).closing as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && cnt > 0 as uint32_t
        {
            (*ind).lastoffset = offset.wrapping_add(*size as uint64_t);
            if ((*ind).readahead as ::core::ffi::c_int) < READAHEAD_MAX {
                (*ind).seqdata = (*ind).seqdata.wrapping_add(*size);
            }
            // C: malloc(cnt * sizeof(iovec)) + passert. Box<[iovec]> leaked as
            // raw ptr (uninit like C; every entry written before use below);
            // freed by read_data_free_buff with the count passed by callers.
            *iov = Box::into_raw(Box::<[iovec]>::new_uninit_slice(cnt as usize).assume_init())
                as *mut iovec;
            // C: for (rl=rhead, i=0 ; i<cnt ; rl=rl->next, i++) with
            // passert(rl). cnt <= list length by construction, so Vec
            // indexing never leaves the list; a bounds violation panics
            // (abort), matching passert. The mmap -1 branch was c2rust's
            // passert macro expansion for malloc — Box never yields it.
            i = 0 as uint32_t;
            while i < cnt {
                let rl = &(&*rls)[i as usize];
                (*(*iov).offset(i as isize)).iov_base =
                    (*rl.rreq).data.offset(rl.offsetadd as isize) as *mut ::core::ffi::c_void;
                (*(*iov).offset(i as isize)).iov_len = rl.reqleng as size_t;
                i = i.wrapping_add(1);
            }
            *iovcnt = i;
        } else {
            *iovcnt = 0 as uint32_t;
            *iov = ::core::ptr::null_mut::<iovec>();
        }
        status = (*ind).status;
        ind_unlock(ind);
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_data_free_buff(
    mut vid: *mut ::core::ffi::c_void,
    mut vrhead: *mut ::core::ffi::c_void,
    mut iov: *mut iovec,
    mut iovcnt: uint32_t,
) {
    unsafe {
        let mut ind: *mut inodedata = vid as *mut inodedata;
        let mut rreq: *mut rrequest = ::core::ptr::null_mut::<rrequest>();
        ind_lock(ind);
        if !vrhead.is_null() {
            // C: rl = vrhead; while (rl) { rln = rl->next; ... free(rl);
            // rl = rln; } — boxed Vec iterates in the same order and frees
            // itself on drop.
            let rls = Box::from_raw(vrhead as *mut Vec<rlist>);
            for rl in rls.iter() {
                rreq = rl.rreq;
                (*rreq).lcnt = (*rreq).lcnt.wrapping_sub(1);
                if (*rreq).lcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*rreq).mode as ::core::ffi::c_int == NOTNEEDED as ::core::ffi::c_int
                {
                    read_delete_request(rreq);
                }
            }
        }
        if !iov.is_null() {
            // C: free(iov). Box<[iovec]> allocated with iovcnt entries.
            drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                iov,
                iovcnt as usize,
            )));
        }
        if (*ind).closing as ::core::ffi::c_int != 0 && (&(*ind).reqs).is_empty() {
            (*ind).closecond.notify_all();
        }
        (*ind).readers_cnt = (*ind).readers_cnt.wrapping_sub(1);
        if (*ind).waiting_writers as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && (*ind).readers_cnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*ind).writerscond.notify_one();
        }
        ind_unlock(ind);
        inode_global_lock();
        (*ind).lcnt = (*ind).lcnt.wrapping_sub(1);
        if (*ind).lcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            read_inode_free(
                (*ind)
                    .inode
                    .wrapping_mul(0xb239fb71 as uint32_t)
                    .wrapping_rem(IDHASHSIZE as uint32_t),
                ind,
            );
        }
        inode_global_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_inode_clear_cache(
    mut inode: uint32_t,
    mut offset: uint64_t,
    mut leng: uint64_t,
) {
    unsafe {
        let mut indh: uint32_t = inode
            .wrapping_mul(0xb239fb71 as uint32_t)
            .wrapping_rem(IDHASHSIZE as uint32_t);
        inode_global_lock();
        let mut bi: usize = 0;
        while bi < (&(*&raw mut indhash)[indh as usize]).len() {
            let ind: *mut inodedata = (&(*&raw mut indhash)[indh as usize])[bi];
            if (*ind).inode == inode {
                ind_lock(ind);
                // C: for (rreq = ind->reqhead ; rreq ; rreq=rreqn) with
                // rreqn captured pre-call — read_rreq_invalidate may delete
                // the current node; its successor then shifted into slot ri.
                let mut ri: usize = 0;
                while ri < (&(*ind).reqs).len() {
                    let rreq: *mut rrequest = (&(*ind).reqs)[ri];
                    if leng == 0 as uint64_t
                        && offset < (*rreq).offset.wrapping_add((*rreq).leng as uint64_t)
                        || offset.wrapping_add(leng) > (*rreq).offset
                            && (*rreq).offset.wrapping_add((*rreq).leng as uint64_t) > offset
                    {
                        read_rreq_invalidate(rreq);
                    }
                    if (&(*ind).reqs).get(ri).copied() == Some(rreq) {
                        ri = ri.wrapping_add(1);
                    }
                }
                ind_unlock(ind);
            }
            bi = bi.wrapping_add(1);
        }
        inode_global_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_data_set_length_active(
    mut ind: *mut inodedata,
    mut newlength: uint64_t,
) {
    unsafe {
        ind_lock(ind);
        (*ind).waiting_writers = (*ind).waiting_writers.wrapping_add(1);
        while (*ind).readers_cnt as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if (*ind).fleng == newlength {
                (*ind).waiting_writers = (*ind).waiting_writers.wrapping_sub(1);
                ind_unlock(ind);
                return;
            }
            ind_cond_wait(&raw const (*ind).writerscond, ind);
        }
        (*ind).waiting_writers = (*ind).waiting_writers.wrapping_sub(1);
        (*ind).fleng = newlength;
        // C: for (rreq = ind->reqhead ; rreq ; rreq=rreqn) — same
        // removal-during-iteration protocol as read_inode_clear_cache.
        let mut ri: usize = 0;
        while ri < (&(*ind).reqs).len() {
            let rreq: *mut rrequest = (&(*ind).reqs)[ri];
            if (*rreq).lcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2675 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rreq->lcnt==0\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/readdata.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2675 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rreq->lcnt==0\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            };
            read_rreq_invalidate(rreq);
            if (&(*ind).reqs).get(ri).copied() == Some(rreq) {
                ri = ri.wrapping_add(1);
            }
        }
        if (*ind).closing as ::core::ffi::c_int != 0 && (&(*ind).reqs).is_empty() {
            (*ind).closecond.notify_all();
        }
        if (*ind).waiting_writers as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            (*ind).writerscond.notify_one();
        } else {
            (*ind).readerscond.notify_all();
        }
        ind_unlock(ind);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_inode_set_length_active(
    mut inode: uint32_t,
    mut newlength: uint64_t,
) {
    unsafe {
        let mut indh: uint32_t = inode
            .wrapping_mul(0xb239fb71 as uint32_t)
            .wrapping_rem(IDHASHSIZE as uint32_t);
        inode_global_lock();
        let mut bi: usize = 0;
        while bi < (&(*&raw mut indhash)[indh as usize]).len() {
            let ind: *mut inodedata = (&(*&raw mut indhash)[indh as usize])[bi];
            if (*ind).inode == inode {
                (*ind).lcnt = (*ind).lcnt.wrapping_add(1);
                inode_global_unlock();
                read_data_set_length_active(ind, newlength);
                inode_global_lock();
                // C: indn = ind->next — re-derived after relock. The bucket
                // may have changed while unlocked, but ind itself is still
                // linked: we hold an lcnt ref, and only the lcnt==0 path
                // unlinks. Re-find ind's slot.
                bi = (*&raw mut indhash)[indh as usize]
                    .iter()
                    .position(|&p| p == ind)
                    .expect("read_inode_set_length_active: ind unlinked while referenced");
                (*ind).lcnt = (*ind).lcnt.wrapping_sub(1);
                if (*ind).lcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    read_inode_free(indh, ind);
                    // Removal shifted the successor into slot bi — rescan it.
                } else {
                    bi = bi.wrapping_add(1);
                }
            } else {
                bi = bi.wrapping_add(1);
            }
        }
        inode_global_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_inode_set_length_passive(
    mut inode: uint32_t,
    mut newlength: uint64_t,
) {
    unsafe {
        let mut indh: uint32_t = inode
            .wrapping_mul(0xb239fb71 as uint32_t)
            .wrapping_rem(IDHASHSIZE as uint32_t);
        let mut minfleng: uint64_t = 0;
        let mut maxfleng: uint64_t = 0;
        inode_global_lock();
        let mut bi: usize = 0;
        while bi < (&(*&raw mut indhash)[indh as usize]).len() {
            let ind: *mut inodedata = (&(*&raw mut indhash)[indh as usize])[bi];
            if (*ind).inode == inode {
                ind_lock(ind);
                if (*ind).fleng != newlength {
                    if (*ind).fleng < newlength {
                        minfleng = (*ind).fleng;
                        maxfleng = newlength;
                    } else {
                        minfleng = newlength;
                        maxfleng = (*ind).fleng;
                    }
                    // C: for (rreq = ind->reqhead ; rreq ; rreq=rreqn) —
                    // same removal-during-iteration protocol as
                    // read_inode_clear_cache.
                    let mut ri: usize = 0;
                    while ri < (&(*ind).reqs).len() {
                        let rreq: *mut rrequest = (&(*ind).reqs)[ri];
                        if (*rreq).offset < maxfleng
                            && (*rreq).offset.wrapping_add((*rreq).leng as uint64_t) > minfleng
                        {
                            read_rreq_invalidate(rreq);
                        }
                        if (&(*ind).reqs).get(ri).copied() == Some(rreq) {
                            ri = ri.wrapping_add(1);
                        }
                    }
                    (*ind).fleng = newlength;
                }
                ind_unlock(ind);
            }
            bi = bi.wrapping_add(1);
        }
        inode_global_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_data_new(
    mut inode: uint32_t,
    mut fleng: uint64_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut indh: uint32_t = inode
            .wrapping_mul(0xb239fb71 as uint32_t)
            .wrapping_rem(IDHASHSIZE as uint32_t);
        let mut ind: *mut inodedata = Box::into_raw(Box::new(inodedata_s {
            inode,
            seqdata: 0 as uint32_t,
            fleng,
            status: 0 as ::core::ffi::c_int,
            closing: 0 as uint8_t,
            inqueue: 0 as uint8_t,
            readahead: 0 as uint8_t,
            lastoffset: 0 as uint64_t,
            waiting_writers: 0 as uint16_t,
            readers_cnt: 0 as uint16_t,
            lcnt: 0 as uint16_t,
            reqs: Vec::new(),
            closecond: std::sync::Condvar::new(),
            readerscond: std::sync::Condvar::new(),
            writerscond: std::sync::Condvar::new(),
            lock: std::sync::Mutex::new(()),
        }));
        inode_global_lock();
        (*ind).lcnt = 1 as uint16_t;
        // C: ind->next = indhash[indh]; indhash[indh] = ind; — head insert.
        (*&raw mut indhash)[indh as usize].insert(0, ind);
        inode_global_unlock();
        return ind as *mut ::core::ffi::c_void;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_data_end(mut vid: *mut ::core::ffi::c_void) {
    unsafe {
        let mut indh: uint32_t = 0;
        let mut ind: *mut inodedata = ::core::ptr::null_mut::<inodedata>();
        ind = vid as *mut inodedata;
        indh = (*ind)
            .inode
            .wrapping_mul(0xb239fb71 as uint32_t)
            .wrapping_rem(IDHASHSIZE as uint32_t);
        ind_lock(ind);
        (*ind).closing = 1 as uint8_t;
        // C: for (rreq = ind->reqhead ; rreq ; rreq=rreqn) with rreqn
        // captured pre-delete — read_delete_request unlinks the current
        // node; its successor shifted into slot ri.
        let mut ri: usize = 0;
        while ri < (&(*ind).reqs).len() {
            let rreq: *mut rrequest = (&(*ind).reqs)[ri];
            if (*rreq).lcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && !((*rreq).mode as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                    || (*rreq).mode as ::core::ffi::c_int == INQUEUE as ::core::ffi::c_int
                    || (*rreq).mode as ::core::ffi::c_int == REFRESH as ::core::ffi::c_int
                    || (*rreq).mode as ::core::ffi::c_int == BREAK as ::core::ffi::c_int
                    || (*rreq).mode as ::core::ffi::c_int == FILLED as ::core::ffi::c_int)
            {
                read_delete_request(rreq);
            } else {
                ri = ri.wrapping_add(1);
            }
        }
        // C: while (ind->reqhead != NULL) { wake reqhead->waitingworker;
        // cond_wait; } — the list head is reqs[0].
        while !(&(*ind).reqs).is_empty() {
            let head: *mut rrequest = (&(*ind).reqs)[0];
            if (*head).waitingworker != 0 {
                if write(
                    (*head).wakeup_fd,
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
                (*head).waitingworker = 0 as uint8_t;
                (*head).wakeup_fd = -1 as ::core::ffi::c_int;
            }
            ind_cond_wait(&raw const (*ind).closecond, ind);
        }
        ind_unlock(ind);
        inode_global_lock();
        (*ind).lcnt = (*ind).lcnt.wrapping_sub(1);
        if (*ind).lcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            read_inode_free(indh, ind);
        }
        inode_global_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_init() {
    unsafe {
        read_get_total_bytes();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_term() {}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

// ponytail: c2rust dropped the _Atomic static (see mastercomm.rs note)
static mut total_bytes_rcvd: uint64_t = 0;
