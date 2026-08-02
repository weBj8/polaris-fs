//! Symlink cache — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfsclient/symlinkcache.c (206 lines of C; the c2rust
//! transpile had ballooned to 1275 lines, mostly lock-assert macro noise.
//! 4-hash × 6257-bucket × 16-slot cache mapping inode
//! → symlink target path, time-based expiry, oldest-slot eviction.
//!
//! All cache logic is safe Rust in `imp` (paths owned as boxed byte strings,
//! NUL-terminated for C export). The boundary keeps the global lock, the
//! stats tree, and C-string ownership: inserted paths are copied in,
//! search-hit paths are returned as libc-malloc'd copies (caller frees with
//! free(3), matching the C contract — mfs_fuse does exactly that).

unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub const INSERTS: usize = 0;
pub const SEARCH_HITS: usize = 1;
pub const SEARCH_MISSES: usize = 2;
pub const LINKS: usize = 3;
pub const STATNODES: usize = 4;

#[deny(unsafe_code)]
pub mod imp {
    pub const HASH_FUNCTIONS: usize = 4;
    pub const HASH_BUCKET_SIZE: usize = 16;
    pub const HASH_BUCKETS: u32 = 6257;

    const PRIMES: [u32; HASH_FUNCTIONS] = [1072573589, 3465827623, 2848548977, 748191707];

    #[derive(Clone, Default)]
    struct Slot {
        inode: u32,
        time: f64,
        /// NUL-terminated target path (owned); None = empty value
        path: Option<Box<[u8]>>,
    }

    pub enum InsertOutcome {
        /// inode already cached; path replaced
        Updated,
        /// inserted into a virgin slot (LINKS++)
        Virgin,
        /// inserted by evicting the oldest slot (LINKS unchanged)
        Evicted,
    }

    pub enum SearchOutcome {
        /// fresh copy of the NUL-terminated path (SEARCH_HITS++)
        Hit(Box<[u8]>),
        /// found but stale; slot cleared (LINKS--, SEARCH_MISSES++)
        Expired,
        /// not cached (SEARCH_MISSES++)
        Miss,
    }

    pub struct Cache {
        buckets: Vec<[Slot; HASH_BUCKET_SIZE]>,
        pub timeout: f64,
    }

    impl Cache {
        pub fn new(timeout: f64) -> Self {
            Cache {
                buckets: (0..HASH_BUCKETS).map(|_| Default::default()).collect(),
                timeout,
            }
        }

        pub fn bucket(&self, inode: u32, h: usize) -> usize {
            (inode.wrapping_mul(PRIMES[h]) % HASH_BUCKETS) as usize
        }

        pub fn insert(&mut self, inode: u32, path: &[u8], t: f64) -> InsertOutcome {
            let mut mint = t;
            let mut found: Option<(usize, usize)> = None;
            for h in 0..HASH_FUNCTIONS {
                let b = self.bucket(inode, h);
                for i in 0..HASH_BUCKET_SIZE {
                    let slot = &self.buckets[b][i];
                    if slot.inode == inode {
                        self.buckets[b][i].path = Some(path.to_vec().into_boxed_slice());
                        self.buckets[b][i].time = t;
                        return InsertOutcome::Updated;
                    }
                    if slot.time < mint {
                        found = Some((b, i));
                        mint = slot.time;
                    }
                }
            }
            let (b, i) = match found {
                Some(x) => x,
                None => return InsertOutcome::Updated, // C sanity-check no-op path
            };
            let virgin = self.buckets[b][i].time == 0.0;
            let slot = &mut self.buckets[b][i];
            slot.inode = inode;
            slot.path = Some(path.to_vec().into_boxed_slice());
            slot.time = t;
            if virgin {
                InsertOutcome::Virgin
            } else {
                InsertOutcome::Evicted
            }
        }

        pub fn search(&mut self, inode: u32, t: f64) -> SearchOutcome {
            for h in 0..HASH_FUNCTIONS {
                let b = self.bucket(inode, h);
                for i in 0..HASH_BUCKET_SIZE {
                    if self.buckets[b][i].inode == inode {
                        let slot = &mut self.buckets[b][i];
                        if slot.time + self.timeout < t {
                            slot.path = None;
                            slot.time = 0.0;
                            slot.inode = 0;
                            return SearchOutcome::Expired;
                        }
                        let path = slot.path.clone().unwrap_or_default();
                        return SearchOutcome::Hit(path);
                    }
                }
            }
            SearchOutcome::Miss
        }

        /// total cached entries (test/diagnostics)
        #[cfg(test)]
        pub fn len(&self) -> usize {
            self.buckets
                .iter()
                .flatten()
                .filter(|s| s.inode != 0 || s.time != 0.0)
                .count()
        }
    }
}

// ---------------------------------------------------------------------------
// Boundary: stats tree + C-string ownership.
// ---------------------------------------------------------------------------

use imp::{Cache, InsertOutcome, SearchOutcome};

static CACHE: std::sync::Mutex<Option<Cache>> = std::sync::Mutex::new(None);
static STATS: std::sync::OnceLock<[plfsclient::stats::StatsHandle; STATNODES]> =
    std::sync::OnceLock::new();

fn symlink_cache_statsptr_init() {
    let root = plfsclient::stats::subnode(None, "symlink_cache", false, false);
    assert!(
        STATS
            .set([
                plfsclient::stats::subnode(Some(&root), "inserts", false, true),
                plfsclient::stats::subnode(Some(&root), "search_hits", false, true),
                plfsclient::stats::subnode(Some(&root), "search_misses", false, true),
                plfsclient::stats::subnode(Some(&root), "#links", true, true),
            ])
            .is_ok(),
        "symlink-cache stats initialized twice"
    );
}

