//! Logging (mfs_log family), migrated (P1).
//!
//! The two variadic leaves (`mfs_log`, `mfs_file_log`) keep their C-variadic
//! ABI — they are the confinement point for `c_variadic` (see porting.md).
//! Everything around them became safe: globals are atomics (relaxed —
//! diagnostic flags, matching the original's unsynchronized semantics),
//! priority-string table is private (verified unreferenced outside this
//! module), `mfs_file_log`'s fn-local FILE* is a module atomic.

pub type size_t = usize;
pub type FILE = libc::FILE;

use std::sync::atomic::{AtomicI32, AtomicPtr, Ordering};

pub const LOG_ERR: ::core::ffi::c_int = 3;
pub const LOG_WARNING: ::core::ffi::c_int = 4;
pub const LOG_NOTICE: ::core::ffi::c_int = 5;
pub const LOG_INFO: ::core::ffi::c_int = 6;
pub const LOG_DEBUG: ::core::ffi::c_int = 7;
pub const LOG_USER: ::core::ffi::c_int = 1 << 3;
pub const LOG_DAEMON: ::core::ffi::c_int = 3 << 3;
pub const LOG_PID: ::core::ffi::c_int = 0x1;
pub const LOG_NDELAY: ::core::ffi::c_int = 0x8;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0;
pub const MFSLOG_ERRNO_SYSLOG: ::core::ffi::c_int = 1;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3;
pub const MFSLOG_DEBUG: ::core::ffi::c_int = 0;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4;
pub const MFSLOG_PRI_MIN: ::core::ffi::c_int = MFSLOG_DEBUG;
pub const MFSLOG_PRI_MAX: ::core::ffi::c_int = MFSLOG_ERR;
pub const LOGBUFFSIZE: usize = 2048;
pub const MSGBUFFSIZE: usize = 4096;
pub const BT_BUF_SIZE: ::core::ffi::c_int = 100;

