//! Node index over directory-blob entries — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfsclient/dirblob_node_index.c. Same open-addressed
//! table as dirblob_name_index but keyed by u32 node id stored big-endian
//! inside each blob entry at offset 1+len. Key subtleties preserved:
//!   - node is read LIVE from the blob on every probe (dirattrcache zeroes
//!     node ids to invalidate entries in place);
//!   - rehash drops entries whose live node became 0 and decrements count.
//! All table logic is safe Rust in `imp`; the blob deref lives in the
//! `node_of` callback at the unsafe export boundary.

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;

#[deny(unsafe_code)]
mod imp {
    pub const REHASH_STEP: usize = 5;
    pub const CLUSTER_SIZE: u32 = 9;

    /// node * 33, wrapping (exact C semantics).
    pub fn hash_function(node: u32) -> u32 {
        node.wrapping_mul(33)
    }

    #[derive(Copy, Clone)]
    struct Entry {
        ptr: *mut u8, // null = empty slot
        hash: u32,    // probe-path hash; may go stale when a blob is zeroed —
                      // matches C, which recomputes live at rehash/find time
    }

    const EMPTY: Entry = Entry {
        ptr: ::core::ptr::null_mut(),
        hash: 0,
    };

    pub struct NodeIndex {
        old: Vec<Entry>,
        new: Vec<Entry>,
        count: usize,
        rehash_pos: usize,
        is_rehashing: bool,
    }

    fn initial_table_size(minelements: u32) -> usize {
        let mut s = minelements.wrapping_mul(5).wrapping_div(3);
        s |= s >> 1;
        s |= s >> 2;
        s |= s >> 4;
        s |= s >> 8;
        s |= s >> 16;
        s = s.wrapping_add(1);
        if s < 256 {
            s = 256;
        }
        s as usize
    }

    fn probe_displacement(hash: u32, mask: u32) -> u32 {
        hash.wrapping_mul(0x53b23891) & mask | 1
    }

    fn hash_insert(table: &mut [Entry], ptr: *mut u8, hash: u32) {
        let mask = table.len() as u32 - 1;
        let disp = probe_displacement(hash, mask);
        // probe = slot cursor; hash = content hash (never clobber the cache)
        let mut probe = hash;
        loop {
            for i in 0..CLUSTER_SIZE {
                let slot = (probe.wrapping_add(i) & mask) as usize;
                if table[slot].ptr.is_null() {
                    table[slot] = Entry { ptr, hash };
                    return;
                }
            }
            probe = probe.wrapping_add(disp);
        }
    }

    /// C semantics: compares the query node against the LIVE blob node of
    /// each candidate via `node_of` (blob deref at the boundary).
    fn hash_find(
        table: &[Entry],
        node: u32,
        hash: u32,
        node_of: &dyn Fn(*mut u8) -> u32,
    ) -> *mut u8 {
        let mask = table.len() as u32 - 1;
        let disp = probe_displacement(hash, mask);
        let mut probe = hash;
        loop {
            for i in 0..CLUSTER_SIZE {
                let slot = (probe.wrapping_add(i) & mask) as usize;
                let e = table[slot];
                if e.ptr.is_null() {
                    return ::core::ptr::null_mut();
                }
                if node_of(e.ptr) == node {
                    return e.ptr;
                }
            }
            probe = probe.wrapping_add(disp);
        }
    }

    impl NodeIndex {
        pub fn new(minelements: u32) -> Self {
            NodeIndex {
                old: vec![EMPTY; initial_table_size(minelements)],
                new: Vec::new(),
                count: 0,
                rehash_pos: 0,
                is_rehashing: false,
            }
        }

