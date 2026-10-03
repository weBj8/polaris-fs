//! Acid's simple socket library (ver 6.0) — port of mfscommon/sockets.c.
//!
//! Layout:
//! - `sys`: the syscall boundary — one small safe wrapper per libc call
//!   (read/write/poll/accept/connect/bind/...), each an annotated unsafe
//!   block over plain fd + buffer arguments;
//! - `imp` (`#[deny(unsafe_code)]`): all logic — the timed stream loops
//!   (`streamtoread/towrite/toforward/towait/toaccept`), the non-blocking
//!   connect wait, resolver entry selection and address formatting —
//!   written against `sys`, step for step with the C;
//! - the exported C ABI (sockets.h signatures), which only turns raw
//!   pointers into slices and out-params.
//!
//! C quirks kept: elapsed times are `(double seconds)*1000.0` truncated to
//! `uint32_t`; poll timeouts are `uint32_t` passed as `int` (values above
//! INT_MAX wait forever); `errno` is the error channel (ETIMEDOUT,
//! ECONNRESET, ...); resolver picks `random()%n` among matching entries
//! (consuming the libc PRNG exactly when C does).
//!
//! One deviation where C is undefined: `udpread` passed an uninitialized
//! `socklen_t` to `recvfrom`; here it is initialized to the buffer size.

use core::ffi::{CStr, c_char, c_int, c_void};

use crate::clocks::monotonic_seconds;

pub const STRIPSIZE: usize = 16;
pub const STRIPPORTSIZE: usize = 32;

/// Syscall boundary: thin safe wrappers, errno is left as the kernel set it.
mod sys {
    use core::ffi::{CStr, c_int, c_void};

    pub fn errno() -> c_int {
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
    }

    pub fn set_errno(e: c_int) {
        // SAFETY: thread-local errno location is always valid.
        unsafe { *libc::__errno_location() = e };
    }

    pub fn read(fd: c_int, buf: &mut [u8]) -> isize {
        // SAFETY: buf is valid for writes of buf.len() bytes.
        unsafe { libc::read(fd, buf.as_mut_ptr() as *mut c_void, buf.len()) }
    }

    pub fn write(fd: c_int, buf: &[u8]) -> isize {
        // SAFETY: buf is valid for reads of buf.len() bytes.
        unsafe { libc::write(fd, buf.as_ptr() as *const c_void, buf.len()) }
    }

    pub fn writev(fd: c_int, bufs: &[&[u8]]) -> isize {
        let iov: Vec<libc::iovec> = bufs
            .iter()
            .map(|b| libc::iovec { iov_base: b.as_ptr() as *mut c_void, iov_len: b.len() })
            .collect();
        // Each iovec describes a live slice borrowed for the call.
        // SAFETY: writev only reads the described, live buffers.
        unsafe { libc::writev(fd, iov.as_ptr(), iov.len() as c_int) }
    }

    pub fn poll(fds: &mut [libc::pollfd], timeout: c_int) -> c_int {
        // SAFETY: fds is a valid array of fds.len() pollfd entries.
        unsafe { libc::poll(fds.as_mut_ptr(), fds.len() as libc::nfds_t, timeout) }
    }

    pub fn accept(lsock: c_int) -> c_int {
        // SAFETY: NULL address/len is allowed by accept(2).
        unsafe { libc::accept(lsock, core::ptr::null_mut(), core::ptr::null_mut()) }
    }

    pub fn socket(domain: c_int, ty: c_int) -> c_int {
        // SAFETY: no memory arguments.
        unsafe { libc::socket(domain, ty, 0) }
    }

    pub fn close(fd: c_int) -> c_int {
        // SAFETY: no memory arguments.
        unsafe { libc::close(fd) }
    }

    pub fn shutdown(fd: c_int, how: c_int) -> c_int {
        // SAFETY: no memory arguments.
        unsafe { libc::shutdown(fd, how) }
    }

    pub fn listen(fd: c_int, queue: c_int) -> c_int {
        // SAFETY: no memory arguments.
        unsafe { libc::listen(fd, queue) }
    }

