//! Producer-consumer queue — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfscommon/pcqueue.c. Bounded (byte-size) blocking
//! queue of (id, op, data, leng) entries shared between mfsclient
//! readdata/writedata worker threads. Entry data is a libc-malloc'd buffer
//! whose ownership follows the C contract: queue_put takes it, queue_get
//! hands it to the caller, queue_delete frees leftovers.
//!
//! Safe core in `imp`: Mutex<VecDeque> + two Condvars. The C freewaiting/
//! fullwaiting counters existed only to decide signal-vs-skip and are
//! dropped: notify_one is a no-op without waiters, so signaling
//! unconditionally is behaviorally identical (close uses notify_all).
//!
//! errno mapping (boundary): EDEADLK entry too large, EBUSY try-would-
//! block, EIO closed.

use std::sync::{Condvar, Mutex};

unsafe extern "C" {
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub const EDEADLK: ::core::ffi::c_int = 35;
pub const EBUSY: ::core::ffi::c_int = 16;
pub const EIO: ::core::ffi::c_int = 5;

#[deny(unsafe_code)]
pub mod imp {
    use super::*;
    use std::collections::VecDeque;

    pub struct QEntry {
        pub id: u32,
        pub op: u32,
        /// libc-malloc'd buffer, owned per the C contract (never derefed
        /// by the queue itself)
        pub data: *mut u8,
        pub leng: u32,
    }
    struct State {
        q: VecDeque<QEntry>,
        elements: u32,
        size: u32,
        maxsize: u32,
        closed: bool,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum PutError {
        /// leng > maxsize (EDEADLK)
        TooLarge,
        /// closed while waiting (EIO)
        Closed,
        /// tryput would block (EBUSY)
        Busy,
    }

    pub struct Queue {
        inner: Mutex<State>,
        waitfree: Condvar,
        waitfull: Condvar,
    }

    impl Queue {
        pub fn new(maxsize: u32) -> Self {
            Queue {
                inner: Mutex::new(State {
                    q: VecDeque::new(),
                    elements: 0,
                    size: 0,
                    maxsize,
                    closed: false,
                }),
                waitfree: Condvar::new(),
                waitfull: Condvar::new(),
            }
        }

        pub fn close(&self) {
            let mut s = self.inner.lock().unwrap();
            s.closed = true;
            self.waitfree.notify_all();
            self.waitfull.notify_all();
        }

        pub fn is_empty(&self) -> bool {
            self.inner.lock().unwrap().elements == 0
        }

        pub fn elements(&self) -> u32 {
            self.inner.lock().unwrap().elements
        }

        pub fn is_full(&self) -> bool {
            let s = self.inner.lock().unwrap();
            s.maxsize > 0 && s.maxsize <= s.size
        }

        pub fn size_left(&self) -> u32 {
            let s = self.inner.lock().unwrap();
            if s.maxsize > 0 {
                s.maxsize - s.size
            } else {
                u32::MAX
            }
        }

        fn push(&self, s: &mut State, e: QEntry) {
            s.elements += 1;
            s.size += e.leng;
            s.q.push_back(e);
            self.waitfree.notify_one();
        }

        fn pop(&self, s: &mut State) -> QEntry {
            let e = s.q.pop_front().unwrap();
            s.elements -= 1;
            s.size -= e.leng;
            self.waitfull.notify_one();
            e
        }

        /// Blocking put. Caller keeps `e` on error (C freed the wrapper
        /// but the data handling is the boundary's business).
        pub fn put(&self, e: QEntry) -> Result<(), PutError> {
            let mut s = self.inner.lock().unwrap();
            if s.maxsize > 0 {
                if e.leng > s.maxsize {
                    return Err(PutError::TooLarge);
                }
                while s.size + e.leng > s.maxsize && !s.closed {
                    s = self.waitfull.wait(s).unwrap();
                }
                if s.closed {
                    return Err(PutError::Closed);
                }
            }
            self.push(&mut s, e);
            Ok(())
        }

        pub fn tryput(&self, e: QEntry) -> Result<(), PutError> {
            let mut s = self.inner.lock().unwrap();
            if s.maxsize > 0 {
                if e.leng > s.maxsize {
                    return Err(PutError::TooLarge);
                }
                if s.size + e.leng > s.maxsize {
                    return Err(PutError::Busy);
                }
            }
            self.push(&mut s, e);
            Ok(())
        }

        /// Blocking get. Err(()) = closed (EIO); tryget Err(()) = empty
        /// (EBUSY) — the boundary maps the errno.
        pub fn get(&self) -> Result<QEntry, ()> {
            let mut s = self.inner.lock().unwrap();
            while s.elements == 0 && !s.closed {
                s = self.waitfree.wait(s).unwrap();
            }
            if s.closed {
                return Err(());
            }
            Ok(self.pop(&mut s))
        }

        pub fn tryget(&self) -> Result<QEntry, ()> {
            let mut s = self.inner.lock().unwrap();
            if s.elements == 0 {
                return Err(());
            }
            Ok(self.pop(&mut s))
        }

        /// queue_delete: drain remaining entries (boundary frees their
        /// data buffers), C asserts no threads are waiting.
        pub fn drain(&self) -> Vec<QEntry> {
            let mut s = self.inner.lock().unwrap();
            s.elements = 0;
            s.size = 0;
            s.q.drain(..).collect()
        }
    }
}

use imp::{PutError, QEntry, Queue};

// SAFETY: data ownership transfers between threads exactly as in C — the
// queue never dereferences the buffer, and each buffer has exactly one
// owner at a time (put → queue → get/caller, or delete → free).
unsafe impl Send for QEntry {}

/// SAFETY: caller sets errno (linux constants).
unsafe fn set_errno(e: ::core::ffi::c_int) {
    unsafe {
        *__errno_location() = e;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_new(size: uint32_t) -> *mut ::core::ffi::c_void {
    Box::into_raw(Box::new(Queue::new(size))) as *mut ::core::ffi::c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_delete(que: *mut ::core::ffi::c_void) {
    if que.is_null() {
        return;
    }
    // SAFETY: handle from queue_new, deleted exactly once; C asserts no
    // waiters at delete time.
    let q = unsafe { Box::from_raw(que as *mut Queue) };
    for e in q.drain() {
        // SAFETY: leftover data buffers are libc-malloc'd, owned by queue.
        unsafe {
            free(e.data as *mut ::core::ffi::c_void);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_close(que: *mut ::core::ffi::c_void) {
    // SAFETY: handle from queue_new.
    unsafe { &*(que as *const Queue) }.close();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_isempty(que: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    // SAFETY: handle from queue_new.
    unsafe { &*(que as *const Queue) }.is_empty() as ::core::ffi::c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_elements(que: *mut ::core::ffi::c_void) -> uint32_t {
    // SAFETY: handle from queue_new.
    unsafe { &*(que as *const Queue) }.elements()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_isfull(que: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    // SAFETY: handle from queue_new.
    unsafe { &*(que as *const Queue) }.is_full() as ::core::ffi::c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_sizeleft(que: *mut ::core::ffi::c_void) -> uint32_t {
    // SAFETY: handle from queue_new.
    unsafe { &*(que as *const Queue) }.size_left()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_put(
    que: *mut ::core::ffi::c_void,
    id: uint32_t,
    op: uint32_t,
    data: *mut uint8_t,
    leng: uint32_t,
) -> ::core::ffi::c_int {
    // SAFETY: handle from queue_new; data is libc-malloc'd, ownership
    // transferred per C contract.
    let q = unsafe { &*(que as *const Queue) };
    let e = QEntry { id, op, data, leng };
    // C frees only the qentry wrapper on error — the DATA stays owned by
    // the caller (shutdown cleanup paths may still hold it → freeing here
    // would double-free). My Queue has no wrapper, so: no free at all.
    match q.put(e) {
        Ok(()) => 0,
        Err(PutError::TooLarge) => unsafe {
            set_errno(EDEADLK);
            -1
        },
        Err(PutError::Closed) => unsafe {
            set_errno(EIO);
            -1
        },
        Err(PutError::Busy) => unreachable!(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_tryput(
    que: *mut ::core::ffi::c_void,
    id: uint32_t,
    op: uint32_t,
    data: *mut uint8_t,
    leng: uint32_t,
) -> ::core::ffi::c_int {
    // SAFETY: as queue_put.
    let q = unsafe { &*(que as *const Queue) };
    let e = QEntry { id, op, data, leng };
    match q.tryput(e) {
        Ok(()) => 0,
        Err(PutError::TooLarge) => unsafe {
            set_errno(EDEADLK);
            -1
        },
        Err(PutError::Busy) => unsafe {
            set_errno(EBUSY);
            -1
        },
        Err(PutError::Closed) => unreachable!(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_get(
    que: *mut ::core::ffi::c_void,
    id: *mut uint32_t,
    op: *mut uint32_t,
    data: *mut *mut uint8_t,
    leng: *mut uint32_t,
) -> ::core::ffi::c_int {
    // SAFETY: handle from queue_new; out params per C contract.
    let q = unsafe { &*(que as *const Queue) };
    match q.get() {
        Ok(e) => unsafe {
            if !id.is_null() {
                *id = e.id;
            }
            if !op.is_null() {
                *op = e.op;
            }
            if !data.is_null() {
                *data = e.data;
            }
            if !leng.is_null() {
                *leng = e.leng;
            }
            0
        },
        Err(()) => unsafe {
            if !id.is_null() {
                *id = 0;
            }
            if !op.is_null() {
                *op = 0;
            }
            if !data.is_null() {
                *data = ::core::ptr::null_mut();
            }
            if !leng.is_null() {
                *leng = 0;
            }
            set_errno(EIO);
            -1
        },
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn queue_tryget(
    que: *mut ::core::ffi::c_void,
    id: *mut uint32_t,
    op: *mut uint32_t,
    data: *mut *mut uint8_t,
    leng: *mut uint32_t,
) -> ::core::ffi::c_int {
    // SAFETY: handle from queue_new; out params per C contract.
    let q = unsafe { &*(que as *const Queue) };
    match q.tryget() {
        Ok(e) => unsafe {
            if !id.is_null() {
                *id = e.id;
            }
            if !op.is_null() {
                *op = e.op;
            }
            if !data.is_null() {
                *data = e.data;
            }
            if !leng.is_null() {
                *leng = e.leng;
            }
            0
        },
        Err(()) => unsafe {
            if !id.is_null() {
                *id = 0;
            }
            if !op.is_null() {
                *op = 0;
            }
            if !data.is_null() {
                *data = ::core::ptr::null_mut();
            }
            if !leng.is_null() {
                *leng = 0;
            }
            set_errno(EBUSY);
            -1
        },
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;
    use std::vec::Vec;

    fn e(id: u32, leng: u32) -> QEntry {
        QEntry {
            id,
            op: id + 1000,
            data: ::core::ptr::null_mut(), // never dereferenced by queue
            leng,
        }
    }

    #[test]
    fn fifo_order_and_counters() {
        let q = Queue::new(0); // unbounded
        q.put(e(1, 10)).unwrap();
        q.put(e(2, 20)).unwrap();
        assert_eq!(q.elements(), 2);
        assert_eq!(q.size_left(), u32::MAX);
        assert!(!q.is_full());
        let a = q.get().unwrap();
        assert_eq!((a.id, a.op, a.leng), (1, 1001, 10));
        let b = q.get().unwrap();
        assert_eq!(b.id, 2);
        assert!(q.is_empty());
        assert!(q.tryget().is_err());
    }

    #[test]
    fn bounded_tryput_and_toolarge() {
        let q = Queue::new(100);
        q.put(e(1, 90)).unwrap();
        assert!(matches!(q.tryput(e(2, 20)), Err(PutError::Busy)));
        assert!(matches!(q.tryput(e(3, 101)), Err(PutError::TooLarge)));
        assert!(q.is_full() == false); // 90 < 100
        q.put(e(4, 10)).unwrap();
        assert!(q.is_full());
        assert_eq!(q.size_left(), 0);
    }

    #[test]
    fn close_unblocks_waiting_put() {
        let q = std::sync::Arc::new(Queue::new(10));
        q.put(e(1, 10)).unwrap(); // full
        let q2 = q.clone();
        let t = std::thread::spawn(move || q2.put(e(2, 10)));
        portable_sleep_ms(100); // let it block on waitfull
        q.close();
        assert_eq!(t.join().unwrap(), Err(PutError::Closed));
        // get after close → EIO even with elements present (C behavior)
        assert!(q.get().is_err());
        // put after close fails immediately
        assert!(matches!(q.put(e(9, 1)), Err(PutError::Closed)));
    }

    #[test]
    fn close_unblocks_waiting_get() {
        let q = std::sync::Arc::new(Queue::new(0));
        let q2 = q.clone();
        let g = std::thread::spawn(move || q2.get());
        portable_sleep_ms(100); // let it block on waitfree
        q.close();
        assert!(g.join().unwrap().is_err());
    }

    #[test]
    fn drain_returns_leftovers() {
        let q = Queue::new(0);
        q.put(e(1, 5)).unwrap();
        q.put(e(2, 5)).unwrap();
        let v: Vec<u32> = q.drain().into_iter().map(|e| e.id).collect();
        assert_eq!(v, std::vec![1, 2]);
        assert!(q.is_empty());
    }

    fn portable_sleep_ms(ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}
