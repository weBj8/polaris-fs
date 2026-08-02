unsafe extern "C" {
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    unsafe fn parse_label_expr(
        exprstr: *const ::core::ffi::c_char,
        pd_0: *mut parser_data,
    ) -> ::core::ffi::c_int;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _parser_data {
    pub uniqmask: uint32_t,
    pub labels_mode: uint8_t,
    pub ec_data_chksum_parts: uint8_t,
    pub labelscnt: uint8_t,
    pub labelexpr: [[uint8_t; 128]; 9],
}
pub type parser_data = _parser_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cspri {
    pub ip: uint32_t,
    pub port: uint16_t,
    pub version: uint32_t,
    pub labelmask: uint32_t,
    pub priority: uint32_t,
}
pub type cspri = _cspri;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SCLASS_EXPR_TYPE_MASK: ::core::ffi::c_int = 0xc0 as ::core::ffi::c_int;
pub const SCLASS_EXPR_VALUE_MASK: ::core::ffi::c_int = 0x3f as ::core::ffi::c_int;
pub const SCLASS_EXPR_SYMBOL: ::core::ffi::c_int = 192;
pub const SCLASS_EXPR_SYMBOL_ANY: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
pub const SCLASS_EXPR_OP_AND: ::core::ffi::c_int = 128;
pub const SCLASS_EXPR_OP_OR: ::core::ffi::c_int = 64;
pub const SCLASS_EXPR_OP_ONE: ::core::ffi::c_int = 0;
pub const SCLASS_EXPR_OP_NOT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn get32bit(mut ptr: *mut *const uint8_t) -> uint32_t {
    unsafe {
        let mut t32: uint32_t = 0;
        memcpy(
            &raw mut t32 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            4 as size_t,
        );
        *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
        return t32.swap_bytes();
    }
}
#[inline]
unsafe extern "C" fn get16bit(mut ptr: *mut *const uint8_t) -> uint16_t {
    unsafe {
        let mut t16: uint16_t = 0;
        memcpy(
            &raw mut t16 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            2 as size_t,
        );
        *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
        return t16.swap_bytes();
    }
}
#[unsafe(no_mangle)]
pub static mut pd: parser_data = parser_data {
    uniqmask: 0,
    labels_mode: 0,
    ec_data_chksum_parts: 0,
    labelscnt: 0,
    labelexpr: [[0; 128]; 9],
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csorder_init(
    mut labelexpr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if labelexpr.is_null() {
            pd.labelscnt = 0 as uint8_t;
            return 0 as ::core::ffi::c_int;
        } else {
            return parse_label_expr(labelexpr, &raw mut pd);
        };
    }
}
#[inline]
unsafe extern "C" fn csorder_server_matches_labelexpr(
    mut labelmask: uint32_t,
    mut labelexpr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        static mut stack: [uint8_t; 128] = [0; 128];
        let mut n: uint8_t = 0;
        let mut r: uint8_t = 0;
        let mut sp: uint8_t = 0;
        sp = 0 as uint8_t;
        if *labelexpr != 0 {
            loop {
                let c2rust_fresh0 = labelexpr;
                labelexpr = labelexpr.offset(1);
                n = *c2rust_fresh0;
                if n == 0 {
                    break;
                }
                match n as ::core::ffi::c_int & SCLASS_EXPR_TYPE_MASK {
                    SCLASS_EXPR_SYMBOL => {
                        if n as ::core::ffi::c_int == SCLASS_EXPR_SYMBOL_ANY {
                            let c2rust_fresh1 = sp;
                            sp = sp.wrapping_add(1);
                            stack[c2rust_fresh1 as usize] = 1 as uint8_t;
                        } else {
                            n = (n as ::core::ffi::c_int & SCLASS_EXPR_VALUE_MASK) as uint8_t;
                            if labelmask
                                & ((1 as ::core::ffi::c_int) << n as ::core::ffi::c_int) as uint32_t
                                != 0
                            {
                                let c2rust_fresh2 = sp;
                                sp = sp.wrapping_add(1);
                                stack[c2rust_fresh2 as usize] = 1 as uint8_t;
                            } else {
                                let c2rust_fresh3 = sp;
                                sp = sp.wrapping_add(1);
                                stack[c2rust_fresh3 as usize] = 0 as uint8_t;
                            }
                        }
                    }
                    SCLASS_EXPR_OP_AND => {
                        n = (n as ::core::ffi::c_int & SCLASS_EXPR_VALUE_MASK) as uint8_t;
                        n = (n as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t;
                        if n as ::core::ffi::c_int > sp as ::core::ffi::c_int {
                            return 0 as uint8_t;
                        }
                        r = 1 as uint8_t;
                        while n as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                            sp = sp.wrapping_sub(1);
                            if stack[sp as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                r = 0 as uint8_t;
                            }
                            n = n.wrapping_sub(1);
                        }
                        let c2rust_fresh4 = sp;
                        sp = sp.wrapping_add(1);
                        stack[c2rust_fresh4 as usize] = r;
                    }
                    SCLASS_EXPR_OP_OR => {
                        n = (n as ::core::ffi::c_int & SCLASS_EXPR_VALUE_MASK) as uint8_t;
                        n = (n as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t;
                        if n as ::core::ffi::c_int > sp as ::core::ffi::c_int {
                            return 0 as uint8_t;
                        }
                        r = 0 as uint8_t;
                        while n as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                            sp = sp.wrapping_sub(1);
                            if stack[sp as usize] as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                                r = 1 as uint8_t;
                            }
                            n = n.wrapping_sub(1);
                        }
                        let c2rust_fresh5 = sp;
                        sp = sp.wrapping_add(1);
                        stack[c2rust_fresh5 as usize] = r;
                    }
                    SCLASS_EXPR_OP_ONE => {
                        n = (n as ::core::ffi::c_int & SCLASS_EXPR_VALUE_MASK) as uint8_t;
                        if n as ::core::ffi::c_int == SCLASS_EXPR_OP_NOT {
                            if sp as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                return 0 as uint8_t;
                            }
                            stack[(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] =
                                (1 as ::core::ffi::c_int
                                    - stack[(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                        as usize]
                                        as ::core::ffi::c_int)
                                    as uint8_t;
                        }
                    }
                    _ => {}
                }
            }
            if sp as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                return stack[0 as usize];
            }
            return 0 as uint8_t;
        } else {
            return 1 as uint8_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csorder_calc(mut labelmask: uint32_t) -> uint8_t {
    unsafe {
        let mut i: uint8_t = 0;
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < pd.labelscnt as ::core::ffi::c_int {
            if csorder_server_matches_labelexpr(
                labelmask,
                &raw mut *(&raw mut pd.labelexpr as *mut [uint8_t; 128]).offset(i as isize)
                    as *mut uint8_t,
            ) != 0
            {
                return i;
            }
            i = i.wrapping_add(1);
        }
        return pd.labelscnt;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csorder_cmp(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aa: *mut cspri = a as *mut cspri;
        let mut bb: *mut cspri = b as *mut cspri;
        if (*aa).priority < (*bb).priority {
            return -1 as ::core::ffi::c_int;
        } else if (*aa).priority > (*bb).priority {
            return 1 as ::core::ffi::c_int;
        } else {
            return 0 as ::core::ffi::c_int;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csorder_sort(
    mut chain: *mut cspri,
    mut csdataver: uint8_t,
    mut csdata: *const uint8_t,
    mut csdatasize: uint32_t,
    mut writeflag: uint8_t,
) -> uint32_t {
    unsafe {
        let mut cp: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut cpe: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        cp = csdata;
        cpe = csdata.offset(csdatasize as isize);
        i = 0 as uint32_t;
        while cp < cpe && i < 100 as uint32_t {
            (*chain.offset(i as isize)).ip = get32bit(&raw mut cp);
            (*chain.offset(i as isize)).port = get16bit(&raw mut cp);
            if csdataver as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                (*chain.offset(i as isize)).version = get32bit(&raw mut cp);
            } else {
                (*chain.offset(i as isize)).version = 0 as uint32_t;
            }
            if csdataver as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                (*chain.offset(i as isize)).labelmask = get32bit(&raw mut cp);
            } else {
                (*chain.offset(i as isize)).labelmask = 0 as uint32_t;
            }
            (*chain.offset(i as isize)).priority =
                csorder_calc((*chain.offset(i as isize)).labelmask) as uint32_t;
            (*chain.offset(i as isize)).priority <<= 24 as ::core::ffi::c_int;
            if writeflag != 0 {
                (*chain.offset(i as isize)).priority =
                    (*chain.offset(i as isize)).priority.wrapping_add(i);
            } else {
                (*chain.offset(i as isize)).priority = (*chain.offset(i as isize))
                    .priority
                    .wrapping_add(crate::csdb::get_operations(
                        (*chain.offset(i as isize)).ip,
                        (*chain.offset(i as isize)).port,
                    ));
            }
            i = i.wrapping_add(1);
        }
        if csdataver as ::core::ffi::c_int != 3 as ::core::ffi::c_int {
            qsort(
                chain as *mut ::core::ffi::c_void,
                i as size_t,
                ::core::mem::size_of::<cspri>(),
                Some(
                    csorder_cmp
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
        }
        return i;
    }
}
