use libc::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct errent {
    pub num: ::core::ffi::c_int,
    pub str: *const ::core::ffi::c_char,
}
macro_rules! errtab {
    ($($name:ident => $description:literal),+ $(,)?) => {
        const ERRTAB: &[errent] = &[$(errent {
            num: $name,
            str: concat!(stringify!($name), " (", $description, ")\0").as_ptr()
                as *const ::core::ffi::c_char,
        },)+];
    };
}

errtab! {
    E2BIG => "Argument list too long",
    EACCES => "Permission denied",
    EADDRINUSE => "Address already in use",
    EADDRNOTAVAIL => "Cannot assign requested address",
    EADV => "Advertise error",
    EAFNOSUPPORT => "Address family not supported by protocol family",
    EAGAIN => "Resource temporarily unavailable",
    EALREADY => "Operation already in progress",
    EBADE => "Invalid exchange",
    EBADFD => "File descriptor invalid for this operation",
    EBADF => "Bad file descriptor",
    EBADMSG => "Bad message",
    EBADRQC => "Invalid request code",
    EBADR => "Invalid request descriptor",
    EBADSLT => "Invalid slot",
    EBFONT => "Bad font file format",
    EBUSY => "Device or resource busy",
    ECANCELED => "Operation canceled",
    ECHILD => "No child processes",
    ECHRNG => "Channel number out of range",
    ECOMM => "Communication error on send",
    ECONNABORTED => "Software caused connection abort",
    ECONNREFUSED => "Connection refused",
    ECONNRESET => "Connection reset by peer",
    EDEADLK => "Resource deadlock would occur",
    EDEADLOCK => "File locking deadlock error",
    EDESTADDRREQ => "Destination address required",
    EDOM => "Numerical argument out of domain",
    EDOTDOT => "RFS specific error",
    EDQUOT => "Quota exceeded",
    EEXIST => "File exists",
    EFAULT => "Bad address",
    EFBIG => "File too large",
    EHOSTDOWN => "Host is down",
    EHOSTUNREACH => "No route to host",
    EIDRM => "Identifier removed",
    EILSEQ => "Illegal byte sequence",
    EINPROGRESS => "Operation now in progress",
    EINTR => "Interrupted system call",
    EINVAL => "Invalid argument",
    EIO => "Input/output error",
    EISCONN => "Transport endpoint is already connected",
    EISDIR => "Is a directory",
    EISNAM => "Is a named type file",
    EKEYEXPIRED => "Key has expired",
    EKEYREJECTED => "Key was rejected by service",
    EKEYREVOKED => "Key has been revoked",
    EL2HLT => "Level 2 halted",
    EL2NSYNC => "Level 2 not synchronized",
    EL3HLT => "Level 3 halted",
    EL3RST => "Level 3 reset",
    ELIBACC => "Can not access a needed shared library",
    ELIBBAD => "Accessing a corrupted shared library",
    ELIBEXEC => "Cannot exec a shared library directly",
    ELIBMAX => "Attempting to link in too many shared libraries",
    ELIBSCN => ".lib section in a.out corrupted",
    ELNRNG => "Link number out of range",
    ELOOP => "Too many levels of symbolic links",
    EMEDIUMTYPE => "Wrong medium type",
    EMFILE => "Too many open files",
    EMLINK => "Too many links",
    EMSGSIZE => "Message too long",
    EMULTIHOP => "Multihop attempted",
    ENAMETOOLONG => "File name too long",
    ENAVAIL => "No XENIX semaphores available",
    ENETDOWN => "Network is down",
    ENETRESET => "Network dropped connection because of reset",
    ENETUNREACH => "Network is unreachable",
    ENFILE => "Too many open files in system",
    ENOANO => "No anode",
    ENOBUFS => "No buffer space available",
    ENOCSI => "No CSI structure available",
    ENODATA => "No data available",
    ENODEV => "Operation not supported by device or no such device",
    ENOENT => "No such file or directory",
    ENOEXEC => "Exec format error",
    ENOKEY => "Required key not available",
    ENOLCK => "No locks available",
    ENOLINK => "Link has been severed",
    ENOMEDIUM => "No medium found",
    ENOMEM => "Cannot allocate memory",
    ENOMSG => "No message of desired type",
    ENONET => "Machine is not on the network",
    ENOPKG => "Package not installed",
    ENOPROTOOPT => "Protocol not available",
    ENOSPC => "No space left on device",
    ENOSR => "Out of streams resources",
    ENOSTR => "Device not a stream",
    ENOSYS => "Unsupported file system operation",
    ENOTBLK => "Block device required",
    ENOTCONN => "Transport endpoint is not connected",
    ENOTDIR => "Not a directory",
    ENOTEMPTY => "Directory not empty",
    ENOTNAM => "Not a XENIX named type file",
    ENOTRECOVERABLE => "State not recoverable",
    ENOTSOCK => "Socket operation on non-socket",
    ENOTSUP => "Operation not supported",
    ENOTTY => "Inappropriate ioctl for device",
    ENOTUNIQ => "Name not unique on network",
    ENXIO => "No such device or address",
    EOPNOTSUPP => "Operation not supported on transport endpoint",
    EOVERFLOW => "Value too large to be stored in data type",
    EOWNERDEAD => "Process died with the lock",
    EPERM => "Operation not permitted",
    EPFNOSUPPORT => "Protocol family not supported",
    EPIPE => "Broken pipe",
    EPROTONOSUPPORT => "Protocol not supported",
    EPROTOTYPE => "Protocol wrong type for socket",
    EPROTO => "Protocol error",
    ERANGE => "Result too large",
    EREMCHG => "Remote address changed",
    EREMOTEIO => "Remote I/O error",
    EREMOTE => "Object is remote",
    ERESTART => "Interrupted system call should be restarted",
    EROFS => "Read-only file system",
    ESHUTDOWN => "Cannot send after transport endpoint shutdown",
    ESOCKTNOSUPPORT => "Socket type not supported",
    ESPIPE => "Illegal seek",
    ESRCH => "No such process",
    ESRMNT => "Srmount error",
    ESTALE => "Stale NFS file handle",
    ESTRPIPE => "Streams pipe error",
    ETIMEDOUT => "Operation timed out",
    ETIME => "Timer expired",
    ETOOMANYREFS => "Too many references: cannot splice",
    ETXTBSY => "Text file busy",
    EUCLEAN => "Structure needs cleaning",
    EUNATCH => "Protocol driver not attached",
    EUSERS => "Too many users",
    EXDEV => "Cross-device link",
    EXFULL => "Exchange full",
}
pub const STRERR_BUFF_SIZE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub extern "C" fn strerr_init() {}