        /// Move up to REHASH_STEP entries; entries whose live node is now 0
        /// are dropped (count decremented) exactly as in C.
        fn incremental_rehash_step(&mut self, node_of: &dyn Fn(*mut u8) -> u32) {
            if !self.is_rehashing {
                return;
            }
            let mut steps = REHASH_STEP;
            while steps > 0 && self.rehash_pos < self.old.len() {
                let e = self.old[self.rehash_pos];
                if !e.ptr.is_null() {
                    let node = node_of(e.ptr);
                    if node > 0 {
                        hash_insert(&mut self.new, e.ptr, hash_function(node));
                    } else {
                        self.count -= 1;
                    }
                }
                self.rehash_pos += 1;
                steps -= 1;
            }
            if self.rehash_pos >= self.old.len() {
                ::core::mem::swap(&mut self.old, &mut self.new);
                self.new = Vec::new();
                self.is_rehashing = false;
                self.rehash_pos = 0;
            }
        }

        pub fn add(&mut self, ptr: *mut u8, node_of: &dyn Fn(*mut u8) -> u32) {
            self.incremental_rehash_step(node_of);
            if !self.is_rehashing && self.count * 5 > self.old.len() * 3 {
                self.new = vec![EMPTY; self.old.len() * 2];
                self.is_rehashing = true;
                self.rehash_pos = 0;
            }
            let node = node_of(ptr);
            if node == 0 {
                return;
            }
            let hash = hash_function(node);
            if self.is_rehashing {
                hash_insert(&mut self.new, ptr, hash);
            } else {
                hash_insert(&mut self.old, ptr, hash);
            }
            self.count += 1;
        }

        pub fn find(&mut self, node: u32, node_of: &dyn Fn(*mut u8) -> u32) -> *mut u8 {
            self.incremental_rehash_step(node_of);
            let hash = hash_function(node);
            if self.is_rehashing {
                let ptr = hash_find(&self.new, node, hash, node_of);
                if !ptr.is_null() {
                    return ptr;
                }
            }
            hash_find(&self.old, node, hash, node_of)
        }
    }
}

use imp::NodeIndex;

