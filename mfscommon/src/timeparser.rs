//! Time period parse/print ("1w2d3h" etc.), migrated to safe Rust (P1).
//! Safe cores operate on Rust strings/byte slices; the exported C ABI
//! (caller buffer + NUL semantics of snprintf, C-string input, out-param)
//! is preserved in thin shims.

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const TPARSE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TPARSE_UNEXPECTED_CHAR: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const TPARSE_VALUE_TOO_BIG: ::core::ffi::c_int = -2 as ::core::ffi::c_int;

#[deny(unsafe_code)]
mod imp {
    /// snprintf semantics: appends `piece` to out, pretending the visible
    /// window is `*mleng` bytes; returns the would-be length (may exceed the
    /// window), NUL-terminates. Mirrors the C original's buffer discipline.
    pub fn snprint_piece(out: &mut Vec<u8>, piece: &str, mleng: &mut usize) -> u32 {
        let avail = *mleng;
        let would = piece.len() as u32;
        if avail > 0 {
            let n = piece.len().min(avail - 1);
            out.extend_from_slice(&piece.as_bytes()[..n]);
            out.push(0);
            *mleng -= n + 1;
        }
        would
    }

    pub fn snprint_time(period: u32, timetab: &[u32], timesym: &[u8]) -> (Vec<u8>, u32) {
        let mut out: Vec<u8> = Vec::new();
        let mut mleng = usize::MAX; // unbounded: we format then the shim clips
        let mut resultlen: u32 = 0;
        if period == 0 {
            let s = format!("0{}", timesym[timesym.len() - 1] as char);
            let n = snprint_piece(&mut out, &s, &mut mleng);
            return (out, n);
        }
        for (i, &tb) in timetab.iter().enumerate() {
            if period >= tb {
                let timebase = tb;
                let mut n = period as u64 * 10;
                let p10 = n;
                if timebase > 1 {
                    n = n.wrapping_add((timebase / 2) as u64);
                    n /= timebase as u64;
                }
                if n.wrapping_mul(timebase as u64) != p10 {
                    out.push(b'~');
                    resultlen = 1;
                }
                let intpart = (n / 10) as u32;
                let decpart = (n % 10) as u32;
                let s = if decpart != 0 {
                    format!("{}.{}{}", intpart, decpart, timesym[i] as char)
                } else {
                    format!("{}{}", intpart, timesym[i] as char)
                };
                resultlen = resultlen.wrapping_add(snprint_piece(&mut out, &s, &mut mleng));
                return (out, resultlen);
            }
        }
        out.push(0);
        (out, resultlen)
    }

