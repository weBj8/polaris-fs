extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
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
static mut padding: [uint8_t; 64] = [
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
unsafe extern "C" fn md5_encode(
    mut output: *mut uint8_t,
    mut input: *const uint32_t,
    mut len: uint32_t,
) {
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    i = 0 as uint32_t;
    j = 0 as uint32_t;
    while i < len {
        *output.offset(j as isize) = (*input.offset(i as isize) & 0xff as uint32_t) as uint8_t;
        *output.offset(j.wrapping_add(1 as uint32_t) as isize) =
            (*input.offset(i as isize) >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        *output.offset(j.wrapping_add(2 as uint32_t) as isize) =
            (*input.offset(i as isize) >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        *output.offset(j.wrapping_add(3 as uint32_t) as isize) =
            (*input.offset(i as isize) >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        i = i.wrapping_add(1);
        j = j.wrapping_add(4 as uint32_t);
    }
}
unsafe extern "C" fn md5_decode(
    mut output: *mut uint32_t,
    mut input: *const uint8_t,
    mut len: uint32_t,
) {
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    i = 0 as uint32_t;
    j = 0 as uint32_t;
    while i < len {
        *output.offset(i as isize) = *input.offset(j as isize) as uint32_t
            | (*input.offset(j.wrapping_add(1 as uint32_t) as isize) as uint32_t)
                << 8 as ::core::ffi::c_int
            | (*input.offset(j.wrapping_add(2 as uint32_t) as isize) as uint32_t)
                << 16 as ::core::ffi::c_int
            | (*input.offset(j.wrapping_add(3 as uint32_t) as isize) as uint32_t)
                << 24 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
        j = j.wrapping_add(4 as uint32_t);
    }
}
unsafe extern "C" fn md5_transform(mut state: *mut uint32_t, mut block: *const uint8_t) {
    let mut a: uint32_t = *state.offset(0 as isize);
    let mut b: uint32_t = *state.offset(1 as isize);
    let mut c: uint32_t = *state.offset(2 as isize);
    let mut d: uint32_t = *state.offset(3 as isize);
    let mut x: [uint32_t; 16] = [0; 16];
    md5_decode(
        &raw mut x as *mut uint32_t,
        block as *const uint8_t,
        16 as uint32_t,
    );
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
    d = d << 12 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (d & a | !d & b)
            .wrapping_add(x[2 as usize])
            .wrapping_add(0x242070db as ::core::ffi::c_int as uint32_t),
    );
    c = c << 17 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c & d | !c & a)
            .wrapping_add(x[3 as usize])
            .wrapping_add(0xc1bdceee as ::core::ffi::c_uint as uint32_t),
    );
    b = b << 22 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
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
    d = d << 12 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (d & a | !d & b)
            .wrapping_add(x[6 as usize])
            .wrapping_add(0xa8304613 as ::core::ffi::c_uint as uint32_t),
    );
    c = c << 17 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c & d | !c & a)
            .wrapping_add(x[7 as usize])
            .wrapping_add(0xfd469501 as ::core::ffi::c_uint as uint32_t),
    );
    b = b << 22 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
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
    d = d << 12 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (d & a | !d & b)
            .wrapping_add(x[10 as usize])
            .wrapping_add(0xffff5bb1 as ::core::ffi::c_uint as uint32_t),
    );
    c = c << 17 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c & d | !c & a)
            .wrapping_add(x[11 as usize])
            .wrapping_add(0x895cd7be as ::core::ffi::c_uint as uint32_t),
    );
    b = b << 22 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
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
    d = d << 12 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (d & a | !d & b)
            .wrapping_add(x[14 as usize])
            .wrapping_add(0xa679438e as ::core::ffi::c_uint as uint32_t),
    );
    c = c << 17 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c & d | !c & a)
            .wrapping_add(x[15 as usize])
            .wrapping_add(0x49b40821 as ::core::ffi::c_int as uint32_t),
    );
    b = b << 22 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
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
    c = c << 14 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c & a | d & !a)
            .wrapping_add(x[0 as usize])
            .wrapping_add(0xe9b6c7aa as ::core::ffi::c_uint as uint32_t),
    );
    b = b << 20 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
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
    c = c << 14 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c & a | d & !a)
            .wrapping_add(x[4 as usize])
            .wrapping_add(0xe7d3fbc8 as ::core::ffi::c_uint as uint32_t),
    );
    b = b << 20 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
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
    c = c << 14 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c & a | d & !a)
            .wrapping_add(x[8 as usize])
            .wrapping_add(0x455a14ed as ::core::ffi::c_int as uint32_t),
    );
    b = b << 20 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
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
    c = c << 14 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c & a | d & !a)
            .wrapping_add(x[12 as usize])
            .wrapping_add(0x8d2a4c8a as ::core::ffi::c_uint as uint32_t),
    );
    b = b << 20 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
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
    d = d << 11 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(x[11 as usize])
            .wrapping_add(0x6d9d6122 as ::core::ffi::c_int as uint32_t),
    );
    c = c << 16 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(x[14 as usize])
            .wrapping_add(0xfde5380c as ::core::ffi::c_uint as uint32_t),
    );
    b = b << 23 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
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
    d = d << 11 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(x[7 as usize])
            .wrapping_add(0xf6bb4b60 as ::core::ffi::c_uint as uint32_t),
    );
    c = c << 16 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(x[10 as usize])
            .wrapping_add(0xbebfbc70 as ::core::ffi::c_uint as uint32_t),
    );
    b = b << 23 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
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
    d = d << 11 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(x[3 as usize])
            .wrapping_add(0xd4ef3085 as ::core::ffi::c_uint as uint32_t),
    );
    c = c << 16 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(x[6 as usize])
            .wrapping_add(0x4881d05 as ::core::ffi::c_int as uint32_t),
    );
    b = b << 23 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
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
    d = d << 11 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(x[15 as usize])
            .wrapping_add(0x1fa27cf8 as ::core::ffi::c_int as uint32_t),
    );
    c = c << 16 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(x[2 as usize])
            .wrapping_add(0xc4ac5665 as ::core::ffi::c_uint as uint32_t),
    );
    b = b << 23 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
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
    d = d << 10 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(x[14 as usize])
            .wrapping_add(0xab9423a7 as ::core::ffi::c_uint as uint32_t),
    );
    c = c << 15 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(x[5 as usize])
            .wrapping_add(0xfc93a039 as ::core::ffi::c_uint as uint32_t),
    );
    b = b << 21 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
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
    d = d << 10 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(x[10 as usize])
            .wrapping_add(0xffeff47d as ::core::ffi::c_uint as uint32_t),
    );
    c = c << 15 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(x[1 as usize])
            .wrapping_add(0x85845dd1 as ::core::ffi::c_uint as uint32_t),
    );
    b = b << 21 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
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
    d = d << 10 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(x[6 as usize])
            .wrapping_add(0xa3014314 as ::core::ffi::c_uint as uint32_t),
    );
    c = c << 15 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(x[13 as usize])
            .wrapping_add(0x4e0811a1 as ::core::ffi::c_int as uint32_t),
    );
    b = b << 21 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
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
    d = d << 10 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(x[2 as usize])
            .wrapping_add(0x2ad7d2bb as ::core::ffi::c_int as uint32_t),
    );
    c = c << 15 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(x[9 as usize])
            .wrapping_add(0xeb86d391 as ::core::ffi::c_uint as uint32_t),
    );
    b = b << 21 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    *state.offset(0 as isize) = (*state.offset(0 as isize)).wrapping_add(a);
    *state.offset(1 as isize) = (*state.offset(1 as isize)).wrapping_add(b);
    *state.offset(2 as isize) = (*state.offset(2 as isize)).wrapping_add(c);
    *state.offset(3 as isize) = (*state.offset(3 as isize)).wrapping_add(d);
    memset(
        &raw mut x as *mut uint32_t as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint32_t; 16]>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn md5_init(mut ctx: *mut md5ctx) {
    (*ctx).count[1 as usize] = 0 as uint32_t;
    (*ctx).count[0 as usize] = (*ctx).count[1 as usize];
    (*ctx).state[0 as usize] = 0x67452301 as ::core::ffi::c_uint as uint32_t;
    (*ctx).state[1 as usize] = 0xefcdab89 as ::core::ffi::c_uint as uint32_t;
    (*ctx).state[2 as usize] = 0x98badcfe as ::core::ffi::c_uint as uint32_t;
    (*ctx).state[3 as usize] = 0x10325476 as ::core::ffi::c_uint as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn md5_update(
    mut ctx: *mut md5ctx,
    mut buff: *const uint8_t,
    mut leng: uint32_t,
) {
    let mut i: uint32_t = 0;
    let mut indx: uint32_t = 0;
    let mut partleng: uint32_t = 0;
    indx = (*ctx).count[0 as usize] >> 3 as ::core::ffi::c_int & 0x3f as uint32_t;
    (*ctx).count[0 as usize] =
        (*ctx).count[0 as usize].wrapping_add(leng << 3 as ::core::ffi::c_int);
    if (*ctx).count[0 as usize] < leng << 3 as ::core::ffi::c_int {
        (*ctx).count[1 as usize] = (*ctx).count[1 as usize].wrapping_add(1);
    }
    (*ctx).count[1 as usize] =
        (*ctx).count[1 as usize].wrapping_add(leng >> 29 as ::core::ffi::c_int);
    partleng = (64 as uint32_t).wrapping_sub(indx);
    if leng >= partleng {
        memcpy(
            (&raw mut (*ctx).buffer as *mut uint8_t).offset(indx as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            buff as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            partleng as size_t,
        );
        md5_transform(
            &raw mut (*ctx).state as *mut uint32_t,
            &raw mut (*ctx).buffer as *mut uint8_t as *const uint8_t,
        );
        i = partleng;
        while i.wrapping_add(63 as uint32_t) < leng {
            md5_transform(
                &raw mut (*ctx).state as *mut uint32_t,
                buff.offset(i as isize),
            );
            i = i.wrapping_add(64 as uint32_t);
        }
        indx = 0 as uint32_t;
    } else {
        i = 0 as uint32_t;
    }
    memcpy(
        (&raw mut (*ctx).buffer as *mut uint8_t).offset(indx as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        buff.offset(i as isize) as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        leng.wrapping_sub(i) as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn md5_final(mut digest: *mut uint8_t, mut ctx: *mut md5ctx) {
    let mut bits: [uint8_t; 8] = [0; 8];
    let mut indx: uint32_t = 0;
    let mut padleng: uint32_t = 0;
    md5_encode(
        &raw mut bits as *mut uint8_t,
        &raw mut (*ctx).count as *mut uint32_t,
        2 as uint32_t,
    );
    indx = (*ctx).count[0 as usize] >> 3 as ::core::ffi::c_int & 0x3f as uint32_t;
    padleng = if indx < 56 as uint32_t {
        (56 as uint32_t).wrapping_sub(indx)
    } else {
        (120 as uint32_t).wrapping_sub(indx)
    };
    md5_update(ctx, &raw const padding as *const uint8_t, padleng);
    md5_update(ctx, &raw mut bits as *mut uint8_t, 8 as uint32_t);
    md5_encode(
        digest as *mut uint8_t,
        &raw mut (*ctx).state as *mut uint32_t,
        4 as uint32_t,
    );
    memset(
        ctx as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<md5ctx>(),
    );
}
