unsafe extern "C" {
    unsafe fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type int8_t = i8;
pub type uint8_t = u8;
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
pub struct _expr_string {
    pub level: uint8_t,
    pub len: uint8_t,
    pub str: *mut ::core::ffi::c_char,
}
pub type node = _node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _node {
    pub op: uint8_t,
    pub val: uint8_t,
    pub arg1: *mut _node,
    pub arg2: *mut _node,
}
pub type expr = _expr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _expr {
    pub str: *const ::core::ffi::c_char,
    pub uniqmask: uint32_t,
    pub labels_mode: uint8_t,
    pub terms: [*mut node; 9],
    pub erroroccured: uint8_t,
    pub ec_data_chksum_parts: uint8_t,
}
pub const REF: C2Rust_Unnamed = 3;
pub type rpnbuff = _rpnbuff;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rpnbuff {
    pub pos: uint8_t,
    pub rpndata: [uint8_t; 128],
}
pub const ANY: C2Rust_Unnamed = 4;
pub const NOT: C2Rust_Unnamed = 2;
pub const AND: C2Rust_Unnamed = 1;
pub const OR: C2Rust_Unnamed = 0;
pub const SYM: C2Rust_Unnamed = 5;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SCLASS_EXPR_MAX_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const SCLASS_EXPR_TYPE_MASK: ::core::ffi::c_int = 0xc0 as ::core::ffi::c_int;
pub const SCLASS_EXPR_VALUE_MASK: ::core::ffi::c_int = 0x3f as ::core::ffi::c_int;
pub const SCLASS_EXPR_SYMBOL: ::core::ffi::c_int = 192;
pub const SCLASS_EXPR_SYMBOL_ANY: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
pub const SCLASS_EXPR_OP_AND: ::core::ffi::c_int = 128;
pub const SCLASS_EXPR_OP_OR: ::core::ffi::c_int = 64;
pub const SCLASS_EXPR_OP_ONE: ::core::ffi::c_int = 0;
pub const SCLASS_EXPR_OP_NOT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const UNIQ_MASK_IP: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
    << 1 as ::core::ffi::c_int + 'Z' as ::core::ffi::c_int - 'A' as ::core::ffi::c_int;
pub const UNIQ_MASK_RACK: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
    << 2 as ::core::ffi::c_int + 'Z' as ::core::ffi::c_int - 'A' as ::core::ffi::c_int;
pub const LABELS_MODE_LOOSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LABELS_MODE_STD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LABELS_MODE_STRICT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LABELS_MODE_GLOBAL: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn labelexpr_diff(
    mut labelexpr1: *const uint8_t,
    mut labelexpr2: *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut n1: uint8_t = 0;
        let mut n2: uint8_t = 0;
        loop {
            n1 = *labelexpr1;
            n2 = *labelexpr2;
            labelexpr1 = labelexpr1.offset(1);
            labelexpr2 = labelexpr2.offset(1);
            if n1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || n2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                return (if n1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && n2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t;
            }
            if n1 as ::core::ffi::c_int != n2 as ::core::ffi::c_int {
                return 0 as uint8_t;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn rpn_to_infix(
    mut labelexpr: *const uint8_t,
    mut outstr: *mut ::core::ffi::c_char,
) -> uint8_t {
    unsafe {
        let mut stack: [_expr_string; 128] = [_expr_string {
            level: 0,
            len: 0,
            str: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        }; 128];
        let mut n: uint8_t = 0;
        let mut r: uint8_t = 0;
        let mut l: uint8_t = 0;
        let mut sp: uint8_t = 0;
        let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
                            stack[sp as usize].level = 0 as uint8_t;
                            stack[sp as usize].len = 1 as uint8_t;
                            stack[sp as usize].str =
                                malloc(1 as size_t) as *mut ::core::ffi::c_char;
                            *stack[sp as usize].str.offset(0 as isize) = '*' as ::core::ffi::c_char;
                        } else {
                            n = (n as ::core::ffi::c_int & SCLASS_EXPR_VALUE_MASK) as uint8_t;
                            stack[sp as usize].level = 0 as uint8_t;
                            stack[sp as usize].len = 1 as uint8_t;
                            stack[sp as usize].str =
                                malloc(1 as size_t) as *mut ::core::ffi::c_char;
                            *stack[sp as usize].str.offset(0 as isize) = ('A' as ::core::ffi::c_int
                                + n as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        }
                        sp = sp.wrapping_add(1);
                    }
                    SCLASS_EXPR_OP_AND => {
                        n = (n as ::core::ffi::c_int & SCLASS_EXPR_VALUE_MASK) as uint8_t;
                        n = (n as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t;
                        if n as ::core::ffi::c_int > sp as ::core::ffi::c_int {
                            while sp as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                sp = sp.wrapping_sub(1);
                                if !stack[sp as usize].str.is_null() {
                                    free(stack[sp as usize].str as *mut ::core::ffi::c_void);
                                }
                            }
                            return 0 as uint8_t;
                        }
                        l = 0 as uint8_t;
                        r = n;
                        while r as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            if stack[(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                .level as ::core::ffi::c_int
                                > 1 as ::core::ffi::c_int
                            {
                                l = (l as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t;
                            }
                            l = (l as ::core::ffi::c_int
                                + stack
                                    [(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                    .len as ::core::ffi::c_int)
                                as uint8_t;
                            r = r.wrapping_sub(1);
                        }
                        l = (l as ::core::ffi::c_int
                            + (n as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                            as uint8_t;
                        str = malloc(l as size_t) as *mut ::core::ffi::c_char;
                        l = 0 as uint8_t;
                        r = n;
                        while r as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            if r as ::core::ffi::c_int != n as ::core::ffi::c_int {
                                let c2rust_fresh1 = l;
                                l = l.wrapping_add(1);
                                *str.offset(c2rust_fresh1 as isize) = '&' as ::core::ffi::c_char;
                            }
                            if stack[(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                .level as ::core::ffi::c_int
                                > 1 as ::core::ffi::c_int
                            {
                                let c2rust_fresh2 = l;
                                l = l.wrapping_add(1);
                                *str.offset(c2rust_fresh2 as isize) = '(' as ::core::ffi::c_char;
                            }
                            memcpy(
                                str.offset(l as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_void,
                                stack[(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                    .str
                                    as *const ::core::ffi::c_void,
                                stack[(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                    .len as size_t,
                            );
                            l = (l as ::core::ffi::c_int
                                + stack
                                    [(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                    .len as ::core::ffi::c_int)
                                as uint8_t;
                            free(
                                stack[(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                    .str
                                    as *mut ::core::ffi::c_void,
                            );
                            if stack[(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                .level as ::core::ffi::c_int
                                > 1 as ::core::ffi::c_int
                            {
                                let c2rust_fresh3 = l;
                                l = l.wrapping_add(1);
                                *str.offset(c2rust_fresh3 as isize) = ')' as ::core::ffi::c_char;
                            }
                            r = r.wrapping_sub(1);
                        }
                        sp = (sp as ::core::ffi::c_int - n as ::core::ffi::c_int) as uint8_t;
                        stack[sp as usize].level = 1 as uint8_t;
                        stack[sp as usize].len = l;
                        stack[sp as usize].str = str;
                        sp = sp.wrapping_add(1);
                    }
                    SCLASS_EXPR_OP_OR => {
                        n = (n as ::core::ffi::c_int & SCLASS_EXPR_VALUE_MASK) as uint8_t;
                        n = (n as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t;
                        if n as ::core::ffi::c_int > sp as ::core::ffi::c_int {
                            while sp as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                sp = sp.wrapping_sub(1);
                                if !stack[sp as usize].str.is_null() {
                                    free(stack[sp as usize].str as *mut ::core::ffi::c_void);
                                }
                            }
                            return 0 as uint8_t;
                        }
                        l = 0 as uint8_t;
                        r = n;
                        while r as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            if stack[(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                .level as ::core::ffi::c_int
                                > 2 as ::core::ffi::c_int
                            {
                                l = (l as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t;
                            }
                            l = (l as ::core::ffi::c_int
                                + stack
                                    [(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                    .len as ::core::ffi::c_int)
                                as uint8_t;
                            r = r.wrapping_sub(1);
                        }
                        l = (l as ::core::ffi::c_int
                            + (n as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                            as uint8_t;
                        str = malloc(l as size_t) as *mut ::core::ffi::c_char;
                        l = 0 as uint8_t;
                        r = n;
                        while r as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            if r as ::core::ffi::c_int != n as ::core::ffi::c_int {
                                let c2rust_fresh4 = l;
                                l = l.wrapping_add(1);
                                *str.offset(c2rust_fresh4 as isize) = '|' as ::core::ffi::c_char;
                            }
                            if stack[(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                .level as ::core::ffi::c_int
                                > 2 as ::core::ffi::c_int
                            {
                                let c2rust_fresh5 = l;
                                l = l.wrapping_add(1);
                                *str.offset(c2rust_fresh5 as isize) = '(' as ::core::ffi::c_char;
                            }
                            memcpy(
                                str.offset(l as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_void,
                                stack[(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                    .str
                                    as *const ::core::ffi::c_void,
                                stack[(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                    .len as size_t,
                            );
                            l = (l as ::core::ffi::c_int
                                + stack
                                    [(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                    .len as ::core::ffi::c_int)
                                as uint8_t;
                            free(
                                stack[(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                    .str
                                    as *mut ::core::ffi::c_void,
                            );
                            if stack[(sp as ::core::ffi::c_int - r as ::core::ffi::c_int) as usize]
                                .level as ::core::ffi::c_int
                                > 2 as ::core::ffi::c_int
                            {
                                let c2rust_fresh6 = l;
                                l = l.wrapping_add(1);
                                *str.offset(c2rust_fresh6 as isize) = ')' as ::core::ffi::c_char;
                            }
                            r = r.wrapping_sub(1);
                        }
                        sp = (sp as ::core::ffi::c_int - n as ::core::ffi::c_int) as uint8_t;
                        stack[sp as usize].level = 2 as uint8_t;
                        stack[sp as usize].len = l;
                        stack[sp as usize].str = str;
                        sp = sp.wrapping_add(1);
                    }
                    SCLASS_EXPR_OP_ONE => {
                        n = (n as ::core::ffi::c_int & SCLASS_EXPR_VALUE_MASK) as uint8_t;
                        if n as ::core::ffi::c_int == SCLASS_EXPR_OP_NOT {
                            if sp as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                return 0 as uint8_t;
                            }
                            l = stack
                                [(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                                .len;
                            if stack[(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                                .level as ::core::ffi::c_int
                                > 0 as ::core::ffi::c_int
                            {
                                l = (l as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t;
                            }
                            l = l.wrapping_add(1);
                            str = malloc(l as size_t) as *mut ::core::ffi::c_char;
                            l = 0 as uint8_t;
                            let c2rust_fresh7 = l;
                            l = l.wrapping_add(1);
                            *str.offset(c2rust_fresh7 as isize) = '~' as ::core::ffi::c_char;
                            if stack[(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                                .level as ::core::ffi::c_int
                                > 0 as ::core::ffi::c_int
                            {
                                let c2rust_fresh8 = l;
                                l = l.wrapping_add(1);
                                *str.offset(c2rust_fresh8 as isize) = '(' as ::core::ffi::c_char;
                            }
                            memcpy(
                                str.offset(l as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_void,
                                stack[(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                                    .str
                                    as *const ::core::ffi::c_void,
                                stack[(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                                    .len as size_t,
                            );
                            l = (l as ::core::ffi::c_int
                                + stack
                                    [(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                                    .len as ::core::ffi::c_int)
                                as uint8_t;
                            free(
                                stack[(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                                    .str
                                    as *mut ::core::ffi::c_void,
                            );
                            if stack[(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                                .level as ::core::ffi::c_int
                                > 0 as ::core::ffi::c_int
                            {
                                let c2rust_fresh9 = l;
                                l = l.wrapping_add(1);
                                *str.offset(c2rust_fresh9 as isize) = ')' as ::core::ffi::c_char;
                            }
                            stack[(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                                .level = 0 as uint8_t;
                            stack[(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                                .len = l;
                            stack[(sp as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                                .str = str;
                        }
                    }
                    _ => {}
                }
            }
            if sp as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                memcpy(
                    outstr as *mut ::core::ffi::c_void,
                    stack[0 as usize].str as *const ::core::ffi::c_void,
                    stack[0 as usize].len as size_t,
                );
                *outstr.offset(stack[0 as usize].len as isize) = 0 as ::core::ffi::c_char;
                free(stack[0 as usize].str as *mut ::core::ffi::c_void);
                return stack[0 as usize].len;
            }
            while sp as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                sp = sp.wrapping_sub(1);
                if !stack[sp as usize].str.is_null() {
                    free(stack[sp as usize].str as *mut ::core::ffi::c_void);
                }
            }
            memcpy(
                outstr as *mut ::core::ffi::c_void,
                b"ERROR\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                5 as size_t,
            );
            *outstr.offset(5 as isize) = 0 as ::core::ffi::c_char;
            return 5 as uint8_t;
        } else {
            *outstr.offset(0 as isize) = '*' as ::core::ffi::c_char;
            *outstr.offset(1 as isize) = 0 as ::core::ffi::c_char;
            return 1 as uint8_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_label_expr(
    mut strbuff: *mut ::core::ffi::c_char,
    mut pd: *const parser_data,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut i: uint8_t = 0;
        let mut j: uint8_t = 0;
        let mut c: uint8_t = 0;
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        p = strbuff;
        if (*pd).ec_data_chksum_parts != 0 {
            i = ((*pd).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                as uint8_t;
            j = ((*pd).ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                as uint8_t;
            *p = '@' as ::core::ffi::c_char;
            p = p.offset(1);
            if i as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                || i as ::core::ffi::c_int == 4 as ::core::ffi::c_int
            {
                *p = ('0' as ::core::ffi::c_int + i as ::core::ffi::c_int) as ::core::ffi::c_char;
                p = p.offset(1);
                *p = '+' as ::core::ffi::c_char;
                p = p.offset(1);
            }
            *p = ('0' as ::core::ffi::c_int + j as ::core::ffi::c_int) as ::core::ffi::c_char;
            p = p.offset(1);
        } else if (*pd).labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *p = '-' as ::core::ffi::c_char;
            p = p.offset(1);
            *p = '\0' as ::core::ffi::c_char;
            return strbuff;
        }
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < (*pd).labelscnt as ::core::ffi::c_int {
            if i as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                || (*pd).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            {
                *p = ',' as ::core::ffi::c_char;
                p = p.offset(1);
            }
            c = 1 as uint8_t;
            while (i as ::core::ffi::c_int + c as ::core::ffi::c_int)
                < (*pd).labelscnt as ::core::ffi::c_int
                && labelexpr_diff(
                    &raw const *(&raw const (*pd).labelexpr as *const [uint8_t; 128])
                        .offset(i as isize) as *const uint8_t,
                    &raw const *(&raw const (*pd).labelexpr as *const [uint8_t; 128])
                        .offset((i as ::core::ffi::c_int + c as ::core::ffi::c_int) as isize)
                        as *const uint8_t,
                ) as ::core::ffi::c_int
                    != 0
            {
                c = c.wrapping_add(1);
            }
            if c as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                *p = ('0' as ::core::ffi::c_int + c as ::core::ffi::c_int) as ::core::ffi::c_char;
                p = p.offset(1);
            }
            j = rpn_to_infix(
                &raw const *(&raw const (*pd).labelexpr as *const [uint8_t; 128]).offset(i as isize)
                    as *const uint8_t,
                p as *mut ::core::ffi::c_char,
            );
            p = p.offset(j as ::core::ffi::c_int as isize);
            i = (i as ::core::ffi::c_int + c as ::core::ffi::c_int) as uint8_t;
        }
        if (*pd).uniqmask != 0 as uint32_t {
            *p = '/' as ::core::ffi::c_char;
            p = p.offset(1);
            if (*pd).uniqmask & UNIQ_MASK_IP as uint32_t != 0 {
                memcpy(
                    p as *mut ::core::ffi::c_void,
                    b"[IP]\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    4 as size_t,
                );
                p = p.offset(4 as ::core::ffi::c_int as isize);
            } else if (*pd).uniqmask & UNIQ_MASK_RACK as uint32_t != 0 {
                memcpy(
                    p as *mut ::core::ffi::c_void,
                    b"[RACK]\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    6 as size_t,
                );
                p = p.offset(6 as ::core::ffi::c_int as isize);
            } else {
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < 26 as ::core::ffi::c_int {
                    if (*pd).uniqmask
                        & ((1 as ::core::ffi::c_int) << i as ::core::ffi::c_int) as uint32_t
                        != 0
                    {
                        if (i as ::core::ffi::c_int) < 24 as ::core::ffi::c_int
                            && (*pd).uniqmask >> i as ::core::ffi::c_int & 7 as uint32_t
                                == 7 as uint32_t
                        {
                            *p = ('A' as ::core::ffi::c_int + i as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                            p = p.offset(1);
                            *p = '-' as ::core::ffi::c_char;
                            p = p.offset(1);
                            while (*pd).uniqmask
                                & ((1 as ::core::ffi::c_int) << i as ::core::ffi::c_int) as uint32_t
                                != 0
                                && (i as ::core::ffi::c_int) < 26 as ::core::ffi::c_int
                            {
                                i = i.wrapping_add(1);
                            }
                            i = i.wrapping_sub(1);
                            *p = ('A' as ::core::ffi::c_int + i as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                            p = p.offset(1);
                        } else {
                            *p = ('A' as ::core::ffi::c_int + i as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                            p = p.offset(1);
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
        }
        if (*pd).labels_mode as ::core::ffi::c_int == LABELS_MODE_STD
            || (*pd).labels_mode as ::core::ffi::c_int == LABELS_MODE_LOOSE
            || (*pd).labels_mode as ::core::ffi::c_int == LABELS_MODE_STRICT
        {
            *p = ':' as ::core::ffi::c_char;
            p = p.offset(1);
            if (*pd).labels_mode as ::core::ffi::c_int == LABELS_MODE_STRICT {
                memcpy(
                    p as *mut ::core::ffi::c_void,
                    b"STRICT\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    6 as size_t,
                );
                p = p.offset(6 as ::core::ffi::c_int as isize);
            } else if (*pd).labels_mode as ::core::ffi::c_int == LABELS_MODE_LOOSE {
                memcpy(
                    p as *mut ::core::ffi::c_void,
                    b"LOOSE\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    5 as size_t,
                );
                p = p.offset(5 as ::core::ffi::c_int as isize);
            } else {
                memcpy(
                    p as *mut ::core::ffi::c_void,
                    b"STD\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    3 as size_t,
                );
                p = p.offset(3 as ::core::ffi::c_int as isize);
            }
        }
        *p = '\0' as ::core::ffi::c_char;
        return strbuff;
    }
}
#[inline]
unsafe extern "C" fn expr_rfree(mut actnode: *mut node) {
    unsafe {
        if !actnode.is_null() {
            if (*actnode).op as ::core::ffi::c_int != REF as ::core::ffi::c_int {
                expr_rfree((*actnode).arg1 as *mut node);
                expr_rfree((*actnode).arg2 as *mut node);
            }
            free(actnode as *mut ::core::ffi::c_void);
        }
    }
}
#[inline]
unsafe extern "C" fn newnode(
    mut op: uint8_t,
    mut val: int8_t,
    mut arg1: *mut node,
    mut arg2: *mut node,
) -> *mut node {
    unsafe {
        let mut aux: *mut node = ::core::ptr::null_mut::<node>();
        aux = malloc(::core::mem::size_of::<node>()) as *mut node;
        (*aux).op = op;
        (*aux).val = val as uint8_t;
        (*aux).arg1 = arg1 as *mut _node;
        (*aux).arg2 = arg2 as *mut _node;
        return aux;
    }
}
#[inline]
unsafe extern "C" fn expr_parse_error(mut e: *mut expr, mut extramsg: *const ::core::ffi::c_char) {
    unsafe {
        if *(*e).str.offset(0 as isize) as int8_t as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
        {
            printf(
                b"parse error, %snext char: '%c'\n\0".as_ptr() as *const ::core::ffi::c_char,
                extramsg,
                *(*e).str.offset(0 as isize) as ::core::ffi::c_int,
            );
        } else {
            printf(
                b"parse error, %snext code: 0x%02hhX\n\0".as_ptr() as *const ::core::ffi::c_char,
                extramsg,
                *(*e).str.offset(0 as isize) as uint8_t as ::core::ffi::c_int,
            );
        }
        (*e).erroroccured = 1 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn expr_eat_white(mut e: *mut expr) {
    unsafe {
        while *(*e).str.offset(0 as isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            (*e).str = (*e).str.offset(1);
        }
    }
}
#[inline]
unsafe extern "C" fn expr_sym(mut e: *mut expr) -> *mut node {
    unsafe {
        let mut a: *mut node = ::core::ptr::null_mut::<node>();
        let mut v: uint8_t = 0;
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '(' as ::core::ffi::c_int {
            (*e).str = (*e).str.offset(1);
            expr_eat_white(e);
            a = expr_or(e);
            expr_eat_white(e);
            if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == ')' as ::core::ffi::c_int {
                (*e).str = (*e).str.offset(1);
                return a;
            } else {
                expr_rfree(a);
                expr_parse_error(
                    e,
                    b"closing round bracket expected, \0".as_ptr() as *const ::core::ffi::c_char,
                );
                return ::core::ptr::null_mut::<node>();
            }
        }
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
            (*e).str = (*e).str.offset(1);
            expr_eat_white(e);
            a = expr_or(e);
            expr_eat_white(e);
            if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == ']' as ::core::ffi::c_int {
                (*e).str = (*e).str.offset(1);
                return a;
            } else {
                expr_rfree(a);
                expr_parse_error(
                    e,
                    b"closing square bracket expected, \0".as_ptr() as *const ::core::ffi::c_char,
                );
                return ::core::ptr::null_mut::<node>();
            }
        }
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int {
            (*e).str = (*e).str.offset(1);
            return newnode(
                ANY as ::core::ffi::c_int as uint8_t,
                0 as int8_t,
                ::core::ptr::null_mut::<node>(),
                ::core::ptr::null_mut::<node>(),
            );
        }
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '!' as ::core::ffi::c_int
            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '~' as ::core::ffi::c_int
            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
        {
            (*e).str = (*e).str.offset(1);
            expr_eat_white(e);
            a = expr_sym(e);
            expr_eat_white(e);
            return newnode(
                NOT as ::core::ffi::c_int as uint8_t,
                0 as int8_t,
                a,
                ::core::ptr::null_mut::<node>(),
            );
        }
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
            && *(*e).str.offset(0 as isize) as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
        {
            v = (*(*e).str.offset(0 as isize) as ::core::ffi::c_int - 'A' as ::core::ffi::c_int)
                as uint8_t;
            (*e).str = (*e).str.offset(1);
            return newnode(
                SYM as ::core::ffi::c_int as uint8_t,
                v as int8_t,
                ::core::ptr::null_mut::<node>(),
                ::core::ptr::null_mut::<node>(),
            );
        }
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
            && *(*e).str.offset(0 as isize) as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
        {
            v = (*(*e).str.offset(0 as isize) as ::core::ffi::c_int - 'a' as ::core::ffi::c_int)
                as uint8_t;
            (*e).str = (*e).str.offset(1);
            return newnode(
                SYM as ::core::ffi::c_int as uint8_t,
                v as int8_t,
                ::core::ptr::null_mut::<node>(),
                ::core::ptr::null_mut::<node>(),
            );
        }
        expr_parse_error(e, b"\0".as_ptr() as *const ::core::ffi::c_char);
        return ::core::ptr::null_mut::<node>();
    }
}
#[inline]
unsafe extern "C" fn expr_and(mut e: *mut expr) -> *mut node {
    unsafe {
        let mut a: *mut node = ::core::ptr::null_mut::<node>();
        let mut b: *mut node = ::core::ptr::null_mut::<node>();
        expr_eat_white(e);
        a = expr_sym(e);
        expr_eat_white(e);
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '&' as ::core::ffi::c_int
            && *(*e).str.offset(1 as isize) as ::core::ffi::c_int == '&' as ::core::ffi::c_int
        {
            (*e).str = (*e).str.offset(2 as ::core::ffi::c_int as isize);
            b = expr_and(e);
            return newnode(AND as ::core::ffi::c_int as uint8_t, 0 as int8_t, a, b);
        } else if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '&' as ::core::ffi::c_int
            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int
        {
            (*e).str = (*e).str.offset(1);
            b = expr_and(e);
            return newnode(AND as ::core::ffi::c_int as uint8_t, 0 as int8_t, a, b);
        } else if *(*e).str.offset(0 as isize) as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
            && *(*e).str.offset(0 as isize) as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                && *(*e).str.offset(0 as isize) as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '(' as ::core::ffi::c_int
            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '[' as ::core::ffi::c_int
        {
            b = expr_and(e);
            return newnode(AND as ::core::ffi::c_int as uint8_t, 0 as int8_t, a, b);
        } else {
            return a;
        };
    }
}
#[inline]
unsafe extern "C" fn expr_or(mut e: *mut expr) -> *mut node {
    unsafe {
        let mut a: *mut node = ::core::ptr::null_mut::<node>();
        let mut b: *mut node = ::core::ptr::null_mut::<node>();
        expr_eat_white(e);
        a = expr_and(e);
        expr_eat_white(e);
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '|' as ::core::ffi::c_int
            && *(*e).str.offset(1 as isize) as ::core::ffi::c_int == '|' as ::core::ffi::c_int
        {
            (*e).str = (*e).str.offset(2 as ::core::ffi::c_int as isize);
            b = expr_or(e);
            return newnode(OR as ::core::ffi::c_int as uint8_t, 0 as int8_t, a, b);
        } else if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '|' as ::core::ffi::c_int
            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '+' as ::core::ffi::c_int
        {
            (*e).str = (*e).str.offset(1);
            b = expr_or(e);
            return newnode(OR as ::core::ffi::c_int as uint8_t, 0 as int8_t, a, b);
        } else {
            return a;
        };
    }
}
#[inline]
unsafe extern "C" fn expr_uniqmask(mut e: *mut expr) {
    unsafe {
        let mut last: uint8_t = 0;
        let mut range: uint8_t = 0;
        let mut current: uint8_t = 0;
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
            (*e).str = (*e).str.offset(1);
            if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 'I' as ::core::ffi::c_int
                || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 'i' as ::core::ffi::c_int
            {
                (*e).str = (*e).str.offset(1);
                if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 'P' as ::core::ffi::c_int
                    || *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                        == 'p' as ::core::ffi::c_int
                {
                    (*e).str = (*e).str.offset(1);
                }
                if *(*e).str.offset(0 as isize) as ::core::ffi::c_int != ']' as ::core::ffi::c_int {
                    expr_parse_error(e, b"\0".as_ptr() as *const ::core::ffi::c_char);
                }
                (*e).str = (*e).str.offset(1);
                (*e).uniqmask = UNIQ_MASK_IP as uint32_t;
            } else if *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                == 'R' as ::core::ffi::c_int
                || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 'r' as ::core::ffi::c_int
            {
                (*e).str = (*e).str.offset(1);
                if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 'A' as ::core::ffi::c_int
                    || *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                        == 'a' as ::core::ffi::c_int
                {
                    (*e).str = (*e).str.offset(1);
                    if *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                        == 'C' as ::core::ffi::c_int
                        || *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                            == 'c' as ::core::ffi::c_int
                    {
                        (*e).str = (*e).str.offset(1);
                        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                            == 'K' as ::core::ffi::c_int
                            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                                == 'k' as ::core::ffi::c_int
                        {
                            (*e).str = (*e).str.offset(1);
                        }
                    }
                }
                if *(*e).str.offset(0 as isize) as ::core::ffi::c_int != ']' as ::core::ffi::c_int {
                    expr_parse_error(e, b"\0".as_ptr() as *const ::core::ffi::c_char);
                }
                (*e).str = (*e).str.offset(1);
                (*e).uniqmask = UNIQ_MASK_RACK as uint32_t;
            } else {
                expr_parse_error(e, b"\0".as_ptr() as *const ::core::ffi::c_char);
            }
            return;
        }
        last = 0xff as uint8_t;
        range = 0 as uint8_t;
        loop {
            expr_eat_white(e);
            current = 0xff as uint8_t;
            if *(*e).str.offset(0 as isize) as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && *(*e).str.offset(0 as isize) as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            {
                current = (*(*e).str.offset(0 as isize) as ::core::ffi::c_int
                    - 'A' as ::core::ffi::c_int) as uint8_t;
                (*e).str = (*e).str.offset(1);
            } else if *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                >= 'a' as ::core::ffi::c_int
                && *(*e).str.offset(0 as isize) as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
            {
                current = (*(*e).str.offset(0 as isize) as ::core::ffi::c_int
                    - 'a' as ::core::ffi::c_int) as uint8_t;
                (*e).str = (*e).str.offset(1);
            } else if *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                == '-' as ::core::ffi::c_int
                && range as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && last as ::core::ffi::c_int != 0xff as ::core::ffi::c_int
            {
                (*e).str = (*e).str.offset(1);
                range = 1 as uint8_t;
            } else {
                if range as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    printf(b"parse error, expected character after '-'\n\0".as_ptr()
                        as *const ::core::ffi::c_char);
                    (*e).erroroccured = 1 as uint8_t;
                }
                return;
            }
            if range as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                && current as ::core::ffi::c_int != 0xff as ::core::ffi::c_int
                && last as ::core::ffi::c_int != 0xff as ::core::ffi::c_int
            {
                if (current as ::core::ffi::c_int) < last as ::core::ffi::c_int {
                    while current as ::core::ffi::c_int <= last as ::core::ffi::c_int {
                        (*e).uniqmask |= ((1 as ::core::ffi::c_int)
                            << current as ::core::ffi::c_int)
                            as uint32_t;
                        current = current.wrapping_add(1);
                    }
                } else {
                    while last as ::core::ffi::c_int <= current as ::core::ffi::c_int {
                        (*e).uniqmask |=
                            ((1 as ::core::ffi::c_int) << last as ::core::ffi::c_int) as uint32_t;
                        last = last.wrapping_add(1);
                    }
                }
                last = 0xff as uint8_t;
                range = 0 as uint8_t;
            } else if current as ::core::ffi::c_int != 0xff as ::core::ffi::c_int {
                (*e).uniqmask |=
                    ((1 as ::core::ffi::c_int) << current as ::core::ffi::c_int) as uint32_t;
                last = current;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn expr_mode(mut e: *mut expr) {
    unsafe {
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 's' as ::core::ffi::c_int
        {
            if memcmp(
                (*e).str as *const ::core::ffi::c_void,
                b"STD\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                3 as size_t,
            ) == 0 as ::core::ffi::c_int
                || memcmp(
                    (*e).str.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                    b"td\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    2 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                (*e).labels_mode = LABELS_MODE_STD as uint8_t;
                (*e).str = (*e).str.offset(3 as ::core::ffi::c_int as isize);
                return;
            }
            if memcmp(
                (*e).str as *const ::core::ffi::c_void,
                b"STRICT\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                6 as size_t,
            ) == 0 as ::core::ffi::c_int
                || memcmp(
                    (*e).str.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                    b"trict\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                (*e).labels_mode = LABELS_MODE_STRICT as uint8_t;
                (*e).str = (*e).str.offset(6 as ::core::ffi::c_int as isize);
                return;
            }
            (*e).labels_mode = LABELS_MODE_STRICT as uint8_t;
            (*e).str = (*e).str.offset(1);
            return;
        }
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 'L' as ::core::ffi::c_int
            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 'l' as ::core::ffi::c_int
        {
            if memcmp(
                (*e).str as *const ::core::ffi::c_void,
                b"LOOSE\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                5 as size_t,
            ) == 0 as ::core::ffi::c_int
                || memcmp(
                    (*e).str.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                    b"oose\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    4 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                (*e).labels_mode = LABELS_MODE_LOOSE as uint8_t;
                (*e).str = (*e).str.offset(5 as ::core::ffi::c_int as isize);
                return;
            }
            (*e).labels_mode = LABELS_MODE_LOOSE as uint8_t;
            (*e).str = (*e).str.offset(1);
            return;
        }
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 'D' as ::core::ffi::c_int
            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 'd' as ::core::ffi::c_int
        {
            (*e).labels_mode = LABELS_MODE_STD as uint8_t;
            (*e).str = (*e).str.offset(1);
            return;
        }
    }
}
#[inline]
unsafe extern "C" fn expr_ending(mut e: *mut expr) {
    unsafe {
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return;
        }
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            (*e).str = (*e).str.offset(1);
            expr_uniqmask(e);
        }
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
            (*e).str = (*e).str.offset(1);
            expr_mode(e);
        }
        if *(*e).str.offset(0 as isize) != 0 {
            expr_parse_error(e, b"\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
}
#[inline]
unsafe extern "C" fn expr_top(mut e: *mut expr) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut g: uint32_t = 0;
        let mut f: uint8_t = 0;
        let mut a: *mut node = ::core::ptr::null_mut::<node>();
        expr_eat_white(e);
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
            let mut p: *const ::core::ffi::c_char = (*e).str;
            (*e).str = (*e).str.offset(1);
            expr_eat_white(e);
            if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return;
            }
            (*e).str = p;
        }
        if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '@' as ::core::ffi::c_int
            || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '=' as ::core::ffi::c_int
        {
            (*e).str = (*e).str.offset(1);
            expr_eat_white(e);
            if *(*e).str.offset(0 as isize) as ::core::ffi::c_int >= '1' as ::core::ffi::c_int
                && *(*e).str.offset(0 as isize) as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                (*e).ec_data_chksum_parts = (*(*e).str.offset(0 as isize) as ::core::ffi::c_int
                    - '0' as ::core::ffi::c_int)
                    as uint8_t;
                (*e).str = (*e).str.offset(1);
            } else {
                printf(
                    b"parse error, in ec mode expected number of checksums or data parts after '@'\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                (*e).erroroccured = 1 as uint8_t;
                return;
            }
            expr_eat_white(e);
            if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == '+' as ::core::ffi::c_int {
                (*e).str = (*e).str.offset(1);
                expr_eat_white(e);
                if *(*e).str.offset(0 as isize) as ::core::ffi::c_int >= '1' as ::core::ffi::c_int
                    && *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                        <= '9' as ::core::ffi::c_int
                    && ((*e).ec_data_chksum_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                        || (*e).ec_data_chksum_parts as ::core::ffi::c_int
                            == 8 as ::core::ffi::c_int)
                {
                    (*e).ec_data_chksum_parts = (((*e).ec_data_chksum_parts as ::core::ffi::c_int)
                        << 4 as ::core::ffi::c_int)
                        as uint8_t;
                    (*e).ec_data_chksum_parts = ((*e).ec_data_chksum_parts as ::core::ffi::c_int
                        | *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                            - '0' as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int)
                        as uint8_t;
                    (*e).str = (*e).str.offset(1);
                    expr_eat_white(e);
                } else {
                    printf(
                        b"parse error, in ec mode expected number of checksums after '+' and data parts be set to '4' or '8'\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    (*e).erroroccured = 1 as uint8_t;
                    return;
                }
            }
            if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                || *(*e).str.offset(0 as isize) as ::core::ffi::c_int == ';' as ::core::ffi::c_int
            {
                (*e).str = (*e).str.offset(1);
                expr_eat_white(e);
                (*e).terms[0 as usize] = expr_or(e);
                expr_eat_white(e);
                if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                    || *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                        == ';' as ::core::ffi::c_int
                {
                    (*e).str = (*e).str.offset(1);
                    expr_eat_white(e);
                    (*e).terms[1 as usize] = expr_or(e);
                    expr_eat_white(e);
                }
            } else {
                (*e).terms[0 as usize] = newnode(
                    ANY as ::core::ffi::c_int as uint8_t,
                    0 as int8_t,
                    ::core::ptr::null_mut::<node>(),
                    ::core::ptr::null_mut::<node>(),
                );
            }
            expr_ending(e);
        } else {
            (*e).ec_data_chksum_parts = 0 as uint8_t;
            i = 0 as uint32_t;
            while i < 9 as uint32_t {
                expr_eat_white(e);
                f = 0 as uint8_t;
                if *(*e).str.offset(0 as isize) as ::core::ffi::c_int >= '1' as ::core::ffi::c_int
                    && *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                        <= '9' as ::core::ffi::c_int
                {
                    g = (*(*e).str.offset(0 as isize) as ::core::ffi::c_int
                        - '0' as ::core::ffi::c_int) as uint32_t;
                    (*e).str = (*e).str.offset(1);
                    f = 1 as uint8_t;
                } else {
                    g = 1 as uint32_t;
                }
                expr_eat_white(e);
                if i == 0 as uint32_t
                    && f as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                    && (*(*e).str.offset(0 as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        || *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                            == '/' as ::core::ffi::c_int)
                {
                    a = newnode(
                        ANY as ::core::ffi::c_int as uint8_t,
                        0 as int8_t,
                        ::core::ptr::null_mut::<node>(),
                        ::core::ptr::null_mut::<node>(),
                    );
                } else {
                    a = expr_or(e);
                }
                expr_eat_white(e);
                if (*e).erroroccured != 0 {
                    expr_rfree(a);
                    return;
                }
                if i.wrapping_add(g) > 9 as uint32_t {
                    break;
                }
                f = 0 as uint8_t;
                while g > 0 as uint32_t {
                    if f as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                        (*e).terms[i as usize] = newnode(
                            REF as ::core::ffi::c_int as uint8_t,
                            0 as int8_t,
                            a,
                            ::core::ptr::null_mut::<node>(),
                        );
                    } else {
                        (*e).terms[i as usize] = a;
                        f = 1 as uint8_t;
                    }
                    i = i.wrapping_add(1);
                    g = g.wrapping_sub(1);
                }
                if *(*e).str.offset(0 as isize) as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                    || *(*e).str.offset(0 as isize) as ::core::ffi::c_int
                        == ';' as ::core::ffi::c_int
                {
                    (*e).str = (*e).str.offset(1);
                } else if *(*e).str.offset(0 as isize) != 0 {
                    expr_ending(e);
                    return;
                } else {
                    return;
                }
            }
            printf(b"parse error, too many copies\n\0".as_ptr() as *const ::core::ffi::c_char);
            (*e).erroroccured = 1 as uint8_t;
        };
    }
}
#[inline]
unsafe extern "C" fn expr_rpn_safe_add(mut o: *mut rpnbuff, mut d: uint8_t) {
    unsafe {
        if ((*o).pos as ::core::ffi::c_int) < SCLASS_EXPR_MAX_SIZE {
            let c2rust_fresh10 = (*o).pos;
            (*o).pos = (*o).pos.wrapping_add(1);
            (*o).rpndata[c2rust_fresh10 as usize] = d;
        }
    }
}
#[inline]
unsafe extern "C" fn expr_rpn_top(mut o: *mut rpnbuff) -> uint8_t {
    unsafe {
        if (*o).pos as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            return (*o).rpndata
                [((*o).pos as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize];
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn expr_rpn_exchg_top(mut o: *mut rpnbuff, mut d: uint8_t) {
    unsafe {
        if (*o).pos as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            (*o).rpndata[((*o).pos as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = d;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn expr_convert_to_rpn(mut n: *mut node, mut o: *mut rpnbuff) {
    unsafe {
        let mut t: uint8_t = 0;
        match (*n).op as ::core::ffi::c_int {
            3 => {
                expr_convert_to_rpn((*n).arg1 as *mut node, o);
                return;
            }
            5 => {
                expr_rpn_safe_add(
                    o,
                    (SCLASS_EXPR_SYMBOL | (*n).val as ::core::ffi::c_int) as uint8_t,
                );
                return;
            }
            0 => {
                expr_convert_to_rpn((*n).arg1 as *mut node, o);
                expr_convert_to_rpn((*n).arg2 as *mut node, o);
                t = expr_rpn_top(o);
                if t as ::core::ffi::c_int & SCLASS_EXPR_TYPE_MASK == SCLASS_EXPR_OP_OR
                    && t as ::core::ffi::c_int & SCLASS_EXPR_VALUE_MASK < SCLASS_EXPR_VALUE_MASK
                {
                    expr_rpn_exchg_top(
                        o,
                        (t as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
                    );
                } else {
                    expr_rpn_safe_add(o, SCLASS_EXPR_OP_OR as uint8_t);
                }
                return;
            }
            1 => {
                expr_convert_to_rpn((*n).arg1 as *mut node, o);
                expr_convert_to_rpn((*n).arg2 as *mut node, o);
                t = expr_rpn_top(o);
                if t as ::core::ffi::c_int & SCLASS_EXPR_TYPE_MASK == SCLASS_EXPR_OP_AND
                    && t as ::core::ffi::c_int & SCLASS_EXPR_VALUE_MASK < SCLASS_EXPR_VALUE_MASK
                {
                    expr_rpn_exchg_top(
                        o,
                        (t as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
                    );
                } else {
                    expr_rpn_safe_add(o, SCLASS_EXPR_OP_AND as uint8_t);
                }
                return;
            }
            2 => {
                expr_convert_to_rpn((*n).arg1 as *mut node, o);
                expr_rpn_safe_add(o, SCLASS_EXPR_OP_NOT as uint8_t);
                return;
            }
            4 => {
                expr_rpn_safe_add(o, SCLASS_EXPR_SYMBOL_ANY as uint8_t);
                return;
            }
            _ => {}
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_label_expr(
    mut exprstr: *const ::core::ffi::c_char,
    mut pd: *mut parser_data,
) -> ::core::ffi::c_int {
    unsafe {
        let mut e: expr = expr {
            str: ::core::ptr::null::<::core::ffi::c_char>(),
            uniqmask: 0,
            labels_mode: 0,
            terms: [::core::ptr::null_mut::<node>(); 9],
            erroroccured: 0,
            ec_data_chksum_parts: 0,
        };
        let mut rpn: rpnbuff = rpnbuff {
            pos: 0,
            rpndata: [0; 128],
        };
        let mut i: uint32_t = 0;
        let mut res: ::core::ffi::c_int = 0;
        res = 0 as ::core::ffi::c_int;
        memset(
            pd as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<parser_data>(),
        );
        memset(
            &raw mut e as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<expr>(),
        );
        e.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
        e.str = exprstr;
        if !(*exprstr.offset(1 as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            && (*exprstr.offset(0 as isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
                || *exprstr.offset(0 as isize) as ::core::ffi::c_int == '~' as ::core::ffi::c_int))
        {
            expr_top(&raw mut e);
            if e.erroroccured != 0 {
                res = -1 as ::core::ffi::c_int;
            }
            i = 0 as uint32_t;
            while i < 9 as uint32_t
                && res == 0 as ::core::ffi::c_int
                && !e.terms[i as usize].is_null()
            {
                rpn.pos = 0 as uint8_t;
                expr_convert_to_rpn(e.terms[i as usize], &raw mut rpn);
                if rpn.pos as ::core::ffi::c_int == SCLASS_EXPR_MAX_SIZE {
                    printf(b"parse error, too many terms in expression\n\0".as_ptr()
                        as *const ::core::ffi::c_char);
                    res = -1 as ::core::ffi::c_int;
                } else {
                    memset(
                        &raw mut *(&raw mut (*pd).labelexpr as *mut [uint8_t; 128])
                            .offset(i as isize) as *mut uint8_t
                            as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        SCLASS_EXPR_MAX_SIZE as size_t,
                    );
                    memcpy(
                        &raw mut *(&raw mut (*pd).labelexpr as *mut [uint8_t; 128])
                            .offset(i as isize) as *mut uint8_t
                            as *mut ::core::ffi::c_void,
                        &raw mut rpn.rpndata as *mut uint8_t as *const ::core::ffi::c_void,
                        rpn.pos as size_t,
                    );
                }
                i = i.wrapping_add(1);
            }
            if res == 0 as ::core::ffi::c_int {
                (*pd).labelscnt = i as uint8_t;
                (*pd).uniqmask = e.uniqmask;
                (*pd).labels_mode = e.labels_mode;
                (*pd).ec_data_chksum_parts = e.ec_data_chksum_parts;
            }
            i = 0 as uint32_t;
            while i < 9 as uint32_t {
                expr_rfree(e.terms[i as usize]);
                i = i.wrapping_add(1);
            }
        }
        return res;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn labelmask_matches_labelexpr(
    mut labelmask: uint32_t,
    mut labelexpr: *const uint8_t,
) -> uint8_t {
    unsafe {
        static mut stack: [uint8_t; 128] = [0; 128];
        let mut n: uint8_t = 0;
        let mut r: uint8_t = 0;
        let mut sp: uint8_t = 0;
        sp = 0 as uint8_t;
        if *labelexpr != 0 {
            loop {
                let c2rust_fresh11 = labelexpr;
                labelexpr = labelexpr.offset(1);
                n = *c2rust_fresh11;
                if n == 0 {
                    break;
                }
                match n as ::core::ffi::c_int & SCLASS_EXPR_TYPE_MASK {
                    SCLASS_EXPR_SYMBOL => {
                        if n as ::core::ffi::c_int == SCLASS_EXPR_SYMBOL_ANY {
                            let c2rust_fresh12 = sp;
                            sp = sp.wrapping_add(1);
                            stack[c2rust_fresh12 as usize] = 1 as uint8_t;
                        } else {
                            n = (n as ::core::ffi::c_int & SCLASS_EXPR_VALUE_MASK) as uint8_t;
                            if labelmask
                                & ((1 as ::core::ffi::c_int) << n as ::core::ffi::c_int) as uint32_t
                                != 0
                            {
                                let c2rust_fresh13 = sp;
                                sp = sp.wrapping_add(1);
                                stack[c2rust_fresh13 as usize] = 1 as uint8_t;
                            } else {
                                let c2rust_fresh14 = sp;
                                sp = sp.wrapping_add(1);
                                stack[c2rust_fresh14 as usize] = 0 as uint8_t;
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
                        let c2rust_fresh15 = sp;
                        sp = sp.wrapping_add(1);
                        stack[c2rust_fresh15 as usize] = r;
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
                        let c2rust_fresh16 = sp;
                        sp = sp.wrapping_add(1);
                        stack[c2rust_fresh16 as usize] = r;
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