    pub fn parse_period(bytes: &[u8], hmode: bool) -> Result<u32, (i32, u32)> {
        let mut str_ = bytes;
        let mut res: u64 = 0;
        let mut mask: u8 = 0;
        while matches!(str_.first(), Some(b' ') | Some(b'\t')) {
            str_ = &str_[1..];
        }
        loop {
            let mut cur: u64 = 0;
            let mut ndiv: u64 = 1;
            if str_.first().is_some_and(|c| c.is_ascii_digit()) {
                if mask & 0xc0 == 0xc0 {
                    return Err((TPARSE_UNEXPECTED_CHAR_I, str_[0] as u32));
                }
                while str_.first().is_some_and(|c| c.is_ascii_digit()) && cur < u32::MAX as u64 {
                    // original used a double multiply — kept bit-exact
                    cur = (cur as f64 * 10.0f64) as u64;
                    cur = cur.wrapping_add((str_[0] - b'0') as u64);
                    ndiv = ndiv.wrapping_mul(10);
                    str_ = &str_[1..];
                }
                if cur > u32::MAX as u64 {
                    return Err((TPARSE_VALUE_TOO_BIG_I, 0));
                }
                while matches!(str_.first(), Some(b' ') | Some(b'\t')) {
                    str_ = &str_[1..];
                }
            } else {
                return Err((TPARSE_UNEXPECTED_CHAR_I, *str_.first().unwrap_or(&0) as u32));
            }
            let ch = *str_.first().unwrap_or(&0);
            let cmask: u8;
            let mul: u32;
            let max: u64;
            match ch {
                b'.' => {
                    if mask != 0 {
                        return Err((TPARSE_UNEXPECTED_CHAR_I, ch as u32));
                    }
                    mask |= 0x80;
                    res = cur;
                    str_ = &str_[1..];
                    continue; // cmask==0x80 in the C original: shared block skipped
                }
                b'w' | b'W' => {
                    mul = if hmode { 7 * 24 } else { 7 * 24 * 3600 };
                    max = u32::MAX as u64;
                    cmask = 0x10;
                }
                b'd' | b'D' => {
                    if hmode {
                        mul = 24;
                        max = (7 * 24) as u64;
                    } else {
                        mul = 24 * 3600;
                        max = (7 * 24 * 3600) as u64;
                    }
                    cmask = 0x8;
                }
                b'h' | b'H' => {
                    if hmode {
                        mul = 1;
                        max = 24;
                    } else {
                        mul = 3600;
                        max = (24 * 3600) as u64;
                    }
                    cmask = 0x4;
                }
                b'm' | b'M' => {
                    if hmode {
                        return Err((TPARSE_UNEXPECTED_CHAR_I, ch as u32));
                    }
                    mul = 60;
                    max = 3600;
                    cmask = 0x2;
                }
                b's' | b'S' => {
                    if hmode {
                        return Err((TPARSE_UNEXPECTED_CHAR_I, ch as u32));
                    }
                    mul = 1;
                    max = 60;
                    cmask = 0x1;
                }
                0 => {
                    if mask == 0 {
                        return Ok(cur as u32);
                    }
                    return Err((TPARSE_UNEXPECTED_CHAR_I, 0));
                }
                _ => return Err((TPARSE_UNEXPECTED_CHAR_I, ch as u32)),
            }
            // shared suffix of the C original's loop body
            cur = cur.wrapping_mul(mul as u64);
            if mask == 0x80 {
                res = res.wrapping_mul(mul as u64);
                cur = cur.wrapping_mul(10).wrapping_div(ndiv);
                cur = cur / 10 + if cur % 10 >= 5 { 1 } else { 0 };
                res = res.wrapping_add(cur);
                mask |= 0x40;
                str_ = &str_[1..];
            } else if mask == 0 {
                mask |= cmask;
                res = cur;
                str_ = &str_[1..];
            } else if mask as u32 & (0xc0 | (((cmask as u32) << 1) - 1)) == 0 {
                if cur < max {
                    mask |= cmask;
                    res = res.wrapping_add(cur);
                    str_ = &str_[1..];
                } else {
                    return Err((TPARSE_VALUE_TOO_BIG_I, ch as u32));
                }
            } else {
                return Err((TPARSE_UNEXPECTED_CHAR_I, ch as u32));
            }
            if res > u32::MAX as u64 {
                return Err((TPARSE_VALUE_TOO_BIG_I, 0));
            }
            while matches!(str_.first(), Some(b' ') | Some(b'\t')) {
                str_ = &str_[1..];
            }
            let ch = *str_.first().unwrap_or(&0);
            if ch == 0 || ch == b'\r' || ch == b'\n' {
                return Ok(res as u32);
            }
        }
    }

    pub const TPARSE_UNEXPECTED_CHAR_I: i32 = -1;
    pub const TPARSE_VALUE_TOO_BIG_I: i32 = -2;
}

/// snprintf-clip helper: writes `body` (already formatted, may contain a
/// trailing NUL from imp) into the caller window and returns resultlen.
///
/// # Safety
/// `where_` must be writable for `mleng` bytes (C caller contract).
unsafe fn clip_out(
    where_: *mut ::core::ffi::c_char,
    mleng: size_t,
    body: &[u8],
    resultlen: u32,
) -> ::core::ffi::c_int {
    if mleng == 0 {
        return 0;
    }
    // strip trailing NULs; we re-terminate ourselves
    let mut body = body;
    while body.last() == Some(&0) {
        body = &body[..body.len() - 1];
    }
    // SAFETY: per fn contract; writes bounded by mleng.
    unsafe {
        if body.is_empty() {
            *where_ = 0;
            return resultlen as ::core::ffi::c_int;
        }
        let text_end = body.len().min(mleng - 1);
        std::ptr::copy_nonoverlapping(
            body.as_ptr() as *const ::core::ffi::c_char,
            where_,
            text_end,
        );
        *where_.add(text_end) = 0;
    }
    resultlen as ::core::ffi::c_int
}

