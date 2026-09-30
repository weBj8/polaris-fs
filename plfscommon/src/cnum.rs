//! C numeric string conversions with glibc C-locale semantics, in safe Rust.
//!
//! Replaces the libc `strtol`/`strtoul`/`strtoll`/`strtoull` (base 0),
//! `strtod` and `printf("%.6f")` calls made by the MooseFS C sources.
//! Every function reports the end index (C `*endptr`, as an offset into the
//! input) and whether C would have set `errno = ERANGE`; callers that need
//! the errno side effect apply it at their C boundary.
//!
//! Grammar and rounding are verified against the host glibc by the
//! differential tests at the bottom of this file.
#![deny(unsafe_code)]

fn at(s: &[u8], i: usize) -> u8 {
    s.get(i).copied().unwrap_or(0)
}

/// C-locale `isspace`.
fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

fn digit_value(b: u8) -> Option<u64> {
    match b {
        b'0'..=b'9' => Some((b - b'0') as u64),
        b'a'..=b'z' => Some((b - b'a') as u64 + 10),
        b'A'..=b'Z' => Some((b - b'A') as u64 + 10),
        _ => None,
    }
}

/// Result of an integer scan: magnitude, sign, end offset, u64 overflow.
struct IntScan {
    mag: u64,
    neg: bool,
    end: usize,
    overflow: bool,
}

/// Shared base-0 scanner of glibc `strtol`/`strtoul`. No conversion yields
/// `end == 0` (C: `*endptr = nptr`).
fn scan_int(s: &[u8]) -> IntScan {
    let mut i = 0;
    while is_space(at(s, i)) {
        i += 1;
    }
    let mut neg = false;
    if at(s, i) == b'-' {
        neg = true;
        i += 1;
    } else if at(s, i) == b'+' {
        i += 1;
    }
    let base: u64 = if at(s, i) == b'0' {
        if at(s, i + 1) | 0x20 == b'x' && at(s, i + 2).is_ascii_hexdigit() {
            i += 2;
            16
        } else {
            8
        }
    } else {
        10
    };
    let start = i;
    let mut mag: u64 = 0;
    let mut overflow = false;
    while let Some(d) = digit_value(at(s, i)).filter(|&d| d < base) {
        match mag.checked_mul(base).and_then(|x| x.checked_add(d)) {
            Some(x) => mag = x,
            None => overflow = true,
        }
        i += 1;
    }
    if i == start {
        return IntScan { mag: 0, neg: false, end: 0, overflow: false };
    }
    IntScan { mag, neg, end: i, overflow }
}

/// glibc `strtol(s, &end, 0)` on LP64 (also `strtoll`).
/// Returns (value, end offset, ERANGE).
pub fn strtol(s: &[u8]) -> (i64, usize, bool) {
    let r = scan_int(s);
    if r.neg {
        if r.overflow || r.mag > 1u64 << 63 {
            (i64::MIN, r.end, true)
        } else {
            ((r.mag as i64).wrapping_neg(), r.end, false)
        }
    } else if r.overflow || r.mag > i64::MAX as u64 {
        (i64::MAX, r.end, true)
    } else {
        (r.mag as i64, r.end, false)
    }
}

/// glibc `strtoul(s, &end, 0)` on LP64 (also `strtoull`): negative inputs
/// wrap, overflow saturates to `ULONG_MAX` with ERANGE.
pub fn strtoul(s: &[u8]) -> (u64, usize, bool) {
    let r = scan_int(s);
    if r.overflow {
        (u64::MAX, r.end, true)
    } else if r.neg {
        (r.mag.wrapping_neg(), r.end, false)
    } else {
        (r.mag, r.end, false)
    }
}

fn starts_with_ci(s: &[u8], i: usize, pat: &[u8]) -> bool {
    s.len() >= i + pat.len() && s[i..i + pat.len()].eq_ignore_ascii_case(pat)
}

