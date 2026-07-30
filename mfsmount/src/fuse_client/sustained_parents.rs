//! Sustained parents cache — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfsclient/sustained_parents.c. inode → parent map with
//! per-entry validity timeout; each hash bucket capped at 20 entries with
//! oldest-validtime eviction; cleanup thread sweeps ~54 buckets per 100ms.
//!
//! Safe core in `imp` (per-bucket std Mutex + Vec), boundary keeps the
//! cleanup thread and monotonic clock. term is a real AtomicU8.

unsafe extern "C" {
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn lwt_minthread_create(
        th: *mut pthread_t,
        detached: uint8_t,
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
}

// local copy, mirroring the C static-inline portable_usleep (each TU had
// its own; cross-module private symbols do not survive clean LTO builds)
#[derive(Copy, Clone)]
#[repr(C)]
struct portable_timespec {
    tv_sec: ::core::ffi::c_long,
    tv_nsec: ::core::ffi::c_long,
}
unsafe extern "C" {
    fn nanosleep(
        __requested_time: *const portable_timespec,
        __remaining: *mut portable_timespec,
    ) -> ::core::ffi::c_int;
}
#[inline]
fn portable_usleep(usec: uint64_t) {
    let mut req = portable_timespec {
        tv_sec: (usec / 1_000_000) as ::core::ffi::c_long,
        tv_nsec: ((usec % 1_000_000) * 1000) as ::core::ffi::c_long,
    };
    let mut rem = portable_timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: req/rem are valid stack structs.
    unsafe {
        loop {
            let s = nanosleep(&req, &mut rem);
            if s < 0 {
                req = rem;
            } else {
                break;
            }
        }
    }
}

pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type pthread_t = ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

#[deny(unsafe_code)]
pub mod imp {
    use std::sync::Mutex;

    pub const HASH_SIZE: usize = 16384;
    pub const MAX_LIST_LENGTH: usize = 20;
    /// buckets swept per cleanup cycle: HASH_SIZE/(10*30), integer division
    pub const CLEANUP_PER_CYCLE: usize = HASH_SIZE / (10 * 30);

    #[derive(Copy, Clone)]
    struct Entry {
        inode: u32,
        parent: u32,
        validtime: u32,
    }

    pub struct SParents {
        buckets: Vec<Mutex<Vec<Entry>>>,
    }

    impl SParents {
        pub fn new() -> Self {
            SParents {
                buckets: (0..HASH_SIZE).map(|_| Mutex::new(Vec::new())).collect(),
            }
        }

        /// C: validtime = (uint32_t)(monotonic_seconds() + timeout)
        pub fn add(&self, inode: u32, parent: u32, timeout: u32, now: f64) {
            let validtime = (now + timeout as f64) as u32;
            let mut b = self.buckets[(inode as usize) % HASH_SIZE].lock().unwrap();
            if let Some(e) = b.iter_mut().find(|e| e.inode == inode) {
                e.parent = parent;
                e.validtime = validtime;
                return;
            }
            if b.len() >= MAX_LIST_LENGTH {
                // replace the oldest-validtime entry in place
                let oldest = b
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, e)| e.validtime)
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                b[oldest] = Entry {
                    inode,
                    parent,
                    validtime,
                };
            } else {
                b.push(Entry {
                    inode,
                    parent,
                    validtime,
                });
            }
        }

        /// 0 when unknown (no validity check on read, as C)
        pub fn get(&self, inode: u32) -> u32 {
            let b = self.buckets[(inode as usize) % HASH_SIZE].lock().unwrap();
            b.iter().find(|e| e.inode == inode).map(|e| e.parent).unwrap_or(0)
        }

        /// Drop entries with validtime < current_time; now == u32::MAX
        /// clears the bucket completely (term path).
        pub fn cleanup(&self, hash: usize, current_time: u32) {
            let mut b = self.buckets[hash].lock().unwrap();
            if current_time == u32::MAX {
                b.clear();
            } else {
                b.retain(|e| e.validtime >= current_time);
            }
        }

        #[cfg(test)]
        pub fn bucket_len(&self, hash: usize) -> usize {
            self.buckets[hash].lock().unwrap().len()
        }
    }
}

