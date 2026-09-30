//! strerr — errno → descriptive string (port of mfscommon/strerr.c).
//!
//! Safe core: a static table searched first-match-wins (the C open-addressed
//! hash keeps the first entry for duplicate errnos, so a linear scan yields
//! identical results). Unknown errors format "Unknown error: %d" into a
//! per-thread buffer valid until the thread's next call (the C
//! `USE_PTHREADS` pthread_key contract; single-threaded builds are
//! observationally identical).
use core::ffi::{c_char, c_int};

pub use imp::{ERRTAB, STRERR_BUFF_SIZE, known, strerr_ptr, unknown_message};

#[deny(unsafe_code)]
mod imp {
use core::ffi::{CStr, c_char, c_int};
use std::cell::RefCell;
use std::ffi::CString;

/// C: `#define STRERR_BUFF_SIZE 100` (output truncated to 99 bytes + NUL).
pub const STRERR_BUFF_SIZE: usize = 100;

/// errtab from strerr.c, in source order, filtered by the Linux errno set.
pub static ERRTAB: &[(c_int, &CStr)] = &[
    (7, c"E2BIG (Argument list too long)"),
    (13, c"EACCES (Permission denied)"),
    (98, c"EADDRINUSE (Address already in use)"),
    (99, c"EADDRNOTAVAIL (Cannot assign requested address)"),
    (68, c"EADV (Advertise error)"),
    (97, c"EAFNOSUPPORT (Address family not supported by protocol family)"),
    (11, c"EAGAIN (Resource temporarily unavailable)"),
    (114, c"EALREADY (Operation already in progress)"),
    (52, c"EBADE (Invalid exchange)"),
    (77, c"EBADFD (File descriptor invalid for this operation)"),
    (9, c"EBADF (Bad file descriptor)"),
    (74, c"EBADMSG (Bad message)"),
    (56, c"EBADRQC (Invalid request code)"),
    (53, c"EBADR (Invalid request descriptor)"),
    (57, c"EBADSLT (Invalid slot)"),
    (59, c"EBFONT (Bad font file format)"),
    (16, c"EBUSY (Device or resource busy)"),
    (125, c"ECANCELED (Operation canceled)"),
    (10, c"ECHILD (No child processes)"),
    (44, c"ECHRNG (Channel number out of range)"),
    (70, c"ECOMM (Communication error on send)"),
    (103, c"ECONNABORTED (Software caused connection abort)"),
    (111, c"ECONNREFUSED (Connection refused)"),
    (104, c"ECONNRESET (Connection reset by peer)"),
    (35, c"EDEADLK (Resource deadlock would occur)"),
    (35, c"EDEADLOCK (File locking deadlock error)"),
    (89, c"EDESTADDRREQ (Destination address required)"),
    (33, c"EDOM (Numerical argument out of domain)"),
    (73, c"EDOTDOT (RFS specific error)"),
    (122, c"EDQUOT (Quota exceeded)"),
    (17, c"EEXIST (File exists)"),
    (14, c"EFAULT (Bad address)"),
    (27, c"EFBIG (File too large)"),
    (112, c"EHOSTDOWN (Host is down)"),
    (113, c"EHOSTUNREACH (No route to host)"),
    (43, c"EIDRM (Identifier removed)"),
    (84, c"EILSEQ (Illegal byte sequence)"),
    (115, c"EINPROGRESS (Operation now in progress)"),
    (4, c"EINTR (Interrupted system call)"),
    (22, c"EINVAL (Invalid argument)"),
    (5, c"EIO (Input/output error)"),
    (106, c"EISCONN (Transport endpoint is already connected)"),
    (21, c"EISDIR (Is a directory)"),
    (120, c"EISNAM (Is a named type file)"),
    (127, c"EKEYEXPIRED (Key has expired)"),
    (129, c"EKEYREJECTED (Key was rejected by service)"),
    (128, c"EKEYREVOKED (Key has been revoked)"),
    (51, c"EL2HLT (Level 2 halted)"),
    (45, c"EL2NSYNC (Level 2 not synchronized)"),
    (46, c"EL3HLT (Level 3 halted)"),
    (47, c"EL3RST (Level 3 reset)"),
    (79, c"ELIBACC (Can not access a needed shared library)"),
    (80, c"ELIBBAD (Accessing a corrupted shared library)"),
    (83, c"ELIBEXEC (Cannot exec a shared library directly)"),
    (82, c"ELIBMAX (Attempting to link in too many shared libraries)"),
    (81, c"ELIBSCN (.lib section in a.out corrupted)"),
    (48, c"ELNRNG (Link number out of range)"),
    (40, c"ELOOP (Too many levels of symbolic links)"),
    (124, c"EMEDIUMTYPE (Wrong medium type)"),
    (24, c"EMFILE (Too many open files)"),
    (31, c"EMLINK (Too many links)"),
    (90, c"EMSGSIZE (Message too long)"),
    (72, c"EMULTIHOP (Multihop attempted)"),
    (36, c"ENAMETOOLONG (File name too long)"),
    (119, c"ENAVAIL (No XENIX semaphores available)"),
    (100, c"ENETDOWN (Network is down)"),
    (102, c"ENETRESET (Network dropped connection because of reset)"),
    (101, c"ENETUNREACH (Network is unreachable)"),
    (23, c"ENFILE (Too many open files in system)"),
    (55, c"ENOANO (No anode)"),
    (105, c"ENOBUFS (No buffer space available)"),
    (50, c"ENOCSI (No CSI structure available)"),
    (61, c"ENODATA (No data available)"),
    (19, c"ENODEV (Operation not supported by device or no such device)"),
    (2, c"ENOENT (No such file or directory)"),
    (8, c"ENOEXEC (Exec format error)"),
    (126, c"ENOKEY (Required key not available)"),
    (37, c"ENOLCK (No locks available)"),
    (67, c"ENOLINK (Link has been severed)"),
    (123, c"ENOMEDIUM (No medium found)"),
    (12, c"ENOMEM (Cannot allocate memory)"),
    (42, c"ENOMSG (No message of desired type)"),
    (64, c"ENONET (Machine is not on the network)"),
    (65, c"ENOPKG (Package not installed)"),
    (92, c"ENOPROTOOPT (Protocol not available)"),
    (28, c"ENOSPC (No space left on device)"),
    (63, c"ENOSR (Out of streams resources)"),
    (60, c"ENOSTR (Device not a stream)"),
    (38, c"ENOSYS (Unsupported file system operation)"),
    (15, c"ENOTBLK (Block device required)"),
    (107, c"ENOTCONN (Transport endpoint is not connected)"),
    (20, c"ENOTDIR (Not a directory)"),
    (39, c"ENOTEMPTY (Directory not empty)"),
    (118, c"ENOTNAM (Not a XENIX named type file)"),
    (131, c"ENOTRECOVERABLE (State not recoverable)"),
    (88, c"ENOTSOCK (Socket operation on non-socket)"),
    (95, c"ENOTSUP (Operation not supported)"),
    (25, c"ENOTTY (Inappropriate ioctl for device)"),
    (76, c"ENOTUNIQ (Name not unique on network)"),
    (6, c"ENXIO (No such device or address)"),
    (95, c"EOPNOTSUPP (Operation not supported on transport endpoint)"),
    (75, c"EOVERFLOW (Value too large to be stored in data type)"),
    (130, c"EOWNERDEAD (Process died with the lock)"),
    (1, c"EPERM (Operation not permitted)"),
    (96, c"EPFNOSUPPORT (Protocol family not supported)"),
    (32, c"EPIPE (Broken pipe)"),
    (93, c"EPROTONOSUPPORT (Protocol not supported)"),
    (91, c"EPROTOTYPE (Protocol wrong type for socket)"),
    (71, c"EPROTO (Protocol error)"),
    (34, c"ERANGE (Result too large)"),
    (78, c"EREMCHG (Remote address changed)"),
    (121, c"EREMOTEIO (Remote I/O error)"),
    (66, c"EREMOTE (Object is remote)"),
    (85, c"ERESTART (Interrupted system call should be restarted)"),
    (30, c"EROFS (Read-only file system)"),
    (108, c"ESHUTDOWN (Cannot send after transport endpoint shutdown)"),
    (94, c"ESOCKTNOSUPPORT (Socket type not supported)"),
    (29, c"ESPIPE (Illegal seek)"),
    (3, c"ESRCH (No such process)"),
    (69, c"ESRMNT (Srmount error)"),
    (116, c"ESTALE (Stale NFS file handle)"),
    (86, c"ESTRPIPE (Streams pipe error)"),
    (110, c"ETIMEDOUT (Operation timed out)"),
    (62, c"ETIME (Timer expired)"),
    (109, c"ETOOMANYREFS (Too many references: cannot splice)"),
    (26, c"ETXTBSY (Text file busy)"),
    (117, c"EUCLEAN (Structure needs cleaning)"),
    (49, c"EUNATCH (Protocol driver not attached)"),
    (87, c"EUSERS (Too many users)"),
    (18, c"EXDEV (Cross-device link)"),
    (54, c"EXFULL (Exchange full)"),
];

/// Look up a known errno (0 → "Success (errno=0)").
pub fn known(error: c_int) -> Option<&'static CStr> {
    if error == 0 {
        return Some(c"Success (errno=0)");
    }
    ERRTAB.iter().find(|(n, _)| *n == error).map(|(_, s)| *s)
}

