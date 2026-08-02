//! Rust port of mfscommon/workers.c (elastic worker pool over a typed SQueue).
//!
//! pthread usage replaced by std::thread + Mutex/Condvar. C semantics
//! preserved exactly (see /tmp/moosefs-ref/mfscommon/workers.c):
//! - worker threads inherit a blocked SIGTERM/SIGINT/SIGHUP/SIGQUIT mask
//!   (blocked around spawn, restored after, like pthread_sigmask in C).
//! - stack size: max(sysconf(_SC_THREAD_STACK_MIN), 0x20000), matching
//!   C's PTHREAD_STACK_MIN clamp.
//! - spawn failure is ignored (C checked `res < 0`, dead: pthread_create
//!   never returns negative) — avail/total still advance, as in C.
//! - workers are detached (C: pthread_detach + free(w)); termination is
//!   coordinated by the term_cond / total==0 protocol, not joins.
//! - pool elasticity: avail > sustainworkers retires a worker; avail == 0
//!   with total < maxworkers spawns one.
//! - avail/total crossing a multiple of 10 logs "%s workers: %u+" / "%u-".
//!
//! The pool struct is Box-allocated (C malloc'd it); the opaque handle is
//! stable for the pool's lifetime and freed by workers_term only after all
//! workers exited, so worker threads may safely dereference it.

use std::ffi::{CStr, CString, c_char, c_void};
use std::sync::{Condvar, Mutex};

use super::squeue::{MallocPtr, SQueue};

type WorkerFn = unsafe extern "C" fn(*mut c_void, u32);

unsafe extern "C" {
    fn mfs_log(mode: ::core::ffi::c_int, priority: ::core::ffi::c_int, fmt: *const c_char, ...);
}

const MFSLOG_SYSLOG: ::core::ffi::c_int = 0;
const MFSLOG_INFO: ::core::ffi::c_int = 1;

struct State {
    avail: u32,
    total: u32,
    lastnotify: u32,
}

struct Workers {
    sustainworkers: u32,
    maxworkers: u32,
    jqueue: SQueue<MallocPtr<c_void>>,
    name: CString,
    workerfn: WorkerFn,
    stack_size: usize,
    state: Mutex<State>,
    term_cond: Condvar,
}

/// Pool pointer handed to worker threads.
#[derive(Clone, Copy)]
struct WorkersPtr(*const Workers);

// SAFETY: workers deref the pointer only while the pool is alive;
// workers_term waits for total == 0 before freeing the Box, so the pointee
// outlives every worker thread. All shared fields are Mutex/Condvar-guarded
// or immutable after init.
unsafe impl Send for WorkersPtr {}

/// ws->lock:LOCKED (state guard held), as in C.
fn spawn_worker(ws: WorkersPtr, state: &mut State) {
    // SAFETY: pool alive (see WorkersPtr).
    let wsr = unsafe { &*ws.0 };
    // C blocks these signals in the spawning thread so the new worker
    // inherits the blocked mask; restore the old mask right after spawn.
    let mut newset: libc::sigset_t = unsafe { std::mem::zeroed() };
    let mut oldset: libc::sigset_t = unsafe { std::mem::zeroed() };
    unsafe {
        libc::sigemptyset(&mut newset);
        libc::sigaddset(&mut newset, libc::SIGTERM);
        libc::sigaddset(&mut newset, libc::SIGINT);
        libc::sigaddset(&mut newset, libc::SIGHUP);
        libc::sigaddset(&mut newset, libc::SIGQUIT);
        libc::pthread_sigmask(libc::SIG_BLOCK, &newset, &mut oldset);
    }
    let res = std::thread::Builder::new()
        .stack_size(wsr.stack_size)
        .spawn(move || worker_loop(ws));
    unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, &oldset, std::ptr::null_mut()) };
    // C: `if (res<0) return;` — dead check, pthread_create never returns
    // negative. Spawn failure is ignored and counters still advance.
    // Dropping the JoinHandle detaches the thread (C: pthread_detach).
    drop(res);
    state.avail = state.avail.wrapping_add(1);
    state.total = state.total.wrapping_add(1);
    if state.total % 10 == 0 && state.total != state.lastnotify {
        // SAFETY: static format string, name is a live CString, u32 matches %u.
        unsafe {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                c"%s workers: %u+".as_ptr(),
                wsr.name.as_ptr(),
                state.total,
            )
        };
        state.lastnotify = state.total;
    }
}

