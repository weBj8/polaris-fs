//! PCLMULQDQ-accelerated CRC32C (Castagnoli) for x86_64.
//!
//! Port of the folding algorithm from the Intel whitepaper "Fast CRC
//! Computation for Generic Polynomials Using PCLMULQDQ Instruction" (Gopal et
//! al., 2009), mirroring crc32fast's structure but with CRC32C constants
//! (Castagnoli polynomial 0x1EDC6F41) derived via GF(2)\[x\] modular
//! arithmetic (folding distance D: hi = rev33(x^(D+32) mod P), lo =
//! rev33(x^(D-32) mod P); Barrett: P_X = rev33(P), U_PRIME = rev33(x^64 div
//! P)). Correctness is locked by property tests against the `crc32c` crate
//! (see `tests` below); the `crc32c` crate remains the fallback for short
//! inputs and unsupported CPUs.

use core::arch::x86_64 as arch;

const K1: i64 = 0x740eef02;
const K2: i64 = 0x9e4addf8;
const K3: i64 = 0xf20c0dfe;
const K4: i64 = 0x14cd00bd6;
const K5: i64 = 0xdd45aab8;
const P_X: i64 = 0x105ec76f1;
const U_PRIME: i64 = 0xdea713f1;

/// True when the CPU supports the PCLMULQDQ folding path (Zen/Intel since
/// Westmere do; detection result is cached by std).
fn available() -> bool {
    is_x86_feature_detected!("pclmulqdq")
        && is_x86_feature_detected!("sse2")
        && is_x86_feature_detected!("sse4.1")
}

/// CRC32C of `data`, continuing from `crc` (`crc32c_append` semantics).
pub(crate) fn append(crc: u32, data: &[u8]) -> u32 {
    if data.len() >= 128 && available() {
        // SAFETY: the CPU features required by `calculate` were checked above.
        unsafe { calculate(crc, data) }
    } else {
        crc32c::crc32c_append(crc, data)
    }
}

/// Folding implementation: 4-lane 128-bit folding, lane collapse, then a
/// Barrett reduction. Mirrors the reference structure exactly; only the
/// polynomial constants differ (Castagnoli instead of IEEE).
#[target_feature(enable = "pclmulqdq", enable = "sse2", enable = "sse4.1")]
unsafe fn calculate(crc: u32, mut data: &[u8]) -> u32 {
    // SAFETY: every intrinsic in this block requires pclmulqdq/sse2/sse4.1,
    // guaranteed by the `available()` gate at the call site and by the
    // `target_feature` attribute on this function. All loads are unaligned
    // (`loadu`) and stay within the slice: `get` consumes exactly 16 bytes and
    // is only called when at least 16 bytes remain.
    unsafe {
        let mut x3 = get(&mut data);
        let mut x2 = get(&mut data);
        let mut x1 = get(&mut data);
        let mut x0 = get(&mut data);
        x3 = arch::_mm_xor_si128(x3, arch::_mm_cvtsi32_si128(!crc as i32));
        let k1k2 = arch::_mm_set_epi64x(K2, K1);
        while data.len() >= 64 {
            x3 = reduce128(x3, get(&mut data), k1k2);
            x2 = reduce128(x2, get(&mut data), k1k2);
            x1 = reduce128(x1, get(&mut data), k1k2);
            x0 = reduce128(x0, get(&mut data), k1k2);
        }
        let k3k4 = arch::_mm_set_epi64x(K4, K3);
        let mut x = reduce128(x3, x2, k3k4);
        x = reduce128(x, x1, k3k4);
        x = reduce128(x, x0, k3k4);
        while data.len() >= 16 {
            x = reduce128(x, get(&mut data), k3k4);
        }
        // Reduce 128 -> 64 bits, then Barrett-reduce 64 -> 32 (bit-reflected
        // variant as in the reference implementation).
        let x = arch::_mm_xor_si128(
            arch::_mm_clmulepi64_si128(x, k3k4, 0x10),
            arch::_mm_srli_si128(x, 8),
        );
        let x = arch::_mm_xor_si128(
            arch::_mm_clmulepi64_si128(
                arch::_mm_and_si128(x, arch::_mm_set_epi32(0, 0, 0, !0)),
                arch::_mm_set_epi64x(0, K5),
                0x00,
            ),
            arch::_mm_srli_si128(x, 4),
        );
        let pu = arch::_mm_set_epi64x(U_PRIME, P_X);
        let t1 = arch::_mm_clmulepi64_si128(
            arch::_mm_and_si128(x, arch::_mm_set_epi32(0, 0, 0, !0)),
            pu,
            0x10,
        );
        let t2 = arch::_mm_clmulepi64_si128(
            arch::_mm_and_si128(t1, arch::_mm_set_epi32(0, 0, 0, !0)),
            pu,
            0x00,
        );
        let c = arch::_mm_extract_epi32(arch::_mm_xor_si128(x, t2), 1) as u32;
        // Tail shorter than 16 bytes: finish with the reference crate.
        if !data.is_empty() {
            crc32c::crc32c_append(!c, data)
        } else {
            !c
        }
    }
}

/// Fold the 128-bit `a` into `b` across one lane distance.
unsafe fn reduce128(a: arch::__m128i, b: arch::__m128i, keys: arch::__m128i) -> arch::__m128i {
    // SAFETY: same feature guarantees as `calculate` (only called from it).
    unsafe {
        let t1 = arch::_mm_clmulepi64_si128(a, keys, 0x00);
        let t2 = arch::_mm_clmulepi64_si128(a, keys, 0x11);
        arch::_mm_xor_si128(arch::_mm_xor_si128(b, t1), t2)
    }
}

/// Consume the next 16 bytes of the slice as a 128-bit lane.
unsafe fn get(a: &mut &[u8]) -> arch::__m128i {
    debug_assert!(a.len() >= 16);
    // SAFETY: callers only invoke this with >= 16 bytes remaining (checked in
    // `calculate`'s loop conditions); `loadu` permits any alignment.
    unsafe {
        let r = arch::_mm_loadu_si128(a.as_ptr() as *const arch::__m128i);
        *a = &a[16..];
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pattern(len: usize, seed: u64) -> Vec<u8> {
        let mut s = seed | 1;
        (0..len)
            .map(|_| {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                s as u8
            })
            .collect()
    }

    #[test]
    fn matches_reference_crate_all_lengths() {
        for len in 0..=600usize {
            let data = pattern(len, len as u64 + 1);
            assert_eq!(
                append(0, &data),
                crc32c::crc32c(&data),
                "mismatch at len {len}"
            );
        }
        for len in [
            1000usize,
            4096,
            65535,
            65536,
            65537,
            1 << 20,
            (1 << 20) + 13,
        ] {
            let data = pattern(len, len as u64);
            assert_eq!(
                append(0, &data),
                crc32c::crc32c(&data),
                "mismatch at len {len}"
            );
        }
    }

    #[test]
    fn matches_reference_crate_chunked() {
        for split in [1usize, 15, 16, 17, 63, 64, 65, 127, 128, 137, 1000] {
            let data = pattern(5000, split as u64);
            let mut crc = 0u32;
            for chunk in data.chunks(split) {
                crc = append(crc, chunk);
            }
            assert_eq!(crc, crc32c::crc32c(&data), "mismatch at split {split}");
        }
    }
}
