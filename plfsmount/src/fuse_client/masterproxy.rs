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
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOTSUP: ::core::ffi::c_int = 95;
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
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

static mut lsock: ::core::ffi::c_int = -1;
static PROXY_THREAD: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
static TERMINATE: AtomicU8 = AtomicU8::new(0);
static mut proxyhost: uint32_t = 0;
static mut proxyport: uint16_t = 0;

struct ConnData {
    sock: ::core::ffi::c_int,
    sendnops: AtomicU8, // 0 idle / 1 in-request / 2 nop-in-flight / 255 dead
}

impl Drop for ConnData {
    fn drop(&mut self) {
        // SAFETY: ConnData owns the accepted socket.
        unsafe { tcpclose(self.sock) };
    }
}

#[inline]
fn portable_usleep(usec: u64) {
    std::thread::sleep(std::time::Duration::from_micros(usec));
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

fn masterproxy_keepalive(cd: Arc<ConnData>) {
    unsafe {
        let nopbuff = imp::nop_packet();
        let mut nopcnt: u32 = 0;
        loop {
            let state = (*cd).sendnops.load(Ordering::SeqCst);
            if state == 255 {
                return;
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
    }
}

fn masterproxy_server(cd: Arc<ConnData>) {
    unsafe {
        let mut header = [0u8; 8];
        // C: malloc(AUXBUFFSIZE) + free at fn end; Vec-owned, dropped at scope
        // exit. try_reserve keeps the C graceful-OOM path (sendnops=255).
        let mut auxv: Vec<u8> = Vec::new();
        if auxv.try_reserve_exact(AUXBUFFSIZE).is_err() {
            (*cd).sendnops.store(255, Ordering::SeqCst);
            return;
        }
        auxv.resize(AUXBUFFSIZE, 0);
        let auxbuffer = auxv.as_mut_ptr();
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
        (*cd).sendnops.store(255, Ordering::SeqCst);
    }
}

fn masterproxy_acceptor() {
    loop {
        if TERMINATE.load(Ordering::SeqCst) != 0 {
            return;
        }
        // SAFETY: listening socket is initialized before this worker starts.
        let sock = unsafe { tcptoaccept(lsock, 1000) };
        if sock < 0 {
            continue;
        }
        let cd = Arc::new(ConnData {
            sock,
            sendnops: AtomicU8::new(0),
        });
        // SAFETY: accepted socket is owned by cd.
        unsafe {
            tcpnodelay(sock);
            tcpnonblock(sock);
        }
        let keepalive =
            plfscommon::lwthread::spawn_with_stack("masterproxy-keepalive", 0x100000, {
                let cd = Arc::clone(&cd);
                move || masterproxy_keepalive(cd)
            });
        if keepalive.is_err() {
            continue;
        }
        if plfscommon::lwthread::spawn_with_stack("masterproxy-server", 0x100000, {
            let cd = Arc::clone(&cd);
            move || masterproxy_server(cd)
        })
        .is_err()
        {
            cd.sendnops.store(255, Ordering::SeqCst);
        }
        // Keepalive handle is intentionally detached; it owns its Arc until
        // it observes the server's terminal state.
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterproxy_term() {
    TERMINATE.store(1, Ordering::SeqCst);
    if let Some(thread) = PROXY_THREAD.lock().unwrap().take() {
        thread.join().expect("master proxy acceptor panicked");
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
        let thread = match plfscommon::lwthread::spawn_with_stack(
            "masterproxy-acceptor",
            0x100000,
            masterproxy_acceptor,
        ) {
            Ok(thread) => thread,
            Err(_) => {
                tcpclose(lsock);
                lsock = -1;
                return -1;
            }
        };
        *PROXY_THREAD.lock().unwrap() = Some(thread);
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