/// Format the unknown-error message exactly as snprintf would (truncated).
pub fn unknown_message(error: c_int) -> CString {
    let mut s = format!("Unknown error: {error}").into_bytes();
    s.truncate(STRERR_BUFF_SIZE - 1);
    CString::new(s).expect("no interior NUL in formatted integer")
}

thread_local! {
    static STRBUFF: RefCell<CString> = RefCell::new(CString::default());
}

/// Pointer-returning lookup used by the C ABI export.
pub fn strerr_ptr(error: c_int) -> *const c_char {
    if let Some(s) = known(error) {
        return s.as_ptr();
    }
    STRBUFF.with(|b| {
        let mut b = b.borrow_mut();
        *b = unknown_message(error);
        // Heap buffer owned by this thread's TLS; stays valid until the
        // thread's next unknown-errno call, matching the C contract.
        b.as_ptr()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_and_zero() {
        assert_eq!(known(0).unwrap().to_str().unwrap(), "Success (errno=0)");
        assert!(known(1).unwrap().to_str().unwrap().starts_with("EPERM"));
        assert!(known(12).unwrap().to_str().unwrap().starts_with("ENOMEM"));
    }

    #[test]
    fn unknown_formats_like_snprintf() {
        assert_eq!(unknown_message(4999).to_str().unwrap(), "Unknown error: 4999");
        assert_eq!(unknown_message(-7).to_str().unwrap(), "Unknown error: -7");
        assert_eq!(unknown_message(i32::MIN).to_str().unwrap(), "Unknown error: -2147483648");
    }

    #[test]
    fn first_match_wins() {
        let mut seen = Vec::new();
        for (n, s) in ERRTAB {
            if !seen.contains(n) {
                seen.push(*n);
                assert_eq!(known(*n).unwrap().as_ptr(), s.as_ptr());
            }
        }
    }

    #[test]
    fn table_is_nonempty_and_positive() {
        assert!(ERRTAB.len() > 100);
        assert!(ERRTAB.iter().all(|(n, _)| *n > 0));
    }
}
}

// ---- C ABI (consumers link by symbol; signatures unchanged) ----

/// C: `void strerr_init(void)` — table needs no building.
#[unsafe(no_mangle)]
pub extern "C" fn strerr_init() {}

/// C: `void strerr_term(void)`.
#[unsafe(no_mangle)]
pub extern "C" fn strerr_term() {}

/// C: `const char* strerr(int error)`.
// SAFETY: exported by symbol name for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn strerr(error: c_int) -> *const c_char {
    imp::strerr_ptr(error)
}