/// Scan an optional exponent (`e`/`p` already matched at `j`): returns the
/// end offset and the (saturated) signed exponent, or None if no digits
/// follow (then C does not consume the exponent marker).
fn scan_exponent(s: &[u8], j: usize) -> Option<(usize, i64)> {
    let mut k = j + 1;
    let mut neg = false;
    if at(s, k) == b'-' {
        neg = true;
        k += 1;
    } else if at(s, k) == b'+' {
        k += 1;
    }
    if !at(s, k).is_ascii_digit() {
        return None;
    }
    let mut e: i64 = 0;
    while at(s, k).is_ascii_digit() {
        e = (e * 10 + (at(s, k) - b'0') as i64).min(1 << 40);
        k += 1;
    }
    Some((k, if neg { -e } else { e }))
}

/// glibc `strtod(s, &end)` in the C locale. Returns (value, end, ERANGE).
pub fn strtod(s: &[u8]) -> (f64, usize, bool) {
    let mut i = 0;
    while is_space(at(s, i)) {
        i += 1;
    }
    let mut neg = false;
    if at(s, i) == b'-' {
        neg = true;
        i += 1;
    } else if at(s, i) == b'+' {
        i += 1;
    }
    let sign = |v: f64| if neg { -v } else { v };
    if starts_with_ci(s, i, b"inf") {
        let end = if starts_with_ci(s, i + 3, b"inity") { i + 8 } else { i + 3 };
        return (sign(f64::INFINITY), end, false);
    }
    if starts_with_ci(s, i, b"nan") {
        let mut end = i + 3;
        let mut bits: u64 = 0x7ff8_0000_0000_0000;
        if at(s, end) == b'(' {
            let mut j = end + 1;
            while at(s, j).is_ascii_alphanumeric() || at(s, j) == b'_' {
                j += 1;
            }
            if at(s, j) == b')' {
                let inner = &s[end + 1..j];
                let (mant, pend, _) = strtoul(inner);
                if pend == inner.len() {
                    // glibc SET_NAN_PAYLOAD: low 51 mantissa bits, quiet bit kept
                    bits |= mant & ((1u64 << 51) - 1);
                }
                end = j + 1;
            }
        }
        return (sign(f64::from_bits(bits)), end, false);
    }
    let c = at(s, i);
    if !(c.is_ascii_digit() || (c == b'.' && at(s, i + 1).is_ascii_digit())) {
        return (0.0, 0, false);
    }
    if c == b'0' && at(s, i + 1) | 0x20 == b'x' {
        return scan_hex(s, i + 2, neg);
    }
    scan_dec(s, i, neg)
}

fn scan_dec(s: &[u8], start: usize, neg: bool) -> (f64, usize, bool) {
    let mut j = start;
    let mut digits: Vec<u8> = Vec::new();
    let mut frac_len: i64 = 0;
    while at(s, j).is_ascii_digit() {
        digits.push(at(s, j));
        j += 1;
    }
    if at(s, j) == b'.' && (!digits.is_empty() || at(s, j + 1).is_ascii_digit()) {
        j += 1;
        while at(s, j).is_ascii_digit() {
            digits.push(at(s, j));
            frac_len += 1;
            j += 1;
        }
    }
    let mut exp10: i64 = 0;
    if at(s, j) | 0x20 == b'e' {
        if let Some((k, e)) = scan_exponent(s, j) {
            exp10 = e;
            j = k;
        }
    }
    // value = 0.D * 10^dexp with D the significant digits (no leading or
    // trailing zeros); empty D means exact zero.
    let lead = digits.iter().take_while(|&&d| d == b'0').count();
    let int_len = digits.len() as i64 - frac_len;
    let mut sig = digits[lead..].to_vec();
    while sig.last() == Some(&b'0') {
        sig.pop();
    }
    let sign = |v: f64| if neg { -v } else { v };
    if sig.is_empty() {
        return (sign(0.0), j, false);
    }
    let dexp = int_len - lead as i64 + exp10;
    let mut text = String::with_capacity(sig.len() + 24);
    text.push_str("0.");
    text.push_str(std::str::from_utf8(&sig).expect("ascii digits"));
    text.push('e');
    text.push_str(&dexp.to_string());
    let v: f64 = text.parse().expect("well-formed decimal literal");
    let erange = if v.is_infinite() {
        true
    } else if v == 0.0 {
        true
    } else if v.is_subnormal() {
        // tiny and inexact (glibc: TININESS_AFTER_ROUNDING on x86)
        exact_digits(v) != (sig.clone(), dexp)
    } else if v == f64::MIN_POSITIVE {
        // rounded up to DBL_MIN from below: tiny iff below the 53-bit
        // rounding threshold DBL_MIN - 2^-1076
        dec_less(&sig, dexp, &dbl_min_threshold())
    } else {
        false
    };
    (sign(v), j, erange)
}

