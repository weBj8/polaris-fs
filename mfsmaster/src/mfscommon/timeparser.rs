unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const TPARSE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TPARSE_UNEXPECTED_CHAR: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const TPARSE_VALUE_TOO_BIG: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snprint_time_common(
    mut r#where: *mut ::core::ffi::c_char,
    mut mleng: size_t,
    mut period: uint32_t,
    mut timetab: *mut uint32_t,
    mut timesym: *mut ::core::ffi::c_char,
    mut timecount: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: uint64_t = 0;
        let mut p10: uint64_t = 0;
        let mut timebase: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut intpart: uint32_t = 0;
        let mut decpart: uint32_t = 0;
        let mut resultlen: uint32_t = 0;
        resultlen = 0 as uint32_t;
        if mleng <= 0 as size_t {
            return resultlen as ::core::ffi::c_int;
        }
        if period == 0 as uint32_t {
            return snprintf(
                r#where,
                mleng,
                b"0%c\0".as_ptr() as *const ::core::ffi::c_char,
                *timesym.offset(timecount.wrapping_sub(1 as uint32_t) as isize)
                    as ::core::ffi::c_int,
            );
        }
        i = 0 as uint32_t;
        while i < timecount {
            if period >= *timetab.offset(i as isize) {
                timebase = *timetab.offset(i as isize);
                n = period as uint64_t;
                n = n.wrapping_mul(10 as uint64_t);
                p10 = n;
                if timebase > 1 as uint32_t {
                    n = n.wrapping_add(timebase.wrapping_div(2 as uint32_t) as uint64_t);
                    n = n.wrapping_div(timebase as uint64_t);
                }
                if n.wrapping_mul(timebase as uint64_t) != p10 {
                    *r#where.offset(0 as isize) = '~' as ::core::ffi::c_char;
                    r#where = r#where.offset(1);
                    mleng = mleng.wrapping_sub(1);
                    resultlen = 1 as uint32_t;
                    if mleng == 0 as size_t {
                        return resultlen as ::core::ffi::c_int;
                    }
                }
                intpart = n.wrapping_div(10 as uint64_t) as uint32_t;
                decpart = n.wrapping_rem(10 as uint64_t) as uint32_t;
                if decpart != 0 {
                    resultlen = resultlen.wrapping_add(snprintf(
                        r#where,
                        mleng,
                        b"%u.%u%c\0".as_ptr() as *const ::core::ffi::c_char,
                        intpart,
                        decpart,
                        *timesym.offset(i as isize) as ::core::ffi::c_int,
                    ) as uint32_t);
                } else {
                    resultlen = resultlen.wrapping_add(snprintf(
                        r#where,
                        mleng,
                        b"%u%c\0".as_ptr() as *const ::core::ffi::c_char,
                        intpart,
                        *timesym.offset(i as isize) as ::core::ffi::c_int,
                    ) as uint32_t);
                }
                return resultlen as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        if (resultlen as size_t) < mleng {
            *r#where.offset(resultlen as isize) = 0 as ::core::ffi::c_char;
        }
        return resultlen as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_period_common(
    mut str: *const ::core::ffi::c_char,
    mut ret: *mut uint32_t,
    mut hmode: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut res: uint64_t = 0;
        let mut cur: uint64_t = 0;
        let mut ndiv: uint64_t = 0;
        let mut max: uint64_t = 0;
        let mut mul: uint32_t = 0;
        let mut mask: uint8_t = 0;
        let mut cmask: uint8_t = 0;
        mask = 0 as uint8_t;
        res = 0 as uint64_t;
        while *str as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *str as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            str = str.offset(1);
        }
        loop {
            if *str as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *str as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                if mask as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    == 0xc0 as ::core::ffi::c_int
                {
                    *ret = *str as uint32_t;
                    return TPARSE_UNEXPECTED_CHAR;
                }
                cur = 0 as uint64_t;
                ndiv = 1 as uint64_t;
                while *str as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *str as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    && cur < UINT32_MAX as uint64_t
                {
                    cur = (cur as ::core::ffi::c_double * 10.0f64) as uint64_t;
                    cur = cur.wrapping_add(
                        (*str as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint64_t,
                    );
                    ndiv = ndiv.wrapping_mul(10 as uint64_t);
                    str = str.offset(1);
                }
                if cur > UINT32_MAX as uint64_t {
                    *ret = 0 as uint32_t;
                    return TPARSE_VALUE_TOO_BIG;
                }
                while *str as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                    || *str as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                {
                    str = str.offset(1);
                }
            } else {
                *ret = *str as uint32_t;
                return TPARSE_UNEXPECTED_CHAR;
            }
            match *str as ::core::ffi::c_int {
                46 => {
                    if mask as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        *ret = *str as uint32_t;
                        return TPARSE_UNEXPECTED_CHAR;
                    }
                    mask = (mask as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as uint8_t;
                    res = cur;
                    str = str.offset(1);
                    cmask = 0x80 as uint8_t;
                }
                119 | 87 => {
                    if hmode != 0 {
                        mul = (7 as ::core::ffi::c_int * 24 as ::core::ffi::c_int) as uint32_t;
                    } else {
                        mul = (7 as ::core::ffi::c_int
                            * 24 as ::core::ffi::c_int
                            * 3600 as ::core::ffi::c_int) as uint32_t;
                    }
                    max = UINT32_MAX as uint64_t;
                    cmask = 0x10 as uint8_t;
                }
                100 | 68 => {
                    if hmode != 0 {
                        mul = 24 as uint32_t;
                        max = (7 as ::core::ffi::c_int * 24 as ::core::ffi::c_int) as uint64_t;
                    } else {
                        mul = (24 as ::core::ffi::c_int * 3600 as ::core::ffi::c_int) as uint32_t;
                        max = (7 as ::core::ffi::c_int
                            * 24 as ::core::ffi::c_int
                            * 3600 as ::core::ffi::c_int) as uint64_t;
                    }
                    cmask = 0x8 as uint8_t;
                }
                104 | 72 => {
                    if hmode != 0 {
                        mul = 1 as uint32_t;
                        max = 24 as uint64_t;
                    } else {
                        mul = 3600 as uint32_t;
                        max = (24 as ::core::ffi::c_int * 3600 as ::core::ffi::c_int) as uint64_t;
                    }
                    cmask = 0x4 as uint8_t;
                }
                109 | 77 => {
                    if hmode != 0 {
                        *ret = *str as uint32_t;
                        return TPARSE_UNEXPECTED_CHAR;
                    }
                    mul = 60 as uint32_t;
                    max = 3600 as uint64_t;
                    cmask = 0x2 as uint8_t;
                }
                115 | 83 => {
                    if hmode != 0 {
                        *ret = *str as uint32_t;
                        return TPARSE_UNEXPECTED_CHAR;
                    }
                    mul = 1 as uint32_t;
                    max = 60 as uint64_t;
                    cmask = 0x1 as uint8_t;
                }
                0 => {
                    if mask as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        *ret = cur as uint32_t;
                        return TPARSE_OK;
                    }
                    *ret = 0 as uint32_t;
                    return TPARSE_UNEXPECTED_CHAR;
                }
                _ => {
                    *ret = *str as uint32_t;
                    return TPARSE_UNEXPECTED_CHAR;
                }
            }
            if cmask as ::core::ffi::c_int != 0x80 as ::core::ffi::c_int {
                cur = cur.wrapping_mul(mul as uint64_t);
                if mask as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int {
                    res = res.wrapping_mul(mul as uint64_t);
                    cur = cur.wrapping_mul(10 as uint64_t).wrapping_div(ndiv);
                    cur = cur.wrapping_div(10 as uint64_t).wrapping_add(
                        (if cur.wrapping_rem(10 as uint64_t) >= 5 as uint64_t {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as uint64_t,
                    );
                    res = res.wrapping_add(cur);
                    mask = (mask as ::core::ffi::c_int | 0x40 as ::core::ffi::c_int) as uint8_t;
                    str = str.offset(1);
                } else if mask as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    mask = (mask as ::core::ffi::c_int | cmask as ::core::ffi::c_int) as uint8_t;
                    res = cur;
                    str = str.offset(1);
                } else if mask as ::core::ffi::c_int
                    & (0xc0 as ::core::ffi::c_int
                        | ((cmask as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int)
                    == 0 as ::core::ffi::c_int
                {
                    if cur < max {
                        mask =
                            (mask as ::core::ffi::c_int | cmask as ::core::ffi::c_int) as uint8_t;
                        res = res.wrapping_add(cur);
                        str = str.offset(1);
                    } else {
                        *ret = *str as uint32_t;
                        return TPARSE_VALUE_TOO_BIG;
                    }
                } else {
                    *ret = *str as uint32_t;
                    return TPARSE_UNEXPECTED_CHAR;
                }
                if res > UINT32_MAX as uint64_t {
                    *ret = 0 as uint32_t;
                    return TPARSE_VALUE_TOO_BIG;
                }
                while *str as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                    || *str as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                {
                    str = str.offset(1);
                }
                if *str as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                    || *str as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                    || *str as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                {
                    *ret = res as uint32_t;
                    return TPARSE_OK;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snprint_speriod(
    mut r#where: *mut ::core::ffi::c_char,
    mut mleng: size_t,
    mut period: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        static mut timetab: [uint32_t; 5] = [
            604800 as uint32_t,
            86400 as uint32_t,
            3600 as uint32_t,
            60 as uint32_t,
            1 as uint32_t,
        ];
        static mut timesym: [::core::ffi::c_char; 5] = [
            'w' as ::core::ffi::c_char,
            'd' as ::core::ffi::c_char,
            'h' as ::core::ffi::c_char,
            'm' as ::core::ffi::c_char,
            's' as ::core::ffi::c_char,
        ];
        return snprint_time_common(
            r#where,
            mleng,
            period,
            &raw mut timetab as *mut uint32_t,
            &raw mut timesym as *mut ::core::ffi::c_char,
            5 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_speriod(
    mut str: *const ::core::ffi::c_char,
    mut ret: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        return parse_period_common(str, ret, 0 as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snprint_hperiod(
    mut r#where: *mut ::core::ffi::c_char,
    mut mleng: size_t,
    mut period: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        static mut timetab: [uint32_t; 3] = [168 as uint32_t, 24 as uint32_t, 1 as uint32_t];
        static mut timesym: [::core::ffi::c_char; 3] = [
            'w' as ::core::ffi::c_char,
            'd' as ::core::ffi::c_char,
            'h' as ::core::ffi::c_char,
        ];
        return snprint_time_common(
            r#where,
            mleng,
            period,
            &raw mut timetab as *mut uint32_t,
            &raw mut timesym as *mut ::core::ffi::c_char,
            3 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_hperiod(
    mut str: *const ::core::ffi::c_char,
    mut ret: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        return parse_period_common(str, ret, 1 as uint8_t);
    }
}
