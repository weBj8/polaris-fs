pub type uint8_t = u8;
pub type uint32_t = u32;
pub const CRC_POLY: ::core::ffi::c_uint = 0xedb88320 as ::core::ffi::c_uint;
static mut crc_table: [[uint32_t; 256]; 16] = [[0; 256]; 16];
unsafe extern "C" fn crc_generate_main_tables() {
    let mut c: uint32_t = 0;
    let mut poly: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    poly = CRC_POLY as uint32_t;
    i = 0 as uint32_t;
    while i < 256 as uint32_t {
        c = i;
        c = if c & 1 as uint32_t != 0 {
            poly ^ c >> 1 as ::core::ffi::c_int
        } else {
            c >> 1 as ::core::ffi::c_int
        };
        c = if c & 1 as uint32_t != 0 {
            poly ^ c >> 1 as ::core::ffi::c_int
        } else {
            c >> 1 as ::core::ffi::c_int
        };
        c = if c & 1 as uint32_t != 0 {
            poly ^ c >> 1 as ::core::ffi::c_int
        } else {
            c >> 1 as ::core::ffi::c_int
        };
        c = if c & 1 as uint32_t != 0 {
            poly ^ c >> 1 as ::core::ffi::c_int
        } else {
            c >> 1 as ::core::ffi::c_int
        };
        c = if c & 1 as uint32_t != 0 {
            poly ^ c >> 1 as ::core::ffi::c_int
        } else {
            c >> 1 as ::core::ffi::c_int
        };
        c = if c & 1 as uint32_t != 0 {
            poly ^ c >> 1 as ::core::ffi::c_int
        } else {
            c >> 1 as ::core::ffi::c_int
        };
        c = if c & 1 as uint32_t != 0 {
            poly ^ c >> 1 as ::core::ffi::c_int
        } else {
            c >> 1 as ::core::ffi::c_int
        };
        c = if c & 1 as uint32_t != 0 {
            poly ^ c >> 1 as ::core::ffi::c_int
        } else {
            c >> 1 as ::core::ffi::c_int
        };
        crc_table[0 as usize][i as usize] = c;
        i = i.wrapping_add(1);
    }
    i = 0 as uint32_t;
    while i < 256 as uint32_t {
        c = crc_table[0 as usize][i as usize];
        j = 1 as uint32_t;
        while j < 16 as uint32_t {
            c = crc_table[0 as usize][(c & 0xff as uint32_t) as usize]
                ^ c >> 8 as ::core::ffi::c_int;
            crc_table[j as usize][i as usize] = c;
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mycrc32(
    mut crc: uint32_t,
    mut data: *const ::core::ffi::c_void,
    mut leng: uint32_t,
) -> uint32_t {
    let mut data4: *const uint32_t = ::core::ptr::null::<uint32_t>();
    let mut data1: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut d0: uint32_t = 0;
    let mut d1: uint32_t = 0;
    let mut d2: uint32_t = 0;
    let mut d3: uint32_t = 0;
    crc = !crc;
    data1 = data as *const uint8_t;
    while data1.expose_addr() as ::core::ffi::c_ulong & 0x3 as ::core::ffi::c_ulong != 0
        && leng != 0 as uint32_t
    {
        let c2rust_fresh0 = data1;
        data1 = data1.offset(1);
        crc = crc >> 8 as ::core::ffi::c_int
            ^ crc_table[0 as usize][(crc & 0xff as uint32_t ^ *c2rust_fresh0 as uint32_t) as usize];
        leng = leng.wrapping_sub(1);
    }
    data4 = data1 as *const uint32_t;
    while leng >= 16 as uint32_t {
        let c2rust_fresh1 = data4;
        data4 = data4.offset(1);
        d0 = *c2rust_fresh1 ^ crc;
        let c2rust_fresh2 = data4;
        data4 = data4.offset(1);
        d1 = *c2rust_fresh2;
        let c2rust_fresh3 = data4;
        data4 = data4.offset(1);
        d2 = *c2rust_fresh3;
        let c2rust_fresh4 = data4;
        data4 = data4.offset(1);
        d3 = *c2rust_fresh4;
        crc = crc_table[0 as usize][(d3 >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ crc_table[1 as usize][(d3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ crc_table[2 as usize][(d3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ crc_table[3 as usize][(d3 & 0xff as uint32_t) as usize]
            ^ crc_table[4 as usize][(d2 >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ crc_table[5 as usize][(d2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ crc_table[6 as usize][(d2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ crc_table[7 as usize][(d2 & 0xff as uint32_t) as usize]
            ^ crc_table[8 as usize][(d1 >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ crc_table[9 as usize][(d1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ crc_table[10 as usize][(d1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ crc_table[11 as usize][(d1 & 0xff as uint32_t) as usize]
            ^ crc_table[12 as usize][(d0 >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ crc_table[13 as usize][(d0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ crc_table[14 as usize][(d0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ crc_table[15 as usize][(d0 & 0xff as uint32_t) as usize];
        leng = leng.wrapping_sub(16 as uint32_t);
    }
    data1 = data4 as *const uint8_t;
    while leng != 0 as uint32_t {
        let c2rust_fresh5 = data1;
        data1 = data1.offset(1);
        crc = crc >> 8 as ::core::ffi::c_int
            ^ crc_table[0 as usize][(crc & 0xff as uint32_t ^ *c2rust_fresh5 as uint32_t) as usize];
        leng = leng.wrapping_sub(1);
    }
    return !crc;
}
static mut crc_combine_table: [[[uint32_t; 256]; 4]; 32] = [[[0; 256]; 4]; 32];
unsafe extern "C" fn crc_matrix_square(mut sqr: *mut uint32_t, mut m: *mut uint32_t) {
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut s: uint32_t = 0;
    let mut v: uint32_t = 0;
    i = 0 as uint32_t;
    while i < 32 as uint32_t {
        j = 0 as uint32_t;
        s = 0 as uint32_t;
        v = *m.offset(i as isize);
        while v != 0 && j < 32 as uint32_t {
            if v & 1 as uint32_t != 0 {
                s ^= *m.offset(j as isize);
            }
            j = j.wrapping_add(1);
            v >>= 1 as ::core::ffi::c_int;
        }
        *sqr.offset(i as isize) = s;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn crc_generate_combine_tables() {
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut k: uint32_t = 0;
    let mut l: uint32_t = 0;
    let mut sum: uint32_t = 0;
    let mut m1: [uint32_t; 32] = [0; 32];
    let mut m2: [uint32_t; 32] = [0; 32];
    let mut mc: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut m: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    m1[0 as usize] = CRC_POLY as uint32_t;
    j = 1 as uint32_t;
    i = 1 as uint32_t;
    while i < 32 as uint32_t {
        m1[i as usize] = j;
        j <<= 1 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    crc_matrix_square(&raw mut m2 as *mut uint32_t, &raw mut m1 as *mut uint32_t);
    crc_matrix_square(&raw mut m1 as *mut uint32_t, &raw mut m2 as *mut uint32_t);
    i = 0 as uint32_t;
    while i < 32 as uint32_t {
        if i & 1 as uint32_t != 0 {
            crc_matrix_square(&raw mut m1 as *mut uint32_t, &raw mut m2 as *mut uint32_t);
            mc = &raw mut m1 as *mut uint32_t;
        } else {
            crc_matrix_square(&raw mut m2 as *mut uint32_t, &raw mut m1 as *mut uint32_t);
            mc = &raw mut m2 as *mut uint32_t;
        }
        j = 0 as uint32_t;
        while j < 4 as uint32_t {
            k = 0 as uint32_t;
            while k < 256 as uint32_t {
                sum = 0 as uint32_t;
                l = k;
                m = mc.offset(j.wrapping_mul(8 as uint32_t) as isize);
                while l != 0 {
                    if l & 1 as uint32_t != 0 {
                        sum ^= *m;
                    }
                    l >>= 1 as ::core::ffi::c_int;
                    m = m.offset(1);
                }
                crc_combine_table[i as usize][j as usize][k as usize] = sum;
                k = k.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mycrc32_combine(
    mut crc1: uint32_t,
    mut crc2: uint32_t,
    mut leng2: uint32_t,
) -> uint32_t {
    let mut i: uint8_t = 0;
    i = 0 as uint8_t;
    while leng2 != 0 {
        if leng2 & 1 as uint32_t != 0 {
            crc1 = crc_combine_table[i as usize][3 as usize]
                [(crc1 >> 24 as ::core::ffi::c_int) as usize]
                ^ crc_combine_table[i as usize][2 as usize]
                    [(crc1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ crc_combine_table[i as usize][1 as usize]
                    [(crc1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ crc_combine_table[i as usize][0 as usize][(crc1 & 0xff as uint32_t) as usize];
        }
        i = i.wrapping_add(1);
        leng2 >>= 1 as ::core::ffi::c_int;
    }
    return crc1 ^ crc2;
}
#[no_mangle]
pub unsafe extern "C" fn mycrc32_init() {
    crc_generate_main_tables();
    crc_generate_combine_tables();
}
