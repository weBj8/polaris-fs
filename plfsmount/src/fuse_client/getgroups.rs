//! Supplementary-groups cache with direct Rust ownership.
//!
//! Original: MooseFS mfsclient/getgroups.c. Cache entries are keyed by
//! (pid, uid, gid), expire after a configured timeout, and own shared group
//! slices through `Arc`. `/proc` access and the monotonic clock are the only
//! external boundaries.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

unsafe extern "C" {
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
}

mod imp {
    #![deny(unsafe_code)]

    use std::collections::HashMap;
    use std::sync::Arc;

    const HASH_SIZE: u32 = 65_536;
    const REAPER_BUCKETS: u32 = 16;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub(super) struct Key {
        pid: i32,
        uid: u32,
        gid: u32,
    }

    impl Key {
        pub(super) fn new(pid: i32, uid: u32, gid: u32) -> Self {
            Self { pid, uid, gid }
        }

        pub(super) fn bucket(self) -> u32 {
            (self
                .pid
                .cast_unsigned()
                .wrapping_mul(0x74BF_4863)
                .wrapping_add(self.uid)
                .wrapping_mul(0xB435_C489)
                .wrapping_add(self.gid))
                % HASH_SIZE
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct GroupSet(Arc<[u32]>);

    impl GroupSet {
        pub(super) fn from_vec(groups: Vec<u32>) -> Self {
            Self(groups.into())
        }

        pub(super) fn primary(gid: u32) -> Self {
            Self(Arc::from([gid]))
        }

        pub fn as_slice(&self) -> &[u32] {
            &self.0
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        #[cfg(test)]
        pub(super) fn strong_count(&self) -> usize {
            Arc::strong_count(&self.0)
        }
    }

    /// Parse a /proc/<pid>/status "Groups:" line with C ptr-walk semantics:
    /// skip blanks, strtoul each number, stop at first non-blank non-digit.
    /// Result starts with primary gid and excludes that gid from remainder.
    pub fn parse_groups_line(line: &str, gid: u32) -> Vec<u32> {
        let mut supplementary = Vec::new();
        let bytes = line.as_bytes();
        let mut i = 0usize;
        loop {
            while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
                i += 1;
            }
            if i < bytes.len() && bytes[i].is_ascii_digit() {
                let mut group: u64 = 0;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    group = group
                        .saturating_mul(10)
                        .saturating_add((bytes[i] - b'0') as u64);
                    i += 1;
                }
                if group as u32 != gid {
                    supplementary.push(group as u32);
                }
            } else {
                break;
            }
        }
        let mut groups = Vec::with_capacity(supplementary.len() + 1);
        groups.push(gid);
        groups.extend(supplementary);
        groups
    }

    struct Entry {
        time: f64,
        groups: GroupSet,
    }

    pub(super) enum Lookup {
        Return(GroupSet),
        Fetch(Option<GroupSet>),
    }

    pub struct GroupCache {
        buckets: HashMap<u32, HashMap<Key, Entry>>,
        timeout: f64,
        reaper_cursor: u32,
    }

    impl GroupCache {
        pub fn new(timeout: f64) -> Self {
            Self {
                buckets: HashMap::new(),
                timeout,
                reaper_cursor: 0,
            }
        }

        fn sweep_bucket(&mut self, bucket_index: u32, now: f64) {
            let empty = self
                .buckets
                .get_mut(&bucket_index)
                .map(|bucket| {
                    let timeout = self.timeout;
                    bucket.retain(|_, entry| entry.time + timeout >= now);
                    bucket.is_empty()
                })
                .unwrap_or(false);
            if empty {
                self.buckets.remove(&bucket_index);
            }
        }

        pub(super) fn lookup(&mut self, key: Key, cache_only: bool, now: f64) -> Lookup {
            let bucket_index = key.bucket();
            if cache_only {
                return self
                    .buckets
                    .get(&bucket_index)
                    .and_then(|bucket| bucket.get(&key))
                    .map(|entry| entry.groups.clone())
                    .map(Lookup::Return)
                    .unwrap_or_else(|| Lookup::Return(GroupSet::primary(key.gid)));
            }

            self.sweep_bucket(bucket_index, now);
            let cached = self
                .buckets
                .get(&bucket_index)
                .and_then(|bucket| bucket.get(&key))
                .map(|entry| entry.groups.clone());
            if key.uid != 0 {
                if let Some(groups) = cached {
                    return Lookup::Return(groups);
                }
            }
            Lookup::Fetch(cached)
        }

