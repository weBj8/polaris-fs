//! Daemon runtime — port of mfscommon/main.c, shared by plfsmaster,
//! plfschunkserver, plfsmetalogger and plfsgui (Linux configuration).
//!
//! Each daemon's `main` builds a [`Spec`] (the C `init.h` + per-daemon
//! `-D` flags) and calls [`run`]. Everything else is common:
//! - the callback registries (`main_*_register_fname`) and the time API
//!   are exported with their main.h signatures for the transpiled daemon
//!   modules;
//! - command line, config, daemonization, lockfile, privilege drop,
//!   limits and the poll main loop follow main.c step by step, with the
//!   same messages, exit statuses and ordering.
//!
//! State: C keeps everything in file statics touched only by the main
//! thread, except `now`/`start_time` (mutex-protected under USE_PTHREADS).
//! Here the registries live in a `Mutex<Registry>`; every loop iterates
//! over a snapshot so callbacks may register new entries (as they do,
//! e.g. child handlers) without holding the lock. `now`/`start_time`/
//! `usecnow` are atomics. The loop-local timer bookkeeping (`nextevent`)
//! is written back by entry id so handles returned to callers stay valid.
//!
//! Boundary: the process-level operations (fork, dup2 of std fds,
//! sigaction, poll over caller-filled `pollfd` arrays, setuid/setgid,
//! fcntl locks, callbacks into the daemon modules) are libc calls in the
//! annotated fns below; argument parsing, config handling and all
//! decision logic are safe code.

use core::ffi::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicPtr, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};

use crate::cfg;
use crate::clocks::{monotonic_method, monotonic_seconds, monotonic_speed};
use crate::getopt::{Getopt, Next};
use crate::mfslog::{
    MFSLOG_ERR, MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_INFO, MFSLOG_NOTICE, MFSLOG_SYSLOG,
    MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, assert_abort, log_bytes_errno, mfs_log_detach_stderr,
    mfs_log_init, mfs_log_set_elevate_to, mfs_log_set_min_level, mfs_log_term, str_to_pri,
    zassert_abort,
};

const MAIN_C: &str = "../mfscommon/main.c";
const VERSSTR: &str = "4.59.2-1";
const BUILDNO: &str = "2106";
const ETC_PATH: &str = "/usr/local/etc";
const DATA_PATH: &str = "/usr/local/var/mfs";
const DEFAULT_USER: &str = "nobody";
const DEFAULT_GROUP: &str = "";

/// A daemon init function (C `runfn`).
pub type RunFn = unsafe extern "C" fn() -> c_int;

/// Per-daemon configuration: C `init.h` plus the Makefile `-D` flags.
pub struct Spec {
    /// C `APPNAME` (e.g. "mfsmaster").
    pub appname: &'static str,
    /// C `MFSMAXFILES` (4096 unless the Makefile overrides it).
    pub maxfiles: u32,
    /// Built with `_USE_PTHREADS` (enables the glibc arena tuning).
    pub pthreads: bool,
    /// Built with `USE_IONICE`.
    pub ionice: bool,
    /// `SILENT_SIGCHLD` (no "child finished" log line).
    pub silent_sigchld: bool,
    pub run_tab: &'static [(RunFn, &'static str)],
    pub late_run_tab: &'static [(RunFn, &'static str)],
    pub restore_run_tab: &'static [(RunFn, &'static str)],
    /// `MODULE_OPTIONS_GETOPT` letters with their `MODULE_OPTIONS_SWITCH`
    /// actions (all MooseFS module options are argument-less).
    pub module_options: &'static [(u8, unsafe extern "C" fn())],
    pub module_synopsis: &'static str,
    pub module_desc: &'static str,
}

// ---------------------------------------------------------------------------
// Registries (main.h interface)
// ---------------------------------------------------------------------------

type VoidFn = unsafe extern "C" fn();
type IntFn = unsafe extern "C" fn() -> c_int;
type InfoFn = unsafe extern "C" fn(*mut libc::FILE);
type DescFn = unsafe extern "C" fn(*mut libc::pollfd, *mut u32);
type ServeFn = unsafe extern "C" fn(*mut libc::pollfd);
type ChldFn = unsafe extern "C" fn(libc::pid_t, c_int);

#[derive(Clone)]
struct Named<F> {
    fun: F,
    name: String,
}

#[derive(Clone)]
struct Poll {
    desc: DescFn,
    serve: ServeFn,
    dname: String,
    sname: String,
}

#[derive(Clone)]
struct Chld {
    pid: libc::pid_t,
    fun: ChldFn,
    name: String,
}

struct Timer {
    nextevent: u64,
    useconds: u64,
    usecoffset: u64,
    fun: VoidFn,
    name: String,
}

/// Lists are kept in C order: C prepends, so the newest entry runs first.
struct Registry {
    destruct: Vec<Named<VoidFn>>,
    mayexit: Vec<Named<IntFn>>,
    wantexit: Vec<Named<VoidFn>>,
    canexit: Vec<Named<IntFn>>,
    reload: Vec<Named<VoidFn>>,
    info: Vec<Named<InfoFn>>,
    keepalive: Vec<Named<VoidFn>>,
    poll: Vec<Poll>,
    eachloop: Vec<Named<VoidFn>>,
    chld: Vec<Chld>,
    /// Boxed so the address handed out as the timer handle is stable.
    timers: Vec<Box<Timer>>,
}

static REG: Mutex<Registry> = Mutex::new(Registry {
    destruct: Vec::new(),
    mayexit: Vec::new(),
    wantexit: Vec::new(),
    canexit: Vec::new(),
    reload: Vec::new(),
    info: Vec::new(),
    keepalive: Vec::new(),
    poll: Vec::new(),
    eachloop: Vec::new(),
    chld: Vec::new(),
    timers: Vec::new(),
});

fn reg() -> MutexGuard<'static, Registry> {
    REG.lock().unwrap_or_else(|e| e.into_inner())
}

static NOW: AtomicU32 = AtomicU32::new(0);
static START_TIME: AtomicU32 = AtomicU32::new(0);
static USECNOW: AtomicU64 = AtomicU64::new(0);
/// C `lcall_trigger` (seconds, f64 bits) and `loop_usleep` (µs).
static LCALL_TRIGGER: AtomicU64 = AtomicU64::new(0);
static LOOP_USLEEP: AtomicU64 = AtomicU64::new(0);
/// C `loop_start` (monotonic seconds, f64 bits).
static LOOP_START: AtomicU64 = AtomicU64::new(0);
static SIGNAL_PIPE: [AtomicI32; 2] = [AtomicI32::new(-1), AtomicI32::new(-1)];
static RELOAD_FIRST_DONE: AtomicBool = AtomicBool::new(false);
static LFD: AtomicI32 = AtomicI32::new(-1);

fn lcall_trigger() -> f64 {
    f64::from_bits(LCALL_TRIGGER.load(Ordering::Relaxed))
}

fn log(mode: c_int, pri: c_int, text: &str) {
    log_bytes_errno(mode, pri, text.as_bytes(), 0);
}

/// `mode & 1` variants append `strerr(errno)` of the current errno.
fn log_errno(mode: c_int, pri: c_int, text: &str) {
    crate::mfslog::log_bytes(mode, pri, text.as_bytes());
}

