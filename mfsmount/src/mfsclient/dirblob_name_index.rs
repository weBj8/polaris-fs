//! Name index over directory-blob name entries — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfsclient/dirblob_name_index.c (open-addressed hash
//! table with incremental rehash, storing `uint8_t*` blob pointers with
//! layout [len][bytes...] owned by dirattrcache).
//!
//! All table logic (growth policy, probing, incremental rehash) is safe Rust
//! in `imp`. The only unsafe left is at the export boundary where blob bytes
//! are read; entries cache their hash at add time so rehash never derefs.
//! dirattrcache uses only the opaque void* handle API, so the Box handle is
//! ABI-compatible.

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;

#[deny(unsafe_code)]
mod imp {
    pub const REHASH_STEP: usize = 5;
    pub const CLUSTER_SIZE: u32 = 9;

    /// djb2 over the name bytes (exact C semantics: wrapping u32).
    pub fn hash_function(name: &[u8]) -> u32 {
        let mut h: u32 = 5381;
        for &b in name {
            h = (h << 5).wrapping_add(h).wrapping_add(b as u32);
        }
        h
    }

    #[derive(Copy, Clone)]
    struct Entry {
        ptr: *mut u8, // null = empty slot
        hash: u32,    // cached at add time
    }
    const EMPTY: Entry = Entry {
        ptr: ::core::ptr::null_mut(),
        hash: 0,
    };

    pub struct NameIndex {
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
        // probe = slot cursor (mutates per hop); hash = content hash (cached
        // in the entry, must never be clobbered by the probe cursor)
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