// charts.rs declares fprintf with its own _IO_FILE; both are correct opaque
// FILE* ABIs — the mismatch is the c2rust standalone-module artifact, benign.
#[allow(clashing_extern_declarations)]
unsafe extern "C" {
    // libc crate doesn't expose these (variadic / statics)
    static mut stderr: *mut FILE;
    fn vfprintf(
        s: *mut FILE,
        format: *const ::core::ffi::c_char,
        arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn vsnprintf(
        s: *mut ::core::ffi::c_char,
        n: size_t,
        format: *const ::core::ffi::c_char,
        arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn fprintf(s: *mut FILE, format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}

static MFS_LOG_SINK: AtomicPtr<::core::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());
static FORCE_STDERR: AtomicI32 = AtomicI32::new(0);
static USE_COLORS: AtomicI32 = AtomicI32::new(1);
static STDERR_ACTIVE: AtomicI32 = AtomicI32::new(1);
static MFS_LOG_MIN_LEVEL: AtomicI32 = AtomicI32::new(MFSLOG_INFO);
static MFS_LOG_ELEVATE_TO: AtomicI32 = AtomicI32::new(MFSLOG_NOTICE);
static SYSLOG_OPEN: AtomicI32 = AtomicI32::new(0);
static LFD: AtomicPtr<FILE> = AtomicPtr::new(std::ptr::null_mut());

// SAFETY (Sync): entries are immutable pointers to immutable static bytes.
unsafe impl Sync for PriorityStrings {}
struct PriorityStrings([*const ::core::ffi::c_char; 5]);
static PRIORITY_STRINGS: PriorityStrings = PriorityStrings([
    b"debug\0".as_ptr() as *const ::core::ffi::c_char,
    b"info\0".as_ptr() as *const ::core::ffi::c_char,
    b"notice\0".as_ptr() as *const ::core::ffi::c_char,
    b"warning\0".as_ptr() as *const ::core::ffi::c_char,
    b"error\0".as_ptr() as *const ::core::ffi::c_char,
]);

const COLOR_DEBUG: &[u8] = b"\x1B[0;90m\0";
const COLOR_INFO: &[u8] = b"\0";
const COLOR_NOTICE: &[u8] = b"\x1B[1;97m\0";
const COLOR_WARNING: &[u8] = b"\x1B[1;93m\0";
const COLOR_ERROR: &[u8] = b"\x1B[1;31m\0";
const COLOR_CLEAR: &[u8] = b"\x1B(B\x1B[m\0";

fn pri_to_str(priority: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    if (MFSLOG_PRI_MIN..=MFSLOG_PRI_MAX).contains(&priority) {
        PRIORITY_STRINGS.0[priority as usize]
    } else {
        b"unknown\0".as_ptr() as *const ::core::ffi::c_char
    }
}

fn pri_to_colorstr(priority: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    let s: &[u8] = match priority {
        MFSLOG_DEBUG => COLOR_DEBUG,
        MFSLOG_INFO => COLOR_INFO,
        MFSLOG_NOTICE => COLOR_NOTICE,
        MFSLOG_WARNING => COLOR_WARNING,
        MFSLOG_ERR => COLOR_ERROR,
        _ => b"\0",
    };
    s.as_ptr() as *const ::core::ffi::c_char
}

fn priority_convert(priority: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let elevated = priority.max(MFS_LOG_ELEVATE_TO.load(Ordering::Relaxed));
    match elevated {
        MFSLOG_DEBUG => LOG_DEBUG,
        MFSLOG_INFO => LOG_INFO,
        MFSLOG_NOTICE => LOG_NOTICE,
        MFSLOG_WARNING => LOG_WARNING,
        _ => LOG_ERR,
    }
}

/// # Safety
/// `pristr` must be a valid NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log_str_to_pri(
    mut pristr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    // SAFETY: per fn contract.
    let s = unsafe { std::ffi::CStr::from_ptr(pristr) }.to_bytes();
    let mut fpri = -1;
    let mut fpri_idx: Option<usize> = None;
    for (i, &c0) in s.iter().enumerate() {
        let c = c0.to_ascii_lowercase();
        if !c.is_ascii_lowercase() {
            return -1;
        }
        match fpri_idx {
            Some(j) => {
                // compare against the already-chosen candidate at same pos
                // SAFETY: PRIORITY_STRINGS entries are static NUL-terminated.
                let cand = unsafe { *PRIORITY_STRINGS.0[j].add(i) } as u8;
                if cand != c {
                    return -1;
                }
            }
            None => {
                for j in MFSLOG_PRI_MIN..=MFSLOG_PRI_MAX {
                    // SAFETY: static NUL-terminated strings.
                    let cand = unsafe { *PRIORITY_STRINGS.0[j as usize].add(i) } as u8;
                    if cand == c {
                        fpri_idx = Some(j as usize);
                        fpri = j;
                    }
                }
                if fpri_idx.is_none() {
                    return -1; // none matched
                }
            }
        }
    }
    fpri
}

/// Debug backtrace logger (mfsdebug.txt). Variadic leaf.
///
/// # Safety
/// C-variadic; `fmt` (and file/func when non-null) must be valid C strings,
/// with arguments matching fmt, per the C caller contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_file_log(
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
    mut bt: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    if fmt.is_null() {
        let lfd = LFD.swap(std::ptr::null_mut(), Ordering::Relaxed);
        if !lfd.is_null() {
            // SAFETY: extern; lfd was opened by us and not yet closed.
            unsafe { libc::fclose(lfd) };
        }
        return;
    }
    let mut lfd = LFD.load(Ordering::Relaxed);
    if lfd.is_null() {
        // SAFETY: extern; literals are valid C strings.
        lfd = unsafe {
            libc::fopen(
                b"mfsdebug.txt\0".as_ptr() as *const _,
                b"a\0".as_ptr() as *const _,
            )
        };
        if lfd.is_null() {
            return;
        }
        LFD.store(lfd, Ordering::Relaxed);
    }
    // file/func are valid C strings per contract; fmt and the variadic
    // SAFETY: pack are forwarded to vfprintf unmodified (C caller contract).
    unsafe {
        libc::fprintf(lfd, b"%s:%d (%s):\0".as_ptr() as *const _, file, line, func);
        let ap = c2rust_args.clone();
        vfprintf(lfd, fmt, ap);
        libc::fprintf(lfd, b"\n\0".as_ptr() as *const _);
    }
    if bt != 0 {
        let mut btbuf = [std::ptr::null_mut::<::core::ffi::c_void>(); 100];
        // SAFETY: extern; btbuf valid for 100 entries.
        let n = unsafe { libc::backtrace(btbuf.as_mut_ptr(), BT_BUF_SIZE) };
        // SAFETY: extern; btbuf filled for n entries by the call above.
        let btstr = unsafe { libc::backtrace_symbols(btbuf.as_ptr(), n) };
        if !btstr.is_null() {
            for i in 1..n {
                // SAFETY: extern; btstr valid for n strings; lfd open.
                unsafe {
                    libc::fprintf(
                        lfd,
                        b"\t%u: %s\n\0".as_ptr() as *const _,
                        i,
                        *btstr.offset(i as isize),
                    )
                };
            }
            // SAFETY: extern; btstr from backtrace_symbols.
            unsafe { libc::free(btstr as *mut ::core::ffi::c_void) };
        } else {
            for i in 1..n {
                // SAFETY: extern; lfd open; btbuf entries printable as %p.
                unsafe {
                    libc::fprintf(
                        lfd,
                        b"\t%u: [%p]\n\0".as_ptr() as *const _,
                        i,
                        btbuf[i as usize],
                    )
                };
            }
        }
    }
}

