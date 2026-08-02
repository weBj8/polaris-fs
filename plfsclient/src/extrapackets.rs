//! Asynchronous master invalidation events.
//!
//! Native Rust queue and worker replace malloc'd packet/free lists, pthread
//! synchronization, and internal C ABI calls.

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::thread::JoinHandle;

const MFS_CHUNK_SIZE: u64 = 0x4000000;

unsafe extern "C" {
    unsafe fn read_inode_clear_cache(inode: u32, offset: u64, length: u64);
    unsafe fn read_inode_set_length_passive(inode: u32, newlength: u64);
}

enum Event {
    ChunkChanged {
        inode: u32,
        chunk: u32,
        chunkid: u64,
        version: u32,
        file_length: u64,
        truncate: bool,
        offset: u32,
        size: u32,
    },
    FileLengthChanged {
        inode: u32,
        file_length: u64,
    },
    Exit,
}

static QUEUE: Mutex<VecDeque<Event>> = Mutex::new(VecDeque::new());
static READY: Condvar = Condvar::new();
static WORKER: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);

fn push(event: Event) {
    QUEUE.lock().unwrap().push_back(event);
    READY.notify_one();
}

fn process(event: Event) -> bool {
    match event {
        Event::ChunkChanged {
            inode,
            chunk,
            chunkid,
            version,
            file_length,
            truncate,
            offset,
            size,
        } => {
            crate::chunksdatacache::change(inode, chunk, chunkid, version);
            let start = (chunk as u64)
                .wrapping_mul(MFS_CHUNK_SIZE)
                .wrapping_add(offset as u64);
            if truncate {
                crate::chunksdatacache::clear_inode(inode, chunk.wrapping_add(1));
                // SAFETY: internal read-cache notification accepts value types.
                unsafe {
                    read_inode_clear_cache(inode, start, 0);
                    read_inode_set_length_passive(inode, file_length);
                }
            } else if size > 0 {
                // SAFETY: internal read-cache notification accepts value types.
                unsafe { read_inode_clear_cache(inode, start, size as u64) };
            }
            true
        }
        Event::FileLengthChanged { inode, file_length } => {
            // SAFETY: internal read-cache notification accepts value types.
            unsafe { read_inode_set_length_passive(inode, file_length) };
            true
        }
        Event::Exit => false,
    }
}

fn worker() {
    loop {
        let event = {
            let mut queue = QUEUE.lock().unwrap();
            while queue.is_empty() {
                queue = READY.wait(queue).unwrap();
            }
            queue.pop_front().unwrap()
        };
        if !process(event) {
            return;
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn chunk_changed(
    inode: u32,
    chunk: u32,
    chunkid: u64,
    version: u32,
    file_length: u64,
    truncate: bool,
    offset: u32,
    size: u32,
) {
    push(Event::ChunkChanged {
        inode,
        chunk,
        chunkid,
        version,
        file_length,
        truncate,
        offset,
        size,
    });
}

pub fn file_length_changed(inode: u32, file_length: u64) {
    push(Event::FileLengthChanged { inode, file_length });
}

pub fn init() {
    QUEUE.lock().unwrap().clear();
    let worker = plfscommon::lwthread::spawn_min("extra-packets", worker)
        .unwrap_or_else(|_| std::process::abort());
    *WORKER.lock().unwrap() = Some(worker);
}

pub fn term() {
    push(Event::Exit);
    if let Some(worker) = WORKER.lock().unwrap().take() {
        worker.join().expect("extra-packets worker panicked");
    }
    QUEUE.lock().unwrap().clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_is_fifo() {
        let mut queue = QUEUE.lock().unwrap();
        queue.clear();
        queue.push_back(Event::FileLengthChanged {
            inode: 1,
            file_length: 10,
        });
        queue.push_back(Event::Exit);
        assert!(matches!(
            queue.pop_front(),
            Some(Event::FileLengthChanged { inode: 1, .. })
        ));
        assert!(matches!(queue.pop_front(), Some(Event::Exit)));
    }
}
