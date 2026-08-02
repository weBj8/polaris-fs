use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock, Weak};

#[derive(Default)]
struct IoState {
    writing: bool,
    readers: u32,
    waiting_writers: u32,
}

pub struct InodeLength {
    inode: u32,
    length: AtomicU64,
    io: Mutex<IoState>,
    io_changed: Condvar,
}

pub type Handle = Arc<InodeLength>;

fn registry() -> &'static Mutex<HashMap<u32, Weak<InodeLength>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<u32, Weak<InodeLength>>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn acquire(inode: u32) -> Handle {
    let mut registry = registry().lock().unwrap();
    if let Some(handle) = registry.get(&inode).and_then(Weak::upgrade) {
        return handle;
    }

    let handle = Arc::new(InodeLength {
        inode,
        length: AtomicU64::new(0),
        io: Mutex::new(IoState::default()),
        io_changed: Condvar::new(),
    });
    registry.insert(inode, Arc::downgrade(&handle));
    handle
}

pub fn get_length(handle: &Handle) -> u64 {
    handle.length.load(Ordering::Relaxed)
}

pub fn set_length(handle: &Handle, length: u64) {
    handle.length.store(length, Ordering::Relaxed);
}

pub fn update_length(inode: u32, length: u64) {
    let mut registry = registry().lock().unwrap();
    match registry.get(&inode).and_then(Weak::upgrade) {
        Some(handle) => handle.length.store(length, Ordering::Relaxed),
        None => {
            registry.remove(&inode);
        }
    }
}

pub fn write_start(handle: &Handle) {
    let mut io = handle.io.lock().unwrap();
    io.waiting_writers = io.waiting_writers.wrapping_add(1);
    while io.readers != 0 || io.writing {
        io = handle.io_changed.wait(io).unwrap();
    }
    io.waiting_writers = io.waiting_writers.wrapping_sub(1);
    io.writing = true;
}

pub fn write_end(handle: &Handle) {
    let mut io = handle.io.lock().unwrap();
    io.writing = false;
    handle.io_changed.notify_all();
}

pub fn read_start(handle: &Handle) {
    let mut io = handle.io.lock().unwrap();
    while io.writing || io.waiting_writers != 0 {
        io = handle.io_changed.wait(io).unwrap();
    }
    io.readers = io.readers.wrapping_add(1);
}

pub fn read_end(handle: &Handle) {
    let mut io = handle.io.lock().unwrap();
    io.readers = io.readers.wrapping_sub(1);
    if io.readers == 0 {
        handle.io_changed.notify_all();
    }
}

pub fn io_wait(handle: &Handle) {
    let mut io = handle.io.lock().unwrap();
    while io.readers != 0 || io.waiting_writers != 0 || io.writing {
        io = handle.io_changed.wait(io).unwrap();
    }
}

pub fn init() {
    registry().lock().unwrap().clear();
}

pub fn term() {
    let mut registry = registry().lock().unwrap();
    for handle in registry.values() {
        let refs = handle.strong_count();
        if refs != 0 {
            if let Some(handle) = handle.upgrade() {
                unsafe {
                    plfscommon::mfslog::mfs_log(
                        0,
                        3,
                        c"inode fleng data structure leftovers (ino: %u ; refcnt: %u)".as_ptr(),
                        handle.inode,
                        refs as u32,
                    );
                }
            }
        }
    }
    registry.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn same_inode_shares_length() {
        let _test = TEST_LOCK.lock().unwrap();
        init();
        let first = acquire(17);
        let second = acquire(17);
        set_length(&first, 123);
        assert_eq!(get_length(&second), 123);
        update_length(17, 456);
        assert_eq!(get_length(&first), 456);
    }

    #[test]
    fn waiting_writer_blocks_new_readers() {
        let _test = TEST_LOCK.lock().unwrap();
        init();
        let handle = acquire(23);
        read_start(&handle);

        let (events_tx, events_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let writer = {
            let handle = Arc::clone(&handle);
            let events_tx = events_tx.clone();
            thread::spawn(move || {
                write_start(&handle);
                events_tx.send("writer").unwrap();
                release_rx.recv().unwrap();
                write_end(&handle);
            })
        };

        while handle.io.lock().unwrap().waiting_writers == 0 {
            thread::yield_now();
        }

        let reader = {
            let handle = Arc::clone(&handle);
            thread::spawn(move || {
                read_start(&handle);
                events_tx.send("reader").unwrap();
                read_end(&handle);
            })
        };

        assert!(events_rx.recv_timeout(Duration::from_millis(20)).is_err());
        read_end(&handle);
        assert_eq!(
            events_rx.recv_timeout(Duration::from_secs(1)).unwrap(),
            "writer"
        );
        release_tx.send(()).unwrap();
        assert_eq!(
            events_rx.recv_timeout(Duration::from_secs(1)).unwrap(),
            "reader"
        );
        writer.join().unwrap();
        reader.join().unwrap();
    }
}