fn errno() -> c_int {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

/// C `LOOP_START` / `LOOP_END(name)` around one callback.
fn timed<R>(name: &str, f: impl FnOnce() -> R) -> R {
    let trig = lcall_trigger();
    if trig <= 0.0 {
        return f();
    }
    let start = monotonic_seconds();
    let r = f();
    let end = monotonic_seconds();
    if end - start > trig {
        let ms = crate::cnum::fmt_fixed((end - start) * 1000.0, 2);
        log(MFSLOG_SYSLOG, MFSLOG_WARNING, &format!("long call detected: {name} : {ms}ms"));
    }
    r
}

/// # Safety
// SAFETY: caller guarantees `p` is a valid NUL-terminated C string.
unsafe fn cname(p: *const c_char) -> String {
    // SAFETY: per fn contract.
    String::from_utf8_lossy(unsafe { CStr::from_ptr(p) }.to_bytes()).into_owned()
}

macro_rules! register_fn {
    ($(#[$m:meta])* $cname:ident, $field:ident, $ty:ty) => {
        $(#[$m])*
        /// # Safety
        /// `fname` must be a valid C string; `fun` must be callable for the
        /// process lifetime.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $cname(fun: Option<$ty>, fname: *const c_char) {
            let Some(fun) = fun else { return };
            // SAFETY: per fn contract.
            let name = unsafe { cname(fname) };
            reg().$field.insert(0, Named { fun, name });
        }
    };
}

register_fn!(main_destruct_register_fname, destruct, VoidFn);
register_fn!(main_mayexit_register_fname, mayexit, IntFn);
register_fn!(main_wantexit_register_fname, wantexit, VoidFn);
register_fn!(main_canexit_register_fname, canexit, IntFn);
register_fn!(main_reload_register_fname, reload, VoidFn);
register_fn!(main_info_register_fname, info, InfoFn);
register_fn!(main_keepalive_register_fname, keepalive, VoidFn);
register_fn!(main_eachloop_register_fname, eachloop, VoidFn);

/// # Safety
/// Names must be valid C strings; callbacks callable for the process
/// lifetime.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_poll_register_fname(
    desc: Option<DescFn>,
    serve: Option<ServeFn>,
    dname: *const c_char,
    sname: *const c_char,
) {
    let (Some(desc), Some(serve)) = (desc, serve) else { return };
    // SAFETY: per fn contract.
    let (dname, sname) = unsafe { (cname(dname), cname(sname)) };
    reg().poll.insert(0, Poll { desc, serve, dname, sname });
}

/// # Safety
/// `fname` must be a valid C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_chld_register_fname(
    pid: libc::pid_t,
    fun: Option<ChldFn>,
    fname: *const c_char,
) {
    let Some(fun) = fun else { return };
    // SAFETY: per fn contract.
    let name = unsafe { cname(fname) };
    reg().chld.insert(0, Chld { pid, fun, name });
}

/// First event time >= usecnow on the (period, offset) grid.
fn first_event(usecnow: u64, useconds: u64, usecoffset: u64) -> u64 {
    let mut n = (usecnow / useconds) * useconds + usecoffset;
    while n < usecnow {
        n += useconds;
    }
    n
}

/// # Safety
/// `fname` must be a valid C string. Returns an opaque timer handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_msectime_register_fname(
    mseconds: u32,
    offset: u32,
    fun: Option<VoidFn>,
    fname: *const c_char,
) -> *mut c_void {
    let useconds = 1000 * mseconds as u64;
    let usecoffset = 1000 * offset as u64;
    let Some(fun) = fun else { return core::ptr::null_mut() };
    if useconds == 0 || usecoffset >= useconds {
        return core::ptr::null_mut();
    }
    // SAFETY: per fn contract.
    let name = unsafe { cname(fname) };
    let t = Box::new(Timer {
        nextevent: first_event(USECNOW.load(Ordering::Relaxed), useconds, usecoffset),
        useconds,
        usecoffset,
        fun,
        name,
    });
    let handle = &*t as *const Timer as *mut c_void;
    reg().timers.insert(0, t);
    handle
}

/// # Safety
/// `x` must be a handle returned by `main_*time_register_fname`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_msectime_change(x: *mut c_void, mseconds: u32, offset: u32) -> c_int {
    let useconds = 1000 * mseconds as u64;
    let usecoffset = 1000 * offset as u64;
    if useconds == 0 || usecoffset >= useconds {
        return -1;
    }
    let mut r = reg();
    if let Some(t) = r.timers.iter_mut().find(|t| &***t as *const Timer as *mut c_void == x) {
        t.nextevent = first_event(USECNOW.load(Ordering::Relaxed), useconds, usecoffset);
        t.useconds = useconds;
        t.usecoffset = usecoffset;
    }
    0
}

/// # Safety
/// As `main_msectime_register_fname`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_time_register_fname(
    seconds: u32,
    offset: u32,
    fun: Option<VoidFn>,
    fname: *const c_char,
) -> *mut c_void {
    // SAFETY: forwarded contract; C multiplies in uint32_t.
    unsafe {
        main_msectime_register_fname(seconds.wrapping_mul(1000), offset.wrapping_mul(1000), fun, fname)
    }
}

/// # Safety
/// As `main_msectime_change`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_time_change(x: *mut c_void, seconds: u32, offset: u32) -> c_int {
    // SAFETY: forwarded contract.
    unsafe { main_msectime_change(x, seconds.wrapping_mul(1000), offset.wrapping_mul(1000)) }
}

fn timeofday() -> (u32, u64) {
    let d = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    (d.as_secs() as u32, d.as_secs() * 1_000_000 + d.subsec_micros() as u64)
}