/// ws->lock:LOCKED (state guard held), as in C.
fn close_worker(wsr: &Workers, state: &mut State) {
    state.avail = state.avail.wrapping_sub(1);
    state.total = state.total.wrapping_sub(1);
    if state.total % 10 == 0 && state.total != state.lastnotify {
        // SAFETY: static format string, name is a live CString, u32 matches %u.
        unsafe {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                c"%s workers: %u-".as_ptr(),
                wsr.name.as_ptr(),
                state.total,
            )
        };
        state.lastnotify = state.total;
    }
    if state.total == 0 {
        wsr.term_cond.notify_one();
    }
    // C: pthread_detach + free(w). std threads are detached (JoinHandle
    // dropped at spawn); the C `worker` struct has no Rust equivalent.
}

fn worker_loop(ws: WorkersPtr) {
    // SAFETY: pool alive (see WorkersPtr).
    let wsr = unsafe { &*ws.0 };
    let mut firstrun = true;
    loop {
        if !firstrun {
            let mut state = wsr.state.lock().unwrap();
            state.avail = state.avail.wrapping_add(1);
            if state.avail > wsr.sustainworkers {
                close_worker(wsr, &mut state);
                return;
            }
        }
        firstrun = false;

        let data = wsr.jqueue.get();

        let mut state = wsr.state.lock().unwrap();
        let Some(data) = data else {
            close_worker(wsr, &mut state);
            return;
        };
        state.avail = state.avail.wrapping_sub(1);
        if state.avail == 0 && state.total < wsr.maxworkers {
            spawn_worker(ws, &mut state);
        }
        let current_workers = state.total;
        drop(state);
        // SAFETY: workerfn is the daemon's nbd_worker_fn; data is a uniquely
        // owned malloc allocation handed off to it exactly as in C.
        unsafe { (wsr.workerfn)(data.into_raw(), current_workers) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn workers_init(
    maxworkers: u32,
    sustainworkers: u32,
    qleng: u32,
    name: *mut c_char,
    workerfn: Option<WorkerFn>,
) -> *mut c_void {
    // SAFETY: name is a valid NUL-terminated string from the caller.
    let name = unsafe { CStr::from_ptr(name) }.to_owned();
    // C: PTHREAD_STACK_MIN (glibc: sysconf(_SC_THREAD_STACK_MIN)),
    // clamped up to 0x20000.
    let stack_size = (unsafe { libc::sysconf(libc::_SC_THREAD_STACK_MIN) } as usize).max(0x20000);
    let ws = Box::new(Workers {
        sustainworkers,
        maxworkers,
        jqueue: SQueue::new(qleng),
        name,
        workerfn: workerfn.expect("non-null function pointer"),
        stack_size,
        state: Mutex::new(State {
            avail: 0,
            total: 0,
            lastnotify: 0,
        }),
        term_cond: Condvar::new(),
    });
    // C passert(malloc): Box aborts on OOM the same way.
    let ws = Box::into_raw(ws);
    let mut state = unsafe { &*ws }.state.lock().unwrap();
    spawn_worker(WorkersPtr(ws), &mut state);
    drop(state);
    ws as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn workers_term(wsv: *mut c_void) {
    // SAFETY: wsv is the workers_init handle, terminated exactly once.
    let ws = unsafe { Box::from_raw(wsv as *mut Workers) };
    ws.jqueue.close();
    let mut state = ws.state.lock().unwrap();
    while state.total > 0 {
        state = ws.term_cond.wait(state).unwrap();
    }
    drop(state);
    // Box drop order matches C: SQueue drop frees queued elements
    // (squeue_delete), CString frees the name; mutex/cond need no destroy.
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn workers_newjob(wsv: *mut c_void, data: *mut c_void) {
    // SAFETY: wsv is a live pool handle; data is a uniquely owned malloc
    // allocation transferred to the queue.
    let ws = unsafe { &*(wsv as *const Workers) };
    // Queue is unbounded in this daemon, so put never fails; on a bounded
    // closed queue C keeps the data with the caller, here MallocPtr frees it.
    let _ = ws.jqueue.put(unsafe { MallocPtr::from_raw(data) });
}
