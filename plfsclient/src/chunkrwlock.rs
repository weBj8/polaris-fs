//! Per-(inode, chunk) reader/writer coordination.
//!
//! Native Rust state replaces malloc'd records, free lists, pthread mutexes,
//! and per-record condition variables. Writer preference matches MooseFS:
//! readers wait while any writer is queued.

use std::collections::HashMap;
use std::sync::{Condvar, LazyLock, Mutex};

#[derive(Default)]
struct LockState {
    writing: bool,
    active_readers: u32,
    waiting_readers: u32,
    waiting_writers: u32,
}

#[derive(Default)]
struct State {
    locks: HashMap<(u32, u32), LockState>,
}

static STATE: LazyLock<(Mutex<State>, Condvar)> =
    LazyLock::new(|| (Mutex::new(State::default()), Condvar::new()));

fn unused(lock: &LockState) -> bool {
    !lock.writing
        && lock.active_readers == 0
        && lock.waiting_readers == 0
        && lock.waiting_writers == 0
}

pub fn init() {
    STATE.0.lock().unwrap().locks.clear();
}

pub fn term() {
    let state = STATE.0.lock().unwrap();
    assert!(
        state.locks.is_empty(),
        "chunkrwlock map not empty during termination"
    );
}

pub fn read_lock(inode: u32, chunk: u32) {
    let key = (inode, chunk);
    let (mutex, condvar) = &*STATE;
    let mut state = mutex.lock().unwrap();
    state.locks.entry(key).or_default().waiting_readers += 1;
    while {
        let lock = state.locks.get(&key).unwrap();
        lock.writing || lock.waiting_writers != 0
    } {
        state = condvar.wait(state).unwrap();
    }
    let lock = state.locks.get_mut(&key).unwrap();
    lock.waiting_readers -= 1;
    lock.active_readers += 1;
}

pub fn read_unlock(inode: u32, chunk: u32) {
    let key = (inode, chunk);
    let (mutex, condvar) = &*STATE;
    let mut state = mutex.lock().unwrap();
    let lock = state
        .locks
        .get_mut(&key)
        .expect("chunk read unlock without lock");
    lock.active_readers = lock
        .active_readers
        .checked_sub(1)
        .expect("chunk read lock underflow");
    let wake = lock.active_readers == 0 && lock.waiting_writers != 0;
    let remove = unused(lock);
    if remove {
        state.locks.remove(&key);
    }
    if wake {
        condvar.notify_all();
    }
}

pub fn write_lock(inode: u32, chunk: u32) {
    let key = (inode, chunk);
    let (mutex, condvar) = &*STATE;
    let mut state = mutex.lock().unwrap();
    state.locks.entry(key).or_default().waiting_writers += 1;
    while {
        let lock = state.locks.get(&key).unwrap();
        lock.active_readers != 0 || lock.writing
    } {
        state = condvar.wait(state).unwrap();
    }
    let lock = state.locks.get_mut(&key).unwrap();
    lock.waiting_writers -= 1;
    lock.writing = true;
}

pub fn write_unlock(inode: u32, chunk: u32) {
    let key = (inode, chunk);
    let (mutex, condvar) = &*STATE;
    let mut state = mutex.lock().unwrap();
    let lock = state
        .locks
        .get_mut(&key)
        .expect("chunk write unlock without lock");
    assert!(lock.writing, "chunk write unlock without writer");
    lock.writing = false;
    let wake = lock.waiting_writers != 0 || lock.waiting_readers != 0;
    let remove = unused(lock);
    if remove {
        state.locks.remove(&key);
    }
    if wake {
        condvar.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{LazyLock, Mutex, mpsc};
    use std::time::Duration;

    static TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    #[test]
    fn readers_share_and_writer_waits() {
        let _test = TEST_LOCK.lock().unwrap();
        init();
        read_lock(1, 2);
        read_lock(1, 2);
        let (tx, rx) = mpsc::channel();
        let writer = std::thread::spawn(move || {
            write_lock(1, 2);
            tx.send(()).unwrap();
            write_unlock(1, 2);
        });
        assert!(rx.recv_timeout(Duration::from_millis(20)).is_err());
        read_unlock(1, 2);
        assert!(rx.recv_timeout(Duration::from_millis(20)).is_err());
        read_unlock(1, 2);
        rx.recv_timeout(Duration::from_secs(1)).unwrap();
        writer.join().unwrap();
        term();
    }

    #[test]
    fn independent_chunks_do_not_block() {
        let _test = TEST_LOCK.lock().unwrap();
        init();
        write_lock(1, 1);
        let (tx, rx) = mpsc::channel();
        let reader = std::thread::spawn(move || {
            read_lock(1, 2);
            tx.send(()).unwrap();
            read_unlock(1, 2);
        });
        rx.recv_timeout(Duration::from_secs(1)).unwrap();
        write_unlock(1, 1);
        reader.join().unwrap();
        term();
    }
}