fn time_now() -> u32 {
    timeofday().0
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn main_time_refresh() -> u32 {
    let n = time_now();
    NOW.store(n, Ordering::Relaxed);
    n
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn main_time() -> u32 {
    NOW.load(Ordering::Relaxed)
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn main_utime() -> u64 {
    timeofday().1
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn main_start_time() -> u32 {
    START_TIME.load(Ordering::Relaxed)
}

fn refresh_now() {
    let (s, us) = timeofday();
    USECNOW.store(us, Ordering::Relaxed);
    NOW.store(s, Ordering::Relaxed);
}

fn check_long_loop(loop_end: f64) {
    let start = f64::from_bits(LOOP_START.load(Ordering::Relaxed));
    if loop_end - start > 5.0 {
        let s = crate::cnum::fmt_fixed(loop_end - start, 3);
        log(MFSLOG_SYSLOG, MFSLOG_WARNING, &format!("long loop detected ({s}s)"));
    }
}

/// C: `main_keep_alive` — called by long-running operations.
#[unsafe(no_mangle)]
pub extern "C" fn main_keep_alive() {
    let loop_end = monotonic_seconds();
    check_long_loop(loop_end);
    LOOP_START.store(loop_end.to_bits(), Ordering::Relaxed);
    refresh_now();
    let list = reg().keepalive.clone();
    for e in list {
        // SAFETY: registered daemon callback.
        timed(&e.name, || unsafe { (e.fun)() });
    }
}

fn signal_write(b: u8) {
    let fd = SIGNAL_PIPE[1].load(Ordering::Relaxed);
    // SAFETY: async-signal-safe write of one byte from a static buffer.
    unsafe { libc::write(fd, [b].as_ptr() as *const c_void, 1) };
}

/// C: `main_exit` — request termination (status 1).
#[unsafe(no_mangle)]
pub extern "C" fn main_exit() {
    signal_write(6);
}

extern "C" fn termhandle(_: c_int) {
    signal_write(1);
}
extern "C" fn reloadhandle(_: c_int) {
    signal_write(2);
}
extern "C" fn chldhandle(_: c_int) {
    signal_write(3);
}
extern "C" fn infohandle(_: c_int) {
    signal_write(4);
}
extern "C" fn alarmhandle(_: c_int) {
    signal_write(5);
}

// ---------------------------------------------------------------------------
// Main loop
// ---------------------------------------------------------------------------

/// C: `main_reload` — log levels and debug options from the config.
fn main_reload() {
    let (t, e) = cfg::get_double(b"LONG_CALL_TRIGGER_MS", 0.0);
    cfg::set_erange(e);
    LCALL_TRIGGER.store((t / 1000.0).to_bits(), Ordering::Relaxed);
    let (s, e) = cfg::get_uint32(b"DEBUG_LOOP_SLEEP_MS", 0);
    cfg::set_erange(e);
    LOOP_USLEEP.store(s.wrapping_mul(1000) as u64, Ordering::Relaxed);
    let minlevel = cfg::get_str(b"SYSLOG_MIN_LEVEL", b"INFO");
    let elevate = cfg::get_str(b"SYSLOG_ELEVATE_TO", b"NOTICE");
    let first = !RELOAD_FIRST_DONE.load(Ordering::Relaxed);
    let tmp = str_to_pri(&minlevel);
    if tmp >= 0 {
        mfs_log_set_min_level(tmp);
    } else if first {
        log(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, "error parsing SYSLOG_MIN_LEVEL option - using INFO");
        mfs_log_set_min_level(MFSLOG_INFO);
    } else {
        log(MFSLOG_SYSLOG, MFSLOG_WARNING, "error parsing SYSLOG_MIN_LEVEL option - left unchanged");
    }
    let tmp = str_to_pri(&elevate);
    if tmp >= 0 {
        mfs_log_set_elevate_to(tmp);
    } else if first {
        log(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, "error parsing SYSLOG_ELEVATE_TO option - using NOTICE");
        mfs_log_set_elevate_to(MFSLOG_NOTICE);
    } else {
        log(MFSLOG_SYSLOG, MFSLOG_WARNING, "error parsing SYSLOG_ELEVATE_TO option - left unchanged");
    }
    RELOAD_FIRST_DONE.store(true, Ordering::Relaxed);
}

/// Snapshot of timer schedules: (handle, nextevent, useconds, usecoffset,
/// fun, name).
type TimerSnap = (usize, u64, u64, u64, VoidFn, String);

fn timer_snapshot() -> Vec<TimerSnap> {
    reg()
        .timers
        .iter()
        .map(|t| (&**t as *const Timer as usize, t.nextevent, t.useconds, t.usecoffset, t.fun, t.name.clone()))
        .collect()
}

/// Live (nextevent, useconds, usecoffset) of a timer.
fn timer_load(h: usize) -> Option<(u64, u64, u64)> {
    reg()
        .timers
        .iter()
        .find(|t| &***t as *const Timer as usize == h)
        .map(|t| (t.nextevent, t.useconds, t.usecoffset))
}

fn timer_store(h: usize, nextevent: u64) {
    if let Some(t) = reg().timers.iter_mut().find(|t| &***t as *const Timer as usize == h) {
        t.nextevent = nextevent;
    }
}

/// C time-jump correction of `nextevent` (safe core, unit-tested).
fn rebase_backward(nextevent: u64, useconds: u64, usecoffset: u64, usecnow: u64, prevtime: u64) -> u64 {
    let mut prev_to_run = nextevent.wrapping_sub(prevtime);
    if prev_to_run > useconds {
        prev_to_run = useconds;
    }
    let mut n = (usecnow / useconds) * useconds + usecoffset;
    while n <= usecnow + prev_to_run {
        n += useconds;
    }
    n
}

fn rebase_forward(useconds: u64, usecoffset: u64, usecnow: u64) -> u64 {
    let mut n = (usecnow / useconds) * useconds + usecoffset;
    while usecnow >= n {
        n += useconds;
    }
    n
}

/// C: `mainloop` — returns the exit status (1 after `main_exit`).
fn mainloop(spec: &Spec) -> c_int {
    let mut prevtime: u64 = 0;
    let (mut t, mut r) = (0, 0);
    let mut status = 0;
    let maxfiles = spec.maxfiles as usize;
    let mut pdesc = vec![libc::pollfd { fd: -1, events: 0, revents: 0 }; maxfiles];
    LOOP_START.store(monotonic_seconds().to_bits(), Ordering::Relaxed);
    while t != 3 {
        let us = LOOP_USLEEP.load(Ordering::Relaxed);
        if us != 0 {
            std::thread::sleep(std::time::Duration::from_micros(us));
        }
        let mut ndesc: u32 = 1;
        pdesc[0] = libc::pollfd { fd: SIGNAL_PIPE[0].load(Ordering::Relaxed), events: libc::POLLIN, revents: 0 };
        let polls = reg().poll.clone();
        for p in &polls {
            // SAFETY: C contract: desc appends entries below MFSMAXFILES.
            timed(&p.dname, || unsafe { (p.desc)(pdesc.as_mut_ptr(), &mut ndesc) });
        }
        check_long_loop(monotonic_seconds());
        // SAFETY: pdesc holds ndesc initialized entries.
        let i = unsafe { libc::poll(pdesc.as_mut_ptr(), ndesc as libc::nfds_t, 10) };
        let err = errno();
        LOOP_START.store(monotonic_seconds().to_bits(), Ordering::Relaxed);
        refresh_now();
        let usecnow = USECNOW.load(Ordering::Relaxed);
        if i < 0 {
            if err == libc::EAGAIN {
                log(MFSLOG_SYSLOG, MFSLOG_WARNING, "poll returned EAGAIN");
                std::thread::sleep(std::time::Duration::from_micros(10000));
                continue;
            }
            if err != libc::EINTR {
                let es = String::from_utf8_lossy(&crate::strerr::message(err)).into_owned();
                log(MFSLOG_SYSLOG, MFSLOG_WARNING, &format!("poll error: {es}"));
                break;
            }
        } else if i > 0 {
            if pdesc[0].revents & libc::POLLIN != 0 {
                let mut sigid = 0u8;
                // SAFETY: one-byte read into a local.
                let n = unsafe { libc::read(pdesc[0].fd, &mut sigid as *mut u8 as *mut c_void, 1) };
                if n == 1 {
                    match sigid {
                        1 if t == 0 => {
                            log(MFSLOG_SYSLOG, MFSLOG_NOTICE, "terminate signal received");
                            t = 1;
                        }
                        2 => {
                            log(MFSLOG_SYSLOG, MFSLOG_INFO, "reloading config files");
                            r = 1;
                        }
                        3 => {
                            if !spec.silent_sigchld {
                                log(MFSLOG_SYSLOG, MFSLOG_INFO, "child finished");
                            }
                            r = 2;
                        }
                        4 => {
                            log(MFSLOG_SYSLOG, MFSLOG_INFO, "log extra info");
                            r = 3;
                        }
                        5 => log(
                            MFSLOG_SYSLOG,
                            MFSLOG_NOTICE,
                            "unexpected alarm/prof signal received - ignoring",
                        ),
                        6 => {
                            log(MFSLOG_SYSLOG, MFSLOG_NOTICE, "internal terminate request");
                            t = 1;
                            status = 1;
                        }
                        _ => {}
                    }
                }
            }
            for p in &polls {
                // SAFETY: serve reads the pollfd array filled above.
                timed(&p.sname, || unsafe { (p.serve)(pdesc.as_mut_ptr()) });
            }
        }
        let eachloop = reg().eachloop.clone();
        for e in &eachloop {
            // SAFETY: registered daemon callback.
            timed(&e.name, || unsafe { (e.fun)() });
        }
        if usecnow < prevtime {
            for (h, ne, us, off, _, _) in timer_snapshot() {
                timer_store(h, rebase_backward(ne, us, off, usecnow, prevtime));
            }
        } else if usecnow > prevtime + 5_000_000 {
            for (h, _, us, off, _, _) in timer_snapshot() {
                timer_store(h, rebase_forward(us, off, usecnow));
            }
        }
        for (h, _, _, _, fun, name) in timer_snapshot() {
            // C reads timeit->nextevent/useconds live: a callback may
            // reschedule its own timer through main_*time_change.
            let Some((ne, _, _)) = timer_load(h) else { continue };
            if usecnow >= ne {
                let mut count = 0u32;
                while count < 10 {
                    match timer_load(h) {
                        Some((ne, _, _)) if usecnow >= ne => {}
                        _ => break,
                    }
                    // SAFETY: registered daemon callback.
                    timed(&name, || unsafe { fun() });
                    if let Some((ne, us, _)) = timer_load(h) {
                        timer_store(h, ne.wrapping_add(us));
                    }
                    count += 1;
                }
                if let Some((ne, us, off)) = timer_load(h) {
                    if usecnow >= ne {
                        timer_store(h, rebase_forward(us, off, usecnow));
                    }
                }
            }
        }
        prevtime = usecnow;
        if r == 1 {
            cfg::reload();
            main_reload();
            let list = reg().reload.clone();
            for e in &list {
                // SAFETY: registered daemon callback.
                timed(&e.name, || unsafe { (e.fun)() });
            }
            r = 0;
        } else if r == 2 {
            reap_children();
            r = 0;
        } else if r == 3 {
            write_info_file(spec);
            r = 0;
        }
        if t == 1 {
            let mut ok = true;
            let list = reg().mayexit.clone();
            for e in list {
                // SAFETY: registered daemon callback.
                if timed(&e.name, || unsafe { (e.fun)() }) == 0 {
                    ok = false;
                    break;
                }
            }
            if ok {
                let list = reg().wantexit.clone();
            for e in list {
                    // SAFETY: registered daemon callback.
                    timed(&e.name, || unsafe { (e.fun)() });
                }
                t = 2;
            }
        }
        if t == 2 {
            let mut ok = true;
            let list = reg().canexit.clone();
            for e in list {
                // SAFETY: registered daemon callback.
                if timed(&e.name, || unsafe { (e.fun)() }) == 0 {
                    ok = false;
                    break;
                }
            }
            if ok {
                t = 3;
            }
        }
    }
    status
}

fn reap_children() {
    loop {
        let mut st: c_int = 0;
        // SAFETY: non-blocking wait for any child; st is a local.
        let pid = unsafe { libc::waitpid(-1, &mut st, libc::WNOHANG) };
        if pid <= 0 {
            break;
        }
        // every entry for this pid runs once and is removed
        let matched: Vec<Chld> = {
            let mut r = reg();
            let (m, keep): (Vec<Chld>, Vec<Chld>) = r.chld.drain(..).partition(|c| c.pid == pid);
            r.chld = keep;
            m
        };
        for c in matched {
            // SAFETY: registered daemon callback.
            timed(&c.name, || unsafe { (c.fun)(pid, st) });
        }
    }
}

fn write_info_file(spec: &Spec) {
    let fname = CString::new(format!(".{}_info.txt", spec.appname)).expect("appname has no NUL");
    // SAFETY: fopen/fwrite/fclose on a FILE* we own; cfg_info and the
    // info callbacks write to it before it is closed below.
    unsafe {
        let f = libc::fopen(fname.as_ptr(), c"w".as_ptr());
        if f.is_null() {
            log(MFSLOG_SYSLOG, MFSLOG_WARNING, "can't create info file");
            return;
        }
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs() as i64);
        let head = format!(
            "[general]\nversion: {VERSSTR}\nbuild: {BUILDNO}\ntimestamp: {secs}\n\n"
        );
        libc::fwrite(head.as_ptr() as *const c_void, 1, head.len(), f);
        cfg::cfg_info(f);
        let list = reg().info.clone();
            for e in list {
            timed(&e.name, || (e.fun)(f));
        }
        libc::fclose(f);
    }
}

fn run_tab(tab: &[(RunFn, &str)], what: &str) -> bool {
    for &(f, name) in tab {
        NOW.store(time_now(), Ordering::Relaxed);
        // SAFETY: daemon init function from the Spec.
        if unsafe { f() } < 0 {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_ERR, &format!("{what}: {name} failed !!!"));
            return false;
        }
    }
    true
}

// ---------------------------------------------------------------------------
// Signals, daemonization, privileges, lockfile
// ---------------------------------------------------------------------------

fn set_signal_handlers(daemon: bool) {
    let mut fds = [0 as c_int; 2];
    // SAFETY: pipe fills the two-element array.
    let ret = unsafe { libc::pipe(fds.as_mut_ptr()) };
    if ret != 0 {
        zassert_abort(MAIN_C, 987, "pipe(signalpipe)", ret, errno());
    }
    SIGNAL_PIPE[0].store(fds[0], Ordering::Relaxed);
    SIGNAL_PIPE[1].store(fds[1], Ordering::Relaxed);
    let set = |sigs: &[c_int], handler: libc::sighandler_t| {
        for &s in sigs {
            // SAFETY: sigaction from a zeroed struct; handlers only write(2).
            unsafe {
                let mut sa: libc::sigaction = std::mem::zeroed();
                sa.sa_flags = libc::SA_RESTART;
                libc::sigemptyset(&mut sa.sa_mask);
                sa.sa_sigaction = handler;
                libc::sigaction(s, &sa, core::ptr::null_mut());
            }
        }
    };
    let h = |f: extern "C" fn(c_int)| f as libc::sighandler_t;
    set(&[libc::SIGTERM], h(termhandle));
    set(&[libc::SIGHUP], h(reloadhandle));
    set(&[libc::SIGUSR1], h(infohandle));
    set(&[libc::SIGALRM, libc::SIGVTALRM, libc::SIGPROF], h(alarmhandle));
    // Linux: SIGCLD == SIGCHLD, so C installs the handler twice on one signal
    set(&[libc::SIGCHLD, libc::SIGCHLD], h(chldhandle));
    set(
        &[libc::SIGQUIT, libc::SIGPIPE, libc::SIGTSTP, libc::SIGTTIN, libc::SIGTTOU, libc::SIGUSR2],
        libc::SIG_IGN,
    );
    set(&[libc::SIGINT], if daemon { libc::SIG_IGN } else { h(termhandle) });
}

fn signal_cleanup() {
    for fd in &SIGNAL_PIPE {
        // SAFETY: closing our own pipe ends (EBADF if never opened, as C).
        unsafe { libc::close(fd.load(Ordering::Relaxed)) };
    }
}

fn exit(code: c_int) -> ! {
    // C exit(): flush stdio (Rust stdout/stderr are unbuffered/line-flushed)
    std::process::exit(code)
}

fn write_stderr(b: &[u8]) {
    // SAFETY: write to fd 2 from a valid buffer.
    unsafe { libc::write(2, b.as_ptr() as *const c_void, b.len()) };
}

/// `sassert(dup(fd)==expected)` of main.c.
fn dup_expect(fd: c_int, expected: c_int, line: u32, expr: &str) {
    // SAFETY: dup of a descriptor we own.
    if unsafe { libc::dup(fd) } != expected {
        assert_abort(MAIN_C, line, expr);
    }
}

/// C: `makedaemon` — double fork; the first parent relays the child's
/// startup messages from a pipe and exits with the child's verdict.
fn makedaemon() {
    let mut piped = [0 as c_int; 2];
    // SAFETY: plain process-level libc calls at single-threaded bootstrap.
    unsafe {
        libc::fflush(core::ptr::null_mut());
        if libc::pipe(piped.as_mut_ptr()) < 0 {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_ERR, "pipe error");
            exit(1);
        }
        let f = libc::fork();
        if f < 0 {
            let es = String::from_utf8_lossy(&crate::strerr::message(errno())).into_owned();
            log(MFSLOG_SYSLOG, MFSLOG_ERR, &format!("first fork error: {es}"));
            exit(1);
        }
        if f > 0 {
            let mut st: c_int = 0;
            libc::wait(&mut st);
            if st != 0 {
                log(MFSLOG_SYSLOG_STDERR, MFSLOG_ERR, &format!("Child status: {st}"));
                exit(1);
            }
            libc::close(piped[1]);
            let mut buf = [0u8; 1000];
            loop {
                let r = libc::read(piped[0], buf.as_mut_ptr() as *mut c_void, 1000);
                if r == 0 {
                    break;
                }
                if r > 0 {
                    let r = r as usize;
                    if buf[r - 1] == 0 {
                        if r > 1 {
                            write_stderr(&buf[..r - 1]);
                        }
                        exit(1);
                    }
                    write_stderr(&buf[..r]);
                } else {
                    let es = String::from_utf8_lossy(&crate::strerr::message(errno())).into_owned();
                    log(MFSLOG_SYSLOG_STDERR, MFSLOG_ERR, &format!("Error reading pipe: {es}"));
                    exit(1);
                }
            }
            exit(0);
        }
        libc::setsid();
        libc::setpgid(0, libc::getpid());
        let f = libc::fork();
        if f < 0 {
            let es = String::from_utf8_lossy(&crate::strerr::message(errno())).into_owned();
            log(MFSLOG_SYSLOG, MFSLOG_ERR, &format!("second fork error: {es}"));
            if libc::write(piped[1], b"fork error\n".as_ptr() as *const c_void, 11) != 11 {
                let es = String::from_utf8_lossy(&crate::strerr::message(errno())).into_owned();
                log(MFSLOG_SYSLOG, MFSLOG_ERR, &format!("pipe write error: {es}"));
            }
            libc::close(piped[1]);
            exit(1);
        }
        if f > 0 {
            exit(0);
        }
        set_signal_handlers(true);
        let f = libc::open(c"/dev/null".as_ptr(), libc::O_RDWR, 0);
        libc::close(0);
        dup_expect(f, 0, 1347, "dup(f)==STDIN_FILENO");
        libc::close(1);
        dup_expect(f, 1, 1349, "dup(f)==STDOUT_FILENO");
        libc::close(2);
        dup_expect(piped[1], 2, 1351, "dup(piped[1])==STDERR_FILENO");
        libc::close(piped[1]);
        libc::close(f);
    }
    mfs_log_detach_stderr();
}

/// C: `close_msg_channel` — stderr to /dev/null (ends the relay).
fn close_msg_channel() {
    // SAFETY: fd juggling on our own std descriptors.
    unsafe {
        libc::fflush(core::ptr::null_mut());
        let f = libc::open(c"/dev/null".as_ptr(), libc::O_RDWR, 0);
        libc::close(2);
        dup_expect(f, 2, 1363, "dup(f)==STDERR_FILENO");
        libc::close(f);
    }
}

/// C `fputc(0,stderr)` — tells the relaying parent that startup failed.
fn signal_startup_failure() {
    write_stderr(&[0]);
}

enum Ident {
    Uid(libc::uid_t, libc::gid_t),
    Missing,
}

fn getpw(name: Option<&CStr>, uid: libc::uid_t) -> Ident {
    let mut buf = vec![0 as c_char; 16384];
    // SAFETY: getpw*_r write into pwd/buf we own; result checked.
    unsafe {
        let mut pwd: libc::passwd = std::mem::zeroed();
        let mut res: *mut libc::passwd = core::ptr::null_mut();
        let rc = match name {
            Some(n) => libc::getpwnam_r(n.as_ptr(), &mut pwd, buf.as_mut_ptr(), buf.len(), &mut res),
            None => libc::getpwuid_r(uid, &mut pwd, buf.as_mut_ptr(), buf.len(), &mut res),
        };
        // C ignores getpwuid_r's return value and only tests the result
        if (name.is_some() && rc != 0) || res.is_null() {
            Ident::Missing
        } else {
            Ident::Uid((*res).pw_uid, (*res).pw_gid)
        }
    }
}

/// C: `changeugid` — drop root to WORKING_USER / WORKING_GROUP.
fn changeugid() {
    // SAFETY: geteuid has no preconditions.
    if unsafe { libc::geteuid() } != 0 {
        return;
    }
    let wuser = cfg::get_str(b"WORKING_USER", DEFAULT_USER.as_bytes());
    let wgroup = cfg::get_str(b"WORKING_GROUP", DEFAULT_GROUP.as_bytes());
    let text = |b: &[u8]| String::from_utf8_lossy(b).into_owned();
    let mut gidok = false;
    let mut wrk_gid: libc::gid_t = libc::gid_t::MAX;
    if wgroup.first() == Some(&b'#') {
        wrk_gid = crate::cnum::strtol10(&wgroup[1..]).0 as libc::gid_t;
        gidok = true;
    } else if !wgroup.is_empty() {
        let cg = CString::new(wgroup.clone()).expect("config values have no NUL");
        let mut buf = vec![0 as c_char; 16384];
        // SAFETY: getgrnam_r writes into grp/buf we own; result checked.
        let gid = unsafe {
            let mut grp: libc::group = std::mem::zeroed();
            let mut res: *mut libc::group = core::ptr::null_mut();
            let rc = libc::getgrnam_r(cg.as_ptr(), &mut grp, buf.as_mut_ptr(), buf.len(), &mut res);
            if rc != 0 || res.is_null() { None } else { Some((*res).gr_gid) }
        };
        match gid {
            Some(g) => wrk_gid = g,
            None => {
                log(MFSLOG_SYSLOG_STDERR, MFSLOG_ERR, &format!("{}: no such group !!!", text(&wgroup)));
                exit(1);
            }
        }
        gidok = true;
    }
    let wrk_uid: libc::uid_t;
    if wuser.first() == Some(&b'#') {
        wrk_uid = crate::cnum::strtol10(&wuser[1..]).0 as libc::uid_t;
        if !gidok {
            match getpw(None, wrk_uid) {
                Ident::Uid(_, g) => wrk_gid = g,
                Ident::Missing => {
                    log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        &format!("{}: no such user id - can't obtain group id", text(&wuser[1..])),
                    );
                    exit(1);
                }
            }
        }
    } else {
        let cu = CString::new(wuser.clone()).expect("config values have no NUL");
        match getpw(Some(&cu), 0) {
            Ident::Uid(u, g) => {
                wrk_uid = u;
                if !gidok {
                    wrk_gid = g;
                }
            }
            Ident::Missing => {
                log(MFSLOG_SYSLOG_STDERR, MFSLOG_ERR, &format!("{}: no such user !!!", text(&wuser)));
                exit(1);
            }
        }
    }
    // SAFETY: credential syscalls on the current process.
    unsafe {
        if libc::setgid(wrk_gid) < 0 {
            log_errno(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_ERR, &format!("can't set gid to {}", wrk_gid as c_int));
            exit(1);
        }
        log(MFSLOG_SYSLOG, MFSLOG_INFO, &format!("set gid to {}", wrk_gid as c_int));
        if libc::setgroups(0, core::ptr::null()) < 0 {
            log_errno(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_ERR, "can't clear auxiliary groups");
            exit(1);
        }
        log(MFSLOG_SYSLOG, MFSLOG_INFO, "cleared auxiliary groups");
        if libc::setuid(wrk_uid) < 0 {
            log_errno(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_ERR, &format!("can't set uid to {}", wrk_uid as c_int));
            exit(1);
        }
        log(MFSLOG_SYSLOG, MFSLOG_INFO, &format!("set uid to {}", wrk_uid as c_int));
    }
}