/// Exact decimal expansion of a finite positive f64 as (0.D digits, exp).
fn exact_digits(v: f64) -> (Vec<u8>, i64) {
    let t = format!("{:.1100e}", v);
    let (m, e) = t.split_once('e').expect("exponent form");
    let mut d: Vec<u8> = m.bytes().filter(|b| b.is_ascii_digit()).collect();
    while d.last() == Some(&b'0') {
        d.pop();
    }
    (d, e.parse::<i64>().expect("exponent") + 1)
}

/// Exact decimal (0.D, exp) of DBL_MIN - 2^-1076 = (2^54 - 1) * 2^-1076,
/// i.e. (2^54 - 1) * 5^1076 * 10^-1076.
fn dbl_min_threshold() -> (Vec<u8>, i64) {
    // little-endian base-1e9 bignum
    let mut n: Vec<u64> = vec![(1u64 << 54) - 1];
    let mul = |n: &mut Vec<u64>, m: u64| {
        let mut carry = 0u64;
        for limb in n.iter_mut() {
            let x = *limb * m + carry;
            *limb = x % 1_000_000_000;
            carry = x / 1_000_000_000;
        }
        while carry > 0 {
            n.push(carry % 1_000_000_000);
            carry /= 1_000_000_000;
        }
    };
    // normalize the initial limb, then multiply by 5 1076 times
    mul(&mut n, 1);
    for _ in 0..1076 {
        mul(&mut n, 5);
    }
    let mut text = n.last().expect("nonzero").to_string();
    for limb in n.iter().rev().skip(1) {
        text.push_str(&format!("{limb:09}"));
    }
    let mut d = text.into_bytes();
    let exp = d.len() as i64 - 1076;
    while d.last() == Some(&b'0') {
        d.pop();
    }
    (d, exp)
}

/// Compare positive decimals (0.D * 10^e, normalized): a < b.
fn dec_less(a: &[u8], ae: i64, b: &(Vec<u8>, i64)) -> bool {
    if ae != b.1 {
        return ae < b.1;
    }
    a < b.0.as_slice()
}

fn scan_hex(s: &[u8], start: usize, neg: bool) -> (f64, usize, bool) {
    let sign = |v: f64| if neg { -v } else { v };
    let mut j = start;
    let mut digits: Vec<u8> = Vec::new();
    let mut frac_len: i64 = 0;
    while at(s, j).is_ascii_hexdigit() {
        digits.push(digit_value(at(s, j)).expect("hex digit") as u8);
        j += 1;
    }
    if at(s, j) == b'.' && (!digits.is_empty() || at(s, j + 1).is_ascii_hexdigit()) {
        j += 1;
        while at(s, j).is_ascii_hexdigit() {
            digits.push(digit_value(at(s, j)).expect("hex digit") as u8);
            frac_len += 1;
            j += 1;
        }
    }
    if digits.is_empty() {
        // "0x" without digits: the "0" converts, end points at the 'x'
        return (sign(0.0), start - 1, false);
    }
    let mut pexp: i64 = 0;
    if at(s, j) | 0x20 == b'p' {
        if let Some((k, e)) = scan_exponent(s, j) {
            pexp = e;
            j = k;
        }
    }
    let lead = digits.iter().take_while(|&&d| d == 0).count();
    let sig = &digits[lead..];
    if sig.is_empty() {
        return (sign(0.0), j, false);
    }
    let taken = sig.len().min(16);
    let mut m: u64 = 0;
    for &d in &sig[..taken] {
        m = (m << 4) | d as u64;
    }
    let sticky = sig[taken..].iter().any(|&d| d != 0);
    let e2 = 4 * (sig.len() - taken) as i64 - 4 * frac_len + pexp;
    let (v, erange) = round_binary(m, sticky, e2);
    (sign(v), j, erange)
}

