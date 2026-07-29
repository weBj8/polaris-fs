unsafe extern "C" {
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn random() -> ::core::ffi::c_long;
    unsafe fn srandom(__seed: ::core::ffi::c_uint);
    unsafe fn monotonic_useconds() -> uint64_t;
}
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut i: uint8_t = 0;
static mut j: uint8_t = 0;
static mut p: [uint8_t; 256] = [0; 256];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rnd_init() -> ::core::ffi::c_int {
    unsafe {
        let mut key: [uint8_t; 64] = [0; 64];
        let mut vkey: [uint8_t; 64] = [0; 64];
        let mut x: uint8_t = 0;
        let mut l: uint16_t = 0;
        srandom(
            (time(::core::ptr::null_mut::<time_t>()) as uint64_t).wrapping_add(monotonic_useconds())
                as ::core::ffi::c_uint,
        );
        l = 0 as uint16_t;
        while (l as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            key[l as usize] = random() as uint8_t;
            vkey[l as usize] = random() as uint8_t;
            l = l.wrapping_add(1);
        }
        l = 0 as uint16_t;
        while (l as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
            p[l as usize] = l as uint8_t;
            l = l.wrapping_add(1);
        }
        l = 0 as uint16_t;
        while (l as ::core::ffi::c_int) < 768 as ::core::ffi::c_int {
            i = (l as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
            x = (j as ::core::ffi::c_int
                + p[i as usize] as ::core::ffi::c_int
                + key[(l as ::core::ffi::c_int % 64 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int) as uint8_t;
            j = p[x as usize];
            x = p[i as usize];
            p[i as usize] = p[j as usize];
            p[j as usize] = x;
            l = l.wrapping_add(1);
        }
        l = 0 as uint16_t;
        while (l as ::core::ffi::c_int) < 768 as ::core::ffi::c_int {
            i = (l as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
            x = (j as ::core::ffi::c_int
                + p[i as usize] as ::core::ffi::c_int
                + vkey[(l as ::core::ffi::c_int % 64 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int) as uint8_t;
            j = p[x as usize];
            x = p[i as usize];
            p[i as usize] = p[j as usize];
            p[j as usize] = x;
            l = l.wrapping_add(1);
        }
        i = 0 as uint8_t;
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rndu8() -> uint8_t {
    unsafe {
        let mut r: uint8_t = 0;
        let mut x: uint8_t = 0;
        x = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x as usize];
        x = p[j as usize];
        x = (p[x as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        r = p[x as usize];
        x = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x;
        i = i.wrapping_add(1);
        return r;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rndu32() -> uint32_t {
    unsafe {
        let mut res: uint32_t = 0;
        let mut r: *mut uint8_t = &raw mut res as *mut uint8_t;
        let mut x: uint8_t = 0;
        x = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x as usize];
        x = p[j as usize];
        x = (p[x as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        *r.offset(0 as isize) = p[x as usize];
        x = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x;
        i = i.wrapping_add(1);
        let mut x_0: uint8_t = 0;
        x_0 = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x_0 as usize];
        x_0 = p[j as usize];
        x_0 = (p[x_0 as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        *r.offset(1 as isize) = p[x_0 as usize];
        x_0 = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x_0;
        i = i.wrapping_add(1);
        let mut x_1: uint8_t = 0;
        x_1 = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x_1 as usize];
        x_1 = p[j as usize];
        x_1 = (p[x_1 as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        *r.offset(2 as isize) = p[x_1 as usize];
        x_1 = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x_1;
        i = i.wrapping_add(1);
        let mut x_2: uint8_t = 0;
        x_2 = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x_2 as usize];
        x_2 = p[j as usize];
        x_2 = (p[x_2 as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        *r.offset(3 as isize) = p[x_2 as usize];
        x_2 = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x_2;
        i = i.wrapping_add(1);
        return res;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rndu64() -> uint64_t {
    unsafe {
        let mut res: uint64_t = 0;
        let mut r: *mut uint8_t = &raw mut res as *mut uint8_t;
        let mut x: uint8_t = 0;
        x = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x as usize];
        x = p[j as usize];
        x = (p[x as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        *r.offset(0 as isize) = p[x as usize];
        x = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x;
        i = i.wrapping_add(1);
        let mut x_0: uint8_t = 0;
        x_0 = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x_0 as usize];
        x_0 = p[j as usize];
        x_0 = (p[x_0 as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        *r.offset(1 as isize) = p[x_0 as usize];
        x_0 = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x_0;
        i = i.wrapping_add(1);
        let mut x_1: uint8_t = 0;
        x_1 = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x_1 as usize];
        x_1 = p[j as usize];
        x_1 = (p[x_1 as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        *r.offset(2 as isize) = p[x_1 as usize];
        x_1 = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x_1;
        i = i.wrapping_add(1);
        let mut x_2: uint8_t = 0;
        x_2 = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x_2 as usize];
        x_2 = p[j as usize];
        x_2 = (p[x_2 as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        *r.offset(3 as isize) = p[x_2 as usize];
        x_2 = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x_2;
        i = i.wrapping_add(1);
        let mut x_3: uint8_t = 0;
        x_3 = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x_3 as usize];
        x_3 = p[j as usize];
        x_3 = (p[x_3 as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        *r.offset(4 as isize) = p[x_3 as usize];
        x_3 = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x_3;
        i = i.wrapping_add(1);
        let mut x_4: uint8_t = 0;
        x_4 = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x_4 as usize];
        x_4 = p[j as usize];
        x_4 = (p[x_4 as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        *r.offset(5 as isize) = p[x_4 as usize];
        x_4 = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x_4;
        i = i.wrapping_add(1);
        let mut x_5: uint8_t = 0;
        x_5 = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x_5 as usize];
        x_5 = p[j as usize];
        x_5 = (p[x_5 as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        *r.offset(6 as isize) = p[x_5 as usize];
        x_5 = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x_5;
        i = i.wrapping_add(1);
        let mut x_6: uint8_t = 0;
        x_6 = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
        j = p[x_6 as usize];
        x_6 = p[j as usize];
        x_6 = (p[x_6 as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        *r.offset(7 as isize) = p[x_6 as usize];
        x_6 = p[i as usize];
        p[i as usize] = p[j as usize];
        p[j as usize] = x_6;
        i = i.wrapping_add(1);
        return res;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rndbuff(mut buff: *mut uint8_t, mut size: uint32_t) {
    unsafe {
        let mut k: uint32_t = 0;
        k = 0 as uint32_t;
        while k < size {
            let mut x: uint8_t = 0;
            x = (j as ::core::ffi::c_int + p[i as usize] as ::core::ffi::c_int) as uint8_t;
            j = p[x as usize];
            x = p[j as usize];
            x = (p[x as usize] as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
            *buff.offset(k as isize) = p[x as usize];
            x = p[i as usize];
            p[i as usize] = p[j as usize];
            p[j as usize] = x;
            i = i.wrapping_add(1);
            k = k.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rndu64_ranged(mut range: uint64_t) -> uint64_t {
    unsafe {
        let mut max: uint64_t = 0;
        let mut r: uint64_t = 0;
        r = rndu64();
        if range == 0 as uint64_t {
            return r;
        }
        max = range.wrapping_neg().wrapping_rem(range).wrapping_neg();
        if max != 0 {
            while r >= max {
                r = rndu64();
            }
        }
        return r.wrapping_rem(range);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rndu32_ranged(mut range: uint32_t) -> uint32_t {
    unsafe {
        let mut max: uint32_t = 0;
        let mut r: uint32_t = 0;
        r = rndu32();
        if range == 0 as uint32_t {
            return r;
        }
        max = range.wrapping_neg().wrapping_rem(range).wrapping_neg();
        if max != 0 {
            while r >= max {
                r = rndu32();
            }
        }
        return r.wrapping_rem(range);
    }
}