/// C: `mylock` — Ok(0) locked, Ok(pid) held by pid, Err on fcntl error.
fn mylock(fd: c_int) -> Result<libc::pid_t, ()> {
    loop {
        // SAFETY: fcntl lock ops with a local flock struct.
        unsafe {
            let mut fl: libc::flock = std::mem::zeroed();
            fl.l_start = 0;
            fl.l_len = 0;
            fl.l_pid = libc::getpid();
            fl.l_type = libc::F_WRLCK as _;
            fl.l_whence = libc::SEEK_SET as _;
            if libc::fcntl(fd, libc::F_SETLK, &mut fl) >= 0 {
                return Ok(0);
            }
            if errno() != libc::EAGAIN {
                return Err(());
            }
            if libc::fcntl(fd, libc::F_GETLK, &mut fl) < 0 {
                return Err(());
            }
            if fl.l_type != libc::F_UNLCK as _ {
                return Ok(fl.l_pid);
            }
        }
    }
}

fn wdunlock() {
    let fd = LFD.load(Ordering::Relaxed);
    if fd >= 0 {
        // SAFETY: closing our lockfile descriptor.
        unsafe { libc::close(fd) };
    }
}

fn kill(pid: libc::pid_t, sig: c_int) -> bool {
    // SAFETY: kill has no memory preconditions.
    unsafe { libc::kill(pid, sig) >= 0 }
}

