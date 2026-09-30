//! RC4-based random generator — port of mfscommon/random.c (`_USE_PTHREADS`).
//!
//! Shared by plfsmaster and plfschunkserver (their c2rust copies differed
//! only in pthread plumbing). The RC4 state lives behind a `Mutex`, which
//! replaces the C `pthread_mutex_t`. The key is still drawn from glibc
//! `srandom`/`random`: that PRNG is process-global state which other
//! modules (sockets, matoclserv) read after `rnd_init` reseeds it, so it
//! stays the libc one rather than getting a private Rust copy.
//!
//! Byte order: `rndu32`/`rndu64` fill the result's bytes in memory order,
//! i.e. little-endian on the supported targets (`from_le_bytes` below).

use std::sync::Mutex;

pub use imp::Rc4;

#[deny(unsafe_code)]
mod imp {
    /// RC4 keystream state (C statics `i`, `j`, `p[256]`).
    pub struct Rc4 {
        i: u8,
        j: u8,
        p: [u8; 256],
    }

    impl Rc4 {
        pub const fn new() -> Self {
            Rc4 { i: 0, j: 0, p: [0; 256] }
        }

        /// C `rnd_init` key schedule: identity permutation, then 768 mixing
        /// rounds for each of the two 64-byte keys. `j` is carried over
        /// from the previous state, as in C.
        pub fn init(&mut self, key: &[u8; 64], vkey: &[u8; 64]) {
            for l in 0..256 {
                self.p[l] = l as u8;
            }
            for k in [key, vkey] {
                for l in 0..768usize {
                    self.i = (l & 0xFF) as u8;
                    let x = self.j.wrapping_add(self.p[self.i as usize]).wrapping_add(k[l % 64]);
                    self.j = self.p[x as usize];
                    self.p.swap(self.i as usize, self.j as usize);
                }
            }
            self.i = 0;
        }

        /// C `RND_RC4_STEP`.
        pub fn step(&mut self) -> u8 {
            let p = &mut self.p;
            let x = self.j.wrapping_add(p[self.i as usize]);
            self.j = p[x as usize];
            let x = p[self.j as usize];
            let x = p[x as usize].wrapping_add(1);
            let r = p[x as usize];
            p.swap(self.i as usize, self.j as usize);
            self.i = self.i.wrapping_add(1);
            r
        }

        pub fn fill(&mut self, buf: &mut [u8]) {
            for b in buf {
                *b = self.step();
            }
        }
    }

    impl Default for Rc4 {
        fn default() -> Self {
            Self::new()
        }
    }

    /// C `rnd*_ranged` rejection sampling: `max` is the largest multiple of
    /// `range` representable (0 means every value is fair).
    pub fn ranged<T>(range: T, mut next: impl FnMut() -> T) -> T
    where
        T: Copy + PartialEq + PartialOrd + From<u8> + std::ops::Rem<Output = T> + WrappingNeg,
    {
        let mut r = next();
        let zero = T::from(0);
        if range == zero {
            return r;
        }
        let max = (range.wneg() % range).wneg();
        if max != zero {
            while r >= max {
                r = next();
            }
        }
        r % range
    }

