//! MD5 (RFC 1321 reference implementation), migrated to safe Rust (P1).
//! The compression-round body is verbatim from the c2rust original (already
//! value arithmetic); only pointer plumbing became slices. ABI preserved:
//! md5ctx stays #[repr(C)], init/update/final keep their C symbols.

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _md5ctx {
    pub state: [uint32_t; 4],
    pub count: [uint32_t; 2],
    pub buffer: [uint8_t; 64],
}
pub type md5ctx = _md5ctx;
const PADDING: [uint8_t; 64] = [
    0x80 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
];
fn md5_encode(output: &mut [uint8_t], input: &[uint32_t]) {
    for (i, &v) in input.iter().enumerate() {
        output[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes());
    }
}
fn md5_decode(output: &mut [uint32_t], input: &[uint8_t]) {
    for (i, e) in output.iter_mut().enumerate() {
        *e = u32::from_le_bytes([input[4 * i], input[4 * i + 1], input[4 * i + 2], input[4 * i + 3]]);
    }
}
fn md5_transform(state: &mut [uint32_t; 4], block: &[uint8_t]) {
        let mut a: uint32_t = state[0];
        let mut b: uint32_t = state[1];
        let mut c: uint32_t = state[2];
        let mut d: uint32_t = state[3];
        let mut x: [uint32_t; 16] = [0; 16];
        md5_decode(&mut x, block);
        a = a.wrapping_add(
            (b & c | !b & d)
                .wrapping_add(x[0 as usize])
                .wrapping_add(0xd76aa478 as ::core::ffi::c_uint as uint32_t),
        );
        a = a << 7 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (a & b | !a & c)
                .wrapping_add(x[1 as usize])
                .wrapping_add(0xe8c7b756 as ::core::ffi::c_uint as uint32_t),
        );
        d = d << 12 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (d & a | !d & b)
                .wrapping_add(x[2 as usize])
                .wrapping_add(0x242070db as ::core::ffi::c_int as uint32_t),
        );
        c = c << 17 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (c & d | !c & a)
                .wrapping_add(x[3 as usize])
                .wrapping_add(0xc1bdceee as ::core::ffi::c_uint as uint32_t),
        );
        b = b << 22 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (b & c | !b & d)
                .wrapping_add(x[4 as usize])
                .wrapping_add(0xf57c0faf as ::core::ffi::c_uint as uint32_t),
        );
        a = a << 7 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (a & b | !a & c)
                .wrapping_add(x[5 as usize])
                .wrapping_add(0x4787c62a as ::core::ffi::c_int as uint32_t),
        );
        d = d << 12 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (d & a | !d & b)
                .wrapping_add(x[6 as usize])
                .wrapping_add(0xa8304613 as ::core::ffi::c_uint as uint32_t),
        );
        c = c << 17 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (c & d | !c & a)
                .wrapping_add(x[7 as usize])
                .wrapping_add(0xfd469501 as ::core::ffi::c_uint as uint32_t),
        );
        b = b << 22 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (b & c | !b & d)
                .wrapping_add(x[8 as usize])
                .wrapping_add(0x698098d8 as ::core::ffi::c_int as uint32_t),
        );
        a = a << 7 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (a & b | !a & c)
                .wrapping_add(x[9 as usize])
                .wrapping_add(0x8b44f7af as ::core::ffi::c_uint as uint32_t),
        );
        d = d << 12 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (d & a | !d & b)
                .wrapping_add(x[10 as usize])
                .wrapping_add(0xffff5bb1 as ::core::ffi::c_uint as uint32_t),
        );
        c = c << 17 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (c & d | !c & a)
                .wrapping_add(x[11 as usize])
                .wrapping_add(0x895cd7be as ::core::ffi::c_uint as uint32_t),
        );
        b = b << 22 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (b & c | !b & d)
                .wrapping_add(x[12 as usize])
                .wrapping_add(0x6b901122 as ::core::ffi::c_int as uint32_t),
        );
        a = a << 7 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (a & b | !a & c)
                .wrapping_add(x[13 as usize])
                .wrapping_add(0xfd987193 as ::core::ffi::c_uint as uint32_t),
        );
        d = d << 12 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (d & a | !d & b)
                .wrapping_add(x[14 as usize])
                .wrapping_add(0xa679438e as ::core::ffi::c_uint as uint32_t),
        );
        c = c << 17 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (c & d | !c & a)
                .wrapping_add(x[15 as usize])
                .wrapping_add(0x49b40821 as ::core::ffi::c_int as uint32_t),
        );
        b = b << 22 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (b & d | c & !d)
                .wrapping_add(x[1 as usize])
                .wrapping_add(0xf61e2562 as ::core::ffi::c_uint as uint32_t),
        );
        a = a << 5 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (a & c | b & !c)
                .wrapping_add(x[6 as usize])
                .wrapping_add(0xc040b340 as ::core::ffi::c_uint as uint32_t),
        );
        d = d << 9 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (d & b | a & !b)
                .wrapping_add(x[11 as usize])
                .wrapping_add(0x265e5a51 as ::core::ffi::c_int as uint32_t),
        );
        c = c << 14 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (c & a | d & !a)
                .wrapping_add(x[0 as usize])
                .wrapping_add(0xe9b6c7aa as ::core::ffi::c_uint as uint32_t),
        );
        b = b << 20 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (b & d | c & !d)
                .wrapping_add(x[5 as usize])
                .wrapping_add(0xd62f105d as ::core::ffi::c_uint as uint32_t),
        );
        a = a << 5 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (a & c | b & !c)
                .wrapping_add(x[10 as usize])
                .wrapping_add(0x2441453 as ::core::ffi::c_int as uint32_t),
        );
        d = d << 9 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (d & b | a & !b)
                .wrapping_add(x[15 as usize])
                .wrapping_add(0xd8a1e681 as ::core::ffi::c_uint as uint32_t),
        );
        c = c << 14 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (c & a | d & !a)
                .wrapping_add(x[4 as usize])
                .wrapping_add(0xe7d3fbc8 as ::core::ffi::c_uint as uint32_t),
        );
        b = b << 20 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (b & d | c & !d)
                .wrapping_add(x[9 as usize])
                .wrapping_add(0x21e1cde6 as ::core::ffi::c_int as uint32_t),
        );
        a = a << 5 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (a & c | b & !c)
                .wrapping_add(x[14 as usize])
                .wrapping_add(0xc33707d6 as ::core::ffi::c_uint as uint32_t),
        );
        d = d << 9 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (d & b | a & !b)
                .wrapping_add(x[3 as usize])
                .wrapping_add(0xf4d50d87 as ::core::ffi::c_uint as uint32_t),
        );
        c = c << 14 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (c & a | d & !a)
                .wrapping_add(x[8 as usize])
                .wrapping_add(0x455a14ed as ::core::ffi::c_int as uint32_t),
        );
        b = b << 20 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (b & d | c & !d)
                .wrapping_add(x[13 as usize])
                .wrapping_add(0xa9e3e905 as ::core::ffi::c_uint as uint32_t),
        );
        a = a << 5 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (a & c | b & !c)
                .wrapping_add(x[2 as usize])
                .wrapping_add(0xfcefa3f8 as ::core::ffi::c_uint as uint32_t),
        );
        d = d << 9 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (d & b | a & !b)
                .wrapping_add(x[7 as usize])
                .wrapping_add(0x676f02d9 as ::core::ffi::c_int as uint32_t),
        );
        c = c << 14 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (c & a | d & !a)
                .wrapping_add(x[12 as usize])
                .wrapping_add(0x8d2a4c8a as ::core::ffi::c_uint as uint32_t),
        );
        b = b << 20 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(x[5 as usize])
                .wrapping_add(0xfffa3942 as ::core::ffi::c_uint as uint32_t),
        );
        a = a << 4 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(x[8 as usize])
                .wrapping_add(0x8771f681 as ::core::ffi::c_uint as uint32_t),
        );
        d = d << 11 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(x[11 as usize])
                .wrapping_add(0x6d9d6122 as ::core::ffi::c_int as uint32_t),
        );
        c = c << 16 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(x[14 as usize])
                .wrapping_add(0xfde5380c as ::core::ffi::c_uint as uint32_t),
        );
        b = b << 23 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(x[1 as usize])
                .wrapping_add(0xa4beea44 as ::core::ffi::c_uint as uint32_t),
        );
        a = a << 4 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(x[4 as usize])
                .wrapping_add(0x4bdecfa9 as ::core::ffi::c_int as uint32_t),
        );
        d = d << 11 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(x[7 as usize])
                .wrapping_add(0xf6bb4b60 as ::core::ffi::c_uint as uint32_t),
        );
        c = c << 16 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(x[10 as usize])
                .wrapping_add(0xbebfbc70 as ::core::ffi::c_uint as uint32_t),
        );
        b = b << 23 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(x[13 as usize])
                .wrapping_add(0x289b7ec6 as ::core::ffi::c_int as uint32_t),
        );
        a = a << 4 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(x[0 as usize])
                .wrapping_add(0xeaa127fa as ::core::ffi::c_uint as uint32_t),
        );
        d = d << 11 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(x[3 as usize])
                .wrapping_add(0xd4ef3085 as ::core::ffi::c_uint as uint32_t),
        );
        c = c << 16 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(x[6 as usize])
                .wrapping_add(0x4881d05 as ::core::ffi::c_int as uint32_t),
        );
        b = b << 23 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(x[9 as usize])
                .wrapping_add(0xd9d4d039 as ::core::ffi::c_uint as uint32_t),
        );
        a = a << 4 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(x[12 as usize])
                .wrapping_add(0xe6db99e5 as ::core::ffi::c_uint as uint32_t),
        );
        d = d << 11 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(x[15 as usize])
                .wrapping_add(0x1fa27cf8 as ::core::ffi::c_int as uint32_t),
        );
        c = c << 16 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(x[2 as usize])
                .wrapping_add(0xc4ac5665 as ::core::ffi::c_uint as uint32_t),
        );
        b = b << 23 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(x[0 as usize])
                .wrapping_add(0xf4292244 as ::core::ffi::c_uint as uint32_t),
        );
        a = a << 6 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(x[7 as usize])
                .wrapping_add(0x432aff97 as ::core::ffi::c_int as uint32_t),
        );
        d = d << 10 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(x[14 as usize])
                .wrapping_add(0xab9423a7 as ::core::ffi::c_uint as uint32_t),
        );
        c = c << 15 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(x[5 as usize])
                .wrapping_add(0xfc93a039 as ::core::ffi::c_uint as uint32_t),
        );
        b = b << 21 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(x[12 as usize])
                .wrapping_add(0x655b59c3 as ::core::ffi::c_int as uint32_t),
        );
        a = a << 6 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(x[3 as usize])
                .wrapping_add(0x8f0ccc92 as ::core::ffi::c_uint as uint32_t),
        );
        d = d << 10 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(x[10 as usize])
                .wrapping_add(0xffeff47d as ::core::ffi::c_uint as uint32_t),
        );
        c = c << 15 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(x[1 as usize])
                .wrapping_add(0x85845dd1 as ::core::ffi::c_uint as uint32_t),
        );
        b = b << 21 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(x[8 as usize])
                .wrapping_add(0x6fa87e4f as ::core::ffi::c_int as uint32_t),
        );
        a = a << 6 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(x[15 as usize])
                .wrapping_add(0xfe2ce6e0 as ::core::ffi::c_uint as uint32_t),
        );
        d = d << 10 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(x[6 as usize])
                .wrapping_add(0xa3014314 as ::core::ffi::c_uint as uint32_t),
        );
        c = c << 15 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(x[13 as usize])
                .wrapping_add(0x4e0811a1 as ::core::ffi::c_int as uint32_t),
        );
        b = b << 21 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a.wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(x[4 as usize])
                .wrapping_add(0xf7537e82 as ::core::ffi::c_uint as uint32_t),
        );
        a = a << 6 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d.wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(x[11 as usize])
                .wrapping_add(0xbd3af235 as ::core::ffi::c_uint as uint32_t),
        );
        d = d << 10 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c.wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(x[2 as usize])
                .wrapping_add(0x2ad7d2bb as ::core::ffi::c_int as uint32_t),
        );
        c = c << 15 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b.wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(x[9 as usize])
                .wrapping_add(0xeb86d391 as ::core::ffi::c_uint as uint32_t),
        );
        b = b << 21 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        x.fill(0); // hygiene: clear the block copy, as the original memset did
}
fn md5_init_imp(ctx: &mut md5ctx) {
    ctx.count = [0, 0];
    ctx.state = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
}