fn eprint_flush(s: &str) {
    write_stderr(s.as_bytes());
}

/// C: `wdlock` — 0 = continue, 1 = error (process exit status).
fn wdlock(spec: &Spec, runmode: RunMode, timeout: u32) -> u8 {
    let app = spec.appname;
    let fname = CString::new(format!(".{app}.lock")).expect("appname has no NUL");
    // SAFETY: open with a valid path.
    let lfd = unsafe { libc::open(fname.as_ptr(), libc::O_WRONLY | libc::O_CREAT, 0o666) };
    LFD.store(lfd, Ordering::Relaxed);
    if lfd < 0 {
        log_errno(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_ERR, "can't create lockfile in working directory");
        return 1;
    }
    let Ok(mut ownerpid) = mylock(lfd) else {
        log_errno(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_ERR, "fcntl error");
        return 1;
    };
    use RunMode::*;
    if ownerpid > 0 {
        match runmode {
            Test => {
                log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, &format!("{app} pid: {ownerpid}"));
                return 0;
            }
            Start => {
                log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    "can't start: lockfile is already locked by another process",
                );
                return 1;
            }
            Reload => {
                if !kill(ownerpid, libc::SIGHUP) {
                    log_errno(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_ERR, "can't send reload signal to lock owner");
                    return 1;
                }
                log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, "reload signal has been sent");
                return 0;
            }
            Info => {
                if !kill(ownerpid, libc::SIGUSR1) {
                    log_errno(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_ERR, "can't send info signal to lock owner");
                    return 1;
                }
                log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, "info signal has been sent");
                return 0;
            }
            _ => {}
        }
        let (sig, sname) = if runmode == Kill { (libc::SIGKILL, "SIGKILL") } else { (libc::SIGTERM, "SIGTERM") };
        log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            &format!("sending {sname} to lock owner (pid:{ownerpid})"),
        );
        if !kill(ownerpid, sig) {
            log_errno(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_ERR, "can't kill lock owner");
            return 1;
        }
        let mut l: u32 = 0;
        eprint_flush("waiting for termination ...");
        loop {
            let Ok(newownerpid) = mylock(lfd) else {
                log_errno(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_ERR, "fcntl error");
                return 1;
            };
            if newownerpid > 0 {
                l += 1;
                if l >= timeout {
                    log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        &format!("about {l} seconds passed and lockfile is still locked - giving up"),
                    );
                    eprint_flush(":giving up\n");
                    return 1;
                }
                if l % 10 == 0 {
                    log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        &format!("about {l} seconds passed and lock still exists"),
                    );
                    eprint_flush(".");
                }
                if newownerpid != ownerpid {
                    eprint_flush("\nnew lock owner detected\n");
                    eprint_flush(&format!(":sending {sname} to lock owner (pid:{newownerpid}):"));
                    if !kill(newownerpid, sig) {
                        log_errno(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_ERR, "can't kill lock owner");
                        return 1;
                    }
                    ownerpid = newownerpid;
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
            if newownerpid == 0 {
                break;
            }
        }
        eprint_flush(" terminated\n");
        return 0;
    }
    match runmode {
        Start | Restart => {
            // SAFETY: getpid has no preconditions.
            let pidstr = format!("{}\n", unsafe { libc::getpid() });
            // SAFETY: truncate/write on our lockfile with a valid buffer.
            unsafe {
                if libc::ftruncate(lfd, 0) < 0 {
                    log(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, "can't truncate pidfile");
                }
                if libc::write(lfd, pidstr.as_ptr() as *const c_void, pidstr.len()) != pidstr.len() as isize {
                    log(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, "can't write pid to pidfile");
                }
            }
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, "lockfile created and locked");
            0
        }
        TryRestart => {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_ERR, "can't find process to restart");
            1
        }
        Stop | Kill => {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_ERR, "can't find process to terminate");
            0
        }
        Reload => {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_ERR, "can't find process to send reload signal");
            1
        }
        Info => {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_ERR, "can't find process to send info signal");
            1
        }
        Test => {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_NOTICE, &format!("{app} is not running"));
            1
        }
        Restore => 0,
    }
}