    pub fn nonblock(fd: c_int) -> c_int {
        // SAFETY: fcntl F_GETFL/F_SETFL take integer arguments.
        unsafe {
            let flags = libc::fcntl(fd, libc::F_GETFL, 0);
            if flags == -1 {
                return -1;
            }
            libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK)
        }
    }

    pub fn setsockopt_int(fd: c_int, level: c_int, name: c_int, v: c_int) -> c_int {
        // SAFETY: option value is a local int of the stated size.
        unsafe {
            libc::setsockopt(
                fd,
                level,
                name,
                &v as *const c_int as *const c_void,
                core::mem::size_of::<c_int>() as libc::socklen_t,
            )
        }
    }

    /// C `sockgetstatus`: SO_ERROR, stored into errno and returned.
    pub fn getstatus(fd: c_int) -> c_int {
        let mut rc: c_int = 0;
        let mut len = core::mem::size_of::<c_int>() as libc::socklen_t;
        // SAFETY: rc/len are locals of the sizes passed.
        let r = unsafe {
            libc::getsockopt(fd, libc::SOL_SOCKET, libc::SO_ERROR, &mut rc as *mut c_int as *mut c_void, &mut len)
        };
        if r < 0 {
            rc = errno();
        }
        set_errno(rc);
        rc
    }

    /// C `sockaddrnumfill`.
    pub fn sin(ip: u32, port: u16) -> libc::sockaddr_in {
        libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: port.to_be(),
            sin_addr: libc::in_addr { s_addr: ip.to_be() },
            sin_zero: [0; 8],
        }
    }

    pub fn connect_in(fd: c_int, sa: &libc::sockaddr_in) -> c_int {
        // SAFETY: sa is a valid sockaddr_in of the stated length.
        unsafe {
            libc::connect(
                fd,
                sa as *const libc::sockaddr_in as *const libc::sockaddr,
                core::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
            )
        }
    }

    pub fn bind_in(fd: c_int, sa: &libc::sockaddr_in) -> c_int {
        // SAFETY: sa is a valid sockaddr_in of the stated length.
        unsafe {
            libc::bind(
                fd,
                sa as *const libc::sockaddr_in as *const libc::sockaddr,
                core::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
            )
        }
    }

    /// C `sockaddrpathfill`: None when the path does not fit `sun_path`.
    pub fn sun(path: &[u8]) -> Option<libc::sockaddr_un> {
        // SAFETY: sockaddr_un is plain data; all-zero is valid.
        let mut sa: libc::sockaddr_un = unsafe { core::mem::zeroed() };
        if path.len() >= sa.sun_path.len() {
            return None;
        }
        sa.sun_family = libc::AF_LOCAL as libc::sa_family_t;
        for (d, &s) in sa.sun_path.iter_mut().zip(path) {
            *d = s as libc::c_char;
        }
        Some(sa)
    }

    pub fn connect_un(fd: c_int, sa: &libc::sockaddr_un) -> c_int {
        // SAFETY: sa is a valid sockaddr_un of the stated length.
        unsafe {
            libc::connect(
                fd,
                sa as *const libc::sockaddr_un as *const libc::sockaddr,
                core::mem::size_of::<libc::sockaddr_un>() as libc::socklen_t,
            )
        }
    }

    pub fn bind_un(fd: c_int, sa: &libc::sockaddr_un) -> c_int {
        // SAFETY: sa is a valid sockaddr_un of the stated length.
        unsafe {
            libc::bind(
                fd,
                sa as *const libc::sockaddr_un as *const libc::sockaddr,
                core::mem::size_of::<libc::sockaddr_un>() as libc::socklen_t,
            )
        }
    }

    /// getpeername/getsockname as (ip, port) in host order.
    pub fn name(fd: c_int, peer: bool) -> Option<(u32, u16)> {
        let mut sa = sin(0, 0);
        let mut len = core::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;
        let p = &mut sa as *mut libc::sockaddr_in as *mut libc::sockaddr;
        // SAFETY: sa/len are locals of the sizes passed.
        let r = unsafe { if peer { libc::getpeername(fd, p, &mut len) } else { libc::getsockname(fd, p, &mut len) } };
        (r >= 0).then(|| (u32::from_be(sa.sin_addr.s_addr), u16::from_be(sa.sin_port)))
    }

    /// getaddrinfo: IPv4 entries matching (family, socktype) with a
    /// sockaddr_in-sized address, in resolver order. None = lookup failed.
    pub fn addrinfo(
        host: Option<&CStr>,
        service: Option<&CStr>,
        family: c_int,
        socktype: c_int,
        passive: bool,
    ) -> Option<Vec<libc::sockaddr_in>> {
        // SAFETY: hints is zeroed plain data; getaddrinfo's result list is
        // walked read-only and freed exactly once.
        unsafe {
            let mut hints: libc::addrinfo = core::mem::zeroed();
            hints.ai_family = family;
            hints.ai_socktype = socktype;
            if passive {
                hints.ai_flags = libc::AI_PASSIVE;
            }
            let mut head: *mut libc::addrinfo = core::ptr::null_mut();
            let h = host.map_or(core::ptr::null(), |c| c.as_ptr());
            let s = service.map_or(core::ptr::null(), |c| c.as_ptr());
            if libc::getaddrinfo(h, s, &hints, &mut head) != 0 {
                return None;
            }
            let mut out = Vec::new();
            let mut r = head;
            while !r.is_null() {
                let ai = &*r;
                if ai.ai_family == family
                    && ai.ai_socktype == socktype
                    && ai.ai_addrlen as usize == core::mem::size_of::<libc::sockaddr_in>()
                {
                    out.push(*(ai.ai_addr as *const libc::sockaddr_in));
                }
                r = ai.ai_next;
            }
            libc::freeaddrinfo(head);
            Some(out)
        }
    }

    /// libc `random()` (the PRNG shared with rnd_init's srandom seed).
    pub fn random() -> libc::c_long {
        unsafe extern "C" {
            fn random() -> libc::c_long;
        }
        // SAFETY: no arguments; glibc random() is thread-safe.
        unsafe { random() }
    }

    pub fn sendto_in(fd: c_int, buf: &[u8], sa: &libc::sockaddr_in) -> c_int {
        // SAFETY: buf and sa are valid for the lengths passed.
        unsafe {
            libc::sendto(
                fd,
                buf.as_ptr() as *const c_void,
                buf.len(),
                0,
                sa as *const libc::sockaddr_in as *const libc::sockaddr,
                core::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t,
            ) as c_int
        }
    }

    /// recvfrom into `buf`; returns (ret, source if it is a sockaddr_in).
    pub fn recvfrom_in(fd: c_int, buf: &mut [u8]) -> (c_int, Option<(u32, u16)>) {
        // SAFETY: sockaddr is plain data; all-zero is valid.
        let mut sa: libc::sockaddr = unsafe { core::mem::zeroed() };
        let mut len = core::mem::size_of::<libc::sockaddr>() as libc::socklen_t;
        // SAFETY: buf/sa/len are valid for the lengths passed.
        let ret = unsafe {
            libc::recvfrom(fd, buf.as_mut_ptr() as *mut c_void, buf.len(), 0, &mut sa, &mut len) as c_int
        };
        let src = (len as usize == core::mem::size_of::<libc::sockaddr_in>()).then(|| {
            // SAFETY: the kernel filled a sockaddr_in (length checked);
            // sockaddr is at least as large and suitably aligned.
            let sin = unsafe { &*(&sa as *const libc::sockaddr as *const libc::sockaddr_in) };
            (u32::from_be(sin.sin_addr.s_addr), u16::from_be(sin.sin_port))
        });
        (ret, src)
    }
}

pub use imp::{fmt_ip, fmt_ip_port};

