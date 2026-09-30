//! Producer/consumer queue — port of mfscommon/pcqueue.c.
//!
//! Shared by plfsmaster and plfschunkserver (their c2rust copies differed
//! only in `__FILE__`). The queue is a `Mutex` + two `Condvar`s over a
//! `VecDeque`; the C waiter counters (`freewaiting`/`fullwaiting`) are kept
//! so wakeups are issued exactly when C would signal/broadcast. Payload
//! pointers are owned by the queue while enqueued and released with libc
//! `free` on `queue_delete`, as in C.
//!
//! C ABI: the handle is an opaque `void*` (a leaked `Box<Queue>`); every
//! export keeps its pcqueue.h signature and `errno` side effects (EDEADLK,
//! EIO, EBUSY). Entry allocation failure aborts via Rust's allocator
//! handler instead of `passert` (both abort).

use core::ffi::{c_int, c_void};

use crate::mfslog::assert_abort;

pub use imp::Queue;

const PCQUEUE_C: &str = "../mfscommon/pcqueue.c";

#[deny(unsafe_code)]
mod imp {
    use std::collections::VecDeque;
    use std::sync::{Condvar, Mutex, MutexGuard};

    /// Payload pointer (owned by the queue while enqueued).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Data(pub usize);

    #[derive(Debug, PartialEq, Eq)]
    pub struct Entry {
        pub id: u32,
        pub op: u32,
        pub data: Data,
        pub leng: u32,
    }

    struct Inner {
        entries: VecDeque<Entry>,
        size: u32,
        maxsize: u32,
        freewaiting: u32,
        fullwaiting: u32,
        closed: bool,
    }

    pub struct Queue {
        inner: Mutex<Inner>,
        waitfree: Condvar,
        waitfull: Condvar,
    }

    impl Queue {
        /// C: `queue_new(size)` (0 = unbounded).
        pub fn new(maxsize: u32) -> Self {
            Queue {
                inner: Mutex::new(Inner {
                    entries: VecDeque::new(),
                    size: 0,
                    maxsize,
                    freewaiting: 0,
                    fullwaiting: 0,
                    closed: false,
                }),
                waitfree: Condvar::new(),
                waitfull: Condvar::new(),
            }
        }

        fn lock(&self) -> MutexGuard<'_, Inner> {
            self.inner.lock().unwrap_or_else(|e| e.into_inner())
        }

        /// C `queue_delete` preconditions and payload release: the remaining
        /// payloads, or Err((line, expr)) of the failed `sassert`.
        pub fn drain_for_delete(&self) -> Result<Vec<Data>, (u32, &'static str)> {
            let mut q = self.lock();
            if q.freewaiting != 0 {
                return Err((75, "q->freewaiting==0"));
            }
            if q.fullwaiting != 0 {
                return Err((76, "q->fullwaiting==0"));
            }
            Ok(q.entries.drain(..).map(|e| e.data).collect())
        }

        /// C: `queue_close`.
        pub fn close(&self) {
            let mut q = self.lock();
            q.closed = true;
            if q.freewaiting > 0 {
                self.waitfree.notify_all();
                q.freewaiting = 0;
            }
            if q.fullwaiting > 0 {
                self.waitfull.notify_all();
                q.fullwaiting = 0;
            }
        }

        pub fn is_empty(&self) -> bool {
            self.lock().entries.is_empty()
        }

        pub fn elements(&self) -> u32 {
            self.lock().entries.len() as u32
        }

        pub fn is_full(&self) -> bool {
            let q = self.lock();
            q.maxsize > 0 && q.maxsize <= q.size
        }

        pub fn size_left(&self) -> u32 {
            let q = self.lock();
            if q.maxsize > 0 { q.maxsize.wrapping_sub(q.size) } else { 0xFFFF_FFFF }
        }

        fn push(&self, mut q: MutexGuard<'_, Inner>, e: Entry) {
            q.size = q.size.wrapping_add(e.leng);
            q.entries.push_back(e);
            if q.freewaiting > 0 {
                self.waitfree.notify_one();
                q.freewaiting -= 1;
            }
        }

        /// C: `queue_put` — blocks while full; Err(errno) on failure.
        pub fn put(&self, e: Entry) -> Result<(), i32> {
            let mut q = self.lock();
            if q.maxsize != 0 {
                if e.leng > q.maxsize {
                    return Err(libc::EDEADLK);
                }
                while q.size.wrapping_add(e.leng) > q.maxsize && !q.closed {
                    q.fullwaiting += 1;
                    q = self.waitfull.wait(q).unwrap_or_else(|p| p.into_inner());
                }
                if q.closed {
                    return Err(libc::EIO);
                }
            }
            self.push(q, e);
            Ok(())
        }