        pub(super) fn store(&mut self, key: Key, groups: GroupSet, before_fetch: f64) {
            let bucket = self.buckets.entry(key.bucket()).or_default();
            if let Some(entry) = bucket.get_mut(&key) {
                entry.groups = groups;
            } else {
                bucket.insert(
                    key,
                    Entry {
                        time: before_fetch,
                        groups,
                    },
                );
            }
        }

        pub fn sweep_next(&mut self, now: f64) {
            for _ in 0..REAPER_BUCKETS {
                self.sweep_bucket(self.reaper_cursor, now);
                self.reaper_cursor = (self.reaper_cursor + 1) % HASH_SIZE;
            }
        }

        pub fn term(&mut self) {
            self.buckets.clear();
        }

        #[cfg(test)]
        pub fn len(&self) -> usize {
            self.buckets.values().map(HashMap::len).sum()
        }

        #[cfg(test)]
        pub fn set_reaper_cursor(&mut self, cursor: u32) {
            self.reaper_cursor = cursor;
        }

        #[cfg(test)]
        pub fn reaper_cursor(&self) -> u32 {
            self.reaper_cursor
        }

        #[cfg(test)]
        pub fn entry_time(&self, key: Key) -> Option<f64> {
            self.buckets
                .get(&key.bucket())
                .and_then(|bucket| bucket.get(&key))
                .map(|entry| entry.time)
        }
    }
}

pub use imp::GroupSet;
use imp::{GroupCache, Key, Lookup};

static CACHE: Mutex<Option<GroupCache>> = Mutex::new(None);
static KEEP_ALIVE: AtomicBool = AtomicBool::new(false);
static THREAD: Mutex<Option<std::thread::JoinHandle<()>>> = Mutex::new(None);
static DEBUG_MODE: AtomicI32 = AtomicI32::new(0);

fn monotonic_now() -> f64 {
    // SAFETY: monotonic_seconds takes no pointers and has no preconditions.
    unsafe { monotonic_seconds() }
}

/// Fetch supplementary groups from Linux procfs. Unreadable or missing group
/// data falls back to primary gid, matching original behavior.
fn fetch_groups(pid: i32, gid: u32) -> GroupSet {
    let path = format!("/proc/{pid}/status");
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return GroupSet::primary(gid),
    };
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("Groups:") {
            return GroupSet::from_vec(imp::parse_groups_line(rest, gid));
        }
    }
    GroupSet::primary(gid)
}

fn get_common_with(
    cache: &Mutex<Option<GroupCache>>,
    pid: i32,
    uid: u32,
    gid: u32,
    cache_only: bool,
    before_fetch: f64,
    fetch: impl FnOnce(i32, u32) -> GroupSet,
) -> GroupSet {
    let key = Key::new(pid, uid, gid);
    let lookup = cache
        .lock()
        .unwrap()
        .as_mut()
        .map(|cache| cache.lookup(key, cache_only, before_fetch));
    match lookup {
        Some(Lookup::Return(groups)) => groups,
        Some(Lookup::Fetch(cached)) => {
            let groups = fetch(pid, gid);
            let mut cache = cache.lock().unwrap();
            drop(cached);
            if let Some(cache) = cache.as_mut() {
                cache.store(key, groups.clone(), before_fetch);
            }
            groups
        }
        None => GroupSet::primary(gid),
    }
}

pub fn get_common(pid: i32, uid: u32, gid: u32, cache_only: bool) -> GroupSet {
    let groups = get_common_with(
        &CACHE,
        pid,
        uid,
        gid,
        cache_only,
        monotonic_now(),
        fetch_groups,
    );
    if DEBUG_MODE.load(Ordering::Relaxed) != 0 {
        eprintln!(
            "groups_get(pid={pid},uid={uid},gid={gid}):{:?}",
            groups.as_slice()
        );
    }
    groups
}

fn cleanup_thread() {
    loop {
        let keep_alive = {
            let mut cache = CACHE.lock().unwrap();
            if let Some(cache) = cache.as_mut() {
                cache.sweep_next(monotonic_now());
            }
            KEEP_ALIVE.load(Ordering::SeqCst)
        };
        std::thread::sleep(std::time::Duration::from_millis(10));
        if !keep_alive {
            return;
        }
    }
}