    /// Find `name` in `table`. `eq` confirms a candidate by comparing its
    /// blob bytes (blob deref lives at the unsafe boundary).
    fn hash_find(table: &[Entry], hash: u32, eq: &dyn Fn(*mut u8) -> bool) -> *mut u8 {
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
                if eq(e.ptr) {
                    return e.ptr;
                }
            }
            probe = probe.wrapping_add(disp);
        }
    }

    impl NameIndex {
        pub fn new(minelements: u32) -> Self {
            NameIndex {
                old: vec![EMPTY; initial_table_size(minelements)],
                new: Vec::new(),
                count: 0,
                rehash_pos: 0,
                is_rehashing: false,
            }
        }

        fn incremental_rehash_step(&mut self) {
            if !self.is_rehashing {
                return;
            }
            let mut steps = REHASH_STEP;
            while steps > 0 && self.rehash_pos < self.old.len() {
                let e = self.old[self.rehash_pos];
                if !e.ptr.is_null() {
                    hash_insert(&mut self.new, e.ptr, e.hash);
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



        /// `hash` must be hash_function over the blob's name bytes.
        pub fn add(&mut self, ptr: *mut u8, hash: u32) {
            self.incremental_rehash_step();
            if !self.is_rehashing && self.count * 5 > self.old.len() * 3 {
                self.new = vec![EMPTY; self.old.len() * 2];
                self.is_rehashing = true;
                self.rehash_pos = 0;
            }
            if self.is_rehashing {
                hash_insert(&mut self.new, ptr, hash);
            } else {
                hash_insert(&mut self.old, ptr, hash);
            }
            self.count += 1;
        }

        /// Find the blob pointer whose name equals `name`; `eq` confirms a
        /// candidate via its blob bytes. Returns null when absent.
        pub fn find(&mut self, name: &[u8], eq: &dyn Fn(*mut u8) -> bool) -> *mut u8 {
            self.incremental_rehash_step();
            let hash = hash_function(name);
            if self.is_rehashing {
                let ptr = hash_find(&self.new, hash, eq);
                if !ptr.is_null() {
                    return ptr;
                }
            }
            hash_find(&self.old, hash, eq)
        }
    }
}

use imp::NameIndex;

/// Blob layout helper: entries are [len][bytes...] pointers into dirattrcache
/// memory. SAFETY: caller guarantees `ptr` points at a live blob entry.
unsafe fn blob_name(ptr: *const uint8_t) -> (uint8_t, *const uint8_t) {
    // SAFETY: per caller contract; blobs outlive the index (dirattrcache
    // destroys the index before freeing its blob memory).
    unsafe { (*ptr, ptr.offset(1)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn name_index_create(minelements: uint32_t) -> *mut ::core::ffi::c_void {
    Box::into_raw(Box::new(NameIndex::new(minelements))) as *mut ::core::ffi::c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn name_index_destroy(vidx: *mut ::core::ffi::c_void) {
    if !vidx.is_null() {
        // SAFETY: handle from name_index_create, freed exactly once by caller.
        drop(unsafe { Box::from_raw(vidx as *mut NameIndex) });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn name_index_add(vidx: *mut ::core::ffi::c_void, ptr: *mut uint8_t) {
    // SAFETY: per caller contract (dirattrcache blob entry).
    let (len, bytes) = unsafe { blob_name(ptr) };
    let name = unsafe { ::core::slice::from_raw_parts(bytes, len as usize) };
    let hash = imp::hash_function(name);
    // SAFETY: handle from name_index_create.
    let idx = unsafe { &mut *(vidx as *mut NameIndex) };
    idx.add(ptr, hash);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn name_index_find(
    vidx: *mut ::core::ffi::c_void,
    str_: *const uint8_t,
    len: uint8_t,
) -> *mut uint8_t {
    // SAFETY: `str_`/`len` describe the lookup name per caller contract.
    let name = unsafe { ::core::slice::from_raw_parts(str_, len as usize) };
    // SAFETY: handle from name_index_create.
    let idx = unsafe { &mut *(vidx as *mut NameIndex) };
    idx.find(name, &|cand: *mut uint8_t| {
        // SAFETY: candidate is a live blob entry (same contract as add).
        let (clen, cbytes) = unsafe { blob_name(cand) };
        clen == len && unsafe { ::core::slice::from_raw_parts(cbytes, clen as usize) } == name
    })
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::vec::Vec;

    struct Blobs {
        mem: Vec<Box<[u8]>>,
    }
    impl Blobs {
        fn add(&mut self, name: &[u8]) -> *mut uint8_t {
            let mut b = Vec::with_capacity(name.len() + 1);
            b.push(name.len() as u8);
            b.extend_from_slice(name);
            let boxed = b.into_boxed_slice();
            let p = boxed.as_ptr() as *mut uint8_t;
            assert!(!self.mem.iter().any(|x| x.as_ptr() as *mut uint8_t == p),
                "blob address reuse: {p:p}");
            self.mem.push(boxed);
            p
        }
    }

    fn find(idx: *mut ::core::ffi::c_void, name: &[u8]) -> *mut uint8_t {
        // SAFETY: test-only; blobs outlive the index.
        unsafe { name_index_find(idx, name.as_ptr(), name.len() as u8) }
    }

    #[test]
    fn add_find_roundtrip_with_rehash() {
        let mut blobs = Blobs { mem: Vec::new() };
        let idx = unsafe { name_index_create(4) };
        // 1000 entries forces several growth + incremental rehash cycles.
        let names: Vec<std::string::String> =
            (0..1000u32).map(|i| format!("file-name-{:04}", i)).collect();
        let mut ptrs = Vec::new();
        for n in &names {
            let p = blobs.add(n.as_bytes());
            // SAFETY: test-only; blobs outlive the index.
            unsafe { name_index_add(idx, p) };
            ptrs.push(p);
        }
        for (n, &p) in names.iter().zip(ptrs.iter()) {
            assert_eq!(find(idx, n.as_bytes()), p, "lookup failed for {n}");
        }
        // absent names
        assert!(find(idx, b"no-such-file").is_null());
        assert!(find(idx, b"file-name-1000").is_null());
        // same prefix, different length
        assert!(find(idx, b"file-name-000").is_null());
        // SAFETY: test-only; blobs outlive the index.
        unsafe { name_index_destroy(idx) };
    }




    #[test]
    fn hash_matches_c_reference() {
        // djb2, wrapping — reference vector computed from the C algorithm.
        assert_eq!(imp::hash_function(b""), 5381);
        assert_eq!(imp::hash_function(b"a"), 177670);
        assert_eq!(imp::hash_function(b"hello"), 261238937);
    }
}