        /// C: `queue_tryput`.
        pub fn try_put(&self, e: Entry) -> Result<(), i32> {
            let q = self.lock();
            if q.maxsize != 0 {
                if e.leng > q.maxsize {
                    return Err(libc::EDEADLK);
                }
                if q.size.wrapping_add(e.leng) > q.maxsize {
                    return Err(libc::EBUSY);
                }
            }
            self.push(q, e);
            Ok(())
        }

        fn pop(&self, mut q: MutexGuard<'_, Inner>) -> Entry {
            let e = q.entries.pop_front().expect("caller checked non-empty");
            q.size = q.size.wrapping_sub(e.leng);
            if q.fullwaiting > 0 {
                self.waitfull.notify_one();
                q.fullwaiting -= 1;
            }
            e
        }

        /// C: `queue_get` — blocks while empty; a closed queue fails with
        /// EIO even if entries remain (C checks `closed` first).
        pub fn get(&self) -> Result<Entry, i32> {
            let mut q = self.lock();
            while q.entries.is_empty() && !q.closed {
                q.freewaiting += 1;
                q = self.waitfree.wait(q).unwrap_or_else(|p| p.into_inner());
            }
            if q.closed {
                return Err(libc::EIO);
            }
            Ok(self.pop(q))
        }

        /// C: `queue_tryget` (ignores `closed`, as C does).
        pub fn try_get(&self) -> Result<Entry, i32> {
            let q = self.lock();
            if q.entries.is_empty() {
                return Err(libc::EBUSY);
            }
            Ok(self.pop(q))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::Arc;
        use std::time::Duration;

        fn ent(id: u32, leng: u32) -> Entry {
            Entry { id, op: id * 10, data: Data(id as usize), leng }
        }

        #[test]
        fn fifo_and_accounting() {
            let q = Queue::new(0);
            assert!(q.is_empty());
            assert_eq!(q.size_left(), 0xFFFF_FFFF);
            q.put(ent(1, 5)).unwrap();
            q.try_put(ent(2, 7)).unwrap();
            assert_eq!(q.elements(), 2);
            assert!(!q.is_full());
            assert_eq!(q.get().unwrap(), ent(1, 5));
            assert_eq!(q.try_get().unwrap(), ent(2, 7));
            assert_eq!(q.try_get(), Err(libc::EBUSY));
        }

        #[test]
        fn bounded_limits_match_c() {
            let q = Queue::new(10);
            assert_eq!(q.put(ent(1, 11)), Err(libc::EDEADLK));
            assert_eq!(q.try_put(ent(1, 11)), Err(libc::EDEADLK));
            q.try_put(ent(1, 6)).unwrap();
            assert_eq!(q.try_put(ent(2, 5)), Err(libc::EBUSY));
            assert_eq!(q.size_left(), 4);
            q.try_put(ent(3, 4)).unwrap();
            assert!(q.is_full());
            q.try_put(ent(4, 0)).unwrap(); // zero-length fits even when full
            assert_eq!(q.elements(), 3);
        }

        #[test]
        fn blocking_put_waits_for_space_and_close_wakes() {
            let q = Arc::new(Queue::new(4));
            q.put(ent(1, 4)).unwrap();
            let q2 = Arc::clone(&q);
            let t = std::thread::spawn(move || q2.put(ent(2, 3)));
            std::thread::sleep(Duration::from_millis(50));
            assert_eq!(q.get().unwrap(), ent(1, 4));
            assert_eq!(t.join().unwrap(), Ok(()));
            assert_eq!(q.elements(), 1);

            let q3 = Arc::clone(&q);
            let t = std::thread::spawn(move || q3.put(ent(3, 4)));
            std::thread::sleep(Duration::from_millis(50));
            q.close();
            assert_eq!(t.join().unwrap(), Err(libc::EIO));
            // closed: get fails even with an entry present; tryget still works
            assert_eq!(q.get(), Err(libc::EIO));
            assert_eq!(q.try_get().unwrap(), ent(2, 3));
        }

        #[test]
        fn blocking_get_wakes_on_put_and_close() {
            let q = Arc::new(Queue::new(0));
            let q2 = Arc::clone(&q);
            let t = std::thread::spawn(move || q2.get());
            std::thread::sleep(Duration::from_millis(50));
            q.put(ent(9, 1)).unwrap();
            assert_eq!(t.join().unwrap(), Ok(ent(9, 1)));
            let q3 = Arc::clone(&q);
            let t = std::thread::spawn(move || q3.get());
            std::thread::sleep(Duration::from_millis(50));
            q.close();
            assert_eq!(t.join().unwrap(), Err(libc::EIO));
            assert_eq!(q.drain_for_delete(), Ok(vec![]));
        }
    }
}

// ---------------------------------------------------------------------------
// C ABI boundary (pcqueue.h). `que` is the pointer returned by queue_new.
// ---------------------------------------------------------------------------