    pub trait WrappingNeg {
        fn wneg(self) -> Self;
    }
    impl WrappingNeg for u32 {
        fn wneg(self) -> Self {
            self.wrapping_neg()
        }
    }
    impl WrappingNeg for u64 {
        fn wneg(self) -> Self {
            self.wrapping_neg()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// Reference RC4-variant from random.c transcribed literally with
        /// wrapping u8 arithmetic.
        fn c_reference(key: &[u8; 64], vkey: &[u8; 64], n: usize) -> Vec<u8> {
            let (mut i, mut j) = (0u8, 0u8);
            let mut p = [0u8; 256];
            for l in 0..256 {
                p[l] = l as u8;
            }
            for k in [key, vkey] {
                for l in 0..768u16 {
                    i = (l & 0xFF) as u8;
                    let mut x = j.wrapping_add(p[i as usize]).wrapping_add(k[(l % 64) as usize]);
                    j = p[x as usize];
                    x = p[i as usize];
                    p[i as usize] = p[j as usize];
                    p[j as usize] = x;
                }
            }
            i = 0;
            (0..n)
                .map(|_| {
                    let mut x = j.wrapping_add(p[i as usize]);
                    j = p[x as usize];
                    x = p[j as usize];
                    x = p[x as usize].wrapping_add(1);
                    let r = p[x as usize];
                    x = p[i as usize];
                    p[i as usize] = p[j as usize];
                    p[j as usize] = x;
                    i = i.wrapping_add(1);
                    r
                })
                .collect()
        }

        #[test]
        fn keystream_matches_c_transcription() {
            let mut key = [0u8; 64];
            let mut vkey = [0u8; 64];
            for n in 0..64 {
                key[n] = (n as u8).wrapping_mul(37).wrapping_add(11);
                vkey[n] = (n as u8).wrapping_mul(101) ^ 0x5a;
            }
            let mut r = Rc4::new();
            r.init(&key, &vkey);
            let mut out = vec![0u8; 4096];
            r.fill(&mut out);
            assert_eq!(out, c_reference(&key, &vkey, 4096));
        }

        #[test]
        fn ranged_is_fair_reduction() {
            let mut seq = [5u32, u32::MAX, 7].into_iter();
            // range 3: max = -(2^32 mod 3) = 2^32-1 → u32::MAX is rejected
            assert_eq!(ranged(3u32, || seq.next().unwrap()), 2);
            assert_eq!(ranged(3u32, || seq.next().unwrap()), 7 % 3);
            assert_eq!(ranged(0u32, || 42), 42);
            assert_eq!(ranged(1u64 << 32, || 0x1_2345_6789u64), 0x2345_6789);
        }
    }
}

// glibc PRNG (not exposed by the libc crate on this target).
unsafe extern "C" {
    fn srandom(seed: core::ffi::c_uint);
    fn random() -> core::ffi::c_long;
}

static STATE: Mutex<Rc4> = Mutex::new(Rc4::new());

fn state() -> std::sync::MutexGuard<'static, Rc4> {
    STATE.lock().unwrap_or_else(|e| e.into_inner())
}

/// C: `rnd_init` — reseeds glibc `random()` with time+monotonic clock and
/// keys the RC4 state from it.
#[unsafe(no_mangle)]
pub extern "C" fn rnd_init() -> core::ffi::c_int {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let seed = secs.wrapping_add(crate::clocks::monotonic_useconds());
    let mut key = [0u8; 64];
    let mut vkey = [0u8; 64];
    // SAFETY: srandom/random only touch glibc's internal locked PRNG state.
    unsafe {
        srandom(seed as core::ffi::c_uint);
        for l in 0..64 {
            key[l] = random() as u8;
            vkey[l] = random() as u8;
        }
    }
    state().init(&key, &vkey);
    0
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn rndu8() -> u8 {
    state().step()
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn rndu32() -> u32 {
    let mut b = [0u8; 4];
    state().fill(&mut b);
    u32::from_le_bytes(b)
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn rndu64() -> u64 {
    let mut b = [0u8; 8];
    state().fill(&mut b);
    u64::from_le_bytes(b)
}

/// # Safety
/// `buff` must be writable for `size` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rndbuff(buff: *mut u8, size: u32) {
    if size == 0 {
        return;
    }
    // SAFETY: per fn contract.
    let buf = unsafe { std::slice::from_raw_parts_mut(buff, size as usize) };
    state().fill(buf);
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn rndu64_ranged(range: u64) -> u64 {
    imp::ranged(range, || rndu64())
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn rndu32_ranged(range: u32) -> u32 {
    imp::ranged(range, || rndu32())
}