pub fn term() {
    {
        let _cache = CACHE.lock().unwrap();
        KEEP_ALIVE.store(false, Ordering::SeqCst);
    }
    if let Some(thread) = THREAD.lock().unwrap().take() {
        thread.join().expect("groups reaper panicked");
    }
    if let Some(cache) = CACHE.lock().unwrap().as_mut() {
        cache.term();
    }
}

pub fn init(timeout: f64, debug: i32) {
    DEBUG_MODE.store(debug, Ordering::Relaxed);
    *CACHE.lock().unwrap() = Some(GroupCache::new(timeout));
    KEEP_ALIVE.store(true, Ordering::SeqCst);
    let thread = plfscommon::lwthread::spawn_min("groups-reaper", cleanup_thread)
        .unwrap_or_else(|_| std::process::abort());
    *THREAD.lock().unwrap() = Some(thread);
}

#[cfg(test)]
mod tests {
    use super::get_common_with;
    use super::imp::*;
    use std::sync::Mutex;

    fn group_set(groups: &[u32]) -> GroupSet {
        GroupSet::from_vec(groups.to_vec())
    }

    fn local_cache(timeout: f64) -> Mutex<Option<GroupCache>> {
        Mutex::new(Some(GroupCache::new(timeout)))
    }

    fn key_in_bucket(target: u32) -> Key {
        (0..65_536)
            .map(|pid| Key::new(pid, 7, 9))
            .find(|key| key.bucket() == target)
            .expect("pid hash must cover every bucket")
    }

    fn lookup_groups(cache: &mut GroupCache, key: Key, cache_only: bool, now: f64) -> GroupSet {
        match cache.lookup(key, cache_only, now) {
            Lookup::Return(groups) => groups,
            Lookup::Fetch(_) => panic!("unexpected fetch"),
        }
    }

    #[test]
    fn parse_groups_line_reference() {
        assert_eq!(
            parse_groups_line(" 4 24 27 30 46 100 114 1000", 1000),
            vec![1000, 4, 24, 27, 30, 46, 100, 114]
        );
        assert_eq!(parse_groups_line(" 1000 4", 1000), vec![1000, 4]);
        assert_eq!(parse_groups_line("\t27\t30", 27), vec![27, 30]);
        assert_eq!(parse_groups_line("", 1000), vec![1000]);
        assert_eq!(parse_groups_line(" ", 1000), vec![1000]);
        assert_eq!(parse_groups_line(" 4 5x 6", 1000), vec![1000, 4, 5]);
        // strtoul overflow saturates ULONG_MAX; gid_t keeps low 32 bits.
        assert_eq!(
            parse_groups_line(" 184467440737095516160", 1000),
            vec![1000, u32::MAX]
        );
    }

    #[test]
    fn nonroot_fresh_hit_and_expiry_boundary() {
        let cache = local_cache(10.0);
        let first = get_common_with(&cache, 111, 1000, 1000, false, 100.0, |_, _| {
            group_set(&[1000, 4])
        });
        assert_eq!(first.as_slice(), &[1000, 4]);
        let hit = get_common_with(&cache, 111, 1000, 1000, false, 105.0, |_, _| {
            panic!("must not fetch")
        });
        assert_eq!(hit.as_slice(), &[1000, 4]);
        let boundary = get_common_with(&cache, 111, 1000, 1000, false, 110.0, |_, _| {
            panic!("must not fetch")
        });
        assert_eq!(boundary.as_slice(), &[1000, 4]);
        let replacement = get_common_with(&cache, 111, 1000, 1000, false, 110.1, |_, _| {
            group_set(&[1000, 8])
        });
        assert_eq!(replacement.as_slice(), &[1000, 8]);
    }

    #[test]
    fn root_refresh_preserves_existing_timestamp() {
        let cache = local_cache(10.0);
        let key = Key::new(111, 0, 0);
        let _ = get_common_with(&cache, 111, 0, 0, false, 100.0, |_, _| group_set(&[0, 1]));
        let refreshed = get_common_with(&cache, 111, 0, 0, false, 105.0, |_, _| group_set(&[0, 2]));
        assert_eq!(refreshed.as_slice(), &[0, 2]);
        assert_eq!(
            cache.lock().unwrap().as_ref().unwrap().entry_time(key),
            Some(100.0)
        );
    }

