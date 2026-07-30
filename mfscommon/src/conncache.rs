//! Connection cache (LRU pool of keep-alive sockets) with a janitor thread,
//! migrated to safe Rust (P1).
//!
//! Design mapping from the c2rust original: the malloc'd intrusive-list pool
//! becomes a `Vec<ConnEntry>` plus index-based free list / hash buckets /
//! LRU deque inside one `Mutex<Cache>`. The pthread mutex + abort-on-error
//! macro taxonomy collapses into std's Mutex (lock failure = poison =
//! abort-equivalent panic). Keep-alive logic is byte-for-byte the original's
//! (8-byte NOP probe: read, check, write).
//!
//! ABI preserved: init/term/insert/get/keepalive_thread keep their C symbols
//! for the consumer daemons.

pub type size_t = usize;
pub type pthread_t = ::core::ffi::c_ulong;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub const CONN_CACHE_HASHSIZE: usize = 256;

use std::collections::VecDeque;
use std::sync::Mutex;

unsafe extern "C" {
    fn monotonic_useconds() -> uint64_t;
    fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn lwt_minthread_create(
        th: *mut pthread_t,
        detached: uint8_t,
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_join(th: pthread_t, thread_return: *mut *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
}

#[inline]
fn hash32(mut key: uint32_t) -> uint32_t {
    key = (!key).wrapping_add(key << 15);
    key ^= key >> 12;
    key = key.wrapping_add(key << 2);
    key ^= key >> 4;
    key = key.wrapping_mul(2057);
    key ^= key >> 16;
    key
}

fn bucket_of(ip: uint32_t, port: uint16_t) -> usize {
    hash32(ip ^ ((port as uint32_t) << 16)) as usize % CONN_CACHE_HASHSIZE
}

fn portable_usleep(usec: uint64_t) {
    let mut req = libc::timespec {
        tv_sec: (usec / 1_000_000) as libc::time_t,
        tv_nsec: (usec % 1_000_000 * 1000) as _,
    };
    // SAFETY: req/rem valid timespecs; EINTR retry loop as the original.
    unsafe {
        let mut rem: libc::timespec = std::mem::zeroed();
        while libc::nanosleep(&req, &mut rem) < 0 {
            req = rem;
        }
    }
}

#[derive(Clone, Copy, Default)]
struct ConnEntry {
    ip: uint32_t,
    port: uint16_t,
    fd: ::core::ffi::c_int, // -1 = free
}

impl ConnEntry {
    fn live(&self) -> bool {
        self.fd >= 0
    }
}

#[derive(Default)]
struct Cache {
    entries: Vec<ConnEntry>,
    buckets: Vec<Vec<u32>>, // CONN_CACHE_HASHSIZE buckets
    lru: VecDeque<u32>,      // front = oldest
    free: Vec<u32>,
    keep_alive: bool,
}

impl Cache {
    fn remove_idx(&mut self, idx: u32, close: bool) {
        let e = &mut self.entries[idx as usize];
        if close {
            // SAFETY: extern; fd owned by this entry.
            unsafe { tcpclose(e.fd) };
        }
        e.fd = -1;
        let e = *e; // ip/port unchanged; bucket keyed on them
        let b = &mut self.buckets[bucket_of(e.ip, e.port)];
        if let Some(pos) = b.iter().position(|&x| x == idx) {
            b.swap_remove(pos);
        }
        if let Some(pos) = self.lru.iter().position(|&x| x == idx) {
            self.lru.remove(pos);
        }
        self.free.push(idx);
    }
}

// None = not initialized / after term. pthread_t of the janitor lives
// outside the cache (joined before the cache is dropped, as the original).
static CACHE: Mutex<Option<Cache>> = Mutex::new(None);
static JANITOR: Mutex<Option<pthread_t>> = Mutex::new(None);

#[unsafe(no_mangle)]
pub extern "C" fn conncache_insert(ip: uint32_t, port: uint16_t, fd: ::core::ffi::c_int) {
    let mut guard = CACHE.lock().unwrap();
    let c = guard.as_mut().expect("conncache_insert before init");
    if c.free.is_empty() {
        // evict oldest (original: conncache_remove(lruhead, close=1))
        let oldest = *c.lru.front().expect("pool empty but free list empty");
        c.remove_idx(oldest, true);
    }
    let idx = c.free.pop().unwrap();
    c.entries[idx as usize] = ConnEntry { ip, port, fd };
    c.lru.push_back(idx);
    c.buckets[bucket_of(ip, port)].push(idx);
}

#[unsafe(no_mangle)]
pub extern "C" fn conncache_get(ip: uint32_t, port: uint16_t) -> ::core::ffi::c_int {
    let mut guard = CACHE.lock().unwrap();
    let c = guard.as_mut().expect("conncache_get before init");
    let b = bucket_of(ip, port);
    let idx = c.buckets[b]
        .iter()
        .copied()
        .find(|&i| {
            let e = &c.entries[i as usize];
            e.live() && e.ip == ip && e.port == port
        });
    match idx {
        Some(i) => {
            let fd = c.entries[i as usize].fd;
            c.remove_idx(i, false);
            fd
        }
        None => -1,
    }
}

/// Janitor: every ~5ms probe 1/200th of the pool with an 8-byte NOP
/// (read → if any byte nonzero, drop; else write NOP back; drop on any
/// error). Logic byte-for-byte from the original.
///
/// # Safety
/// pthread start-routine signature; `arg` passed through, as the original.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn conncache_keepalive_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut p: uint32_t = 0;
    loop {
        let st = unsafe { monotonic_useconds() }; // SAFETY: extern, no args
        {
            let mut guard = CACHE.lock().unwrap();
            let Some(c) = guard.as_mut() else { return arg }; // term raced us
            let capacity = c.entries.len() as uint32_t;
            let mut q = p;
            while q < capacity {
                let fd = c.entries[q as usize].fd;
                if fd >= 0 {
                    let mut nopbuff = [0u8; 8];
                    // SAFETY: extern; nopbuff valid for 8 bytes.
                    let mut i = unsafe {
                        libc::read(fd, nopbuff.as_mut_ptr() as *mut _, 8) as ::core::ffi::c_int
                    };
                    if i < 0 {
                        let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
                        if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
                            nopbuff = [0; 8];
                            i = 8;
                        }
                    }
                    if i != 8 || nopbuff.iter().any(|&b| b != 0) {
                        c.remove_idx(q, true);
                    } else {
                        nopbuff = [0; 8];
                        // SAFETY: extern; nopbuff valid for 8 bytes.
                        let w = unsafe {
                            libc::write(fd, nopbuff.as_ptr() as *const _, 8) as ::core::ffi::c_int
                        };
                        if w != 8 {
                            c.remove_idx(q, true);
                        }
                    }
                }
                q = q.wrapping_add(200);
            }
            p = p.wrapping_add(1);
            if p >= 200 {
                p = 0;
            }
            if !c.keep_alive {
                return arg;
            }
        }
        let en = unsafe { monotonic_useconds() }; // SAFETY: extern, no args
        let elapsed = en.wrapping_sub(st);
        if elapsed < 5000 {
            portable_usleep(5000 - elapsed);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn conncache_term() {
    if let Some(c) = CACHE.lock().unwrap().as_mut() {
        c.keep_alive = false;
    }
    let th = JANITOR.lock().unwrap().take();
    if let Some(th) = th {
        // SAFETY: extern; th is the janitor, joinable, outlives this call.
        unsafe { pthread_join(th, std::ptr::null_mut()) };
    }
    if let Some(c) = CACHE.lock().unwrap().take() {
        for e in &c.entries {
            if e.live() {
                // SAFETY: extern; fd owned by the cache.
                unsafe { tcpclose(e.fd) };
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn conncache_init(mut cap: uint32_t) -> ::core::ffi::c_int {
    let mut c = Cache::default();
    c.entries = vec![ConnEntry { ip: 0, port: 0, fd: -1 }; cap as usize];
    c.buckets = vec![Vec::new(); CONN_CACHE_HASHSIZE];
    c.free = (0..cap).rev().collect();
    c.keep_alive = true;
    *CACHE.lock().unwrap() = Some(c);

    let mut th: pthread_t = 0;
    // SAFETY: th written by the call; thread fn is our janitor; joinable
    // (detached=0), joined in conncache_term.
    let rc = unsafe {
        lwt_minthread_create(
            &mut th,
            0,
            Some(conncache_keepalive_thread),
            std::ptr::null_mut(),
        )
    };
    if rc < 0 {
        *CACHE.lock().unwrap() = None;
        return -1;
    }
    *JANITOR.lock().unwrap() = Some(th);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh(cap: u32) {
        let mut c = Cache::default();
        c.entries = vec![ConnEntry { ip: 0, port: 0, fd: -1 }; cap as usize];
        c.buckets = vec![Vec::new(); CONN_CACHE_HASHSIZE];
        c.free = (0..cap).rev().collect();
        c.keep_alive = true;
        *CACHE.lock().unwrap() = Some(c);
    }

    #[test]
    fn insert_get_roundtrip() {
        fresh(4);
        conncache_insert(0x7f000001, 9420, 42);
        assert_eq!(conncache_get(0x7f000001, 9420), 42);
        // get removes from cache — second lookup misses
        assert_eq!(conncache_get(0x7f000001, 9420), -1);
        *CACHE.lock().unwrap() = None;
    }

    #[test]
    fn lru_eviction_oldest() {
        fresh(2);
        conncache_insert(1, 1, 10);
        conncache_insert(2, 2, 20);
        // pool full → inserting third evicts fd 10 (oldest).
        // NOTE: eviction calls tcpclose, which the test binary does not
        // provide — so test eviction logic on Cache directly instead.
        *CACHE.lock().unwrap() = None;
        let mut c = Cache::default();
        c.entries = vec![ConnEntry { ip: 0, port: 0, fd: -1 }; 2];
        c.buckets = vec![Vec::new(); CONN_CACHE_HASHSIZE];
        c.free = vec![];
        c.lru = VecDeque::from(vec![0, 1]);
        c.entries[0] = ConnEntry { ip: 1, port: 1, fd: 10 };
        c.entries[1] = ConnEntry { ip: 2, port: 2, fd: 20 };
        c.buckets[bucket_of(1, 1)].push(0);
        c.buckets[bucket_of(2, 2)].push(1);
        assert_eq!(*c.lru.front().unwrap(), 0); // oldest first
        let oldest = *c.lru.front().unwrap();
        c.remove_idx(oldest, false); // close=false avoids tcpclose in test
        assert_eq!(c.entries[0].fd, -1);
        assert_eq!(c.free, vec![0]);
        assert!(c.buckets[bucket_of(1, 1)].is_empty());
    }
}