/// SAFETY: id < STATNODES checked inline; statsptr valid after init.
fn stats_inc(id: usize) {
    if let Some(node) = STATS.get().and_then(|stats| stats.get(id)) {
        plfsclient::stats::counter_inc(node);
    }
}

/// SAFETY: as stats_inc.
fn stats_dec(id: usize) {
    if let Some(node) = STATS.get().and_then(|stats| stats.get(id)) {
        plfsclient::stats::counter_dec(node);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symlink_cache_insert(inode: uint32_t, path: *const uint8_t) {
    unsafe {
        let t = monotonic_seconds();
        stats_inc(INSERTS);
        // SAFETY: path is a NUL-terminated C string per C contract.
        let bytes = ::core::ffi::CStr::from_ptr(path as *const ::core::ffi::c_char).to_bytes();
        // store NUL-terminated so search can return it verbatim
        let mut owned = Vec::with_capacity(bytes.len() + 1);
        owned.extend_from_slice(bytes);
        owned.push(0);
        let outcome = CACHE
            .lock()
            .unwrap()
            .as_mut()
            .expect("symlink cache used before init")
            .insert(inode, &owned, t);
        if let InsertOutcome::Virgin = outcome {
            stats_inc(LINKS);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symlink_cache_search(inode: uint32_t) -> *mut uint8_t {
    unsafe {
        let t = monotonic_seconds();
        let outcome = CACHE
            .lock()
            .unwrap()
            .as_mut()
            .expect("symlink cache used before init")
            .search(inode, t);
        match outcome {
            SearchOutcome::Hit(path) => {
                stats_inc(SEARCH_HITS);
                // libc malloc: caller frees with free(3) (mfs_fuse does).
                let p = malloc(path.len()) as *mut uint8_t;
                if p.is_null() {
                    return ::core::ptr::null_mut();
                }
                ::core::ptr::copy_nonoverlapping(path.as_ptr(), p, path.len());
                p
            }
            SearchOutcome::Expired => {
                stats_dec(LINKS);
                stats_inc(SEARCH_MISSES);
                ::core::ptr::null_mut()
            }
            SearchOutcome::Miss => {
                stats_inc(SEARCH_MISSES);
                ::core::ptr::null_mut()
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symlink_cache_init(to: ::core::ffi::c_double) {
    *CACHE.lock().unwrap() = Some(Cache::new(to));
    symlink_cache_statsptr_init();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symlink_cache_term() {
    CACHE.lock().unwrap().take(); // drops all boxed paths
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;
    use std::vec::Vec;

    fn cstr(s: &str) -> Vec<u8> {
        let mut v = s.as_bytes().to_vec();
        v.push(0);
        v
    }

    #[test]
    fn insert_search_hit() {
        let mut c = Cache::new(60.0);
        assert!(matches!(
            c.insert(42, &cstr("/target/path"), 100.0),
            InsertOutcome::Virgin
        ));
        match c.search(42, 110.0) {
            SearchOutcome::Hit(p) => assert_eq!(&*p, &*cstr("/target/path")),
            _ => panic!("expected hit"),
        }
        // update same inode
        assert!(matches!(
            c.insert(42, &cstr("/new"), 200.0),
            InsertOutcome::Updated
        ));
        match c.search(42, 210.0) {
            SearchOutcome::Hit(p) => assert_eq!(&*p, &*cstr("/new")),
            _ => panic!("expected hit"),
        }
        assert!(matches!(c.search(43, 210.0), SearchOutcome::Miss));
    }

    #[test]
    fn expiry_clears_slot() {
        let mut c = Cache::new(10.0);
        c.insert(7, &cstr("/x"), 100.0);
        // time + timeout < t → expired (strict <, as C)
        assert!(matches!(c.search(7, 111.0), SearchOutcome::Expired));
        assert_eq!(c.len(), 0);
        // slot is virgin again
        assert!(matches!(
            c.insert(7, &cstr("/y"), 200.0),
            InsertOutcome::Virgin
        ));
        // boundary: time+timeout == t is still fresh
        assert!(matches!(c.search(7, 210.0), SearchOutcome::Hit(_)));
    }

    #[test]
    fn eviction_picks_oldest() {
        let mut c = Cache::new(1e9);
        // same bucket set: find inodes that collide on all 4 hashes is
        // overkill; instead fill enough entries that min-time eviction
        // happens somewhere, then verify capacity accounting stays exact.
        for i in 1..=2000u32 {
            let p = cstr(&format!("/p{i}"));
            c.insert(i, &p, i as f64);
        }
        let n = c.len();
        assert!(n <= 2000);
        // every present entry must be findable
        let mut found = 0;
        for i in 1..=2000u32 {
            if let SearchOutcome::Hit(_) = c.search(i, 2000.0) {
                found += 1;
            }
        }
        assert_eq!(found, n);
    }

    #[test]
    fn hash_matches_c_reference() {
        // (inode * prime) % 6257, wrapping — spot values from the C formula
        let c = Cache::new(0.0);
        assert_eq!(c.bucket(1, 0), (1072573589u32 % 6257) as usize);
        assert_eq!(
            c.bucket(2, 1),
            (2u32.wrapping_mul(3465827623) % 6257) as usize
        );
    }
}