/// Main logger. Variadic leaf — the message is still formatted by libc's
/// vsnprintf (variadic packs cannot be constructed in safe Rust).
///
/// # Safety
/// C-variadic; `fmt` must be a valid C string with matching arguments.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log(
    mut mode: ::core::ffi::c_int,
    mut priority: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    if priority < MFS_LOG_MIN_LEVEL.load(Ordering::Relaxed) {
        return;
    }
    // C reads errno (via strerr) before vsnprintf can clobber it.
    let err = if mode & 1 != 0 {
        std::io::Error::last_os_error().raw_os_error()
    } else {
        None
    };
    let mut p = [0 as ::core::ffi::c_char; LOGBUFFSIZE];
    // SAFETY: p valid for LOGBUFFSIZE; fmt+args forwarded per contract.
    let n = unsafe {
        let ap = c2rust_args.clone();
        vsnprintf(p.as_mut_ptr(), LOGBUFFSIZE, fmt, ap)
    };
    if n < 0 {
        return;
    }
    p[LOGBUFFSIZE - 1] = 0;
    // SAFETY: p is NUL-terminated (last byte forced to 0 above).
    let text = unsafe { std::ffi::CStr::from_ptr(p.as_ptr()) }.to_bytes();
    emit(mode, priority, text, err.unwrap_or(0));
}

/// Safe logging entry for Rust callers: `text` is the already-formatted
/// message (what C's `vsnprintf` would produce). With `mode & 1` the
/// current thread's errno is appended, exactly as in `mfs_log`.
pub fn log_bytes(mode: ::core::ffi::c_int, priority: ::core::ffi::c_int, text: &[u8]) {
    let err = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
    log_bytes_errno(mode, priority, text, err);
}

/// As [`log_bytes`], with an explicit errno value for `mode & 1`.
pub fn log_bytes_errno(
    mode: ::core::ffi::c_int,
    priority: ::core::ffi::c_int,
    text: &[u8],
    errno: ::core::ffi::c_int,
) {
    if priority < MFS_LOG_MIN_LEVEL.load(Ordering::Relaxed) {
        return;
    }
    // C: vsnprintf into p[LOGBUFFSIZE]; %s stops at the first NUL.
    let text = &text[..text.iter().position(|&b| b == 0).unwrap_or(text.len())];
    emit(mode, priority, &text[..text.len().min(LOGBUFFSIZE - 1)], errno);
}

/// C: `massert.h` `passert` failure path — report and abort.
pub fn oom_abort(file: &str, line: u32, what: &str) -> ! {
    let text = format!("{file}:{line} - out of memory: {what} is NULL");
    eprintln!("{text}");
    log_bytes_errno(MFSLOG_SYSLOG, MFSLOG_ERR, text.as_bytes(), 0);
    std::process::abort()
}

