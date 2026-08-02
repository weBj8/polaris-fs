//! Chunk metadata cache shared by read, write, and master-update paths.
//!
//! Native Rust state replaces two malloc'd intrusive hash tables and their
//! pthread mutex. Callers exchange owned snapshots and byte slices.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedChunk {
    pub chunkid: u64,
    pub version: u32,
    pub csdataver: u8,
    pub csdata: Vec<u8>,
}

static CACHE: LazyLock<Mutex<HashMap<(u32, u32), CachedChunk>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn init() {
    CACHE.lock().unwrap().clear();
}

pub fn term() {
    cleanup();
}

pub fn cleanup() {
    CACHE.lock().unwrap().clear();
}

pub fn clear_inode(inode: u32, first_chunk: u32) {
    CACHE
        .lock()
        .unwrap()
        .retain(|&(entry_inode, chunk), _| entry_inode != inode || chunk < first_chunk);
}

pub fn invalidate(inode: u32, chunk: u32) {
    CACHE.lock().unwrap().remove(&(inode, chunk));
}

pub fn check(inode: u32, chunk: u32, chunkid: u64, version: u32) -> bool {
    CACHE
        .lock()
        .unwrap()
        .get(&(inode, chunk))
        .is_some_and(|entry| entry.chunkid == chunkid && entry.version == version)
}

pub fn change(inode: u32, chunk: u32, chunkid: u64, version: u32) {
    if let Some(entry) = CACHE.lock().unwrap().get_mut(&(inode, chunk)) {
        entry.chunkid = chunkid;
        entry.version = version;
    }
}

pub fn insert(inode: u32, chunk: u32, chunkid: u64, version: u32, csdataver: u8, csdata: &[u8]) {
    CACHE.lock().unwrap().insert(
        (inode, chunk),
        CachedChunk {
            chunkid,
            version,
            csdataver,
            csdata: csdata.to_vec(),
        },
    );
}

pub fn find(inode: u32, chunk: u32) -> Option<CachedChunk> {
    CACHE.lock().unwrap().get(&(inode, chunk)).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn insert_find_change_invalidate() {
        let _guard = TEST_LOCK.lock().unwrap();
        init();
        insert(1, 2, 10, 3, 4, b"servers");
        assert_eq!(
            find(1, 2),
            Some(CachedChunk {
                chunkid: 10,
                version: 3,
                csdataver: 4,
                csdata: b"servers".to_vec(),
            })
        );
        assert!(check(1, 2, 10, 3));
        change(1, 2, 11, 5);
        assert!(check(1, 2, 11, 5));
        invalidate(1, 2);
        assert!(find(1, 2).is_none());
    }

    #[test]
    fn clear_inode_keeps_lower_chunks_and_other_inodes() {
        let _guard = TEST_LOCK.lock().unwrap();
        init();
        for chunk in 0..4 {
            insert(1, chunk, chunk as u64, 1, 0, &[]);
            insert(2, chunk, chunk as u64, 1, 0, &[]);
        }
        clear_inode(1, 2);
        assert!(find(1, 0).is_some());
        assert!(find(1, 1).is_some());
        assert!(find(1, 2).is_none());
        assert!(find(1, 3).is_none());
        assert!(find(2, 3).is_some());
        cleanup();
        assert!(find(2, 3).is_none());
    }
}
