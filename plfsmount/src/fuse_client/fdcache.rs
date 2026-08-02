//! File descriptor (open-context) cache — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfsclient/fdcache.c. Per-(inode,uid,gid,pid) cache of
//! lookup answers with a 1s lifetime, optionally carrying chunk-zero data
//! injected into chunksdatacache on release.
//!
//! Safe core in `imp` (per-bucket std Mutex + Vec); boundary keeps the
//! monotonic clock and the chunksdatacache call. `acquire` returns an owned
//! `FdEntry`, so its lifetime ends through normal Rust drop.

unsafe extern "C" {
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type uid_t = ::core::ffi::c_uint;
pub type gid_t = ::core::ffi::c_uint;
pub type pid_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_ctx {
    pub uid: uid_t,
    pub gid: gid_t,
    pub pid: pid_t,
    pub umask: ::core::ffi::c_uint,
}
pub const LOOKUP_CHUNK_ZERO_DATA: uint16_t = 0x0100; // MFSCommunication.h

#[deny(unsafe_code)]
pub mod imp {
    use std::sync::Mutex;

    pub const FDCACHE_HASHSIZE: usize = 1024;
    pub const FDCACHE_TIMEOUT: f64 = 1.0;
    pub const CSDATA_MAX: usize = 10 * 14;

    #[derive(Clone)]
    pub struct FdEntry {
        pub createtime: f64,
        pub uid: u32,
        pub gid: u32,
        pub pid: i32,
        pub inode: u32,
        pub attr: [u8; 35],
        pub lflags: u16,
        pub csdataver: u8,
        pub chunkid: u64,
        pub version: u32,
        pub csdata: Vec<u8>,
    }

    pub struct FdCache {
        buckets: Vec<Mutex<Vec<FdEntry>>>,
    }

    impl FdCache {
        pub fn new() -> Self {
            FdCache {
                buckets: (0..FDCACHE_HASHSIZE)
                    .map(|_| Mutex::new(Vec::new()))
                    .collect(),
            }
        }

        fn bucket(&self, inode: u32) -> &Mutex<Vec<FdEntry>> {
            &self.buckets[(inode as usize) % FDCACHE_HASHSIZE]
        }

        /// Insert or replace the entry for (ctx,inode); sweeps expired
        /// entries and collapses duplicates while walking (C behavior).
        #[allow(clippy::too_many_arguments)]
        pub fn insert(
            &self,
            uid: u32,
            gid: u32,
            pid: i32,
            inode: u32,
            attr: [u8; 35],
            lflags: u16,
            csdataver: u8,
            chunkid: u64,
            version: u32,
            csdata: &[u8],
            now: f64,
        ) {
            let mut b = self.bucket(inode).lock().unwrap();
            // sweep expired; drop duplicate (ctx,inode) entries beyond first
            let mut seen = false;
            b.retain(|e| {
                if e.createtime + FDCACHE_TIMEOUT < now {
                    return false;
                }
                if e.inode == inode && e.uid == uid && e.gid == gid && e.pid == pid {
                    if seen {
                        return false; // duplicate
                    }
                    seen = true;
                }
                true
            });
            let (lflags, csdataver, chunkid, version, csdata) =
                if lflags & super::LOOKUP_CHUNK_ZERO_DATA != 0 && csdata.len() <= CSDATA_MAX {
                    (lflags, csdataver, chunkid, version, csdata.to_vec())
                } else {
                    (lflags & !super::LOOKUP_CHUNK_ZERO_DATA, 0, 0, 0, Vec::new())
                };
            let entry = FdEntry {
                createtime: now,
                uid,
                gid,
                pid,
                inode,
                attr,
                lflags,
                csdataver,
                chunkid,
                version,
                csdata,
            };
            if let Some(i) = b
                .iter()
                .position(|e| e.inode == inode && e.uid == uid && e.gid == gid && e.pid == pid)
            {
                b[i] = entry;
            } else {
                b.push(entry);
            }
        }

        pub fn invalidate(&self, inode: u32) {
            let mut b = self.bucket(inode).lock().unwrap();
            b.retain(|e| e.inode != inode);
        }

        /// Fresh entry for (ctx,inode) → (attr, lflags)
        pub fn find(
            &self,
            uid: u32,
            gid: u32,
            pid: i32,
            inode: u32,
            now: f64,
        ) -> Option<([u8; 35], u16)> {
            let b = self.bucket(inode).lock().unwrap();
            b.iter()
                .find(|e| {
                    e.inode == inode
                        && e.uid == uid
                        && e.gid == gid
                        && e.pid == pid
                        && e.createtime + FDCACHE_TIMEOUT >= now
                })
                .map(|e| (e.attr, e.lflags))
        }

        /// Like find but removes the entry and hands ownership to the caller.
        pub fn acquire(
            &self,
            uid: u32,
            gid: u32,
            pid: i32,
            inode: u32,
            now: f64,
        ) -> Option<FdEntry> {
            let mut b = self.bucket(inode).lock().unwrap();
            let i = b.iter().position(|e| {
                e.inode == inode
                    && e.uid == uid
                    && e.gid == gid
                    && e.pid == pid
                    && e.createtime + FDCACHE_TIMEOUT >= now
            })?;
            Some(b.remove(i))
        }
    }
}

use imp::{FdCache, FdEntry};
use std::sync::OnceLock;

static FDCACHE: OnceLock<FdCache> = OnceLock::new();

fn fdcache() -> &'static FdCache {
    FDCACHE.get_or_init(FdCache::new)
}