/// Read the live big-endian u32 node id from a blob entry.
/// SAFETY: `ptr` must point at a live blob entry [len][bytes...] whose node
/// field sits at offset 1+len (dirattrcache layout contract).
unsafe fn node_of_impl(ptr: *mut uint8_t) -> uint32_t {
    unsafe {
        let len = *ptr as usize;
        let bp = ptr.add(1 + len);
        u32::from_be_bytes([*bp, *bp.add(1), *bp.add(2), *bp.add(3)])
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn node_index_create(minelements: uint32_t) -> *mut ::core::ffi::c_void {
    Box::into_raw(Box::new(NodeIndex::new(minelements))) as *mut ::core::ffi::c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn node_index_destroy(vidx: *mut ::core::ffi::c_void) {
    if !vidx.is_null() {
        // SAFETY: handle from node_index_create, freed exactly once by caller.
        drop(unsafe { Box::from_raw(vidx as *mut NodeIndex) });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn node_index_add(vidx: *mut ::core::ffi::c_void, ptr: *mut uint8_t) {
    // SAFETY: handle from node_index_create; ptr is a live blob entry.
    let idx = unsafe { &mut *(vidx as *mut NodeIndex) };
    idx.add(ptr, &|p| unsafe { node_of_impl(p) });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn node_index_find(
    vidx: *mut ::core::ffi::c_void,
    node: uint32_t,
) -> *mut uint8_t {
    // SAFETY: handle from node_index_create.
    let idx = unsafe { &mut *(vidx as *mut NodeIndex) };
    idx.find(node, &|p| unsafe { node_of_impl(p) })
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::vec::Vec;

    /// Blob entry: [len][name bytes][node u32 BE]. Node offset = 1+len.
    struct Blobs {
        mem: Vec<Box<[u8]>>,
    }
    impl Blobs {
        fn add(&mut self, name: &[u8], node: u32) -> *mut uint8_t {
            let mut b = Vec::with_capacity(name.len() + 5);
            b.push(name.len() as u8);
            b.extend_from_slice(name);
            b.extend_from_slice(&node.to_be_bytes());
            let boxed = b.into_boxed_slice();
            let p = boxed.as_ptr() as *mut uint8_t;
            self.mem.push(boxed);
            p
        }
        /// Zero the node field in place (dirattrcache invalidation).
        fn zero_node(&self, p: *mut uint8_t) {
            // SAFETY: test-only; blobs outlive the index.
            unsafe {
                let len = *p as usize;
                ::core::ptr::write_bytes(p.add(1 + len), 0, 4);
            }
        }
    }

    #[test]
    fn add_find_roundtrip_with_rehash() {
        let mut blobs = Blobs { mem: Vec::new() };
        let idx = unsafe { node_index_create(4) };
        let mut ptrs = Vec::new();
        for i in 1..=1000u32 {
            let name = format!("n{:04}", i);
            let p = blobs.add(name.as_bytes(), i);
            // SAFETY: test-only; blobs outlive the index.
            unsafe { node_index_add(idx, p) };
            ptrs.push(p);
        }
        for (i, &p) in (1..=1000u32).zip(ptrs.iter()) {
            assert_eq!(unsafe { node_index_find(idx, i) }, p, "lookup failed for {i}");
        }
        assert!(unsafe { node_index_find(idx, 1001) }.is_null());
        assert!(unsafe { node_index_find(idx, 0) }.is_null());
        // SAFETY: test-only; blobs outlive the index.
        unsafe { node_index_destroy(idx) };
    }

    #[test]
    fn zeroed_node_is_invalidated_live() {
        let mut blobs = Blobs { mem: Vec::new() };
        let idx = unsafe { node_index_create(4) };
        let p1 = blobs.add(b"aaa", 11);
        let p2 = blobs.add(b"bbb", 22);
        // SAFETY: test-only; blobs outlive the index.
        unsafe {
            node_index_add(idx, p1);
            node_index_add(idx, p2);
        }
        assert_eq!(unsafe { node_index_find(idx, 11) }, p1);
        // zero node 11 in place: C semantics — find must stop returning it
        // immediately (live read), and rehash must drop it (count -= 1).
        blobs.zero_node(p1);
        assert!(unsafe { node_index_find(idx, 11) }.is_null());
        assert_eq!(unsafe { node_index_find(idx, 22) }, p2);
        // node==0 entries are never added
        let p0 = blobs.add(b"ccc", 0);
        // SAFETY: test-only; blobs outlive the index.
        unsafe { node_index_add(idx, p0) };
        assert!(unsafe { node_index_find(idx, 0) }.is_null());
        // SAFETY: test-only; blobs outlive the index.
        unsafe { node_index_destroy(idx) };
    }

    #[test]
    fn rehash_drops_zeroed_nodes() {
        let mut blobs = Blobs { mem: Vec::new() };
        let idx = unsafe { node_index_create(4) };
        // fill past growth threshold (256 * 3/5 = 153) to force rehashing
        let mut ptrs = Vec::new();
        for i in 1..=200u32 {
            let name = format!("z{:04}", i);
            let p = blobs.add(name.as_bytes(), i);
            // SAFETY: test-only; blobs outlive the index.
            unsafe { node_index_add(idx, p) };
            ptrs.push(p);
        }
        // zero a bunch, then keep adding to drive the rehash to completion;
        // every remaining live node must still be findable.
        for &p in ptrs.iter().take(50) {
            blobs.zero_node(p);
        }
        for i in 201..=400u32 {
            let name = format!("z{:04}", i);
            let p = blobs.add(name.as_bytes(), i);
            // SAFETY: test-only; blobs outlive the index.
            unsafe { node_index_add(idx, p) };
            ptrs.push(p);
        }
        for (i, &p) in (51..=400u32).zip(ptrs[50..].iter()) {
            assert_eq!(unsafe { node_index_find(idx, i) }, p, "lost node {i}");
        }
        for i in 1..=50u32 {
            assert!(unsafe { node_index_find(idx, i) }.is_null(), "zombie node {i}");
        }
        // SAFETY: test-only; blobs outlive the index.
        unsafe { node_index_destroy(idx) };
    }
}