// ---------------------------------------------------------------------------
// Command line and process setup
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RunMode {
    Restart,
    Start,
    Stop,
    Reload,
    Info,
    Test,
    Kill,
    TryRestart,
    Restore,
}

impl RunMode {
    fn starts(self) -> bool {
        matches!(self, RunMode::Start | RunMode::Restart | RunMode::TryRestart)
    }
}

/// C `strcasecmp`-based run-mode keyword table.
fn parse_runmode(w: &[u8]) -> Option<RunMode> {
    let w = w.to_ascii_lowercase();
    Some(match w.as_slice() {
        b"start" => RunMode::Start,
        b"stop" => RunMode::Stop,
        b"restart" => RunMode::Restart,
        b"try-restart" => RunMode::TryRestart,
        b"reload" => RunMode::Reload,
        b"info" => RunMode::Info,
        b"test" | b"status" => RunMode::Test,
        b"kill" => RunMode::Kill,
        b"restore" => RunMode::Restore,
        _ => return None,
    })
}

fn usage(spec: &Spec, appname: &[u8]) -> ! {
    let text = format!(
        "usage: {} [-vhfdun] [-t locktimeout] [-c cfgfile] {}[start|stop|restart|reload|info|test|kill|restore]\n\
         \n\
         -v : print version number and exit\n\
         -h : print this info and exit\n\
         -f : run in foreground\n\
         -d : run with dangerous options (names: DANGEROUS_*)\n\
         -u : log undefined config variables\n\
         -n : do not attempt to increase limit of core dump size\n\
         -t locktimeout : how long wait for lockfile\n\
         -c cfgfile : use given config file\n\
         {}",
        String::from_utf8_lossy(appname),
        spec.module_synopsis,
        spec.module_desc
    );
    print_stdout(&text);
    exit(1)
}

fn print_stdout(s: &str) {
    use std::io::Write;
    let mut o = std::io::stdout();
    let _ = o.write_all(s.as_bytes());
    let _ = o.flush();
}

fn file_exists_readable(path: &str) -> Result<(), c_int> {
    std::fs::File::open(path).map(|_| ()).map_err(|e| e.raw_os_error().unwrap_or(0))
}

fn setrlimit(res: libc::__rlimit_resource_t, cur: u64, max: u64) -> bool {
    let r = libc::rlimit { rlim_cur: cur, rlim_max: max };
    // SAFETY: setrlimit reads a local struct.
    unsafe { libc::setrlimit(res, &r) >= 0 }
}

fn getrlimit(res: libc::__rlimit_resource_t) -> Option<libc::rlimit> {
    let mut r = libc::rlimit { rlim_cur: 0, rlim_max: 0 };
    // SAFETY: getrlimit writes a local struct.
    (unsafe { libc::getrlimit(res, &mut r) } >= 0).then_some(r)
}

/// C `main.c` open-files limit block.
fn raise_nofile(maxfiles: u32) {
    if !setrlimit(libc::RLIMIT_NOFILE, maxfiles as u64, maxfiles as u64) {
        log(
            MFSLOG_SYSLOG,
            MFSLOG_NOTICE,
            &format!("can't change open files limit to: {maxfiles} (trying to set smaller value)"),
        );
        if let Some(r) = getrlimit(libc::RLIMIT_NOFILE) {
            let mut limit: u32 = if r.rlim_max > maxfiles as u64 { maxfiles } else { r.rlim_max as u32 };
            while limit > 1024 {
                if setrlimit(libc::RLIMIT_NOFILE, limit as u64, r.rlim_max) {
                    log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_INFO,
                        &format!("open files limit has been set to: {limit}"),
                    );
                    break;
                }
                limit = limit.wrapping_mul(3) / 4;
            }
        }
    } else {
        log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, &format!("open files limit has been set to: {maxfiles}"));
    }
}

