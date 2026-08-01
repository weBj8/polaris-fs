//! Sustained stats cache — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfsclient/sustained_stats.c. inode → attr[35] cache
//! for files held open across unlinks, 65536 individually locked hash
//! buckets, reaper thread evicting entries idle > 60s.
//!
//! Safe core in `imp`: per-bucket std Mutex + HashMap (the C used malloc'd
//! chains under per-bucket pthread mutexes; std Mutex is a drop-in). The
//! boundary keeps the reaper thread, mfs_log warnings and monotonic clock.
//! term is a real AtomicU8 (replaces core::intrinsics::__sync_fetch_and_or).

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
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
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
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3;
pub const TYPE_DIRECTORY: ::core::ffi::c_int = 2;

#[deny(unsafe_code)]
pub mod imp {
    use std::collections::HashMap;
    use std::sync::Mutex;

    pub const HASHSIZE: usize = 65536;
    pub const SSTATS_TIMEOUT: f64 = 60.0;
    pub const DEFAULT_ATTR: [u8; 35] = {
        let mut a = [0u8; 35];
        a[1] = 0x01 | (2 << 4); // TYPE_DIRECTORY << 4 | 1
        a[2] = 0xFF;
        a
    };

    struct Entry {
        attr: [u8; 35],
        lastrefresh: f64,
    }

    pub struct Stats {
        buckets: Vec<Mutex<HashMap<u32, Entry>>>,
    }

    pub enum GetOutcome {
        /// cached attr (status OK)
        Hit([u8; 35]),
        /// not cached; defaults copied out and (forceok) cached — C still
        /// returns ENOENT here, attr carries the defaults
        Default([u8; 35]),
    }

    impl Stats {
        pub fn new() -> Self {
            Stats {
                buckets: (0..HASHSIZE).map(|_| Mutex::new(HashMap::new())).collect(),
            }
        }

        fn bucket(&self, inode: u32) -> &Mutex<HashMap<u32, Entry>> {
            &self.buckets[(inode as usize) % HASHSIZE]
        }

        /// C semantics: found → refresh + copy; missing+forceok → cache
        /// defaults and hand them out; status is ENOENT either way.
        pub fn get(&self, inode: u32, forceok: bool, now: f64) -> GetOutcome {
            let mut b = self.bucket(inode).lock().unwrap();
            if let Some(e) = b.get_mut(&inode) {
                e.lastrefresh = now;
                return GetOutcome::Hit(e.attr);
            }
            if forceok {
                b.insert(
                    inode,
                    Entry {
                        attr: DEFAULT_ATTR,
                        lastrefresh: now,
                    },
                );
            }
            GetOutcome::Default(DEFAULT_ATTR)
        }

        pub fn set(&self, inode: u32, attr: [u8; 35], createflag: bool, now: f64) {
            let mut b = self.bucket(inode).lock().unwrap();
            match b.get_mut(&inode) {
                Some(e) => {
                    e.attr = attr;
                    e.lastrefresh = now;
                }
                None if createflag => {
                    b.insert(
                        inode,
                        Entry {
                            attr,
                            lastrefresh: now,
                        },
                    );
                }
                None => {}
            }
        }

        /// Reap one bucket: drop entries idle longer than SSTATS_TIMEOUT.
        pub fn reap_bucket(&self, hash: usize, now: f64) {
            let mut b = self.buckets[hash].lock().unwrap();
            b.retain(|_, e| e.lastrefresh + SSTATS_TIMEOUT >= now);
        }

        #[cfg(test)]
        pub fn len(&self) -> usize {
            self.buckets.iter().map(|b| b.lock().unwrap().len()).sum()
        }
    }
}

use imp::Stats;

static mut STATS: Option<Stats> = None;
static TERM: ::core::sync::atomic::AtomicU8 = ::core::sync::atomic::AtomicU8::new(0);
static mut clthread: pthread_t = 0;

