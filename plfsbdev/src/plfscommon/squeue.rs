//! Typed Rust port of mfscommon/squeue.c (count-bounded blocking FIFO queue).
//!
//! C semantics preserved exactly (see /tmp/moosefs-ref/mfscommon/squeue.c):
//! - `new(0)` is UNBOUNDED: `put` never blocks and never checks `closed`
//!   (put-after-close on an unbounded queue succeeds, as in C).
//! - Bounded `put` blocks while full; after `close` it returns the value to
//!   the caller (`Err(elem)`), matching C's -1/EIO where the caller keeps data.
//! - `get` after `close` returns `None` even if entries are still queued.
//! - `close` wakes all waiters (get and put).
//! - Dropping the queue drops (frees) any queued elements, like squeue_delete.

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

struct State<T> {
    entries: VecDeque<T>,
    maxelements: u32,
    closed: bool,
}

pub struct SQueue<T> {
    state: Mutex<State<T>>,
    waitfree: Condvar,
    waitfull: Condvar,
}

impl<T> SQueue<T> {
    /// `maxelements == 0` means unbounded (matches squeue_new(0)).
    pub fn new(maxelements: u32) -> Self {
        Self {
            state: Mutex::new(State {
                entries: VecDeque::new(),
                maxelements,
                closed: false,
            }),
            waitfree: Condvar::new(),
            waitfull: Condvar::new(),
        }
    }

    /// Push one element. Bounded queue blocks while full; if closed while
    /// bounded, returns the value back to the caller (C: -1/EIO, caller frees).
    pub fn put(&self, elem: T) -> Result<(), T> {
        let mut state = self.state.lock().unwrap();
        if state.maxelements != 0 {
            while state.entries.len() as u32 >= state.maxelements && !state.closed {
                state = self.waitfull.wait(state).unwrap();
            }
            if state.closed {
                return Err(elem);
            }
        }
        state.entries.push_back(elem);
        self.waitfree.notify_one();
        Ok(())
    }

    /// Pop one element, blocking while empty. Returns `None` once closed,
    /// even if entries remain queued (C: *data=NULL, -1/EIO).
    pub fn get(&self) -> Option<T> {
        let mut state = self.state.lock().unwrap();
        while state.entries.is_empty() && !state.closed {
            state = self.waitfree.wait(state).unwrap();
        }
        if state.closed {
            return None;
        }
        let elem = state.entries.pop_front();
        if state.maxelements != 0 {
            self.waitfull.notify_one();
        }
        elem
    }

    pub fn close(&self) {
        let mut state = self.state.lock().unwrap();
        state.closed = true;
        self.waitfree.notify_all();
        self.waitfull.notify_all();
    }
}

/// Owns a libc-malloc'd allocation without interpreting its contents.
///
/// Consumers (worker jobs, NBD answer requests) are `malloc`'d — the NBD
/// request in particular is variable-size (`malloc(36 + length)`), so a
/// `Box` conversion is not possible. Drop therefore stays `libc::free`.
pub struct MallocPtr<T>(*mut T);

impl<T> MallocPtr<T> {
    /// # Safety
    ///
    /// `ptr` must be a non-null, uniquely owned libc-malloc allocation.
    /// No other owner may free it until ownership is returned by
    /// [`Self::into_raw`].
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        assert!(!ptr.is_null(), "queued element pointer must not be null");
        Self(ptr)
    }

    pub fn into_raw(self) -> *mut T {
        let ptr = self.0;
        std::mem::forget(self);
        ptr
    }
}

impl<T> Drop for MallocPtr<T> {
    fn drop(&mut self) {
        // SAFETY: self.0 is a uniquely owned libc-malloc allocation per from_raw.
        unsafe { libc::free(self.0 as *mut ::core::ffi::c_void) };
    }
}

// SAFETY: wrapper transfers unique allocation ownership between threads and never
// dereferences T. Consumer must uphold T's synchronization protocol after into_raw.
unsafe impl<T> Send for MallocPtr<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    #[test]
    fn fifo() {
        let q = SQueue::new(0);
        q.put(1).unwrap();
        q.put(2).unwrap();
        assert_eq!(q.get(), Some(1));
        assert_eq!(q.get(), Some(2));
    }

    #[test]
    fn unbounded_never_blocks() {
        let q = SQueue::new(0);
        for i in 0..1000 {
            q.put(i).unwrap();
        }
        for i in 0..1000 {
            assert_eq!(q.get(), Some(i));
        }
    }

    #[test]
    fn bounded_put_blocks_when_full_and_get_unblocks() {
        let q = Arc::new(SQueue::new(1));
        q.put(1).unwrap();
        let putter = {
            let q = q.clone();
            std::thread::spawn(move || q.put(2))
        };
        std::thread::sleep(Duration::from_millis(20));
        assert!(!putter.is_finished());
        assert_eq!(q.get(), Some(1));
        assert_eq!(putter.join().unwrap(), Ok(()));
        assert_eq!(q.get(), Some(2));
    }

    #[test]
    fn close_wakes_blocked_getter() {
        let q = Arc::new(SQueue::<u32>::new(1));
        let getter = {
            let q = q.clone();
            std::thread::spawn(move || q.get())
        };
        std::thread::sleep(Duration::from_millis(20));
        q.close();
        assert_eq!(getter.join().unwrap(), None);
    }

    #[test]
    fn close_wakes_full_putter_with_value_returned() {
        let q = Arc::new(SQueue::new(1));
        q.put(1).unwrap();
        let putter = {
            let q = q.clone();
            std::thread::spawn(move || q.put(2))
        };
        std::thread::sleep(Duration::from_millis(20));
        assert!(!putter.is_finished());
        q.close();
        // C: put after close on bounded queue returns -1, caller keeps data.
        assert_eq!(putter.join().unwrap(), Err(2));
    }

    #[test]
    fn get_after_close_returns_none_even_with_queued_entries() {
        let q = SQueue::new(0);
        q.put(7).unwrap();
        q.close();
        assert_eq!(q.get(), None);
    }

    #[test]
    fn put_after_close_unbounded_succeeds() {
        // C squeue_put only checks `closed` when maxelements != 0.
        let q = SQueue::new(0);
        q.close();
        assert_eq!(q.put(7), Ok(()));
    }

    #[test]
    fn drop_frees_queued_elements() {
        struct CountDrop(Arc<AtomicUsize>);
        impl Drop for CountDrop {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let drops = Arc::new(AtomicUsize::new(0));
        let q = SQueue::new(0);
        q.put(CountDrop(drops.clone())).ok().unwrap();
        q.put(CountDrop(drops.clone())).ok().unwrap();
        assert_eq!(drops.load(Ordering::SeqCst), 0);
        drop(q);
        assert_eq!(drops.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn malloc_ptr_roundtrip_and_drop() {
        // SAFETY: p is a fresh libc malloc allocation, uniquely owned.
        unsafe {
            let p = libc::malloc(64) as *mut u8;
            assert!(!p.is_null());
            let ptr = MallocPtr::from_raw(p);
            let raw = ptr.into_raw();
            assert_eq!(raw, p);
            let ptr = MallocPtr::from_raw(raw);
            drop(ptr); // frees via libc::free
        }
    }
}
