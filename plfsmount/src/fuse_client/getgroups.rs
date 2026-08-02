//! Supplementary-groups cache — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfsclient/getgroups.c. Cache of /proc/<pid>/status
//! "Groups:" lookups keyed by (pid,uid,gid), entries expiring after a
//! timeout, a reaper thread, and a cacheonly "emergency mode" that may
//! return stale entries. Results are handed out as C-layout `groups`
//! blobs (lcnt/gidcnt/gidtab, gids inline after the header) that the
//! caller frees via groups_rel.
//!
//! Safe core in `imp`: the /proc line parser (pure, reference-tested) and
//! the cache decision tree. The boundary keeps the C blob ABI, the lcnt
//! refcount protocol and the reaper thread. Two deliberate deviations,
//! externally invisible: C swept expired same-bucket neighbors during a
//! lookup (here only the reaper sweeps), and C stored the blob pointer in
//! the cache entry (here the entry stores it as `usize` so `imp` stays
//! free of unsafe) — the refcounting itself is behavior and is preserved
//! exactly: caller ref (lcnt=1 at make), +1 while cached, free at 0.
//! mfs_fuse RELIES on the cache ref: it reads a groups blob after
//! groups_rel (opendir gidtab copy), valid only because the cache keeps
//! the blob alive.

use std::sync::Mutex as StdMutex;

unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type pid_t = ::core::ffi::c_int;
pub type uid_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;

/// C-layout result blob (getgroups.h); gidtab points right after header.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct groups {
    pub lcnt: uint32_t,
    pub gidcnt: uint32_t,
    pub gidtab: *mut uint32_t,
}

#[deny(unsafe_code)]
pub mod imp {
    use std::collections::HashMap;

    /// Parse a /proc/<pid>/status "Groups:" line with the C ptr-walk
    /// semantics: skip blanks, strtoul each number, stop at the first
    /// non-blank non-digit. Returns [gid, ...supplementary] (gid first,
    /// duplicates of gid excluded), or None if the line doesn't parse
    /// (caller falls back to [gid]).
    pub fn parse_groups_line(line: &str, gid: u32) -> Vec<u32> {
        let mut out = Vec::new();
        let b = line.as_bytes();
        let mut i = 0usize;
        loop {
            while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
                i += 1;
            }
            if i < b.len() && b[i].is_ascii_digit() {
                let mut g: u64 = 0;
                while i < b.len() && b[i].is_ascii_digit() {
                    g = g * 10 + (b[i] - b'0') as u64; // strtoul semantics
                    i += 1;
                }
                if g as u32 != gid {
                    out.push(g as u32);
                }
            } else {
                break;
            }
        }
        let mut ret = Vec::with_capacity(out.len() + 1);
        ret.push(gid);
        ret.extend(out);
        ret
    }

    /// Cache entry. `blob` is a `*mut groups` stored as usize (imp is
    /// unsafe-free); the cache holds ONE refcount of the blob, released
    /// on replace/sweep/term via the boundary's blob_decref.
    struct Entry {
        time: f64,
        blob: usize,
    }

    /// Outcome of the groups_get_common decision tree. Refcount side
    /// effects happen at the boundary under the same lock:
    /// - Cached(blob): caller must incref before use.
    /// - Emergency: cacheonly miss — caller builds an UNCACHED [gid] blob.
    /// - Fetched(new, old): new blob from `fetch` (lcnt=1, caller ref) was
    ///   stored; caller must incref for the cache and decref `old` (the
    ///   replaced entry's cache ref), C-order.
    pub enum GetResult {
        Cached(usize),
        Emergency,
        Fetched(usize, Option<usize>),
    }

    pub struct GroupCache {
        map: HashMap<(i32, u32, u32), Entry>,
        pub timeout: f64,
    }

    impl GroupCache {
        pub fn new(timeout: f64) -> Self {
            GroupCache {
                map: HashMap::new(),
                timeout,
            }
        }

        /// The groups_get_common decision tree. `fetch` (boundary) reads
        /// /proc and returns a fresh blob with lcnt=1 as usize.
        pub fn get_common(
            &mut self,
            pid: i32,
            uid: u32,
            gid: u32,
            cacheonly: bool,
            now: f64,
            fetch: &dyn Fn(i32, u32) -> usize,
        ) -> GetResult {
            let key = (pid, uid, gid);
            let fresh = self
                .map
                .get(&key)
                .map(|e| e.time + self.timeout >= now)
                .unwrap_or(false);
            if cacheonly {
                // emergency mode: any cached entry (even stale) wins
                return match self.map.get(&key) {
                    Some(e) => GetResult::Cached(e.blob),
                    None => GetResult::Emergency,
                };
            }
            // expired entries are treated as missing (reaper sweeps them)
            if uid != 0 && fresh {
                return GetResult::Cached(self.map.get(&key).unwrap().blob);
            }
            // root always refetches; non-root refetches on miss/stale
            let blob = fetch(pid, gid);
            let old = self
                .map
                .insert(key, Entry { time: now, blob }) // C stamps pre-fetch time
                .map(|e| e.blob);
            GetResult::Fetched(blob, old)
        }

        /// reaper sweep: drop expired entries, returning their blobs so
        /// the boundary can release the cache refs.
        pub fn sweep(&mut self, now: f64) -> Vec<usize> {
            let to = self.timeout;
            let mut out = Vec::new();
            self.map.retain(|_, e| {
                let keep = e.time + to >= now;
                if !keep {
                    out.push(e.blob);
                }
                keep
            });
            out
        }

        pub fn term(&mut self) -> Vec<usize> {
            self.map.drain().map(|(_, e)| e.blob).collect()
        }

        #[cfg(test)]
        pub fn len(&self) -> usize {
            self.map.len()
        }
    }
}

