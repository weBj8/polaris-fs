use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pthread_sigmask(
        __how: ::core::ffi::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> ::core::ffi::c_int;
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
    fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn tcpsocket() -> ::core::ffi::c_int;
    fn tcpnonblock(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tcpsetacceptfilter(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tcpnodelay(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tcpstrlisten(
        sock: ::core::ffi::c_int,
        hostname: *const ::core::ffi::c_char,
        service: *const ::core::ffi::c_char,
        queue: uint16_t,
    ) -> ::core::ffi::c_int;
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
    fn tcptoaccept(sock: ::core::ffi::c_int, msecto: uint32_t) -> ::core::ffi::c_int;
    fn tcpgetmyaddr(
        sock: ::core::ffi::c_int,
        ip: *mut uint32_t,
        port: *mut uint16_t,
    ) -> ::core::ffi::c_int;
    fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fs_custom(
        qcmd: uint32_t,
        query: *const uint8_t,
        queryleng: uint32_t,
        acmd: *mut uint32_t,
        answer: *mut *const uint8_t,
        answerleng: *mut uint32_t,
    ) -> uint8_t;
    fn negentry_cache_clear();
    fn mfs_dentry_invalidate(parent: uint32_t, nleng: uint8_t, name: *const ::core::ffi::c_char);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
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
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const PTHREAD_CREATE_DETACHED: C2Rust_Unnamed = 1;
pub const PTHREAD_CREATE_JOINABLE: C2Rust_Unnamed = 0;
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
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _conn_data {
    pub sock: ::core::ffi::c_int,
    pub sendnops: ::core::ffi::c_int,
    pub lock: pthread_mutex_t,
}
pub type conn_data = _conn_data;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SIG_BLOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIG_SETMASK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FUSE_REGISTER_BLOB_ACL: [::core::ffi::c_char; 65] = unsafe {
    ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
        *b"DjI1GAQDULI5d2YjA26ypc3ovkhjvhciTQVx3CS4nYgtBoUcsljiVpsErJENHaw0\0",
    )
};
pub const REGISTER_TOOLS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_REGISTER: ::core::ffi::c_int = PROTO_BASE + 400 as ::core::ffi::c_int;
pub const MATOCL_FUSE_REGISTER: ::core::ffi::c_int = PROTO_BASE + 401 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SNAPSHOT: ::core::ffi::c_int = PROTO_BASE + 468 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SNAPSHOT: ::core::ffi::c_int = PROTO_BASE + 469 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PROTO_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn put32bit(mut ptr: *mut *mut uint8_t, mut val: uint32_t) {
    val = val.swap_bytes() as uint32_t;
    memcpy(
        *ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        4 as size_t,
    );
    *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn put16bit(mut ptr: *mut *mut uint8_t, mut val: uint16_t) {
    val = val.swap_bytes() as uint16_t;
    memcpy(
        *ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        2 as size_t,
    );
    *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn put8bit(mut ptr: *mut *mut uint8_t, mut val: uint8_t) {
    *(*ptr).offset(0 as isize) =
        (val as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
    *ptr = (*ptr).offset(1);
}
#[inline]
unsafe extern "C" fn get32bit(mut ptr: *mut *const uint8_t) -> uint32_t {
    let mut t32: uint32_t = 0;
    memcpy(
        &raw mut t32 as *mut ::core::ffi::c_void,
        *ptr as *const ::core::ffi::c_void,
        4 as size_t,
    );
    *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
    return t32.swap_bytes();
}
#[inline]
unsafe extern "C" fn get8bit(mut ptr: *mut *const uint8_t) -> uint8_t {
    let mut t8: uint8_t = 0;
    t8 = *(*ptr).offset(0 as isize);
    *ptr = (*ptr).offset(1);
    return t8;
}
#[inline]
unsafe extern "C" fn portable_usleep(mut usec: uint64_t) {
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
pub const TOOLTIMEOUTPARTMS: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const TOOLTIMEOUTALLMS: ::core::ffi::c_int = 30000 as ::core::ffi::c_int;
pub const TOOLSNOPMS: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
pub const AUXBUFFSIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
static mut lsock: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
static mut proxythread: pthread_t = 0;
static mut terminate: uint8_t = 0;
static mut proxyhost: uint32_t = 0;
static mut proxyport: uint16_t = 0;
#[no_mangle]
pub unsafe extern "C" fn masterproxy_getlocation(mut masterinfo: *mut uint8_t) {
    let mut rptr: *const uint8_t = masterinfo.offset(10 as ::core::ffi::c_int as isize);
    if lsock >= 0 as ::core::ffi::c_int && get32bit(&raw mut rptr) >= 0x10618 as uint32_t {
        put32bit(&raw mut masterinfo, proxyhost);
        put16bit(&raw mut masterinfo, proxyport);
    }
}
#[no_mangle]
pub unsafe extern "C" fn masterproxy_free_conn_data(mut cd: *mut conn_data) {
    pthread_mutex_destroy(&raw mut (*cd).lock);
    tcpclose((*cd).sock);
    free(cd as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn masterproxy_keepalive(
    mut args: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut cd: *mut conn_data = args as *mut conn_data;
    let mut nopcnt: ::core::ffi::c_int = 0;
    let mut nopbuff: [uint8_t; 8] = [0; 8];
    let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    wptr = &raw mut nopbuff as *mut uint8_t;
    put32bit(&raw mut wptr, ANTOAN_NOP as uint32_t);
    put32bit(&raw mut wptr, 0 as uint32_t);
    nopcnt = 0 as ::core::ffi::c_int;
    loop {
        pthread_mutex_lock(&raw mut (*cd).lock);
        if (*cd).sendnops == 255 as ::core::ffi::c_int {
            pthread_mutex_unlock(&raw mut (*cd).lock);
            masterproxy_free_conn_data(cd);
            return NULL;
        }
        if (*cd).sendnops == 0 as ::core::ffi::c_int {
            pthread_mutex_unlock(&raw mut (*cd).lock);
            nopcnt = 0 as ::core::ffi::c_int;
        } else {
            nopcnt += 1;
            if nopcnt >= TOOLSNOPMS / 100 as ::core::ffi::c_int {
                (*cd).sendnops = 2 as ::core::ffi::c_int;
                pthread_mutex_unlock(&raw mut (*cd).lock);
                if tcptowrite(
                    (*cd).sock,
                    &raw mut nopbuff as *mut uint8_t as *const ::core::ffi::c_void,
                    8 as uint32_t,
                    TOOLTIMEOUTPARTMS as uint32_t,
                    TOOLTIMEOUTALLMS as uint32_t,
                ) != 8 as int32_t
                {
                    break;
                }
                pthread_mutex_lock(&raw mut (*cd).lock);
                (*cd).sendnops = 1 as ::core::ffi::c_int;
                nopcnt = 0 as ::core::ffi::c_int;
            }
            pthread_mutex_unlock(&raw mut (*cd).lock);
        }
        portable_usleep(100000 as uint64_t);
    }
    return NULL;
}
unsafe extern "C" fn masterproxy_server(
    mut args: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut header: [uint8_t; 8] = [0; 8];
    let mut auxbuffer: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut aptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut cd: *mut conn_data = args as *mut conn_data;
    let mut psize: uint32_t = 0;
    let mut cmd: uint32_t = 0;
    let mut msgid: uint32_t = 0;
    let mut asize: uint32_t = 0;
    let mut acmd: uint32_t = 0;
    let mut inode_dst: uint32_t = 0;
    let mut name_dst_len: uint8_t = 0;
    let mut name_dst: [::core::ffi::c_char; 256] = [0; 256];
    auxbuffer = malloc(AUXBUFFSIZE as size_t) as *mut uint8_t;
    while tcptoread(
        (*cd).sock,
        &raw mut header as *mut uint8_t as *mut ::core::ffi::c_void,
        8 as uint32_t,
        TOOLTIMEOUTPARTMS as uint32_t,
        TOOLTIMEOUTALLMS as uint32_t,
    ) == 8 as int32_t
    {
        rptr = &raw mut header as *mut uint8_t;
        cmd = get32bit(&raw mut rptr);
        psize = get32bit(&raw mut rptr);
        if cmd == CLTOMA_FUSE_REGISTER as uint32_t {
            if psize != 73 as uint32_t {
                break;
            }
            if tcptoread(
                (*cd).sock,
                auxbuffer as *mut ::core::ffi::c_void,
                psize,
                TOOLTIMEOUTPARTMS as uint32_t,
                TOOLTIMEOUTALLMS as uint32_t,
            ) != psize as int32_t
            {
                break;
            }
            if memcmp(
                auxbuffer as *const ::core::ffi::c_void,
                FUSE_REGISTER_BLOB_ACL.as_ptr() as *const ::core::ffi::c_void,
                64 as size_t,
            ) != 0 as ::core::ffi::c_int
            {
                break;
            }
            if *auxbuffer.offset(64 as isize) as ::core::ffi::c_int != REGISTER_TOOLS {
                break;
            }
            wptr = auxbuffer;
            put32bit(&raw mut wptr, MATOCL_FUSE_REGISTER as uint32_t);
            put32bit(&raw mut wptr, 1 as uint32_t);
            put8bit(&raw mut wptr, MFS_STATUS_OK as uint8_t);
            if tcptowrite(
                (*cd).sock,
                auxbuffer as *const ::core::ffi::c_void,
                9 as uint32_t,
                TOOLTIMEOUTPARTMS as uint32_t,
                TOOLTIMEOUTALLMS as uint32_t,
            ) != 9 as int32_t
            {
                break;
            }
        } else {
            if psize < 4 as uint32_t || psize > AUXBUFFSIZE as uint32_t {
                break;
            }
            if tcptoread(
                (*cd).sock,
                auxbuffer as *mut ::core::ffi::c_void,
                psize,
                TOOLTIMEOUTPARTMS as uint32_t,
                TOOLTIMEOUTALLMS as uint32_t,
            ) != psize as int32_t
            {
                break;
            }
            pthread_mutex_lock(&raw mut (*cd).lock);
            (*cd).sendnops = 1 as ::core::ffi::c_int;
            pthread_mutex_unlock(&raw mut (*cd).lock);
            rptr = auxbuffer;
            msgid = get32bit(&raw mut rptr);
            inode_dst = 0 as uint32_t;
            name_dst_len = 0 as uint8_t;
            if cmd == CLTOMA_FUSE_SNAPSHOT as uint32_t {
                if psize >= 13 as uint32_t {
                    rptr = rptr.offset(4 as ::core::ffi::c_int as isize);
                    inode_dst = get32bit(&raw mut rptr);
                    name_dst_len = get8bit(&raw mut rptr);
                    if psize >= (13 as uint32_t).wrapping_add(name_dst_len as uint32_t) {
                        memcpy(
                            &raw mut name_dst as *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_void,
                            rptr as *const ::core::ffi::c_void,
                            name_dst_len as size_t,
                        );
                        name_dst[name_dst_len as usize] = 0 as ::core::ffi::c_char;
                    } else {
                        inode_dst = 0 as uint32_t;
                    }
                }
            }
            if fs_custom(
                cmd,
                auxbuffer.offset(4 as ::core::ffi::c_int as isize),
                psize.wrapping_sub(4 as uint32_t),
                &raw mut acmd,
                &raw mut aptr,
                &raw mut asize,
            ) as ::core::ffi::c_int
                != MFS_STATUS_OK
            {
                break;
            }
            if cmd == CLTOMA_FUSE_SNAPSHOT as uint32_t && acmd == MATOCL_FUSE_SNAPSHOT as uint32_t {
                negentry_cache_clear();
                if inode_dst > 0 as uint32_t
                    && name_dst_len as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                {
                    mfs_dentry_invalidate(
                        inode_dst,
                        name_dst_len,
                        &raw mut name_dst as *mut ::core::ffi::c_char,
                    );
                }
            }
            wptr = auxbuffer;
            put32bit(&raw mut wptr, acmd);
            put32bit(&raw mut wptr, asize.wrapping_add(4 as uint32_t));
            put32bit(&raw mut wptr, msgid);
            pthread_mutex_lock(&raw mut (*cd).lock);
            while (*cd).sendnops == 2 as ::core::ffi::c_int {
                pthread_mutex_unlock(&raw mut (*cd).lock);
                portable_usleep(10000 as uint64_t);
                pthread_mutex_lock(&raw mut (*cd).lock);
            }
            (*cd).sendnops = 0 as ::core::ffi::c_int;
            pthread_mutex_unlock(&raw mut (*cd).lock);
            if tcptowrite(
                (*cd).sock,
                auxbuffer as *const ::core::ffi::c_void,
                12 as uint32_t,
                TOOLTIMEOUTPARTMS as uint32_t,
                TOOLTIMEOUTALLMS as uint32_t,
            ) != 12 as int32_t
            {
                break;
            }
            if tcptowrite(
                (*cd).sock,
                aptr as *const ::core::ffi::c_void,
                asize,
                TOOLTIMEOUTPARTMS as uint32_t,
                TOOLTIMEOUTALLMS as uint32_t,
            ) != asize as int32_t
            {
                break;
            }
        }
    }
    free(auxbuffer as *mut ::core::ffi::c_void);
    pthread_mutex_lock(&raw mut (*cd).lock);
    (*cd).sendnops = 255 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut (*cd).lock);
    return NULL;
}
unsafe extern "C" fn masterproxy_acceptor(
    mut args: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut clientthread: pthread_t = 0;
    let mut nopthread: pthread_t = 0;
    let mut thattr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    let mut newset: sigset_t = sigset_t { __val: [0; 16] };
    let mut cd: *mut conn_data = ::core::ptr::null_mut::<conn_data>();
    let mut sock: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_attr_init(&raw mut thattr);
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int =
        pthread_attr_setstacksize(&raw mut thattr, 0x100000 as ::core::ffi::c_int as size_t);
    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                246 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                246 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                246 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                246 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
        } else {
            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                246 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                246 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_attr_setdetachstate(
        &raw mut thattr,
        PTHREAD_CREATE_DETACHED as ::core::ffi::c_int,
    );
    if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_3: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setdetachstate(&thattr,PTHREAD_CREATE_DETACHED)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setdetachstate(&thattr,PTHREAD_CREATE_DETACHED)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setdetachstate(&thattr,PTHREAD_CREATE_DETACHED)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setdetachstate(&thattr,PTHREAD_CREATE_DETACHED)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
        } else {
            let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setdetachstate(&thattr,PTHREAD_CREATE_DETACHED)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setdetachstate(&thattr,PTHREAD_CREATE_DETACHED)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    loop {
        let c2rust_lhs = &raw mut terminate;
        let c2rust_rhs = 0 as uint8_t;
        if (::core::intrinsics::atomic_or_seqcst(c2rust_lhs, c2rust_rhs) | c2rust_rhs)
            as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            break;
        }
        sock = tcptoaccept(lsock, 1000 as uint32_t);
        if sock >= 0 as ::core::ffi::c_int {
            cd = malloc(::core::mem::size_of::<conn_data>()) as *mut conn_data;
            (*cd).sock = sock;
            (*cd).sendnops = 0 as ::core::ffi::c_int;
            pthread_mutex_init(
                &raw mut (*cd).lock,
                ::core::ptr::null::<pthread_mutexattr_t>(),
            );
            tcpnodelay(sock);
            tcpnonblock(sock);
            sigemptyset(&raw mut newset);
            sigaddset(&raw mut newset, SIGTERM);
            sigaddset(&raw mut newset, SIGINT);
            sigaddset(&raw mut newset, SIGHUP);
            sigaddset(&raw mut newset, SIGQUIT);
            pthread_sigmask(SIG_BLOCK, &raw mut newset, &raw mut oldset);
            res = pthread_create(
                &raw mut nopthread,
                &raw mut thattr,
                Some(
                    masterproxy_keepalive
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                        )
                            -> *mut ::core::ffi::c_void,
                ),
                cd as *mut ::core::ffi::c_void,
            );
            if res < 0 as ::core::ffi::c_int {
                masterproxy_free_conn_data(cd);
            } else {
                res = pthread_create(
                    &raw mut clientthread,
                    &raw mut thattr,
                    Some(
                        masterproxy_server
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                            )
                                -> *mut ::core::ffi::c_void,
                    ),
                    cd as *mut ::core::ffi::c_void,
                );
                if res < 0 as ::core::ffi::c_int {
                    pthread_mutex_lock(&raw mut (*cd).lock);
                    (*cd).sendnops = 255 as ::core::ffi::c_int;
                    pthread_mutex_unlock(&raw mut (*cd).lock);
                }
            }
            pthread_sigmask(
                SIG_SETMASK,
                &raw mut oldset,
                ::core::ptr::null_mut::<__sigset_t>(),
            );
        }
    }
    pthread_attr_destroy(&raw mut thattr);
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn masterproxy_term() {
    let c2rust_lhs = &raw mut terminate;
    let c2rust_rhs = 1 as uint8_t;
    ::core::intrinsics::atomic_or_seqcst(c2rust_lhs, c2rust_rhs) | c2rust_rhs;
    pthread_join(
        proxythread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn masterproxy_init(
    mut masterproxyip: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut thattr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    let mut newset: sigset_t = sigset_t { __val: [0; 16] };
    lsock = tcpsocket();
    if lsock < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"master proxy module: can't create socket\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    tcpnonblock(lsock);
    tcpnodelay(lsock);
    if tcpstrlisten(
        lsock,
        masterproxyip,
        ::core::ptr::null::<::core::ffi::c_char>(),
        100 as uint16_t,
    ) < 0 as ::core::ffi::c_int
    {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"master proxy module: can't listen on socket\0".as_ptr() as *const ::core::ffi::c_char,
        );
        tcpclose(lsock);
        lsock = -1 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    if tcpsetacceptfilter(lsock) < 0 as ::core::ffi::c_int && *__errno_location() != ENOTSUP {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"master proxy module: can't set accept filter\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if tcpgetmyaddr(lsock, &raw mut proxyhost, &raw mut proxyport) < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"master proxy module: can't obtain my address and port\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        tcpclose(lsock);
        lsock = -1 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    terminate = 0 as uint8_t;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_attr_init(&raw mut thattr);
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int =
        pthread_attr_setstacksize(&raw mut thattr, 0x100000 as ::core::ffi::c_int as size_t);
    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                345 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                345 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                345 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                345 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
        } else {
            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                345 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                345 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    sigemptyset(&raw mut newset);
    sigaddset(&raw mut newset, SIGTERM);
    sigaddset(&raw mut newset, SIGINT);
    sigaddset(&raw mut newset, SIGHUP);
    sigaddset(&raw mut newset, SIGQUIT);
    pthread_sigmask(SIG_BLOCK, &raw mut newset, &raw mut oldset);
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_create(
        &raw mut proxythread,
        &raw mut thattr,
        Some(
            masterproxy_acceptor
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_3: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                353 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_create(&proxythread,&thattr,masterproxy_acceptor,NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                353 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_create(&proxythread,&thattr,masterproxy_acceptor,NULL)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                353 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_create(&proxythread,&thattr,masterproxy_acceptor,NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                353 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_create(&proxythread,&thattr,masterproxy_acceptor,NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
        } else {
            let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                353 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_create(&proxythread,&thattr,masterproxy_acceptor,NULL)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                353 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_create(&proxythread,&thattr,masterproxy_acceptor,NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    pthread_sigmask(
        SIG_SETMASK,
        &raw mut oldset,
        ::core::ptr::null_mut::<__sigset_t>(),
    );
    let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_attr_destroy(&raw mut thattr);
    if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_5: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_destroy(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_destroy(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_destroy(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_destroy(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
        } else {
            let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_destroy(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/masterproxy.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_attr_destroy(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
        }
        abort();
    }
    return 1 as ::core::ffi::c_int;
}
