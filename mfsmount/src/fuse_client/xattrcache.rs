//! xattr cache — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfsclient/xattrcache.c. Cache of getxattr answers
//! keyed by (node, uid, gid, name), absolute-expiry entries in an
//! insertion-ordered LRU list, values refcounted (lcnt) so a caller can
//! hold a value pointer across the get→rel window while the cache evicts.
//!
//! Safe core in `imp`: HashMap + insertion-ordered VecDeque (timestamps
//! are monotonic so insertion order IS expiry order), values as
//! Rc<XattrValue> — the C lcnt refcount maps 1:1 onto Rc: get clones into
//! a raw token (Rc::into_raw), rel drops it (Rc::from_raw), eviction drops
//! the cache's handle while readers keep theirs. All Rc traffic happens
//! under the module mutex, so non-atomic refcounts are sound.

unsafe extern "C" {
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn monotonic_useconds() -> int64_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __glibc_reserved: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type int64_t = i64;
pub const PTHREAD_MUTEX_TIMED_NP: ::core::ffi::c_uint = 0;

#[deny(unsafe_code)]
pub mod imp {
    use std::collections::{HashMap, VecDeque};
    use std::rc::Rc;

    /// One cached xattr answer; the Rc replaces C's lcnt refcount.
    pub struct XattrValue {
        pub value: Option<Box<[u8]>>,
        pub vleng: u32,
        pub status: i32,
    }

    struct Entry {
        value: Rc<XattrValue>,
        utimestamp: i64, // absolute expiry, microseconds
    }

    type Key = (u32, u32, u32, Box<[u8]>); // (node, uid, gid, name)

    pub struct XattrCache {
        map: HashMap<Key, Entry>,
        /// insertion order == expiry order (monotonic clock)
        lru: VecDeque<Key>,
        timeout_us: i64,
    }

    impl XattrCache {
        pub fn new(timeout_us: i64) -> Self {
            XattrCache {
                map: HashMap::new(),
                lru: VecDeque::new(),
                timeout_us,
            }
        }

        /// drop entries with utimestamp < now (LRU head walk, as C)
        fn invalidate(&mut self, now: i64) {
            while let Some(k) = self.lru.front() {
                let expired = match self.map.get(k) {
                    Some(e) => e.utimestamp < now,
                    None => true, // stale node (deleted/overwritten)
                };
                if !expired {
                    break;
                }
                let k = self.lru.pop_front().unwrap();
                if let Some(e) = self.map.get(&k) {
                    if e.utimestamp < now {
                        self.map.remove(&k);
                    }
                }
            }
        }

        #[allow(clippy::too_many_arguments)]
        pub fn get(
            &mut self,
            node: u32,
            uid: u32,
            gid: u32,
            name: &[u8],
            now: i64,
        ) -> Option<Rc<XattrValue>> {
            self.invalidate(now);
            let key: Key = (node, uid, gid, name.into());
            self.map.get(&key).map(|e| Rc::clone(&e.value))
        }

        #[allow(clippy::too_many_arguments)]
        pub fn set(
            &mut self,
            node: u32,
            uid: u32,
            gid: u32,
            name: &[u8],
            value: Option<&[u8]>,
            status: i32,
            now: i64,
        ) {
            let key: Key = (node, uid, gid, name.into());
            if self.map.remove(&key).is_some() {
                // old LRU node goes stale; dropped at the head walk
            }
            let v = XattrValue {
                vleng: value.as_ref().map_or(0, |v| v.len()) as u32,
                value: value.map(|v| v.to_vec().into_boxed_slice()),
                status,
            };
            self.map.insert(
                key.clone(),
                Entry {
                    value: Rc::new(v),
                    utimestamp: now + self.timeout_us,
                },
            );
            self.lru.push_back(key);
        }

        /// delete ALL uid/gid variants of (node, name)
        pub fn del(&mut self, node: u32, name: &[u8]) {
            self.map
                .retain(|(n, _, _, nm), _| !(*n == node && nm.as_ref() == name));
            // their LRU nodes go stale
        }

        pub fn term(&mut self) {
            self.map.clear();
            self.lru.clear();
        }

        #[cfg(test)]
        pub fn len(&self) -> usize {
            self.map.len()
        }
    }
}

// ---------------------------------------------------------------------------
// Boundary: module mutex + Rc↔raw-token conversion (C lcnt contract).
// ---------------------------------------------------------------------------

use imp::{XattrCache, XattrValue};
use std::rc::Rc;

static mut glock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut CACHE: Option<XattrCache> = None;

/// SAFETY: set in xattr_cache_init before FUSE threads; accessed only with
/// glock held afterwards.
unsafe fn cache() -> &'static mut XattrCache {
    unsafe { (*(&raw mut CACHE)).as_mut().unwrap_unchecked() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_cache_get(
    node: uint32_t,
    uid: uint32_t,
    gid: uint32_t,
    nleng: uint32_t,
    name: *const uint8_t,
    value: *mut *const uint8_t,
    vleng: *mut uint32_t,
    status: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let now = monotonic_useconds();
        assert_eq!(pthread_mutex_lock(&raw mut glock), 0);
        // SAFETY: name points at nleng bytes per C contract.
        let key = ::core::slice::from_raw_parts(name, nleng as usize);
        let hit = cache().get(node, uid, gid, key, now);
        let token = match hit {
            Some(v) => {
                if !value.is_null() {
                    *value = match &v.value {
                        Some(b) => b.as_ptr(),
                        None => ::core::ptr::null(),
                    };
                }
                if !vleng.is_null() {
                    *vleng = v.vleng;
                }
                if !status.is_null() {
                    *status = v.status;
                }
                // hand one ref to the caller (C: lcnt++)
                Rc::into_raw(v) as *mut ::core::ffi::c_void
            }
            None => ::core::ptr::null_mut(),
        };
        assert_eq!(pthread_mutex_unlock(&raw mut glock), 0);
        token
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_cache_set(
    node: uint32_t,
    uid: uint32_t,
    gid: uint32_t,
    nleng: uint32_t,
    name: *const uint8_t,
    value: *const uint8_t,
    vleng: uint32_t,
    status: ::core::ffi::c_int,
) {
    unsafe {
        let now = monotonic_useconds();
        assert_eq!(pthread_mutex_lock(&raw mut glock), 0);
        // SAFETY: name/value point at nleng/vleng bytes per C contract.
        let key = ::core::slice::from_raw_parts(name, nleng as usize);
        let v = if value.is_null() || vleng == 0 {
            None
        } else {
            Some(::core::slice::from_raw_parts(value, vleng as usize))
        };
        cache().set(node, uid, gid, key, v, status, now);
        assert_eq!(pthread_mutex_unlock(&raw mut glock), 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_cache_del(node: uint32_t, nleng: uint32_t, name: *const uint8_t) {
    unsafe {
        assert_eq!(pthread_mutex_lock(&raw mut glock), 0);
        // SAFETY: name points at nleng bytes per C contract.
        let key = ::core::slice::from_raw_parts(name, nleng as usize);
        cache().del(node, key);
        assert_eq!(pthread_mutex_unlock(&raw mut glock), 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_cache_rel(vv: *mut ::core::ffi::c_void) {
    if vv.is_null() {
        return;
    }
    unsafe {
        assert_eq!(pthread_mutex_lock(&raw mut glock), 0);
        // SAFETY: token from xattr_cache_get (Rc::into_raw), released at
        // most once — the C lcnt-- contract.
        drop(Rc::<XattrValue>::from_raw(vv as *const XattrValue));
        assert_eq!(pthread_mutex_unlock(&raw mut glock), 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_cache_term() {
    unsafe {
        assert_eq!(pthread_mutex_lock(&raw mut glock), 0);
        if let Some(c) = (&mut *(&raw mut CACHE)).as_mut() {
            c.term();
        }
        assert_eq!(pthread_mutex_unlock(&raw mut glock), 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_cache_init(timeout: ::core::ffi::c_double) {
    unsafe {
        CACHE = Some(XattrCache::new((1_000_000.0 * timeout) as int64_t));
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;
    use std::rc::Rc;
    use std::vec;

    #[test]
    fn set_get_hit_and_fields() {
        let mut c = XattrCache::new(1_000_000);
        c.set(1, 10, 20, b"user.k", Some(b"val42"), 0, 1_000_000);
        let v = c.get(1, 10, 20, b"user.k", 1_500_000).unwrap();
        assert_eq!(v.value.as_deref(), Some(&b"val42"[..]));
        assert_eq!(v.vleng, 5);
        assert_eq!(v.status, 0);
        // wrong uid/gid/name miss
        assert!(c.get(1, 11, 20, b"user.k", 1_500_000).is_none());
        assert!(c.get(1, 10, 20, b"user.x", 1_500_000).is_none());
        // negative-answer caching (no value)
        c.set(2, 10, 20, b"user.no", None, 61, 1_000_000);
        let v = c.get(2, 10, 20, b"user.no", 1_500_000).unwrap();
        assert!(v.value.is_none());
        assert_eq!(v.status, 61);
    }

    #[test]
    fn expiry_by_absolute_utimestamp() {
        let mut c = XattrCache::new(1_000_000);
        c.set(1, 0, 0, b"a", None, 0, 1_000_000); // expires at 2_000_000
        assert!(c.get(1, 0, 0, b"a", 2_000_000).is_some()); // < not <=
        assert!(c.get(1, 0, 0, b"a", 2_000_001).is_none());
    }

    #[test]
    fn held_ref_survives_eviction() {
        let mut c = XattrCache::new(1_000_000);
        c.set(1, 0, 0, b"a", Some(b"data"), 0, 1_000_000);
        let held: Rc<XattrValue> = c.get(1, 0, 0, b"a", 1_000_000).unwrap();
        // overwrite (evicts old entry) then expire everything
        c.set(1, 0, 0, b"a", Some(b"new!"), 0, 1_500_000);
        let _ = c.get(9, 9, 9, b"zz", 9_999_999); // invalidate far future: clears cache
        assert_eq!(c.len(), 0);
        // held ref still valid (C: lcnt>0 kept the value alive)
        assert_eq!(held.value.as_deref(), Some(&b"data"[..]));
        assert_eq!(Rc::strong_count(&held), 1);
    }

    #[test]
    fn del_removes_all_uidgid_variants() {
        let mut c = XattrCache::new(1_000_000);
        c.set(1, 10, 20, b"k", None, 0, 1_000_000);
        c.set(1, 30, 40, b"k", None, 0, 1_000_000);
        c.set(1, 10, 20, b"other", None, 0, 1_000_000);
        c.del(1, b"k");
        assert!(c.get(1, 10, 20, b"k", 1_000_000).is_none());
        assert!(c.get(1, 30, 40, b"k", 1_000_000).is_none());
        assert!(c.get(1, 10, 20, b"other", 1_000_000).is_some());
    }

    #[test]
    fn lru_head_walk_stops_at_first_live() {
        let mut c = XattrCache::new(1_000_000);
        c.set(1, 0, 0, b"a", None, 0, 1_000_000); // ts 2M
        c.set(1, 0, 0, b"b", None, 0, 1_500_000); // ts 2.5M
        c.set(1, 0, 0, b"c", None, 0, 2_000_000); // ts 3M
        let _ = c.get(9, 9, 9, b"zz", 2_200_000); // invalidate at 2.2M
        assert!(c.get(1, 0, 0, b"a", 1_000_000).is_none()); // a (2M) gone
        // head walk stops at first live entry: b (2.5M) and c survive
        assert!(c.get(1, 0, 0, b"b", 1_000_000).is_some());
        assert!(c.get(1, 0, 0, b"c", 1_000_000).is_some());
        let _ = c.get(9, 9, 9, b"zz", 2_600_000); // now b goes too
        assert!(c.get(1, 0, 0, b"b", 1_000_000).is_none());
        assert!(c.get(1, 0, 0, b"c", 1_000_000).is_some());
    }
}