/// Safe Rust-facing API for migrated modules: the same code paths as the
/// C exports below (`tcpsocket`, `tcpresolve`, ...), with fd + slice
/// arguments. errno stays the error channel, as in C.
pub mod tcp {
    use super::{imp, sys};
    use core::ffi::{CStr, c_int};

    pub use super::sys::{errno, set_errno};

    /// `tcpsocket`.
    pub fn socket() -> c_int {
        sys::socket(libc::AF_INET, libc::SOCK_STREAM)
    }
    /// `tcpnonblock`.
    pub fn nonblock(sock: c_int) -> c_int {
        sys::nonblock(sock)
    }
    /// `tcpnodelay`.
    pub fn nodelay(sock: c_int) -> c_int {
        sys::setsockopt_int(sock, libc::IPPROTO_TCP, libc::TCP_NODELAY, 1)
    }
    /// `tcpgetstatus`.
    pub fn getstatus(sock: c_int) -> c_int {
        sys::getstatus(sock)
    }
    /// `tcpclose` (shutdown write side, then close).
    pub fn close(sock: c_int) -> c_int {
        sys::shutdown(sock, libc::SHUT_WR);
        sys::close(sock)
    }
    /// `tcpnumbind`.
    pub fn numbind(sock: c_int, ip: u32, port: u16) -> c_int {
        if sys::bind_in(sock, &sys::sin(ip, port)) < 0 { -1 } else { 0 }
    }
    /// `tcpnumconnect`: 0 connected, 1 in progress, -1 error.
    pub fn numconnect(sock: c_int, ip: u32, port: u16) -> c_int {
        imp::connect_status(sys::connect_in(sock, &sys::sin(ip, port)))
    }
    /// `tcpresolve` (`*` = any, `random()%n` among matches): (ip, port).
    pub fn resolve(host: Option<&CStr>, service: Option<&CStr>, passive: bool) -> Option<(u32, u16)> {
        imp::addrfill(host, service, libc::AF_INET, libc::SOCK_STREAM, passive)
            .map(|sa| (u32::from_be(sa.sin_addr.s_addr), u16::from_be(sa.sin_port)))
    }
    /// One `read(2)`.
    pub fn read(sock: c_int, buf: &mut [u8]) -> isize {
        sys::read(sock, buf)
    }
    /// One `writev(2)` over the given slices (at most IOV_MAX of them).
    pub fn writev(sock: c_int, bufs: &[&[u8]]) -> isize {
        sys::writev(sock, bufs)
    }
}

#[deny(unsafe_code)]
mod imp {
    use super::sys;
    use super::{CStr, c_int, monotonic_seconds};
    use libc::{
        EAGAIN, ECONNRESET, EINPROGRESS, EINTR, ETIMEDOUT, POLLERR, POLLHUP, POLLIN, POLLOUT, pollfd,
    };

    /// `(uint32_t)((c-s)*1000.0)`.
    fn ms(d: f64) -> u32 {
        (d * 1000.0) as u32
    }

    /// C `ERRNO_ERROR` (Linux: EWOULDBLOCK == EAGAIN).
    fn errno_error() -> bool {
        sys::errno() != EAGAIN
    }

    fn pfd(fd: c_int, events: i16) -> pollfd {
        pollfd { fd, events, revents: 0 }
    }

    /// Shared per-part / total deadline bookkeeping of the stream loops.
    struct Clock {
        s: f64,
        c: f64,
    }

    impl Clock {
        fn new() -> Self {
            Clock { s: 0.0, c: 0.0 }
        }
        /// Returns msecpassed, or Err after setting ETIMEDOUT.
        fn step(&mut self, msectopart: u32, msectoall: u32) -> Result<u32, ()> {
            if self.s == 0.0 {
                self.s = monotonic_seconds();
                self.c = self.s;
                return Ok(0);
            }
            let l = self.c;
            self.c = monotonic_seconds();
            if ms(self.c - l) >= msectopart {
                sys::set_errno(ETIMEDOUT);
                return Err(());
            }
            let passed = ms(self.c - self.s);
            if passed >= msectoall {
                sys::set_errno(ETIMEDOUT);
                return Err(());
            }
            Ok(passed)
        }
    }

    fn poll_timeout(msectopart: u32, msectoall: u32, passed: u32) -> c_int {
        let mut msecpoll = msectoall.wrapping_sub(passed);
        if msectopart < msecpoll {
            msecpoll = msectopart;
        }
        msecpoll as c_int
    }

    /// C `streamtowait`.
    pub fn towait(sock: c_int, msectoall: u32) -> c_int {
        let (mut s, mut msecpassed) = (0.0f64, 0u32);
        let mut p = [pfd(sock, POLLIN)];
        loop {
            if s == 0.0 {
                s = monotonic_seconds();
                msecpassed = 0;
            } else {
                msecpassed = ms(monotonic_seconds() - s);
                if msecpassed >= msectoall {
                    sys::set_errno(ETIMEDOUT);
                    return -1;
                }
            }
            p[0].revents = 0;
            if sys::poll(&mut p, msectoall.wrapping_sub(msecpassed) as c_int) < 0 {
                if sys::errno() != EINTR {
                    return -1;
                }
                continue;
            }
            if p[0].revents & (POLLHUP | POLLIN) != 0 {
                return 0;
            }
            if p[0].revents & POLLERR != 0 {
                return -1;
            }
            sys::set_errno(ETIMEDOUT);
            return -1;
        }
    }