fn disable_oom_killer() {
    let (v, e) = cfg::get_uint8(b"DISABLE_OOM_KILLER", 1);
    cfg::set_erange(e);
    if v != 1 {
        return;
    }
    let write = |path: &str, val: &str| -> Option<bool> {
        let mut f = std::fs::OpenOptions::new().write(true).truncate(true).open(path).ok()?;
        use std::io::Write;
        Some(f.write_all(val.as_bytes()).and_then(|_| f.flush()).is_ok())
    };
    let oomdis = match write("/proc/self/oom_score_adj", "-1000\n") {
        Some(ok) => ok,
        None => write("/proc/self/oom_adj", "-17\n").unwrap_or(false),
    };
    if oomdis {
        log(MFSLOG_SYSLOG, MFSLOG_INFO, "out of memory killer disabled");
    } else {
        log(MFSLOG_SYSLOG, MFSLOG_WARNING, "can't disable out of memory killer");
    }
}

fn lock_memory() {
    let inf = libc::RLIM_INFINITY;
    match getrlimit(libc::RLIMIT_MEMLOCK) {
        None => log_errno(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_WARNING, "error getting memory lock limits"),
        Some(r) => {
            if r.rlim_cur != inf && r.rlim_max == inf && !setrlimit(libc::RLIMIT_MEMLOCK, inf, inf) {
                log_errno(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    "error setting memory lock limit to unlimited",
                );
            }
            match getrlimit(libc::RLIMIT_MEMLOCK) {
                None => log_errno(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    "error getting memory lock limits",
                ),
                Some(r) if r.rlim_cur != inf => log_errno(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    "can't set memory lock limit to unlimited",
                ),
                Some(_) => {
                    // SAFETY: mlockall has no memory preconditions.
                    if unsafe { libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE) } < 0 {
                        log_errno(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_WARNING, "memory lock error");
                    } else {
                        log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_INFO,
                            "process memory was successfully locked in RAM",
                        );
                    }
                }
            }
        }
    }
}

fn tune_glibc_arenas() {
    let (n, e) = cfg::get_uint8(b"LIMIT_GLIBC_MALLOC_ARENAS", 4);
    cfg::set_erange(e);
    if n == 0 {
        log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, "setting glibc malloc arenas turned off");
        return;
    }
    if std::env::var_os("MALLOC_ARENA_MAX").is_none() {
        log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, &format!("setting glibc malloc arena max to {n}"));
        // SAFETY: mallopt adjusts allocator parameters.
        unsafe { libc::mallopt(libc::M_ARENA_MAX, n as c_int) };
    }
    if std::env::var_os("MALLOC_ARENA_TEST").is_none() {
        log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, &format!("setting glibc malloc arena test to {n}"));
        // SAFETY: as above.
        unsafe { libc::mallopt(libc::M_ARENA_TEST, n as c_int) };
    }
}

/// Final teardown shared by the early-exit paths.
fn teardown() {
    signal_cleanup();
    cfg::term();
    wdunlock();
    // SAFETY: closes syslog and the debug log file.
    unsafe { mfs_log_term() };
}

static ARGC: AtomicI32 = AtomicI32::new(0);
static ARGV: AtomicPtr<*mut c_char> = AtomicPtr::new(core::ptr::null_mut());

/// glibc calls `.init_array` entries with (argc, argv, envp); this captures
/// the C runtime's own argv array. `processname_set` rewrites the process
/// title inside that memory block, and C `getopt` permutes that array in
/// place, so the daemon runtime must work on it rather than on a copy.
extern "C" fn capture_args(argc: c_int, argv: *mut *mut c_char, _envp: *mut *mut c_char) {
    ARGC.store(argc, Ordering::Relaxed);
    ARGV.store(argv, Ordering::Relaxed);
}

#[used]
#[unsafe(link_section = ".init_array.00099")]
static CAPTURE_ARGS: extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char) = capture_args;

/// The process argv as raw bytes (like C's `char **argv`).
fn raw_args() -> Vec<Vec<u8>> {
    let argc = ARGC.load(Ordering::Relaxed) as usize;
    let argv = ARGV.load(Ordering::Relaxed);
    assert!(!argv.is_null(), "glibc passes argv to .init_array constructors");
    // SAFETY: argv holds argc valid C strings (C runtime contract).
    (0..argc).map(|i| unsafe { CStr::from_ptr(*argv.add(i)) }.to_bytes().to_vec()).collect()
}

/// Apply getopt's permutation to the C runtime's argv array, as glibc's
/// `getopt` does in place.
fn permute_argv(order: &[usize]) {
    let argv = ARGV.load(Ordering::Relaxed);
    // SAFETY: argv has order.len() == argc writable pointer slots.
    unsafe {
        let orig: Vec<*mut c_char> = (0..order.len()).map(|i| *argv.add(i)).collect();
        for (i, &o) in order.iter().enumerate() {
            *argv.add(i) = orig[o];
        }
    }
}