/// Shared tail of `mfs_log`: `p` is the formatted body (no NUL, at most
/// LOGBUFFSIZE-1 bytes).
fn emit(mode: ::core::ffi::c_int, priority: ::core::ffi::c_int, p: &[u8], errno: ::core::ffi::c_int) {
    let mut msg: Vec<u8> = Vec::with_capacity(MSGBUFFSIZE);
    msg.extend_from_slice(p);
    if mode & 1 != 0 {
        msg.extend_from_slice(b": ");
        let e = crate::strerr::known(errno)
            .map(|c| c.to_bytes().to_vec())
            .unwrap_or_else(|| crate::strerr::unknown_message(errno).into_bytes());
        msg.extend_from_slice(&e);
    }
    msg.truncate(MSGBUFFSIZE - 1);
    msg.push(0);
    let msg_ptr = msg.as_ptr() as *const ::core::ffi::c_char;
    let sink = MFS_LOG_SINK.load(Ordering::Relaxed);
    if !sink.is_null() {
        // stored by mfs_log_set_sink_function from this exact fn-pointer type
        type Sink = unsafe extern "C" fn(*const ::core::ffi::c_char);
        // SAFETY: the pointer is a `Sink` (see above), transmuted back.
        let sink: Sink = unsafe { std::mem::transmute::<*mut ::core::ffi::c_void, Sink>(sink) };
        // SAFETY: sink contract; msg is NUL-terminated.
        unsafe { sink(msg_ptr) };
    }
    if SYSLOG_OPEN.load(Ordering::Relaxed) != 0 {
        // SAFETY: extern; both strings valid C strings.
        unsafe {
            libc::syslog(
                priority_convert(priority),
                b"[%s] %s\0".as_ptr() as *const _,
                pri_to_str(priority),
                msg_ptr,
            )
        };
    }
    if STDERR_ACTIVE.load(Ordering::Relaxed) == 0 {
        return;
    }
    if SYSLOG_OPEN.load(Ordering::Relaxed) == 0
        || FORCE_STDERR.load(Ordering::Relaxed) != 0
        || mode & 2 != 0
    {
        // SAFETY: extern; stderr valid; all strings NUL-terminated.
        unsafe {
            if USE_COLORS.load(Ordering::Relaxed) != 0 {
                fprintf(
                    stderr,
                    b"%s%s%s\n\0".as_ptr() as *const _,
                    pri_to_colorstr(priority),
                    msg_ptr,
                    COLOR_CLEAR.as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                fprintf(stderr, b"%s\n\0".as_ptr() as *const _, msg_ptr);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mfs_log_set_min_level(mut minlevel: ::core::ffi::c_int) {
    MFS_LOG_MIN_LEVEL.store(minlevel, Ordering::Relaxed);
}

#[unsafe(no_mangle)]
pub extern "C" fn mfs_log_set_elevate_to(mut elevateto: ::core::ffi::c_int) {
    MFS_LOG_ELEVATE_TO.store(elevateto, Ordering::Relaxed);
}

#[unsafe(no_mangle)]
pub extern "C" fn mfs_log_set_sink_function(
    mut s: Option<unsafe extern "C" fn(*const ::core::ffi::c_char)>,
) {
    MFS_LOG_SINK.store(
        s.map(|f| f as *mut ::core::ffi::c_void)
            .unwrap_or(std::ptr::null_mut()),
        Ordering::Relaxed,
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn mfs_log_detach_stderr() {
    STDERR_ACTIVE.store(0, Ordering::Relaxed);
}

#[unsafe(no_mangle)]
pub extern "C" fn mfs_log_detach_syslog() {
    if SYSLOG_OPEN.swap(0, Ordering::Relaxed) != 0 {
        // SAFETY: extern; log was open.
        unsafe { libc::closelog() };
    }
}

/// # Safety
/// C ABI; closes syslog and the debug log file.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log_term() {
    mfs_log_detach_syslog();
    // SAFETY: fmt null → close path, no variadic args.
    unsafe { mfs_file_log(std::ptr::null(), 0, std::ptr::null(), 0, std::ptr::null()) };
}

/// # Safety
/// `ident` must be null or a valid C string that outlives the open log
/// (openlog keeps the pointer on some libcs; daemons pass static strings).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log_init(
    mut ident: *const ::core::ffi::c_char,
    mut daemonflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !ident.is_null() {
        // SAFETY: per fn contract.
        unsafe {
            libc::openlog(
                ident,
                LOG_PID | LOG_NDELAY,
                if daemonflag != 0 {
                    LOG_DAEMON
                } else {
                    LOG_USER
                },
            )
        };
        SYSLOG_OPEN.store(1, Ordering::Relaxed);
    }
    FORCE_STDERR.store(if daemonflag != 0 { 0 } else { 1 }, Ordering::Relaxed);
    // SAFETY: extern; fd 2.
    let tty = unsafe { libc::isatty(2) };
    USE_COLORS.store(if tty != 0 { 1 } else { 0 }, Ordering::Relaxed);
    STDERR_ACTIVE.store(1, Ordering::Relaxed);
    0
}