pub fn init() {
    let _ = fdcache();
}

pub fn insert(
    uid: u32,
    gid: u32,
    pid: i32,
    inode: u32,
    attr: [u8; 35],
    lflags: u16,
    csdataver: u8,
    chunkid: u64,
    version: u32,
    csdata: &[u8],
) {
    fdcache().insert(
        uid,
        gid,
        pid,
        inode,
        attr,
        lflags,
        csdataver,
        chunkid,
        version,
        csdata,
        unsafe { monotonic_seconds() },
    );
}

pub fn invalidate(inode: u32) {
    fdcache().invalidate(inode);
}

pub fn find(uid: u32, gid: u32, pid: i32, inode: u32) -> Option<([u8; 35], u16)> {
    fdcache().find(uid, gid, pid, inode, unsafe { monotonic_seconds() })
}

pub fn acquire(uid: u32, gid: u32, pid: i32, inode: u32) -> Option<FdEntry> {
    fdcache().acquire(uid, gid, pid, inode, unsafe { monotonic_seconds() })
}

pub fn inject_chunkdata(entry: &FdEntry) {
    if entry.lflags & LOOKUP_CHUNK_ZERO_DATA != 0 {
        plfsclient::chunksdatacache::insert(
            entry.inode,
            0,
            entry.chunkid,
            entry.version,
            entry.csdataver,
            &entry.csdata,
        );
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::LOOKUP_CHUNK_ZERO_DATA;
    use super::imp::*;

    const UID: u32 = 1000;
    const GID: u32 = 1000;
    const PID: i32 = 4242;

    fn ins(c: &FdCache, inode: u32, attr0: u8, now: f64) {
        let mut a = [0u8; 35];
        a[0] = attr0;
        c.insert(UID, GID, PID, inode, a, 0, 0, 0, 0, &[], now);
    }

    #[test]
    fn insert_find_acquire() {
        let c = FdCache::new();
        ins(&c, 5, 0xAA, 100.0);
        assert_eq!(c.find(UID, GID, PID, 5, 100.5).unwrap().0[0], 0xAA);
        // wrong ctx → miss
        assert!(c.find(UID, GID, PID + 1, 5, 100.5).is_none());
        // acquire removes
        let e = c.acquire(UID, GID, PID, 5, 100.9).unwrap();
        assert_eq!(e.attr[0], 0xAA);
        assert!(c.find(UID, GID, PID, 5, 100.9).is_none());
    }

    #[test]
    fn expiry_is_strict_and_swept_on_insert() {
        let c = FdCache::new();
        ins(&c, 5, 1, 100.0);
        // createtime + TIMEOUT == now → still fresh (C: < evicts)
        assert!(c.find(UID, GID, PID, 5, 101.0).is_some());
        assert!(c.find(UID, GID, PID, 5, 101.1).is_none());
        // stale entry lingers until an insert sweeps it
        ins(&c, 5 + FDCACHE_HASHSIZE as u32, 2, 102.0); // same bucket
        assert!(c.find(UID, GID, PID, 5, 102.1).is_none());
    }

    #[test]
    fn chunk_zero_data_gating() {
        let c = FdCache::new();
        let mut a = [0u8; 35];
        // flag set, data fits → kept
        c.insert(
            UID,
            GID,
            PID,
            7,
            a,
            LOOKUP_CHUNK_ZERO_DATA,
            3,
            55,
            9,
            &[1u8; 20],
            100.0,
        );
        let e = c.acquire(UID, GID, PID, 7, 100.1).unwrap();
        assert_eq!(e.lflags & LOOKUP_CHUNK_ZERO_DATA, LOOKUP_CHUNK_ZERO_DATA);
        assert_eq!(e.csdata.len(), 20);
        assert_eq!(e.chunkid, 55);
        // oversized data → flag cleared, fields zeroed
        a[0] = 1;
        c.insert(
            UID,
            GID,
            PID,
            8,
            a,
            LOOKUP_CHUNK_ZERO_DATA,
            3,
            55,
            9,
            &[1u8; CSDATA_MAX + 1],
            100.0,
        );
        let e = c.acquire(UID, GID, PID, 8, 100.1).unwrap();
        assert_eq!(e.lflags & LOOKUP_CHUNK_ZERO_DATA, 0);
        assert!(e.csdata.is_empty());
        assert_eq!(e.chunkid, 0);
    }

    #[test]
    fn invalidate_drops_inode_only() {
        let c = FdCache::new();
        ins(&c, 5, 1, 100.0);
        ins(&c, 5 + FDCACHE_HASHSIZE as u32, 2, 100.0); // same bucket
        c.invalidate(5);
        assert!(c.find(UID, GID, PID, 5, 100.1).is_none());
        assert!(
            c.find(UID, GID, PID, 5 + FDCACHE_HASHSIZE as u32, 100.1)
                .is_some()
        );
    }
}
