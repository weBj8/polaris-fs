//! CRC32 (zlib poly) with slicing-by-16 and combine tables, migrated to safe
//! Rust (P1). Tables moved from `static mut` + explicit init to `LazyLock`
//! (identical contents, computed on first use); `mycrc32_init` remains as an
//! exported no-op because consumers call it by symbol.
//!
//! PORT NOTE: the original aligned the data pointer to 4 before the u32
//! block loop. That was a performance detail, not semantics — CRC output
//! depends only on the byte sequence — so the safe version uses
//! `u32::from_le_bytes` over chunks (x86_64 little-endian target).

pub type uint8_t = u8;
pub type uint32_t = u32;
pub const CRC_POLY: ::core::ffi::c_uint = 0xedb88320 as ::core::ffi::c_uint;

use std::sync::LazyLock;

type MainTables = [[uint32_t; 256]; 16];
type CombineTables = [[[uint32_t; 256]; 4]; 32];

#[deny(unsafe_code)]
mod imp {
    use super::{CRC_POLY, CombineTables, MainTables};

    pub fn generate_main_tables() -> MainTables {
        let mut t = [[0u32; 256]; 16];
        let poly = CRC_POLY;
        for (i, e) in t[0].iter_mut().enumerate() {
            let mut c = i as u32;
            for _ in 0..8 {
                c = if c & 1 != 0 { poly ^ (c >> 1) } else { c >> 1 };
            }
            *e = c;
        }
        let row0 = t[0]; // copy lets us mutate later rows while reading row 0
        for i in 0..256 {
            let mut c = row0[i];
            for row in t.iter_mut().skip(1) {
                c = row0[(c & 0xff) as usize] ^ (c >> 8);
                row[i] = c;
            }
        }
        t
    }

    fn matrix_square(m: &[u32; 32]) -> [u32; 32] {
        let mut sqr = [0u32; 32];
        for (i, &v0) in m.iter().enumerate() {
            let mut s = 0u32;
            let mut v = v0;
            let mut j = 0;
            while v != 0 && j < 32 {
                if v & 1 != 0 {
                    s ^= m[j];
                }
                j += 1;
                v >>= 1;
            }
            sqr[i] = s;
        }
        sqr
    }

    pub fn generate_combine_tables() -> CombineTables {
        let mut ct = [[[0u32; 256]; 4]; 32];
        let mut m1 = [0u32; 32];
        m1[0] = CRC_POLY;
        let mut j = 1u32;
        for e in m1.iter_mut().skip(1) {
            *e = j;
            j <<= 1;
        }
        let m2 = matrix_square(&m1);
        let mut m1 = matrix_square(&m2);
        let mut m2 = m2;
        #[allow(clippy::needless_range_loop)]
        for i in 0..32 {
            let mc = if i & 1 != 0 {
                m1 = matrix_square(&m2);
                &m1
            } else {
                m2 = matrix_square(&m1);
                &m2
            };
            for j in 0..4 {
                for k in 0..256 {
                    let mut sum = 0u32;
                    let mut l = k as u32;
                    let mut m = j * 8;
                    while l != 0 {
                        if l & 1 != 0 {
                            sum ^= mc[m];
                        }
                        l >>= 1;
                        m += 1;
                    }
                    ct[i][j][k] = sum;
                }
            }
        }
        ct
    }