// ---------------------------------------------------------------------------
// Boundary: C blob ABI, /proc IO, reaper thread.
// ---------------------------------------------------------------------------

use imp::{GetResult, GroupCache};

static CACHE: StdMutex<Option<GroupCache>> = StdMutex::new(None);
static KEEP_ALIVE: ::core::sync::atomic::AtomicU8 = ::core::sync::atomic::AtomicU8::new(0);
static THREAD: StdMutex<Option<std::thread::JoinHandle<()>>> = StdMutex::new(None);
static DEBUG_MODE: ::core::sync::atomic::AtomicI32 = ::core::sync::atomic::AtomicI32::new(0);

/// Fetch supplementary groups from /proc (Linux path of the C original).
/// Fallback [gid] when unreadable/unparseable, exactly like C.
fn fetch_groups(pid: pid_t, gid: gid_t) -> Vec<u32> {
    let path = format!("/proc/{pid}/status");
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return vec![gid],
    };
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("Groups:") {
            return imp::parse_groups_line(rest, gid);
        }
    }
    vec![gid]
}

/// Build the C-layout blob: header + inline gid array, libc-malloc'd
/// (caller releases with groups_rel → free(3)).
/// SAFETY: gids non-empty (always at least the primary gid).
unsafe fn make_blob(gids: &[u32]) -> *mut groups {
    unsafe {
        let n = gids.len();
        let bytes = ::core::mem::size_of::<groups>() + n * 4;
        let p = malloc(bytes) as *mut groups;
        if p.is_null() {
            return ::core::ptr::null_mut();
        }
        (*p).lcnt = 1;
        (*p).gidcnt = n as uint32_t;
        let tab = (p as *mut uint8_t).add(::core::mem::size_of::<groups>()) as *mut uint32_t;
        ::core::ptr::copy_nonoverlapping(gids.as_ptr(), tab, n);
        (*p).gidtab = tab;
        p
    }
}