/// SAFETY: STATS is set once in sstats_init before the reaper and any FUSE
/// thread can call in, and never cleared until process exit (C leaked the
/// table the same way; sstats_term only joins the reaper).
unsafe fn stats() -> &'static Stats {
    unsafe { (*(&raw const STATS)).as_ref().unwrap_unchecked() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sstats_get(
    inode: uint32_t,
    attr: *mut uint8_t,
    forceok: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let now = monotonic_seconds();
        match stats().get(inode, forceok != 0, now) {
            imp::GetOutcome::Hit(a) => {
                ::core::ptr::copy_nonoverlapping(a.as_ptr(), attr, 35);
                MFS_STATUS_OK
            }
            imp::GetOutcome::Default(a) => {
                if forceok != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"no sustained stats for node: %u - using defaults\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        inode,
                    );
                    ::core::ptr::copy_nonoverlapping(a.as_ptr(), attr, 35);
                }
                MFS_ERROR_ENOENT
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sstats_set(inode: uint32_t, attr: *const uint8_t, createflag: uint8_t) {
    unsafe {
        let now = monotonic_seconds();
        let a: [u8; 35] = ::core::ptr::read(attr as *const [u8; 35]);
        stats().set(inode, a, createflag != 0, now);
    }
}

unsafe extern "C" fn sstats_thread(_arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut hash: usize = 0;
        loop {
            let now = monotonic_seconds();
            stats().reap_bucket(hash, now);
            hash = (hash + 1) % imp::HASHSIZE;
            portable_usleep(100000);
            if TERM.load(::core::sync::atomic::Ordering::SeqCst) == 1 {
                return NULL;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sstats_term() {
    unsafe {
        TERM.store(1, ::core::sync::atomic::Ordering::SeqCst);
        pthread_join(
            clthread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        // C freed the tables; process is exiting anyway — leave STATS to the
        // OS (identical observable behavior, no post-join use).
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sstats_init() {
    unsafe {
        STATS = Some(Stats::new());
        TERM.store(0, ::core::sync::atomic::Ordering::SeqCst);
        lwt_minthread_create(&raw mut clthread, 0, Some(sstats_thread), NULL);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;

    #[test]
    fn get_set_roundtrip() {
        let s = Stats::new();
        let mut a = [0u8; 35];
        a[0] = 7;
        s.set(42, a, true, 100.0);
        match s.get(42, false, 110.0) {
            GetOutcome::Hit(got) => assert_eq!(got[0], 7),
            _ => panic!("expected hit"),
        }
        // set without createflag on unknown inode: no-op
        s.set(43, a, false, 110.0);
        assert!(matches!(s.get(43, false, 110.0), GetOutcome::Default(_)));
    }

    #[test]
    fn forceok_caches_defaults_but_reports_missing() {
        let s = Stats::new();
        match s.get(99, true, 100.0) {
            GetOutcome::Default(a) => assert_eq!(a, DEFAULT_ATTR),
            _ => panic!("expected default"),
        }
        // now cached
        assert!(matches!(s.get(99, false, 101.0), GetOutcome::Hit(_)));
    }

    #[test]
    fn reaper_evicts_idle_entries() {
        let s = Stats::new();
        s.set(1, [1u8; 35], true, 100.0);
        s.set(2, [2u8; 35], true, 100.0);
        // refresh inode 1 via get
        let _ = s.get(1, false, 150.0);
        s.reap_bucket(1, 200.0);
        s.reap_bucket(2, 200.0);
        assert!(matches!(s.get(1, false, 201.0), GetOutcome::Hit(_)));
        assert!(matches!(s.get(2, false, 201.0), GetOutcome::Default(_)));
    }

    #[test]
    fn timeout_boundary_is_strict() {
        let s = Stats::new();
        s.set(1, [1u8; 35], true, 100.0);
        // lastrefresh + TIMEOUT == now → kept (C: evict when < now)
        s.reap_bucket(1, 100.0 + SSTATS_TIMEOUT);
        assert!(matches!(s.get(1, false, 200.0), GetOutcome::Hit(_)));
    }
}