/// # Safety
/// `where_` writable for `mleng`; `timetab`/`timesym` readable for
/// `timecount` elements (C caller contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snprint_time_common(
    mut r#where: *mut ::core::ffi::c_char,
    mut mleng: size_t,
    mut period: uint32_t,
    mut timetab: *mut uint32_t,
    mut timesym: *mut ::core::ffi::c_char,
    mut timecount: uint32_t,
) -> ::core::ffi::c_int {
    if mleng == 0 {
        return 0;
    }
    // SAFETY: per fn contract.
    let (tt, ts) = unsafe {
        (
            std::slice::from_raw_parts(timetab, timecount as usize),
            std::slice::from_raw_parts(timesym as *const u8, timecount as usize),
        )
    };
    let (body, resultlen) = imp::snprint_time(period, tt, ts);
    // SAFETY: per fn contract.
    unsafe { clip_out(r#where, mleng, &body, resultlen) }
}

/// # Safety
/// `str` must be a valid NUL-terminated C string; `ret` writable (C caller
/// contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_period_common(
    mut str: *const ::core::ffi::c_char,
    mut ret: *mut uint32_t,
    mut hmode: uint8_t,
) -> ::core::ffi::c_int {
    // SAFETY: per fn contract.
    let bytes = unsafe { std::ffi::CStr::from_ptr(str) }.to_bytes();
    match imp::parse_period(bytes, hmode != 0) {
        Ok(v) => {
            // SAFETY: per fn contract.
            unsafe { *ret = v };
            TPARSE_OK
        }
        Err((code, retv)) => {
            // SAFETY: per fn contract.
            unsafe { *ret = retv };
            code
        }
    }
}

/// # Safety
/// see snprint_time_common.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snprint_speriod(
    mut r#where: *mut ::core::ffi::c_char,
    mut mleng: size_t,
    mut period: uint32_t,
) -> ::core::ffi::c_int {
    const TIMETAB: [uint32_t; 5] = [604800, 86400, 3600, 60, 1];
    const TIMESYM: [u8; 5] = [b'w', b'd', b'h', b'm', b's'];
    let (body, resultlen) = imp::snprint_time(period, &TIMETAB, &TIMESYM);
    if mleng == 0 {
        return 0;
    }
    // SAFETY: see snprint_time_common.
    unsafe { clip_out(r#where, mleng, &body, resultlen) }
}

/// # Safety
/// see parse_period_common.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_speriod(
    mut str: *const ::core::ffi::c_char,
    mut ret: *mut uint32_t,
) -> ::core::ffi::c_int {
    // SAFETY: forwarded contract, see parse_period_common.
    unsafe { parse_period_common(str, ret, 0) }
}

/// # Safety
/// see snprint_time_common.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snprint_hperiod(
    mut r#where: *mut ::core::ffi::c_char,
    mut mleng: size_t,
    mut period: uint32_t,
) -> ::core::ffi::c_int {
    const TIMETAB: [uint32_t; 3] = [168, 24, 1];
    const TIMESYM: [u8; 3] = [b'w', b'd', b'h'];
    let (body, resultlen) = imp::snprint_time(period, &TIMETAB, &TIMESYM);
    if mleng == 0 {
        return 0;
    }
    // SAFETY: see snprint_time_common.
    unsafe { clip_out(r#where, mleng, &body, resultlen) }
}

/// # Safety
/// see parse_period_common.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_hperiod(
    mut str: *const ::core::ffi::c_char,
    mut ret: *mut uint32_t,
) -> ::core::ffi::c_int {
    // SAFETY: forwarded contract, see parse_period_common.
    unsafe { parse_period_common(str, ret, 1) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fmt(period: u32) -> String {
        let (body, _) = imp::snprint_time(period, &[604800, 86400, 3600, 60, 1], b"wdhms");
        String::from_utf8(body)
            .unwrap()
            .trim_end_matches('\0')
            .to_string()
    }

    #[test]
    fn format_vectors() {
        assert_eq!(fmt(0), "0s");
        assert_eq!(fmt(1), "1s");
        assert_eq!(fmt(61), "~1m"); // 61s → 1.02m rounds to 1m with '~'
        assert_eq!(fmt(3600), "1h");
        assert_eq!(fmt(90061), "~1d"); // 25.02h → 1d with '~'
    }

    #[test]
    fn parse_vectors() {
        assert_eq!(imp::parse_period(b"3600", false), Ok(3600));
        assert_eq!(imp::parse_period(b"1h", false), Ok(3600));
        assert_eq!(imp::parse_period(b"1h30m", false), Ok(5400));
        assert_eq!(imp::parse_period(b"1.5h", false), Ok(5400));
        assert_eq!(imp::parse_period(b"2d", false), Ok(172800));
        assert_eq!(imp::parse_period(b"1w", false), Ok(604800));
        assert_eq!(imp::parse_period(b"24h", true), Ok(24));
        assert!(imp::parse_period(b"1m", true).is_err());
        assert!(imp::parse_period(b"", false).is_err());
        // round-trip
        for p in [1u32, 59, 60, 3600, 86400, 604800] {
            assert_eq!(imp::parse_period(fmt(p).as_bytes(), false), Ok(p));
        }
    }
}