/// C groups_decref: lcnt-- (guarded), free at 0. Under CACHE lock at
/// every call site, mirroring C's glock.
/// SAFETY: b is a live groups blob from make_blob.
unsafe fn blob_decref(b: *mut groups) {
    unsafe {
        if (*b).lcnt > 0 {
            (*b).lcnt -= 1;
        }
        if (*b).lcnt == 0 {
            free(b as *mut ::core::ffi::c_void);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_get_common(
    pid: pid_t,
    uid: uid_t,
    gid: gid_t,
    cacheonly: uint8_t,
) -> *mut groups {
    unsafe {
        let now = monotonic_seconds();
        let mut g = CACHE.lock().unwrap();
        let res = match g.as_mut() {
            Some(c) => c.get_common(pid, uid, gid, cacheonly != 0, now, &|p, gg| {
                let gids = fetch_groups(p, gg);
                make_blob(&gids) as usize
            }),
            None => GetResult::Emergency,
        };
        let blob = match res {
            GetResult::Cached(b) => {
                let b = b as *mut groups;
                (*b).lcnt += 1; // caller ref
                b
            }
            GetResult::Emergency => make_blob(&[gid]),
            GetResult::Fetched(b, old) => {
                let b = b as *mut groups;
                (*b).lcnt += 1; // cache ref
                if let Some(o) = old {
                    blob_decref(o as *mut groups);
                }
                b
            }
        };
        if DEBUG_MODE.load(::core::sync::atomic::Ordering::Relaxed) != 0 {
            eprintln!(
                "groups_get(pid={pid},uid={uid},gid={gid}): gidcnt={} lcnt={}",
                (*blob).gidcnt,
                (*blob).lcnt
            );
        }
        blob
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_rel(g: *mut groups) {
    if !g.is_null() {
        // C protocol: decref under glock; the blob dies only when the
        // cache AND all callers released it. Callers (mfs_fuse opendir)
        // legally read the blob after rel via the surviving cache ref.
        let _lock = CACHE.lock().unwrap();
        // SAFETY: blob from groups_get_common, decref'd at most once per
        // get (mfs_fuse get→use→rel pattern).
        unsafe {
            blob_decref(g);
        }
    }
}

fn groups_cleanup_thread() {
    unsafe {
        loop {
            {
                let now = monotonic_seconds();
                let mut g = CACHE.lock().unwrap();
                if let Some(c) = g.as_mut() {
                    // C swept 16 of 65536 buckets per 10ms; a whole-map
                    // retain each 10ms is the same work amortized — only
                    // memory-reclamation timing differs, never a decision.
                    for b in c.sweep(now) {
                        blob_decref(b as *mut groups);
                    }
                }
            }
            if KEEP_ALIVE.load(::core::sync::atomic::Ordering::SeqCst) == 0 {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_term() {
    unsafe {
        KEEP_ALIVE.store(0, ::core::sync::atomic::Ordering::SeqCst);
        if let Some(thread) = THREAD.lock().unwrap().take() {
            thread.join().expect("groups reaper panicked");
        }
        let mut g = CACHE.lock().unwrap();
        if let Some(c) = g.as_mut() {
            for b in c.term() {
                blob_decref(b as *mut groups);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_init(to: ::core::ffi::c_double, dm: ::core::ffi::c_int) {
    DEBUG_MODE.store(dm, ::core::sync::atomic::Ordering::Relaxed);
    {
        let mut g = CACHE.lock().unwrap();
        *g = Some(GroupCache::new(to));
    }
    KEEP_ALIVE.store(1, ::core::sync::atomic::Ordering::SeqCst);
    let thread = plfscommon::lwthread::spawn_min("groups-reaper", groups_cleanup_thread)
        .unwrap_or_else(|_| std::process::abort());
    *THREAD.lock().unwrap() = Some(thread);
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;
    use std::vec;
    use std::vec::Vec;

    #[test]
    fn parse_groups_line_reference() {
        // real /proc format: "Groups: 4 24 27 30 46 100 114 1000"
        assert_eq!(
            parse_groups_line(" 4 24 27 30 46 100 114 1000", 1000),
            vec![1000, 4, 24, 27, 30, 46, 100, 114]
        );
        // gid duplicated in list → excluded from supplementary
        assert_eq!(parse_groups_line(" 1000 4", 1000), vec![1000, 4]);
        // tabs as separators
        assert_eq!(parse_groups_line("\t27\t30", 27), vec![27, 30]);
        // empty list
        assert_eq!(parse_groups_line("", 1000), vec![1000]);
        assert_eq!(parse_groups_line(" ", 1000), vec![1000]);
        // trailing junk stops the walk (C: *ptr not blank/digit)
        assert_eq!(parse_groups_line(" 4 5x 6", 1000), vec![1000, 4, 5]);
    }

    // imp never dereferences blobs; tests use fake pointer values.
    fn fetch_blob(v: usize) -> impl Fn(i32, u32) -> usize {
        move |_, _| v
    }

    #[test]
    fn cache_tree_nonroot() {
        let mut c = GroupCache::new(10.0);
        let g = c.get_common(111, 1000, 1000, false, 100.0, &fetch_blob(0x1000));
        assert!(matches!(g, GetResult::Fetched(0x1000, None)));
        // cached hit within timeout (fetch would panic if called)
        let g2 = c.get_common(111, 1000, 1000, false, 105.0, &|_, _| {
            panic!("must not fetch")
        });
        assert!(matches!(g2, GetResult::Cached(0x1000)));
        // expired → refetch, old blob handed back for decref
        let g3 = c.get_common(111, 1000, 1000, false, 111.0, &fetch_blob(0x2000));
        assert!(matches!(g3, GetResult::Fetched(0x2000, Some(0x1000))));
        // strict-< : time+to == now is still fresh
        let g4 = c.get_common(111, 1000, 1000, false, 121.0, &|_, _| {
            panic!("must not fetch")
        });
        assert!(matches!(g4, GetResult::Cached(0x2000)));
    }

    #[test]
    fn root_always_refetches() {
        let mut c = GroupCache::new(1000.0);
        let _ = c.get_common(111, 0, 0, false, 100.0, &fetch_blob(0x1000));
        let g = c.get_common(111, 0, 0, false, 101.0, &fetch_blob(0x2000));
        assert!(matches!(g, GetResult::Fetched(0x2000, Some(0x1000))));
    }

    #[test]
    fn cacheonly_returns_stale_but_never_fetches() {
        let mut c = GroupCache::new(10.0);
        let _ = c.get_common(111, 1000, 1000, false, 100.0, &fetch_blob(0x1000));
        // stale but cacheonly → still returned, no fetch
        let g = c.get_common(111, 1000, 1000, true, 1000.0, &|_, _| {
            panic!("must not fetch")
        });
        assert!(matches!(g, GetResult::Cached(0x1000)));
        // cacheonly miss → Emergency (uncached [gid] at boundary), not stored
        let g = c.get_common(222, 1000, 1000, true, 1000.0, &|_, _| {
            panic!("must not fetch")
        });
        assert!(matches!(g, GetResult::Emergency));
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn sweep_drops_expired_and_returns_blobs() {
        let mut c = GroupCache::new(10.0);
        let _ = c.get_common(111, 1000, 1000, false, 100.0, &fetch_blob(0x1000));
        let _ = c.get_common(222, 1000, 1000, false, 105.0, &fetch_blob(0x2000));
        let mut dead = c.sweep(120.0); // both stale (110/115 < 120)
        dead.sort();
        assert_eq!(dead, vec![0x1000, 0x2000]);
        assert_eq!(c.len(), 0);
    }

    #[test]
    fn term_drains_all_blobs() {
        let mut c = GroupCache::new(10.0);
        let _ = c.get_common(111, 1000, 1000, false, 100.0, &fetch_blob(0x1000));
        assert_eq!(c.term(), vec![0x1000]);
        assert_eq!(c.len(), 0);
    }
}