    /// C `streamtoread`.
    pub fn toread(sock: c_int, buff: &mut [u8], msectopart: u32, msectoall: u32) -> i32 {
        let leng = buff.len() as u32;
        let mut rcvd: u32 = 0;
        let mut clock = Clock::new();
        let mut p = [pfd(sock, POLLIN)];
        loop {
            let i = sys::read(sock, &mut buff[rcvd as usize..]) as c_int;
            if i == 0 {
                sys::set_errno(ECONNRESET);
                return rcvd as i32;
            }
            if i > 0 {
                rcvd = rcvd.wrapping_add(i as u32);
            } else if errno_error() {
                return -1;
            }
            if p[0].revents & POLLHUP != 0 {
                sys::set_errno(ECONNRESET);
                return rcvd as i32;
            }
            if rcvd >= leng {
                break;
            }
            let Ok(passed) = clock.step(msectopart, msectoall) else { return -1 };
            p[0].revents = 0;
            if sys::poll(&mut p, poll_timeout(msectopart, msectoall, passed)) < 0 {
                if sys::errno() != EINTR {
                    return -1;
                }
                continue;
            }
            if p[0].revents & POLLERR != 0 {
                return -1;
            }
            if p[0].revents & POLLIN == 0 {
                sys::set_errno(ETIMEDOUT);
                return -1;
            }
        }
        rcvd as i32
    }

    /// C `streamtowrite`.
    pub fn towrite(sock: c_int, buff: &[u8], msectopart: u32, msectoall: u32) -> i32 {
        let leng = buff.len() as u32;
        let mut sent: u32 = 0;
        let mut clock = Clock::new();
        let mut p = [pfd(sock, POLLOUT)];
        loop {
            let i = sys::write(sock, &buff[sent as usize..]) as i32;
            if i >= 0 {
                sent = sent.wrapping_add(i as u32);
            } else if errno_error() {
                return -1;
            }
            if sent >= leng {
                break;
            }
            let Ok(passed) = clock.step(msectopart, msectoall) else { return -1 };
            p[0].revents = 0;
            if sys::poll(&mut p, poll_timeout(msectopart, msectoall, passed)) < 0 {
                if sys::errno() != EINTR {
                    return -1;
                }
                continue;
            }
            if p[0].revents & (POLLHUP | POLLERR) != 0 {
                return -1;
            }
            if p[0].revents & POLLOUT == 0 {
                sys::set_errno(ETIMEDOUT);
                return -1;
            }
        }
        sent as i32
    }

    /// C `streamtoforward`: copy `buff.len()` bytes src→dst through buff,
    /// resuming from (rcvd, sent). Returns the forwarded length.
    #[allow(clippy::too_many_arguments)]
    pub fn toforward(
        src: c_int,
        dst: c_int,
        buff: &mut [u8],
        mut rcvd: u32,
        mut sent: u32,
        msectopart: u32,
        msectoall: u32,
    ) -> i32 {
        let mut leng = buff.len() as u32;
        let mut clock = Clock::new();
        let mut p = [pfd(src, POLLIN), pfd(dst, POLLOUT)];
        loop {
            if rcvd < leng {
                let i = sys::read(src, &mut buff[rcvd as usize..leng as usize]) as i32;
                if i == 0 {
                    leng = rcvd;
                }
                if i > 0 {
                    rcvd = rcvd.wrapping_add(i as u32);
                } else if errno_error() {
                    return -1;
                }
            }
            if p[0].revents & POLLHUP != 0 {
                leng = rcvd;
            }
            if rcvd > sent {
                let i = sys::write(dst, &buff[sent as usize..rcvd as usize]) as i32;
                if i >= 0 {
                    sent = sent.wrapping_add(i as u32);
                } else if errno_error() {
                    return -1;
                }
            }
            if rcvd >= leng && sent >= leng {
                break;
            }
            let Ok(passed) = clock.step(msectopart, msectoall) else { return -1 };
            p[0].revents = 0;
            p[1].revents = 0;
            let t = poll_timeout(msectopart, msectoall, passed);
            if rcvd == leng {
                // only wait for write
                if sys::poll(&mut p[1..], t) < 0 {
                    if sys::errno() != EINTR {
                        return -1;
                    }
                    continue;
                }
                if p[1].revents & (POLLERR | POLLHUP) != 0 {
                    return -1;
                }
                p[0].revents = 0;
            } else if rcvd == sent {
                // only wait for read
                if sys::poll(&mut p[..1], t) < 0 {
                    if sys::errno() != EINTR {
                        return -1;
                    }
                    continue;
                }
                if p[0].revents & POLLERR != 0 {
                    return -1;
                }
                p[1].revents = 0;
            } else {
                if sys::poll(&mut p, t) < 0 {
                    if sys::errno() != EINTR {
                        return -1;
                    }
                    continue;
                }
                if p[0].revents & POLLERR != 0 || p[1].revents & (POLLERR | POLLHUP) != 0 {
                    return -1;
                }
            }
            if p[0].revents & (POLLIN | POLLHUP) == 0 && p[1].revents & POLLOUT == 0 {
                sys::set_errno(ETIMEDOUT);
                return -1;
            }
        }
        leng as i32
    }

    /// Poll `fd` for `events` until it fires, retrying EINTR against the
    /// total budget (the shared wait of toaccept/toconnect). Ok(revents).
    fn wait_once(fd: c_int, events: i16, msecto: u32) -> Result<i16, ()> {
        let s = monotonic_seconds();
        let mut msecpassed: u32 = 0;
        loop {
            let mut p = [pfd(fd, events)];
            if sys::poll(&mut p, msecto.wrapping_sub(msecpassed) as c_int) >= 0 {
                return Ok(p[0].revents);
            }
            if sys::errno() != EINTR {
                return Err(());
            }
            msecpassed = ms(monotonic_seconds() - s);
            if msecpassed >= msecto {
                sys::set_errno(ETIMEDOUT);
                return Err(());
            }
        }
    }

    /// C `streamtoaccept`.
    pub fn toaccept(lsock: c_int, msecto: u32) -> c_int {
        let i = sys::accept(lsock);
        if i >= 0 {
            return i;
        }
        if errno_error() {
            return -1;
        }
        let Ok(rev) = wait_once(lsock, POLLIN, msecto) else { return -1 };
        if rev & (POLLHUP | POLLERR) != 0 {
            return -1;
        }
        if rev & POLLIN != 0 {
            return sys::accept(lsock);
        }
        sys::set_errno(ETIMEDOUT);
        -1
    }

