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
//! the cache decision tree over a HashMap. The boundary keeps the C blob
//! ABI and the reaper thread. One deliberate deviation, externally
//! invisible: C swept expired same-bucket neighbors during a lookup;
//! here only the queried key is validated and the reaper does the rest
//! (the hit/miss/refresh decision per key is unchanged).

use std::sync::Mutex as StdMutex;

unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const ::core::ffi::c_void,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type pid_t = ::core::ffi::c_int;
pub type uid_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
pub type pthread_t = ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

// local copy, mirroring the C static-inline portable_usleep (per TU)
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

    struct Entry {
        time: f64,
        gids: Vec<u32>,
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

        /// The groups_get_common decision tree. `fetch` reads /proc at the
        /// boundary and returns the full gid list (gid first).
        pub fn get_common(
            &mut self,
            pid: i32,
            uid: u32,
            gid: u32,
            cacheonly: bool,
            now: f64,
            fetch: &dyn Fn(i32, u32) -> Vec<u32>,
        ) -> Vec<u32> {
            let key = (pid, uid, gid);
            let fresh = self
                .map
                .get(&key)
                .map(|e| e.time + self.timeout >= now)
                .unwrap_or(false);
            if cacheonly {
                // emergency mode: any cached entry (even stale) wins;
                // else a fresh [gid] that is NOT cached
                return match self.map.get(&key) {
                    Some(e) => e.gids.clone(),
                    None => vec![gid],
                };
            }
            // expired entries are treated as missing (reaper sweeps them)
            if uid != 0 && fresh {
                return self.map.get(&key).unwrap().gids.clone();
            }
            // root always refetches; non-root refetches on miss/stale
            let gids = fetch(pid, gid);
            self.map.insert(
                key,
                Entry {
                    time: now, // C stamps the pre-fetch time
                    gids: gids.clone(),
                },
            );
            gids
        }

        /// reaper sweep: drop expired entries
        pub fn sweep(&mut self, now: f64) {
            let to = self.timeout;
            self.map.retain(|_, e| e.time + to >= now);
        }

        pub fn term(&mut self) {
            self.map.clear();
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

use imp::GroupCache;

static CACHE: StdMutex<Option<GroupCache>> = StdMutex::new(None);
static KEEP_ALIVE: ::core::sync::atomic::AtomicU8 = ::core::sync::atomic::AtomicU8::new(0);
static mut main_thread: pthread_t = 0;
static mut debug_mode: ::core::ffi::c_int = 0;

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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_get_common(
    pid: pid_t,
    uid: uid_t,
    gid: gid_t,
    cacheonly: uint8_t,
) -> *mut groups {
    unsafe {
        let now = monotonic_seconds();
        let gids = {
            let mut g = CACHE.lock().unwrap();
            match g.as_mut() {
                Some(c) => c.get_common(pid, uid, gid, cacheonly != 0, now, &|p, gg| {
                    fetch_groups(p, gg)
                }),
                None => vec![gid],
            }
        };
        if debug_mode != 0 {
            eprintln!("groups_get(pid={pid},uid={uid},gid={gid}): {gids:?}");
        }
        make_blob(&gids)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_rel(g: *mut groups) {
    if !g.is_null() {
        // SAFETY: blob from groups_get_common, libc-malloc'd, released at
        // most once (mfs_fuse get→use→rel pattern).
        unsafe {
            free(g as *mut ::core::ffi::c_void);
        }
    }
}

unsafe extern "C" fn groups_cleanup_thread(arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    unsafe {
        loop {
            {
                let now = monotonic_seconds();
                let mut g = CACHE.lock().unwrap();
                if let Some(c) = g.as_mut() {
                    // C swept 16 of 65536 buckets per 10ms; a whole-map
                    // retain each 10ms is the same work amortized — only
                    // memory-reclamation timing differs, never a decision.
                    c.sweep(now);
                }
            }
            if KEEP_ALIVE.load(::core::sync::atomic::Ordering::SeqCst) == 0 {
                return arg;
            }
            portable_usleep(10000);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_term() {
    unsafe {
        KEEP_ALIVE.store(0, ::core::sync::atomic::Ordering::SeqCst);
        pthread_join(
            main_thread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        let mut g = CACHE.lock().unwrap();
        if let Some(c) = g.as_mut() {
            c.term();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_init(to: ::core::ffi::c_double, dm: ::core::ffi::c_int) {
    unsafe {
        debug_mode = dm;
        {
            let mut g = CACHE.lock().unwrap();
            *g = Some(GroupCache::new(to));
        }
        KEEP_ALIVE.store(1, ::core::sync::atomic::Ordering::SeqCst);
        pthread_create(
            &raw mut main_thread,
            ::core::ptr::null(),
            Some(groups_cleanup_thread),
            NULL,
        );
    }
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

    fn fetch_fixed(v: Vec<u32>) -> impl Fn(i32, u32) -> Vec<u32> {
        move |_, _| v.clone()
    }

    #[test]
    fn cache_tree_nonroot() {
        let mut c = GroupCache::new(10.0);
        let f = fetch_fixed(vec![1000, 4, 24]);
        let g = c.get_common(111, 1000, 1000, false, 100.0, &f);
        assert_eq!(&*g, &vec![1000, 4, 24]);
        // cached hit within timeout (fetch would panic if called)
        let g2 = c.get_common(111, 1000, 1000, false, 105.0, &|_, _| {
            panic!("must not fetch")
        });
        assert_eq!(&*g2, &vec![1000, 4, 24]);
        // expired → refetch
        let g3 = c.get_common(111, 1000, 1000, false, 111.0, &fetch_fixed(vec![1000, 7]));
        assert_eq!(&*g3, &vec![1000, 7]);
        // strict-< : time+to == now is still fresh
        let g4 = c.get_common(111, 1000, 1000, false, 121.0, &|_, _| {
            panic!("must not fetch")
        });
        assert_eq!(&*g4, &vec![1000, 7]);
    }

    #[test]
    fn root_always_refetches() {
        let mut c = GroupCache::new(1000.0);
        let _ = c.get_common(111, 0, 0, false, 100.0, &fetch_fixed(vec![0, 1]));
        let g = c.get_common(111, 0, 0, false, 101.0, &fetch_fixed(vec![0, 2]));
        assert_eq!(&*g, &vec![0, 2]);
    }

    #[test]
    fn cacheonly_returns_stale_but_never_fetches() {
        let mut c = GroupCache::new(10.0);
        let _ = c.get_common(111, 1000, 1000, false, 100.0, &fetch_fixed(vec![1000, 4]));
        // stale but cacheonly → still returned, no fetch
        let g = c.get_common(111, 1000, 1000, true, 1000.0, &|_, _| {
            panic!("must not fetch")
        });
        assert_eq!(&*g, &vec![1000, 4]);
        // cacheonly miss → [gid], not cached
        let g = c.get_common(222, 1000, 1000, true, 1000.0, &|_, _| {
            panic!("must not fetch")
        });
        assert_eq!(&*g, &vec![1000]);
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn sweep_drops_expired() {
        let mut c = GroupCache::new(10.0);
        let _ = c.get_common(111, 1000, 1000, false, 100.0, &fetch_fixed(vec![1000, 4]));
        let _ = c.get_common(222, 1000, 1000, false, 105.0, &fetch_fixed(vec![1000, 5]));
        c.sweep(120.0); // both stale (110/115 < 120)
        assert_eq!(c.len(), 0);
    }
}