/// Round `m * 2^e2` (+ sticky bits below m) to the nearest f64, ties to
/// even. Returns (value, ERANGE) with glibc tininess-after-rounding.
fn round_binary(m: u64, sticky: bool, e2: i64) -> (f64, bool) {
    let lz = m.leading_zeros() as i64;
    let m = m << lz;
    let e = e2 + 63 - lz; // value in [2^e, 2^(e+1))
    let round = |q: u64, rem: u64, half: u64| -> u64 {
        if rem > half || (rem == half && (sticky || q & 1 == 1)) {
            q + 1
        } else {
            q
        }
    };
    if e > 1023 {
        return (f64::INFINITY, true);
    }
    if e >= -1022 {
        let mut q = round(m >> 11, m & 0x7ff, 0x400);
        let mut e = e;
        if q == 1 << 53 {
            q >>= 1;
            e += 1;
            if e > 1023 {
                return (f64::INFINITY, true);
            }
        }
        let bits = (((e + 1023) as u64) << 52) | (q & ((1 << 52) - 1));
        return (f64::from_bits(bits), false);
    }
    let inexact;
    let s = -e - 1011; // right shift to units of 2^-1074 (>= 12)
    let q = if s >= 65 {
        inexact = true;
        0
    } else {
        let (q, rem) = if s == 64 { (0, m) } else { (m >> s, m & ((1u64 << s) - 1)) };
        inexact = rem != 0 || sticky;
        round(q, rem, 1u64 << (s - 1))
    };
    // tiny unless rounding to 53 bits with unbounded exponent reaches DBL_MIN
    let tiny = !(e == -1023 && round(m >> 11, m & 0x7ff, 0x400) == 1 << 53);
    (f64::from_bits(q), tiny && inexact)
}

/// `printf("%.6f", v)` (glibc: exact value, round-half-even; "nan"/"-nan").
pub fn fmt_f6(v: f64) -> String {
    if v.is_nan() {
        if v.is_sign_negative() { "-nan".into() } else { "nan".into() }
    } else if v.is_infinite() {
        if v < 0.0 { "-inf".into() } else { "inf".into() }
    } else {
        format!("{v:.6}")
    }
}

