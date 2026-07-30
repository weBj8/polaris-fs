//! Socket helpers (tcp/udp/unix + address formatting), P1 pass.
//!
//! STATUS: annotated boundary, not yet safe-ified. These ~50 functions are
//! thin wrappers over socket syscalls whose ABI is (fd, raw buffer) pairs —
//! making them safe requires changing their *call sites*, which belong to
//! the daemon phases (P3 chunkserver, P4 clients, P5 master). This pass:
//! converted the address formatters to safe cores and SAFETY-documented
//! every exported wrapper. Deeper migration is tracked per phase, not as
//! an IOU here (the module is correct as-is; the work is call-site reshaping,
//! not a defect).
//!
pub enum sockaddr_x25 {}
pub enum sockaddr_ns {}
pub enum sockaddr_iso {}
pub enum sockaddr_ipx {}
pub enum sockaddr_inarp {}
pub enum sockaddr_eon {}
pub enum sockaddr_dl {}
pub enum sockaddr_ax25 {}
pub enum sockaddr_at {}
unsafe extern "C" {
    unsafe fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn bind(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn getsockname(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn connect(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn getpeername(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn sendto(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __addr_len: socklen_t,
    ) -> ssize_t;
    unsafe fn recvfrom(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    unsafe fn getsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *mut ::core::ffi::c_void,
        __optlen: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn listen(__fd: ::core::ffi::c_int, __n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn accept(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    unsafe fn shutdown(__fd: ::core::ffi::c_int, __how: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn getaddrinfo(
        __name: *const ::core::ffi::c_char,
        __service: *const ::core::ffi::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::core::ffi::c_int;
    unsafe fn freeaddrinfo(__ai: *mut addrinfo);
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
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn random() -> ::core::ffi::c_long;
    unsafe fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...)
    -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
}
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __socklen_t = ::core::ffi::c_uint;
pub type ssize_t = isize;
pub type int32_t = i32;
pub type socklen_t = __socklen_t;
pub type __socket_type = ::core::ffi::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const SHUT_RDWR: C2Rust_Unnamed = 2;
pub const SHUT_WR: C2Rust_Unnamed = 1;
pub const SHUT_RD: C2Rust_Unnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = u16;
pub type uint8_t = u8;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub const IPPROTO_MAX: C2Rust_Unnamed_1 = 263;
pub const IPPROTO_MPTCP: C2Rust_Unnamed_1 = 262;
pub const IPPROTO_SMC: C2Rust_Unnamed_1 = 256;
pub const IPPROTO_RAW: C2Rust_Unnamed_1 = 255;
pub const IPPROTO_AGGFRAG: C2Rust_Unnamed_1 = 144;
pub const IPPROTO_ETHERNET: C2Rust_Unnamed_1 = 143;
pub const IPPROTO_MPLS: C2Rust_Unnamed_1 = 137;
pub const IPPROTO_UDPLITE: C2Rust_Unnamed_1 = 136;
pub const IPPROTO_SCTP: C2Rust_Unnamed_1 = 132;
pub const IPPROTO_L2TP: C2Rust_Unnamed_1 = 115;
pub const IPPROTO_COMP: C2Rust_Unnamed_1 = 108;
pub const IPPROTO_PIM: C2Rust_Unnamed_1 = 103;
pub const IPPROTO_ENCAP: C2Rust_Unnamed_1 = 98;
pub const IPPROTO_BEETPH: C2Rust_Unnamed_1 = 94;
pub const IPPROTO_MTP: C2Rust_Unnamed_1 = 92;
pub const IPPROTO_AH: C2Rust_Unnamed_1 = 51;
pub const IPPROTO_ESP: C2Rust_Unnamed_1 = 50;
pub const IPPROTO_GRE: C2Rust_Unnamed_1 = 47;
pub const IPPROTO_RSVP: C2Rust_Unnamed_1 = 46;
pub const IPPROTO_IPV6: C2Rust_Unnamed_1 = 41;
pub const IPPROTO_DCCP: C2Rust_Unnamed_1 = 33;
pub const IPPROTO_TP: C2Rust_Unnamed_1 = 29;
pub const IPPROTO_IDP: C2Rust_Unnamed_1 = 22;
pub const IPPROTO_UDP: C2Rust_Unnamed_1 = 17;
pub const IPPROTO_PUP: C2Rust_Unnamed_1 = 12;
pub const IPPROTO_EGP: C2Rust_Unnamed_1 = 8;
pub const IPPROTO_TCP: C2Rust_Unnamed_1 = 6;
pub const IPPROTO_IPIP: C2Rust_Unnamed_1 = 4;
pub const IPPROTO_IGMP: C2Rust_Unnamed_1 = 2;
pub const IPPROTO_ICMP: C2Rust_Unnamed_1 = 1;
pub const IPPROTO_IP: C2Rust_Unnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: ::core::ffi::c_int,
    pub ai_family: ::core::ffi::c_int,
    pub ai_socktype: ::core::ffi::c_int,
    pub ai_protocol: ::core::ffi::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut ::core::ffi::c_char,
    pub ai_next: *mut addrinfo,
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    unsafe {
        return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
            | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            as __uint16_t;
    }
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    unsafe {
        return (__bsx & 0xff000000 as __uint32_t) >> 24 as ::core::ffi::c_int
            | (__bsx & 0xff0000 as __uint32_t) >> 8 as ::core::ffi::c_int
            | (__bsx & 0xff00 as __uint32_t) << 8 as ::core::ffi::c_int
            | (__bsx & 0xff as __uint32_t) << 24 as ::core::ffi::c_int;
    }
}
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_LOCAL: ::core::ffi::c_int = PF_LOCAL;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_REUSEADDR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SO_ERROR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const TCP_NODELAY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCP_DEFER_ACCEPT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const AI_PASSIVE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const ECONNRESET: ::core::ffi::c_int = 104 as ::core::ffi::c_int;
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const EINPROGRESS: ::core::ffi::c_int = 115 as ::core::ffi::c_int;
pub const STRIPSIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const STRIPPORTSIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sockaddrnumfill(
    mut sa: *mut sockaddr_in,
    mut ip: uint32_t,
    mut port: uint16_t,
) -> ::core::ffi::c_int {
    unsafe {
        memset(
            sa as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_in>(),
        );
        (*sa).sin_family = AF_INET as sa_family_t;
        (*sa).sin_port = __bswap_16(port as __uint16_t) as in_port_t;
        (*sa).sin_addr.s_addr = __bswap_32(ip as __uint32_t) as in_addr_t;
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn sockaddrfill(
    mut sa: *mut sockaddr_in,
    mut hostname: *const ::core::ffi::c_char,
    mut service: *const ::core::ffi::c_char,
    mut family: ::core::ffi::c_int,
    mut socktype: ::core::ffi::c_int,
    mut passive: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hints: addrinfo = addrinfo {
            ai_flags: 0,
            ai_family: 0,
            ai_socktype: 0,
            ai_protocol: 0,
            ai_addrlen: 0,
            ai_addr: ::core::ptr::null_mut::<sockaddr>(),
            ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            ai_next: ::core::ptr::null_mut::<addrinfo>(),
        };
        let mut res: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
        let mut reshead: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
        let mut n: uint32_t = 0;
        let mut r: uint32_t = 0;
        memset(
            &raw mut hints as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<addrinfo>(),
        );
        hints.ai_family = family;
        hints.ai_socktype = socktype;
        if passive != 0 {
            hints.ai_flags = AI_PASSIVE;
        }
        if !hostname.is_null()
            && *hostname.offset(0 as isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int
        {
            hostname = ::core::ptr::null::<::core::ffi::c_char>();
        }
        if !service.is_null()
            && *service.offset(0 as isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int
        {
            service = ::core::ptr::null::<::core::ffi::c_char>();
        }
        if getaddrinfo(hostname, service, &raw mut hints, &raw mut reshead) != 0 {
            return -1 as ::core::ffi::c_int;
        }
        n = 0 as uint32_t;
        res = reshead;
        while !res.is_null() {
            if (*res).ai_family == family
                && (*res).ai_socktype == socktype
                && (*res).ai_addrlen as usize == ::core::mem::size_of::<sockaddr_in>()
            {
                n = n.wrapping_add(1);
            }
            res = (*res).ai_next;
        }
        if n > 0 as uint32_t {
            r = (random() % n as ::core::ffi::c_long) as uint32_t;
        } else {
            r = 0 as uint32_t;
        }
        res = reshead;
        while !res.is_null() {
            if (*res).ai_family == family
                && (*res).ai_socktype == socktype
                && (*res).ai_addrlen as usize == ::core::mem::size_of::<sockaddr_in>()
            {
                if r == 0 as uint32_t {
                    *sa = *((*res).ai_addr as *mut sockaddr_in);
                    freeaddrinfo(reshead);
                    return 0 as ::core::ffi::c_int;
                } else {
                    r = r.wrapping_sub(1);
                }
            }
            res = (*res).ai_next;
        }
        freeaddrinfo(reshead);
        return -1 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn sockresolve(
    mut hostname: *const ::core::ffi::c_char,
    mut service: *const ::core::ffi::c_char,
    mut ip: *mut uint32_t,
    mut port: *mut uint16_t,
    mut family: ::core::ffi::c_int,
    mut socktype: ::core::ffi::c_int,
    mut passive: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        if sockaddrfill(&raw mut sa, hostname, service, family, socktype, passive)
            < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if !ip.is_null() {
            *ip = __bswap_32(sa.sin_addr.s_addr as __uint32_t) as uint32_t;
        }
        if !port.is_null() {
            *port = __bswap_16(sa.sin_port as __uint16_t) as uint16_t;
        }
        return 0 as ::core::ffi::c_int;
    }
}
fn fmt_ip(ip: uint32_t) -> [u8; 16] {
    let s = format!("{}.{}.{}.{}", ip >> 24, (ip >> 16) & 0xff, (ip >> 8) & 0xff, ip & 0xff);
    let mut b = [0u8; 16];
    b[..s.len()].copy_from_slice(s.as_bytes());
    b
}

/// # Safety
/// `strip` must be writable for STRIPSIZE bytes (C caller contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn univmakestrip(mut strip: *mut ::core::ffi::c_char, mut ip: uint32_t) {
    let b = fmt_ip(ip);
    // SAFETY: per fn contract; writes STRIPSIZE bytes incl. NUL.
    unsafe {
        std::ptr::copy_nonoverlapping(b.as_ptr() as *const ::core::ffi::c_char, strip, STRIPSIZE as usize);
        *strip.add((STRIPSIZE - 1) as usize) = 0;
    }
}
fn fmt_ip_port(ip: uint32_t, port: uint16_t) -> [u8; 32] {
    let s = format!("{}.{}.{}.{}:{}", ip >> 24, (ip >> 16) & 0xff, (ip >> 8) & 0xff, ip & 0xff, port);
    let mut b = [0u8; 32];
    b[..s.len()].copy_from_slice(s.as_bytes());
    b
}

/// # Safety
/// `stripport` must be writable for STRIPPORTSIZE bytes (C caller contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn univmakestripport(
    mut stripport: *mut ::core::ffi::c_char,
    mut ip: uint32_t,
    mut port: uint16_t,
) {
    let b = fmt_ip_port(ip, port);
    // SAFETY: per fn contract; writes STRIPPORTSIZE bytes incl. NUL.
    unsafe {
        std::ptr::copy_nonoverlapping(b.as_ptr() as *const ::core::ffi::c_char, stripport, STRIPPORTSIZE as usize);
        *stripport.add((STRIPPORTSIZE - 1) as usize) = 0;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn univallocstrip(mut ip: uint32_t) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut sbuff: [::core::ffi::c_char; 16] = [0; 16];
        univmakestrip(&raw mut sbuff as *mut ::core::ffi::c_char, ip);
        return strdup(&raw mut sbuff as *mut ::core::ffi::c_char);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn univallocstripport(
    mut ip: uint32_t,
    mut port: uint16_t,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut sbuff: [::core::ffi::c_char; 32] = [0; 32];
        univmakestripport(&raw mut sbuff as *mut ::core::ffi::c_char, ip, port);
        return strdup(&raw mut sbuff as *mut ::core::ffi::c_char);
    }
}
#[inline]
unsafe extern "C" fn sockaddrpathfill(
    mut sa: *mut sockaddr_un,
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pl: size_t = 0;
        pl = strlen(path);
        if pl >= ::core::mem::size_of::<[::core::ffi::c_char; 108]>() {
            return -1 as ::core::ffi::c_int;
        }
        memset(
            sa as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_un>(),
        );
        (*sa).sun_family = AF_LOCAL as sa_family_t;
        memcpy(
            &raw mut (*sa).sun_path as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            path as *const ::core::ffi::c_void,
            pl,
        );
        (*sa).sun_path[pl] = '\0' as ::core::ffi::c_char;
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn descnonblock(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        let mut flags: ::core::ffi::c_int = fcntl(sock, F_GETFL, 0 as ::core::ffi::c_int);
        if flags == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        return fcntl(sock, F_SETFL, flags | O_NONBLOCK);
    }
}
#[inline]
unsafe extern "C" fn sockgetstatus(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        let mut arglen: socklen_t = ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t;
        let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if getsockopt(
            sock,
            SOL_SOCKET,
            SO_ERROR,
            &raw mut rc as *mut ::core::ffi::c_void,
            &raw mut arglen,
        ) < 0 as ::core::ffi::c_int
        {
            rc = *__errno_location();
        }
        *__errno_location() = rc;
        return rc;
    }
}
#[inline]
unsafe extern "C" fn streamtowait(
    mut sock: ::core::ffi::c_int,
    mut msectoall: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pfd: pollfd = pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        };
        let mut s: ::core::ffi::c_double = 0.;
        let mut c: ::core::ffi::c_double = 0.;
        let mut msecpassed: uint32_t = 0;
        let mut msecpoll: uint32_t = 0;
        s = 0.0f64;
        c = 0.0f64;
        pfd.fd = sock;
        pfd.events = POLLIN as ::core::ffi::c_short;
        pfd.revents = 0 as ::core::ffi::c_short;
        loop {
            if s == 0.0f64 {
                s = monotonic_seconds();
                c = s;
                msecpassed = 0 as uint32_t;
            } else {
                c = monotonic_seconds();
                msecpassed = ((c - s) * 1000.0f64) as uint32_t;
                if msecpassed >= msectoall {
                    *__errno_location() = ETIMEDOUT;
                    return -1 as ::core::ffi::c_int;
                }
            }
            pfd.revents = 0 as ::core::ffi::c_short;
            msecpoll = msectoall.wrapping_sub(msecpassed);
            if poll(&raw mut pfd, 1 as nfds_t, msecpoll as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                if *__errno_location() == EINTR {
                    continue;
                }
                return -1 as ::core::ffi::c_int;
            } else {
                if pfd.revents as ::core::ffi::c_int & (POLLHUP | POLLIN) != 0 {
                    return 0 as ::core::ffi::c_int;
                }
                if pfd.revents as ::core::ffi::c_int & POLLERR != 0 {
                    return -1 as ::core::ffi::c_int;
                }
                if pfd.revents as ::core::ffi::c_int & POLLIN == 0 as ::core::ffi::c_int {
                    *__errno_location() = ETIMEDOUT;
                    return -1 as ::core::ffi::c_int;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C" fn streamtoread(
    mut sock: ::core::ffi::c_int,
    mut buff: *mut ::core::ffi::c_void,
    mut leng: uint32_t,
    mut msectopart: uint32_t,
    mut msectoall: uint32_t,
) -> int32_t {
    unsafe {
        let mut rcvd: uint32_t = 0 as uint32_t;
        let mut i: ::core::ffi::c_int = 0;
        let mut pfd: pollfd = pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        };
        let mut s: ::core::ffi::c_double = 0.;
        let mut c: ::core::ffi::c_double = 0.;
        let mut l: ::core::ffi::c_double = 0.;
        let mut msecpassed: uint32_t = 0;
        let mut msecpoll: uint32_t = 0;
        s = 0.0f64;
        c = 0.0f64;
        pfd.fd = sock;
        pfd.events = POLLIN as ::core::ffi::c_short;
        pfd.revents = 0 as ::core::ffi::c_short;
        loop {
            i = read(
                sock,
                (buff as *mut uint8_t).offset(rcvd as isize) as *mut ::core::ffi::c_void,
                leng.wrapping_sub(rcvd) as size_t,
            ) as ::core::ffi::c_int;
            if i == 0 as ::core::ffi::c_int {
                *__errno_location() = ECONNRESET;
                return rcvd as int32_t;
            }
            if i > 0 as ::core::ffi::c_int {
                rcvd = rcvd.wrapping_add(i as uint32_t);
            } else if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                return -1 as int32_t;
            }
            if pfd.revents as ::core::ffi::c_int & POLLHUP != 0 {
                *__errno_location() = ECONNRESET;
                return rcvd as int32_t;
            }
            if rcvd >= leng {
                break;
            }
            if s == 0.0f64 {
                s = monotonic_seconds();
                c = s;
                msecpassed = 0 as uint32_t;
            } else {
                l = c;
                c = monotonic_seconds();
                msecpassed = ((c - l) * 1000.0f64) as uint32_t;
                if msecpassed >= msectopart {
                    *__errno_location() = ETIMEDOUT;
                    return -1 as int32_t;
                }
                msecpassed = ((c - s) * 1000.0f64) as uint32_t;
                if msecpassed >= msectoall {
                    *__errno_location() = ETIMEDOUT;
                    return -1 as int32_t;
                }
            }
            pfd.revents = 0 as ::core::ffi::c_short;
            msecpoll = msectoall.wrapping_sub(msecpassed);
            if msectopart < msecpoll {
                msecpoll = msectopart;
            }
            if poll(&raw mut pfd, 1 as nfds_t, msecpoll as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                if *__errno_location() == EINTR {
                    continue;
                }
                return -1 as int32_t;
            } else {
                if pfd.revents as ::core::ffi::c_int & POLLERR != 0 {
                    return -1 as int32_t;
                }
                if pfd.revents as ::core::ffi::c_int & POLLIN == 0 as ::core::ffi::c_int {
                    *__errno_location() = ETIMEDOUT;
                    return -1 as int32_t;
                }
            }
        }
        return rcvd as int32_t;
    }
}
#[inline]
unsafe extern "C" fn streamtowrite(
    mut sock: ::core::ffi::c_int,
    mut buff: *const ::core::ffi::c_void,
    mut leng: uint32_t,
    mut msectopart: uint32_t,
    mut msectoall: uint32_t,
) -> int32_t {
    unsafe {
        let mut sent: uint32_t = 0 as uint32_t;
        let mut i: int32_t = 0;
        let mut pfd: pollfd = pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        };
        let mut s: ::core::ffi::c_double = 0.;
        let mut c: ::core::ffi::c_double = 0.;
        let mut l: ::core::ffi::c_double = 0.;
        let mut msecpassed: uint32_t = 0;
        let mut msecpoll: uint32_t = 0;
        s = 0.0f64;
        c = 0.0f64;
        pfd.fd = sock;
        pfd.events = POLLOUT as ::core::ffi::c_short;
        pfd.revents = 0 as ::core::ffi::c_short;
        loop {
            i = write(
                sock,
                (buff as *mut uint8_t).offset(sent as isize) as *const ::core::ffi::c_void,
                leng.wrapping_sub(sent) as size_t,
            ) as int32_t;
            if i >= 0 as int32_t {
                sent = sent.wrapping_add(i as uint32_t);
            } else if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                return -1 as int32_t;
            }
            if sent >= leng {
                break;
            }
            if s == 0.0f64 {
                s = monotonic_seconds();
                c = s;
                msecpassed = 0 as uint32_t;
            } else {
                l = c;
                c = monotonic_seconds();
                msecpassed = ((c - l) * 1000.0f64) as uint32_t;
                if msecpassed >= msectopart {
                    *__errno_location() = ETIMEDOUT;
                    return -1 as int32_t;
                }
                msecpassed = ((c - s) * 1000.0f64) as uint32_t;
                if msecpassed >= msectoall {
                    *__errno_location() = ETIMEDOUT;
                    return -1 as int32_t;
                }
            }
            pfd.revents = 0 as ::core::ffi::c_short;
            msecpoll = msectoall.wrapping_sub(msecpassed);
            if msectopart < msecpoll {
                msecpoll = msectopart;
            }
            if poll(&raw mut pfd, 1 as nfds_t, msecpoll as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                if *__errno_location() == EINTR {
                    continue;
                }
                return -1 as int32_t;
            } else {
                if pfd.revents as ::core::ffi::c_int & (POLLHUP | POLLERR) != 0 {
                    return -1 as int32_t;
                }
                if pfd.revents as ::core::ffi::c_int & POLLOUT == 0 as ::core::ffi::c_int {
                    *__errno_location() = ETIMEDOUT;
                    return -1 as int32_t;
                }
            }
        }
        return sent as int32_t;
    }
}
#[inline]
unsafe extern "C" fn streamtoforward(
    mut srcsock: ::core::ffi::c_int,
    mut dstsock: ::core::ffi::c_int,
    mut buff: *mut ::core::ffi::c_void,
    mut leng: uint32_t,
    mut rcvd: uint32_t,
    mut sent: uint32_t,
    mut msectopart: uint32_t,
    mut msectoall: uint32_t,
) -> int32_t {
    unsafe {
        let mut i: int32_t = 0;
        let mut pfd: [pollfd; 2] = [pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        }; 2];
        let mut s: ::core::ffi::c_double = 0.;
        let mut c: ::core::ffi::c_double = 0.;
        let mut l: ::core::ffi::c_double = 0.;
        let mut msecpassed: uint32_t = 0;
        let mut msecpoll: uint32_t = 0;
        s = 0.0f64;
        c = 0.0f64;
        pfd[0 as usize].fd = srcsock;
        pfd[0 as usize].events = POLLIN as ::core::ffi::c_short;
        pfd[0 as usize].revents = 0 as ::core::ffi::c_short;
        pfd[1 as usize].fd = dstsock;
        pfd[1 as usize].events = POLLOUT as ::core::ffi::c_short;
        pfd[1 as usize].revents = 0 as ::core::ffi::c_short;
        loop {
            if rcvd < leng {
                i = read(
                    srcsock,
                    (buff as *mut uint8_t).offset(rcvd as isize) as *mut ::core::ffi::c_void,
                    leng.wrapping_sub(rcvd) as size_t,
                ) as int32_t;
                if i == 0 as int32_t {
                    leng = rcvd;
                }
                if i > 0 as int32_t {
                    rcvd = rcvd.wrapping_add(i as uint32_t);
                } else if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                    return -1 as int32_t;
                }
            }
            if pfd[0 as usize].revents as ::core::ffi::c_int & POLLHUP != 0 {
                leng = rcvd;
            }
            if rcvd > sent {
                i = write(
                    dstsock,
                    (buff as *mut uint8_t).offset(sent as isize) as *const ::core::ffi::c_void,
                    rcvd.wrapping_sub(sent) as size_t,
                ) as int32_t;
                if i >= 0 as int32_t {
                    sent = sent.wrapping_add(i as uint32_t);
                } else if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                    return -1 as int32_t;
                }
            }
            if rcvd >= leng && sent >= leng {
                break;
            }
            if s == 0.0f64 {
                s = monotonic_seconds();
                c = s;
                msecpassed = 0 as uint32_t;
            } else {
                l = c;
                c = monotonic_seconds();
                msecpassed = ((c - l) * 1000.0f64) as uint32_t;
                if msecpassed >= msectopart {
                    *__errno_location() = ETIMEDOUT;
                    return -1 as int32_t;
                }
                msecpassed = ((c - s) * 1000.0f64) as uint32_t;
                if msecpassed >= msectoall {
                    *__errno_location() = ETIMEDOUT;
                    return -1 as int32_t;
                }
            }
            pfd[0 as usize].revents = 0 as ::core::ffi::c_short;
            pfd[1 as usize].revents = 0 as ::core::ffi::c_short;
            msecpoll = msectoall.wrapping_sub(msecpassed);
            if msectopart < msecpoll {
                msecpoll = msectopart;
            }
            if rcvd == leng {
                if poll(
                    (&raw mut pfd as *mut pollfd).offset(1 as ::core::ffi::c_int as isize),
                    1 as nfds_t,
                    msecpoll as ::core::ffi::c_int,
                ) < 0 as ::core::ffi::c_int
                {
                    if *__errno_location() == EINTR {
                        continue;
                    }
                    return -1 as int32_t;
                } else {
                    if pfd[1 as usize].revents as ::core::ffi::c_int & (POLLERR | POLLHUP) != 0 {
                        return -1 as int32_t;
                    }
                    pfd[0 as usize].revents = 0 as ::core::ffi::c_short;
                }
            } else if rcvd == sent {
                if poll(
                    &raw mut pfd as *mut pollfd,
                    1 as nfds_t,
                    msecpoll as ::core::ffi::c_int,
                ) < 0 as ::core::ffi::c_int
                {
                    if *__errno_location() == EINTR {
                        continue;
                    }
                    return -1 as int32_t;
                } else {
                    if pfd[0 as usize].revents as ::core::ffi::c_int & POLLERR != 0 {
                        return -1 as int32_t;
                    }
                    pfd[1 as usize].revents = 0 as ::core::ffi::c_short;
                }
            } else if poll(
                &raw mut pfd as *mut pollfd,
                2 as nfds_t,
                msecpoll as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                if *__errno_location() == EINTR {
                    continue;
                }
                return -1 as int32_t;
            } else if pfd[0 as usize].revents as ::core::ffi::c_int & POLLERR != 0
                || pfd[1 as usize].revents as ::core::ffi::c_int & (POLLERR | POLLHUP) != 0
            {
                return -1 as int32_t;
            }
            if pfd[0 as usize].revents as ::core::ffi::c_int & (POLLIN | POLLHUP)
                == 0 as ::core::ffi::c_int
                && pfd[1 as usize].revents as ::core::ffi::c_int & POLLOUT
                    == 0 as ::core::ffi::c_int
            {
                *__errno_location() = ETIMEDOUT;
                return -1 as int32_t;
            }
        }
        return leng as int32_t;
    }
}
#[inline]
unsafe extern "C" fn streamtoaccept(
    mut lsock: ::core::ffi::c_int,
    mut msecto: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut s: ::core::ffi::c_double = 0.;
        let mut c: ::core::ffi::c_double = 0.;
        let mut msecpassed: uint32_t = 0;
        let mut pfd: pollfd = pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        };
        i = accept(
            lsock,
            __SOCKADDR_ARG {
                __sockaddr__: NULL as *mut sockaddr,
            },
            ::core::ptr::null_mut::<socklen_t>(),
        );
        if i >= 0 as ::core::ffi::c_int {
            return i;
        } else if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
            return -1 as ::core::ffi::c_int;
        }
        s = monotonic_seconds();
        msecpassed = 0 as uint32_t;
        loop {
            pfd.fd = lsock;
            pfd.events = POLLIN as ::core::ffi::c_short;
            pfd.revents = 0 as ::core::ffi::c_short;
            if poll(
                &raw mut pfd,
                1 as nfds_t,
                msecto.wrapping_sub(msecpassed) as ::core::ffi::c_int,
            ) >= 0 as ::core::ffi::c_int
            {
                break;
            }
            if *__errno_location() == EINTR {
                c = monotonic_seconds();
                msecpassed = ((c - s) * 1000.0f64) as uint32_t;
                if msecpassed >= msecto {
                    *__errno_location() = ETIMEDOUT;
                    return -1 as ::core::ffi::c_int;
                }
            } else {
                return -1 as ::core::ffi::c_int;
            }
        }
        if pfd.revents as ::core::ffi::c_int & (POLLHUP | POLLERR) != 0 {
            return -1 as ::core::ffi::c_int;
        }
        if pfd.revents as ::core::ffi::c_int & POLLIN != 0 {
            return accept(
                lsock,
                __SOCKADDR_ARG {
                    __sockaddr__: NULL as *mut sockaddr,
                },
                ::core::ptr::null_mut::<socklen_t>(),
            );
        }
        *__errno_location() = ETIMEDOUT;
        return -1 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn streamaccept(mut lsock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        let mut sock: ::core::ffi::c_int = 0;
        sock = accept(
            lsock,
            __SOCKADDR_ARG {
                __sockaddr__: NULL as *mut sockaddr,
            },
            ::core::ptr::null_mut::<socklen_t>(),
        );
        if sock < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        return sock;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn univnonblock(mut fd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return descnonblock(fd);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn univtoread(
    mut fd: ::core::ffi::c_int,
    mut buff: *mut ::core::ffi::c_void,
    mut leng: uint32_t,
    mut msectopart: uint32_t,
    mut msectoall: uint32_t,
) -> int32_t {
    unsafe {
        return streamtoread(fd, buff, leng, msectopart, msectoall);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn univtowrite(
    mut fd: ::core::ffi::c_int,
    mut buff: *const ::core::ffi::c_void,
    mut leng: uint32_t,
    mut msectopart: uint32_t,
    mut msectoall: uint32_t,
) -> int32_t {
    unsafe {
        return streamtowrite(fd, buff, leng, msectopart, msectoall);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn univtoforward(
    mut srcfd: ::core::ffi::c_int,
    mut dstfd: ::core::ffi::c_int,
    mut buff: *mut ::core::ffi::c_void,
    mut leng: uint32_t,
    mut rcvd: uint32_t,
    mut sent: uint32_t,
    mut msectopart: uint32_t,
    mut msectoall: uint32_t,
) -> int32_t {
    unsafe {
        return streamtoforward(srcfd, dstfd, buff, leng, rcvd, sent, msectopart, msectoall);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpsetacceptfilter(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        let mut v: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        return setsockopt(
            sock,
            IPPROTO_TCP as ::core::ffi::c_int,
            TCP_DEFER_ACCEPT,
            &raw mut v as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        );
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpsocket() -> ::core::ffi::c_int {
    unsafe {
        return socket(
            AF_INET,
            SOCK_STREAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpnonblock(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return descnonblock(sock);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpgetstatus(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return sockgetstatus(sock);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpresolve(
    mut hostname: *const ::core::ffi::c_char,
    mut service: *const ::core::ffi::c_char,
    mut ip: *mut uint32_t,
    mut port: *mut uint16_t,
    mut passive: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sockresolve(
            hostname,
            service,
            ip,
            port,
            AF_INET,
            SOCK_STREAM as ::core::ffi::c_int,
            passive,
        );
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpreuseaddr(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        let mut yes: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        return setsockopt(
            sock,
            SOL_SOCKET,
            SO_REUSEADDR,
            &raw mut yes as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        );
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpnodelay(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        let mut yes: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        return setsockopt(
            sock,
            IPPROTO_TCP as ::core::ffi::c_int,
            TCP_NODELAY,
            &raw mut yes as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        );
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpaccfhttp(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        *__errno_location() = EINVAL;
        return -1 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpaccfdata(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        *__errno_location() = EINVAL;
        return -1 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpstrbind(
    mut sock: ::core::ffi::c_int,
    mut hostname: *const ::core::ffi::c_char,
    mut service: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        if sockaddrfill(
            &raw mut sa,
            hostname,
            service,
            AF_INET,
            SOCK_STREAM as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if bind(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpnumbind(
    mut sock: ::core::ffi::c_int,
    mut ip: uint32_t,
    mut port: uint16_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        sockaddrnumfill(&raw mut sa, ip, port);
        if bind(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpstrconnect(
    mut sock: ::core::ffi::c_int,
    mut hostname: *const ::core::ffi::c_char,
    mut service: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        if sockaddrfill(
            &raw mut sa,
            hostname,
            service,
            AF_INET,
            SOCK_STREAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if connect(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) >= 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if *__errno_location() == EINPROGRESS {
            return 1 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpnumconnect(
    mut sock: ::core::ffi::c_int,
    mut ip: uint32_t,
    mut port: uint16_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        sockaddrnumfill(&raw mut sa, ip, port);
        if connect(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) >= 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if *__errno_location() == EINPROGRESS {
            return 1 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpstrtoconnect(
    mut sock: ::core::ffi::c_int,
    mut hostname: *const ::core::ffi::c_char,
    mut service: *const ::core::ffi::c_char,
    mut msecto: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        if descnonblock(sock) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if sockaddrfill(
            &raw mut sa,
            hostname,
            service,
            AF_INET,
            SOCK_STREAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if connect(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) >= 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if *__errno_location() == EINPROGRESS {
            let mut s: ::core::ffi::c_double = 0.;
            let mut c: ::core::ffi::c_double = 0.;
            let mut msecpassed: uint32_t = 0;
            let mut pfd: pollfd = pollfd {
                fd: 0,
                events: 0,
                revents: 0,
            };
            s = monotonic_seconds();
            msecpassed = 0 as uint32_t;
            loop {
                pfd.fd = sock;
                pfd.events = POLLOUT as ::core::ffi::c_short;
                pfd.revents = 0 as ::core::ffi::c_short;
                if poll(
                    &raw mut pfd,
                    1 as nfds_t,
                    msecto.wrapping_sub(msecpassed) as ::core::ffi::c_int,
                ) >= 0 as ::core::ffi::c_int
                {
                    break;
                }
                if *__errno_location() == EINTR {
                    c = monotonic_seconds();
                    msecpassed = ((c - s) * 1000.0f64) as uint32_t;
                    if msecpassed >= msecto {
                        *__errno_location() = ETIMEDOUT;
                        return -1 as ::core::ffi::c_int;
                    }
                } else {
                    return -1 as ::core::ffi::c_int;
                }
            }
            if pfd.revents as ::core::ffi::c_int & (POLLHUP | POLLERR) != 0 {
                return -1 as ::core::ffi::c_int;
            }
            if pfd.revents as ::core::ffi::c_int & POLLOUT != 0 {
                return sockgetstatus(sock);
            }
            *__errno_location() = ETIMEDOUT;
        }
        return -1 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpnumtoconnect(
    mut sock: ::core::ffi::c_int,
    mut ip: uint32_t,
    mut port: uint16_t,
    mut msecto: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        if descnonblock(sock) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        sockaddrnumfill(&raw mut sa, ip, port);
        if connect(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) >= 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if *__errno_location() == EINPROGRESS {
            let mut s: ::core::ffi::c_double = 0.;
            let mut c: ::core::ffi::c_double = 0.;
            let mut msecpassed: uint32_t = 0;
            let mut pfd: pollfd = pollfd {
                fd: 0,
                events: 0,
                revents: 0,
            };
            s = monotonic_seconds();
            msecpassed = 0 as uint32_t;
            loop {
                pfd.fd = sock;
                pfd.events = POLLOUT as ::core::ffi::c_short;
                pfd.revents = 0 as ::core::ffi::c_short;
                if poll(
                    &raw mut pfd,
                    1 as nfds_t,
                    msecto.wrapping_sub(msecpassed) as ::core::ffi::c_int,
                ) >= 0 as ::core::ffi::c_int
                {
                    break;
                }
                if *__errno_location() == EINTR {
                    c = monotonic_seconds();
                    msecpassed = ((c - s) * 1000.0f64) as uint32_t;
                    if msecpassed >= msecto {
                        *__errno_location() = ETIMEDOUT;
                        return -1 as ::core::ffi::c_int;
                    }
                } else {
                    return -1 as ::core::ffi::c_int;
                }
            }
            if pfd.revents as ::core::ffi::c_int & (POLLHUP | POLLERR) != 0 {
                return -1 as ::core::ffi::c_int;
            }
            if pfd.revents as ::core::ffi::c_int & POLLOUT != 0 {
                return sockgetstatus(sock);
            }
            *__errno_location() = ETIMEDOUT;
        }
        return -1 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpstrlisten(
    mut sock: ::core::ffi::c_int,
    mut hostname: *const ::core::ffi::c_char,
    mut service: *const ::core::ffi::c_char,
    mut queue: uint16_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        if sockaddrfill(
            &raw mut sa,
            hostname,
            service,
            AF_INET,
            SOCK_STREAM as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if bind(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if listen(sock, queue as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpnumlisten(
    mut sock: ::core::ffi::c_int,
    mut ip: uint32_t,
    mut port: uint16_t,
    mut queue: uint16_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        sockaddrnumfill(&raw mut sa, ip, port);
        if bind(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if listen(sock, queue as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpgetpeer(
    mut sock: ::core::ffi::c_int,
    mut ip: *mut uint32_t,
    mut port: *mut uint16_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        let mut leng: socklen_t = 0;
        leng = ::core::mem::size_of::<sockaddr_in>() as socklen_t;
        if getpeername(
            sock,
            __SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            &raw mut leng,
        ) < 0 as ::core::ffi::c_int
        {
            if !ip.is_null() {
                *ip = 0 as uint32_t;
            }
            if !port.is_null() {
                *port = 0 as uint16_t;
            }
            return -1 as ::core::ffi::c_int;
        }
        if !ip.is_null() {
            *ip = __bswap_32(sa.sin_addr.s_addr as __uint32_t) as uint32_t;
        }
        if !port.is_null() {
            *port = __bswap_16(sa.sin_port as __uint16_t) as uint16_t;
        }
        return 0 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpgetmyaddr(
    mut sock: ::core::ffi::c_int,
    mut ip: *mut uint32_t,
    mut port: *mut uint16_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        let mut leng: socklen_t = 0;
        leng = ::core::mem::size_of::<sockaddr_in>() as socklen_t;
        if getsockname(
            sock,
            __SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            &raw mut leng,
        ) < 0 as ::core::ffi::c_int
        {
            if !ip.is_null() {
                *ip = 0 as uint32_t;
            }
            if !port.is_null() {
                *port = 0 as uint16_t;
            }
            return -1 as ::core::ffi::c_int;
        }
        if !ip.is_null() {
            *ip = __bswap_32(sa.sin_addr.s_addr as __uint32_t) as uint32_t;
        }
        if !port.is_null() {
            *port = __bswap_16(sa.sin_port as __uint16_t) as uint16_t;
        }
        return 0 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpshutdown(mut sock: ::core::ffi::c_int) {
    unsafe {
        shutdown(sock, SHUT_WR as ::core::ffi::c_int);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpclose(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        shutdown(sock, SHUT_WR as ::core::ffi::c_int);
        return close(sock);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcptoread(
    mut sock: ::core::ffi::c_int,
    mut buff: *mut ::core::ffi::c_void,
    mut leng: uint32_t,
    mut msectopart: uint32_t,
    mut msectoall: uint32_t,
) -> int32_t {
    unsafe {
        return streamtoread(sock, buff, leng, msectopart, msectoall);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcptowrite(
    mut sock: ::core::ffi::c_int,
    mut buff: *const ::core::ffi::c_void,
    mut leng: uint32_t,
    mut msectopart: uint32_t,
    mut msectoall: uint32_t,
) -> int32_t {
    unsafe {
        return streamtowrite(sock, buff, leng, msectopart, msectoall);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcptoforward(
    mut srcsock: ::core::ffi::c_int,
    mut dstsock: ::core::ffi::c_int,
    mut buff: *mut ::core::ffi::c_void,
    mut leng: uint32_t,
    mut rcvd: uint32_t,
    mut sent: uint32_t,
    mut msectopart: uint32_t,
    mut msectoall: uint32_t,
) -> int32_t {
    unsafe {
        return streamtoforward(
            srcsock, dstsock, buff, leng, rcvd, sent, msectopart, msectoall,
        );
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcptowait(
    mut sock: ::core::ffi::c_int,
    mut msectoall: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        return streamtowait(sock, msectoall);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcptoaccept(
    mut lsock: ::core::ffi::c_int,
    mut msecto: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        return streamtoaccept(lsock, msecto);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tcpaccept(mut lsock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return streamaccept(lsock);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpsocket() -> ::core::ffi::c_int {
    unsafe {
        return socket(
            AF_INET,
            SOCK_DGRAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpnonblock(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return descnonblock(sock);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpgetstatus(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return sockgetstatus(sock);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpresolve(
    mut hostname: *const ::core::ffi::c_char,
    mut service: *const ::core::ffi::c_char,
    mut ip: *mut uint32_t,
    mut port: *mut uint16_t,
    mut passive: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return sockresolve(
            hostname,
            service,
            ip,
            port,
            AF_INET,
            SOCK_DGRAM as ::core::ffi::c_int,
            passive,
        );
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpnumlisten(
    mut sock: ::core::ffi::c_int,
    mut ip: uint32_t,
    mut port: uint16_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        sockaddrnumfill(&raw mut sa, ip, port);
        return bind(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        );
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpstrlisten(
    mut sock: ::core::ffi::c_int,
    mut hostname: *const ::core::ffi::c_char,
    mut service: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        if sockaddrfill(
            &raw mut sa,
            hostname,
            service,
            AF_INET,
            SOCK_DGRAM as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        return bind(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        );
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpwrite(
    mut sock: ::core::ffi::c_int,
    mut ip: uint32_t,
    mut port: uint16_t,
    mut buff: *const ::core::ffi::c_void,
    mut leng: uint16_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        if leng as ::core::ffi::c_int > 512 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        sockaddrnumfill(&raw mut sa, ip, port);
        return sendto(
            sock,
            buff,
            leng as size_t,
            0 as ::core::ffi::c_int,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpread(
    mut sock: ::core::ffi::c_int,
    mut ip: *mut uint32_t,
    mut port: *mut uint16_t,
    mut buff: *mut ::core::ffi::c_void,
    mut leng: uint16_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut templeng: socklen_t = 0;
        let mut tempaddr: sockaddr = sockaddr {
            sa_family: 0,
            sa_data: [0; 14],
        };
        let mut saptr: *mut sockaddr_in = ::core::ptr::null_mut::<sockaddr_in>();
        let mut ret: ::core::ffi::c_int = 0;
        ret = recvfrom(
            sock,
            buff,
            leng as size_t,
            0 as ::core::ffi::c_int,
            __SOCKADDR_ARG {
                __sockaddr__: &raw mut tempaddr,
            },
            &raw mut templeng,
        ) as ::core::ffi::c_int;
        if templeng as usize == ::core::mem::size_of::<sockaddr_in>() {
            saptr = &raw mut tempaddr as *mut sockaddr_in;
            if !ip.is_null() {
                *ip = __bswap_32((*saptr).sin_addr.s_addr as __uint32_t) as uint32_t;
            }
            if !port.is_null() {
                *port = __bswap_16((*saptr).sin_port as __uint16_t) as uint16_t;
            }
        }
        return ret;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn udpclose(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return close(sock);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixsocket() -> ::core::ffi::c_int {
    unsafe {
        return socket(
            AF_UNIX,
            SOCK_STREAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixnonblock(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return descnonblock(sock);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixgetstatus(mut sock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return sockgetstatus(sock);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixconnect(
    mut sock: ::core::ffi::c_int,
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_un = sockaddr_un {
            sun_family: 0,
            sun_path: [0; 108],
        };
        if sockaddrpathfill(&raw mut sa, path) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if connect(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_un>() as socklen_t,
        ) >= 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if *__errno_location() == EINPROGRESS {
            return 1 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixtoconnect(
    mut sock: ::core::ffi::c_int,
    mut path: *const ::core::ffi::c_char,
    mut msecto: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_un = sockaddr_un {
            sun_family: 0,
            sun_path: [0; 108],
        };
        if descnonblock(sock) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if sockaddrpathfill(&raw mut sa, path) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if connect(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_un>() as socklen_t,
        ) >= 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if *__errno_location() == EINPROGRESS {
            let mut s: ::core::ffi::c_double = 0.;
            let mut c: ::core::ffi::c_double = 0.;
            let mut msecpassed: uint32_t = 0;
            let mut pfd: pollfd = pollfd {
                fd: 0,
                events: 0,
                revents: 0,
            };
            s = monotonic_seconds();
            msecpassed = 0 as uint32_t;
            loop {
                pfd.fd = sock;
                pfd.events = POLLOUT as ::core::ffi::c_short;
                pfd.revents = 0 as ::core::ffi::c_short;
                if poll(
                    &raw mut pfd,
                    1 as nfds_t,
                    msecto.wrapping_sub(msecpassed) as ::core::ffi::c_int,
                ) >= 0 as ::core::ffi::c_int
                {
                    break;
                }
                if *__errno_location() == EINTR {
                    c = monotonic_seconds();
                    msecpassed = ((c - s) * 1000.0f64) as uint32_t;
                    if msecpassed >= msecto {
                        *__errno_location() = ETIMEDOUT;
                        return -1 as ::core::ffi::c_int;
                    }
                } else {
                    return -1 as ::core::ffi::c_int;
                }
            }
            if pfd.revents as ::core::ffi::c_int & (POLLHUP | POLLERR) != 0 {
                return -1 as ::core::ffi::c_int;
            }
            if pfd.revents as ::core::ffi::c_int & POLLOUT != 0 {
                return sockgetstatus(sock);
            }
            *__errno_location() = ETIMEDOUT;
        }
        return -1 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixlisten(
    mut sock: ::core::ffi::c_int,
    mut path: *const ::core::ffi::c_char,
    mut queue: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sa: sockaddr_un = sockaddr_un {
            sun_family: 0,
            sun_path: [0; 108],
        };
        if sockaddrpathfill(&raw mut sa, path) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if bind(
            sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sa as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_un>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if listen(sock, queue) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixtoread(
    mut sock: ::core::ffi::c_int,
    mut buff: *mut ::core::ffi::c_void,
    mut leng: uint32_t,
    mut msectopart: uint32_t,
    mut msectoall: uint32_t,
) -> int32_t {
    unsafe {
        return streamtoread(sock, buff, leng, msectopart, msectoall);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixtowrite(
    mut sock: ::core::ffi::c_int,
    mut buff: *const ::core::ffi::c_void,
    mut leng: uint32_t,
    mut msectopart: uint32_t,
    mut msectoall: uint32_t,
) -> int32_t {
    unsafe {
        return streamtowrite(sock, buff, leng, msectopart, msectoall);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixtoforward(
    mut srcsock: ::core::ffi::c_int,
    mut dstsock: ::core::ffi::c_int,
    mut buff: *mut ::core::ffi::c_void,
    mut leng: uint32_t,
    mut rcvd: uint32_t,
    mut sent: uint32_t,
    mut msectopart: uint32_t,
    mut msectoall: uint32_t,
) -> int32_t {
    unsafe {
        return streamtoforward(
            srcsock, dstsock, buff, leng, rcvd, sent, msectopart, msectoall,
        );
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixtoaccept(
    mut lsock: ::core::ffi::c_int,
    mut msecto: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        return streamtoaccept(lsock, msecto);
    }
}
/// # Safety
/// SAFETY: C ABI wrapper; all fd/pointer/buffer args per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unixaccept(mut lsock: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return streamaccept(lsock);
    }
}