/// C: `main()` of main.c. Returns the process exit status.
pub fn run(spec: &Spec) -> c_int {
    let args = raw_args();
    let app = spec.appname;
    crate::crc::mycrc32_init();

    let mut movewarning = false;
    let mut cfgfile = format!("{ETC_PATH}/mfs/{app}.cfg");
    if file_exists_readable(&cfgfile) == Err(libc::ENOENT) {
        let ocfgfile = format!("{ETC_PATH}/{app}.cfg");
        if file_exists_readable(&ocfgfile).is_ok() {
            cfgfile = ocfgfile;
            movewarning = true;
        }
    }
    let mut locktimeout: u32 = 1800;
    let mut rundaemon = true;
    let mut allowdangerous = false;
    let mut runmode = RunMode::Start;
    let mut logundefined = false;
    let mut forcecoredump = true;
    let mut userconfig = false;
    let appname = args.first().cloned().unwrap_or_default();

    let mut optstring = b"nuvfdc:t:h?".to_vec();
    optstring.extend(spec.module_options.iter().map(|(c, _)| *c));
    let posixly = std::env::var_os("POSIXLY_CORRECT").is_some();
    let mut g = Getopt::new(&args, &optstring, posixly);
    loop {
        match g.next() {
            Next::End => break,
            Next::Opt(b'v', _) => {
                print_stdout(&format!("version: {VERSSTR} ; build: {BUILDNO}\n"));
                return 0;
            }
            Next::Opt(b'f', _) => rundaemon = false,
            Next::Opt(b'd', _) => allowdangerous = true,
            Next::Opt(b't', a) => locktimeout = crate::cnum::strtoul10(&a.unwrap_or_default()).0 as u32,
            Next::Opt(b'c', a) => {
                cfgfile = String::from_utf8_lossy(&a.unwrap_or_default()).into_owned();
                movewarning = false;
                userconfig = true;
            }
            Next::Opt(b'u', _) => logundefined = true,
            Next::Opt(b'n', _) => forcecoredump = false,
            Next::Opt(c, _) => match spec.module_options.iter().find(|(o, _)| *o == c) {
                // SAFETY: module option action from the Spec.
                Some((_, f)) => unsafe { f() },
                None => usage(spec, &appname),
            },
            Next::Invalid(c) => {
                eprint_flush(&format!(
                    "{}: invalid option -- '{}'\n",
                    String::from_utf8_lossy(&appname),
                    c as char
                ));
                usage(spec, &appname)
            }
            Next::MissingArg(c) => {
                eprint_flush(&format!(
                    "{}: option requires an argument -- '{}'\n",
                    String::from_utf8_lossy(&appname),
                    c as char
                ));
                usage(spec, &appname)
            }
        }
    }
    // argv after getopt's permutation (the order processname_init sees)
    permute_argv(&g.order);
    let permuted: Vec<Vec<u8>> = g.order.iter().map(|&i| args[i].clone()).collect();
    let rest = &permuted[g.optind..];
    if rest.len() == 1 {
        match parse_runmode(&rest[0]) {
            Some(m) => runmode = m,
            None => usage(spec, &appname),
        }
    } else if !rest.is_empty() {
        usage(spec, &appname);
    }

    if movewarning {
        log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            &format!(
                "default sysconf path has changed - please move {app}.cfg from {ETC_PATH}/ to {ETC_PATH}/mfs/"
            ),
        );
    }
    if !cfg::load(cfgfile.as_bytes(), logundefined) {
        if userconfig {
            if rundaemon {
                signal_startup_failure();
                close_msg_channel();
            }
            return 1;
        }
        log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            &format!("can't load config file: {cfgfile} - using defaults"),
        );
    }
    if cfg::dangerous_options()
        && !allowdangerous
        && (runmode == RunMode::Reload || runmode.starts())
    {
        let what = if runmode == RunMode::Reload { "reload" } else { "start" };
        log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            &format!("Dangerous option(s) detected in config file: {cfgfile} - use '-d' option to force {what}"),
        );
        return 1;
    }

    if runmode.starts() {
        if rundaemon {
            makedaemon();
        } else {
            set_signal_handlers(false);
        }
    }

    // SAFETY: the C runtime's argv, valid for the process lifetime.
    unsafe {
        crate::processname::processname_init(ARGC.load(Ordering::Relaxed), ARGV.load(Ordering::Relaxed))
    };

    let logappname = cfg::get_str(b"SYSLOG_IDENT", app.as_bytes());
    let ident = CString::new(if logappname.is_empty() { app.as_bytes().to_vec() } else { logappname.clone() })
        .expect("config values have no NUL");
    // openlog keeps the pointer, so the CString is leaked below.
    // SAFETY: valid C string that outlives the process's syslog use.
    unsafe { mfs_log_init(ident.as_ptr(), rundaemon as c_int) };
    std::mem::forget(ident);
    let logappname = String::from_utf8_lossy(&logappname).into_owned();

    main_reload();

    let mut lockmemory = false;
    if runmode.starts() {
        raise_nofile(spec.maxfiles);
        let (lm, e) = cfg::get_num(b"LOCK_MEMORY", 0);
        cfg::set_erange(e);
        lockmemory = lm != 0;
        if lockmemory {
            setrlimit(libc::RLIMIT_MEMLOCK, libc::RLIM_INFINITY, libc::RLIM_INFINITY);
        }
        let (nice, e) = cfg::get_int32(b"NICE_LEVEL", -19);
        cfg::set_erange(e);
        // SAFETY: setpriority on our own pid.
        unsafe { libc::setpriority(libc::PRIO_PROCESS, libc::getpid() as libc::id_t, nice) };
        disable_oom_killer();
    }
    if spec.ionice {
        setrlimit(libc::RLIMIT_NICE, libc::RLIM_INFINITY, libc::RLIM_INFINITY);
        log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, "nice limit set to infinity");
        crate::ionice::ionice_test();
        crate::ionice::ionice_high();
    }

    changeugid();

    let wrkdir = String::from_utf8_lossy(&cfg::get_str(b"DATA_PATH", DATA_PATH.as_bytes())).into_owned();
    if runmode.starts() {
        log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, &format!("working directory: {wrkdir}"));
    }
    if std::env::set_current_dir(&wrkdir).is_err() {
        log(MFSLOG_SYSLOG_STDERR, MFSLOG_ERR, &format!("can't set working directory to {wrkdir}"));
        if rundaemon {
            signal_startup_failure();
            close_msg_channel();
        }
        // SAFETY: closes syslog and the debug log file.
        unsafe { mfs_log_term() };
        return 1;
    }
    let (umask, e) = cfg::get_uint32(b"FILE_UMASK", 0o27);
    cfg::set_erange(e);
    // SAFETY: umask has no preconditions.
    unsafe { libc::umask((umask & 0o77) as libc::mode_t) };

    let ch = wdlock(spec, runmode, locktimeout);
    if ch != 0 {
        if rundaemon {
            signal_startup_failure();
            close_msg_channel();
        }
        teardown();
        return ch as c_int;
    }

    let mut ch: c_int = 0;
    if runmode == RunMode::Restore && !run_tab(spec.restore_run_tab, "restore") {
        ch = 1;
    }
    if matches!(
        runmode,
        RunMode::Stop | RunMode::Kill | RunMode::Reload | RunMode::Info | RunMode::Test | RunMode::Restore
    ) {
        if rundaemon {
            close_msg_channel();
        }
        teardown();
        return ch;
    }

    if lockmemory {
        lock_memory();
    }
    if forcecoredump {
        setrlimit(libc::RLIMIT_CORE, libc::RLIM_INFINITY, libc::RLIM_INFINITY);
        // SAFETY: prctl(PR_SET_DUMPABLE) takes integer arguments.
        unsafe { libc::prctl(libc::PR_SET_DUMPABLE, 1) };
    }
    if spec.pthreads {
        tune_glibc_arenas();
    }

    // SAFETY: monotonic_method returns a static C string.
    let method = unsafe { CStr::from_ptr(monotonic_method()) }.to_string_lossy().into_owned();
    log(MFSLOG_SYSLOG, MFSLOG_INFO, &format!("monotonic clock function: {method}"));
    log(
        MFSLOG_SYSLOG,
        MFSLOG_INFO,
        &format!("monotonic clock speed: {} ops / 10 mili seconds", monotonic_speed()),
    );
    log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, &format!("initializing {logappname} modules ..."));

    if run_tab(spec.run_tab, "init") {
        log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, &format!("{logappname} daemon initialized properly"));
        if rundaemon {
            close_msg_channel();
        }
        let late = run_tab(spec.late_run_tab, "init");
        let n = time_now();
        START_TIME.store(n, Ordering::Relaxed);
        NOW.store(n, Ordering::Relaxed);
        if late {
            ch = mainloop(spec);
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, "exited from main loop");
        } else {
            ch = 1;
        }
    } else {
        log(MFSLOG_SYSLOG_STDERR, MFSLOG_ERR, "error occurred during initialization - exiting");
        if rundaemon {
            signal_startup_failure();
            close_msg_channel();
        }
        ch = 1;
    }
    log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, "exititng ...");
    let list = reg().destruct.clone();
            for e in list {
        // SAFETY: registered daemon callback.
        timed(&e.name, || unsafe { (e.fun)() });
    }
    signal_cleanup();
    cfg::term();
    wdunlock();
    log(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, &format!("process exited successfully (status:{ch})"));
    // SAFETY: closes syslog and the debug log file.
    unsafe { mfs_log_term() };
    ch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runmode_keywords_are_case_insensitive() {
        assert_eq!(parse_runmode(b"START"), Some(RunMode::Start));
        assert_eq!(parse_runmode(b"status"), Some(RunMode::Test));
        assert_eq!(parse_runmode(b"Try-Restart"), Some(RunMode::TryRestart));
        assert_eq!(parse_runmode(b"bogus"), None);
    }

    #[test]
    fn timer_grid_matches_c() {
        assert_eq!(first_event(1_000_500, 1_000_000, 0), 2_000_000);
        assert_eq!(first_event(2_000_000, 1_000_000, 0), 2_000_000);
        assert_eq!(first_event(2_000_000, 1_000_000, 250), 2_000_250);
        // forward jump: strictly after now
        assert_eq!(rebase_forward(1_000_000, 0, 5_000_000), 6_000_000);
        // backward jump keeps the remaining wait (capped at one period)
        assert_eq!(rebase_backward(10_300_000, 1_000_000, 0, 5_000_000, 10_000_000), 6_000_000);
        assert_eq!(rebase_backward(10_900_000, 1_000_000, 0, 5_000_000, 10_000_000), 6_000_000);
    }

    #[test]
    fn argv_is_captured_from_the_c_runtime() {
        let args = raw_args();
        assert!(!args.is_empty());
        let std_args: Vec<Vec<u8>> = {
            use std::os::unix::ffi::OsStringExt;
            std::env::args_os().map(|a| a.into_vec()).collect()
        };
        assert_eq!(args, std_args);
    }
}
