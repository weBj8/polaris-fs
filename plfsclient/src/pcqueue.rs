use std::collections::VecDeque;
use std::ptr::NonNull;
use std::sync::{Arc, Condvar, Mutex};

struct State<T> {
    jobs: VecDeque<T>,
    closed: bool,
}

pub struct JobQueue<T> {
    state: Mutex<State<T>>,
    ready: Condvar,
}

impl<T> JobQueue<T> {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(State {
                jobs: VecDeque::new(),
                closed: false,
            }),
            ready: Condvar::new(),
        }
    }

    pub fn put(&self, job: T) -> Result<(), T> {
        let mut state = self.state.lock().unwrap();
        if state.closed {
            return Err(job);
        }
        state.jobs.push_back(job);
        self.ready.notify_one();
        Ok(())
    }

    pub fn get(&self) -> Option<T> {
        let mut state = self.state.lock().unwrap();
        while state.jobs.is_empty() && !state.closed {
            state = self.ready.wait(state).unwrap();
        }
        if state.closed {
            None
        } else {
            state.jobs.pop_front()
        }
    }

    pub fn close(&self) {
        let mut state = self.state.lock().unwrap();
        state.closed = true;
        self.ready.notify_all();
    }
}

impl<T> Default for JobQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Owns one allocation created by libc `malloc` without interpreting its contents.
pub struct OwnedJob<T> {
    ptr: NonNull<T>,
}

impl<T> OwnedJob<T> {
    /// # Safety
    ///
    /// `ptr` must be a non-null, uniquely owned allocation from libc `malloc`.
    /// No other owner may free it until ownership is returned by [`Self::into_raw`].
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("queued job pointer must not be null"),
        }
    }

    pub fn into_raw(self) -> *mut T {
        let ptr = self.ptr.as_ptr();
        std::mem::forget(self);
        ptr
    }
}

// SAFETY: wrapper transfers unique allocation ownership between threads and never
// dereferences T. Consumer must uphold T's synchronization protocol after into_raw.
unsafe impl<T> Send for OwnedJob<T> {}

impl<T> Drop for OwnedJob<T> {
    fn drop(&mut self) {
        // SAFETY: from_raw requires a uniquely owned libc allocation.
        unsafe { libc::free(self.ptr.as_ptr().cast()) };
    }
}

pub(crate) struct QueueSlot<T> {
    queue: Mutex<Option<Arc<JobQueue<T>>>>,
}

impl<T> QueueSlot<T> {
    pub const fn new() -> Self {
        Self {
            queue: Mutex::new(None),
        }
    }

    pub fn init(&self) {
        if let Some(old) = self
            .queue
            .lock()
            .unwrap()
            .replace(Arc::new(JobQueue::new()))
        {
            old.close();
        }
    }

    pub fn get(&self) -> Option<Arc<JobQueue<T>>> {
        self.queue.lock().unwrap().clone()
    }

    pub fn close(&self) {
        if let Some(queue) = self.get() {
            queue.close();
        }
    }

    pub fn delete(&self) {
        self.queue.lock().unwrap().take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn fifo() {
        let queue = JobQueue::new();
        queue.put(1).unwrap();
        queue.put(2).unwrap();
        assert_eq!(queue.get(), Some(1));
        assert_eq!(queue.get(), Some(2));
    }

    #[test]
    fn close_wakes_waiter_and_owns_queued_values() {
        struct CountDrop(Arc<AtomicUsize>);
        impl Drop for CountDrop {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }

        let queued_drops = Arc::new(AtomicUsize::new(0));
        let queue = Arc::new(JobQueue::new());
        queue.put(CountDrop(queued_drops.clone())).ok().unwrap();
        queue.close();
        assert!(queue.get().is_none());
        assert_eq!(queued_drops.load(Ordering::SeqCst), 0);
        drop(queue);
        assert_eq!(queued_drops.load(Ordering::SeqCst), 1);

        let queue = Arc::new(JobQueue::<()>::new());
        let waiter = {
            let queue = queue.clone();
            std::thread::spawn(move || queue.get())
        };
        std::thread::sleep(std::time::Duration::from_millis(20));
        queue.close();
        assert_eq!(waiter.join().unwrap(), None);
    }

    #[test]
    fn put_after_close_returns_value() {
        let queue = JobQueue::new();
        queue.close();
        assert_eq!(queue.put(7), Err(7));
    }

    #[test]
    fn slot_can_be_reinitialized() {
        let slot = QueueSlot::new();
        slot.init();
        let first = slot.get().unwrap();
        slot.init();
        assert!(first.get().is_none());
        let second = slot.get().unwrap();
        second.put(3).unwrap();
        assert_eq!(second.get(), Some(3));
        slot.close();
        slot.delete();
        assert!(slot.get().is_none());
    }
}
