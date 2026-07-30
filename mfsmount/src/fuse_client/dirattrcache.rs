//! Directory attribute cache — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfsclient/dirattrcache.c. Per-(ctx,parent) caches of
//! readdir blobs with lazily built name/node indexes (dirblob_*_index).
//! The module does NOT own the blob memory — the caller (mfs_fuse readdir
//! path) keeps it alive until dcache_release; indexes and all blob walks
//! are therefore raw-pointer operations and stay at the boundary.
//!
//! Safe core in `imp`: the blob element counter, the attr record fixup
//! (copy min(attrsize,36) + zero-fill), and the global dircache registry
//! matching. Everything that dereferences a blob pointer is boundary code
//! with the lifetime contract documented.

use std::sync::Mutex as StdMutex;

unsafe extern "C" {
    unsafe fn name_index_create(minelements: uint32_t) -> *mut ::core::ffi::c_void;
    unsafe fn name_index_destroy(vidx: *mut ::core::ffi::c_void);
    unsafe fn name_index_add(vidx: *mut ::core::ffi::c_void, ptr: *mut uint8_t);
    unsafe fn name_index_find(
        vidx: *mut ::core::ffi::c_void,
        str_: *const uint8_t,
        len: uint8_t,
    ) -> *mut uint8_t;
    unsafe fn node_index_create(minelements: uint32_t) -> *mut ::core::ffi::c_void;
    unsafe fn node_index_destroy(vidx: *mut ::core::ffi::c_void);
    unsafe fn node_index_add(vidx: *mut ::core::ffi::c_void, ptr: *mut uint8_t);
    unsafe fn node_index_find(vidx: *mut ::core::ffi::c_void, node: uint32_t) -> *mut uint8_t;
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uid_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
pub type pid_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_ctx {
    pub uid: uid_t,
    pub gid: gid_t,
    pub pid: pid_t,
    pub umask: ::core::ffi::c_uint,
}
pub const ATTR_RECORD_SIZE: usize = 36;

const NAME_INDEX_FLAG: u8 = 1;
const NODE_INDEX_FLAG: u8 = 2;

#[deny(unsafe_code)]
pub mod imp {
    /// Count complete entries in a directory blob.
    /// Entry layout: [nleng:1][name][inode:4][attr:attrsize]; the walk
    /// advances nleng+5+attrsize per step and counts entries that fit
    /// completely (exact C semantics, including the final partial entry
    /// being skipped but still terminating the walk).
    pub fn elemcount(dbuff: &[u8], attrsize: u8) -> u32 {
        let mut ret = 0u32;
        let mut pos = 0usize;
        let step_extra = 5usize + attrsize as usize;
        while pos < dbuff.len() {
            let enleng = dbuff[pos] as usize;
            if pos + enleng + step_extra <= dbuff.len() {
                ret += 1;
            }
            pos += enleng + step_extra;
        }
        ret
    }

    /// Copy an attr record of `attrsize` bytes into a 36-byte record:
    /// copy min(attrsize,36), zero-fill the rest (C memcpy/memset pair).
    pub fn fix_attr(record: &[u8], attrsize: u8) -> [u8; 36] {
        let mut out = [0u8; 36];
        let n = (attrsize as usize).min(36).min(record.len());
        out[..n].copy_from_slice(&record[..n]);
        out
    }

    /// Registry matching: does a dircache belong to (ctx, parent)?
    pub fn ctx_matches(c_pid: i32, c_uid: u32, c_gid: u32, d_pid: i32, d_uid: u32, d_gid: u32) -> bool {
        c_pid == d_pid && c_uid == d_uid && c_gid == d_gid
    }
}

// ---------------------------------------------------------------------------
// Boundary: blob memory (caller-owned) and index handles.
// ---------------------------------------------------------------------------

struct Inner {
    /// caller-owned readdir blobs; NOT freed here (C contract)
    blobs: Vec<(*mut uint8_t, uint32_t)>,
    name_index: *mut ::core::ffi::c_void,
    node_index: *mut ::core::ffi::c_void,
}

struct DirCache {
    pid: pid_t,
    uid: uid_t,
    gid: gid_t,
    parent: uint32_t,
    attrsize: u8,
    inner: StdMutex<Inner>,
}
// SAFETY: raw blob pointers are only dereferenced with the inner mutex
// held and the caller keeps blobs alive until dcache_release (mfs_fuse
// readdir flow).
unsafe impl Send for DirCache {}
unsafe impl Send for Inner {}
unsafe impl Sync for DirCache {}

/// registry: handles as usize (glock in C); lock order registry → d.lock
static REGISTRY: StdMutex<Vec<usize>> = StdMutex::new(Vec::new());

/// Walk every entry pointer of every blob (C: dcache_add_blob_to_indexes).
/// SAFETY: blobs alive per module contract; inner mutex held by caller.
unsafe fn add_blobs_to_indexes(inner: &Inner, attrsize: u8, mask: u8) {
    unsafe {
        for &(mut ptr, dsize) in &inner.blobs {
            let end = ptr.add(dsize as usize);
            while ptr < end {
                let enleng = *ptr as usize;
                if mask & NAME_INDEX_FLAG != 0 && !inner.name_index.is_null() {
                    name_index_add(inner.name_index, ptr);
                }
                if mask & NODE_INDEX_FLAG != 0 && !inner.node_index.is_null() {
                    node_index_add(inner.node_index, ptr);
                }
                ptr = ptr.add(enleng + 5 + attrsize as usize);
            }
        }
    }
}

/// SAFETY: inner mutex held; blobs alive.
unsafe fn make_name_index(inner: &mut Inner, attrsize: u8) {
    unsafe {
        let mut elemcount = 0u32;
        for &(ptr, dsize) in &inner.blobs {
            let buf = ::core::slice::from_raw_parts(ptr, dsize as usize);
            elemcount += imp::elemcount(buf, attrsize);
        }
        inner.name_index = name_index_create(elemcount);
        add_blobs_to_indexes(inner, attrsize, NAME_INDEX_FLAG);
    }
}

/// SAFETY: inner mutex held; blobs alive.
unsafe fn make_node_index(inner: &mut Inner, attrsize: u8) {
    unsafe {
        let mut elemcount = 0u32;
        for &(ptr, dsize) in &inner.blobs {
            let buf = ::core::slice::from_raw_parts(ptr, dsize as usize);
            elemcount += imp::elemcount(buf, attrsize);
        }
        inner.node_index = node_index_create(elemcount);
        add_blobs_to_indexes(inner, attrsize, NODE_INDEX_FLAG);
    }
}

/// dcache_namehash_get: find name, read inode + attr record.
/// SAFETY: d.lock held; blobs alive. Returns (inode, attr) on valid hit.
unsafe fn namehash_get(inner: &mut Inner, attrsize: u8, nleng: uint8_t, name: *const uint8_t) -> Option<(uint32_t, [u8; 36])> {
    unsafe {
        if inner.name_index.is_null() {
            make_name_index(inner, attrsize);
        }
        let ptr = name_index_find(inner.name_index, name, nleng);
        if ptr.is_null() {
            return None;
        }
        let rptr = ptr.add(*ptr as usize + 1);
        let inode = u32::from_be_bytes([*rptr, *rptr.add(1), *rptr.add(2), *rptr.add(3)]);
        let abase = rptr.add(4);
        if *abase == 0 {
            return None; // attributes invalidated
        }
        let record = ::core::slice::from_raw_parts(abase, attrsize as usize);
        Some((inode, imp::fix_attr(record, attrsize)))
    }
}

/// dcache_inodehash_get: find inode, read attr record.
/// SAFETY: d.lock held; blobs alive.
unsafe fn inodehash_get(inner: &mut Inner, attrsize: u8, inode: uint32_t) -> Option<[u8; 36]> {
    unsafe {
        if inner.node_index.is_null() {
            make_node_index(inner, attrsize);
        }
        let ptr = node_index_find(inner.node_index, inode);
        if ptr.is_null() {
            return None;
        }
        let abase = ptr.add(*ptr as usize + 5);
        if *abase == 0 {
            return None;
        }
        let record = ::core::slice::from_raw_parts(abase, attrsize as usize);
        Some(imp::fix_attr(record, attrsize))
    }
}

/// SAFETY: d.lock held; blobs alive.
unsafe fn inodehash_set(inner: &mut Inner, attrsize: u8, inode: uint32_t, attr: *const uint8_t) -> bool {
    unsafe {
        if inner.node_index.is_null() {
            make_node_index(inner, attrsize);
        }
        let ptr = node_index_find(inner.node_index, inode);
        if ptr.is_null() {
            return false;
        }
        let wptr = ptr.add(*ptr as usize + 5);
        let n = (attrsize as usize).min(ATTR_RECORD_SIZE);
        ::core::ptr::copy_nonoverlapping(attr, wptr, n);
        true
    }
}

/// SAFETY: d.lock held; blobs alive.
unsafe fn inodehash_invalidate_attr(inner: &mut Inner, attrsize: u8, inode: uint32_t) -> bool {
    unsafe {
        if inner.node_index.is_null() {
            make_node_index(inner, attrsize);
        }
        let ptr = node_index_find(inner.node_index, inode);
        if ptr.is_null() {
            return false;
        }
        let wptr = ptr.add(*ptr as usize + 5);
        ::core::ptr::write_bytes(wptr, 0, attrsize as usize);
        true
    }
}

/// SAFETY: d.lock held; blobs alive.
unsafe fn namehash_invalidate(inner: &mut Inner, attrsize: u8, nleng: uint8_t, name: *const uint8_t) {
    unsafe {
        if inner.name_index.is_null() {
            make_name_index(inner, attrsize);
        }
        let ptr = name_index_find(inner.name_index, name, nleng);
        if !ptr.is_null() {
            let wptr = ptr.add(*ptr as usize + 1);
            ::core::ptr::write_bytes(wptr, 0, 4 + attrsize as usize);
        }
    }
}

fn registry_find(parent: uint32_t, ctx: *const fuse_ctx) -> Vec<usize> {
    let g = REGISTRY.lock().unwrap();
    g.iter()
        .copied()
        .filter(|&h| {
            let d = unsafe { &*(h as *const DirCache) };
            // SAFETY: handle validity is the caller's contract (dcache_new
            // → dcache_release); registry holds only live handles.
            d.parent == parent
                && imp::ctx_matches(
                    unsafe { (*ctx).pid },
                    unsafe { (*ctx).uid },
                    unsafe { (*ctx).gid },
                    d.pid,
                    d.uid,
                    d.gid,
                )
        })
        .collect()
}

fn registry_all_ctx(ctx: *const fuse_ctx) -> Vec<usize> {
    let g = REGISTRY.lock().unwrap();
    g.iter()
        .copied()
        .filter(|&h| {
            let d = unsafe { &*(h as *const DirCache) };
            imp::ctx_matches(
                unsafe { (*ctx).pid },
                unsafe { (*ctx).uid },
                unsafe { (*ctx).gid },
                d.pid,
                d.uid,
                d.gid,
            )
        })
        .collect()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dcache_new(ctx: *const fuse_ctx, parent: uint32_t, attrsize: uint8_t) -> *mut ::core::ffi::c_void {
    unsafe {
        let d = Box::new(DirCache {
            pid: (*ctx).pid,
            uid: (*ctx).uid,
            gid: (*ctx).gid,
            parent,
            attrsize,
            inner: StdMutex::new(Inner {
                blobs: Vec::new(),
                name_index: ::core::ptr::null_mut(),
                node_index: ::core::ptr::null_mut(),
            }),
        });
        let h = Box::into_raw(d) as usize;
        REGISTRY.lock().unwrap().push(h);
        h as *mut ::core::ffi::c_void
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dcache_release(r: *mut ::core::ffi::c_void) {
    if r.is_null() {
        return;
    }
    let h = r as usize;
    REGISTRY.lock().unwrap().retain(|&x| x != h);
    // SAFETY: handle from dcache_new, released exactly once.
    let d = unsafe { Box::from_raw(r as *mut DirCache) };
    let inner = d.inner.lock().unwrap();
    unsafe {
        if !inner.name_index.is_null() {
            name_index_destroy(inner.name_index);
        }
        if !inner.node_index.is_null() {
            node_index_destroy(inner.node_index);
        }
    }
    // blobs are caller-owned — intentionally not freed (C contract)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dcache_append(r: *mut ::core::ffi::c_void, dbuff: *mut uint8_t, dsize: uint32_t) {
    unsafe {
        // SAFETY: handle from dcache_new; blob stays caller-owned.
        let d = &*(r as *const DirCache);
        let mut inner = d.inner.lock().unwrap();
        inner.blobs.push((dbuff, dsize));
        let mask = (if inner.name_index.is_null() {
            0
        } else {
            NAME_INDEX_FLAG
        }) | (if inner.node_index.is_null() {
            0
        } else {
            NODE_INDEX_FLAG
        });
        if mask != 0 {
            // index the new blob only (C: dcache_add_blob_to_indexes(db))
            let end = dbuff.add(dsize as usize);
            let mut ptr = dbuff;
            while ptr < end {
                let enleng = *ptr as usize;
                if mask & NAME_INDEX_FLAG != 0 {
                    name_index_add(inner.name_index, ptr);
                }
                if mask & NODE_INDEX_FLAG != 0 {
                    node_index_add(inner.node_index, ptr);
                }
                ptr = ptr.add(enleng + 5 + d.attrsize as usize);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dcache_lookup(
    ctx: *const fuse_ctx,
    parent: uint32_t,
    nleng: uint8_t,
    name: *const uint8_t,
    inode: *mut uint32_t,
    attr: *mut uint8_t,
) -> uint8_t {
    for h in registry_find(parent, ctx) {
        // SAFETY: live handle; blobs alive.
        let d = unsafe { &*(h as *const DirCache) };
        let mut inner = d.inner.lock().unwrap();
        if let Some((ino, rec)) = unsafe { namehash_get(&mut inner, d.attrsize, nleng, name) } {
            unsafe {
                *inode = ino;
                ::core::ptr::copy_nonoverlapping(rec.as_ptr(), attr, ATTR_RECORD_SIZE);
            }
            return 1;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dcache_getattr(
    ctx: *const fuse_ctx,
    inode: uint32_t,
    attr: *mut uint8_t,
) -> uint8_t {
    for h in registry_all_ctx(ctx) {
        // SAFETY: live handle; blobs alive.
        let d = unsafe { &*(h as *const DirCache) };
        let mut inner = d.inner.lock().unwrap();
        if let Some(rec) = unsafe { inodehash_get(&mut inner, d.attrsize, inode) } {
            unsafe {
                ::core::ptr::copy_nonoverlapping(rec.as_ptr(), attr, ATTR_RECORD_SIZE);
            }
            return 1;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dcache_setattr(inode: uint32_t, attr: *const uint8_t) {
    let all: Vec<usize> = REGISTRY.lock().unwrap().clone();
    for h in all {
        // SAFETY: live handle; blobs alive.
        let d = unsafe { &*(h as *const DirCache) };
        let mut inner = d.inner.lock().unwrap();
        unsafe {
            inodehash_set(&mut inner, d.attrsize, inode, attr);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dcache_invalidate_attr(inode: uint32_t) {
    let all: Vec<usize> = REGISTRY.lock().unwrap().clone();
    for h in all {
        // SAFETY: live handle; blobs alive.
        let d = unsafe { &*(h as *const DirCache) };
        let mut inner = d.inner.lock().unwrap();
        unsafe {
            inodehash_invalidate_attr(&mut inner, d.attrsize, inode);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dcache_invalidate_name(parent: uint32_t, nleng: uint8_t, name: *const uint8_t) {
    let all: Vec<usize> = {
        let g = REGISTRY.lock().unwrap();
        g.iter()
            .copied()
            .filter(|&h| {
                // SAFETY: live handle.
                let d = unsafe { &*(h as *const DirCache) };
                d.parent == parent
            })
            .collect()
    };
    for h in all {
        // SAFETY: live handle; blobs alive.
        let d = unsafe { &*(h as *const DirCache) };
        let mut inner = d.inner.lock().unwrap();
        unsafe {
            namehash_invalidate(&mut inner, d.attrsize, nleng, name);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;
    use std::vec::Vec;

    /// build a blob: entries [nleng][name][inode:4][attr:attrsize]
    fn blob(entries: &[(&[u8], u32)], attrsize: u8) -> Vec<u8> {
        let mut v = Vec::new();
        for (name, inode) in entries {
            v.push(name.len() as u8);
            v.extend_from_slice(name);
            v.extend_from_slice(&inode.to_be_bytes());
            v.extend(std::iter::repeat(1u8).take(attrsize as usize));
        }
        v
    }

    #[test]
    fn elemcount_matches_walk() {
        let b = blob(&[(b"aa", 1), (b"bbbb", 2), (b"c", 3)], 10);
        assert_eq!(elemcount(&b, 10), 3);
        // trailing partial entry: counted by C only if complete
        let mut partial = blob(&[(b"aa", 1)], 10);
        partial.extend_from_slice(&[3, b'x']); // claims nleng=3 but truncated
        assert_eq!(elemcount(&partial, 10), 1);
        // empty
        assert_eq!(elemcount(&[], 10), 0);
        // attrsize affects step
        let b2 = blob(&[(b"aa", 1), (b"bb", 2)], 0);
        assert_eq!(elemcount(&b2, 0), 2);
        assert_eq!(elemcount(&b2, 10), 0); // steps overshoot
    }

    #[test]
    fn fix_attr_copy_and_zerofill() {
        let rec = [7u8; 10];
        let out = fix_attr(&rec, 10);
        assert_eq!(&out[..10], &[7u8; 10]);
        assert_eq!(&out[10..], &[0u8; 26]);
        let rec36 = [9u8; 36];
        let out = fix_attr(&rec36, 36);
        assert_eq!(out, [9u8; 36]);
        // attrsize larger than record: C reads min(attrsize,36) — same here
        let out = fix_attr(&rec36, 40);
        assert_eq!(out, [9u8; 36]);
    }

    #[test]
    fn ctx_matching() {
        assert!(ctx_matches(1, 2, 3, 1, 2, 3));
        assert!(!ctx_matches(1, 2, 3, 1, 2, 4));
        assert!(!ctx_matches(0, 2, 3, 1, 2, 3));
    }
}