    #[test]
    fn fetch_runs_without_cache_lock() {
        let cache = local_cache(10.0);
        let groups = get_common_with(&cache, 111, 1000, 1000, false, 100.0, |_, _| {
            assert!(cache.try_lock().is_ok());
            group_set(&[1000, 4])
        });
        assert_eq!(groups.as_slice(), &[1000, 4]);
    }

    #[test]
    fn cacheonly_keeps_stale_entry_before_its_bucket_sweep() {
        let mut cache = GroupCache::new(10.0);
        let key = key_in_bucket(16);
        cache.store(key, group_set(&[9, 40]), 100.0);

        cache.sweep_next(1000.0);

        assert_eq!(cache.reaper_cursor(), 16);
        assert_eq!(
            lookup_groups(&mut cache, key, true, 1000.0).as_slice(),
            &[9, 40]
        );
    }

    #[test]
    fn normal_lookup_sweeps_expired_same_bucket_neighbor() {
        let mut cache = GroupCache::new(10.0);
        let stale = Key::new(111, 1000, 1000);
        let neighbor = Key::new(111 + 65_536, 1000, 1000);
        assert_eq!(stale.bucket(), neighbor.bucket());
        cache.store(stale, group_set(&[1000, 4]), 100.0);
        cache.store(neighbor, group_set(&[1000, 8]), 105.0);

        assert_eq!(
            lookup_groups(&mut cache, neighbor, false, 111.0).as_slice(),
            &[1000, 8]
        );
        assert_eq!(
            lookup_groups(&mut cache, stale, true, 111.0).as_slice(),
            &[1000]
        );
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn reaper_sweeps_exactly_16_buckets_and_wraps() {
        let mut cache = GroupCache::new(10.0);
        let before = key_in_bucket(65_527);
        let first = key_in_bucket(65_528);
        let last = key_in_bucket(7);
        let after = key_in_bucket(8);
        for key in [before, first, last, after] {
            cache.store(key, group_set(&[9, 40]), 0.0);
        }
        cache.set_reaper_cursor(65_528);

        cache.sweep_next(20.0);

        assert_eq!(cache.reaper_cursor(), 8);
        assert_eq!(lookup_groups(&mut cache, before, true, 20.0).len(), 2);
        assert_eq!(lookup_groups(&mut cache, first, true, 20.0).len(), 1);
        assert_eq!(lookup_groups(&mut cache, last, true, 20.0).len(), 1);
        assert_eq!(lookup_groups(&mut cache, after, true, 20.0).len(), 2);
    }

    #[test]
    fn concurrent_store_order_preserves_first_timestamp_and_last_groups() {
        let mut cache = GroupCache::new(10.0);
        let key = Key::new(111, 0, 0);
        assert!(matches!(
            cache.lookup(key, false, 100.0),
            Lookup::Fetch(None)
        ));
        assert!(matches!(
            cache.lookup(key, false, 101.0),
            Lookup::Fetch(None)
        ));

        cache.store(key, group_set(&[0, 2]), 101.0);
        cache.store(key, group_set(&[0, 1]), 100.0);

        assert_eq!(cache.entry_time(key), Some(101.0));
        assert_eq!(
            lookup_groups(&mut cache, key, true, 101.0).as_slice(),
            &[0, 1]
        );
    }

    #[test]
    fn replacement_preserves_live_arc() {
        let cache = local_cache(10.0);
        let old = get_common_with(&cache, 111, 1000, 1000, false, 100.0, |_, _| {
            group_set(&[1000, 4])
        });
        assert_eq!(old.strong_count(), 2);
        let new = get_common_with(&cache, 111, 1000, 1000, false, 111.0, |_, _| {
            group_set(&[1000, 8])
        });
        assert_eq!(old.as_slice(), &[1000, 4]);
        assert_eq!(old.strong_count(), 1);
        assert_eq!(new.strong_count(), 2);
    }

    #[test]
    fn term_releases_cache_but_not_caller_arc() {
        let cache = local_cache(10.0);
        let groups = get_common_with(&cache, 111, 1000, 1000, false, 100.0, |_, _| {
            group_set(&[1000, 4])
        });
        assert_eq!(groups.strong_count(), 2);
        let mut cache = cache.lock().unwrap();
        let cache = cache.as_mut().unwrap();
        cache.term();
        assert_eq!(groups.strong_count(), 1);
        assert_eq!(groups.as_slice(), &[1000, 4]);
        assert_eq!(cache.len(), 0);
    }
}