/// # Safety
/// `ctx` must point to a valid, writable md5ctx (C caller contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_init(mut ctx: *mut md5ctx) {
    // SAFETY: per fn contract.
    unsafe { md5_init_imp(&mut *ctx) }
}

fn md5_update_imp(ctx: &mut md5ctx, buff: &[uint8_t]) {
    let leng = buff.len() as uint32_t;
    let mut indx = (ctx.count[0] >> 3) & 0x3f;
    ctx.count[0] = ctx.count[0].wrapping_add(leng << 3);
    if ctx.count[0] < leng << 3 {
        ctx.count[1] = ctx.count[1].wrapping_add(1);
    }
    ctx.count[1] = ctx.count[1].wrapping_add(leng >> 29);
    let partleng = 64u32.wrapping_sub(indx);
    let mut i = 0usize;
    if leng >= partleng {
        ctx.buffer[indx as usize..64].copy_from_slice(&buff[..partleng as usize]);
        let block = ctx.buffer;
        md5_transform(&mut ctx.state, &block);
        i = partleng as usize;
        while i + 63 < leng as usize {
            md5_transform(&mut ctx.state, &buff[i..i + 64]);
            i += 64;
        }
        indx = 0;
    }
    ctx.buffer[indx as usize..indx as usize + (leng as usize - i)]
        .copy_from_slice(&buff[i..]);
}