    /// Result of a `connect()` call, mapped like tcp*connect: 0 done,
    /// 1 in progress, -1 error.
    pub fn connect_status(r: c_int) -> c_int {
        if r >= 0 {
            0
        } else if sys::errno() == EINPROGRESS {
            1
        } else {
            -1
        }
    }

    /// The `*toconnect` tail after a non-blocking `connect()`.
    pub fn finish_toconnect(sock: c_int, r: c_int, msecto: u32) -> c_int {
        if r >= 0 {
            return 0;
        }
        if sys::errno() != EINPROGRESS {
            return -1;
        }
        let Ok(rev) = wait_once(sock, POLLOUT, msecto) else { return -1 };
        if rev & (POLLHUP | POLLERR) != 0 {
            return -1;
        }
        if rev & POLLOUT != 0 {
            return sys::getstatus(sock);
        }
        sys::set_errno(ETIMEDOUT);
        -1
    }

    /// C `sockaddrfill`: '*' means NULL; picks `random()%n` among matches.
    pub fn addrfill(
        host: Option<&CStr>,
        service: Option<&CStr>,
        family: c_int,
        socktype: c_int,
        passive: bool,
    ) -> Option<libc::sockaddr_in> {
        let host = host.filter(|h| h.to_bytes().first() != Some(&b'*'));
        let service = service.filter(|s| s.to_bytes().first() != Some(&b'*'));
        let list = sys::addrinfo(host, service, family, socktype, passive)?;
        let n = list.len() as u32;
        let r = if n > 0 { (sys::random() % n as libc::c_long) as u32 } else { 0 };
        list.get(r as usize).copied()
    }

    /// C `univmakestrip` text.
    pub fn fmt_ip(ip: u32) -> String {
        format!("{}.{}.{}.{}", (ip >> 24) as u8, (ip >> 16) as u8, (ip >> 8) as u8, ip as u8)
    }

