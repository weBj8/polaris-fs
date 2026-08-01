//! Master proxy — safe core + socket boundary (P4).
//!
//! Original: MooseFS mfsclient/masterproxy.c. Listens on a local socket
//! and proxies mfs-tools connections into fs_custom, with a NOP keepalive
//! thread per connection and snapshot side-effects (negative-entry cache
//! clear + dentry invalidation).
//!
//! The module is mostly thread/socket orchestration, which stays at the
//! boundary. All packet parsing/packing and the bounds-checked snapshot
//! extra-data parse are safe functions in `imp` with reference tests.

unsafe extern "C" {
    unsafe fn tcpsocket() -> ::core::ffi::c_int;
    unsafe fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnonblock(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnodelay(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpstrlisten(
        sock: ::core::ffi::c_int,
        ip: *const ::core::ffi::c_char,
        port: uint16_t,
        queue: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpsetacceptfilter(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpgetmyaddr(
        sock: ::core::ffi::c_int,
        ip: *mut uint32_t,
        port: *mut uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcptoaccept(lsock: ::core::ffi::c_int, msecto: uint32_t) -> ::core::ffi::c_int;
    unsafe fn tcptoread(
        sock: ::core::ffi::c_int,
        buff: *mut ::core::ffi::c_void,
        leng: uint32_t,
        mstopart: uint32_t,
        mstoall: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcptowrite(
        sock: ::core::ffi::c_int,
        buff: *const ::core::ffi::c_void,
        leng: uint32_t,
        mstopart: uint32_t,
        mstoall: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn fs_custom(
        cmd: uint32_t,
        buf: *const uint8_t,
        leng: uint32_t,
        acmd: *mut uint32_t,
        aptr: *mut *const uint8_t,
        asize: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn negentry_cache_clear();
    unsafe fn mfs_dentry_invalidate(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const ::core::ffi::c_char,
    );
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_sigmask(
        __how: ::core::ffi::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> ::core::ffi::c_int;
    unsafe fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOTSUP: ::core::ffi::c_int = 95;
pub const SIGINT: ::core::ffi::c_int = 2;
pub const SIGTERM: ::core::ffi::c_int = 15;
pub const SIGHUP: ::core::ffi::c_int = 1;
pub const SIGQUIT: ::core::ffi::c_int = 3;
pub const SIG_BLOCK: ::core::ffi::c_int = 0;
pub const SIG_SETMASK: ::core::ffi::c_int = 2;
pub const PTHREAD_CREATE_DETACHED: ::core::ffi::c_int = 1;
pub const PTHREAD_MUTEX_TIMED_NP: ::core::ffi::c_uint = 0;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2;
pub const ANTOAN_NOP: uint32_t = 0;
pub const REGISTER_TOOLS: ::core::ffi::c_int = 4;
pub const CLTOMA_FUSE_REGISTER: uint32_t = 400;
pub const MATOCL_FUSE_REGISTER: uint32_t = 401;
pub const CLTOMA_FUSE_SNAPSHOT: uint32_t = 468;
pub const MATOCL_FUSE_SNAPSHOT: uint32_t = 469;
pub const TOOLTIMEOUTPARTMS: uint32_t = 10000;
pub const TOOLTIMEOUTALLMS: uint32_t = 30000;
pub const TOOLSNOPMS: uint32_t = 5000;
pub const AUXBUFFSIZE: usize = 65536;

#[deny(unsafe_code)]
pub mod imp {
    pub const FUSE_REGISTER_BLOB_ACL: &[u8; 64] =
        b"DjI1GAQDULI5d2YjA26ypc3ovkhjvhciTQVx3CS4nYgtBoUcsljiVpsErJENHaw0";

    pub fn get32(b: &[u8]) -> u32 {
        u32::from_be_bytes([b[0], b[1], b[2], b[3]])
    }

    pub fn put32(out: &mut [u8], v: u32) {
        out[..4].copy_from_slice(&v.to_be_bytes());
    }

    /// CLTOMA_FUSE_REGISTER body validity: exactly 73 bytes, 64-byte ACL
    /// magic, then REGISTER_TOOLS.
    pub fn check_register_body(body: &[u8]) -> bool {
        body.len() == 73 && &body[..64] == FUSE_REGISTER_BLOB_ACL && body[64] == 4
    }

    /// MATOCL_FUSE_REGISTER success reply: cmd, size=1, status OK.
    pub fn register_reply() -> [u8; 9] {
        let mut out = [0u8; 9];
        out[..4].copy_from_slice(&super::MATOCL_FUSE_REGISTER.to_be_bytes());
        out[4..8].copy_from_slice(&1u32.to_be_bytes());
        out[8] = super::MFS_STATUS_OK as u8;
        out
    }

    /// NOP packet: ANTOAN_NOP, size 0.
    pub fn nop_packet() -> [u8; 8] {
        [0; 8]
    }

    /// CLTOMA_FUSE_SNAPSHOT extra data: after the msgid, [src inode:4]
    /// [dst inode:4][name len:1][name]. Bounds-checked (C: name_dst only
    /// when psize >= 13 + len counting the msgid, else inode_dst resets
    /// to 0).
    pub fn parse_snapshot_dst(body: &[u8]) -> Option<(u32, Vec<u8>)> {
        // body starts at msgid (4 bytes), then the snapshot payload
        if body.len() < 4 + 13 {
            return None;
        }
        let p = &body[4..]; // payload: src(4) dst(4) namelen(1) name
        let inode_dst = get32(&p[4..]);
        let name_len = p[8] as usize;
        if p.len() >= 9 + name_len && name_len > 0 && inode_dst > 0 {
            Some((inode_dst, p[9..9 + name_len].to_vec()))
        } else {
            None
        }
    }

    /// answer header: acmd, asize+4, msgid (12 bytes)
    pub fn answer_header(acmd: u32, asize: u32, msgid: u32) -> [u8; 12] {
        let mut out = [0u8; 12];
        out[..4].copy_from_slice(&acmd.to_be_bytes());
        out[4..8].copy_from_slice(&(asize + 4).to_be_bytes());
        out[8..12].copy_from_slice(&msgid.to_be_bytes());
        out
    }

    /// keepalive counter: after how many 100ms ticks a NOP goes out
    pub fn nop_due(nopcnt: u32) -> bool {
        nopcnt >= (super::TOOLSNOPMS / 100)
    }
}

// ---------------------------------------------------------------------------
// Boundary: threads, sockets, signals.
// ---------------------------------------------------------------------------

use std::sync::atomic::{AtomicU8, Ordering};

static mut lsock: ::core::ffi::c_int = -1;
static mut proxythread: pthread_t = 0;
static TERMINATE: AtomicU8 = AtomicU8::new(0);
static mut proxyhost: uint32_t = 0;
static mut proxyport: uint16_t = 0;

struct ConnData {
    sock: ::core::ffi::c_int,
    sendnops: AtomicU8, // 0 idle / 1 in-request / 2 nop-in-flight / 255 dead
}

// local copy, mirroring the C static-inline portable_usleep (per TU)
#[derive(Copy, Clone)]
#[repr(C)]
struct portable_timespec {
    tv_sec: ::core::ffi::c_long,
    tv_nsec: ::core::ffi::c_long,
}
unsafe extern "C" {
    fn nanosleep(
        __requested_time: *const portable_timespec,
        __remaining: *mut portable_timespec,
    ) -> ::core::ffi::c_int;
}
#[inline]
fn portable_usleep(usec: uint64_t) {
    let mut req = portable_timespec {
        tv_sec: (usec / 1_000_000) as ::core::ffi::c_long,
        tv_nsec: ((usec % 1_000_000) * 1000) as ::core::ffi::c_long,
    };
    let mut rem = portable_timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: req/rem are valid stack structs.
    unsafe {
        loop {
            let s = nanosleep(&req, &mut rem);
            if s < 0 {
                req = rem;
            } else {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterproxy_getlocation(masterinfo: *mut uint8_t) {
    unsafe {
        // masterinfo: [..10 bytes..][version:4] — writable header area
        let rptr = masterinfo.add(10);
        let version = u32::from_be_bytes([*rptr, *rptr.add(1), *rptr.add(2), *rptr.add(3)]);
        if lsock >= 0 && version >= 0x00010618 {
            ::core::ptr::copy_nonoverlapping(proxyhost.to_be_bytes().as_ptr(), masterinfo, 4);
            ::core::ptr::copy_nonoverlapping(
                proxyport.to_be_bytes().as_ptr(),
                masterinfo.add(4),
                2,
            );
        }
    }
}

/// SAFETY: cd owned by the two connection threads; called once via the
/// keepalive thread when sendnops goes 255.
unsafe fn free_conn_data(cd: *mut ConnData) {
    unsafe {
        tcpclose((*cd).sock);
        drop(Box::from_raw(cd));
    }
}

unsafe extern "C" fn masterproxy_keepalive(
    args: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let cd = args as *mut ConnData;
        let nopbuff = imp::nop_packet();
        let mut nopcnt: u32 = 0;
        loop {
            let state = (*cd).sendnops.load(Ordering::SeqCst);
            if state == 255 {
                free_conn_data(cd);
                return NULL;
            }
            if state == 0 {
                nopcnt = 0;
            } else {
                nopcnt += 1;
                if imp::nop_due(nopcnt) {
                    (*cd).sendnops.store(2, Ordering::SeqCst);
                    if tcptowrite(
                        (*cd).sock,
                        nopbuff.as_ptr() as *const ::core::ffi::c_void,
                        8,
                        TOOLTIMEOUTPARTMS,
                        TOOLTIMEOUTALLMS,
                    ) != 8
                    {
                        break;
                    }
                    (*cd).sendnops.store(1, Ordering::SeqCst);
                    nopcnt = 0;
                }
            }
            portable_usleep(100000);
        }
        NULL
    }
}

unsafe extern "C" fn masterproxy_server(
    args: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let cd = args as *mut ConnData;
        let mut header = [0u8; 8];
        let auxbuffer = libc::malloc(AUXBUFFSIZE) as *mut uint8_t;
        if auxbuffer.is_null() {
            (*cd).sendnops.store(255, Ordering::SeqCst);
            return NULL;
        }
        loop {
            if tcptoread(
                (*cd).sock,
                header.as_mut_ptr() as *mut ::core::ffi::c_void,
                8,
                TOOLTIMEOUTPARTMS,
                TOOLTIMEOUTALLMS,
            ) != 8
            {
                break;
            }
            let cmd = imp::get32(&header[..4]);
            let psize = imp::get32(&header[4..]);
            if cmd == CLTOMA_FUSE_REGISTER {
                if psize != 73 {
                    break;
                }
                if tcptoread(
                    (*cd).sock,
                    auxbuffer as *mut ::core::ffi::c_void,
                    psize,
                    TOOLTIMEOUTPARTMS,
                    TOOLTIMEOUTALLMS,
                ) != psize as ::core::ffi::c_int
                {
                    break;
                }
                let body = ::core::slice::from_raw_parts(auxbuffer, psize as usize);
                if !imp::check_register_body(body) {
                    break;
                }
                let reply = imp::register_reply();
                if tcptowrite(
                    (*cd).sock,
                    reply.as_ptr() as *const ::core::ffi::c_void,
                    9,
                    TOOLTIMEOUTPARTMS,
                    TOOLTIMEOUTALLMS,
                ) != 9
                {
                    break;
                }
            } else {
                if psize < 4 || psize as usize > AUXBUFFSIZE {
                    break;
                }
                if tcptoread(
                    (*cd).sock,
                    auxbuffer as *mut ::core::ffi::c_void,
                    psize,
                    TOOLTIMEOUTPARTMS,
                    TOOLTIMEOUTALLMS,
                ) != psize as ::core::ffi::c_int
                {
                    break;
                }
                (*cd).sendnops.store(1, Ordering::SeqCst);
                let body = ::core::slice::from_raw_parts(auxbuffer, psize as usize);
                let msgid = imp::get32(&body[..4]);
                let snapshot_dst = if cmd == CLTOMA_FUSE_SNAPSHOT {
                    imp::parse_snapshot_dst(body)
                } else {
                    None
                };
                let mut acmd: uint32_t = 0;
                let mut aptr: *const uint8_t = ::core::ptr::null();
                let mut asize: uint32_t = 0;
                if fs_custom(
                    cmd,
                    auxbuffer.add(4),
                    psize - 4,
                    &raw mut acmd,
                    &raw mut aptr,
                    &raw mut asize,
                ) != MFS_STATUS_OK
                {
                    break;
                }
                if cmd == CLTOMA_FUSE_SNAPSHOT && acmd == MATOCL_FUSE_SNAPSHOT {
                    negentry_cache_clear();
                    if let Some((inode_dst, name)) = snapshot_dst {
                        let mut cname = Vec::with_capacity(name.len() + 1);
                        cname.extend_from_slice(&name);
                        cname.push(0);
                        mfs_dentry_invalidate(
                            inode_dst,
                            name.len() as uint8_t,
                            cname.as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                }
                let ahdr = imp::answer_header(acmd, asize, msgid);
                // C's active wait: don't interleave the answer with a NOP
                while (*cd).sendnops.load(Ordering::SeqCst) == 2 {
                    portable_usleep(10000);
                }
                (*cd).sendnops.store(0, Ordering::SeqCst);
                if tcptowrite(
                    (*cd).sock,
                    ahdr.as_ptr() as *const ::core::ffi::c_void,
                    12,
                    TOOLTIMEOUTPARTMS,
                    TOOLTIMEOUTALLMS,
                ) != 12
                {
                    break;
                }
                if tcptowrite(
                    (*cd).sock,
                    aptr as *const ::core::ffi::c_void,
                    asize,
                    TOOLTIMEOUTPARTMS,
                    TOOLTIMEOUTALLMS,
                ) != asize as ::core::ffi::c_int
                {
                    break;
                }
            }
        }
        libc::free(auxbuffer as *mut ::core::ffi::c_void);
        (*cd).sendnops.store(255, Ordering::SeqCst);
        NULL
    }
}

/// block the usual termination signals around thread creation (C pattern)
unsafe fn sigmask_block(oldset: *mut sigset_t) {
    unsafe {
        let mut newset: sigset_t = ::core::mem::zeroed();
        sigemptyset(&raw mut newset);
        sigaddset(&raw mut newset, SIGTERM);
        sigaddset(&raw mut newset, SIGINT);
        sigaddset(&raw mut newset, SIGHUP);
        sigaddset(&raw mut newset, SIGQUIT);
        pthread_sigmask(SIG_BLOCK, &raw const newset, oldset);
    }
}

unsafe extern "C" fn masterproxy_acceptor(
    _args: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut thattr: pthread_attr_t = ::core::mem::zeroed();
        pthread_attr_init(&raw mut thattr);
        pthread_attr_setstacksize(&raw mut thattr, 0x100000);
        pthread_attr_setdetachstate(&raw mut thattr, PTHREAD_CREATE_DETACHED);
        while TERMINATE.load(Ordering::SeqCst) == 0 {
            let sock = tcptoaccept(lsock, 1000);
            if sock >= 0 {
                let cd = Box::into_raw(Box::new(ConnData {
                    sock,
                    sendnops: AtomicU8::new(0),
                }));
                tcpnodelay(sock);
                tcpnonblock(sock);
                let mut oldset: sigset_t = ::core::mem::zeroed();
                sigmask_block(&raw mut oldset);
                let mut nopthread: pthread_t = 0;
                let res = pthread_create(
                    &raw mut nopthread,
                    &raw const thattr,
                    Some(masterproxy_keepalive),
                    cd as *mut ::core::ffi::c_void,
                );
                if res != 0 {
                    free_conn_data(cd);
                } else {
                    let mut clientthread: pthread_t = 0;
                    let res = pthread_create(
                        &raw mut clientthread,
                        &raw const thattr,
                        Some(masterproxy_server),
                        cd as *mut ::core::ffi::c_void,
                    );
                    if res != 0 {
                        (*cd).sendnops.store(255, Ordering::SeqCst);
                    }
                }
                let mut nullset: sigset_t = ::core::mem::zeroed();
                pthread_sigmask(SIG_SETMASK, &raw const oldset, &raw mut nullset);
            }
        }
        pthread_attr_destroy(&raw mut thattr);
        NULL
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterproxy_term() {
    unsafe {
        TERMINATE.store(1, Ordering::SeqCst);
        pthread_join(
            proxythread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterproxy_init(
    masterproxyip: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        lsock = tcpsocket();
        if lsock < 0 {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"master proxy module: can't create socket\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1;
        }
        tcpnonblock(lsock);
        tcpnodelay(lsock);
        if tcpstrlisten(lsock, masterproxyip, 0, 100) < 0 {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"master proxy module: can't listen on socket\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            tcpclose(lsock);
            lsock = -1;
            return -1;
        }
        if tcpsetacceptfilter(lsock) < 0 && *__errno_location() != ENOTSUP {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_NOTICE,
                b"master proxy module: can't set accept filter\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if tcpgetmyaddr(lsock, &raw mut proxyhost, &raw mut proxyport) < 0 {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"master proxy module: can't obtain my address and port\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            tcpclose(lsock);
            lsock = -1;
            return -1;
        }
        TERMINATE.store(0, Ordering::SeqCst);
        let mut thattr: pthread_attr_t = ::core::mem::zeroed();
        pthread_attr_init(&raw mut thattr);
        pthread_attr_setstacksize(&raw mut thattr, 0x100000);
        let mut oldset: sigset_t = ::core::mem::zeroed();
        sigmask_block(&raw mut oldset);
        pthread_create(
            &raw mut proxythread,
            &raw const thattr,
            Some(masterproxy_acceptor),
            NULL,
        );
        let mut nullset: sigset_t = ::core::mem::zeroed();
        pthread_sigmask(SIG_SETMASK, &raw const oldset, &raw mut nullset);
        pthread_attr_destroy(&raw mut thattr);
        1
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;
    use std::vec;
    use std::vec::Vec;

    #[test]
    fn register_body_validation() {
        let mut good = Vec::new();
        good.extend_from_slice(FUSE_REGISTER_BLOB_ACL);
        good.push(4); // REGISTER_TOOLS
        good.extend_from_slice(&[0u8; 8]); // trailing to 73
        assert_eq!(good.len(), 73);
        assert!(check_register_body(&good));
        let mut bad_magic = good.clone();
        bad_magic[0] = b'X';
        assert!(!check_register_body(&bad_magic));
        let mut bad_type = good.clone();
        bad_type[64] = 5;
        assert!(!check_register_body(&bad_type));
        assert!(!check_register_body(&good[..72]));
        assert!(!check_register_body(&[0u8; 74]));
    }

    #[test]
    fn register_reply_bytes() {
        let r = register_reply();
        assert_eq!(&r[..4], &401u32.to_be_bytes());
        assert_eq!(&r[4..8], &1u32.to_be_bytes());
        assert_eq!(r[8], 0);
    }

    #[test]
    fn snapshot_dst_parsing_bounds() {
        // msgid(4) src(4) dst(4) namelen(1) name
        let mut body = Vec::new();
        body.extend_from_slice(&0xAAu32.to_be_bytes()); // msgid
        body.extend_from_slice(&1u32.to_be_bytes()); // src
        body.extend_from_slice(&77u32.to_be_bytes()); // dst
        body.push(5);
        body.extend_from_slice(b"hello");
        assert_eq!(parse_snapshot_dst(&body), Some((77, b"hello".to_vec())));
        // truncated name → None (C resets inode_dst)
        let trunc = &body[..body.len() - 2];
        assert_eq!(parse_snapshot_dst(trunc), None);
        // namelen 0 → None
        let mut z = body.clone();
        z[12] = 0;
        assert_eq!(parse_snapshot_dst(&z[..13]), None);
        // dst inode 0 → None
        let mut d0 = body.clone();
        d0[8..12].copy_from_slice(&0u32.to_be_bytes());
        assert_eq!(parse_snapshot_dst(&d0), None);
        // too short overall
        assert_eq!(parse_snapshot_dst(&body[..10]), None);
    }

    #[test]
    fn answer_header_bytes() {
        let h = answer_header(469, 20, 0xDEAD);
        assert_eq!(&h[..4], &469u32.to_be_bytes());
        assert_eq!(&h[4..8], &24u32.to_be_bytes());
        assert_eq!(&h[8..], &0xDEADu32.to_be_bytes());
        let _ = vec![0u8];
    }

    #[test]
    fn nop_due_threshold() {
        assert!(!nop_due(49));
        assert!(nop_due(50));
        assert!(nop_due(51));
    }
}
