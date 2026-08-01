//! Delayed-task scheduler (min-heap + janitor thread), migrated to safe
//! Rust (P1). The malloc'd raw heap becomes std's `BinaryHeap<Reverse<…>>`;
//! pthread mutex/cond become `Mutex` + `Condvar`; the abort-on-error macro
//! taxonomy collapses into unwrap (poison = abort-equivalent).
//!
//! Symbol notes: `delay_heap_sort_up/down` and `delay_scheduler` were
//! exported by c2rust but are referenced by no daemon (verified by grep,
//! P1) — the sort helpers are dropped; the scheduler keeps its symbol only
//! because it is the pthread start routine.
//!
//! Callbacks cross the thread boundary exactly as the C original: the caller
//! hands ownership of `udata` to the scheduler thread.

pub type size_t = usize;
pub type pthread_t = ::core::ffi::c_ulong;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

use std::collections::BinaryHeap;
use std::sync::{Condvar, Mutex};
use std::time::Duration;

unsafe extern "C" {
    fn monotonic_useconds() -> uint64_t;
    fn lwt_minthread_create(
        th: *mut pthread_t,
        detached: uint8_t,
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_join(
        th: pthread_t,
        thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

pub type DelayFn = unsafe extern "C" fn(*mut ::core::ffi::c_void);

/// owned callback payload crossing to the scheduler thread
struct Udata(*mut ::core::ffi::c_void);
// SAFETY: the C contract hands `udata` ownership to the scheduler at
// delay_run; no aliasing guarantees are needed beyond that (as in C).
unsafe impl Send for Udata {}

struct Elem {
    firetime: uint64_t,
    f: DelayFn,
    udata: Udata,
}
impl PartialEq for Elem {
    fn eq(&self, o: &Self) -> bool {
        self.firetime == o.firetime
    }
}
impl Eq for Elem {}
impl PartialOrd for Elem {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Elem {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        o.firetime.cmp(&self.firetime) // reversed: BinaryHeap becomes min-heap
    }
}

#[derive(Default)]
struct State {
    heap: BinaryHeap<Elem>,
    exitflag: bool,
}

static STATE: Mutex<State> = Mutex::new(State {
    heap: BinaryHeap::new(),
    exitflag: false,
});
static COND: Condvar = Condvar::new();
static SCHEDULER: Mutex<Option<pthread_t>> = Mutex::new(None);

/// # Safety
/// pthread start-routine signature; `arg` passed through, as the original.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delay_scheduler(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut guard = STATE.lock().unwrap();
    loop {
        if guard.exitflag {
            return arg;
        }
        let Some(top) = guard.heap.peek() else {
            guard = COND.wait(guard).unwrap();
            continue;
        };
        let firetime = top.firetime;
        // SAFETY: extern, no args.
        let now = unsafe { monotonic_useconds() };
        if now < firetime {
            let (g, _timeout) = COND
                .wait_timeout(guard, Duration::from_micros(firetime - now))
                .unwrap();
            guard = g;
        } else {
            let elem = guard.heap.pop().unwrap();
            drop(guard);
            // SAFETY: caller contract — f and udata were paired at delay_run.
            unsafe { (elem.f)(elem.udata.0) };
            guard = STATE.lock().unwrap();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn delay_run(
    mut r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>,
    mut udata: *mut ::core::ffi::c_void,
    mut useconds: uint64_t,
) {
    let f = r#fn.expect("delay_run with null fn");
    // SAFETY: extern, no args.
    let firetime = unsafe { monotonic_useconds() }.wrapping_add(useconds);
    let mut guard = STATE.lock().unwrap();
    guard.heap.push(Elem {
        firetime,
        f,
        udata: Udata(udata),
    });
    // wake the scheduler only if the new task is now the soonest
    if matches!(guard.heap.peek(), Some(t) if t.firetime == firetime) {
        COND.notify_one();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn delay_term() {
    {
        let mut guard = STATE.lock().unwrap();
        guard.exitflag = true;
        COND.notify_one();
    }
    let th = SCHEDULER.lock().unwrap().take();
    if let Some(th) = th {
        // SAFETY: extern; th is the scheduler thread, joinable, alive.
        unsafe { pthread_join(th, std::ptr::null_mut()) };
    }
    STATE.lock().unwrap().heap.clear();
}

#[unsafe(no_mangle)]
pub extern "C" fn delay_init() {
    {
        let mut guard = STATE.lock().unwrap();
        guard.exitflag = false;
        guard.heap.clear();
    }
    let mut th: pthread_t = 0;
    // SAFETY: th written by the call; scheduler is joinable (detached=0),
    // joined in delay_term.
    let rc =
        unsafe { lwt_minthread_create(&mut th, 0, Some(delay_scheduler), std::ptr::null_mut()) };
    if rc < 0 {
        // original abort()ed on thread-create failure paths
        // SAFETY: abort has no preconditions.
        unsafe { libc::abort() };
    }
    *SCHEDULER.lock().unwrap() = Some(th);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static FIRED: AtomicU32 = AtomicU32::new(0);

    unsafe extern "C" fn bump(p: *mut ::core::ffi::c_void) {
        // SAFETY: test passes a valid &AtomicU32 as udata.
        unsafe { (*(p as *const AtomicU32)).fetch_add(1, Ordering::SeqCst) };
    }

    #[test]
    fn fires_in_order_and_term() {
        delay_init();
        let counter = AtomicU32::new(0);
        delay_run(Some(bump), &counter as *const _ as *mut _, 50_000);
        delay_run(Some(bump), &counter as *const _ as *mut _, 10_000);
        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(counter.load(Ordering::SeqCst), 2);
        delay_term();
        assert!(SCHEDULER.lock().unwrap().is_none());
    }
}
