//! u64 → pointer map — port of mfscommon/cuckoohash.c.
//!
//! plfsmaster uses one instance (`snapshot_inodehash`, hardlink tracking
//! during snapshots) through `chash_new/add/find/erase`. C implements it as
//! a two-choice bucketed cuckoo table with a treap overflow; its observable
//! contract is a map where `chash_add` keeps the first value for a key.
//! Here it is a `HashMap<u64, usize>` behind the opaque handle.
//!
//! Differences, all unobservable to MooseFS: the treap's node priorities
//! and delete tie-breaks drew from the shared RC4 stream (`rndu32`/`rndu8`)
//! on the rare overflow path; that stream is time-seeded and consumers only
//! need uniformity, so not drawing is equivalent. `chash_get_size` (a
//! C-layout byte estimate) is not exported: nothing in the tree calls it.

use core::ffi::c_void;
use std::collections::HashMap;

pub use imp::Chash;

#[deny(unsafe_code)]
mod imp {
    use super::HashMap;

    /// Keys → opaque value addresses.
    #[derive(Default)]
    pub struct Chash(HashMap<u64, usize>);

    impl Chash {
        /// C `chash_add`: an existing key keeps its first value.
        pub fn add(&mut self, k: u64, v: usize) {
            self.0.entry(k).or_insert(v);
        }
        /// C `chash_find` (0 = NULL = absent).
        pub fn find(&self, k: u64) -> usize {
            self.0.get(&k).copied().unwrap_or(0)
        }
        pub fn delete(&mut self, k: u64) {
            self.0.remove(&k);
        }
        pub fn erase(&mut self) {
            self.0.clear();
        }
        pub fn len(&self) -> u32 {
            self.0.len() as u32
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn first_value_wins_and_erase_keeps_structure() {
            let mut h = Chash::default();
            assert_eq!(h.find(7), 0);
            h.add(7, 0x1000);
            h.add(7, 0x2000);
            assert_eq!(h.find(7), 0x1000);
            for k in 0..100_000u64 {
                h.add(k << 20, k as usize + 1);
            }
            assert_eq!(h.find(99_999 << 20), 100_000);
            assert_eq!(h.len(), 100_001);
            h.delete(7);
            assert_eq!(h.find(7), 0);
            h.erase();
            assert_eq!(h.len(), 0);
            h.add(1, 2);
            assert_eq!(h.find(1), 2);
        }
    }
}

/// # Safety
// SAFETY: caller guarantees `h` came from `chash_new` and is not freed;
// MooseFS uses each table from the main thread only (as the C did).
unsafe fn tab<'a>(h: *mut c_void) -> &'a mut Chash {
    // SAFETY: per fn contract `h` is a live leaked Box<Chash>.
    unsafe { &mut *(h as *mut Chash) }
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn chash_new() -> *mut c_void {
    Box::into_raw(Box::<Chash>::default()) as *mut c_void
}

/// # Safety
/// `h` from `chash_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chash_find(h: *mut c_void, x: u64) -> *mut c_void {
    // SAFETY: per fn contract.
    unsafe { tab(h) }.find(x) as *mut c_void
}

/// # Safety
/// `h` from `chash_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chash_delete(h: *mut c_void, x: u64) {
    // SAFETY: per fn contract.
    unsafe { tab(h) }.delete(x)
}

/// # Safety
/// `h` from `chash_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chash_add(h: *mut c_void, x: u64, v: *mut c_void) {
    // SAFETY: per fn contract.
    unsafe { tab(h) }.add(x, v as usize)
}

/// # Safety
/// `h` from `chash_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chash_erase(h: *mut c_void) {
    // SAFETY: per fn contract.
    unsafe { tab(h) }.erase()
}

/// # Safety
/// `h` from `chash_new`; invalid afterwards.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chash_free(h: *mut c_void) {
    // SAFETY: per fn contract this is the last use of the Box.
    drop(unsafe { Box::from_raw(h as *mut Chash) });
}

/// # Safety
/// `h` from `chash_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chash_get_elemcount(h: *mut c_void) -> u32 {
    // SAFETY: per fn contract.
    unsafe { tab(h) }.len()
}