    pub fn crc_compute(mut crc: u32, data: &[u8], t: &MainTables) -> u32 {
        crc = !crc;
        let mut chunks = data.chunks_exact(16);
        for b in &mut chunks {
            let d0 = u32::from_le_bytes([b[0], b[1], b[2], b[3]]) ^ crc;
            let d1 = u32::from_le_bytes([b[4], b[5], b[6], b[7]]);
            let d2 = u32::from_le_bytes([b[8], b[9], b[10], b[11]]);
            let d3 = u32::from_le_bytes([b[12], b[13], b[14], b[15]]);
            crc = t[0][((d3 >> 24) & 0xff) as usize]
                ^ t[1][((d3 >> 16) & 0xff) as usize]
                ^ t[2][((d3 >> 8) & 0xff) as usize]
                ^ t[3][(d3 & 0xff) as usize]
                ^ t[4][((d2 >> 24) & 0xff) as usize]
                ^ t[5][((d2 >> 16) & 0xff) as usize]
                ^ t[6][((d2 >> 8) & 0xff) as usize]
                ^ t[7][(d2 & 0xff) as usize]
                ^ t[8][((d1 >> 24) & 0xff) as usize]
                ^ t[9][((d1 >> 16) & 0xff) as usize]
                ^ t[10][((d1 >> 8) & 0xff) as usize]
                ^ t[11][(d1 & 0xff) as usize]
                ^ t[12][((d0 >> 24) & 0xff) as usize]
                ^ t[13][((d0 >> 16) & 0xff) as usize]
                ^ t[14][((d0 >> 8) & 0xff) as usize]
                ^ t[15][(d0 & 0xff) as usize];
        }
        for &byte in chunks.remainder() {
            crc = (crc >> 8) ^ t[0][((crc & 0xff) ^ byte as u32) as usize];
        }
        !crc
    }
}

static CRC_TABLE: LazyLock<MainTables> = LazyLock::new(imp::generate_main_tables);
static CRC_COMBINE_TABLE: LazyLock<CombineTables> = LazyLock::new(imp::generate_combine_tables);

/// # Safety
/// `data` must be readable for `leng` bytes (C caller contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mycrc32(
    crc: uint32_t,
    data: *const ::core::ffi::c_void,
    leng: uint32_t,
) -> uint32_t {
    if leng == 0 {
        return imp::crc_compute(crc, &[], &CRC_TABLE);
    }
    // SAFETY: per fn contract; leng>0 here, data valid for leng bytes.
    let buf = unsafe { std::slice::from_raw_parts(data as *const u8, leng as usize) };
    imp::crc_compute(crc, buf, &CRC_TABLE)
}

#[unsafe(no_mangle)]
pub extern "C" fn mycrc32_combine(
    mut crc1: uint32_t,
    crc2: uint32_t,
    mut leng2: uint32_t,
) -> uint32_t {
    let ct = &*CRC_COMBINE_TABLE;
    let mut i: uint8_t = 0;
    while leng2 != 0 {
        if leng2 & 1 != 0 {
            crc1 = ct[i as usize][3][(crc1 >> 24) as usize]
                ^ ct[i as usize][2][((crc1 >> 16) & 0xff) as usize]
                ^ ct[i as usize][1][((crc1 >> 8) & 0xff) as usize]
                ^ ct[i as usize][0][(crc1 & 0xff) as usize];
        }
        i = i.wrapping_add(1);
        leng2 >>= 1;
    }
    crc1 ^ crc2
}

/// Exported no-op: tables are lazy now. Kept because consumers call it by
/// symbol at startup; calling it still forces table generation (harmless).
#[unsafe(no_mangle)]
pub extern "C" fn mycrc32_init() {
    LazyLock::force(&CRC_TABLE);
    LazyLock::force(&CRC_COMBINE_TABLE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_vectors() {
        // zlib crc32 reference values
        assert_eq!(imp::crc_compute(0, b"123456789", &CRC_TABLE), 0xCBF43926);
        assert_eq!(imp::crc_compute(0, b"", &CRC_TABLE), 0);
        assert_eq!(
            imp::crc_compute(0xdeadbeef, b"hello world", &CRC_TABLE),
            imp::crc_compute(0xdeadbeef, b"hello ", &CRC_TABLE)
                .pipe(|c| { imp::crc_compute(c, b"world", &CRC_TABLE) })
        );
    }

    #[test]
    fn combine_matches_concat() {
        let a = b"the quick brown fox ";
        let b = b"jumps over the lazy dog";
        let whole = imp::crc_compute(0, &[&a[..], &b[..]].concat(), &CRC_TABLE);
        let ca = imp::crc_compute(0, a, &CRC_TABLE);
        let cb = imp::crc_compute(0, b, &CRC_TABLE);
        assert_eq!(mycrc32_combine(ca, cb, b.len() as u32), whole);
    }

    trait Pipe: Sized {
        fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
            f(self)
        }
    }
    impl Pipe for u32 {}
}