#[cfg(test)]
#[allow(unsafe_code)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn errno_reset() {
        // SAFETY: thread-local errno location.
        unsafe { *libc::__errno_location() = 0 };
    }
    fn errno_erange() -> bool {
        // SAFETY: thread-local errno location.
        unsafe { *libc::__errno_location() == libc::ERANGE }
    }

    fn c_strtol(s: &[u8]) -> (i64, usize, bool) {
        let cs = CString::new(s).unwrap();
        let mut e = std::ptr::null_mut();
        errno_reset();
        // SAFETY: valid C string and out-pointer.
        let v = unsafe { libc::strtol(cs.as_ptr(), &mut e, 0) };
        (v, e as usize - cs.as_ptr() as usize, errno_erange())
    }
    fn c_strtoul(s: &[u8]) -> (u64, usize, bool) {
        let cs = CString::new(s).unwrap();
        let mut e = std::ptr::null_mut();
        errno_reset();
        // SAFETY: valid C string and out-pointer.
        let v = unsafe { libc::strtoul(cs.as_ptr(), &mut e, 0) };
        (v, e as usize - cs.as_ptr() as usize, errno_erange())
    }
    fn c_strtod(s: &[u8]) -> (f64, usize, bool) {
        let cs = CString::new(s).unwrap();
        let mut e = std::ptr::null_mut();
        errno_reset();
        // SAFETY: valid C string and out-pointer.
        let v = unsafe { libc::strtod(cs.as_ptr(), &mut e) };
        (v, e as usize - cs.as_ptr() as usize, errno_erange())
    }
    fn c_fmt6(v: f64) -> String {
        let mut buf = [0u8; 512];
        // SAFETY: buffer valid for its length; format matches the argument.
        let n = unsafe {
            libc::snprintf(buf.as_mut_ptr() as *mut _, buf.len(), c"%.6f".as_ptr(), v)
        };
        String::from_utf8(buf[..n as usize].to_vec()).unwrap()
    }

    fn check(s: &[u8]) {
        assert_eq!(strtol(s), c_strtol(s), "strtol {:?}", String::from_utf8_lossy(s));
        assert_eq!(strtoul(s), c_strtoul(s), "strtoul {:?}", String::from_utf8_lossy(s));
        let (a, ae, ar) = strtod(s);
        let (b, be, br) = c_strtod(s);
        assert!(
            a.to_bits() == b.to_bits() && ae == be && ar == br,
            "strtod {:?}: ours ({a:e} {:#x}, {ae}, {ar}) libc ({b:e} {:#x}, {be}, {br})",
            String::from_utf8_lossy(s),
            a.to_bits(),
            b.to_bits()
        );
    }

    #[test]
    fn fixed_cases_match_glibc() {
        let cases: &[&str] = &[
            "", "0", "-0", "+5", " \t\n12", "12abc", "0x", "0x1g", "0X1F", "010", "08", "-",
            "+", "x", "  ", "9223372036854775807", "9223372036854775808",
            "-9223372036854775808", "-9223372036854775809", "18446744073709551615",
            "18446744073709551616", "-1", "-18446744073709551615", "0xffffffffffffffff",
            "0x10000000000000000", "077777777777777777777777", "1.5", ".5", "5.", ".", "1e5",
            "1e", "1e+", "1e-3x", "1.e3", "0x1p3", "0x1p", "0x.8p1", "0x.p1", "0x1.8",
            "0xp1", "inf", "INFINITY", "infinit", "-inf", "nan", "NaN(123)", "nan(0x7)",
            "nan(", "nan()", "nan(abc", "-nan(1)", "1e309", "-1e309", "1e-400", "4.9e-324",
            "2.4703282292062327e-324", "2.4703282292062328e-324", "2.2250738585072011e-308",
            "2.2250738585072014e-308", "2.2250738585072012e-308", "2.225073858507201e-308",
            "0x1p-1074", "0x1p-1075", "0x1.8p-1075", "0x1p-1076", "0x1.fffffffffffffp-1023",
            "0x1.fffffffffffff8p-1023", "0x1.fffffffffffffcp-1023", "0x1.fffffffffffffffp1023",
            "0x1p1024", "1e99999999999999999999", "1e-99999999999999999999",
            "0.000000000000000000000000000000001", "123456789012345678901234567890",
            "0x123456789abcdef0123", "00000.00001e5", "1_000", "0x0.0000001p30",
        ];
        for c in cases {
            check(c.as_bytes());
        }
    }

    #[test]
    fn fuzz_matches_glibc() {
        const ALPHA: &[u8] = b"0123456789abcdefABCDEFxXpPeE.+- \tinfinityan()_";
        let mut seed: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for _ in 0..200_000 {
            let len = (next() % 12) as usize;
            let s: Vec<u8> = (0..len).map(|_| ALPHA[(next() % ALPHA.len() as u64) as usize]).collect();
            check(&s);
        }
    }

    #[test]
    fn fmt_f6_matches_glibc() {
        let mut vals = vec![
            0.0, -0.0, 1.0, -1.5, 0.0078125, 0.0234375, 2.5e-7, 5e-7, 1.0000005, 123456.7890125,
            1e300, -1e308, f64::MIN_POSITIVE, 5e-324, 0.1, 0.3, 1.9999995, 0.5e-6,
        ];
        let mut seed: u64 = 12345;
        for _ in 0..20_000 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let v = f64::from_bits(seed);
            if v.is_finite() && v.abs() < 1e30 {
                vals.push(v);
            }
            vals.push((seed >> 40) as f64 / 128.0); // many exact ties
        }
        for v in vals {
            assert_eq!(fmt_f6(v), c_fmt6(v), "value {v:e}");
        }
        assert_eq!(fmt_f6(f64::INFINITY), c_fmt6(f64::INFINITY));
        assert_eq!(fmt_f6(f64::NEG_INFINITY), c_fmt6(f64::NEG_INFINITY));
        assert_eq!(fmt_f6(f64::NAN), c_fmt6(f64::NAN));
        assert_eq!(fmt_f6(-f64::NAN), c_fmt6(-f64::NAN));
    }
}
