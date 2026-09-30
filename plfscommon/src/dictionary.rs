//! Reference-counted byte-string dictionary — port of mfscommon/dictionary.c.
//!
//! Used by plfsmaster (xattr names/values). C interns each distinct byte
//! string once in an incrementally rehashed chained table (hash_begin.h)
//! and hands out the entry pointer as an opaque handle. Here the table is a
//! `HashMap` keyed by the bytes, and each handle is a stable `Box<Entry>`.
//! The table layout was internal (only `dict_printall`, which no daemon
//! calls, exposed it), so it is not reproduced; lookups, refcounting,
//! `dict_get_hash` and the clean-up assertion match C.
//!
//! `dict_printall` is not exported: it was a debug-only helper referenced
//! nowhere in the tree.

use core::ffi::{c_int, c_void};

use crate::mfslog::massert_abort;

pub use imp::{Entry, c_hash};

const DICTIONARY_C: &str = "../mfscommon/dictionary.c";
const HASH_BEGIN_H: &str = "../mfscommon/hash_begin.h";

#[deny(unsafe_code)]
mod imp {
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::{Mutex, MutexGuard};

    /// Handle target. `data` is the address of the owning map key's heap
    /// buffer (`leng` bytes), which never moves while the entry is live.
    pub struct Entry {
        pub data: usize,
        pub leng: u32,
        pub hashval: u32,
        refcnt: AtomicU32,
    }

    struct Slot {
        entry: Box<Entry>,
    }

    static MAP: Mutex<Option<HashMap<Box<[u8]>, Slot>>> = Mutex::new(None);

    fn lock() -> MutexGuard<'static, Option<HashMap<Box<[u8]>, Slot>>> {
        MAP.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// C `dict_hash`: `hash = leng; hash = hash*33 + byte`.
    pub fn c_hash(data: &[u8]) -> u32 {
        data.iter()
            .fold(data.len() as u32, |h, &b| h.wrapping_mul(33).wrapping_add(b as u32))
    }

    pub fn init() {
        let mut m = lock();
        if m.is_none() {
            *m = Some(HashMap::new());
        }
    }

    /// C `dict_cleanup`: Err if entries remain (C `massert` in hash_begin.h).
    pub fn cleanup() -> Result<(), ()> {
        let mut m = lock();
        if m.as_ref().is_some_and(|h| !h.is_empty()) {
            return Err(());
        }
        *m = None;
        Ok(())
    }

    /// Handle address of an interned string, if present.
    pub fn search(data: &[u8]) -> Option<usize> {
        lock().as_ref()?.get(data).map(|s| &*s.entry as *const Entry as usize)
    }

    /// C `dict_insert`: existing entry gets refcnt+1, else a new one (1).
    pub fn insert(data: &[u8]) -> usize {
        let mut m = lock();
        let h = m.get_or_insert_with(HashMap::new);
        if let Some(s) = h.get(data) {
            s.entry.refcnt.fetch_add(1, Ordering::Relaxed);
            return &*s.entry as *const Entry as usize;
        }
        let key: Box<[u8]> = data.into();
        let entry = Box::new(Entry {
            data: key.as_ptr() as usize,
            leng: data.len() as u32,
            hashval: c_hash(data),
            refcnt: AtomicU32::new(1),
        });
        let addr = &*entry as *const Entry as usize;
        h.insert(key, Slot { entry });
        addr
    }

