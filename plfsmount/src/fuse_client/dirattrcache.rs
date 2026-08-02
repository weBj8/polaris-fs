//! Directory attribute cache.
//!
//! Readdir blobs are copied into cache-owned memory. Callers may free their
//! buffers immediately after `append`; all cache handles and registry entries
//! use `Arc`/`Weak` ownership.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};

pub const ATTR_RECORD_SIZE: usize = 36;

#[derive(Clone, Copy)]
struct Location {
    blob: usize,
    offset: usize,
}

#[derive(Default)]
struct Inner {
    blobs: Vec<Vec<u8>>,
    names: HashMap<Vec<u8>, Location>,
    inodes: HashMap<u32, Location>,
}

pub struct DirCache {
    pid: i32,
    uid: u32,
    gid: u32,
    parent: u32,
    attrsize: usize,
    inner: Mutex<Inner>,
}

pub type Handle = Arc<DirCache>;

static REGISTRY: Mutex<Vec<Weak<DirCache>>> = Mutex::new(Vec::new());

fn live_caches() -> Vec<Handle> {
    let mut registry = REGISTRY.lock().unwrap();
    let caches = registry.iter().filter_map(Weak::upgrade).collect();
    registry.retain(|cache| cache.strong_count() != 0);
    caches
}

fn entry_parts(blob: &[u8], offset: usize, attrsize: usize) -> Option<(&[u8], u32, usize)> {
    let name_len = *blob.get(offset)? as usize;
    let name_start = offset + 1;
    let inode_start = name_start.checked_add(name_len)?;
    let attr_start = inode_start.checked_add(4)?;
    let end = attr_start.checked_add(attrsize)?;
    if end > blob.len() {
        return None;
    }
    let inode = u32::from_be_bytes(blob[inode_start..attr_start].try_into().unwrap());
    Some((&blob[name_start..inode_start], inode, attr_start))
}

fn fixed_attr(blob: &[u8], attr_start: usize, attrsize: usize) -> [u8; ATTR_RECORD_SIZE] {
    let mut attr = [0; ATTR_RECORD_SIZE];
    let count = attrsize.min(ATTR_RECORD_SIZE);
    attr[..count].copy_from_slice(&blob[attr_start..attr_start + count]);
    attr
}

pub fn new(pid: i32, uid: u32, gid: u32, parent: u32, attrsize: u8) -> Handle {
    let cache = Arc::new(DirCache {
        pid,
        uid,
        gid,
        parent,
        attrsize: attrsize as usize,
        inner: Mutex::new(Inner::default()),
    });
    REGISTRY.lock().unwrap().push(Arc::downgrade(&cache));
    cache
}

pub fn append(cache: &Handle, blob: &[u8]) {
    let mut inner = cache.inner.lock().unwrap();
    let blob_index = inner.blobs.len();
    inner.blobs.push(blob.to_vec());

    let mut offset = 0;
    while let Some((name, inode, attr_start)) =
        entry_parts(&inner.blobs[blob_index], offset, cache.attrsize)
    {
        let location = Location {
            blob: blob_index,
            offset,
        };
        let name = name.to_vec();
        inner.names.entry(name).or_insert(location);
        if inode != 0 {
            inner.inodes.entry(inode).or_insert(location);
        }
        offset = attr_start + cache.attrsize;
    }
}

pub fn lookup(
    pid: i32,
    uid: u32,
    gid: u32,
    parent: u32,
    name: &[u8],
) -> Option<(u32, [u8; ATTR_RECORD_SIZE])> {
    for cache in live_caches() {
        if cache.pid != pid || cache.uid != uid || cache.gid != gid || cache.parent != parent {
            continue;
        }
        let inner = cache.inner.lock().unwrap();
        let Some(location) = inner.names.get(name).copied() else {
            continue;
        };
        let blob = &inner.blobs[location.blob];
        let Some((_, inode, attr_start)) = entry_parts(blob, location.offset, cache.attrsize)
        else {
            continue;
        };
        if blob[attr_start] != 0 {
            return Some((inode, fixed_attr(blob, attr_start, cache.attrsize)));
        }
    }
    None
}

