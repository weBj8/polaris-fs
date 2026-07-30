//! File descriptor (open-context) cache — safe Rust rewrite (P4).
//!
//! Original: MooseFS mfsclient/fdcache.c. Per-(inode,uid,gid,pid) cache of
//! lookup answers with a 1s lifetime, optionally carrying chunk-zero data
//! injected into chunksdatacache on release.
//!
//! Safe core in `imp` (per-bucket std Mutex + Vec); boundary keeps the
//! monotonic clock and the chunksdatacache call. The acquire/release
//! contract changes shape: C returned a raw entry pointer released later;
//! here acquire returns an owned FdEntry (RAII-safe, same lifetime rules —
//! mfs_fuse calls fdcache_release/fdcache_inject_chunkdata on it).

unsafe extern "C" {
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn chunksdatacache_insert(
        inode: uint32_t,
        chunkindx: uint32_t,
        chunkid: uint64_t,
        chunkversion: uint32_t,
        csdataver: uint8_t,
        csdata: *const uint8_t,
        csdatasize: uint32_t,
    );
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
                buckets: (0..FDCACHE_HASHSIZE).map(|_| Mutex::new(Vec::new())).collect(),
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
        pub fn find(&self, uid: u32, gid: u32, pid: i32, inode: u32, now: f64) -> Option<([u8; 35], u16)> {
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

        /// Like find but removes the entry and hands ownership to the
        /// caller (C: unlinked pointer + later fdcache_release).
        pub fn acquire(&self, uid: u32, gid: u32, pid: i32, inode: u32, now: f64) -> Option<FdEntry> {
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

use imp::FdCache;

static mut FDCACHE: Option<FdCache> = None;

/// SAFETY: set once in fdcache_init before FUSE threads start; never
/// cleared (C kept the tables for process lifetime too).
unsafe fn fdcache() -> &'static FdCache {
    unsafe { (*(&raw const FDCACHE)).as_ref().unwrap_unchecked() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdcache_insert(
    ctx: *const fuse_ctx,
    inode: uint32_t,
    attr: *mut uint8_t,
    lflags: uint16_t,
    csdataver: uint8_t,
    chunkid: uint64_t,
    version: uint32_t,
    csdata: *const uint8_t,
    csdatasize: uint32_t,
) {
    unsafe {
        let now = monotonic_seconds();
        let a: [u8; 35] = ::core::ptr::read(attr as *const [u8; 35]);
        let data = if csdata.is_null() || csdatasize == 0 {
            &[][..]
        } else {
            ::core::slice::from_raw_parts(csdata, csdatasize as usize)
        };
        fdcache().insert(
            (*ctx).uid,
            (*ctx).gid,
            (*ctx).pid,
            inode,
            a,
            lflags,
            csdataver,
            chunkid,
            version,
            data,
            now,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdcache_invalidate(inode: uint32_t) {
    unsafe {
        fdcache().invalidate(inode);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdcache_find(
    ctx: *const fuse_ctx,
    inode: uint32_t,
    attr: *mut uint8_t,
    lflags: *mut uint16_t,
) -> uint8_t {
    unsafe {
        let now = monotonic_seconds();
        match fdcache().find((*ctx).uid, (*ctx).gid, (*ctx).pid, inode, now) {
            Some((a, lf)) => {
                if !attr.is_null() {
                    ::core::ptr::copy_nonoverlapping(a.as_ptr(), attr, 35);
                }
                if !lflags.is_null() {
                    *lflags = lf;
                }
                1
            }
            None => 0,
        }
    }
}

/// C returned a raw entry pointer; here the owned entry is boxed so the
/// release/inject calls below stay pointer-compatible with mfs_fuse.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdcache_acquire(
    ctx: *const fuse_ctx,
    inode: uint32_t,
    attr: *mut uint8_t,
    lflags: *mut uint16_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let now = monotonic_seconds();
        match fdcache().acquire((*ctx).uid, (*ctx).gid, (*ctx).pid, inode, now) {
            Some(e) => {
                if !attr.is_null() {
                    ::core::ptr::copy_nonoverlapping(e.attr.as_ptr(), attr, 35);
                }
                if !lflags.is_null() {
                    *lflags = e.lflags;
                }
                Box::into_raw(Box::new(e)) as *mut ::core::ffi::c_void
            }
            None => ::core::ptr::null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdcache_release(vfdce: *mut ::core::ffi::c_void) {
    if !vfdce.is_null() {
        // SAFETY: handle from fdcache_acquire, released at most once (same
        // contract as the C fdcachee_free).
        drop(unsafe { Box::from_raw(vfdce as *mut imp::FdEntry) });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdcache_inject_chunkdata(vfdce: *mut ::core::ffi::c_void) {
    unsafe {
        // SAFETY: handle from fdcache_acquire; caller keeps ownership and
        // must still call fdcache_release (C did not free here either).
        let e = &*(vfdce as *const imp::FdEntry);
        if e.lflags & LOOKUP_CHUNK_ZERO_DATA != 0 {
            chunksdatacache_insert(
                e.inode,
                0,
                e.chunkid,
                e.version,
                e.csdataver,
                e.csdata.as_ptr(),
                e.csdata.len() as uint32_t,
            );
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdcache_init() {
    unsafe {
        FDCACHE = Some(FdCache::new());
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;
    use super::LOOKUP_CHUNK_ZERO_DATA;

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
        c.insert(UID, GID, PID, 7, a, LOOKUP_CHUNK_ZERO_DATA, 3, 55, 9, &[1u8; 20], 100.0);
        let e = c.acquire(UID, GID, PID, 7, 100.1).unwrap();
        assert_eq!(e.lflags & LOOKUP_CHUNK_ZERO_DATA, LOOKUP_CHUNK_ZERO_DATA);
        assert_eq!(e.csdata.len(), 20);
        assert_eq!(e.chunkid, 55);
        // oversized data → flag cleared, fields zeroed
        a[0] = 1;
        c.insert(UID, GID, PID, 8, a, LOOKUP_CHUNK_ZERO_DATA, 3, 55, 9, &[1u8; CSDATA_MAX + 1], 100.0);
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
        assert!(c.find(UID, GID, PID, 5 + FDCACHE_HASHSIZE as u32, 100.1).is_some());
    }
}