    /// C `dict_inc_ref`; Err if the counter was zero.
    pub fn inc_ref(e: &Entry) -> Result<(), ()> {
        let _m = lock();
        if e.refcnt.load(Ordering::Relaxed) == 0 {
            return Err(());
        }
        e.refcnt.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// C `dict_dec_ref`: frees the entry at zero; Err if already zero.
    /// Looked up by (an owned copy of) the entry's bytes, so no reference
    /// into the entry is alive when it is freed.
    pub fn dec_ref(key: &[u8]) -> Result<(), ()> {
        let mut m = lock();
        let h = m.as_mut().ok_or(())?;
        let n = h.get(key).ok_or(())?.entry.refcnt.load(Ordering::Relaxed);
        if n == 0 {
            return Err(());
        }
        if n == 1 {
            h.remove(key);
        } else if let Some(s) = h.get(key) {
            s.entry.refcnt.store(n - 1, Ordering::Relaxed);
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn refcnt(e: &Entry) -> u32 {
        e.refcnt.load(Ordering::Relaxed)
    }

    #[cfg(test)]
    pub fn test_lock() -> MutexGuard<'static, ()> {
        static T: Mutex<()> = Mutex::new(());
        T.lock().unwrap_or_else(|e| e.into_inner())
    }
}

// ---------------------------------------------------------------------------
// C ABI boundary (dictionary.h). Handles are `Entry` addresses.
// ---------------------------------------------------------------------------

/// # Safety
// SAFETY: caller guarantees `dptr` is a live handle from dict_insert.
unsafe fn entry<'a>(dptr: *mut c_void) -> &'a Entry {
    // SAFETY: per fn contract the Box<Entry> is alive in the map.
    unsafe { &*(dptr as *const Entry) }
}

/// # Safety
// SAFETY: caller guarantees `data` is readable for `leng` bytes.
unsafe fn slice<'a>(data: *const u8, leng: u32) -> &'a [u8] {
    if leng == 0 {
        &[]
    } else {
        // SAFETY: per fn contract.
        unsafe { std::slice::from_raw_parts(data, leng as usize) }
    }
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn dict_init() -> c_int {
    imp::init();
    0
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn dict_cleanup() {
    if imp::cleanup().is_err() {
        massert_abort(
            HASH_BEGIN_H,
            133,
            "dicthashtab[i][j]==NULL",
            "hash map has elements during clean up",
        );
    }
}

/// # Safety
/// `data` readable for `leng` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_search(data: *const u8, leng: u32) -> *mut c_void {
    // SAFETY: per fn contract.
    imp::search(unsafe { slice(data, leng) }).map_or(core::ptr::null_mut(), |a| a as *mut c_void)
}

/// # Safety
/// `data` readable for `leng` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_insert(data: *const u8, leng: u32) -> *mut c_void {
    // SAFETY: per fn contract.
    imp::insert(unsafe { slice(data, leng) }) as *mut c_void
}

/// # Safety
/// `dptr` is a live handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_get_ptr(dptr: *mut c_void) -> *const u8 {
    // SAFETY: per fn contract.
    unsafe { entry(dptr) }.data as *const u8
}

/// # Safety
/// `dptr` is a live handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_get_leng(dptr: *mut c_void) -> u32 {
    // SAFETY: per fn contract.
    unsafe { entry(dptr) }.leng
}

/// # Safety
/// `dptr` is a live handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_get_hash(dptr: *mut c_void) -> u32 {
    // SAFETY: per fn contract.
    unsafe { entry(dptr) }.hashval
}

/// # Safety
/// `dptr` is a live handle; it is invalid after the last reference drops.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_dec_ref(dptr: *mut c_void) {
    // SAFETY: per fn contract; the bytes are copied out before the entry
    // can be freed inside dec_ref.
    let key = unsafe {
        let e = entry(dptr);
        slice(e.data as *const u8, e.leng).to_vec()
    };
    if imp::dec_ref(&key).is_err() {
        massert_abort(DICTIONARY_C, 139, "de->refcnt>0", "dictionary reference counter is zero");
    }
}

/// # Safety
/// `dptr` is a live handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_inc_ref(dptr: *mut c_void) {
    // SAFETY: per fn contract.
    if imp::inc_ref(unsafe { entry(dptr) }).is_err() {
        massert_abort(DICTIONARY_C, 149, "de->refcnt>0", "dictionary reference counter is zero");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bytes(h: *mut c_void) -> Vec<u8> {
        // SAFETY: test handles are live.
        unsafe { slice(dict_get_ptr(h), dict_get_leng(h)).to_vec() }
    }

    #[test]
    fn intern_refcount_and_free() {
        let _g = imp::test_lock();
        dict_init();
        // SAFETY: literal buffers; handles used only while live.
        unsafe {
            assert!(dict_search(b"user.x".as_ptr(), 6).is_null());
            let a = dict_insert(b"user.x".as_ptr(), 6);
            let b = dict_insert(b"user.x".as_ptr(), 6);
            assert_eq!(a, b);
            assert_eq!(imp::refcnt(entry(a)), 2);
            assert_eq!(bytes(a), b"user.x");
            assert_eq!(dict_get_hash(a), c_hash(b"user.x"));
            let e = dict_insert(b"".as_ptr(), 0);
            assert_eq!(dict_get_leng(e), 0);
            assert!(!dict_get_ptr(e).is_null());
            dict_inc_ref(e);
            dict_dec_ref(e);
            dict_dec_ref(e);
            assert!(dict_search(b"".as_ptr(), 0).is_null());
            assert_eq!(dict_search(b"user.x".as_ptr(), 6), a);
            dict_dec_ref(a);
            dict_dec_ref(b);
            assert!(dict_search(b"user.x".as_ptr(), 6).is_null());
        }
        dict_cleanup();
    }

    #[test]
    fn c_hash_formula() {
        assert_eq!(c_hash(b""), 0);
        assert_eq!(c_hash(b"a"), 33 + 97);
        let mut h: u32 = 3;
        for &c in b"abc" {
            h = h.wrapping_mul(33).wrapping_add(c as u32);
        }
        assert_eq!(c_hash(b"abc"), h);
        let long = vec![0xffu8; 1000];
        let mut h: u32 = 1000;
        for &c in &long {
            h = h.wrapping_mul(33).wrapping_add(c as u32);
        }
        assert_eq!(c_hash(&long), h);
    }
}