/// # Safety
/// `ctx` must point to a valid, writable md5ctx; `buff` readable for `leng`
/// bytes (C caller contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_update(
    mut ctx: *mut md5ctx,
    mut buff: *const uint8_t,
    mut leng: uint32_t,
) {
    // SAFETY: per fn contract.
    unsafe { md5_update_imp(&mut *ctx, std::slice::from_raw_parts(buff, leng as usize)) }
}

fn md5_final_imp(digest: &mut [uint8_t; 16], ctx: &mut md5ctx) {
    let mut bits = [0u8; 8];
    md5_encode(&mut bits, &ctx.count);
    let indx = (ctx.count[0] >> 3) & 0x3f;
    let padleng = if indx < 56 { 56 - indx } else { 120 - indx };
    md5_update_imp(ctx, &PADDING[..padleng as usize]);
    md5_update_imp(ctx, &bits);
    md5_encode(&mut digest[..], &ctx.state);
    *ctx = md5ctx {
        state: [0; 4],
        count: [0; 2],
        buffer: [0; 64],
    };
}

/// # Safety
/// `digest` must be writable for 16 bytes; `ctx` a valid md5ctx (C caller
/// contract). ctx is zeroed before return, as the original did.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn md5_final(mut digest: *mut uint8_t, mut ctx: *mut md5ctx) {
    // SAFETY: per fn contract.
    unsafe { md5_final_imp(&mut *(digest as *mut [uint8_t; 16]), &mut *ctx) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn md5_of(data: &[u8]) -> [u8; 16] {
        let mut ctx = md5ctx { state: [0; 4], count: [0; 2], buffer: [0; 64] };
        md5_init_imp(&mut ctx);
        md5_update_imp(&mut ctx, data);
        let mut d = [0u8; 16];
        md5_final_imp(&mut d, &mut ctx);
        d
    }

    #[test]
    fn rfc1321_vectors() {
        assert_eq!(md5_of(b""), [0xd4,0x1d,0x8c,0xd9,0x8f,0x00,0xb2,0x04,0xe9,0x80,0x09,0x98,0xec,0xf8,0x42,0x7e]);
        assert_eq!(md5_of(b"abc"), [0x90,0x01,0x50,0x98,0x3c,0xd2,0x4f,0xb0,0xd6,0x96,0x3f,0x7d,0x28,0xe1,0x7f,0x72]);
        // split updates must equal one-shot
        let mut ctx = md5ctx { state: [0; 4], count: [0; 2], buffer: [0; 64] };
        md5_init_imp(&mut ctx);
        md5_update_imp(&mut ctx, b"message ");
        md5_update_imp(&mut ctx, b"digest");
        let mut d = [0u8; 16];
        md5_final_imp(&mut d, &mut ctx);
        assert_eq!(d, md5_of(b"message digest"));
    }
}