pub fn getattr(pid: i32, uid: u32, gid: u32, inode: u32) -> Option<[u8; ATTR_RECORD_SIZE]> {
    for cache in live_caches() {
        if cache.pid != pid || cache.uid != uid || cache.gid != gid {
            continue;
        }
        let inner = cache.inner.lock().unwrap();
        let Some(location) = inner.inodes.get(&inode).copied() else {
            continue;
        };
        let blob = &inner.blobs[location.blob];
        let Some((_, _, attr_start)) = entry_parts(blob, location.offset, cache.attrsize) else {
            continue;
        };
        if blob[attr_start] != 0 {
            return Some(fixed_attr(blob, attr_start, cache.attrsize));
        }
    }
    None
}

pub fn setattr(inode: u32, attr: &[u8; ATTR_RECORD_SIZE]) {
    for cache in live_caches() {
        let mut inner = cache.inner.lock().unwrap();
        let Some(location) = inner.inodes.get(&inode).copied() else {
            continue;
        };
        let attrsize = cache.attrsize;
        let blob = &mut inner.blobs[location.blob];
        let Some((_, _, attr_start)) = entry_parts(blob, location.offset, attrsize) else {
            continue;
        };
        let count = attrsize.min(ATTR_RECORD_SIZE);
        blob[attr_start..attr_start + count].copy_from_slice(&attr[..count]);
    }
}

pub fn invalidate_attr(inode: u32) {
    for cache in live_caches() {
        let mut inner = cache.inner.lock().unwrap();
        let Some(location) = inner.inodes.get(&inode).copied() else {
            continue;
        };
        let attrsize = cache.attrsize;
        let blob = &mut inner.blobs[location.blob];
        let Some((_, _, attr_start)) = entry_parts(blob, location.offset, attrsize) else {
            continue;
        };
        blob[attr_start..attr_start + attrsize].fill(0);
    }
}

pub fn invalidate_name(parent: u32, name: &[u8]) {
    for cache in live_caches() {
        if cache.parent != parent {
            continue;
        }
        let mut inner = cache.inner.lock().unwrap();
        let Some(location) = inner.names.get(name).copied() else {
            continue;
        };
        let attrsize = cache.attrsize;
        let blob = &mut inner.blobs[location.blob];
        let Some((_, inode, attr_start)) = entry_parts(blob, location.offset, attrsize) else {
            continue;
        };
        let inode_start = attr_start - 4;
        blob[inode_start..attr_start + attrsize].fill(0);
        inner.inodes.remove(&inode);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blob(entries: &[(&[u8], u32, u8)], attrsize: u8) -> Vec<u8> {
        let mut blob = Vec::new();
        for (name, inode, attr) in entries {
            blob.push(name.len() as u8);
            blob.extend_from_slice(name);
            blob.extend_from_slice(&inode.to_be_bytes());
            blob.extend(std::iter::repeat_n(*attr, attrsize as usize));
        }
        blob
    }

    #[test]
    fn cache_owns_blob_and_invalidates_entries() {
        let cache = new(1, 2, 3, 10, 8);
        let mut source = blob(&[(b"one", 11, 7), (b"two", 12, 9)], 8);
        append(&cache, &source);
        source.fill(0);

        let (inode, attr) = lookup(1, 2, 3, 10, b"one").unwrap();
        assert_eq!(inode, 11);
        assert_eq!(&attr[..8], &[7; 8]);
        assert_eq!(getattr(1, 2, 3, 12).unwrap()[0], 9);

        invalidate_attr(11);
        assert!(lookup(1, 2, 3, 10, b"one").is_none());
        invalidate_name(10, b"two");
        assert!(getattr(1, 2, 3, 12).is_none());
    }

    #[test]
    fn ignores_truncated_entry() {
        let cache = new(4, 5, 6, 20, 8);
        append(&cache, &[3, b'x']);
        assert!(lookup(4, 5, 6, 20, b"xxx").is_none());
    }
}