#[unsafe(no_mangle)]
pub extern "C" fn strerr_term() {}

thread_local! {
    static STRBUFF: ::core::cell::UnsafeCell<[u8; STRERR_BUFF_SIZE as usize]> =
        const { ::core::cell::UnsafeCell::new([0; STRERR_BUFF_SIZE as usize]) };
}

fn format_unknown(buffer: &mut [u8], error: ::core::ffi::c_int) {
    use std::io::Write;

    let end = buffer.len() - 1;
    let mut output = &mut buffer[..end];
    let capacity = output.len();
    let _ = write!(output, "Unknown error: {error}");
    let len = capacity - output.len();
    buffer[len] = 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    if error == 0 {
        return b"Success (errno=0)\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if let Some(entry) = ERRTAB.iter().find(|entry| entry.num == error) {
        return entry.str;
    }
    STRBUFF.with(|storage| {
        // SAFETY: storage is thread-local; pointer remains valid until this thread's next call.
        let buffer = unsafe { &mut *storage.get() };
        format_unknown(buffer, error);
        buffer.as_ptr().cast()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ffi::CStr;

    fn text(error: ::core::ffi::c_int) -> &'static str {
        // SAFETY: strerr returns a NUL-terminated process-lifetime or thread-local string.
        unsafe { CStr::from_ptr(strerr(error)).to_str().unwrap() }
    }

    #[test]
    fn resolves_known_success_and_unknown_errors() {
        assert_eq!(text(0), "Success (errno=0)");
        assert_eq!(text(EPERM), "EPERM (Operation not permitted)");
        assert_eq!(text(4999), "Unknown error: 4999");
        assert_eq!(text(-4999), "Unknown error: -4999");
    }
}