/// # Safety
// SAFETY: caller guarantees `que` came from `queue_new` and is not deleted.
unsafe fn q<'a>(que: *mut c_void) -> &'a Queue {
    // SAFETY: per fn contract `que` is a live leaked Box<Queue>.
    unsafe { &*(que as *const Queue) }
}

fn set_errno(e: i32) {
    // SAFETY: thread-local errno location is always valid.
    unsafe { *libc::__errno_location() = e };
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn queue_new(size: u32) -> *mut c_void {
    Box::into_raw(Box::new(Queue::new(size))) as *mut c_void
}

/// # Safety
/// `que` from `queue_new`; no other thread may use it concurrently or after.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_delete(que: *mut c_void) {
    // SAFETY: per fn contract.
    match unsafe { q(que) }.drain_for_delete() {
        Ok(data) => {
            for d in data {
                // SAFETY: payloads were handed to the queue as malloc'd
                // buffers (C contract: queue_delete frees them).
                unsafe { libc::free(d.0 as *mut c_void) };
            }
        }
        Err((line, expr)) => assert_abort(PCQUEUE_C, line, expr),
    }
    // SAFETY: per fn contract this is the last use of the Box.
    drop(unsafe { Box::from_raw(que as *mut Queue) });
}

/// # Safety
/// `que` from `queue_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_close(que: *mut c_void) {
    // SAFETY: per fn contract.
    unsafe { q(que) }.close()
}

/// # Safety
/// `que` from `queue_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_isempty(que: *mut c_void) -> c_int {
    // SAFETY: per fn contract.
    unsafe { q(que) }.is_empty() as c_int
}

/// # Safety
/// `que` from `queue_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_elements(que: *mut c_void) -> u32 {
    // SAFETY: per fn contract.
    unsafe { q(que) }.elements()
}

/// # Safety
/// `que` from `queue_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_isfull(que: *mut c_void) -> c_int {
    // SAFETY: per fn contract.
    unsafe { q(que) }.is_full() as c_int
}

/// # Safety
/// `que` from `queue_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_sizeleft(que: *mut c_void) -> u32 {
    // SAFETY: per fn contract.
    unsafe { q(que) }.size_left()
}

fn entry(id: u32, op: u32, data: *mut u8, leng: u32) -> imp::Entry {
    imp::Entry { id, op, data: imp::Data(data as usize), leng }
}

fn status(r: Result<(), i32>) -> c_int {
    match r {
        Ok(()) => 0,
        Err(e) => {
            set_errno(e);
            -1
        }
    }
}

/// # Safety
/// `que` from `queue_new`; `data` ownership passes to the queue on success.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_put(que: *mut c_void, id: u32, op: u32, data: *mut u8, leng: u32) -> c_int {
    // SAFETY: per fn contract.
    status(unsafe { q(que) }.put(entry(id, op, data, leng)))
}

/// # Safety
/// As `queue_put`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_tryput(que: *mut c_void, id: u32, op: u32, data: *mut u8, leng: u32) -> c_int {
    // SAFETY: per fn contract.
    status(unsafe { q(que) }.try_put(entry(id, op, data, leng)))
}

/// Write the C out-params (each may be NULL); a failure zeroes them.
///
/// # Safety
// SAFETY: caller guarantees non-null out-params are writable.
unsafe fn deliver(
    r: Result<imp::Entry, i32>,
    id: *mut u32,
    op: *mut u32,
    data: *mut *mut u8,
    leng: *mut u32,
) -> c_int {
    let (e, ret) = match r {
        Ok(e) => (e, 0),
        Err(err) => {
            set_errno(err);
            (entry(0, 0, core::ptr::null_mut(), 0), -1)
        }
    };
    // SAFETY: per fn contract; each pointer is checked for NULL as in C.
    unsafe {
        if !id.is_null() {
            *id = e.id;
        }
        if !op.is_null() {
            *op = e.op;
        }
        if !data.is_null() {
            *data = e.data.0 as *mut u8;
        }
        if !leng.is_null() {
            *leng = e.leng;
        }
    }
    ret
}

/// # Safety
/// `que` from `queue_new`; non-null out-params writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_get(
    que: *mut c_void,
    id: *mut u32,
    op: *mut u32,
    data: *mut *mut u8,
    leng: *mut u32,
) -> c_int {
    // SAFETY: per fn contract.
    unsafe { deliver(q(que).get(), id, op, data, leng) }
}

/// # Safety
/// As `queue_get`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_tryget(
    que: *mut c_void,
    id: *mut u32,
    op: *mut u32,
    data: *mut *mut u8,
    leng: *mut u32,
) -> c_int {
    // SAFETY: per fn contract.
    unsafe { deliver(q(que).try_get(), id, op, data, leng) }
}
