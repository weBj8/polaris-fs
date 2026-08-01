pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
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
    unsafe fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn cfg_getuint8(name: *const ::core::ffi::c_char, def: uint8_t) -> uint8_t;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn tcpsocket() -> ::core::ffi::c_int;
    unsafe fn tcpnonblock(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpgetstatus(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnodelay(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnumtoconnect(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
        msecto: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcptoread(
        sock: ::core::ffi::c_int,
        buff: *mut ::core::ffi::c_void,
        leng: uint32_t,
        msectopart: uint32_t,
        msectoall: uint32_t,
    ) -> int32_t;
    unsafe fn tcptowrite(
        sock: ::core::ffi::c_int,
        buff: *const ::core::ffi::c_void,
        leng: uint32_t,
        msectopart: uint32_t,
        msectoall: uint32_t,
    ) -> int32_t;
    unsafe fn tcptoforward(
        srcsock: ::core::ffi::c_int,
        dstsock: ::core::ffi::c_int,
        buff: *mut ::core::ffi::c_void,
        leng: uint32_t,
        rcvd: uint32_t,
        sent: uint32_t,
        msectopart: uint32_t,
        msectoall: uint32_t,
    ) -> int32_t;
    unsafe fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn hdd_precache_data(chunkid: uint64_t, offset: uint32_t, size: uint32_t);
    unsafe fn hdd_open(chunkid: uint64_t, version: uint32_t) -> ::core::ffi::c_int;
    unsafe fn hdd_close(chunkid: uint64_t, forcefsync: uint8_t) -> ::core::ffi::c_int;
    unsafe fn hdd_read(
        chunkid: uint64_t,
        version: uint32_t,
        blocknum: uint16_t,
        buffer: *mut uint8_t,
        offset: uint32_t,
        size: uint32_t,
        crcbuff: *mut uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn hdd_write(
        chunkid: uint64_t,
        version: uint32_t,
        blocknum: uint16_t,
        buffer: *const uint8_t,
        offset: uint32_t,
        size: uint32_t,
        crcbuff: *const uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn lwt_minthread_create(
        th: *mut pthread_t,
        detached: uint8_t,
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn monotonic_useconds() -> uint64_t;
    unsafe fn conncache_insert(ip: uint32_t, port: uint16_t, fd: ::core::ffi::c_int);
    unsafe fn conncache_get(ip: uint32_t, port: uint16_t) -> ::core::ffi::c_int;
    unsafe fn conncache_term();
    unsafe fn conncache_init(capacity: uint32_t) -> ::core::ffi::c_int;
    unsafe fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub struct sock_nops {
    pub sock: ::core::ffi::c_int,
    pub error: uint8_t,
    pub monotonic_utime: uint64_t,
    pub bytesleft: uint32_t,
    pub next: *mut sock_nops,
    pub prev: *mut *mut sock_nops,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct write_xchg {
    pub chunkid: uint64_t,
    pub version: uint32_t,
    pub pipe: [::core::ffi::c_int; 2],
    pub head: *mut write_job,
    pub hddhead: *mut write_job,
    pub nethead: *mut write_job,
    pub tail: *mut *mut write_job,
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub condwaiting: uint8_t,
    pub term: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct write_job {
    pub chunkid: uint64_t,
    pub writeid: uint32_t,
    pub blocknum: uint16_t,
    pub offset: uint16_t,
    pub size: uint32_t,
    pub crcptr: *const uint8_t,
    pub buff: *const uint8_t,
    pub hddstatus: uint8_t,
    pub netstatus: uint8_t,
    pub ack: uint8_t,
    pub next: *mut write_job,
    pub structsize: uint32_t,
    pub data: [uint8_t; 1],
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MFSCHUNKSIZE: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const MFSBLOCKSIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const MFSBLOCKMASK: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;
pub const MFSBLOCKBITS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_WRONGSIZE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const MFS_ERROR_WRONGOFFSET: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const MFS_ERROR_CANTCONNECT: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const MFS_ERROR_WRONGCHUNKID: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const MFS_ERROR_DISCONNECTED: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CSTOCS_MAXPACKETSIZE: ::core::ffi::c_int = 100000 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CSTOCL_READ_STATUS: ::core::ffi::c_int = PROTO_BASE + 201 as ::core::ffi::c_int;
pub const CSTOCL_READ_DATA: ::core::ffi::c_int = PROTO_BASE + 202 as ::core::ffi::c_int;
pub const CLTOCS_WRITE: ::core::ffi::c_int = PROTO_BASE + 210 as ::core::ffi::c_int;
pub const CSTOCL_WRITE_STATUS: ::core::ffi::c_int = PROTO_BASE + 211 as ::core::ffi::c_int;
pub const CLTOCS_WRITE_DATA: ::core::ffi::c_int = PROTO_BASE + 212 as ::core::ffi::c_int;
pub const CLTOCS_WRITE_FINISH: ::core::ffi::c_int = PROTO_BASE + 213 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const ECONNRESET: ::core::ffi::c_int = 104 as ::core::ffi::c_int;
pub const PROTO_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EPIPE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_PRIVATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_ANONYMOUS: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MAP_ANON: ::core::ffi::c_int = MAP_ANONYMOUS;
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
unsafe extern "C" fn portable_usleep(mut usec: uint64_t) {
    unsafe {
        let mut req: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut rem: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut s: ::core::ffi::c_int = 0;
        req.tv_sec = usec.wrapping_div(1000000 as uint64_t) as __time_t;
        req.tv_nsec = usec
            .wrapping_rem(1000000 as uint64_t)
            .wrapping_mul(1000 as uint64_t) as __syscall_slong_t;
        loop {
            s = nanosleep(&raw mut req, &raw mut rem);
            if s < 0 as ::core::ffi::c_int {
                req = rem;
            }
            if s >= 0 as ::core::ffi::c_int {
                break;
            }
        }
    }
}
pub const SERV_TIMEOUT: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
pub const NOPS_INTERVAL: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
pub const SMALL_PACKET_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const CONNECT_RETRIES: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut MaxPacketSize: uint32_t = CSTOCS_MAXPACKETSIZE as uint32_t;
static mut CanUseMmap: uint8_t = 0 as uint8_t;
static mut stats_bytesin: uint64_t = 0 as uint64_t;
static mut stats_bytesout: uint64_t = 0 as uint64_t;
static mut stats_hlopr: uint32_t = 0 as uint32_t;
static mut stats_hlopw: uint32_t = 0 as uint32_t;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainserv_stats(
    mut bin: *mut uint64_t,
    mut bout: *mut uint64_t,
    mut hlopr: *mut uint32_t,
    mut hlopw: *mut uint32_t,
) {
    unsafe {
        *bin = ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut stats_bytesin,
            0 as uint64_t,
        );
        *bout = ::core::intrinsics::atomic_and::<
            _,
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut stats_bytesout, 0 as uint64_t);
        *hlopr = ::core::intrinsics::atomic_and::<
            _,
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut stats_hlopr, 0 as uint32_t);
        *hlopw = ::core::intrinsics::atomic_and::<
            _,
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut stats_hlopw, 0 as uint32_t);
    }
}
#[inline]
unsafe extern "C" fn mainserv_bytesin(mut bytes: uint64_t) {
    unsafe {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut stats_bytesin,
            bytes,
        );
    }
}
#[inline]
unsafe extern "C" fn mainserv_bytesout(mut bytes: uint64_t) {
    unsafe {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut stats_bytesout,
            bytes,
        );
    }
}
#[inline]
unsafe extern "C" fn mainserv_toread(
    mut sock: ::core::ffi::c_int,
    mut ptr: *mut uint8_t,
    mut leng: uint32_t,
    mut timeout: uint32_t,
) -> int32_t {
    unsafe {
        let mut r: int32_t = 0;
        r = tcptoread(
            sock,
            ptr as *mut ::core::ffi::c_void,
            leng,
            timeout,
            timeout.wrapping_mul(30 as uint32_t),
        );
        if r > 0 as int32_t {
            mainserv_bytesin(r as uint64_t);
        }
        return r;
    }
}
#[inline]
unsafe extern "C" fn mainserv_towrite(
    mut sock: ::core::ffi::c_int,
    mut ptr: *const uint8_t,
    mut leng: uint32_t,
    mut timeout: uint32_t,
) -> int32_t {
    unsafe {
        let mut r: int32_t = 0;
        r = tcptowrite(
            sock,
            ptr as *const ::core::ffi::c_void,
            leng,
            timeout,
            timeout.wrapping_mul(30 as uint32_t),
        );
        if r > 0 as int32_t {
            mainserv_bytesout(r as uint64_t);
        }
        return r;
    }
}
#[inline]
unsafe extern "C" fn mainserv_toforward(
    mut sock1: ::core::ffi::c_int,
    mut sock2: ::core::ffi::c_int,
    mut ptr: *mut uint8_t,
    mut leng: uint32_t,
    mut rskip: uint32_t,
    mut wskip: uint32_t,
    mut timeout: uint32_t,
) -> int32_t {
    unsafe {
        let mut r: int32_t = 0;
        r = tcptoforward(
            sock1,
            sock2,
            ptr as *mut ::core::ffi::c_void,
            leng,
            rskip,
            wskip,
            timeout,
            timeout.wrapping_mul(30 as uint32_t),
        );
        if r > 0 as int32_t {
            if r as uint32_t > rskip {
                mainserv_bytesin((r as uint32_t).wrapping_sub(rskip) as uint64_t);
            }
            if r as uint32_t > wskip {
                mainserv_bytesout((r as uint32_t).wrapping_sub(wskip) as uint64_t);
            }
        }
        return r;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainserv_create_packet(
    mut wptr: *mut *mut uint8_t,
    mut cmd: uint32_t,
    mut leng: uint32_t,
) -> *mut uint8_t {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        ptr = malloc(leng.wrapping_add(8 as uint32_t) as size_t) as *mut uint8_t;
        if ptr.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                179 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ptr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                179 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ptr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if ptr
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                179 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ptr\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                179 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ptr\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        *wptr = ptr;
        put32bit(wptr, cmd);
        put32bit(wptr, leng);
        return ptr;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainserv_send_and_free(
    mut packetname: *const ::core::ffi::c_char,
    mut sock: ::core::ffi::c_int,
    mut ptr: *mut uint8_t,
    mut pleng: uint32_t,
) -> uint8_t {
    unsafe {
        let mut r: uint8_t = 0;
        let mut d: uint8_t = 0;
        r = (if mainserv_towrite(
            sock,
            ptr,
            pleng.wrapping_add(8 as uint32_t),
            SERV_TIMEOUT as uint32_t,
        ) != pleng.wrapping_add(8 as uint32_t) as int32_t
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as uint8_t;
        d = (if *__errno_location() == EPIPE || *__errno_location() == ECONNRESET {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        free(ptr as *mut ::core::ffi::c_void);
        if r as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if d != 0 {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"send_and_free: 'send(%s)' disconnected\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    packetname,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"send_and_free: 'send(%s)' timed out\0".as_ptr() as *const ::core::ffi::c_char,
                    packetname,
                );
            }
        }
        return r;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainserv_connect(
    mut fwdip: uint32_t,
    mut fwdport: uint16_t,
    mut timeout: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut fwdsock: ::core::ffi::c_int = 0;
        fwdsock = tcpsocket();
        if fwdsock < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"create socket, error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if tcpnonblock(fwdsock) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"set nonblock, error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            tcpclose(fwdsock);
            return -1 as ::core::ffi::c_int;
        }
        if tcpnumtoconnect(fwdsock, fwdip, fwdport, timeout) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"connect to %u.%u.%u.%u:%u failed, error\0".as_ptr() as *const ::core::ffi::c_char,
                fwdip >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                fwdip >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                fwdip >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                fwdip & 0xff as uint32_t,
                fwdport as ::core::ffi::c_int,
            );
            tcpclose(fwdsock);
            return -1 as ::core::ffi::c_int;
        }
        if tcpnodelay(fwdsock) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_NOTICE,
                b"can't set TCP_NODELAY, error\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return fwdsock;
    }
}
static mut sock_nops_tail: *mut *mut sock_nops = ::core::ptr::null_mut::<*mut sock_nops>();
static mut sock_nops_head: *mut sock_nops = ::core::ptr::null_mut::<sock_nops>();
static mut sock_nops_lock: pthread_mutex_t = pthread_mutex_t {
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
static mut sock_nop_buff: [uint8_t; 8] = [0; 8];
#[inline]
unsafe extern "C" fn mainserv_sock_nop_append(mut sn: *mut sock_nops) {
    unsafe {
        if (*sn).prev.is_null() {
            (*sn).next = ::core::ptr::null_mut::<sock_nops>();
            (*sn).prev = sock_nops_tail as *mut *mut sock_nops;
            *sock_nops_tail = sn;
            sock_nops_tail = &raw mut (*sn).next as *mut *mut sock_nops;
            (*sn).monotonic_utime = monotonic_useconds();
        }
    }
}
#[inline]
unsafe extern "C" fn mainserv_sock_nop_remove(mut sn: *mut sock_nops) {
    unsafe {
        if !(*sn).prev.is_null() {
            if !(*sn).next.is_null() {
                (*(*sn).next).prev = (*sn).prev;
            } else {
                sock_nops_tail = (*sn).prev as *mut *mut sock_nops;
            }
            *(*sn).prev = (*sn).next;
            (*sn).next = ::core::ptr::null_mut::<sock_nops>();
            (*sn).prev = ::core::ptr::null_mut::<*mut sock_nops>();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainserv_sock_nop_sender(
    _arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut monotonic_utime: uint64_t = 0;
        let mut sleep_utime: uint64_t = 0;
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut i: int32_t = 0;
        let mut sn: *mut sock_nops = ::core::ptr::null_mut::<sock_nops>();
        wptr = &raw mut sock_nop_buff as *mut uint8_t;
        put32bit(&raw mut wptr, ANTOAN_NOP as uint32_t);
        put32bit(&raw mut wptr, 0 as uint32_t);
        loop {
            let mut _mfs_assert_ret: ::core::ffi::c_int =
                pthread_mutex_lock(&raw mut sock_nops_lock);
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        272 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        272 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        272 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        272 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        272 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        272 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                }
                abort();
            }
            if sock_nops_head.is_null() {
                sleep_utime = NOPS_INTERVAL as uint64_t;
            } else {
                monotonic_utime = monotonic_useconds();
                while (*sock_nops_head)
                    .monotonic_utime
                    .wrapping_add(NOPS_INTERVAL as uint64_t)
                    <= monotonic_utime
                {
                    if (*sock_nops_head).error as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        if (*sock_nops_head).bytesleft != 0 {
                            i = write(
                                (*sock_nops_head).sock,
                                (&raw mut sock_nop_buff as *mut uint8_t).offset(
                                    (8 as uint32_t).wrapping_sub((*sock_nops_head).bytesleft)
                                        as isize,
                                ) as *const ::core::ffi::c_void,
                                (*sock_nops_head).bytesleft as size_t,
                            ) as int32_t;
                        } else {
                            (*sock_nops_head).bytesleft = 8 as uint32_t;
                            i = write(
                                (*sock_nops_head).sock,
                                &raw mut sock_nop_buff as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                8 as size_t,
                            ) as int32_t;
                        }
                        if i < 0 as int32_t {
                            if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                                (*sock_nops_head).error = 1 as uint8_t;
                                (*sock_nops_head).bytesleft = 0 as uint32_t;
                            }
                        }
                        if i > 0 as int32_t {
                            (*sock_nops_head).bytesleft =
                                (*sock_nops_head).bytesleft.wrapping_sub(i as uint32_t);
                        }
                    }
                    sn = sock_nops_head;
                    mainserv_sock_nop_remove(sn);
                    mainserv_sock_nop_append(sn);
                }
                if monotonic_utime.wrapping_sub((*sock_nops_head).monotonic_utime)
                    < NOPS_INTERVAL as uint64_t
                {
                    sleep_utime = (NOPS_INTERVAL as uint64_t).wrapping_sub(
                        monotonic_utime.wrapping_sub((*sock_nops_head).monotonic_utime),
                    );
                } else {
                    sleep_utime = 0 as uint64_t;
                }
            }
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut sock_nops_lock);
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
            if sleep_utime > 0 as uint64_t {
                portable_usleep(sleep_utime);
            }
        }
    }
}
#[inline]
unsafe extern "C" fn mainserv_sock_nop_init(mut sn: *mut sock_nops, mut sock: ::core::ffi::c_int) {
    unsafe {
        (*sn).sock = sock;
        (*sn).error = 0 as uint8_t;
        (*sn).bytesleft = 0 as uint32_t;
        (*sn).next = ::core::ptr::null_mut::<sock_nops>();
        (*sn).prev = ::core::ptr::null_mut::<*mut sock_nops>();
    }
}
#[inline]
unsafe extern "C" fn mainserv_sock_nop_add(mut sn: *mut sock_nops) {
    unsafe {
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut sock_nops_lock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    322 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    322 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    322 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    322 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    322 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    322 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        mainserv_sock_nop_append(sn);
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_mutex_unlock(&raw mut sock_nops_lock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    324 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    324 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    324 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    324 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    324 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    324 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
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
}
#[inline]
unsafe extern "C" fn mainserv_sock_nop_del(mut sn: *mut sock_nops) {
    unsafe {
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut sock_nops_lock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    328 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    328 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    328 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    328 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    328 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    328 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&sock_nops_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        mainserv_sock_nop_remove(sn);
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_mutex_unlock(&raw mut sock_nops_lock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&sock_nops_lock)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        if (*sn).bytesleft > 0 as uint32_t && (*sn).bytesleft < 8 as uint32_t {
            if mainserv_towrite(
                (*sn).sock,
                (&raw mut sock_nop_buff as *mut uint8_t)
                    .offset((8 as uint32_t).wrapping_sub((*sn).bytesleft) as isize),
                (*sn).bytesleft,
                SERV_TIMEOUT as uint32_t,
            ) != (*sn).bytesleft as int32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"nop_tail: 'send(nop)' timed out\0".as_ptr() as *const ::core::ffi::c_char,
                );
                (*sn).error = 1 as uint8_t;
            }
        }
        (*sn).bytesleft = 0 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainserv_read(
    mut sock: ::core::ffi::c_int,
    mut data: *const uint8_t,
    mut length: uint32_t,
) -> uint8_t {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut offset: uint32_t = 0;
        let mut size: uint32_t = 0;
        let mut blocknum: uint16_t = 0;
        let mut blockoffset: uint16_t = 0;
        let mut blocksize: uint32_t = 0;
        let mut packet: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut ret: uint8_t = 0;
        let mut protover: uint8_t = 0;
        let mut i: int32_t = 0;
        let mut rcvd: uint32_t = 0;
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut hdr: [uint8_t; 8] = [0; 8];
        let mut cmd: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut sn: sock_nops = sock_nops {
            sock: 0,
            error: 0,
            monotonic_utime: 0,
            bytesleft: 0,
            next: ::core::ptr::null_mut::<sock_nops>(),
            prev: ::core::ptr::null_mut::<*mut sock_nops>(),
        };
        if length != 20 as uint32_t && length != 21 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOCS_READ - wrong size (%u/20|21)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            return 0 as uint8_t;
        }
        if length == 21 as uint32_t {
            protover = get8bit(&raw mut data);
            if protover != 0 {
                mainserv_sock_nop_init(&raw mut sn, sock);
            }
        } else {
            protover = 0 as uint8_t;
        }
        chunkid = get64bit(&raw mut data);
        version = get32bit(&raw mut data);
        offset = get32bit(&raw mut data);
        size = get32bit(&raw mut data);
        if size == 0 as uint32_t {
            packet = mainserv_create_packet(
                &raw mut wptr,
                CSTOCL_READ_STATUS as uint32_t,
                (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(&raw mut wptr, chunkid);
            put8bit(&raw mut wptr, MFS_STATUS_OK as uint8_t);
            return mainserv_send_and_free(
                b"read status\0".as_ptr() as *const ::core::ffi::c_char,
                sock,
                packet,
                (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
            );
        }
        if size > MFSCHUNKSIZE as uint32_t {
            packet = mainserv_create_packet(
                &raw mut wptr,
                CSTOCL_READ_STATUS as uint32_t,
                (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(&raw mut wptr, chunkid);
            put8bit(&raw mut wptr, MFS_ERROR_WRONGSIZE as uint8_t);
            return mainserv_send_and_free(
                b"read status\0".as_ptr() as *const ::core::ffi::c_char,
                sock,
                packet,
                (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
            );
        }
        if offset >= MFSCHUNKSIZE as uint32_t
            || offset.wrapping_add(size) > MFSCHUNKSIZE as uint32_t
        {
            packet = mainserv_create_packet(
                &raw mut wptr,
                CSTOCL_READ_STATUS as uint32_t,
                (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(&raw mut wptr, chunkid);
            put8bit(&raw mut wptr, MFS_ERROR_WRONGOFFSET as uint8_t);
            return mainserv_send_and_free(
                b"read status\0".as_ptr() as *const ::core::ffi::c_char,
                sock,
                packet,
                (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
            );
        }
        if protover != 0 {
            mainserv_sock_nop_add(&raw mut sn);
        }
        status = hdd_open(chunkid, version) as uint8_t;
        if protover != 0 {
            mainserv_sock_nop_del(&raw mut sn);
            if sn.error != 0 {
                if status as ::core::ffi::c_int == MFS_STATUS_OK {
                    hdd_close(chunkid, 0 as uint8_t);
                }
                return 0 as uint8_t;
            }
        }
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            packet = mainserv_create_packet(
                &raw mut wptr,
                CSTOCL_READ_STATUS as uint32_t,
                (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(&raw mut wptr, chunkid);
            put8bit(&raw mut wptr, status);
            return mainserv_send_and_free(
                b"read status\0".as_ptr() as *const ::core::ffi::c_char,
                sock,
                packet,
                (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
            );
        }
        if protover != 0 {
            mainserv_sock_nop_add(&raw mut sn);
        }
        hdd_precache_data(chunkid, offset, size);
        if protover != 0 {
            mainserv_sock_nop_del(&raw mut sn);
            if sn.error != 0 {
                if status as ::core::ffi::c_int == MFS_STATUS_OK {
                    hdd_close(chunkid, 0 as uint8_t);
                }
                return 0 as uint8_t;
            }
        }
        rcvd = 0 as uint32_t;
        while size > 0 as uint32_t {
            blocknum = (offset >> MFSBLOCKBITS) as uint16_t;
            blockoffset = (offset & MFSBLOCKMASK as uint32_t) as uint16_t;
            if offset.wrapping_add(size).wrapping_sub(1 as uint32_t) >> MFSBLOCKBITS
                == blocknum as uint32_t
            {
                blocksize = size;
            } else {
                blocksize = (MFSBLOCKSIZE - blockoffset as ::core::ffi::c_int) as uint32_t;
            }
            packet = mainserv_create_packet(
                &raw mut wptr,
                CSTOCL_READ_DATA as uint32_t,
                ((8 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add(blocksize),
            );
            put64bit(&raw mut wptr, chunkid);
            put16bit(&raw mut wptr, blocknum);
            put16bit(&raw mut wptr, blockoffset);
            put32bit(&raw mut wptr, blocksize);
            if protover != 0 {
                mainserv_sock_nop_add(&raw mut sn);
            }
            status = hdd_read(
                chunkid,
                version,
                blocknum,
                wptr.offset(4 as ::core::ffi::c_int as isize),
                blockoffset as uint32_t,
                blocksize,
                wptr,
            ) as uint8_t;
            if protover != 0 {
                mainserv_sock_nop_del(&raw mut sn);
                if sn.error != 0 {
                    hdd_close(chunkid, 0 as uint8_t);
                    return 0 as uint8_t;
                }
            }
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                free(packet as *mut ::core::ffi::c_void);
                hdd_close(chunkid, 0 as uint8_t);
                packet = mainserv_create_packet(
                    &raw mut wptr,
                    CSTOCL_READ_STATUS as uint32_t,
                    (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
                );
                put64bit(&raw mut wptr, chunkid);
                put8bit(&raw mut wptr, status);
                ret = mainserv_send_and_free(
                    b"read status\0".as_ptr() as *const ::core::ffi::c_char,
                    sock,
                    packet,
                    (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
                );
                ::core::intrinsics::atomic_xadd::<
                    _,
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(&raw mut stats_hlopr, 1 as uint32_t);
                return ret;
            }
            if mainserv_send_and_free(
                b"read data\0".as_ptr() as *const ::core::ffi::c_char,
                sock,
                packet,
                ((8 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add(blocksize),
            ) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                hdd_close(chunkid, 0 as uint8_t);
                return 0 as uint8_t;
            }
            offset = offset.wrapping_add(blocksize);
            size = size.wrapping_sub(blocksize);
            i = read(
                sock,
                (&raw mut hdr as *mut uint8_t).offset(rcvd as isize) as *mut ::core::ffi::c_void,
                (8 as uint32_t).wrapping_sub(rcvd) as size_t,
            ) as int32_t;
            if i < 0 as int32_t {
                if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                    hdd_close(chunkid, 0 as uint8_t);
                    return 0 as uint8_t;
                }
            } else if i == 0 as int32_t {
                hdd_close(chunkid, 0 as uint8_t);
                return 0 as uint8_t;
            } else {
                rcvd = rcvd.wrapping_add(i as uint32_t);
                if rcvd == 8 as uint32_t {
                    rptr = &raw mut hdr as *mut uint8_t;
                    cmd = get32bit(&raw mut rptr);
                    leng = get32bit(&raw mut rptr);
                    if cmd == ANTOAN_NOP as uint32_t && leng == 0 as uint32_t {
                        rcvd = 0 as uint32_t;
                    } else {
                        hdd_close(chunkid, 0 as uint8_t);
                        return 0 as uint8_t;
                    }
                }
            }
        }
        hdd_close(chunkid, 0 as uint8_t);
        packet = mainserv_create_packet(
            &raw mut wptr,
            CSTOCL_READ_STATUS as uint32_t,
            (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
        );
        put64bit(&raw mut wptr, chunkid);
        put8bit(&raw mut wptr, MFS_STATUS_OK as uint8_t);
        ret = mainserv_send_and_free(
            b"read status\0".as_ptr() as *const ::core::ffi::c_char,
            sock,
            packet,
            (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
        );
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut stats_hlopr,
            1 as uint32_t,
        );
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainserv_write_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut wrdata: *mut write_xchg = arg as *mut write_xchg;
        let mut wrjob: *mut write_job = ::core::ptr::null_mut::<write_job>();
        let mut gchunkid: uint64_t = 0;
        let mut gversion: uint32_t = 0;
        let mut status: uint8_t = 0;
        loop {
            let mut _mfs_assert_ret: ::core::ffi::c_int =
                pthread_mutex_lock(&raw mut (*wrdata).lock);
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        545 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(wrdata->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        545 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(wrdata->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        545 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(wrdata->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        545 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(wrdata->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        545 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(wrdata->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        545 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(wrdata->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                }
                abort();
            }
            while (*wrdata).hddhead.is_null()
                && (*wrdata).term as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                (*wrdata).condwaiting = 1 as uint8_t;
                let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                    pthread_cond_wait(&raw mut (*wrdata).cond, &raw mut (*wrdata).lock);
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
                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            548 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&(wrdata->cond),&(wrdata->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            548 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&(wrdata->cond),&(wrdata->lock))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            548 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&(wrdata->cond),&(wrdata->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            548 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&(wrdata->cond),&(wrdata->lock))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            548 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&(wrdata->cond),&(wrdata->lock))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            548 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&(wrdata->cond),&(wrdata->lock))\0".as_ptr()
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
            if (*wrdata).term != 0 {
                let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut (*wrdata).lock);
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
                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            551 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_3,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            551 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            551 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_4,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            551 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            551 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            551 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_err_1,
                        );
                    }
                    abort();
                }
                return NULL;
            }
            wrjob = (*wrdata).hddhead;
            gchunkid = (*wrdata).chunkid;
            gversion = (*wrdata).version;
            let mut _mfs_assert_ret_2: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut (*wrdata).lock);
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        557 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_5,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        557 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        557 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_6,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        557 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        557 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        557 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_err_2,
                    );
                }
                abort();
            }
            status = hdd_write(
                gchunkid,
                gversion,
                (*wrjob).blocknum,
                (*wrjob).buff,
                (*wrjob).offset as uint32_t,
                (*wrjob).size,
                (*wrjob).crcptr,
            ) as uint8_t;
            let mut _mfs_assert_ret_3: ::core::ffi::c_int =
                pthread_mutex_lock(&raw mut (*wrdata).lock);
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        559 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(wrdata->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_3,
                        *__errno_location(),
                        _mfs_errorstring_7,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        559 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(wrdata->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        559 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(wrdata->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_3,
                        _mfs_errorstring_8,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        559 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(wrdata->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        559 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(wrdata->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        559 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(wrdata->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_3,
                        _mfs_errorstring_ret_3,
                        *__errno_location(),
                        _mfs_errorstring_err_3,
                    );
                }
                abort();
            }
            (*wrjob).hddstatus = status;
            (*wrjob).ack =
                ((*wrjob).ack as ::core::ffi::c_int | 1 as ::core::ffi::c_int) as uint8_t;
            (*wrdata).hddhead = (*wrjob).next as *mut write_job;
            let mut _mfs_assert_ret_4: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut (*wrdata).lock);
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        563 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        *__errno_location(),
                        _mfs_errorstring_9,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        563 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        563 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        _mfs_errorstring_10,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        563 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        563 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        563 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(wrdata->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        _mfs_errorstring_ret_4,
                        *__errno_location(),
                        _mfs_errorstring_err_4,
                    );
                }
                abort();
            }
            if write(
                (*wrdata).pipe[1 as usize],
                b"*\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                1 as size_t,
            ) != 1 as ssize_t
            {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"pipe write error\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                break;
            }
        }
        return NULL;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainserv_write_middle(
    mut sock: ::core::ffi::c_int,
    mut fwdsock: ::core::ffi::c_int,
    mut gchunkid: uint64_t,
    mut gversion: uint32_t,
    mut protover: uint8_t,
    mut sn: *mut sock_nops,
    mut fsn: *mut sock_nops,
) -> uint8_t {
    unsafe {
        let mut wrthread: pthread_t = 0;
        let mut wrdata: write_xchg = write_xchg {
            chunkid: 0,
            version: 0,
            pipe: [0; 2],
            head: ::core::ptr::null_mut::<write_job>(),
            hddhead: ::core::ptr::null_mut::<write_job>(),
            nethead: ::core::ptr::null_mut::<write_job>(),
            tail: ::core::ptr::null_mut::<*mut write_job>(),
            lock: pthread_mutex_t {
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
            },
            cond: pthread_cond_t {
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
            },
            condwaiting: 0,
            term: 0,
        };
        let mut wrjob: *mut write_job = ::core::ptr::null_mut::<write_job>();
        let mut packet: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut pfd: [pollfd; 3] = [pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        }; 3];
        let mut hdr: [uint8_t; 20] = [0; 20];
        let mut pdata: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut pdataleng: uint32_t = 0;
        let mut cmd: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut writeid: uint32_t = 0;
        let mut status: uint8_t = 0;
        let mut gotlast: uint8_t = 0;
        wrdata.chunkid = gchunkid;
        wrdata.version = gversion;
        if pipe(&raw mut wrdata.pipe as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            return 0 as uint8_t;
        }
        pfd[0 as usize].fd = sock;
        pfd[0 as usize].events = POLLIN as ::core::ffi::c_short;
        pfd[1 as usize].fd = fwdsock;
        pfd[1 as usize].events = POLLIN as ::core::ffi::c_short;
        pfd[2 as usize].fd = wrdata.pipe[0 as usize];
        pfd[2 as usize].events = POLLIN as ::core::ffi::c_short;
        wrdata.head = ::core::ptr::null_mut::<write_job>();
        wrdata.tail = &raw mut wrdata.head;
        wrdata.hddhead = ::core::ptr::null_mut::<write_job>();
        wrdata.nethead = ::core::ptr::null_mut::<write_job>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_init(
            &raw mut wrdata.lock,
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    605 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(wrdata.lock),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    605 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(wrdata.lock),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    605 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(wrdata.lock),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    605 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(wrdata.lock),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    605 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(wrdata.lock),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    605 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(wrdata.lock),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_cond_init(
            &raw mut wrdata.cond,
            ::core::ptr::null::<pthread_condattr_t>(),
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    606 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(wrdata.cond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    606 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(wrdata.cond),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    606 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(wrdata.cond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    606 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(wrdata.cond),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    606 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(wrdata.cond),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    606 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(wrdata.cond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        wrdata.condwaiting = 0 as uint8_t;
        wrdata.term = 0 as uint8_t;
        lwt_minthread_create(
            &raw mut wrthread,
            0 as uint8_t,
            Some(
                mainserv_write_thread
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            &raw mut wrdata as *mut ::core::ffi::c_void,
        );
        wrjob = ::core::ptr::null_mut::<write_job>();
        pdataleng = 4096 as uint32_t;
        if CanUseMmap != 0 {
            pdata = mmap(
                NULL,
                pdataleng as size_t,
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut uint8_t;
            if pdata.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    613 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    613 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if pdata
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    613 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    613 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                abort();
            }
        } else {
            pdata = malloc(pdataleng as size_t) as *mut uint8_t;
            if pdata.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    613 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    613 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if pdata
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    613 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    613 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_4,
                );
                abort();
            }
        }
        gotlast = 0 as uint8_t;
        loop {
            if poll(&raw mut pfd as *mut pollfd, 3 as nfds_t, SERV_TIMEOUT)
                < 0 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"write_middle: 'poll' timed out\0".as_ptr() as *const ::core::ffi::c_char,
                );
                break;
            } else {
                if pfd[0 as usize].revents as ::core::ffi::c_int & POLLERR != 0 {
                    tcpgetstatus(pfd[0 as usize].fd);
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG,
                        MFSLOG_WARNING,
                        b"write_middle: 'poll(prev)' returned POLLERR\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                if pfd[1 as usize].revents as ::core::ffi::c_int & POLLERR != 0 {
                    tcpgetstatus(pfd[1 as usize].fd);
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG,
                        MFSLOG_WARNING,
                        b"write_middle: 'poll(next)' returned POLLERR\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                if pfd[2 as usize].revents as ::core::ffi::c_int & POLLERR != 0 {
                    tcpgetstatus(pfd[2 as usize].fd);
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG,
                        MFSLOG_WARNING,
                        b"write_middle: 'poll(pipe)' returned POLLERR\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                if pfd[0 as usize].revents as ::core::ffi::c_int & POLLERR != 0
                    || pfd[1 as usize].revents as ::core::ffi::c_int & POLLERR != 0
                    || pfd[2 as usize].revents as ::core::ffi::c_int & POLLERR != 0
                {
                    break;
                }
                if pfd[0 as usize].revents as ::core::ffi::c_int & (POLLIN | POLLHUP) == POLLHUP
                    || pfd[1 as usize].revents as ::core::ffi::c_int & (POLLIN | POLLHUP) == POLLHUP
                    || pfd[2 as usize].revents as ::core::ffi::c_int & (POLLIN | POLLHUP) == POLLHUP
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_NOTICE,
                        b"write_middle: 'poll' returned POLLHUP\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    break;
                } else {
                    if pfd[0 as usize].revents as ::core::ffi::c_int & POLLIN != 0 {
                        if mainserv_toread(
                            sock,
                            &raw mut hdr as *mut uint8_t,
                            8 as uint32_t,
                            SERV_TIMEOUT as uint32_t,
                        ) != 8 as int32_t
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"write_middle: 'receive(header prev)' timed out\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            break;
                        } else {
                            rptr = &raw mut hdr as *mut uint8_t;
                            cmd = get32bit(&raw mut rptr);
                            leng = get32bit(&raw mut rptr);
                            if protover != 0 {
                                mainserv_sock_nop_del(fsn);
                            }
                            if cmd == CLTOCS_WRITE_DATA as uint32_t {
                                if leng <= 0 as uint32_t {
                                    break;
                                }
                                if leng > MaxPacketSize {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"packet too long (%u/%u) ; command:%u\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        leng,
                                        MaxPacketSize,
                                        cmd,
                                    );
                                    break;
                                } else {
                                    if CanUseMmap != 0 {
                                        wrjob = mmap(
                                            NULL,
                                            (60 as size_t)
                                                .wrapping_add(leng as size_t)
                                                .wrapping_add(8 as size_t),
                                            PROT_READ | PROT_WRITE,
                                            MAP_ANON | MAP_PRIVATE,
                                            -1 as ::core::ffi::c_int,
                                            0 as __off64_t,
                                        )
                                            as *mut write_job;
                                        if wrjob.is_null() {
                                            fprintf(
                                                stderr,
                                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                657 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"wrjob\0".as_ptr() as *const ::core::ffi::c_char,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                657 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"wrjob\0".as_ptr() as *const ::core::ffi::c_char,
                                            );
                                            abort();
                                        } else if wrjob
                                            == ::core::ptr::with_exposed_provenance_mut::<
                                                ::core::ffi::c_void,
                                            >(
                                                -1 as ::core::ffi::c_int as usize
                                            )
                                                as *mut write_job
                                        {
                                            let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                                                strerr(*__errno_location());
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                657 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"wrjob\0".as_ptr() as *const ::core::ffi::c_char,
                                                _mfs_errorstring_5,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                657 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"wrjob\0".as_ptr() as *const ::core::ffi::c_char,
                                                _mfs_errorstring_5,
                                            );
                                            abort();
                                        }
                                    } else {
                                        wrjob = malloc(
                                            (60 as size_t)
                                                .wrapping_add(leng as size_t)
                                                .wrapping_add(8 as size_t),
                                        )
                                            as *mut write_job;
                                        if wrjob.is_null() {
                                            fprintf(
                                                stderr,
                                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                657 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"wrjob\0".as_ptr() as *const ::core::ffi::c_char,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                657 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"wrjob\0".as_ptr() as *const ::core::ffi::c_char,
                                            );
                                            abort();
                                        } else if wrjob
                                            == ::core::ptr::with_exposed_provenance_mut::<
                                                ::core::ffi::c_void,
                                            >(
                                                -1 as ::core::ffi::c_int as usize
                                            )
                                                as *mut write_job
                                        {
                                            let mut _mfs_errorstring_6: *const ::core::ffi::c_char =
                                                strerr(*__errno_location());
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                657 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"wrjob\0".as_ptr() as *const ::core::ffi::c_char,
                                                _mfs_errorstring_6,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                657 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"wrjob\0".as_ptr() as *const ::core::ffi::c_char,
                                                _mfs_errorstring_6,
                                            );
                                            abort();
                                        }
                                    }
                                    (*wrjob).structsize = (60 as ::core::ffi::c_ulong)
                                        .wrapping_add(leng as ::core::ffi::c_ulong)
                                        .wrapping_add(8 as ::core::ffi::c_ulong)
                                        as uint32_t;
                                    memcpy(
                                        &raw mut (*wrjob).data as *mut uint8_t
                                            as *mut ::core::ffi::c_void,
                                        &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                                        8 as size_t,
                                    );
                                    if mainserv_toforward(
                                        sock,
                                        fwdsock,
                                        &raw mut (*wrjob).data as *mut uint8_t,
                                        leng.wrapping_add(8 as uint32_t),
                                        8 as uint32_t,
                                        0 as uint32_t,
                                        SERV_TIMEOUT as uint32_t,
                                    ) != leng.wrapping_add(8 as uint32_t) as int32_t
                                    {
                                        if CanUseMmap != 0 {
                                            munmap(
                                                wrjob as *mut ::core::ffi::c_void,
                                                (*wrjob).structsize as size_t,
                                            );
                                        } else {
                                            free(wrjob as *mut ::core::ffi::c_void);
                                        }
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_NOTICE,
                                            b"write_middle: 'forward(write data)' timed out\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        break;
                                    }
                                }
                            } else if leng > 0 as uint32_t {
                                if leng > SMALL_PACKET_SIZE as uint32_t {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"packet too long (%u/12) ; command:%u\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        leng,
                                        cmd,
                                    );
                                    break;
                                } else if mainserv_toforward(
                                    sock,
                                    fwdsock,
                                    &raw mut hdr as *mut uint8_t,
                                    leng.wrapping_add(8 as uint32_t),
                                    8 as uint32_t,
                                    0 as uint32_t,
                                    SERV_TIMEOUT as uint32_t,
                                ) != leng.wrapping_add(8 as uint32_t) as int32_t
                                {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_NOTICE,
                                        b"write_middle: 'forward(%s)' timed out\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        if cmd == CLTOCS_WRITE_FINISH as uint32_t {
                                            b"write finish\0".as_ptr() as *const ::core::ffi::c_char
                                        } else if cmd == ANTOAN_NOP as uint32_t {
                                            b"nop\0".as_ptr() as *const ::core::ffi::c_char
                                        } else {
                                            b"???\0".as_ptr() as *const ::core::ffi::c_char
                                        },
                                    );
                                    break;
                                }
                            } else if mainserv_towrite(
                                fwdsock,
                                &raw mut hdr as *mut uint8_t,
                                8 as uint32_t,
                                SERV_TIMEOUT as uint32_t,
                            ) != 8 as int32_t
                            {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_NOTICE,
                                    b"write_middle: 'send(%s)' timed out\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    if cmd == CLTOCS_WRITE_FINISH as uint32_t {
                                        b"write finish\0".as_ptr() as *const ::core::ffi::c_char
                                    } else if cmd == ANTOAN_NOP as uint32_t {
                                        b"nop\0".as_ptr() as *const ::core::ffi::c_char
                                    } else {
                                        b"???\0".as_ptr() as *const ::core::ffi::c_char
                                    },
                                );
                                break;
                            }
                            if protover != 0 {
                                mainserv_sock_nop_add(fsn);
                            }
                            if cmd == CLTOCS_WRITE_FINISH as uint32_t {
                                if leng < 12 as uint32_t {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"CLTOCS_WRITE_FINISH - wrong size (%u/12)\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        leng,
                                    );
                                    break;
                                } else {
                                    rptr = (&raw mut hdr as *mut uint8_t)
                                        .offset(8 as ::core::ffi::c_int as isize);
                                    if gchunkid != get64bit(&raw mut rptr) {
                                        packet = mainserv_create_packet(
                                            &raw mut wptr,
                                            CSTOCL_WRITE_STATUS as uint32_t,
                                            (8 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int)
                                                as uint32_t,
                                        );
                                        put64bit(&raw mut wptr, gchunkid);
                                        put32bit(&raw mut wptr, 0 as uint32_t);
                                        put8bit(&raw mut wptr, MFS_ERROR_WRONGCHUNKID as uint8_t);
                                        if protover != 0 {
                                            mainserv_sock_nop_del(sn);
                                        }
                                        mainserv_send_and_free(
                                            b"write status\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            sock,
                                            packet,
                                            (8 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int)
                                                as uint32_t,
                                        );
                                        break;
                                    } else if gversion != get32bit(&raw mut rptr) {
                                        packet = mainserv_create_packet(
                                            &raw mut wptr,
                                            CSTOCL_WRITE_STATUS as uint32_t,
                                            (8 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int)
                                                as uint32_t,
                                        );
                                        put64bit(&raw mut wptr, gchunkid);
                                        put32bit(&raw mut wptr, 0 as uint32_t);
                                        put8bit(&raw mut wptr, MFS_ERROR_WRONGCHUNKID as uint8_t);
                                        if protover != 0 {
                                            mainserv_sock_nop_del(sn);
                                        }
                                        mainserv_send_and_free(
                                            b"write status\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            sock,
                                            packet,
                                            (8 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int)
                                                as uint32_t,
                                        );
                                        break;
                                    } else {
                                        gotlast = (if wrdata.head.is_null() {
                                            2 as ::core::ffi::c_int
                                        } else {
                                            1 as ::core::ffi::c_int
                                        })
                                            as uint8_t;
                                        break;
                                    }
                                }
                            } else if cmd == CLTOCS_WRITE_DATA as uint32_t {
                                if leng
                                    < (8 as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int)
                                        as uint32_t
                                {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"CLTOCS_WRITE_DATA - wrong size (%u/24+size)\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        leng,
                                    );
                                    if CanUseMmap != 0 {
                                        munmap(
                                            wrjob as *mut ::core::ffi::c_void,
                                            (*wrjob).structsize as size_t,
                                        );
                                    } else {
                                        free(wrjob as *mut ::core::ffi::c_void);
                                    }
                                    break;
                                } else {
                                    rptr = (&raw mut (*wrjob).data as *mut uint8_t)
                                        .offset(8 as ::core::ffi::c_int as isize);
                                    (*wrjob).chunkid = get64bit(&raw mut rptr);
                                    (*wrjob).writeid = get32bit(&raw mut rptr);
                                    (*wrjob).blocknum = get16bit(&raw mut rptr);
                                    (*wrjob).offset = get16bit(&raw mut rptr);
                                    (*wrjob).size = get32bit(&raw mut rptr);
                                    if leng
                                        != ((8 as ::core::ffi::c_int
                                            + 4 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int
                                            + 4 as ::core::ffi::c_int
                                            + 4 as ::core::ffi::c_int)
                                            as uint32_t)
                                            .wrapping_add((*wrjob).size)
                                    {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_WARNING,
                                            b"CLTOCS_WRITE_DATA - wrong size (%u/24+%u)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            leng,
                                            (*wrjob).size,
                                        );
                                        if CanUseMmap != 0 {
                                            munmap(
                                                wrjob as *mut ::core::ffi::c_void,
                                                (*wrjob).structsize as size_t,
                                            );
                                        } else {
                                            free(wrjob as *mut ::core::ffi::c_void);
                                        }
                                        break;
                                    } else if gchunkid != (*wrjob).chunkid {
                                        packet = mainserv_create_packet(
                                            &raw mut wptr,
                                            CSTOCL_WRITE_STATUS as uint32_t,
                                            (8 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int)
                                                as uint32_t,
                                        );
                                        put64bit(&raw mut wptr, gchunkid);
                                        put32bit(&raw mut wptr, 0 as uint32_t);
                                        put8bit(&raw mut wptr, MFS_ERROR_WRONGCHUNKID as uint8_t);
                                        if protover != 0 {
                                            mainserv_sock_nop_del(sn);
                                        }
                                        mainserv_send_and_free(
                                            b"write status\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            sock,
                                            packet,
                                            (8 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int)
                                                as uint32_t,
                                        );
                                        if CanUseMmap != 0 {
                                            munmap(
                                                wrjob as *mut ::core::ffi::c_void,
                                                (*wrjob).structsize as size_t,
                                            );
                                        } else {
                                            free(wrjob as *mut ::core::ffi::c_void);
                                        }
                                        break;
                                    } else {
                                        (*wrjob).crcptr = rptr;
                                        (*wrjob).buff =
                                            rptr.offset(4 as ::core::ffi::c_int as isize);
                                        (*wrjob).ack = 0 as uint8_t;
                                        (*wrjob).hddstatus = 0xff as uint8_t;
                                        (*wrjob).netstatus = 0xff as uint8_t;
                                        (*wrjob).next = ::core::ptr::null_mut::<write_job>();
                                        let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                                            pthread_mutex_lock(&raw mut wrdata.lock);
                                        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
                                            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                                                && *__errno_location() != 0 as ::core::ffi::c_int
                                            {
                                                let mut _mfs_errorstring_7: *const ::core::ffi::c_char = strerr(
                                                    *__errno_location(),
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    780 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_1,
                                                    *__errno_location(),
                                                    _mfs_errorstring_7,
                                                );
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    780 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_1,
                                                    *__errno_location(),
                                                    _mfs_errorstring_7,
                                                );
                                            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                                                && *__errno_location() == 0 as ::core::ffi::c_int
                                            {
                                                let mut _mfs_errorstring_8: *const ::core::ffi::c_char = strerr(
                                                    _mfs_assert_ret_1,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    780 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_1,
                                                    _mfs_errorstring_8,
                                                );
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    780 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_1,
                                                    _mfs_errorstring_8,
                                                );
                                            } else {
                                                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char = strerr(
                                                    *__errno_location(),
                                                );
                                                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char = strerr(
                                                    _mfs_assert_ret_1,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    780 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
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
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    780 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_1,
                                                    _mfs_errorstring_ret_1,
                                                    *__errno_location(),
                                                    _mfs_errorstring_err_1,
                                                );
                                            }
                                            abort();
                                        }
                                        *wrdata.tail = wrjob;
                                        wrdata.tail = &raw mut (*wrjob).next as *mut *mut write_job;
                                        if wrdata.hddhead.is_null() {
                                            wrdata.hddhead = wrjob;
                                        }
                                        if wrdata.nethead.is_null() {
                                            wrdata.nethead = wrjob;
                                        }
                                        if wrdata.condwaiting != 0 {
                                            let mut _mfs_assert_ret_2: ::core::ffi::c_int =
                                                pthread_cond_signal(&raw mut wrdata.cond);
                                            if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
                                                if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        != 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_9: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        790 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_cond_signal(&(wrdata.cond))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_2,
                                                        *__errno_location(),
                                                        _mfs_errorstring_9,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        790 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_cond_signal(&(wrdata.cond))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_2,
                                                        *__errno_location(),
                                                        _mfs_errorstring_9,
                                                    );
                                                } else if _mfs_assert_ret_2
                                                    > 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        == 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_10: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_2,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        790 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_cond_signal(&(wrdata.cond))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_2,
                                                        _mfs_errorstring_10,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        790 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_cond_signal(&(wrdata.cond))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_2,
                                                        _mfs_errorstring_10,
                                                    );
                                                } else {
                                                    let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_2,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        790 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_cond_signal(&(wrdata.cond))\0".as_ptr()
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
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        790 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_cond_signal(&(wrdata.cond))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_2,
                                                        _mfs_errorstring_ret_2,
                                                        *__errno_location(),
                                                        _mfs_errorstring_err_2,
                                                    );
                                                }
                                                abort();
                                            }
                                            wrdata.condwaiting = 0 as uint8_t;
                                        }
                                        let mut _mfs_assert_ret_3: ::core::ffi::c_int =
                                            pthread_mutex_unlock(&raw mut wrdata.lock);
                                        if _mfs_assert_ret_3 != 0 as ::core::ffi::c_int {
                                            if _mfs_assert_ret_3 < 0 as ::core::ffi::c_int
                                                && *__errno_location() != 0 as ::core::ffi::c_int
                                            {
                                                let mut _mfs_errorstring_11: *const ::core::ffi::c_char = strerr(
                                                    *__errno_location(),
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    793 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_3,
                                                    *__errno_location(),
                                                    _mfs_errorstring_11,
                                                );
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    793 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_3,
                                                    *__errno_location(),
                                                    _mfs_errorstring_11,
                                                );
                                            } else if _mfs_assert_ret_3 > 0 as ::core::ffi::c_int
                                                && *__errno_location() == 0 as ::core::ffi::c_int
                                            {
                                                let mut _mfs_errorstring_12: *const ::core::ffi::c_char = strerr(
                                                    _mfs_assert_ret_3,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    793 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_3,
                                                    _mfs_errorstring_12,
                                                );
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    793 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_3,
                                                    _mfs_errorstring_12,
                                                );
                                            } else {
                                                let mut _mfs_errorstring_err_3: *const ::core::ffi::c_char = strerr(
                                                    *__errno_location(),
                                                );
                                                let mut _mfs_errorstring_ret_3: *const ::core::ffi::c_char = strerr(
                                                    _mfs_assert_ret_3,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    793 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
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
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    793 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                    b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_assert_ret_3,
                                                    _mfs_errorstring_ret_3,
                                                    *__errno_location(),
                                                    _mfs_errorstring_err_3,
                                                );
                                            }
                                            abort();
                                        }
                                    }
                                }
                            } else if cmd != ANTOAN_NOP as uint32_t {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"received unrecognized packet !!!\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                break;
                            }
                        }
                    }
                    if pfd[1 as usize].revents as ::core::ffi::c_int & POLLIN != 0 {
                        if mainserv_toread(fwdsock, pdata, 8 as uint32_t, SERV_TIMEOUT as uint32_t)
                            != 8 as int32_t
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"write_middle: 'receive(header next)' timed out\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            break;
                        } else {
                            rptr = pdata;
                            cmd = get32bit(&raw mut rptr);
                            leng = get32bit(&raw mut rptr);
                            if leng > 0 as uint32_t {
                                if leng > MaxPacketSize {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"packet too long (%u/%u) ; command:%u\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        leng,
                                        MaxPacketSize,
                                        cmd,
                                    );
                                    break;
                                } else {
                                    if pdataleng < leng {
                                        if !pdata.is_null() {
                                            if CanUseMmap != 0 {
                                                munmap(
                                                    pdata as *mut ::core::ffi::c_void,
                                                    pdataleng as size_t,
                                                );
                                            } else {
                                                free(pdata as *mut ::core::ffi::c_void);
                                            }
                                        }
                                        pdataleng = leng;
                                        if CanUseMmap != 0 {
                                            pdata = mmap(
                                                NULL,
                                                pdataleng as size_t,
                                                PROT_READ | PROT_WRITE,
                                                MAP_ANON | MAP_PRIVATE,
                                                -1 as ::core::ffi::c_int,
                                                0 as __off64_t,
                                            )
                                                as *mut uint8_t;
                                            if pdata.is_null() {
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - out of memory: %s is NULL\n\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    817 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"pdata\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    817 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"pdata\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                abort();
                                            } else if pdata
                                                == ::core::ptr::with_exposed_provenance_mut::<
                                                    ::core::ffi::c_void,
                                                >(
                                                    -1 as ::core::ffi::c_int as usize
                                                )
                                                    as *mut uint8_t
                                            {
                                                let mut _mfs_errorstring_13: *const ::core::ffi::c_char = strerr(
                                                    *__errno_location(),
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - mmap error on %s, error: %s\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    817 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"pdata\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_errorstring_13,
                                                );
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - mmap error on %s, error: %s\n\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    817 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"pdata\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_errorstring_13,
                                                );
                                                abort();
                                            }
                                        } else {
                                            pdata = malloc(pdataleng as size_t) as *mut uint8_t;
                                            if pdata.is_null() {
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - out of memory: %s is NULL\n\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    817 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"pdata\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    817 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"pdata\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                abort();
                                            } else if pdata
                                                == ::core::ptr::with_exposed_provenance_mut::<
                                                    ::core::ffi::c_void,
                                                >(
                                                    -1 as ::core::ffi::c_int as usize
                                                )
                                                    as *mut uint8_t
                                            {
                                                let mut _mfs_errorstring_14: *const ::core::ffi::c_char = strerr(
                                                    *__errno_location(),
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - mmap error on %s, error: %s\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    817 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"pdata\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_errorstring_14,
                                                );
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - mmap error on %s, error: %s\n\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    817 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"pdata\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_errorstring_14,
                                                );
                                                abort();
                                            }
                                        }
                                    }
                                    if mainserv_toread(
                                        fwdsock,
                                        pdata,
                                        leng,
                                        SERV_TIMEOUT as uint32_t,
                                    ) != leng as int32_t
                                    {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_NOTICE,
                                            b"write_middle: 'receive(%s)' timed out\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            if cmd == CSTOCL_WRITE_STATUS as uint32_t {
                                                b"write status\0".as_ptr()
                                                    as *const ::core::ffi::c_char
                                            } else if cmd == ANTOAN_NOP as uint32_t {
                                                b"nop\0".as_ptr() as *const ::core::ffi::c_char
                                            } else {
                                                b"???\0".as_ptr() as *const ::core::ffi::c_char
                                            },
                                        );
                                        break;
                                    }
                                }
                            }
                            if cmd == CSTOCL_WRITE_STATUS as uint32_t {
                                if leng
                                    != (8 as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int
                                        + 1 as ::core::ffi::c_int)
                                        as uint32_t
                                {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"CSTOCL_WRITE_STATUS - wrong size (%u/13)\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        leng,
                                    );
                                    break;
                                } else {
                                    rptr = pdata;
                                    chunkid = get64bit(&raw mut rptr);
                                    writeid = get32bit(&raw mut rptr);
                                    status = get8bit(&raw mut rptr);
                                    let mut _mfs_assert_ret_4: ::core::ffi::c_int =
                                        pthread_mutex_lock(&raw mut wrdata.lock);
                                    if _mfs_assert_ret_4 != 0 as ::core::ffi::c_int {
                                        if _mfs_assert_ret_4 < 0 as ::core::ffi::c_int
                                            && *__errno_location() != 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_15: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                833 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_4,
                                                *__errno_location(),
                                                _mfs_errorstring_15,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                833 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_4,
                                                *__errno_location(),
                                                _mfs_errorstring_15,
                                            );
                                        } else if _mfs_assert_ret_4 > 0 as ::core::ffi::c_int
                                            && *__errno_location() == 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_16: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_4,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                833 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_4,
                                                _mfs_errorstring_16,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                833 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_4,
                                                _mfs_errorstring_16,
                                            );
                                        } else {
                                            let mut _mfs_errorstring_err_4: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            let mut _mfs_errorstring_ret_4: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_4,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                833 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
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
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                833 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_4,
                                                _mfs_errorstring_ret_4,
                                                *__errno_location(),
                                                _mfs_errorstring_err_4,
                                            );
                                        }
                                        abort();
                                    }
                                    if writeid == 0 as uint32_t {
                                        if CanUseMmap != 0 {
                                            wrjob = mmap(
                                                NULL,
                                                ::core::mem::size_of::<write_job>(),
                                                PROT_READ | PROT_WRITE,
                                                MAP_ANON | MAP_PRIVATE,
                                                -1 as ::core::ffi::c_int,
                                                0 as __off64_t,
                                            )
                                                as *mut write_job;
                                            if wrjob.is_null() {
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - out of memory: %s is NULL\n\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    838 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"wrjob\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    838 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"wrjob\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                abort();
                                            } else if wrjob
                                                == ::core::ptr::with_exposed_provenance_mut::<
                                                    ::core::ffi::c_void,
                                                >(
                                                    -1 as ::core::ffi::c_int as usize
                                                )
                                                    as *mut write_job
                                            {
                                                let mut _mfs_errorstring_17: *const ::core::ffi::c_char = strerr(
                                                    *__errno_location(),
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - mmap error on %s, error: %s\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    838 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"wrjob\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_errorstring_17,
                                                );
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - mmap error on %s, error: %s\n\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    838 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"wrjob\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_errorstring_17,
                                                );
                                                abort();
                                            }
                                        } else {
                                            wrjob = malloc(::core::mem::size_of::<write_job>())
                                                as *mut write_job;
                                            if wrjob.is_null() {
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - out of memory: %s is NULL\n\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    838 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"wrjob\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    838 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"wrjob\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                );
                                                abort();
                                            } else if wrjob
                                                == ::core::ptr::with_exposed_provenance_mut::<
                                                    ::core::ffi::c_void,
                                                >(
                                                    -1 as ::core::ffi::c_int as usize
                                                )
                                                    as *mut write_job
                                            {
                                                let mut _mfs_errorstring_18: *const ::core::ffi::c_char = strerr(
                                                    *__errno_location(),
                                                );
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_ERR,
                                                    b"%s:%u - mmap error on %s, error: %s\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    838 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"wrjob\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_errorstring_18,
                                                );
                                                fprintf(
                                                    stderr,
                                                    b"%s:%u - mmap error on %s, error: %s\n\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0"
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    838 as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                                    b"wrjob\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    _mfs_errorstring_18,
                                                );
                                                abort();
                                            }
                                        }
                                        (*wrjob).structsize =
                                            ::core::mem::size_of::<write_job>() as uint32_t;
                                        (*wrjob).chunkid = chunkid;
                                        (*wrjob).writeid = 0 as uint32_t;
                                        (*wrjob).ack = 3 as uint8_t;
                                        (*wrjob).hddstatus = MFS_STATUS_OK as uint8_t;
                                        (*wrjob).netstatus = status;
                                        (*wrjob).next = wrdata.head as *mut write_job;
                                        wrdata.head = wrjob;
                                        if (*wrjob).next.is_null() {
                                            wrdata.tail =
                                                &raw mut (*wrjob).next as *mut *mut write_job;
                                        }
                                    } else {
                                        wrjob = wrdata.nethead;
                                        if wrjob.is_null() {
                                            let mut _mfs_assert_ret_5: ::core::ffi::c_int =
                                                pthread_mutex_unlock(&raw mut wrdata.lock);
                                            if _mfs_assert_ret_5 != 0 as ::core::ffi::c_int {
                                                if _mfs_assert_ret_5 < 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        != 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_19: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        853 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_5,
                                                        *__errno_location(),
                                                        _mfs_errorstring_19,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        853 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_5,
                                                        *__errno_location(),
                                                        _mfs_errorstring_19,
                                                    );
                                                } else if _mfs_assert_ret_5
                                                    > 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        == 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_20: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_5,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        853 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_5,
                                                        _mfs_errorstring_20,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        853 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_5,
                                                        _mfs_errorstring_20,
                                                    );
                                                } else {
                                                    let mut _mfs_errorstring_err_5: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    let mut _mfs_errorstring_ret_5: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_5,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        853 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
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
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        853 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_5,
                                                        _mfs_errorstring_ret_5,
                                                        *__errno_location(),
                                                        _mfs_errorstring_err_5,
                                                    );
                                                }
                                                abort();
                                            }
                                            break;
                                        } else if chunkid != (*wrjob).chunkid {
                                            let mut _mfs_assert_ret_6: ::core::ffi::c_int =
                                                pthread_mutex_unlock(&raw mut wrdata.lock);
                                            if _mfs_assert_ret_6 != 0 as ::core::ffi::c_int {
                                                if _mfs_assert_ret_6 < 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        != 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_21: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        857 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_6,
                                                        *__errno_location(),
                                                        _mfs_errorstring_21,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        857 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_6,
                                                        *__errno_location(),
                                                        _mfs_errorstring_21,
                                                    );
                                                } else if _mfs_assert_ret_6
                                                    > 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        == 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_22: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_6,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        857 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_6,
                                                        _mfs_errorstring_22,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        857 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_6,
                                                        _mfs_errorstring_22,
                                                    );
                                                } else {
                                                    let mut _mfs_errorstring_err_6: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    let mut _mfs_errorstring_ret_6: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_6,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        857 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
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
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        857 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_6,
                                                        _mfs_errorstring_ret_6,
                                                        *__errno_location(),
                                                        _mfs_errorstring_err_6,
                                                    );
                                                }
                                                abort();
                                            }
                                            break;
                                        } else if writeid != (*wrjob).writeid {
                                            let mut _mfs_assert_ret_7: ::core::ffi::c_int =
                                                pthread_mutex_unlock(&raw mut wrdata.lock);
                                            if _mfs_assert_ret_7 != 0 as ::core::ffi::c_int {
                                                if _mfs_assert_ret_7 < 0 as ::core::ffi::c_int
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
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        861 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_7,
                                                        *__errno_location(),
                                                        _mfs_errorstring_23,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        861 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_7,
                                                        *__errno_location(),
                                                        _mfs_errorstring_23,
                                                    );
                                                } else if _mfs_assert_ret_7
                                                    > 0 as ::core::ffi::c_int
                                                    && *__errno_location()
                                                        == 0 as ::core::ffi::c_int
                                                {
                                                    let mut _mfs_errorstring_24: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_7,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        861 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_7,
                                                        _mfs_errorstring_24,
                                                    );
                                                    fprintf(
                                                        stderr,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        861 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_7,
                                                        _mfs_errorstring_24,
                                                    );
                                                } else {
                                                    let mut _mfs_errorstring_err_7: *const ::core::ffi::c_char = strerr(
                                                        *__errno_location(),
                                                    );
                                                    let mut _mfs_errorstring_ret_7: *const ::core::ffi::c_char = strerr(
                                                        _mfs_assert_ret_7,
                                                    );
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_ERR,
                                                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        861 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
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
                                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        861 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        _mfs_assert_ret_7,
                                                        _mfs_errorstring_ret_7,
                                                        *__errno_location(),
                                                        _mfs_errorstring_err_7,
                                                    );
                                                }
                                                abort();
                                            }
                                            break;
                                        } else {
                                            (*wrjob).netstatus = status;
                                            (*wrjob).ack = ((*wrjob).ack as ::core::ffi::c_int
                                                | 2 as ::core::ffi::c_int)
                                                as uint8_t;
                                            wrdata.nethead = (*wrjob).next as *mut write_job;
                                        }
                                    }
                                    let mut _mfs_assert_ret_8: ::core::ffi::c_int =
                                        pthread_mutex_unlock(&raw mut wrdata.lock);
                                    if _mfs_assert_ret_8 != 0 as ::core::ffi::c_int {
                                        if _mfs_assert_ret_8 < 0 as ::core::ffi::c_int
                                            && *__errno_location() != 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_25: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                868 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_8,
                                                *__errno_location(),
                                                _mfs_errorstring_25,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                868 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_8,
                                                *__errno_location(),
                                                _mfs_errorstring_25,
                                            );
                                        } else if _mfs_assert_ret_8 > 0 as ::core::ffi::c_int
                                            && *__errno_location() == 0 as ::core::ffi::c_int
                                        {
                                            let mut _mfs_errorstring_26: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_8,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                868 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_8,
                                                _mfs_errorstring_26,
                                            );
                                            fprintf(
                                                stderr,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                868 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_8,
                                                _mfs_errorstring_26,
                                            );
                                        } else {
                                            let mut _mfs_errorstring_err_8: *const ::core::ffi::c_char = strerr(
                                                *__errno_location(),
                                            );
                                            let mut _mfs_errorstring_ret_8: *const ::core::ffi::c_char = strerr(
                                                _mfs_assert_ret_8,
                                            );
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_ERR,
                                                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                868 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
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
                                                b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                868 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                _mfs_assert_ret_8,
                                                _mfs_errorstring_ret_8,
                                                *__errno_location(),
                                                _mfs_errorstring_err_8,
                                            );
                                        }
                                        abort();
                                    }
                                }
                            }
                        }
                    }
                    if pfd[2 as usize].revents as ::core::ffi::c_int & POLLIN != 0 {
                        if read(
                            wrdata.pipe[0 as usize],
                            &raw mut status as *mut ::core::ffi::c_void,
                            1 as size_t,
                        ) != 1 as ssize_t
                        {
                            mfs_log(
                                MFSLOG_ERRNO_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"read pipe error\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    status = MFS_STATUS_OK as uint8_t;
                    while status as ::core::ffi::c_int == MFS_STATUS_OK {
                        let mut exitloop: uint8_t = 0;
                        let mut _mfs_assert_ret_9: ::core::ffi::c_int =
                            pthread_mutex_lock(&raw mut wrdata.lock);
                        if _mfs_assert_ret_9 != 0 as ::core::ffi::c_int {
                            if _mfs_assert_ret_9 < 0 as ::core::ffi::c_int
                                && *__errno_location() != 0 as ::core::ffi::c_int
                            {
                                let mut _mfs_errorstring_27: *const ::core::ffi::c_char =
                                    strerr(*__errno_location());
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    879 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_9,
                                    *__errno_location(),
                                    _mfs_errorstring_27,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    879 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_9,
                                    *__errno_location(),
                                    _mfs_errorstring_27,
                                );
                            } else if _mfs_assert_ret_9 > 0 as ::core::ffi::c_int
                                && *__errno_location() == 0 as ::core::ffi::c_int
                            {
                                let mut _mfs_errorstring_28: *const ::core::ffi::c_char =
                                    strerr(_mfs_assert_ret_9);
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    879 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_9,
                                    _mfs_errorstring_28,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    879 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_9,
                                    _mfs_errorstring_28,
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
                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    879 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
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
                                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    879 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    _mfs_assert_ret_9,
                                    _mfs_errorstring_ret_9,
                                    *__errno_location(),
                                    _mfs_errorstring_err_9,
                                );
                            }
                            abort();
                        }
                        if wrdata.head.is_null() {
                            let mut _mfs_assert_ret_10: ::core::ffi::c_int =
                                pthread_mutex_unlock(&raw mut wrdata.lock);
                            if _mfs_assert_ret_10 != 0 as ::core::ffi::c_int {
                                if _mfs_assert_ret_10 < 0 as ::core::ffi::c_int
                                    && *__errno_location() != 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_29: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        881 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_10,
                                        *__errno_location(),
                                        _mfs_errorstring_29,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        881 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_10,
                                        *__errno_location(),
                                        _mfs_errorstring_29,
                                    );
                                } else if _mfs_assert_ret_10 > 0 as ::core::ffi::c_int
                                    && *__errno_location() == 0 as ::core::ffi::c_int
                                {
                                    let mut _mfs_errorstring_30: *const ::core::ffi::c_char =
                                        strerr(_mfs_assert_ret_10);
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        881 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_10,
                                        _mfs_errorstring_30,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        881 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_10,
                                        _mfs_errorstring_30,
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
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        881 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
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
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        881 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        _mfs_assert_ret_10,
                                        _mfs_errorstring_ret_10,
                                        *__errno_location(),
                                        _mfs_errorstring_err_10,
                                    );
                                }
                                abort();
                            }
                            break;
                        } else {
                            exitloop = 1 as uint8_t;
                            wrjob = wrdata.head;
                            if (*wrjob).ack as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
                                && (*wrjob).hddstatus as ::core::ffi::c_int != MFS_STATUS_OK
                            {
                                status = (*wrjob).hddstatus;
                            } else if (*wrjob).ack as ::core::ffi::c_int & 2 as ::core::ffi::c_int
                                != 0
                                && (*wrjob).netstatus as ::core::ffi::c_int != MFS_STATUS_OK
                            {
                                status = (*wrjob).netstatus;
                            } else if (*wrjob).ack as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                            {
                                status = MFS_STATUS_OK as uint8_t;
                                exitloop = 0 as uint8_t;
                            }
                            if exitloop != 0 {
                                let mut _mfs_assert_ret_11: ::core::ffi::c_int =
                                    pthread_mutex_unlock(&raw mut wrdata.lock);
                                if _mfs_assert_ret_11 != 0 as ::core::ffi::c_int {
                                    if _mfs_assert_ret_11 < 0 as ::core::ffi::c_int
                                        && *__errno_location() != 0 as ::core::ffi::c_int
                                    {
                                        let mut _mfs_errorstring_31: *const ::core::ffi::c_char =
                                            strerr(*__errno_location());
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            895 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_11,
                                            *__errno_location(),
                                            _mfs_errorstring_31,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            895 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_11,
                                            *__errno_location(),
                                            _mfs_errorstring_31,
                                        );
                                    } else if _mfs_assert_ret_11 > 0 as ::core::ffi::c_int
                                        && *__errno_location() == 0 as ::core::ffi::c_int
                                    {
                                        let mut _mfs_errorstring_32: *const ::core::ffi::c_char =
                                            strerr(_mfs_assert_ret_11);
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            895 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_11,
                                            _mfs_errorstring_32,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            895 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_11,
                                            _mfs_errorstring_32,
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
                                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            895 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
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
                                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            895 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_11,
                                            _mfs_errorstring_ret_11,
                                            *__errno_location(),
                                            _mfs_errorstring_err_11,
                                        );
                                    }
                                    abort();
                                }
                                break;
                            } else {
                                chunkid = (*wrjob).chunkid;
                                writeid = (*wrjob).writeid;
                                wrdata.head = (*wrjob).next as *mut write_job;
                                if wrdata.head.is_null() {
                                    wrdata.tail = &raw mut wrdata.head;
                                }
                                if CanUseMmap != 0 {
                                    munmap(
                                        wrjob as *mut ::core::ffi::c_void,
                                        (*wrjob).structsize as size_t,
                                    );
                                } else {
                                    free(wrjob as *mut ::core::ffi::c_void);
                                }
                                let mut _mfs_assert_ret_12: ::core::ffi::c_int =
                                    pthread_mutex_unlock(&raw mut wrdata.lock);
                                if _mfs_assert_ret_12 != 0 as ::core::ffi::c_int {
                                    if _mfs_assert_ret_12 < 0 as ::core::ffi::c_int
                                        && *__errno_location() != 0 as ::core::ffi::c_int
                                    {
                                        let mut _mfs_errorstring_33: *const ::core::ffi::c_char =
                                            strerr(*__errno_location());
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            905 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_12,
                                            *__errno_location(),
                                            _mfs_errorstring_33,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            905 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_12,
                                            *__errno_location(),
                                            _mfs_errorstring_33,
                                        );
                                    } else if _mfs_assert_ret_12 > 0 as ::core::ffi::c_int
                                        && *__errno_location() == 0 as ::core::ffi::c_int
                                    {
                                        let mut _mfs_errorstring_34: *const ::core::ffi::c_char =
                                            strerr(_mfs_assert_ret_12);
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            905 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_12,
                                            _mfs_errorstring_34,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            905 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_12,
                                            _mfs_errorstring_34,
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
                                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            905 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
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
                                            b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            905 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            _mfs_assert_ret_12,
                                            _mfs_errorstring_ret_12,
                                            *__errno_location(),
                                            _mfs_errorstring_err_12,
                                        );
                                    }
                                    abort();
                                }
                                packet = mainserv_create_packet(
                                    &raw mut wptr,
                                    CSTOCL_WRITE_STATUS as uint32_t,
                                    (8 as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int
                                        + 1 as ::core::ffi::c_int)
                                        as uint32_t,
                                );
                                put64bit(&raw mut wptr, chunkid);
                                put32bit(&raw mut wptr, writeid);
                                put8bit(&raw mut wptr, status);
                                if protover != 0 {
                                    mainserv_sock_nop_del(sn);
                                }
                                if mainserv_send_and_free(
                                    b"write status\0".as_ptr() as *const ::core::ffi::c_char,
                                    sock,
                                    packet,
                                    (8 as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int
                                        + 1 as ::core::ffi::c_int)
                                        as uint32_t,
                                ) as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    status = MFS_ERROR_DISCONNECTED as uint8_t;
                                }
                                if protover != 0 {
                                    mainserv_sock_nop_add(sn);
                                }
                            }
                        }
                    }
                    if (*sn).error as ::core::ffi::c_int != 0
                        || (*fsn).error as ::core::ffi::c_int != 0
                    {
                        status = MFS_ERROR_DISCONNECTED as uint8_t;
                    }
                    if status as ::core::ffi::c_int != MFS_STATUS_OK {
                        break;
                    }
                    if pfd[1 as usize].revents as ::core::ffi::c_int & POLLHUP != 0 {
                        packet = mainserv_create_packet(
                            &raw mut wptr,
                            CSTOCL_WRITE_STATUS as uint32_t,
                            (8 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as uint32_t,
                        );
                        put64bit(&raw mut wptr, gchunkid);
                        put32bit(&raw mut wptr, 0 as uint32_t);
                        put8bit(&raw mut wptr, MFS_ERROR_DISCONNECTED as uint8_t);
                        if protover != 0 {
                            mainserv_sock_nop_del(sn);
                        }
                        mainserv_send_and_free(
                            b"write status\0".as_ptr() as *const ::core::ffi::c_char,
                            sock,
                            packet,
                            (8 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as uint32_t,
                        );
                        break;
                    } else if pfd[0 as usize].revents as ::core::ffi::c_int & POLLHUP != 0 {
                        break;
                    }
                }
            }
        }
        let mut _mfs_assert_ret_13: ::core::ffi::c_int = pthread_mutex_lock(&raw mut wrdata.lock);
        if _mfs_assert_ret_13 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_13 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_35: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    941 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_13,
                    *__errno_location(),
                    _mfs_errorstring_35,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    941 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_13,
                    *__errno_location(),
                    _mfs_errorstring_35,
                );
            } else if _mfs_assert_ret_13 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_36: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_13);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    941 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_13,
                    _mfs_errorstring_36,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    941 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_13,
                    _mfs_errorstring_36,
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
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    941 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_13,
                    _mfs_errorstring_ret_13,
                    *__errno_location(),
                    _mfs_errorstring_err_13,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    941 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(wrdata.lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_13,
                    _mfs_errorstring_ret_13,
                    *__errno_location(),
                    _mfs_errorstring_err_13,
                );
            }
            abort();
        }
        wrdata.term = 1 as uint8_t;
        if wrdata.condwaiting != 0 {
            let mut _mfs_assert_ret_14: ::core::ffi::c_int =
                pthread_cond_signal(&raw mut wrdata.cond);
            if _mfs_assert_ret_14 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_14 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_37: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        944 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(wrdata.cond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_14,
                        *__errno_location(),
                        _mfs_errorstring_37,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        944 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(wrdata.cond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_14,
                        *__errno_location(),
                        _mfs_errorstring_37,
                    );
                } else if _mfs_assert_ret_14 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_38: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_14);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        944 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(wrdata.cond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_14,
                        _mfs_errorstring_38,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        944 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(wrdata.cond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_14,
                        _mfs_errorstring_38,
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        944 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(wrdata.cond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        944 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(wrdata.cond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_14,
                        _mfs_errorstring_ret_14,
                        *__errno_location(),
                        _mfs_errorstring_err_14,
                    );
                }
                abort();
            }
            wrdata.condwaiting = 0 as uint8_t;
        }
        let mut _mfs_assert_ret_15: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut wrdata.lock);
        if _mfs_assert_ret_15 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_15 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_39: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    947 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_15,
                    *__errno_location(),
                    _mfs_errorstring_39,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    947 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_15,
                    *__errno_location(),
                    _mfs_errorstring_39,
                );
            } else if _mfs_assert_ret_15 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_40: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_15);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    947 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_15,
                    _mfs_errorstring_40,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    947 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_15,
                    _mfs_errorstring_40,
                );
            } else {
                let mut _mfs_errorstring_err_15: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_15: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_15);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    947 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_15,
                    _mfs_errorstring_ret_15,
                    *__errno_location(),
                    _mfs_errorstring_err_15,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    947 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(wrdata.lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_15,
                    _mfs_errorstring_ret_15,
                    *__errno_location(),
                    _mfs_errorstring_err_15,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_16: ::core::ffi::c_int = pthread_join(
            wrthread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        if _mfs_assert_ret_16 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_16 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_41: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    948 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_join(wrthread,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_16,
                    *__errno_location(),
                    _mfs_errorstring_41,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    948 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_join(wrthread,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_16,
                    *__errno_location(),
                    _mfs_errorstring_41,
                );
            } else if _mfs_assert_ret_16 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_42: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_16);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    948 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_join(wrthread,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_16,
                    _mfs_errorstring_42,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    948 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_join(wrthread,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_16,
                    _mfs_errorstring_42,
                );
            } else {
                let mut _mfs_errorstring_err_16: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_16: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_16);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    948 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_join(wrthread,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_16,
                    _mfs_errorstring_ret_16,
                    *__errno_location(),
                    _mfs_errorstring_err_16,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    948 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_join(wrthread,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_16,
                    _mfs_errorstring_ret_16,
                    *__errno_location(),
                    _mfs_errorstring_err_16,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_17: ::core::ffi::c_int =
            pthread_mutex_destroy(&raw mut wrdata.lock);
        if _mfs_assert_ret_17 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_17 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_43: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    949 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&(wrdata.lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_17,
                    *__errno_location(),
                    _mfs_errorstring_43,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    949 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&(wrdata.lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_17,
                    *__errno_location(),
                    _mfs_errorstring_43,
                );
            } else if _mfs_assert_ret_17 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_44: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_17);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    949 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&(wrdata.lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_17,
                    _mfs_errorstring_44,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    949 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&(wrdata.lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_17,
                    _mfs_errorstring_44,
                );
            } else {
                let mut _mfs_errorstring_err_17: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_17: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_17);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    949 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&(wrdata.lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_17,
                    _mfs_errorstring_ret_17,
                    *__errno_location(),
                    _mfs_errorstring_err_17,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    949 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&(wrdata.lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_17,
                    _mfs_errorstring_ret_17,
                    *__errno_location(),
                    _mfs_errorstring_err_17,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_18: ::core::ffi::c_int = pthread_cond_destroy(&raw mut wrdata.cond);
        if _mfs_assert_ret_18 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_18 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_45: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    950 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(wrdata.cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_18,
                    *__errno_location(),
                    _mfs_errorstring_45,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    950 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(wrdata.cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_18,
                    *__errno_location(),
                    _mfs_errorstring_45,
                );
            } else if _mfs_assert_ret_18 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_46: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_18);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    950 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(wrdata.cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_18,
                    _mfs_errorstring_46,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    950 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(wrdata.cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_18,
                    _mfs_errorstring_46,
                );
            } else {
                let mut _mfs_errorstring_err_18: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_18: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_18);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    950 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(wrdata.cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_18,
                    _mfs_errorstring_ret_18,
                    *__errno_location(),
                    _mfs_errorstring_err_18,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    950 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(wrdata.cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_18,
                    _mfs_errorstring_ret_18,
                    *__errno_location(),
                    _mfs_errorstring_err_18,
                );
            }
            abort();
        }
        loop {
            wrjob = wrdata.head;
            if wrjob.is_null() {
                break;
            }
            wrdata.head = (*wrjob).next as *mut write_job;
            if CanUseMmap != 0 {
                munmap(
                    wrjob as *mut ::core::ffi::c_void,
                    (*wrjob).structsize as size_t,
                );
            } else {
                free(wrjob as *mut ::core::ffi::c_void);
            }
        }
        if !pdata.is_null() {
            if CanUseMmap != 0 {
                munmap(pdata as *mut ::core::ffi::c_void, pdataleng as size_t);
            } else {
                free(pdata as *mut ::core::ffi::c_void);
            }
        }
        close(wrdata.pipe[0 as usize]);
        close(wrdata.pipe[1 as usize]);
        return gotlast;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainserv_write_last(
    mut sock: ::core::ffi::c_int,
    mut gchunkid: uint64_t,
    mut gversion: uint32_t,
    mut protover: uint8_t,
    mut sn: *mut sock_nops,
) -> uint8_t {
    unsafe {
        let mut packet: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut pdata: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut pdataleng: uint32_t = 0;
        let mut cmd: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut writeid: uint32_t = 0;
        let mut size: uint32_t = 0;
        let mut blocknum: uint16_t = 0;
        let mut offset: uint16_t = 0;
        let mut rstat: uint8_t = 0;
        let mut status: uint8_t = 0;
        status = 0 as uint8_t;
        writeid = 0 as uint32_t;
        packet = mainserv_create_packet(
            &raw mut wptr,
            CSTOCL_WRITE_STATUS as uint32_t,
            (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint32_t,
        );
        put64bit(&raw mut wptr, gchunkid);
        put32bit(&raw mut wptr, 0 as uint32_t);
        put8bit(&raw mut wptr, MFS_STATUS_OK as uint8_t);
        if mainserv_send_and_free(
            b"write status\0".as_ptr() as *const ::core::ffi::c_char,
            sock,
            packet,
            (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint32_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return 0 as uint8_t;
        }
        pdataleng = (65536 as ::core::ffi::c_int + 4096 as ::core::ffi::c_int) as uint32_t;
        if CanUseMmap != 0 {
            pdata = mmap(
                NULL,
                pdataleng as size_t,
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut uint8_t;
            if pdata.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    992 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    992 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if pdata
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    992 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    992 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        } else {
            pdata = malloc(pdataleng as size_t) as *mut uint8_t;
            if pdata.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    992 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    992 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if pdata
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    992 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    992 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
        }
        rstat = 0 as uint8_t;
        loop {
            if mainserv_toread(sock, pdata, 8 as uint32_t, SERV_TIMEOUT as uint32_t) != 8 as int32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"write_last: 'receive(header prev)' timed out\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                break;
            } else {
                rptr = pdata;
                cmd = get32bit(&raw mut rptr);
                leng = get32bit(&raw mut rptr);
                if leng > 0 as uint32_t {
                    if leng > MaxPacketSize {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"packet too long (%u/%u) ; command:%u\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            leng,
                            MaxPacketSize,
                            cmd,
                        );
                        break;
                    } else {
                        if pdataleng < leng.wrapping_add(8 as uint32_t) {
                            if !pdata.is_null() {
                                if CanUseMmap != 0 {
                                    munmap(pdata as *mut ::core::ffi::c_void, pdataleng as size_t);
                                } else {
                                    free(pdata as *mut ::core::ffi::c_void);
                                }
                            }
                            pdataleng = leng.wrapping_add(8 as uint32_t);
                            pdataleng = pdataleng.wrapping_add(0xfff as uint32_t);
                            pdataleng &= !(0xfff as ::core::ffi::c_int) as uint32_t;
                            if CanUseMmap != 0 {
                                pdata = mmap(
                                    NULL,
                                    pdataleng as size_t,
                                    PROT_READ | PROT_WRITE,
                                    MAP_ANON | MAP_PRIVATE,
                                    -1 as ::core::ffi::c_int,
                                    0 as __off64_t,
                                ) as *mut uint8_t;
                                if pdata.is_null() {
                                    fprintf(
                                        stderr,
                                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1015 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1015 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    abort();
                                } else if pdata
                                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                        -1 as ::core::ffi::c_int as usize,
                                    ) as *mut uint8_t
                                {
                                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1015 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                                        _mfs_errorstring_1,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1015 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                                        _mfs_errorstring_1,
                                    );
                                    abort();
                                }
                            } else {
                                pdata = malloc(pdataleng as size_t) as *mut uint8_t;
                                if pdata.is_null() {
                                    fprintf(
                                        stderr,
                                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1015 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1015 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    abort();
                                } else if pdata
                                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                        -1 as ::core::ffi::c_int as usize,
                                    ) as *mut uint8_t
                                {
                                    let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1015 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                                        _mfs_errorstring_2,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfschunkserver/mainserv.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        1015 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"pdata\0".as_ptr() as *const ::core::ffi::c_char,
                                        _mfs_errorstring_2,
                                    );
                                    abort();
                                }
                            }
                        }
                        if mainserv_toread(
                            sock,
                            pdata.offset(8 as ::core::ffi::c_int as isize),
                            leng,
                            SERV_TIMEOUT as uint32_t,
                        ) != leng as int32_t
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"write_last: 'receive(%s)' timed out\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                if cmd == CLTOCS_WRITE_FINISH as uint32_t {
                                    b"write finish\0".as_ptr() as *const ::core::ffi::c_char
                                } else if cmd == CLTOCS_WRITE_DATA as uint32_t {
                                    b"write data\0".as_ptr() as *const ::core::ffi::c_char
                                } else if cmd == ANTOAN_NOP as uint32_t {
                                    b"nop\0".as_ptr() as *const ::core::ffi::c_char
                                } else {
                                    b"???\0".as_ptr() as *const ::core::ffi::c_char
                                },
                            );
                            break;
                        }
                    }
                }
                if cmd == CLTOCS_WRITE_FINISH as uint32_t {
                    rptr = pdata.offset(8 as ::core::ffi::c_int as isize);
                    chunkid = get64bit(&raw mut rptr);
                    version = get32bit(&raw mut rptr);
                    if gchunkid != chunkid || gversion != version {
                        rstat = 1 as uint8_t;
                        status = MFS_ERROR_WRONGCHUNKID as uint8_t;
                        break;
                    } else {
                        rstat = 1 as uint8_t;
                        status = MFS_STATUS_OK as uint8_t;
                        break;
                    }
                } else {
                    if cmd != CLTOCS_WRITE_DATA as uint32_t {
                        continue;
                    }
                    if leng
                        < (8 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int) as uint32_t
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOCS_WRITE_DATA - wrong size (%u/24+size)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            leng,
                        );
                        break;
                    } else {
                        rptr = pdata.offset(8 as ::core::ffi::c_int as isize);
                        chunkid = get64bit(&raw mut rptr);
                        writeid = get32bit(&raw mut rptr);
                        blocknum = get16bit(&raw mut rptr);
                        offset = get16bit(&raw mut rptr);
                        size = get32bit(&raw mut rptr);
                        if leng
                            != ((8 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int)
                                as uint32_t)
                                .wrapping_add(size)
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"CLTOCS_WRITE_DATA - wrong size (%u/24+%u)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                leng,
                                size,
                            );
                            break;
                        } else if gchunkid != chunkid {
                            rstat = 1 as uint8_t;
                            status = MFS_ERROR_WRONGCHUNKID as uint8_t;
                            break;
                        } else {
                            status = hdd_write(
                                gchunkid,
                                gversion,
                                blocknum,
                                rptr.offset(4 as ::core::ffi::c_int as isize),
                                offset as uint32_t,
                                size,
                                rptr,
                            ) as uint8_t;
                            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                                rstat = 1 as uint8_t;
                                break;
                            } else {
                                packet = mainserv_create_packet(
                                    &raw mut wptr,
                                    CSTOCL_WRITE_STATUS as uint32_t,
                                    (8 as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int
                                        + 1 as ::core::ffi::c_int)
                                        as uint32_t,
                                );
                                put64bit(&raw mut wptr, chunkid);
                                put32bit(&raw mut wptr, writeid);
                                put8bit(&raw mut wptr, MFS_STATUS_OK as uint8_t);
                                if protover != 0 {
                                    mainserv_sock_nop_del(sn);
                                }
                                if mainserv_send_and_free(
                                    b"write status\0".as_ptr() as *const ::core::ffi::c_char,
                                    sock,
                                    packet,
                                    (8 as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int
                                        + 1 as ::core::ffi::c_int)
                                        as uint32_t,
                                ) as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                                if protover != 0 {
                                    mainserv_sock_nop_add(sn);
                                }
                            }
                        }
                    }
                }
            }
        }
        if !pdata.is_null() {
            if CanUseMmap != 0 {
                munmap(pdata as *mut ::core::ffi::c_void, pdataleng as size_t);
            } else {
                free(pdata as *mut ::core::ffi::c_void);
            }
        }
        if rstat as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if status as ::core::ffi::c_int == MFS_STATUS_OK {
                return 1 as uint8_t;
            } else {
                packet = mainserv_create_packet(
                    &raw mut wptr,
                    CSTOCL_WRITE_STATUS as uint32_t,
                    (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as uint32_t,
                );
                put64bit(&raw mut wptr, gchunkid);
                put32bit(&raw mut wptr, writeid);
                put8bit(&raw mut wptr, status);
                if protover != 0 {
                    mainserv_sock_nop_del(sn);
                }
                return mainserv_send_and_free(
                    b"write status\0".as_ptr() as *const ::core::ffi::c_char,
                    sock,
                    packet,
                    (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as uint32_t,
                );
            }
        } else {
            return 0 as uint8_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainserv_write(
    mut sock: ::core::ffi::c_int,
    mut data: *const uint8_t,
    mut length: uint32_t,
) -> uint8_t {
    unsafe {
        let mut packet: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut fwdsock: ::core::ffi::c_int = 0;
        let mut fwdip: uint32_t = 0;
        let mut fwdport: uint16_t = 0;
        let mut protover: uint8_t = 0;
        let mut gchunkid: uint64_t = 0;
        let mut gversion: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut sn: sock_nops = sock_nops {
            sock: 0,
            error: 0,
            monotonic_utime: 0,
            bytesleft: 0,
            next: ::core::ptr::null_mut::<sock_nops>(),
            prev: ::core::ptr::null_mut::<*mut sock_nops>(),
        };
        let mut fsn: sock_nops = sock_nops {
            sock: 0,
            error: 0,
            monotonic_utime: 0,
            bytesleft: 0,
            next: ::core::ptr::null_mut::<sock_nops>(),
            prev: ::core::ptr::null_mut::<*mut sock_nops>(),
        };
        fwdport = 0 as uint16_t;
        fwdip = 0 as uint32_t;
        if length & 1 as uint32_t != 0 {
            protover = get8bit(&raw mut data);
            if protover as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOCS_WRITE - wrong protover (%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    protover as ::core::ffi::c_int,
                );
                return 0 as uint8_t;
            }
            if length < 13 as uint32_t
                || length
                    .wrapping_sub(13 as uint32_t)
                    .wrapping_rem(6 as uint32_t)
                    != 0 as uint32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOCS_WRITE - wrong size (%u/13+N*6)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                return 0 as uint8_t;
            }
        } else {
            if length < 12 as uint32_t
                || length
                    .wrapping_sub(12 as uint32_t)
                    .wrapping_rem(6 as uint32_t)
                    != 0 as uint32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOCS_WRITE - wrong size (%u/12+N*6)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                return 0 as uint8_t;
            }
            protover = 0 as uint8_t;
        }
        if protover != 0 {
            mainserv_sock_nop_init(&raw mut sn, sock);
        }
        gchunkid = get64bit(&raw mut data);
        gversion = get32bit(&raw mut data);
        if length
            > (if protover as ::core::ffi::c_int != 0 {
                13 as ::core::ffi::c_int
            } else {
                12 as ::core::ffi::c_int
            }) as uint32_t
        {
            fwdip = get32bit(&raw mut data);
            fwdport = get16bit(&raw mut data);
            fwdsock = -1 as ::core::ffi::c_int;
            if protover != 0 {
                mainserv_sock_nop_add(&raw mut sn);
            }
            i = 0 as uint32_t;
            while i < CONNECT_RETRIES as uint32_t && fwdsock < 0 as ::core::ffi::c_int {
                if i == 0 as uint32_t {
                    fwdsock = conncache_get(fwdip, fwdport);
                }
                if fwdsock < 0 as ::core::ffi::c_int {
                    fwdsock = mainserv_connect(
                        fwdip,
                        fwdport,
                        (if i.wrapping_rem(2 as uint32_t) != 0 {
                            300 as ::core::ffi::c_int
                                * ((1 as ::core::ffi::c_int) << (i >> 1 as ::core::ffi::c_int))
                        } else {
                            200 as ::core::ffi::c_int
                                * ((1 as ::core::ffi::c_int) << (i >> 1 as ::core::ffi::c_int))
                        }) as uint32_t,
                    );
                }
                if fwdsock >= 0 as ::core::ffi::c_int {
                    packet = mainserv_create_packet(
                        &raw mut wptr,
                        CLTOCS_WRITE as uint32_t,
                        length.wrapping_sub(6 as uint32_t),
                    );
                    if protover != 0 {
                        put8bit(&raw mut wptr, protover);
                    }
                    put64bit(&raw mut wptr, gchunkid);
                    put32bit(&raw mut wptr, gversion);
                    if protover != 0 {
                        memcpy(
                            wptr as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            length
                                .wrapping_sub(13 as uint32_t)
                                .wrapping_sub(6 as uint32_t) as size_t,
                        );
                    } else {
                        memcpy(
                            wptr as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            length
                                .wrapping_sub(12 as uint32_t)
                                .wrapping_sub(6 as uint32_t) as size_t,
                        );
                    }
                    if mainserv_send_and_free(
                        b"write init\0".as_ptr() as *const ::core::ffi::c_char,
                        fwdsock,
                        packet,
                        length.wrapping_sub(6 as uint32_t),
                    ) != 0
                    {
                        break;
                    }
                    tcpclose(fwdsock);
                    fwdsock = -1 as ::core::ffi::c_int;
                }
                i = i.wrapping_add(1);
            }
            if fwdsock < 0 as ::core::ffi::c_int {
                packet = mainserv_create_packet(
                    &raw mut wptr,
                    CSTOCL_WRITE_STATUS as uint32_t,
                    (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as uint32_t,
                );
                put64bit(&raw mut wptr, gchunkid);
                put32bit(&raw mut wptr, 0 as uint32_t);
                put8bit(&raw mut wptr, MFS_ERROR_CANTCONNECT as uint8_t);
                if protover != 0 {
                    mainserv_sock_nop_del(&raw mut sn);
                }
                return mainserv_send_and_free(
                    b"write status\0".as_ptr() as *const ::core::ffi::c_char,
                    sock,
                    packet,
                    (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as uint32_t,
                );
            }
        } else {
            fwdsock = -1 as ::core::ffi::c_int;
        }
        if protover != 0 {
            if fwdsock >= 0 as ::core::ffi::c_int {
                mainserv_sock_nop_init(&raw mut fsn, fwdsock);
                mainserv_sock_nop_add(&raw mut fsn);
            }
        }
        status = hdd_open(gchunkid, gversion) as uint8_t;
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            if fwdsock >= 0 as ::core::ffi::c_int {
                tcpclose(fwdsock);
            }
            packet = mainserv_create_packet(
                &raw mut wptr,
                CSTOCL_WRITE_STATUS as uint32_t,
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as uint32_t,
            );
            put64bit(&raw mut wptr, gchunkid);
            put32bit(&raw mut wptr, 0 as uint32_t);
            put8bit(&raw mut wptr, status);
            if protover != 0 {
                mainserv_sock_nop_del(&raw mut sn);
                if fwdsock >= 0 as ::core::ffi::c_int {
                    mainserv_sock_nop_del(&raw mut fsn);
                }
            }
            return mainserv_send_and_free(
                b"write status\0".as_ptr() as *const ::core::ffi::c_char,
                sock,
                packet,
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as uint32_t,
            );
        }
        if fwdsock >= 0 as ::core::ffi::c_int {
            ret = mainserv_write_middle(
                sock,
                fwdsock,
                gchunkid,
                gversion,
                protover,
                &raw mut sn,
                &raw mut fsn,
            );
            if protover != 0 {
                mainserv_sock_nop_del(&raw mut fsn);
            }
            if (ret as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
                || protover as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                tcpclose(fwdsock);
            } else {
                conncache_insert(fwdip, fwdport, fwdsock);
            }
        } else {
            ret = mainserv_write_last(sock, gchunkid, gversion, protover, &raw mut sn);
        }
        hdd_close(gchunkid, 0 as uint8_t);
        if protover != 0 {
            mainserv_sock_nop_del(&raw mut sn);
        }
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut stats_hlopw,
            1 as uint32_t,
        );
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainserv_term() {
    unsafe {
        conncache_term();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainserv_init() -> ::core::ffi::c_int {
    unsafe {
        let mut rnthread: pthread_t = 0;
        CanUseMmap = cfg_getuint8(
            b"CAN_USE_MMAP\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint8_t,
        );
        if conncache_init(250 as uint32_t) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        main_destruct_register_fname(
            Some(mainserv_term as unsafe extern "C" fn() -> ()),
            b"mainserv_term\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sock_nops_head = ::core::ptr::null_mut::<sock_nops>();
        sock_nops_tail = &raw mut sock_nops_head;
        if pthread_mutex_init(
            &raw mut sock_nops_lock,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if lwt_minthread_create(
            &raw mut rnthread,
            1 as uint8_t,
            Some(
                mainserv_sock_nop_sender
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            NULL,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
