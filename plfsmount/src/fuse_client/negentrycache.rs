//! Negative-entry (ENOENT) cache — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfsclient/negentrycache.c. 4-hash × 6257 × 16 cache of
//! (parent inode, name) pairs known NOT to exist, 1s-class timeout, lazy
//! expiry swept during search/remove walks, `clear()` watermark
//! (lastvalidentry) invalidating older entries lazily.
//!
//! Safe core in `imp`; boundary keeps the pthread mutex, stats tree and
//! monotonic clock. Swept-entry counts are returned to the boundary so the
//! ENTRIES gauge stays exact.

unsafe extern "C" {
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub const INSERTS: usize = 0;
pub const REMOVALS: usize = 1;
pub const SEARCH_HITS: usize = 2;
pub const SEARCH_MISSES: usize = 3;
pub const ENTRIES: usize = 4;
pub const STATNODES: usize = 5;

#[deny(unsafe_code)]
pub mod imp {
    pub const HASH_FUNCTIONS: usize = 4;
    pub const HASH_BUCKET_SIZE: usize = 16;
    pub const HASH_BUCKETS: u32 = 6257;

    const PRIMES: [u32; HASH_FUNCTIONS] = [1072573589, 3465827623, 2848548977, 748191707];

    /// rolling hash: ((inode*prime + nleng) then per byte: *prime + byte),
    /// all wrapping u32 (FAST_DATAPACK off in the C build)
    pub fn hash(n: usize, inode: u32, nleng: u8, name: &[u8]) -> u32 {
        let p = PRIMES[n];
        let mut hash = inode.wrapping_mul(p).wrapping_add(nleng as u32);
        for &b in name {
            hash = hash.wrapping_mul(p).wrapping_add(b as u32);
        }
        hash
    }

    #[derive(Clone, Default)]
    struct Slot {
        inode: u32,
        name: Option<Box<[u8]>>, // owned name bytes (no NUL), nleng = len
        time: f64,
    }

    pub enum InsertOutcome {
        Refreshed,
        Virgin,
        Evicted,
    }

    pub struct NegCache {
        buckets: Vec<[Slot; HASH_BUCKET_SIZE]>,
        pub timeout: f64,
        pub lastvalidentry: f64,
        pub enabled: bool,
    }

    impl NegCache {
        pub fn new(timeout: f64) -> Self {
            NegCache {
                buckets: (0..HASH_BUCKETS).map(|_| Default::default()).collect(),
                timeout,
                lastvalidentry: 0.0,
                enabled: timeout > 0.0,
            }
        }

        fn slot_matches(slot: &Slot, inode: u32, name: &[u8]) -> bool {
            slot.inode == inode && slot.name.as_deref() == Some(name)
        }

        /// stale = timed out or older than the clear() watermark
        fn stale(&self, slot: &Slot, t: f64) -> bool {
            slot.time > 0.0 && (slot.time + self.timeout < t || slot.time < self.lastvalidentry)
        }

        pub fn insert(&mut self, inode: u32, name: &[u8], t: f64) -> InsertOutcome {
            let mut mint = t;
            let mut found: Option<(usize, usize)> = None;
            for h in 0..HASH_FUNCTIONS {
                let b = (hash(h, inode, name.len() as u8, name) % HASH_BUCKETS) as usize;
                for i in 0..HASH_BUCKET_SIZE {
                    let slot = &self.buckets[b][i];
                    if Self::slot_matches(slot, inode, name) {
                        self.buckets[b][i].time = t;
                        return InsertOutcome::Refreshed;
                    }
                    if slot.time < mint {
                        found = Some((b, i));
                        mint = slot.time;
                    }
                }
            }
            let (b, i) = match found {
                Some(x) => x,
                None => return InsertOutcome::Refreshed, // C sanity-check no-op
            };
            let virgin = self.buckets[b][i].time == 0.0;
            self.buckets[b][i] = Slot {
                inode,
                name: Some(name.to_vec().into_boxed_slice()),
                time: t,
            };
            if virgin {
                InsertOutcome::Virgin
            } else {
                InsertOutcome::Evicted
            }
        }

        /// Remove the entry (and sweep stale entries in visited buckets).
        /// Returns (found, swept_count).
        pub fn remove(&mut self, inode: u32, name: &[u8], t: f64) -> (bool, usize) {
            let mut swept = 0;
            for h in 0..HASH_FUNCTIONS {
                let mut found_here = false;
                let b = (hash(h, inode, name.len() as u8, name) % HASH_BUCKETS) as usize;
                for i in 0..HASH_BUCKET_SIZE {
                    let is_match = Self::slot_matches(&self.buckets[b][i], inode, name);
                    if is_match {
                        found_here = true;
                    }
                    // C clears stale slots AND the matching slot itself
                    let stale = self.stale(&self.buckets[b][i], t);
                    if stale || is_match {
                        if self.buckets[b][i].time > 0.0 || self.buckets[b][i].name.is_some() {
                            if self.buckets[b][i].time > 0.0 {
                                swept += 1;
                            }
                            self.buckets[b][i] = Slot::default();
                        }
                    }
                }
                if found_here {
                    return (true, swept);
                }
            }
            (false, swept)
        }

        /// Search (sweeping stale entries in visited buckets).
        /// Returns (found, swept_count).
        pub fn search(&mut self, inode: u32, name: &[u8], t: f64) -> (bool, usize) {
            let mut swept = 0;
            for h in 0..HASH_FUNCTIONS {
                let b = (hash(h, inode, name.len() as u8, name) % HASH_BUCKETS) as usize;
                let mut found_here = false;
                for i in 0..HASH_BUCKET_SIZE {
                    if self.stale(&self.buckets[b][i], t) {
                        self.buckets[b][i] = Slot::default();
                        swept += 1;
                    } else if Self::slot_matches(&self.buckets[b][i], inode, name) {
                        found_here = true;
                    }
                }
                if found_here {
                    return (true, swept);
                }
            }
            (false, swept)
        }

        pub fn clear(&mut self, t: f64) {
            self.lastvalidentry = t;
        }

        #[cfg(test)]
        pub fn len(&self) -> usize {
            self.buckets
                .iter()
                .flatten()
                .filter(|s| s.time > 0.0)
                .count()
        }
    }
}

// ---------------------------------------------------------------------------
// Boundary: stats tree + clock.
// ---------------------------------------------------------------------------

use imp::{InsertOutcome, NegCache};

static CACHE: std::sync::Mutex<Option<NegCache>> = std::sync::Mutex::new(None);
static STATS: std::sync::OnceLock<[plfsclient::stats::StatsHandle; STATNODES]> =
    std::sync::OnceLock::new();

fn negentry_cache_statsptr_init() {
    let root = plfsclient::stats::subnode(None, "negentry_cache", false, false);
    assert!(
        STATS
            .set([
                plfsclient::stats::subnode(Some(&root), "inserts", false, true),
                plfsclient::stats::subnode(Some(&root), "removals", false, true),
                plfsclient::stats::subnode(Some(&root), "search_hits", false, true),
                plfsclient::stats::subnode(Some(&root), "search_misses", false, true),
                plfsclient::stats::subnode(Some(&root), "#entries", true, true),
            ])
            .is_ok(),
        "negative-entry stats initialized twice"
    );
}

/// SAFETY: id < STATNODES checked; statsptr valid after init.
fn stats_inc(id: usize) {
    if let Some(node) = STATS.get().and_then(|stats| stats.get(id)) {
        plfsclient::stats::counter_inc(node);
    }
}

/// SAFETY: as stats_inc.
fn stats_dec_n(id: usize, n: usize) {
    if let Some(node) = STATS.get().and_then(|stats| stats.get(id)) {
        plfsclient::stats::counter_sub(node, n as u64);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn negentry_cache_insert(
    inode: uint32_t,
    nleng: uint8_t,
    name: *const uint8_t,
) {
    unsafe {
        let t = monotonic_seconds();
        stats_inc(INSERTS);
        // SAFETY: name points at nleng bytes per C contract.
        let key = ::core::slice::from_raw_parts(name, nleng as usize);
        let outcome = {
            let mut cache = CACHE.lock().unwrap();
            match cache.as_mut() {
                Some(c) if c.enabled => c.insert(inode, key, t),
                _ => return,
            }
        };
        if let InsertOutcome::Virgin = outcome {
            stats_inc(ENTRIES);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn negentry_cache_remove(
    inode: uint32_t,
    nleng: uint8_t,
    name: *const uint8_t,
) {
    unsafe {
        let t = monotonic_seconds();
        stats_inc(REMOVALS);
        // SAFETY: name points at nleng bytes per C contract.
        let key = ::core::slice::from_raw_parts(name, nleng as usize);
        let swept = {
            let mut cache = CACHE.lock().unwrap();
            match cache.as_mut() {
                Some(c) if c.enabled => c.remove(inode, key, t).1,
                _ => return,
            }
        };
        stats_dec_n(ENTRIES, swept);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn negentry_cache_search(
    inode: uint32_t,
    nleng: uint8_t,
    name: *const uint8_t,
) -> uint8_t {
    unsafe {
        let t = monotonic_seconds();
        // SAFETY: name points at nleng bytes per C contract.
        let key = ::core::slice::from_raw_parts(name, nleng as usize);
        let (found, swept) = {
            let mut cache = CACHE.lock().unwrap();
            match cache.as_mut() {
                Some(c) if c.enabled => c.search(inode, key, t),
                _ => return 0,
            }
        };
        stats_dec_n(ENTRIES, swept);
        if found {
            stats_inc(SEARCH_HITS);
            1
        } else {
            stats_inc(SEARCH_MISSES);
            0
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn negentry_cache_clear() {
    unsafe {
        let t = monotonic_seconds();
        if let Some(c) = CACHE.lock().unwrap().as_mut() {
            c.clear(t);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn negentry_cache_init(to: ::core::ffi::c_double) {
    *CACHE.lock().unwrap() = Some(NegCache::new(to));
    if to > 0.0 {
        negentry_cache_statsptr_init();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn negentry_cache_term() {
    CACHE.lock().unwrap().take();
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;

    #[test]
    fn insert_search_refresh() {
        let mut c = NegCache::new(10.0);
        assert!(matches!(c.insert(5, b"foo", 100.0), InsertOutcome::Virgin));
        assert_eq!(c.search(5, b"foo", 101.0).0, true);
        // refresh keeps it alive
        assert!(matches!(
            c.insert(5, b"foo", 105.0),
            InsertOutcome::Refreshed
        ));
        assert_eq!(c.search(5, b"foo", 114.0).0, true);
        // different name / inode miss
        assert_eq!(c.search(5, b"bar", 105.0).0, false);
        assert_eq!(c.search(6, b"foo", 105.0).0, false);
    }

    #[test]
    fn timeout_and_watermark_sweeps() {
        let mut c = NegCache::new(10.0);
        c.insert(5, b"foo", 100.0);
        c.insert(5, b"bar", 105.0);
        // "foo" expired at t=111 (100+10 < 111); C only sweeps the buckets
        // the walk visits, so searching "foo" itself sweeps it
        let (found, swept) = c.search(5, b"foo", 111.0);
        assert!(!found);
        assert!(swept >= 1);
        // "bar" (105) still fresh
        assert!(c.search(5, b"bar", 111.0).0);
        // watermark: clear at 112; "bar" (105) older than watermark
        c.clear(112.0);
        let (found, _) = c.search(5, b"bar", 113.0);
        assert!(!found);
    }

    #[test]
    fn remove_deletes_and_sweeps() {
        let mut c = NegCache::new(10.0);
        c.insert(5, b"foo", 100.0);
        let (found, _) = c.remove(5, b"foo", 101.0);
        assert!(found);
        assert_eq!(c.search(5, b"foo", 101.0).0, false);
        let (found, _) = c.remove(5, b"foo", 101.0);
        assert!(!found);
    }

    #[test]
    fn disabled_cache_is_noop() {
        let mut c = NegCache::new(0.0);
        assert!(!c.enabled);
    }

    #[test]
    fn hash_matches_c_reference() {
        // hand-computed from the C formula, prime[0] = 1072573589
        // hash = inode*p + nleng; then per byte: hash = hash*p + b
        let p: u32 = 1072573589;
        let mut want = 5u32.wrapping_mul(p).wrapping_add(3);
        for &b in b"foo" {
            want = want.wrapping_mul(p).wrapping_add(b as u32);
        }
        assert_eq!(hash(0, 5, 3, b"foo"), want);
        assert_eq!(hash(3, 5, 3, b"foo"), {
            let p: u32 = 748191707;
            let mut h = 5u32.wrapping_mul(p).wrapping_add(3);
            for &b in b"foo" {
                h = h.wrapping_mul(p).wrapping_add(b as u32);
            }
            h
        });
    }
}