    /// C `univmakestripport` text.
    pub fn fmt_ip_port(ip: u32, port: u16) -> String {
        format!("{}:{port}", fmt_ip(ip))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn pair() -> (c_int, c_int) {
            let mut fds = [0 as c_int; 2];
            #[allow(unsafe_code)]
            // SAFETY: socketpair fills the two-element array.
            let r = unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) };
            assert_eq!(r, 0);
            (fds[0], fds[1])
        }

        #[test]
        fn formatting() {
            assert_eq!(fmt_ip(0xC0A8_0001), "192.168.0.1");
            assert_eq!(fmt_ip_port(0xFFFF_FFFF, 65535), "255.255.255.255:65535");
        }

        #[test]
        fn read_write_forward_and_timeouts() {
            let (a, b) = pair();
            assert_eq!(towrite(a, b"hello", 1000, 1000), 5);
            let mut buf = [0u8; 5];
            assert_eq!(toread(b, &mut buf, 1000, 1000), 5);
            assert_eq!(&buf, b"hello");
            sys::nonblock(b);
            let mut buf = [0u8; 4];
            assert_eq!(toread(b, &mut buf, 50, 100), -1);
            assert_eq!(sys::errno(), ETIMEDOUT);
            assert_eq!(towait(b, 30), -1);
            assert_eq!(sys::errno(), ETIMEDOUT);

            let (c, d) = pair();
            towrite(a, b"abcdef", 1000, 1000);
            let mut fbuf = [0u8; 6];
            assert_eq!(toforward(b, c, &mut fbuf, 0, 0, 1000, 1000), 6);
            let mut out = [0u8; 6];
            assert_eq!(toread(d, &mut out, 1000, 1000), 6);
            assert_eq!(&out, b"abcdef");

            // peer closed: short read returns what arrived, errno ECONNRESET
            towrite(a, b"xy", 1000, 1000);
            sys::close(a);
            let mut buf = [0u8; 8];
            assert_eq!(toread(b, &mut buf, 1000, 1000), 2);
            assert_eq!(sys::errno(), ECONNRESET);
            for fd in [b, c, d] {
                sys::close(fd);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// C ABI boundary (sockets.h)
// ---------------------------------------------------------------------------

/// # Safety
// SAFETY: caller guarantees `p` is NULL or a valid C string.
unsafe fn opt_cstr<'a>(p: *const c_char) -> Option<&'a CStr> {
    // SAFETY: per fn contract.
    (!p.is_null()).then(|| unsafe { CStr::from_ptr(p) })
}

/// # Safety
// SAFETY: caller guarantees `buff` is valid for `leng` bytes.
unsafe fn buf_mut<'a>(buff: *mut c_void, leng: u32) -> &'a mut [u8] {
    if leng == 0 {
        return &mut [];
    }
    // SAFETY: per fn contract.
    unsafe { std::slice::from_raw_parts_mut(buff as *mut u8, leng as usize) }
}

/// # Safety
// SAFETY: caller guarantees `buff` is valid for `leng` bytes.
unsafe fn buf<'a>(buff: *const c_void, leng: u32) -> &'a [u8] {
    if leng == 0 {
        return &[];
    }
    // SAFETY: per fn contract.
    unsafe { std::slice::from_raw_parts(buff as *const u8, leng as usize) }
}

/// Write the non-null (ip, port) out-params.
///
/// # Safety
/// Non-null pointers must be writable.
unsafe fn put_addr(ip: *mut u32, port: *mut u16, v: (u32, u16)) {
    // SAFETY: per fn contract; each pointer checked for NULL as in C.
    unsafe {
        if !ip.is_null() {
            *ip = v.0;
        }
        if !port.is_null() {
            *port = v.1;
        }
    }
}

/// C `snprintf(dst,size,"%s",s); dst[size-1]=0` for text shorter than size.
///
/// # Safety
/// `dst` must be writable for `size` bytes.
unsafe fn put_text(dst: *mut c_char, size: usize, s: &str) {
    let n = s.len().min(size - 1);
    // SAFETY: per fn contract; n+1 <= size and index size-1 < size.
    unsafe {
        core::ptr::copy_nonoverlapping(s.as_ptr() as *const c_char, dst, n);
        *dst.add(n) = 0;
        *dst.add(size - 1) = 0;
    }
}

fn strdup(s: &str) -> *mut c_char {
    let c = std::ffi::CString::new(s).expect("address text has no NUL");
    // SAFETY: strdup copies a valid C string into malloc'd memory (the
    // caller frees it, as with the C strdup).
    unsafe { libc::strdup(c.as_ptr()) }
}

/// # Safety
/// `strip` writable for STRIPSIZE bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn univmakestrip(strip: *mut c_char, ip: u32) {
    // SAFETY: per fn contract.
    unsafe { put_text(strip, STRIPSIZE, &fmt_ip(ip)) }
}

/// # Safety
/// `stripport` writable for STRIPPORTSIZE bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn univmakestripport(stripport: *mut c_char, ip: u32, port: u16) {
    // SAFETY: per fn contract.
    unsafe { put_text(stripport, STRIPPORTSIZE, &fmt_ip_port(ip, port)) }
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn univallocstrip(ip: u32) -> *mut c_char {
    strdup(&fmt_ip(ip))
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn univallocstripport(ip: u32, port: u16) -> *mut c_char {
    strdup(&fmt_ip_port(ip, port))
}

macro_rules! fd_fn {
    ($($name:ident => $body:expr;)*) => {$(
        // SAFETY: exported by symbol for C-ABI consumers; body is safe.
        #[unsafe(no_mangle)]
        pub extern "C" fn $name(sock: c_int) -> c_int {
            ($body)(sock)
        }
    )*};
}

fd_fn! {
    univnonblock => sys::nonblock;
    tcpnonblock => sys::nonblock;
    udpnonblock => sys::nonblock;
    unixnonblock => sys::nonblock;
    tcpgetstatus => sys::getstatus;
    udpgetstatus => sys::getstatus;
    unixgetstatus => sys::getstatus;
    tcpaccept => sys::accept;
    unixaccept => sys::accept;
    udpclose => sys::close;
    tcpreuseaddr => |s| sys::setsockopt_int(s, libc::SOL_SOCKET, libc::SO_REUSEADDR, 1);
    tcpnodelay => |s| sys::setsockopt_int(s, libc::IPPROTO_TCP, libc::TCP_NODELAY, 1);
    tcpsetacceptfilter => |s| sys::setsockopt_int(s, libc::IPPROTO_TCP, libc::TCP_DEFER_ACCEPT, 1);
    tcpaccfhttp => |_| { sys::set_errno(libc::EINVAL); -1 };
    tcpaccfdata => |_| { sys::set_errno(libc::EINVAL); -1 };
    tcpclose => |s| { sys::shutdown(s, libc::SHUT_WR); sys::close(s) };
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn tcpsocket() -> c_int {
    sys::socket(libc::AF_INET, libc::SOCK_STREAM)
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn udpsocket() -> c_int {
    sys::socket(libc::AF_INET, libc::SOCK_DGRAM)
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn unixsocket() -> c_int {
    sys::socket(libc::AF_UNIX, libc::SOCK_STREAM)
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn tcpshutdown(sock: c_int) {
    sys::shutdown(sock, libc::SHUT_WR);
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn tcptowait(sock: c_int, msectoall: u32) -> c_int {
    imp::towait(sock, msectoall)
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn tcptoaccept(lsock: c_int, msecto: u32) -> c_int {
    imp::toaccept(lsock, msecto)
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn unixtoaccept(lsock: c_int, msecto: u32) -> c_int {
    imp::toaccept(lsock, msecto)
}

macro_rules! toread_fn {
    ($($name:ident),*) => {$(
        /// # Safety
        /// `buff` writable for `leng` bytes.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(sock: c_int, buff: *mut c_void, leng: u32, msectopart: u32, msectoall: u32) -> i32 {
            // SAFETY: per fn contract.
            imp::toread(sock, unsafe { buf_mut(buff, leng) }, msectopart, msectoall)
        }
    )*};
}
toread_fn!(univtoread, tcptoread, unixtoread);

macro_rules! towrite_fn {
    ($($name:ident),*) => {$(
        /// # Safety
        /// `buff` readable for `leng` bytes.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(sock: c_int, buff: *const c_void, leng: u32, msectopart: u32, msectoall: u32) -> i32 {
            // SAFETY: per fn contract.
            imp::towrite(sock, unsafe { buf(buff, leng) }, msectopart, msectoall)
        }
    )*};
}
towrite_fn!(univtowrite, tcptowrite, unixtowrite);

macro_rules! toforward_fn {
    ($($name:ident),*) => {$(
        /// # Safety
        /// `buff` valid for `leng` bytes; `rcvd`,`sent` <= `leng`.
        #[unsafe(no_mangle)]
        #[allow(clippy::too_many_arguments)]
        pub unsafe extern "C" fn $name(
            srcsock: c_int,
            dstsock: c_int,
            buff: *mut c_void,
            leng: u32,
            rcvd: u32,
            sent: u32,
            msectopart: u32,
            msectoall: u32,
        ) -> i32 {
            // SAFETY: per fn contract.
            let b = unsafe { buf_mut(buff, leng) };
            imp::toforward(srcsock, dstsock, b, rcvd, sent, msectopart, msectoall)
        }
    )*};
}
toforward_fn!(univtoforward, tcptoforward, unixtoforward);

/// # Safety
/// Strings NULL or valid; non-null out-params writable.
unsafe fn resolve(
    host: *const c_char,
    service: *const c_char,
    ip: *mut u32,
    port: *mut u16,
    socktype: c_int,
    passive: c_int,
) -> c_int {
    // SAFETY: per fn contract.
    let (h, s) = unsafe { (opt_cstr(host), opt_cstr(service)) };
    match imp::addrfill(h, s, libc::AF_INET, socktype, passive != 0) {
        Some(sa) => {
            // SAFETY: per fn contract.
            unsafe { put_addr(ip, port, (u32::from_be(sa.sin_addr.s_addr), u16::from_be(sa.sin_port))) };
            0
        }
        None => -1,
    }
}

/// # Safety
/// Strings NULL or valid C strings; non-null out-params writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpresolve(
    hostname: *const c_char,
    service: *const c_char,
    ip: *mut u32,
    port: *mut u16,
    passive: c_int,
) -> c_int {
    // SAFETY: forwarded contract.
    unsafe { resolve(hostname, service, ip, port, libc::SOCK_STREAM, passive) }
}

/// # Safety
/// As `tcpresolve`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpresolve(
    hostname: *const c_char,
    service: *const c_char,
    ip: *mut u32,
    port: *mut u16,
    passive: c_int,
) -> c_int {
    // SAFETY: forwarded contract.
    unsafe { resolve(hostname, service, ip, port, libc::SOCK_DGRAM, passive) }
}

/// # Safety
/// Strings NULL or valid C strings.
unsafe fn strfill(host: *const c_char, service: *const c_char, socktype: c_int, passive: bool) -> Option<libc::sockaddr_in> {
    // SAFETY: per fn contract.
    let (h, s) = unsafe { (opt_cstr(host), opt_cstr(service)) };
    imp::addrfill(h, s, libc::AF_INET, socktype, passive)
}

fn bind_listen(sock: c_int, sa: &libc::sockaddr_in, queue: c_int) -> c_int {
    if sys::bind_in(sock, sa) < 0 || sys::listen(sock, queue) < 0 {
        return -1;
    }
    0
}

/// # Safety
/// Strings NULL or valid C strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpstrbind(sock: c_int, hostname: *const c_char, service: *const c_char) -> c_int {
    // SAFETY: forwarded contract.
    match unsafe { strfill(hostname, service, libc::SOCK_STREAM, true) } {
        Some(sa) if sys::bind_in(sock, &sa) >= 0 => 0,
        _ => -1,
    }
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn tcpnumbind(sock: c_int, ip: u32, port: u16) -> c_int {
    if sys::bind_in(sock, &sys::sin(ip, port)) < 0 { -1 } else { 0 }
}

/// # Safety
/// Strings NULL or valid C strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpstrconnect(sock: c_int, hostname: *const c_char, service: *const c_char) -> c_int {
    // SAFETY: forwarded contract.
    match unsafe { strfill(hostname, service, libc::SOCK_STREAM, false) } {
        Some(sa) => imp::connect_status(sys::connect_in(sock, &sa)),
        None => -1,
    }
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn tcpnumconnect(sock: c_int, ip: u32, port: u16) -> c_int {
    imp::connect_status(sys::connect_in(sock, &sys::sin(ip, port)))
}

/// # Safety
/// Strings NULL or valid C strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpstrtoconnect(
    sock: c_int,
    hostname: *const c_char,
    service: *const c_char,
    msecto: u32,
) -> c_int {
    if sys::nonblock(sock) < 0 {
        return -1;
    }
    // SAFETY: forwarded contract.
    let Some(sa) = (unsafe { strfill(hostname, service, libc::SOCK_STREAM, false) }) else { return -1 };
    imp::finish_toconnect(sock, sys::connect_in(sock, &sa), msecto)
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn tcpnumtoconnect(sock: c_int, ip: u32, port: u16, msecto: u32) -> c_int {
    if sys::nonblock(sock) < 0 {
        return -1;
    }
    imp::finish_toconnect(sock, sys::connect_in(sock, &sys::sin(ip, port)), msecto)
}

/// # Safety
/// Strings NULL or valid C strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpstrlisten(
    sock: c_int,
    hostname: *const c_char,
    service: *const c_char,
    queue: u16,
) -> c_int {
    // SAFETY: forwarded contract.
    match unsafe { strfill(hostname, service, libc::SOCK_STREAM, true) } {
        Some(sa) => bind_listen(sock, &sa, queue as c_int),
        None => -1,
    }
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn tcpnumlisten(sock: c_int, ip: u32, port: u16, queue: u16) -> c_int {
    bind_listen(sock, &sys::sin(ip, port), queue as c_int)
}

/// # Safety
/// Non-null out-params writable.
unsafe fn getname(sock: c_int, ip: *mut u32, port: *mut u16, peer: bool) -> c_int {
    let r = sys::name(sock, peer);
    // SAFETY: per fn contract; C zeroes the outputs on failure.
    unsafe { put_addr(ip, port, r.unwrap_or((0, 0))) };
    if r.is_some() { 0 } else { -1 }
}

/// # Safety
/// Non-null out-params writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpgetpeer(sock: c_int, ip: *mut u32, port: *mut u16) -> c_int {
    // SAFETY: forwarded contract.
    unsafe { getname(sock, ip, port, true) }
}

/// # Safety
/// Non-null out-params writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpgetmyaddr(sock: c_int, ip: *mut u32, port: *mut u16) -> c_int {
    // SAFETY: forwarded contract.
    unsafe { getname(sock, ip, port, false) }
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn udpnumlisten(sock: c_int, ip: u32, port: u16) -> c_int {
    sys::bind_in(sock, &sys::sin(ip, port))
}

/// # Safety
/// Strings NULL or valid C strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpstrlisten(sock: c_int, hostname: *const c_char, service: *const c_char) -> c_int {
    // SAFETY: forwarded contract.
    match unsafe { strfill(hostname, service, libc::SOCK_DGRAM, true) } {
        Some(sa) => sys::bind_in(sock, &sa),
        None => -1,
    }
}

/// # Safety
/// `buff` readable for `leng` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpwrite(sock: c_int, ip: u32, port: u16, buff: *const c_void, leng: u16) -> c_int {
    if leng > 512 {
        return -1;
    }
    // SAFETY: per fn contract.
    sys::sendto_in(sock, unsafe { buf(buff, leng as u32) }, &sys::sin(ip, port))
}

/// # Safety
/// `buff` writable for `leng` bytes; non-null out-params writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpread(sock: c_int, ip: *mut u32, port: *mut u16, buff: *mut c_void, leng: u16) -> c_int {
    // SAFETY: per fn contract.
    let (ret, src) = sys::recvfrom_in(sock, unsafe { buf_mut(buff, leng as u32) });
    if let Some(a) = src {
        // SAFETY: per fn contract.
        unsafe { put_addr(ip, port, a) };
    }
    ret
}

/// # Safety
/// `path` must be a valid C string.
unsafe fn sun(path: *const c_char) -> Option<libc::sockaddr_un> {
    // SAFETY: per fn contract.
    sys::sun(unsafe { CStr::from_ptr(path) }.to_bytes())
}

/// # Safety
/// `path` must be a valid C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixconnect(sock: c_int, path: *const c_char) -> c_int {
    // SAFETY: forwarded contract.
    match unsafe { sun(path) } {
        Some(sa) => imp::connect_status(sys::connect_un(sock, &sa)),
        None => -1,
    }
}