use imp::SParents;

static mut SPARENTS: Option<SParents> = None;
static TERM: ::core::sync::atomic::AtomicU8 = ::core::sync::atomic::AtomicU8::new(0);
static mut clthread: pthread_t = 0;

/// SAFETY: set once in sparents_init before any thread can call in; never
/// cleared (C also kept the tables until process exit).
unsafe fn sparents() -> &'static SParents {
    unsafe { (*(&raw const SPARENTS)).as_ref().unwrap_unchecked() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sparents_add(inode: uint32_t, parent: uint32_t, timeout: uint32_t) {
    unsafe {
        sparents().add(inode, parent, timeout, monotonic_seconds());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sparents_get(inode: uint32_t) -> uint32_t {
    unsafe { sparents().get(inode) }
}

unsafe extern "C" fn sparents_cleanupthread(_arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut cuhashpos: usize = 0;
        loop {
            let current_time = monotonic_seconds() as uint32_t;
            for _ in 0..imp::CLEANUP_PER_CYCLE {
                sparents().cleanup(cuhashpos, current_time);
                cuhashpos = (cuhashpos + 1) % imp::HASH_SIZE;
            }
            portable_usleep(100000);
            if TERM.load(::core::sync::atomic::Ordering::SeqCst) == 1 {
                return NULL;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sparents_term() {
    unsafe {
        TERM.store(1, ::core::sync::atomic::Ordering::SeqCst);
        pthread_join(
            clthread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        for hash in 0..imp::HASH_SIZE {
            sparents().cleanup(hash, u32::MAX);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sparents_init() {
    unsafe {
        SPARENTS = Some(SParents::new());
        TERM.store(0, ::core::sync::atomic::Ordering::SeqCst);
        lwt_minthread_create(&raw mut clthread, 0, Some(sparents_cleanupthread), NULL);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;
    use std::vec::Vec;

    #[test]
    fn add_get_update() {
        let s = SParents::new();
        s.add(10, 100, 60, 1000.0);
        assert_eq!(s.get(10), 100);
        assert_eq!(s.get(11), 0);
        s.add(10, 200, 60, 1001.0);
        assert_eq!(s.get(10), 200);
    }

    #[test]
    fn bucket_cap_evicts_oldest_validtime() {
        let s = SParents::new();
        // same bucket: inodes 0, HASH_SIZE, 2*HASH_SIZE, ...
        let mut live: Vec<u32> = Vec::new();
        for i in 0..MAX_LIST_LENGTH as u32 {
            let inode = i * HASH_SIZE as u32;
            s.add(inode, 1000 + i, 60, 100.0 + i as f64); // increasing validtime
            live.push(inode);
        }
        assert_eq!(s.bucket_len(0), MAX_LIST_LENGTH);
        // add one more with the largest validtime → evicts inode 0 (oldest)
        let newi = (MAX_LIST_LENGTH as u32) * HASH_SIZE as u32;
        s.add(newi, 9999, 60, 1000.0);
        assert_eq!(s.bucket_len(0), MAX_LIST_LENGTH);
        assert_eq!(s.get(0), 0); // evicted
        assert_eq!(s.get(newi), 9999);
        assert_eq!(s.get(HASH_SIZE as u32), 1001); // second-oldest survives
    }

    #[test]
    fn cleanup_respects_validtime() {
        let s = SParents::new();
        s.add(1, 10, 10, 100.0); // validtime 110
        s.add(2, 20, 50, 100.0); // validtime 150
        s.cleanup(1, 120);
        s.cleanup(2, 120);
        assert_eq!(s.get(1), 0);
        assert_eq!(s.get(2), 20);
        // term clear-all
        s.cleanup(2, u32::MAX);
        assert_eq!(s.get(2), 0);
    }

    #[test]
    fn validtime_truncates_like_c() {
        let s = SParents::new();
        s.add(1, 10, 60, 1000.9); // (u32)(1060.9) = 1060
        s.cleanup(1, 1060); // validtime 1060 >= 1060 → kept (C: < drops)
        assert_eq!(s.get(1), 10);
        s.cleanup(1, 1061);
        assert_eq!(s.get(1), 0);
    }
}
