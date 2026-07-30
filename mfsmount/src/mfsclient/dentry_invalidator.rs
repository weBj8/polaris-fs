//! Dentry invalidator — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfsclient/dentry_invalidator.c. Time-ordered queue +
//! hash of (parent,name)→inode dentries the kernel should forget; reaper
//! thread (10ms) expires entries, re-queueing ones whose inode is still
//! open (fs_isopen), calling mfs_dentry_invalidate (kernel notify) with
//! the global lock RELEASED.
//!
//! Safe core in `imp`: index-slab doubly linked queue (same time-ordered
//! semantics as the C intrusive pointers) + HashMap keyed by
//! (parent, name). The reaper loop lives at the boundary: it takes one
//! core step per iteration and performs the unlocked kernel callback
//! between steps, exactly like C.

unsafe extern "C" {
    unsafe fn lwt_minthread_create(
        th: *mut pthread_t,
        detached: uint8_t,
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn portable_usleep(usec: uint64_t);
    unsafe fn fs_isopen(inode: uint32_t) -> ::core::ffi::c_int;
    unsafe fn mfs_dentry_invalidate(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const ::core::ffi::c_char,
    );
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type pthread_t = ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

#[deny(unsafe_code)]
pub mod imp {
    use std::collections::{HashMap, VecDeque};
    use std::sync::Mutex;

    pub const MAX_ELEMENTS: usize = 10000;
    pub const MIN_TIMEOUT: f64 = 30.0;
    /// heads processed per reaper cycle
    pub const REAP_PER_CYCLE: usize = 100;

    struct Entry {
        inode: u32,
        ts: f64,
    }

    pub enum ReapAction {
        /// queue empty or head not expired
        Stop,
        /// inode still open; entry re-queued at tail with fresh timestamp
        Requeued,
        /// entry dropped; boundary must call mfs_dentry_invalidate with
        /// the lock released (parent, name, inode)
        Invalidate(u32, Box<[u8]>, u32),
    }

    pub struct Dinval {
        map: HashMap<(u32, Box<[u8]>), Entry>,
        /// time-ordered queue nodes; a node is stale when the map holds a
        /// newer timestamp for the key (refresh requeues instead of
        /// detaching — stale nodes are dropped when they reach the head)
        queue: VecDeque<(u32, Box<[u8]>, f64)>,
        timeout: f64,
    }

    impl Dinval {
        pub fn new(timeout: f64) -> Self {
            Dinval {
                map: HashMap::new(),
                queue: VecDeque::new(),
                timeout: if timeout > MIN_TIMEOUT {
                    timeout
                } else {
                    MIN_TIMEOUT
                },
            }
        }

        pub fn add(&mut self, parent: u32, name: &[u8], inode: u32, now: f64) {
            let key = (parent, name.to_vec().into_boxed_slice());
            self.map
                .entry(key.clone())
                .and_modify(|e| {
                    e.inode = inode;
                    e.ts = now;
                })
                .or_insert(Entry { inode, ts: now });
            self.queue.push_back((key.0, key.1, now));
        }

        pub fn remove(&mut self, parent: u32, name: &[u8]) -> bool {
            // queue nodes for this key go stale and are dropped at the head
            self.map.remove(&(parent, name.into())).is_some()
        }

        /// One head step of the reaper loop (C: one iteration of the
        /// 100-step inner while, i decremented only for real heads — stale
        /// nodes are a Rust-side artifact and skipped freely).
        pub fn reap_step(&mut self, now: f64, is_open: &dyn Fn(u32) -> bool) -> ReapAction {
            loop {
                let (parent, name, ts) = match self.queue.front() {
                    Some((p, n, t)) => (*p, n.clone(), *t),
                    None => return ReapAction::Stop,
                };
                let over_cap = self.map.len() > MAX_ELEMENTS && ts + MIN_TIMEOUT < now;
                let expired = ts + self.timeout < now;
                if !over_cap && !expired {
                    return ReapAction::Stop;
                }
                self.queue.pop_front();
                let key = (parent, name);
                let cur = match self.map.get(&key) {
                    Some(e) if e.ts == ts => e.inode,
                    _ => continue, // stale node (refreshed or removed)
                };
                if is_open(cur) {
                    self.map.get_mut(&key).unwrap().ts = now;
                    self.queue.push_back((key.0, key.1.clone(), now));
                    return ReapAction::Requeued;
                }
                self.map.remove(&key);
                return ReapAction::Invalidate(key.0, key.1, cur);
            }
        }

        #[cfg(test)]
        pub fn len(&self) -> usize {
            self.map.len()
        }
    }

    /// The shared instance: one Mutex for the whole module (C: glock).
    pub static DI: Mutex<Option<Dinval>> = Mutex::new(None);
}

// ---------------------------------------------------------------------------
// Boundary: reaper thread + kernel callback with lock released.
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dinval_add(
    parent: uint32_t,
    nleng: uint8_t,
    name: *const uint8_t,
    inode: uint32_t,
) {
    unsafe {
        let now = monotonic_seconds();
        // SAFETY: name points at nleng bytes per C contract.
        let key = ::core::slice::from_raw_parts(name, nleng as usize);
        let mut g = imp::DI.lock().unwrap();
        if let Some(d) = g.as_mut() {
            d.add(parent, key, inode, now);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dinval_remove(parent: uint32_t, nleng: uint8_t, name: *const uint8_t) {
    unsafe {
        // SAFETY: name points at nleng bytes per C contract.
        let key = ::core::slice::from_raw_parts(name, nleng as usize);
        let mut g = imp::DI.lock().unwrap();
        if let Some(d) = g.as_mut() {
            d.remove(parent, key);
        }
    }
}

unsafe extern "C" fn dinval_invalthread(arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    unsafe {
        loop {
            let now = monotonic_seconds();
            for _ in 0..imp::REAP_PER_CYCLE {
                let action = {
                    let mut g = imp::DI.lock().unwrap();
                    match g.as_mut() {
                        Some(d) => d.reap_step(now, &|ino| fs_isopen(ino) != 0),
                        None => imp::ReapAction::Stop,
                    }
                };
                match action {
                    imp::ReapAction::Stop => break,
                    imp::ReapAction::Requeued => {}
                    imp::ReapAction::Invalidate(parent, name, _inode) => {
                        // lock released here, as C — kernel callback may
                        // re-enter the filesystem
                        let mut cname = Vec::with_capacity(name.len() + 1);
                        cname.extend_from_slice(&name);
                        cname.push(0);
                        mfs_dentry_invalidate(
                            parent,
                            name.len() as uint8_t,
                            cname.as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                }
            }
            portable_usleep(10000);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dinval_init(timeout: ::core::ffi::c_double) {
    unsafe {
        {
            let mut g = imp::DI.lock().unwrap();
            *g = Some(imp::Dinval::new(timeout));
        }
        let mut th: pthread_t = 0;
        lwt_minthread_create(&raw mut th, 1, Some(dinval_invalthread), NULL);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;

    #[test]
    fn add_refresh_remove() {
        let mut d = Dinval::new(60.0);
        d.add(1, b"foo", 100, 1000.0);
        d.add(1, b"foo", 101, 1005.0); // refresh, new inode
        assert_eq!(d.len(), 1);
        assert!(d.remove(1, b"foo"));
        assert!(!d.remove(1, b"foo"));
        assert_eq!(d.len(), 0);
        // stale queue node from the refresh is dropped harmlessly
        assert!(matches!(
            d.reap_step(100000.0, &|_| false),
            ReapAction::Stop
        ));
    }

    #[test]
    fn expiry_and_invalidate_order() {
        let mut d = Dinval::new(30.0);
        d.add(1, b"a", 10, 100.0);
        d.add(1, b"b", 11, 102.0);
        d.add(1, b"c", 12, 104.0);
        // timeout 30: "a" expires at 130.1
        match d.reap_step(130.1, &|_| false) {
            ReapAction::Invalidate(p, n, i) => {
                assert_eq!(p, 1);
                assert_eq!(&*n, b"a");
                assert_eq!(i, 10);
            }
            _ => panic!("expected invalidate"),
        }
        assert_eq!(d.len(), 2);
        // "b" not yet expired
        assert!(matches!(d.reap_step(130.1, &|_| false), ReapAction::Stop));
    }

    #[test]
    fn open_inode_requeues_at_tail() {
        let mut d = Dinval::new(30.0);
        d.add(1, b"a", 10, 100.0);
        d.add(1, b"b", 11, 101.0);
        // "a" expired but open → requeued with fresh ts
        assert!(matches!(
            d.reap_step(200.0, &|ino| ino == 10),
            ReapAction::Requeued
        ));
        assert_eq!(d.len(), 2);
        // next head is "b" (also expired, not open) → invalidated
        match d.reap_step(200.0, &|ino| ino == 10) {
            ReapAction::Invalidate(_, n, i) => {
                assert_eq!(&*n, b"b");
                assert_eq!(i, 11);
            }
            _ => panic!("expected invalidate"),
        }
        // "a" now fresh at 200 → stops
        assert!(matches!(
            d.reap_step(200.0, &|_| false),
            ReapAction::Stop
        ));
    }

    #[test]
    fn over_cap_uses_min_timeout() {
        let mut d = Dinval::new(1e9); // huge timeout
        for i in 0..(MAX_ELEMENTS + 1) as u32 {
            d.add(1, &i.to_be_bytes(), i, 100.0 + i as f64);
        }
        assert_eq!(d.len(), MAX_ELEMENTS + 1);
        // head ts=100, over cap, MIN_TIMEOUT=30 → expired at 130.1
        match d.reap_step(130.1, &|_| false) {
            ReapAction::Invalidate(_, _, i) => assert_eq!(i, 0),
            _ => panic!("expected over-cap eviction"),
        }
    }
}
