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
    unsafe fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn hdd_open(chunkid: uint64_t, version: uint32_t) -> ::core::ffi::c_int;
    unsafe fn hdd_close(chunkid: uint64_t, forcefsync: uint8_t) -> ::core::ffi::c_int;
    unsafe fn hdd_write(
        chunkid: uint64_t,
        version: uint32_t,
        blocknum: uint16_t,
        buffer: *const uint8_t,
        offset: uint32_t,
        size: uint32_t,
        crcbuff: *const uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn hdd_rep_setversion(chunkid: uint64_t, version: uint32_t) -> ::core::ffi::c_int;
    unsafe fn hdd_chunkop(
        chunkid: uint64_t,
        version: uint32_t,
        newversion: uint32_t,
        copychunkid: uint64_t,
        copyversion: uint32_t,
        length: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpsocket() -> ::core::ffi::c_int;
    unsafe fn tcpnonblock(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpgetstatus(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnodelay(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnumconnect(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn mycrc32_combine(crc1: uint32_t, crc2: uint32_t, leng2: uint32_t) -> uint32_t;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn monotonic_useconds() -> uint64_t;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type int32_t = i32;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
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
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2Rust_Unnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2Rust_Unnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2Rust_Unnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2Rust_Unnamed = 0;
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
pub type repmodeenum = ::core::ffi::c_uint;
pub const JOIN: repmodeenum = 3;
pub const RECOVER: repmodeenum = 2;
pub const SPLIT: repmodeenum = 1;
pub const SIMPLE: repmodeenum = 0;
pub type replication = _replication;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _replication {
    pub chunkid: uint64_t,
    pub version: uint32_t,
    pub xorbuff: *mut uint8_t,
    pub created: uint8_t,
    pub opened: uint8_t,
    pub needsreadrequest: uint8_t,
    pub srccnt: uint8_t,
    pub repsources: *mut repsrc,
}
pub type repsrc = _repsrc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _repsrc {
    pub sock: ::core::ffi::c_int,
    pub mode: modetypeenum,
    pub hdrbuff: [uint8_t; 8],
    pub packet: *mut uint8_t,
    pub startptr: *mut uint8_t,
    pub bytesleft: uint32_t,
    pub chunkid: uint64_t,
    pub version: uint32_t,
    pub blocks: uint16_t,
    pub ip: uint32_t,
    pub port: uint16_t,
    pub datapackets: [*mut uint8_t; 4],
    pub crcsums: [uint32_t; 4],
}
pub type modetypeenum = _modetypenum;
pub type _modetypenum = ::core::ffi::c_uint;
pub const DATA: _modetypenum = 4;
pub const HEADER: _modetypenum = 3;
pub const CONNECTED: _modetypenum = 2;
pub const CONNECTING: _modetypenum = 1;
pub const IDLE: _modetypenum = 0;
pub const STATE_CONNECTED: _data_source_state = 2;
pub type data_source_state = _data_source_state;
pub type _data_source_state = ::core::ffi::c_uint;
pub const STATE_ERROR: _data_source_state = 3;
pub const STATE_CONNECTING: _data_source_state = 1;
pub const STATE_IDLE: _data_source_state = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const MFSBLOCKSIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_NOTDONE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFS_ERROR_WRONGVERSION: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const MFS_ERROR_WRONGSIZE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const MFS_ERROR_WRONGOFFSET: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const MFS_ERROR_CANTCONNECT: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const MFS_ERROR_WRONGCHUNKID: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const MFS_ERROR_DISCONNECTED: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const MFS_ERROR_ETIMEDOUT: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const MFS_ERROR_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAX_EC_PARTS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLTOCS_READ: ::core::ffi::c_int = PROTO_BASE + 200 as ::core::ffi::c_int;
pub const CSTOCL_READ_STATUS: ::core::ffi::c_int = PROTO_BASE + 201 as ::core::ffi::c_int;
pub const CSTOCL_READ_DATA: ::core::ffi::c_int = PROTO_BASE + 202 as ::core::ffi::c_int;
pub const ANTOCS_GET_CHUNK_BLOCKS: ::core::ffi::c_int = PROTO_BASE + 250 as ::core::ffi::c_int;
pub const CSTOAN_CHUNK_BLOCKS: ::core::ffi::c_int = PROTO_BASE + 251 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
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
pub const MAX_REP_TIME_SEC: ::core::ffi::c_int = 150 as ::core::ffi::c_int;
pub const PROGRESS_CHECK: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const SENDMSECTO: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const RECVMSECTO: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const CONNMAXTRY: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const REP_RETRY_CNT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SRCCNTMAX: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MAX_RECV_PACKET_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int + MFSBLOCKSIZE;
static mut stats_repl: uint32_t = 0 as uint32_t;
static mut stats_bytesin: uint64_t = 0 as uint64_t;
static mut stats_bytesout: uint64_t = 0 as uint64_t;
static mut statslock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn replicator_stats(
    mut bin: *mut uint64_t,
    mut bout: *mut uint64_t,
    mut repl: *mut uint32_t,
) {
    unsafe {
        pthread_mutex_lock(&raw mut statslock);
        *bin = stats_bytesin;
        *bout = stats_bytesout;
        *repl = stats_repl;
        stats_repl = 0 as uint32_t;
        stats_bytesin = 0 as uint64_t;
        stats_bytesout = 0 as uint64_t;
        pthread_mutex_unlock(&raw mut statslock);
    }
}
#[inline]
unsafe extern "C" fn replicator_bytesin(mut bytes: uint64_t) {
    unsafe {
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut statslock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        stats_bytesin = stats_bytesin.wrapping_add(bytes);
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut statslock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
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
unsafe extern "C" fn replicator_bytesout(mut bytes: uint64_t) {
    unsafe {
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut statslock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        stats_bytesout = stats_bytesout.wrapping_add(bytes);
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut statslock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&statslock)\0".as_ptr() as *const ::core::ffi::c_char,
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
unsafe extern "C" fn xordata(mut dst: *mut uint8_t, mut src: *const uint8_t, mut leng: uint32_t) {
    unsafe {
        let mut dst4: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut src4: *const uint32_t = ::core::ptr::null::<uint32_t>();
        if dst.expose_provenance() as ::core::ffi::c_ulong & 3 as ::core::ffi::c_ulong
            == src.expose_provenance() as ::core::ffi::c_ulong & 3 as ::core::ffi::c_ulong
        {
            while leng != 0
                && src.expose_provenance() as ::core::ffi::c_ulong & 3 as ::core::ffi::c_ulong != 0
            {
                let c2rust_fresh0 = src;
                src = src.offset(1);
                let c2rust_fresh1 = dst;
                dst = dst.offset(1);
                let c2rust_lvalue_ptr = &raw mut *c2rust_fresh1;
                *c2rust_lvalue_ptr = (*c2rust_lvalue_ptr as ::core::ffi::c_int
                    ^ *c2rust_fresh0 as ::core::ffi::c_int)
                    as uint8_t;
                leng = leng.wrapping_sub(1);
            }
            dst4 = dst as *mut uint32_t;
            src4 = src as *const uint32_t;
            while leng >= 32 as uint32_t {
                let c2rust_fresh2 = src4;
                src4 = src4.offset(1);
                let c2rust_fresh3 = dst4;
                dst4 = dst4.offset(1);
                *c2rust_fresh3 ^= *c2rust_fresh2;
                let c2rust_fresh4 = src4;
                src4 = src4.offset(1);
                let c2rust_fresh5 = dst4;
                dst4 = dst4.offset(1);
                *c2rust_fresh5 ^= *c2rust_fresh4;
                let c2rust_fresh6 = src4;
                src4 = src4.offset(1);
                let c2rust_fresh7 = dst4;
                dst4 = dst4.offset(1);
                *c2rust_fresh7 ^= *c2rust_fresh6;
                let c2rust_fresh8 = src4;
                src4 = src4.offset(1);
                let c2rust_fresh9 = dst4;
                dst4 = dst4.offset(1);
                *c2rust_fresh9 ^= *c2rust_fresh8;
                let c2rust_fresh10 = src4;
                src4 = src4.offset(1);
                let c2rust_fresh11 = dst4;
                dst4 = dst4.offset(1);
                *c2rust_fresh11 ^= *c2rust_fresh10;
                let c2rust_fresh12 = src4;
                src4 = src4.offset(1);
                let c2rust_fresh13 = dst4;
                dst4 = dst4.offset(1);
                *c2rust_fresh13 ^= *c2rust_fresh12;
                let c2rust_fresh14 = src4;
                src4 = src4.offset(1);
                let c2rust_fresh15 = dst4;
                dst4 = dst4.offset(1);
                *c2rust_fresh15 ^= *c2rust_fresh14;
                let c2rust_fresh16 = src4;
                src4 = src4.offset(1);
                let c2rust_fresh17 = dst4;
                dst4 = dst4.offset(1);
                *c2rust_fresh17 ^= *c2rust_fresh16;
                leng = leng.wrapping_sub(32 as uint32_t);
            }
            while leng >= 4 as uint32_t {
                let c2rust_fresh18 = src4;
                src4 = src4.offset(1);
                let c2rust_fresh19 = dst4;
                dst4 = dst4.offset(1);
                *c2rust_fresh19 ^= *c2rust_fresh18;
                leng = leng.wrapping_sub(4 as uint32_t);
            }
            src = src4 as *const uint8_t;
            dst = dst4 as *mut uint8_t;
            if leng != 0 {
                loop {
                    let c2rust_fresh20 = src;
                    src = src.offset(1);
                    let c2rust_fresh21 = dst;
                    dst = dst.offset(1);
                    let c2rust_lvalue_ptr_0 = &raw mut *c2rust_fresh21;
                    *c2rust_lvalue_ptr_0 = (*c2rust_lvalue_ptr_0 as ::core::ffi::c_int
                        ^ *c2rust_fresh20 as ::core::ffi::c_int)
                        as uint8_t;
                    leng = leng.wrapping_sub(1);
                    if leng == 0 {
                        break;
                    }
                }
            }
        } else {
            while leng >= 8 as uint32_t {
                let c2rust_fresh22 = src;
                src = src.offset(1);
                let c2rust_fresh23 = dst;
                dst = dst.offset(1);
                let c2rust_lvalue_ptr_1 = &raw mut *c2rust_fresh23;
                *c2rust_lvalue_ptr_1 = (*c2rust_lvalue_ptr_1 as ::core::ffi::c_int
                    ^ *c2rust_fresh22 as ::core::ffi::c_int)
                    as uint8_t;
                let c2rust_fresh24 = src;
                src = src.offset(1);
                let c2rust_fresh25 = dst;
                dst = dst.offset(1);
                let c2rust_lvalue_ptr_2 = &raw mut *c2rust_fresh25;
                *c2rust_lvalue_ptr_2 = (*c2rust_lvalue_ptr_2 as ::core::ffi::c_int
                    ^ *c2rust_fresh24 as ::core::ffi::c_int)
                    as uint8_t;
                let c2rust_fresh26 = src;
                src = src.offset(1);
                let c2rust_fresh27 = dst;
                dst = dst.offset(1);
                let c2rust_lvalue_ptr_3 = &raw mut *c2rust_fresh27;
                *c2rust_lvalue_ptr_3 = (*c2rust_lvalue_ptr_3 as ::core::ffi::c_int
                    ^ *c2rust_fresh26 as ::core::ffi::c_int)
                    as uint8_t;
                let c2rust_fresh28 = src;
                src = src.offset(1);
                let c2rust_fresh29 = dst;
                dst = dst.offset(1);
                let c2rust_lvalue_ptr_4 = &raw mut *c2rust_fresh29;
                *c2rust_lvalue_ptr_4 = (*c2rust_lvalue_ptr_4 as ::core::ffi::c_int
                    ^ *c2rust_fresh28 as ::core::ffi::c_int)
                    as uint8_t;
                let c2rust_fresh30 = src;
                src = src.offset(1);
                let c2rust_fresh31 = dst;
                dst = dst.offset(1);
                let c2rust_lvalue_ptr_5 = &raw mut *c2rust_fresh31;
                *c2rust_lvalue_ptr_5 = (*c2rust_lvalue_ptr_5 as ::core::ffi::c_int
                    ^ *c2rust_fresh30 as ::core::ffi::c_int)
                    as uint8_t;
                let c2rust_fresh32 = src;
                src = src.offset(1);
                let c2rust_fresh33 = dst;
                dst = dst.offset(1);
                let c2rust_lvalue_ptr_6 = &raw mut *c2rust_fresh33;
                *c2rust_lvalue_ptr_6 = (*c2rust_lvalue_ptr_6 as ::core::ffi::c_int
                    ^ *c2rust_fresh32 as ::core::ffi::c_int)
                    as uint8_t;
                let c2rust_fresh34 = src;
                src = src.offset(1);
                let c2rust_fresh35 = dst;
                dst = dst.offset(1);
                let c2rust_lvalue_ptr_7 = &raw mut *c2rust_fresh35;
                *c2rust_lvalue_ptr_7 = (*c2rust_lvalue_ptr_7 as ::core::ffi::c_int
                    ^ *c2rust_fresh34 as ::core::ffi::c_int)
                    as uint8_t;
                let c2rust_fresh36 = src;
                src = src.offset(1);
                let c2rust_fresh37 = dst;
                dst = dst.offset(1);
                let c2rust_lvalue_ptr_8 = &raw mut *c2rust_fresh37;
                *c2rust_lvalue_ptr_8 = (*c2rust_lvalue_ptr_8 as ::core::ffi::c_int
                    ^ *c2rust_fresh36 as ::core::ffi::c_int)
                    as uint8_t;
                leng = leng.wrapping_sub(8 as uint32_t);
            }
            if leng > 0 as uint32_t {
                loop {
                    let c2rust_fresh38 = src;
                    src = src.offset(1);
                    let c2rust_fresh39 = dst;
                    dst = dst.offset(1);
                    let c2rust_lvalue_ptr_9 = &raw mut *c2rust_fresh39;
                    *c2rust_lvalue_ptr_9 = (*c2rust_lvalue_ptr_9 as ::core::ffi::c_int
                        ^ *c2rust_fresh38 as ::core::ffi::c_int)
                        as uint8_t;
                    leng = leng.wrapping_sub(1);
                    if leng == 0 {
                        break;
                    }
                }
            }
        };
    }
}
unsafe extern "C" fn rep_read(mut rs: *mut repsrc) -> ::core::ffi::c_int {
    unsafe {
        let mut i: int32_t = 0;
        let mut r#type: uint32_t = 0;
        let mut size: uint32_t = 0;
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        while (*rs).bytesleft > 0 as uint32_t {
            i = read(
                (*rs).sock,
                (*rs).startptr as *mut ::core::ffi::c_void,
                (*rs).bytesleft as size_t,
            ) as int32_t;
            if i == 0 as int32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"replicator: connection lost (read)\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            if i < 0 as int32_t {
                if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"replicator: read error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                return 0 as ::core::ffi::c_int;
            }
            replicator_bytesin(i as uint64_t);
            (*rs).startptr = (*rs).startptr.offset(i as isize);
            (*rs).bytesleft = (*rs).bytesleft.wrapping_sub(i as uint32_t);
            if (*rs).bytesleft > 0 as uint32_t {
                return 0 as ::core::ffi::c_int;
            }
            if (*rs).mode as ::core::ffi::c_uint
                == HEADER as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                ptr = &raw mut (*rs).hdrbuff as *mut uint8_t;
                r#type = get32bit(&raw mut ptr);
                size = get32bit(&raw mut ptr);
                if r#type == ANTOAN_NOP as uint32_t && size == 0 as uint32_t {
                    (*rs).startptr = &raw mut (*rs).hdrbuff as *mut uint8_t;
                    (*rs).bytesleft = 8 as uint32_t;
                    return 0 as ::core::ffi::c_int;
                }
                if !(*rs).packet.is_null() {
                    free((*rs).packet as *mut ::core::ffi::c_void);
                    (*rs).packet = ::core::ptr::null_mut::<uint8_t>();
                }
                if size > 0 as uint32_t {
                    if size > MAX_RECV_PACKET_SIZE as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"replicator: packet too long (%u/%u) ; command:%u\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            size,
                            MAX_RECV_PACKET_SIZE,
                            r#type,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    (*rs).packet = malloc(size as size_t) as *mut uint8_t;
                    if (*rs).packet.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"rs->packet\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"rs->packet\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if (*rs).packet
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut uint8_t
                    {
                        let mut _mfs_errorstring: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"rs->packet\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"rs->packet\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        abort();
                    }
                    (*rs).startptr = (*rs).packet;
                }
                (*rs).bytesleft = size;
                (*rs).mode = DATA;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn rep_receive_all_packets(
    mut r: *mut replication,
    mut msecto: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint8_t = 0;
        let mut desc: uint8_t = 0;
        let mut st: uint64_t = 0;
        let mut now: uint64_t = 0;
        let mut msec: uint32_t = 0;
        let mut pfd: [pollfd; 8] = [pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        }; 8];
        let mut noptime: uint64_t = 0;
        let mut nopbuff: [uint8_t; 8] = [0; 8];
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if (*r).srccnt as ::core::ffi::c_int > SRCCNTMAX {
            return -1 as ::core::ffi::c_int;
        }
        wptr = &raw mut nopbuff as *mut uint8_t;
        put32bit(&raw mut wptr, ANTOAN_NOP as uint32_t);
        put32bit(&raw mut wptr, 0 as uint32_t);
        st = monotonic_useconds();
        noptime = st;
        loop {
            desc = 0 as uint8_t;
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < (*r).srccnt as ::core::ffi::c_int {
                if (*(*r).repsources.offset(i as isize)).bytesleft > 0 as uint32_t {
                    pfd[desc as usize].fd = (*(*r).repsources.offset(i as isize)).sock;
                    pfd[desc as usize].events = POLLIN as ::core::ffi::c_short;
                    pfd[desc as usize].revents = 0 as ::core::ffi::c_short;
                    desc = desc.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            if desc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            msec = monotonic_useconds()
                .wrapping_sub(st)
                .wrapping_div(1000 as uint64_t) as uint32_t;
            if msec >= msecto {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"replicator: receive timed out\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            msec = msecto.wrapping_sub(msec);
            if msec > 1000 as uint32_t {
                msec = 1000 as uint32_t;
            }
            if poll(
                &raw mut pfd as *mut pollfd,
                desc as nfds_t,
                msecto.wrapping_sub(msec) as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                if *__errno_location() != EINTR
                    && (*__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK)
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"replicator: poll error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else {
                desc = 0 as uint8_t;
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < (*r).srccnt as ::core::ffi::c_int {
                    if (*(*r).repsources.offset(i as isize)).bytesleft > 0 as uint32_t {
                        if pfd[desc as usize].revents as ::core::ffi::c_int & POLLERR != 0 {
                            tcpgetstatus(pfd[desc as usize].fd);
                            mfs_log(
                                MFSLOG_ERRNO_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"replicator: socket error (pollerr/receive)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        if pfd[desc as usize].revents as ::core::ffi::c_int & POLLIN != 0 {
                            if rep_read((*r).repsources.offset(i as ::core::ffi::c_int as isize))
                                < 0 as ::core::ffi::c_int
                            {
                                return -1 as ::core::ffi::c_int;
                            }
                        }
                        desc = desc.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
                now = monotonic_useconds();
                if now > noptime.wrapping_add(1000000 as uint64_t) {
                    noptime = now;
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < (*r).srccnt as ::core::ffi::c_int {
                        if write(
                            (*(*r).repsources.offset(i as isize)).sock,
                            &raw mut nopbuff as *mut uint8_t as *const ::core::ffi::c_void,
                            8 as size_t,
                        ) != 8 as ssize_t
                        {
                            return -1 as ::core::ffi::c_int;
                        }
                        i = i.wrapping_add(1);
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn rep_create_packet(
    mut rs: *mut repsrc,
    mut r#type: uint32_t,
    mut size: uint32_t,
) -> *mut uint8_t {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if !(*rs).packet.is_null() {
            free((*rs).packet as *mut ::core::ffi::c_void);
        }
        (*rs).packet = malloc(size.wrapping_add(8 as uint32_t) as size_t) as *mut uint8_t;
        if (*rs).packet.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                311 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"rs->packet\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                311 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"rs->packet\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if (*rs).packet
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                311 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"rs->packet\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                311 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"rs->packet\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        ptr = (*rs).packet;
        put32bit(&raw mut ptr, r#type);
        put32bit(&raw mut ptr, size);
        (*rs).startptr = (*rs).packet;
        (*rs).bytesleft = (8 as uint32_t).wrapping_add(size);
        return ptr;
    }
}
unsafe extern "C" fn rep_create_read_request(
    mut rs: *mut repsrc,
    mut offset: uint32_t,
    mut bsize: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        ptr = rep_create_packet(rs, CLTOCS_READ as uint32_t, 20 as uint32_t);
        put64bit(&raw mut ptr, (*rs).chunkid);
        put32bit(&raw mut ptr, (*rs).version);
        put32bit(&raw mut ptr, offset);
        put32bit(&raw mut ptr, bsize);
    }
}
unsafe extern "C" fn rep_write(mut rs: *mut repsrc) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = write(
            (*rs).sock,
            (*rs).startptr as *const ::core::ffi::c_void,
            (*rs).bytesleft as size_t,
        ) as ::core::ffi::c_int;
        if i == 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"replicator: connection lost (write)\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if i < 0 as ::core::ffi::c_int {
            if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"replicator: write error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        replicator_bytesout(i as uint64_t);
        (*rs).startptr = (*rs).startptr.offset(i as isize);
        (*rs).bytesleft = (*rs).bytesleft.wrapping_sub(i as uint32_t);
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn rep_send_all_packets(
    mut r: *mut replication,
    mut msecto: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint8_t = 0;
        let mut desc: uint8_t = 0;
        let mut st: uint64_t = 0;
        let mut msec: uint32_t = 0;
        let mut pfd: [pollfd; 8] = [pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        }; 8];
        if (*r).srccnt as ::core::ffi::c_int > SRCCNTMAX {
            return -1 as ::core::ffi::c_int;
        }
        st = monotonic_useconds();
        loop {
            desc = 0 as uint8_t;
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < (*r).srccnt as ::core::ffi::c_int {
                if (*(*r).repsources.offset(i as isize)).bytesleft > 0 as uint32_t {
                    pfd[desc as usize].fd = (*(*r).repsources.offset(i as isize)).sock;
                    pfd[desc as usize].events = POLLOUT as ::core::ffi::c_short;
                    pfd[desc as usize].revents = 0 as ::core::ffi::c_short;
                    desc = desc.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            if desc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            msec = monotonic_useconds()
                .wrapping_sub(st)
                .wrapping_div(1000 as uint64_t) as uint32_t;
            if msec >= msecto {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"replicator: send timed out\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            if poll(
                &raw mut pfd as *mut pollfd,
                desc as nfds_t,
                msecto.wrapping_sub(msec) as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                if *__errno_location() != EINTR
                    && (*__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK)
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"replicator: poll error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else {
                desc = 0 as uint8_t;
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < (*r).srccnt as ::core::ffi::c_int {
                    if (*(*r).repsources.offset(i as isize)).bytesleft > 0 as uint32_t {
                        if pfd[desc as usize].revents as ::core::ffi::c_int & POLLERR != 0 {
                            tcpgetstatus(pfd[desc as usize].fd);
                            mfs_log(
                                MFSLOG_ERRNO_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"replicator: socket error (pollerr/send)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        if pfd[desc as usize].revents as ::core::ffi::c_int & POLLOUT != 0 {
                            if rep_write((*r).repsources.offset(i as ::core::ffi::c_int as isize))
                                < 0 as ::core::ffi::c_int
                            {
                                return -1 as ::core::ffi::c_int;
                            }
                        }
                        desc = desc.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
            }
        }
    }
}
unsafe extern "C" fn rep_concurrent_connect(mut r: *mut replication) -> ::core::ffi::c_int {
    unsafe {
        let mut s: ::core::ffi::c_int = 0;
        let mut cres: ::core::ffi::c_int = 0;
        let mut connect_state: data_source_state = STATE_IDLE;
        let mut pfd: [pollfd; 8] = [pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        }; 8];
        let mut desc: uint8_t = 0;
        let mut finished: uint8_t = 0;
        let mut cnt: uint8_t = 0;
        let mut i: uint8_t = 0;
        if (*r).srccnt as ::core::ffi::c_int > SRCCNTMAX {
            return -1 as ::core::ffi::c_int;
        }
        connect_state = STATE_CONNECTING;
        cnt = 0 as uint8_t;
        while (cnt as ::core::ffi::c_int) < CONNMAXTRY
            && connect_state as ::core::ffi::c_uint
                == STATE_CONNECTING as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut newconnection: uint8_t = 0;
            newconnection = 0 as uint8_t;
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < (*r).srccnt as ::core::ffi::c_int {
                if (*(*r).repsources.offset(i as isize)).mode as ::core::ffi::c_uint
                    == IDLE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    s = tcpsocket();
                    if s < 0 as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"replicator: socket error\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        connect_state = STATE_ERROR;
                        break;
                    } else if tcpnonblock(s) < 0 as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"replicator: nonblock error\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        connect_state = STATE_ERROR;
                        break;
                    } else {
                        (*(*r).repsources.offset(i as isize)).sock = s;
                        cres = tcpnumconnect(
                            s,
                            (*(*r).repsources.offset(i as isize)).ip,
                            (*(*r).repsources.offset(i as isize)).port,
                        );
                        if cres < 0 as ::core::ffi::c_int {
                            tcpclose(s);
                            (*(*r).repsources.offset(i as isize)).sock = -1 as ::core::ffi::c_int;
                            (*(*r).repsources.offset(i as isize)).mode = IDLE;
                            newconnection = 1 as uint8_t;
                        } else if cres == 0 as ::core::ffi::c_int {
                            (*(*r).repsources.offset(i as isize)).mode = CONNECTED;
                        } else {
                            (*(*r).repsources.offset(i as isize)).mode = CONNECTING;
                        }
                    }
                }
                i = i.wrapping_add(1);
            }
            if connect_state as ::core::ffi::c_uint
                == STATE_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            desc = 0 as uint8_t;
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < (*r).srccnt as ::core::ffi::c_int {
                if (*(*r).repsources.offset(i as isize)).mode as ::core::ffi::c_uint
                    == CONNECTING as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    pfd[desc as usize].fd = (*(*r).repsources.offset(i as isize)).sock;
                    pfd[desc as usize].events = POLLOUT as ::core::ffi::c_short;
                    pfd[desc as usize].revents = 0 as ::core::ffi::c_short;
                    desc = desc.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            if desc as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                if poll(
                    &raw mut pfd as *mut pollfd,
                    desc as nfds_t,
                    if cnt as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0 {
                        300 as ::core::ffi::c_int
                            * ((1 as ::core::ffi::c_int)
                                << (cnt as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                    } else {
                        200 as ::core::ffi::c_int
                            * ((1 as ::core::ffi::c_int)
                                << (cnt as ::core::ffi::c_int >> 1 as ::core::ffi::c_int))
                    },
                ) < 0 as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"replicator: poll error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        strerr(*__errno_location()),
                    );
                    connect_state = STATE_ERROR;
                    break;
                } else {
                    finished = 0 as uint8_t;
                    desc = 0 as uint8_t;
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < (*r).srccnt as ::core::ffi::c_int {
                        if (*(*r).repsources.offset(i as isize)).mode as ::core::ffi::c_uint
                            == CONNECTING as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            if pfd[desc as usize].revents as ::core::ffi::c_int
                                & (POLLOUT | POLLERR | POLLHUP)
                                != 0
                            {
                                if tcpgetstatus((*(*r).repsources.offset(i as isize)).sock) != 0 {
                                    tcpclose((*(*r).repsources.offset(i as isize)).sock);
                                    (*(*r).repsources.offset(i as isize)).sock =
                                        -1 as ::core::ffi::c_int;
                                    (*(*r).repsources.offset(i as isize)).mode = IDLE;
                                    newconnection = 1 as uint8_t;
                                } else {
                                    (*(*r).repsources.offset(i as isize)).mode = CONNECTED;
                                }
                                finished = 1 as uint8_t;
                            }
                            desc = desc.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                    }
                    if finished as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        desc = 0 as uint8_t;
                        i = 0 as uint8_t;
                        while (i as ::core::ffi::c_int) < (*r).srccnt as ::core::ffi::c_int {
                            if (*(*r).repsources.offset(i as isize)).mode as ::core::ffi::c_uint
                                == CONNECTING as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                tcpclose((*(*r).repsources.offset(i as isize)).sock);
                                (*(*r).repsources.offset(i as isize)).sock =
                                    -1 as ::core::ffi::c_int;
                                (*(*r).repsources.offset(i as isize)).mode = IDLE;
                                newconnection = 1 as uint8_t;
                                desc = desc.wrapping_add(1);
                            }
                            i = i.wrapping_add(1);
                        }
                    }
                }
            }
            connect_state = STATE_CONNECTED;
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < (*r).srccnt as ::core::ffi::c_int {
                if (*(*r).repsources.offset(i as isize)).mode as ::core::ffi::c_uint
                    == CONNECTING as ::core::ffi::c_int as ::core::ffi::c_uint
                    || (*(*r).repsources.offset(i as isize)).mode as ::core::ffi::c_uint
                        == IDLE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    connect_state = STATE_CONNECTING;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            if newconnection != 0 {
                cnt = cnt.wrapping_add(1);
            }
        }
        return if connect_state as ::core::ffi::c_uint
            != STATE_CONNECTED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            -1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn rep_reconnect(mut r: *mut replication) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut g: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*r).srccnt as ::core::ffi::c_int {
            if (*(*r).repsources.offset(i as isize)).sock >= 0 as ::core::ffi::c_int {
                tcpclose((*(*r).repsources.offset(i as isize)).sock);
                (*(*r).repsources.offset(i as isize)).sock = -1 as ::core::ffi::c_int;
            }
            if !(*(*r).repsources.offset(i as isize)).packet.is_null() {
                free((*(*r).repsources.offset(i as isize)).packet as *mut ::core::ffi::c_void);
                (*(*r).repsources.offset(i as isize)).packet = ::core::ptr::null_mut::<uint8_t>();
            }
            g = 0 as ::core::ffi::c_int;
            while g < 4 as ::core::ffi::c_int {
                if !(*(*r).repsources.offset(i as isize)).datapackets[g as usize].is_null() {
                    free(
                        (*(*r).repsources.offset(i as isize)).datapackets[g as usize]
                            as *mut ::core::ffi::c_void,
                    );
                    (*(*r).repsources.offset(i as isize)).datapackets[g as usize] =
                        ::core::ptr::null_mut::<uint8_t>();
                }
                g += 1;
            }
            (*(*r).repsources.offset(i as isize)).mode = IDLE;
            i += 1;
        }
        if rep_concurrent_connect(r) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*r).srccnt as ::core::ffi::c_int {
            tcpnodelay((*(*r).repsources.offset(i as isize)).sock);
            i += 1;
        }
        (*r).needsreadrequest = 1 as uint8_t;
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn rep_cleanup(mut r: *mut replication) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut g: ::core::ffi::c_int = 0;
        if (*r).opened != 0 {
            hdd_close((*r).chunkid, 0 as uint8_t);
        }
        if (*r).created != 0 {
            hdd_chunkop(
                (*r).chunkid,
                0 as uint32_t,
                0 as uint32_t,
                0 as uint64_t,
                0 as uint32_t,
                10 as uint32_t,
            );
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*r).srccnt as ::core::ffi::c_int {
            if (*(*r).repsources.offset(i as isize)).sock >= 0 as ::core::ffi::c_int {
                tcpclose((*(*r).repsources.offset(i as isize)).sock);
            }
            if !(*(*r).repsources.offset(i as isize)).packet.is_null() {
                free((*(*r).repsources.offset(i as isize)).packet as *mut ::core::ffi::c_void);
            }
            g = 0 as ::core::ffi::c_int;
            while g < 4 as ::core::ffi::c_int {
                if !(*(*r).repsources.offset(i as isize)).datapackets[g as usize].is_null() {
                    free(
                        (*(*r).repsources.offset(i as isize)).datapackets[g as usize]
                            as *mut ::core::ffi::c_void,
                    );
                }
                g += 1;
            }
            i += 1;
        }
        if !(*r).repsources.is_null() {
            free((*r).repsources as *mut ::core::ffi::c_void);
        }
        if !(*r).xorbuff.is_null() {
            free((*r).xorbuff as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn replicate(
    mut rmode: repmodeenum,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut partno: uint8_t,
    mut parts: uint8_t,
    mut srcip: *const uint32_t,
    mut srcport: *const uint16_t,
    mut srcchunkid: *const uint64_t,
) -> uint8_t {
    unsafe {
        let mut status: uint8_t = 0;
        let mut i: uint8_t = 0;
        let mut nonzero: uint8_t = 0;
        let mut r: replication = replication {
            chunkid: 0,
            version: 0,
            xorbuff: ::core::ptr::null_mut::<uint8_t>(),
            created: 0,
            opened: 0,
            needsreadrequest: 0,
            srccnt: 0,
            repsources: ::core::ptr::null_mut::<repsrc>(),
        };
        let mut srccnt: uint8_t = 0;
        let mut lastsrccnt: uint8_t = 0;
        let mut readsrccnt: uint8_t = 0;
        let mut bg: uint8_t = 0;
        let mut blockgroup: uint8_t = 0;
        let mut b: uint16_t = 0;
        let mut blocks: uint16_t = 0;
        let mut trycnt: uint8_t = 0;
        let mut reptotalto: ::core::ffi::c_double = 0.;
        let mut progcheck: ::core::ffi::c_double = 0.;
        let mut start: ::core::ffi::c_double = 0.;
        let mut now: ::core::ffi::c_double = 0.;
        let mut xcrc: uint32_t = 0;
        let mut zcrc: uint32_t = 0;
        let mut bind: uint32_t = 0;
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        start = monotonic_seconds();
        progcheck = start + PROGRESS_CHECK as ::core::ffi::c_double;
        reptotalto = start + MAX_REP_TIME_SEC as ::core::ffi::c_double;
        if rmode as ::core::ffi::c_uint == SIMPLE as ::core::ffi::c_int as ::core::ffi::c_uint
            || rmode as ::core::ffi::c_uint == SPLIT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            srccnt = 1 as uint8_t;
        } else {
            srccnt = parts;
        }
        if rmode as ::core::ffi::c_uint != SPLIT as ::core::ffi::c_int as ::core::ffi::c_uint {
            partno = 0 as uint8_t;
        } else if partno as ::core::ffi::c_int >= parts as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if rmode as ::core::ffi::c_uint == RECOVER as ::core::ffi::c_int as ::core::ffi::c_uint {
            zcrc = mycrc32_combine(
                0 as uint32_t ^ 0xffffffff as uint32_t,
                0xffffffff as uint32_t,
                0x10000 as uint32_t,
            );
        } else {
            zcrc = 0 as uint32_t;
        }
        if srccnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || srccnt as ::core::ffi::c_int > MAX_EC_PARTS
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        pthread_mutex_lock(&raw mut statslock);
        stats_repl = stats_repl.wrapping_add(1);
        pthread_mutex_unlock(&raw mut statslock);
        r.chunkid = chunkid;
        r.version = version;
        r.srccnt = 0 as uint8_t;
        r.created = 0 as uint8_t;
        r.opened = 0 as uint8_t;
        r.repsources =
            malloc(::core::mem::size_of::<repsrc>().wrapping_mul(srccnt as size_t)) as *mut repsrc;
        if r.repsources.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                654 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r.repsources\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                654 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r.repsources\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if r.repsources
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut repsrc
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                654 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r.repsources\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                654 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r.repsources\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        if rmode as ::core::ffi::c_uint == RECOVER as ::core::ffi::c_int as ::core::ffi::c_uint {
            r.xorbuff = malloc((MFSBLOCKSIZE + 4 as ::core::ffi::c_int) as size_t) as *mut uint8_t;
            if r.xorbuff.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    657 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r.xorbuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    657 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r.xorbuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if r.xorbuff
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
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    657 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r.xorbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/replicator.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    657 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r.xorbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
        } else {
            r.xorbuff = ::core::ptr::null_mut::<uint8_t>();
        }
        status = hdd_chunkop(
            chunkid,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint64_t,
            0 as uint32_t,
            11 as uint32_t,
        ) as uint8_t;
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"replicator: create status: %s\0".as_ptr() as *const ::core::ffi::c_char,
                mfsstrerr(status),
            );
            rep_cleanup(&raw mut r);
            return status;
        }
        r.created = 1 as uint8_t;
        r.srccnt = srccnt;
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < srccnt as ::core::ffi::c_int {
            (*r.repsources.offset(i as isize)).mode = IDLE;
            (*r.repsources.offset(i as isize)).chunkid = *srcchunkid.offset(i as isize);
            (*r.repsources.offset(i as isize)).version = version;
            (*r.repsources.offset(i as isize)).ip = *srcip.offset(i as isize);
            (*r.repsources.offset(i as isize)).port = *srcport.offset(i as isize);
            (*r.repsources.offset(i as isize)).sock = -1 as ::core::ffi::c_int;
            (*r.repsources.offset(i as isize)).packet = ::core::ptr::null_mut::<uint8_t>();
            bg = 0 as uint8_t;
            while (bg as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
                (*r.repsources.offset(i as isize)).datapackets[bg as usize] =
                    ::core::ptr::null_mut::<uint8_t>();
                bg = bg.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if rep_concurrent_connect(&raw mut r) < 0 as ::core::ffi::c_int {
            rep_cleanup(&raw mut r);
            return MFS_ERROR_CANTCONNECT as uint8_t;
        }
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < srccnt as ::core::ffi::c_int {
            tcpnodelay((*r.repsources.offset(i as isize)).sock);
            i = i.wrapping_add(1);
        }
        status = hdd_open(chunkid, 0 as uint32_t) as uint8_t;
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"replicator: open status: %s\0".as_ptr() as *const ::core::ffi::c_char,
                mfsstrerr(status),
            );
            rep_cleanup(&raw mut r);
            return status;
        }
        r.opened = 1 as uint8_t;
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < srccnt as ::core::ffi::c_int {
            wptr = rep_create_packet(
                r.repsources.offset(i as ::core::ffi::c_int as isize),
                ANTOCS_GET_CHUNK_BLOCKS as uint32_t,
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(&raw mut wptr, (*r.repsources.offset(i as isize)).chunkid);
            put32bit(&raw mut wptr, (*r.repsources.offset(i as isize)).version);
            i = i.wrapping_add(1);
        }
        if rep_send_all_packets(&raw mut r, SENDMSECTO as uint32_t) < 0 as ::core::ffi::c_int {
            rep_cleanup(&raw mut r);
            return MFS_ERROR_DISCONNECTED as uint8_t;
        }
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < srccnt as ::core::ffi::c_int {
            (*r.repsources.offset(i as isize)).mode = HEADER;
            (*r.repsources.offset(i as isize)).startptr =
                &raw mut (*r.repsources.offset(i as isize)).hdrbuff as *mut uint8_t;
            (*r.repsources.offset(i as isize)).bytesleft = 8 as uint32_t;
            i = i.wrapping_add(1);
        }
        if rep_receive_all_packets(&raw mut r, RECVMSECTO as uint32_t) < 0 as ::core::ffi::c_int {
            rep_cleanup(&raw mut r);
            return MFS_ERROR_DISCONNECTED as uint8_t;
        }
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < srccnt as ::core::ffi::c_int {
            let mut r#type: uint32_t = 0;
            let mut size: uint32_t = 0;
            let mut pchid: uint64_t = 0;
            let mut pver: uint32_t = 0;
            let mut pblocks: uint16_t = 0;
            let mut pstatus: uint8_t = 0;
            let mut ip: uint32_t = 0;
            rptr = &raw mut (*r.repsources.offset(i as isize)).hdrbuff as *mut uint8_t;
            r#type = get32bit(&raw mut rptr);
            size = get32bit(&raw mut rptr);
            rptr = (*r.repsources.offset(i as isize)).packet;
            ip = (*r.repsources.offset(i as isize)).ip;
            if rptr.is_null() || r#type != CSTOAN_CHUNK_BLOCKS as uint32_t || size != 15 as uint32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"replicator,get # of blocks: got wrong answer (type:0x%08X/size:0x%08X) from (%u.%u.%u.%u:%hu)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    r#type,
                    size,
                    ip >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                    ip >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                    ip >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                    ip & 0xff as uint32_t,
                    (*r.repsources.offset(i as isize)).port as ::core::ffi::c_int,
                );
                rep_cleanup(&raw mut r);
                return MFS_ERROR_DISCONNECTED as uint8_t;
            }
            pchid = get64bit(&raw mut rptr);
            pver = get32bit(&raw mut rptr);
            pblocks = get16bit(&raw mut rptr);
            pstatus = get8bit(&raw mut rptr);
            if pchid != (*r.repsources.offset(i as isize)).chunkid {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"replicator,get # of blocks: got wrong answer (chunk_status:chunkid:%lX/%lX) from (%u.%u.%u.%u:%hu)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    pchid,
                    (*r.repsources.offset(i as isize)).chunkid,
                    ip >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                    ip >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                    ip >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                    ip & 0xff as uint32_t,
                    (*r.repsources.offset(i as isize)).port as ::core::ffi::c_int,
                );
                rep_cleanup(&raw mut r);
                return MFS_ERROR_WRONGCHUNKID as uint8_t;
            }
            if pver != (*r.repsources.offset(i as isize)).version {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"replicator,get # of blocks: got wrong answer (chunk_status:version:%X/%X) from (%u.%u.%u.%u:%hu)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    pver,
                    (*r.repsources.offset(i as isize)).version,
                    ip >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                    ip >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                    ip >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                    ip & 0xff as uint32_t,
                    (*r.repsources.offset(i as isize)).port as ::core::ffi::c_int,
                );
                rep_cleanup(&raw mut r);
                return MFS_ERROR_WRONGVERSION as uint8_t;
            }
            if pstatus as ::core::ffi::c_int != MFS_STATUS_OK {
                if pstatus as ::core::ffi::c_int == MFS_ERROR_NOTDONE {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_NOTICE,
                        b"replicator: chunkserver (%u.%u.%u.%u:%hu) is overloaded\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ip >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                        ip >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                        ip >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                        ip & 0xff as uint32_t,
                        (*r.repsources.offset(i as isize)).port as ::core::ffi::c_int,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"replicator,get # of blocks: got status: %s from (%u.%u.%u.%u:%hu)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        mfsstrerr(pstatus),
                        ip >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                        ip >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                        ip >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                        ip & 0xff as uint32_t,
                        (*r.repsources.offset(i as isize)).port as ::core::ffi::c_int,
                    );
                }
                rep_cleanup(&raw mut r);
                return pstatus;
            }
            (*r.repsources.offset(i as isize)).blocks = pblocks;
            i = i.wrapping_add(1);
        }
        match rmode as ::core::ffi::c_uint {
            0 => {
                blocks = (*r.repsources.offset(0 as isize)).blocks;
                blockgroup = 1 as uint8_t;
                lastsrccnt = srccnt;
            }
            1 => {
                blocks = ((((*r.repsources.offset(0 as isize)).blocks as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    + parts as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int
                    - partno as ::core::ffi::c_int)
                    / parts as ::core::ffi::c_int) as uint16_t;
                blockgroup = 4 as uint8_t;
                lastsrccnt = srccnt;
            }
            2 => {
                blocks = 0 as uint16_t;
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < srccnt as ::core::ffi::c_int {
                    if (*r.repsources.offset(i as isize)).blocks as ::core::ffi::c_int
                        > blocks as ::core::ffi::c_int
                    {
                        blocks = (*r.repsources.offset(i as isize)).blocks;
                    }
                    i = i.wrapping_add(1);
                }
                blocks = (blocks as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as uint16_t;
                blockgroup = 4 as uint8_t;
                lastsrccnt = srccnt;
            }
            3 => {
                blocks = 0 as uint16_t;
                lastsrccnt = srccnt;
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < srccnt as ::core::ffi::c_int {
                    if (*r.repsources.offset(i as isize)).blocks as ::core::ffi::c_int
                        > blocks as ::core::ffi::c_int
                    {
                        blocks = (*r.repsources.offset(i as isize)).blocks;
                    }
                    i = i.wrapping_add(1);
                }
                blocks = (blocks as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as uint16_t;
                blockgroup = 4 as uint8_t;
                if blocks as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    while lastsrccnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                        && (*r.repsources.offset(
                            (lastsrccnt as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                        ))
                        .blocks as ::core::ffi::c_int
                            <= (blocks as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int
                    {
                        lastsrccnt = lastsrccnt.wrapping_sub(1);
                    }
                }
                if blocks as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    srccnt = lastsrccnt;
                }
            }
            _ => {
                blocks = 0 as uint16_t;
                blockgroup = 0 as uint8_t;
                lastsrccnt = 0 as uint8_t;
            }
        }
        r.needsreadrequest = 1 as uint8_t;
        b = 0 as uint16_t;
        trycnt = REP_RETRY_CNT as uint8_t;
        while (b as ::core::ffi::c_int) < blocks as ::core::ffi::c_int {
            now = monotonic_seconds();
            if now > reptotalto {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"replicator: operation timed out after %.2lfs (total timeout)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    now - start,
                );
                rep_cleanup(&raw mut r);
                return MFS_ERROR_ETIMEDOUT as uint8_t;
            }
            if now > progcheck {
                progcheck += PROGRESS_CHECK as ::core::ffi::c_double;
                if (now - start) * blocks as ::core::ffi::c_int as ::core::ffi::c_double
                    / b as ::core::ffi::c_int as ::core::ffi::c_double
                    > MAX_REP_TIME_SEC as ::core::ffi::c_double
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_NOTICE,
                        b"replicator: operation timed out after %.2lfs (partial timeout, replicated only %u blocks out of %u)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        now - start,
                        b as ::core::ffi::c_int,
                        blocks as ::core::ffi::c_int,
                    );
                    rep_cleanup(&raw mut r);
                    return MFS_ERROR_ETIMEDOUT as uint8_t;
                }
            }
            if b as ::core::ffi::c_int + 1 as ::core::ffi::c_int == blocks as ::core::ffi::c_int {
                readsrccnt = lastsrccnt;
            } else {
                readsrccnt = srccnt;
            }
            match rmode as ::core::ffi::c_uint {
                0 => {
                    if r.needsreadrequest != 0 {
                        rep_create_read_request(
                            r.repsources,
                            (b as ::core::ffi::c_int * MFSBLOCKSIZE) as uint32_t,
                            ((blocks as ::core::ffi::c_int - b as ::core::ffi::c_int)
                                * MFSBLOCKSIZE) as uint32_t,
                        );
                    }
                }
                1 => {
                    rep_create_read_request(
                        r.repsources,
                        (partno as ::core::ffi::c_int * 4 as ::core::ffi::c_int * MFSBLOCKSIZE
                            + b as ::core::ffi::c_int
                                * (parts as ::core::ffi::c_int
                                    * 4 as ::core::ffi::c_int
                                    * MFSBLOCKSIZE)) as uint32_t,
                        (4 as ::core::ffi::c_int * MFSBLOCKSIZE) as uint32_t,
                    );
                }
                2 => {
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < readsrccnt as ::core::ffi::c_int {
                        rep_create_read_request(
                            r.repsources.offset(i as ::core::ffi::c_int as isize),
                            (b as ::core::ffi::c_int * 4 as ::core::ffi::c_int * MFSBLOCKSIZE)
                                as uint32_t,
                            (4 as ::core::ffi::c_int * MFSBLOCKSIZE) as uint32_t,
                        );
                        i = i.wrapping_add(1);
                    }
                }
                3 => {
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < readsrccnt as ::core::ffi::c_int {
                        rep_create_read_request(
                            r.repsources.offset(i as ::core::ffi::c_int as isize),
                            (b as ::core::ffi::c_int * 4 as ::core::ffi::c_int * MFSBLOCKSIZE)
                                as uint32_t,
                            (4 as ::core::ffi::c_int * MFSBLOCKSIZE) as uint32_t,
                        );
                        i = i.wrapping_add(1);
                    }
                }
                _ => {}
            }
            r.needsreadrequest = 0 as uint8_t;
            if rep_send_all_packets(&raw mut r, SENDMSECTO as uint32_t) < 0 as ::core::ffi::c_int {
                if trycnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_NOTICE,
                        b"chunkid: %016lX ; version: %08X ; replication '%s' ; send timeout - reconnect\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        chunkid,
                        version,
                        if rmode as ::core::ffi::c_uint
                            == SIMPLE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            b"SIMPLE\0".as_ptr() as *const ::core::ffi::c_char
                        } else if rmode as ::core::ffi::c_uint
                            == SPLIT as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            b"SPLIT\0".as_ptr() as *const ::core::ffi::c_char
                        } else if rmode as ::core::ffi::c_uint
                            == RECOVER as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            b"RECOVER\0".as_ptr() as *const ::core::ffi::c_char
                        } else if rmode as ::core::ffi::c_uint
                            == JOIN as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            b"JOIN\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"???\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                    if rep_reconnect(&raw mut r) < 0 as ::core::ffi::c_int {
                        rep_cleanup(&raw mut r);
                        return MFS_ERROR_DISCONNECTED as uint8_t;
                    }
                    trycnt = trycnt.wrapping_sub(1);
                } else {
                    rep_cleanup(&raw mut r);
                    return MFS_ERROR_DISCONNECTED as uint8_t;
                }
            } else {
                bg = 0 as uint8_t;
                while (bg as ::core::ffi::c_int) < blockgroup as ::core::ffi::c_int {
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < readsrccnt as ::core::ffi::c_int {
                        (*r.repsources.offset(i as isize)).mode = HEADER;
                        (*r.repsources.offset(i as isize)).startptr =
                            &raw mut (*r.repsources.offset(i as isize)).hdrbuff as *mut uint8_t;
                        (*r.repsources.offset(i as isize)).bytesleft = 8 as uint32_t;
                        i = i.wrapping_add(1);
                    }
                    if rep_receive_all_packets(&raw mut r, RECVMSECTO as uint32_t)
                        < 0 as ::core::ffi::c_int
                    {
                        if trycnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"chunkid: %016lX ; version: %08X ; replication '%s' ; receive timeout - reconnect\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                chunkid,
                                version,
                                if rmode as ::core::ffi::c_uint
                                    == SIMPLE as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    b"SIMPLE\0".as_ptr() as *const ::core::ffi::c_char
                                } else if rmode as ::core::ffi::c_uint
                                    == SPLIT as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    b"SPLIT\0".as_ptr() as *const ::core::ffi::c_char
                                } else if rmode as ::core::ffi::c_uint
                                    == RECOVER as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    b"RECOVER\0".as_ptr() as *const ::core::ffi::c_char
                                } else if rmode as ::core::ffi::c_uint
                                    == JOIN as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    b"JOIN\0".as_ptr() as *const ::core::ffi::c_char
                                } else {
                                    b"???\0".as_ptr() as *const ::core::ffi::c_char
                                },
                            );
                            if rep_reconnect(&raw mut r) < 0 as ::core::ffi::c_int {
                                rep_cleanup(&raw mut r);
                                return MFS_ERROR_DISCONNECTED as uint8_t;
                            }
                            trycnt = trycnt.wrapping_sub(1);
                            break;
                        } else {
                            rep_cleanup(&raw mut r);
                            return MFS_ERROR_DISCONNECTED as uint8_t;
                        }
                    } else {
                        i = 0 as uint8_t;
                        while (i as ::core::ffi::c_int) < readsrccnt as ::core::ffi::c_int {
                            let mut type_0: uint32_t = 0;
                            let mut size_0: uint32_t = 0;
                            let mut pchid_0: uint64_t = 0;
                            let mut pblocknum: uint16_t = 0;
                            let mut poffset: uint16_t = 0;
                            let mut expectedblocknum: uint16_t = 0;
                            let mut psize: uint32_t = 0;
                            let mut pstatus_0: uint8_t = 0;
                            let mut ip_0: uint32_t = 0;
                            rptr =
                                &raw mut (*r.repsources.offset(i as isize)).hdrbuff as *mut uint8_t;
                            type_0 = get32bit(&raw mut rptr);
                            size_0 = get32bit(&raw mut rptr);
                            rptr = (*r.repsources.offset(i as isize)).packet;
                            ip_0 = (*r.repsources.offset(i as isize)).ip;
                            match rmode as ::core::ffi::c_uint {
                                0 => {
                                    expectedblocknum = b;
                                }
                                1 => {
                                    expectedblocknum = ((b as ::core::ffi::c_int
                                        * parts as ::core::ffi::c_int
                                        + partno as ::core::ffi::c_int)
                                        * 4 as ::core::ffi::c_int
                                        + bg as ::core::ffi::c_int)
                                        as uint16_t;
                                }
                                2 | 3 => {
                                    expectedblocknum = (b as ::core::ffi::c_int
                                        * 4 as ::core::ffi::c_int
                                        + bg as ::core::ffi::c_int)
                                        as uint16_t;
                                }
                                _ => {
                                    expectedblocknum = 0xffff as uint16_t;
                                }
                            }
                            if rptr.is_null() {
                                rep_cleanup(&raw mut r);
                                return MFS_ERROR_DISCONNECTED as uint8_t;
                            }
                            if type_0 == CSTOCL_READ_STATUS as uint32_t && size_0 == 9 as uint32_t {
                                pchid_0 = get64bit(&raw mut rptr);
                                pstatus_0 = get8bit(&raw mut rptr);
                                if pchid_0 != (*r.repsources.offset(i as isize)).chunkid {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"replicator,read chunks: got wrong answer (read_status:chunkid:%lX/%lX) from (%u.%u.%u.%u:%hu)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        pchid_0,
                                        (*r.repsources.offset(i as isize)).chunkid,
                                        ip_0 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 & 0xff as uint32_t,
                                        (*r.repsources.offset(i as isize)).port
                                            as ::core::ffi::c_int,
                                    );
                                    rep_cleanup(&raw mut r);
                                    return MFS_ERROR_WRONGCHUNKID as uint8_t;
                                }
                                if pstatus_0 as ::core::ffi::c_int == MFS_STATUS_OK {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"replicator,read chunks: got unexpected ok status from (%u.%u.%u.%u:%hu)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        ip_0 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 & 0xff as uint32_t,
                                        (*r.repsources.offset(i as isize)).port
                                            as ::core::ffi::c_int,
                                    );
                                    rep_cleanup(&raw mut r);
                                    return MFS_ERROR_DISCONNECTED as uint8_t;
                                }
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_NOTICE,
                                    b"replicator,read chunks: got status: %s from (%u.%u.%u.%u:%hu)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    mfsstrerr(pstatus_0),
                                    ip_0 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                    ip_0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                    ip_0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                    ip_0 & 0xff as uint32_t,
                                    (*r.repsources.offset(i as isize)).port
                                        as ::core::ffi::c_int,
                                );
                                rep_cleanup(&raw mut r);
                                return pstatus_0;
                            } else if type_0 == CSTOCL_READ_DATA as uint32_t
                                && size_0 == (20 as ::core::ffi::c_int + MFSBLOCKSIZE) as uint32_t
                            {
                                pchid_0 = get64bit(&raw mut rptr);
                                pblocknum = get16bit(&raw mut rptr);
                                poffset = get16bit(&raw mut rptr);
                                psize = get32bit(&raw mut rptr);
                                if pchid_0 != (*r.repsources.offset(i as isize)).chunkid {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"replicator,read chunks: got wrong answer (read_data:chunkid:%lX/%lX) from (%u.%u.%u.%u:%hu)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        pchid_0,
                                        (*r.repsources.offset(i as isize)).chunkid,
                                        ip_0 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 & 0xff as uint32_t,
                                        (*r.repsources.offset(i as isize)).port
                                            as ::core::ffi::c_int,
                                    );
                                    rep_cleanup(&raw mut r);
                                    return MFS_ERROR_WRONGCHUNKID as uint8_t;
                                }
                                if pblocknum as ::core::ffi::c_int
                                    != expectedblocknum as ::core::ffi::c_int
                                {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"replicator,read chunks: got wrong answer (read_data:blocknum:%hu/%hu) from (%u.%u.%u.%u:%hu)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        pblocknum as ::core::ffi::c_int,
                                        b as ::core::ffi::c_int,
                                        ip_0 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 & 0xff as uint32_t,
                                        (*r.repsources.offset(i as isize)).port
                                            as ::core::ffi::c_int,
                                    );
                                    rep_cleanup(&raw mut r);
                                    return MFS_ERROR_DISCONNECTED as uint8_t;
                                }
                                if poffset as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"replicator,read chunks: got wrong answer (read_data:offset:%hu) from (%u.%u.%u.%u:%hu)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        poffset as ::core::ffi::c_int,
                                        ip_0 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 & 0xff as uint32_t,
                                        (*r.repsources.offset(i as isize)).port
                                            as ::core::ffi::c_int,
                                    );
                                    rep_cleanup(&raw mut r);
                                    return MFS_ERROR_WRONGOFFSET as uint8_t;
                                }
                                if psize != MFSBLOCKSIZE as uint32_t {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"replicator,read chunks: got wrong answer (read_data:size:%u) from (%u.%u.%u.%u:%hu)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        psize,
                                        ip_0 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_0 & 0xff as uint32_t,
                                        (*r.repsources.offset(i as isize)).port
                                            as ::core::ffi::c_int,
                                    );
                                    rep_cleanup(&raw mut r);
                                    return MFS_ERROR_WRONGSIZE as uint8_t;
                                }
                            } else {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"replicator,read chunks: got wrong answer (type:0x%08X/size:0x%08X) from (%u.%u.%u.%u:%hu)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    type_0,
                                    size_0,
                                    ip_0 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                    ip_0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                    ip_0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                    ip_0 & 0xff as uint32_t,
                                    (*r.repsources.offset(i as isize)).port
                                        as ::core::ffi::c_int,
                                );
                                rep_cleanup(&raw mut r);
                                return MFS_ERROR_DISCONNECTED as uint8_t;
                            }
                            (*r.repsources.offset(i as isize)).datapackets[bg as usize] =
                                (*r.repsources.offset(i as isize)).packet;
                            (*r.repsources.offset(i as isize)).packet =
                                ::core::ptr::null_mut::<uint8_t>();
                            i = i.wrapping_add(1);
                        }
                        bg = bg.wrapping_add(1);
                    }
                }
                if r.needsreadrequest != 0 {
                    continue;
                }
                if rmode as ::core::ffi::c_uint
                    != SIMPLE as ::core::ffi::c_int as ::core::ffi::c_uint
                    || b as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        == blocks as ::core::ffi::c_int
                {
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < readsrccnt as ::core::ffi::c_int {
                        (*r.repsources.offset(i as isize)).mode = HEADER;
                        (*r.repsources.offset(i as isize)).startptr =
                            &raw mut (*r.repsources.offset(i as isize)).hdrbuff as *mut uint8_t;
                        (*r.repsources.offset(i as isize)).bytesleft = 8 as uint32_t;
                        i = i.wrapping_add(1);
                    }
                    if rep_receive_all_packets(&raw mut r, RECVMSECTO as uint32_t)
                        < 0 as ::core::ffi::c_int
                    {
                        if trycnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"chunkid: %016lX ; version: %08X ; replication '%s' ; receive status timeout - reconnect\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                chunkid,
                                version,
                                if rmode as ::core::ffi::c_uint
                                    == SIMPLE as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    b"SIMPLE\0".as_ptr() as *const ::core::ffi::c_char
                                } else if rmode as ::core::ffi::c_uint
                                    == SPLIT as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    b"SPLIT\0".as_ptr() as *const ::core::ffi::c_char
                                } else if rmode as ::core::ffi::c_uint
                                    == RECOVER as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    b"RECOVER\0".as_ptr() as *const ::core::ffi::c_char
                                } else if rmode as ::core::ffi::c_uint
                                    == JOIN as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    b"JOIN\0".as_ptr() as *const ::core::ffi::c_char
                                } else {
                                    b"???\0".as_ptr() as *const ::core::ffi::c_char
                                },
                            );
                            if rep_reconnect(&raw mut r) < 0 as ::core::ffi::c_int {
                                rep_cleanup(&raw mut r);
                                return MFS_ERROR_DISCONNECTED as uint8_t;
                            }
                            trycnt = trycnt.wrapping_sub(1);
                            continue;
                        } else {
                            rep_cleanup(&raw mut r);
                            return MFS_ERROR_DISCONNECTED as uint8_t;
                        }
                    } else {
                        i = 0 as uint8_t;
                        while (i as ::core::ffi::c_int) < readsrccnt as ::core::ffi::c_int {
                            let mut type_1: uint32_t = 0;
                            let mut size_1: uint32_t = 0;
                            let mut pchid_1: uint64_t = 0;
                            let mut pstatus_1: uint8_t = 0;
                            let mut ip_1: uint32_t = 0;
                            rptr =
                                &raw mut (*r.repsources.offset(i as isize)).hdrbuff as *mut uint8_t;
                            type_1 = get32bit(&raw mut rptr);
                            size_1 = get32bit(&raw mut rptr);
                            rptr = (*r.repsources.offset(i as isize)).packet;
                            ip_1 = (*r.repsources.offset(i as isize)).ip;
                            if rptr.is_null()
                                || type_1 != CSTOCL_READ_STATUS as uint32_t
                                || size_1 != 9 as uint32_t
                            {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"replicator,check status: got wrong answer (type:0x%08X/size:0x%08X) from (%u.%u.%u.%u:%hu)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    type_1,
                                    size_1,
                                    ip_1 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                    ip_1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                    ip_1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                    ip_1 & 0xff as uint32_t,
                                    (*r.repsources.offset(i as isize)).port
                                        as ::core::ffi::c_int,
                                );
                                rep_cleanup(&raw mut r);
                                return MFS_ERROR_DISCONNECTED as uint8_t;
                            }
                            pchid_1 = get64bit(&raw mut rptr);
                            pstatus_1 = get8bit(&raw mut rptr);
                            if pchid_1 != (*r.repsources.offset(i as isize)).chunkid {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"replicator,check status: got wrong answer (read_status:chunkid:%lX/%lX) from (%u.%u.%u.%u:%hu)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    pchid_1,
                                    (*r.repsources.offset(i as isize)).chunkid,
                                    ip_1 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                    ip_1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                    ip_1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                    ip_1 & 0xff as uint32_t,
                                    (*r.repsources.offset(i as isize)).port
                                        as ::core::ffi::c_int,
                                );
                                rep_cleanup(&raw mut r);
                                return MFS_ERROR_WRONGCHUNKID as uint8_t;
                            }
                            if pstatus_1 as ::core::ffi::c_int != MFS_STATUS_OK {
                                if pstatus_1 as ::core::ffi::c_int == MFS_ERROR_NOTDONE {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_NOTICE,
                                        b"replicator: chunkserver (%u.%u.%u.%u:%hu) is overloaded\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        ip_1 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_1 & 0xff as uint32_t,
                                        (*r.repsources.offset(i as isize)).port
                                            as ::core::ffi::c_int,
                                    );
                                } else {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_NOTICE,
                                        b"replicator,check status: got status: %s from (%u.%u.%u.%u:%hu)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        mfsstrerr(pstatus_1),
                                        ip_1 >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                                        ip_1 & 0xff as uint32_t,
                                        (*r.repsources.offset(i as isize)).port
                                            as ::core::ffi::c_int,
                                    );
                                }
                                rep_cleanup(&raw mut r);
                                return pstatus_1;
                            }
                            i = i.wrapping_add(1);
                        }
                    }
                }
                match rmode as ::core::ffi::c_uint {
                    0 => {
                        rptr = (*r.repsources.offset(0 as isize)).datapackets[0 as usize];
                        status = hdd_write(
                            chunkid,
                            0 as uint32_t,
                            b,
                            rptr.offset(20 as ::core::ffi::c_int as isize),
                            0 as uint32_t,
                            MFSBLOCKSIZE as uint32_t,
                            rptr.offset(16 as ::core::ffi::c_int as isize),
                        ) as uint8_t;
                        if status as ::core::ffi::c_int != MFS_STATUS_OK {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"replicator: write status: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                mfsstrerr(status),
                            );
                            rep_cleanup(&raw mut r);
                            return status;
                        }
                    }
                    1 => {
                        bg = 0 as uint8_t;
                        while (bg as ::core::ffi::c_int) < blockgroup as ::core::ffi::c_int {
                            rptr = (*r.repsources.offset(0 as isize)).datapackets[bg as usize];
                            status = hdd_write(
                                chunkid,
                                0 as uint32_t,
                                (b as ::core::ffi::c_int * blockgroup as ::core::ffi::c_int
                                    + bg as ::core::ffi::c_int)
                                    as uint16_t,
                                rptr.offset(20 as ::core::ffi::c_int as isize),
                                0 as uint32_t,
                                MFSBLOCKSIZE as uint32_t,
                                rptr.offset(16 as ::core::ffi::c_int as isize),
                            ) as uint8_t;
                            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"replicator: write status: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    mfsstrerr(status),
                                );
                                rep_cleanup(&raw mut r);
                                return status;
                            }
                            bg = bg.wrapping_add(1);
                        }
                    }
                    2 => {
                        bg = 0 as uint8_t;
                        while (bg as ::core::ffi::c_int) < blockgroup as ::core::ffi::c_int {
                            xcrc = 0 as uint32_t;
                            i = 0 as uint8_t;
                            while (i as ::core::ffi::c_int) < srccnt as ::core::ffi::c_int {
                                rptr = (*r.repsources.offset(i as isize)).datapackets[bg as usize];
                                rptr = rptr.offset(16 as ::core::ffi::c_int as isize);
                                if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                    xcrc = get32bit(&raw mut rptr);
                                    memcpy(
                                        r.xorbuff.offset(4 as ::core::ffi::c_int as isize)
                                            as *mut ::core::ffi::c_void,
                                        rptr as *const ::core::ffi::c_void,
                                        MFSBLOCKSIZE as size_t,
                                    );
                                } else {
                                    xcrc ^= get32bit(&raw mut rptr);
                                    xordata(
                                        r.xorbuff.offset(4 as ::core::ffi::c_int as isize),
                                        rptr,
                                        MFSBLOCKSIZE as uint32_t,
                                    );
                                }
                                i = i.wrapping_add(1);
                            }
                            xcrc ^= zcrc;
                            wptr = r.xorbuff;
                            put32bit(&raw mut wptr, xcrc);
                            nonzero = 1 as uint8_t;
                            if b as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                == blocks as ::core::ffi::c_int
                                && xcrc == zcrc
                            {
                                let mut aptr: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
                                let mut oraux: uint32_t = 0;
                                rptr = r.xorbuff;
                                oraux = 0 as uint32_t;
                                if rptr.expose_provenance() as ::core::ffi::c_ulong
                                    & 0x3 as ::core::ffi::c_ulong
                                    == 0 as ::core::ffi::c_ulong
                                    && MFSBLOCKSIZE & 0x3 as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int
                                {
                                    aptr = rptr.offset(4 as ::core::ffi::c_int as isize)
                                        as *mut uint32_t;
                                    bind = 0 as uint32_t;
                                    while bind
                                        < (MFSBLOCKSIZE / 4 as ::core::ffi::c_int) as uint32_t
                                    {
                                        oraux |= *aptr;
                                        aptr = aptr.offset(1);
                                        bind = bind.wrapping_add(1);
                                    }
                                } else {
                                    bind = 0 as uint32_t;
                                    while bind < MFSBLOCKSIZE as uint32_t {
                                        oraux |= *rptr
                                            .offset((4 as uint32_t).wrapping_add(bind) as isize)
                                            as uint32_t;
                                        bind = bind.wrapping_add(1);
                                    }
                                }
                                if oraux == 0 as uint32_t {
                                    nonzero = 0 as uint8_t;
                                }
                            }
                            if nonzero != 0 {
                                rptr = r.xorbuff;
                                status = hdd_write(
                                    chunkid,
                                    0 as uint32_t,
                                    (b as ::core::ffi::c_int * blockgroup as ::core::ffi::c_int
                                        + bg as ::core::ffi::c_int)
                                        as uint16_t,
                                    rptr.offset(4 as ::core::ffi::c_int as isize),
                                    0 as uint32_t,
                                    MFSBLOCKSIZE as uint32_t,
                                    rptr,
                                ) as uint8_t;
                                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"replicator: write status: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        mfsstrerr(status),
                                    );
                                    rep_cleanup(&raw mut r);
                                    return status;
                                }
                            }
                            bg = bg.wrapping_add(1);
                        }
                    }
                    3 => {
                        i = 0 as uint8_t;
                        while (i as ::core::ffi::c_int) < readsrccnt as ::core::ffi::c_int {
                            bg = 0 as uint8_t;
                            while (bg as ::core::ffi::c_int) < blockgroup as ::core::ffi::c_int {
                                rptr = (*r.repsources.offset(i as isize)).datapackets[bg as usize];
                                status = hdd_write(
                                    chunkid,
                                    0 as uint32_t,
                                    ((b as ::core::ffi::c_int * parts as ::core::ffi::c_int
                                        + i as ::core::ffi::c_int)
                                        * blockgroup as ::core::ffi::c_int
                                        + bg as ::core::ffi::c_int)
                                        as uint16_t,
                                    rptr.offset(20 as ::core::ffi::c_int as isize),
                                    0 as uint32_t,
                                    MFSBLOCKSIZE as uint32_t,
                                    rptr.offset(16 as ::core::ffi::c_int as isize),
                                ) as uint8_t;
                                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"replicator: write status: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        mfsstrerr(status),
                                    );
                                    rep_cleanup(&raw mut r);
                                    return status;
                                }
                                bg = bg.wrapping_add(1);
                            }
                            i = i.wrapping_add(1);
                        }
                    }
                    _ => {}
                }
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < readsrccnt as ::core::ffi::c_int {
                    bg = 0 as uint8_t;
                    while (bg as ::core::ffi::c_int) < blockgroup as ::core::ffi::c_int {
                        if !(*r.repsources.offset(i as isize)).datapackets[bg as usize].is_null() {
                            free(
                                (*r.repsources.offset(i as isize)).datapackets[bg as usize]
                                    as *mut ::core::ffi::c_void,
                            );
                        }
                        (*r.repsources.offset(i as isize)).datapackets[bg as usize] =
                            ::core::ptr::null_mut::<uint8_t>();
                        bg = bg.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
                b = b.wrapping_add(1);
                trycnt = REP_RETRY_CNT as uint8_t;
            }
        }
        status = hdd_rep_setversion(chunkid, version) as uint8_t;
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"replicator: set version status: %s\0".as_ptr() as *const ::core::ffi::c_char,
                mfsstrerr(status),
            );
            rep_cleanup(&raw mut r);
            return status;
        }
        status = hdd_close(chunkid, 1 as uint8_t) as uint8_t;
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"replicator: close status: %s\0".as_ptr() as *const ::core::ffi::c_char,
                mfsstrerr(status),
            );
            rep_cleanup(&raw mut r);
            return status;
        }
        r.opened = 0 as uint8_t;
        r.created = 0 as uint8_t;
        rep_cleanup(&raw mut r);
        return MFS_STATUS_OK as uint8_t;
    }
}