/// # Safety
/// `path` must be a valid C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixtoconnect(sock: c_int, path: *const c_char, msecto: u32) -> c_int {
    if sys::nonblock(sock) < 0 {
        return -1;
    }
    // SAFETY: forwarded contract.
    let Some(sa) = (unsafe { sun(path) }) else { return -1 };
    imp::finish_toconnect(sock, sys::connect_un(sock, &sa), msecto)
}

/// # Safety
/// `path` must be a valid C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixlisten(sock: c_int, path: *const c_char, queue: c_int) -> c_int {
    // SAFETY: forwarded contract.
    let Some(sa) = (unsafe { sun(path) }) else { return -1 };
    if sys::bind_un(sock, &sa) < 0 || sys::listen(sock, queue) < 0 {
        return -1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_buffers_match_c_snprintf() {
        let mut b = [b'#' as c_char; STRIPSIZE];
        // SAFETY: local buffers of the required sizes.
        unsafe { univmakestrip(b.as_mut_ptr(), 0x0A00_0001) };
        let s: Vec<u8> = b.iter().map(|&c| c as u8).collect();
        assert_eq!(&s[..9], b"10.0.0.1\0");
        assert_eq!(s[9], b'#'); // bytes past the NUL untouched (as snprintf)
        assert_eq!(s[15], 0);
        let mut b = [0 as c_char; STRIPPORTSIZE];
        // SAFETY: as above.
        unsafe { univmakestripport(b.as_mut_ptr(), 0x7F00_0001, 9421) };
        // SAFETY: NUL-terminated by univmakestripport.
        assert_eq!(unsafe { CStr::from_ptr(b.as_ptr()) }.to_bytes(), b"127.0.0.1:9421");
    }

    #[test]
    fn tcp_listen_connect_resolve() {
        let l = tcpsocket();
        assert_eq!(tcpreuseaddr(l), 0);
        assert_eq!(tcpnumlisten(l, 0x7F00_0001, 0, 5), 0);
        let (mut ip, mut port) = (0u32, 0u16);
        // SAFETY: locals as out-params.
        unsafe { assert_eq!(tcpgetmyaddr(l, &mut ip, &mut port), 0) };
        assert_eq!(ip, 0x7F00_0001);
        let c = tcpsocket();
        assert_eq!(tcpnumtoconnect(c, ip, port, 1000), 0);
        let a = tcptoaccept(l, 1000);
        assert!(a >= 0);
        // SAFETY: literal buffer.
        unsafe { assert_eq!(tcptowrite(c, b"ping".as_ptr() as *const c_void, 4, 1000, 1000), 4) };
        let mut buf = [0u8; 4];
        // SAFETY: local buffer.
        unsafe { assert_eq!(tcptoread(a, buf.as_mut_ptr() as *mut c_void, 4, 1000, 1000), 4) };
        assert_eq!(&buf, b"ping");
        let (mut pip, mut pport) = (1u32, 1u16);
        // SAFETY: locals as out-params.
        unsafe { assert_eq!(tcpgetpeer(-1, &mut pip, &mut pport), -1) };
        assert_eq!((pip, pport), (0, 0));
        // SAFETY: literal C strings / locals.
        unsafe {
            assert_eq!(tcpresolve(c"127.0.0.1".as_ptr(), c"9421".as_ptr(), &mut ip, &mut port, 0), 0);
            assert_eq!((ip, port), (0x7F00_0001, 9421));
            assert_eq!(tcpresolve(c"*".as_ptr(), c"*".as_ptr(), &mut ip, &mut port, 1), -1);
        }
        for fd in [a, c, l] {
            tcpclose(fd);
        }
    }

    #[test]
    fn unix_paths_and_accept_timeout() {
        let long = std::ffi::CString::new(vec![b'x'; 108]).unwrap();
        let s = unixsocket();
        // SAFETY: valid C strings.
        unsafe { assert_eq!(unixlisten(s, long.as_ptr(), 1), -1) };
        let dir = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/socktest"));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join(format!("socktest-{}", std::process::id()));
        let _ = std::fs::remove_file(&p);
        let cp = std::ffi::CString::new(p.to_str().unwrap()).unwrap();
        // SAFETY: valid C strings.
        unsafe { assert_eq!(unixlisten(s, cp.as_ptr(), 1), 0) };
        unixnonblock(s);
        assert_eq!(unixtoaccept(s, 30), -1);
        assert_eq!(sys::errno(), libc::ETIMEDOUT);
        let c = unixsocket();
        // SAFETY: valid C string.
        unsafe { assert_eq!(unixtoconnect(c, cp.as_ptr(), 1000), 0) };
        assert!(unixtoaccept(s, 1000) >= 0);
        std::fs::remove_file(&p).unwrap();
        assert_eq!(tcpaccfhttp(s), -1);
        assert_eq!(sys::errno(), libc::EINVAL);
    }
}
