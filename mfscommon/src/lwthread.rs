//! Lightweight pthread wrappers, migrated (P1).
//!
//! PORT NOTE (deliberate simplification): the original cached a malloc'd
//! pthread_attr_t in a static. Thread creation happens at daemon startup /
//! worker spawn — never hot — so we build the attr on the stack per call and
//! drop the static, the malloc, and the detached-state tracking. Observable
//! behavior (signal masking, stack size floor, detach state) is identical.
//! The abort-on-error paths kept their mfs_log + abort semantics, minus the
//! macro-expanded errno taxonomy (all branches ended in abort()).

pub type size_t = usize;
pub type pthread_t = ::core::ffi::c_ulong;
pub type uint8_t = u8;

const MFSLOG_ERR: ::core::ffi::c_int = 4;
const MFSLOG_SYSLOG: ::core::ffi::c_int = 0;

unsafe extern "C" {
    fn mfs_log(mode: ::core::ffi::c_int, priority: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
    // own decl: libc's takes a *safe* fn pointer; C callers hand us unsafe fns
    fn pthread_create(
        newthread: *mut pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

#[cold]
fn die(what: &std::ffi::CStr) {
    // SAFETY: fmt is a literal with one %s; `what` is a valid CStr.
    unsafe {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"lwthread: %s failed\0".as_ptr() as *const ::core::ffi::c_char,
            what.as_ptr(),
        );
        libc::abort();
    }
}

/// pthread_create with the daemon's signal set blocked in the new thread.
///
/// # Safety
/// `th`, `attr`, `fn`, `arg` per pthread_create contract (C callers).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lwt_thread_create(
    mut th: *mut pthread_t,
    mut attr: *const libc::pthread_attr_t,
    mut r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
    mut arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut newset: libc::sigset_t = unsafe { std::mem::zeroed() };
    let mut oldset: libc::sigset_t = unsafe { std::mem::zeroed() };
    // SAFETY: newset/oldset are valid, initialized sigsets.
    unsafe {
        libc::sigemptyset(&mut newset);
        for sig in [
            libc::SIGTERM, libc::SIGINT, libc::SIGHUP, libc::SIGQUIT, libc::SIGPIPE,
            libc::SIGTSTP, libc::SIGTTIN, libc::SIGTTOU, libc::SIGUSR1, libc::SIGUSR2,
            libc::SIGALRM, libc::SIGVTALRM, libc::SIGPROF,
        ] {
            libc::sigaddset(&mut newset, sig);
        }
        libc::pthread_sigmask(libc::SIG_BLOCK, &newset, &mut oldset);
    }
    // SAFETY: per pthread_create contract.
    let res = unsafe { pthread_create(th, attr, r#fn, arg) };
    // SAFETY: oldset was filled by the mask call above.
    unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, &oldset, std::ptr::null_mut()) };
    res
}

/// pthread_create with a minimal stack (>= 128 KiB or PTHREAD_STACK_MIN).
///
/// # Safety
/// see lwt_thread_create.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lwt_minthread_create(
    mut th: *mut pthread_t,
    mut detached: uint8_t,
    mut r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
    mut arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut attr: libc::pthread_attr_t = unsafe { std::mem::zeroed() };
    // SAFETY: attr valid; on failure we abort, matching the original.
    unsafe {
        if libc::pthread_attr_init(&mut attr) != 0 {
            die(c"pthread_attr_init");
        }
    }
    // glibc _SC_THREAD_STACK_MIN = 75 (literal kept from the original)
    let mut stacksize = unsafe { libc::sysconf(75) } as usize;
    if stacksize < 0x20000 {
        stacksize = 0x20000;
    }
    // SAFETY: attr initialized above.
    unsafe {
        if libc::pthread_attr_setstacksize(&mut attr, stacksize) != 0 {
            die(c"pthread_attr_setstacksize");
        }
        let state = if detached != 0 {
            libc::PTHREAD_CREATE_DETACHED
        } else {
            libc::PTHREAD_CREATE_JOINABLE
        };
        if libc::pthread_attr_setdetachstate(&mut attr, state) != 0 {
            die(c"pthread_attr_setdetachstate");
        }
    }
    // SAFETY: forwarded contract.
    unsafe { lwt_thread_create(th, &attr, r#fn, arg) }
}
