extern "C" {
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __glibc_reserved: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2Rust_Unnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2Rust_Unnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2Rust_Unnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2Rust_Unnamed = 0;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _statsnode {
    pub counter: uint64_t,
    pub printflag: uint8_t,
    pub absolute: uint8_t,
    pub name: *mut ::core::ffi::c_char,
    pub fullname: *mut ::core::ffi::c_char,
    pub nleng: uint32_t,
    pub fnleng: uint32_t,
    pub parent: *mut _statsnode,
    pub firstchild: *mut _statsnode,
    pub nextsibling: *mut _statsnode,
}
pub type statsnode = _statsnode;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut firstnode: *mut statsnode = ::core::ptr::null_mut::<statsnode>();
static mut allactiveplengs: uint32_t = 0 as uint32_t;
static mut activenodes: uint32_t = 0 as uint32_t;
static mut glock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
#[no_mangle]
pub unsafe extern "C" fn stats_counter_add(
    mut node: *mut ::core::ffi::c_void,
    mut delta: uint64_t,
) {
    let mut sn: *mut statsnode = node as *mut statsnode;
    pthread_mutex_lock(&raw mut glock);
    while !sn.is_null() {
        (*sn).counter = (*sn).counter.wrapping_add(delta);
        if (*sn).absolute != 0 {
            break;
        }
        sn = (*sn).parent as *mut statsnode;
    }
    pthread_mutex_unlock(&raw mut glock);
}
#[no_mangle]
pub unsafe extern "C" fn stats_counter_sub(
    mut node: *mut ::core::ffi::c_void,
    mut delta: uint64_t,
) {
    let mut sn: *mut statsnode = node as *mut statsnode;
    pthread_mutex_lock(&raw mut glock);
    while !sn.is_null() {
        (*sn).counter = (*sn).counter.wrapping_sub(delta);
        if (*sn).absolute != 0 {
            break;
        }
        sn = (*sn).parent as *mut statsnode;
    }
    pthread_mutex_unlock(&raw mut glock);
}
#[no_mangle]
pub unsafe extern "C" fn stats_counter_inc(mut node: *mut ::core::ffi::c_void) {
    let mut sn: *mut statsnode = node as *mut statsnode;
    pthread_mutex_lock(&raw mut glock);
    while !sn.is_null() {
        (*sn).counter = (*sn).counter.wrapping_add(1);
        if (*sn).absolute != 0 {
            break;
        }
        sn = (*sn).parent as *mut statsnode;
    }
    pthread_mutex_unlock(&raw mut glock);
}
#[no_mangle]
pub unsafe extern "C" fn stats_counter_dec(mut node: *mut ::core::ffi::c_void) {
    let mut sn: *mut statsnode = node as *mut statsnode;
    pthread_mutex_lock(&raw mut glock);
    while !sn.is_null() {
        (*sn).counter = (*sn).counter.wrapping_sub(1);
        if (*sn).absolute != 0 {
            break;
        }
        sn = (*sn).parent as *mut statsnode;
    }
    pthread_mutex_unlock(&raw mut glock);
}
#[no_mangle]
pub unsafe extern "C" fn stats_counter_set(
    mut node: *mut ::core::ffi::c_void,
    mut value: uint64_t,
) {
    let mut sn: *mut statsnode = node as *mut statsnode;
    pthread_mutex_lock(&raw mut glock);
    if (*sn).absolute != 0 {
        (*sn).counter = value;
    }
    pthread_mutex_unlock(&raw mut glock);
}
#[no_mangle]
pub unsafe extern "C" fn stats_get_subnode(
    mut node: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut absolute: uint8_t,
    mut printflag: uint8_t,
) -> *mut ::core::ffi::c_void {
    let mut sn: *mut statsnode = node as *mut statsnode;
    let mut a: *mut statsnode = ::core::ptr::null_mut::<statsnode>();
    pthread_mutex_lock(&raw mut glock);
    a = (if !sn.is_null() {
        (*sn).firstchild
    } else {
        firstnode as *mut _statsnode
    }) as *mut statsnode;
    while !a.is_null() {
        if strcmp((*a).name, name) == 0 as ::core::ffi::c_int {
            pthread_mutex_unlock(&raw mut glock);
            return a as *mut ::core::ffi::c_void;
        }
        a = (*a).nextsibling as *mut statsnode;
    }
    a = malloc(::core::mem::size_of::<statsnode>()) as *mut statsnode;
    (*a).nextsibling = if !sn.is_null() {
        (*sn).firstchild
    } else {
        firstnode as *mut _statsnode
    };
    (*a).firstchild = ::core::ptr::null_mut::<_statsnode>();
    (*a).counter = 0 as uint64_t;
    (*a).printflag = printflag;
    (*a).absolute = absolute;
    (*a).name = strdup(name);
    (*a).nleng = strlen(name) as uint32_t;
    if !sn.is_null() {
        let mut bstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*a).fnleng = (*sn)
            .fnleng
            .wrapping_add(1 as uint32_t)
            .wrapping_add((*a).nleng);
        bstr =
            malloc((*a).fnleng.wrapping_add(1 as uint32_t) as size_t) as *mut ::core::ffi::c_char;
        memcpy(
            bstr as *mut ::core::ffi::c_void,
            (*sn).fullname as *const ::core::ffi::c_void,
            (*sn).fnleng as size_t,
        );
        *bstr.offset((*sn).fnleng as isize) = '.' as ::core::ffi::c_char;
        memcpy(
            bstr.offset((*sn).fnleng as isize)
                .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            (*a).name as *const ::core::ffi::c_void,
            (*a).nleng as size_t,
        );
        *bstr.offset((*a).fnleng as isize) = 0 as ::core::ffi::c_char;
        (*a).fullname = bstr;
    } else {
        (*a).fullname = (*a).name;
        (*a).fnleng = (*a).nleng;
    }
    if !sn.is_null() {
        (*sn).firstchild = a as *mut _statsnode;
    } else {
        firstnode = a;
    }
    (*a).parent = sn as *mut _statsnode;
    if printflag != 0 {
        activenodes = activenodes.wrapping_add(1);
        allactiveplengs = allactiveplengs.wrapping_add((*a).fnleng);
    }
    pthread_mutex_unlock(&raw mut glock);
    return a as *mut ::core::ffi::c_void;
}
#[inline]
unsafe extern "C" fn stats_reset(mut n: *mut statsnode) {
    let mut a: *mut statsnode = ::core::ptr::null_mut::<statsnode>();
    if (*n).absolute as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*n).counter = 0 as uint64_t;
    }
    a = (*n).firstchild as *mut statsnode;
    while !a.is_null() {
        stats_reset(a);
        a = (*a).nextsibling as *mut statsnode;
    }
}
#[no_mangle]
pub unsafe extern "C" fn stats_reset_all() {
    let mut a: *mut statsnode = ::core::ptr::null_mut::<statsnode>();
    pthread_mutex_lock(&raw mut glock);
    a = firstnode;
    while !a.is_null() {
        stats_reset(a);
        a = (*a).nextsibling as *mut statsnode;
    }
    pthread_mutex_unlock(&raw mut glock);
}
#[inline]
unsafe extern "C" fn stats_print_values(
    mut buff: *mut ::core::ffi::c_char,
    mut maxleng: uint32_t,
    mut n: *mut statsnode,
) -> uint32_t {
    let mut a: *mut statsnode = ::core::ptr::null_mut::<statsnode>();
    let mut l: uint32_t = 0;
    if (*n).printflag != 0 {
        if (*n).absolute != 0 {
            l = snprintf(
                buff,
                maxleng as size_t,
                b"%s: [%lu]\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*n).fullname,
                (*n).counter,
            ) as uint32_t;
        } else {
            l = snprintf(
                buff,
                maxleng as size_t,
                b"%s: %lu\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*n).fullname,
                (*n).counter,
            ) as uint32_t;
        }
    } else {
        l = 0 as uint32_t;
    }
    a = (*n).firstchild as *mut statsnode;
    while !a.is_null() {
        if maxleng > l {
            l = l.wrapping_add(stats_print_values(
                buff.offset(l as isize),
                maxleng.wrapping_sub(l),
                a,
            ));
        }
        a = (*a).nextsibling as *mut statsnode;
    }
    return l;
}
#[inline]
unsafe extern "C" fn stats_print_total(
    mut buff: *mut ::core::ffi::c_char,
    mut maxleng: uint32_t,
) -> uint32_t {
    let mut a: *mut statsnode = ::core::ptr::null_mut::<statsnode>();
    let mut l: uint32_t = 0;
    l = 0 as uint32_t;
    a = firstnode;
    while !a.is_null() {
        if maxleng > l {
            l = l.wrapping_add(stats_print_values(
                buff.offset(l as isize),
                maxleng.wrapping_sub(l),
                a,
            ));
        }
        a = (*a).nextsibling as *mut statsnode;
    }
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn stats_show_all(
    mut buff: *mut *mut ::core::ffi::c_char,
    mut leng: *mut uint32_t,
) {
    let mut rl: uint32_t = 0;
    pthread_mutex_lock(&raw mut glock);
    rl = allactiveplengs.wrapping_add((50 as uint32_t).wrapping_mul(activenodes));
    *buff = malloc(rl as size_t) as *mut ::core::ffi::c_char;
    if !(*buff).is_null() {
        *leng = stats_print_total(*buff, rl);
    } else {
        *leng = 0 as uint32_t;
    }
    pthread_mutex_unlock(&raw mut glock);
}
#[no_mangle]
pub unsafe extern "C" fn stats_free(mut n: *mut statsnode) {
    let mut a: *mut statsnode = ::core::ptr::null_mut::<statsnode>();
    let mut an: *mut statsnode = ::core::ptr::null_mut::<statsnode>();
    free((*n).name as *mut ::core::ffi::c_void);
    if (*n).fullname != (*n).name {
        free((*n).fullname as *mut ::core::ffi::c_void);
    }
    a = (*n).firstchild as *mut statsnode;
    while !a.is_null() {
        an = (*a).nextsibling as *mut statsnode;
        stats_free(a);
        free(a as *mut ::core::ffi::c_void);
        a = an;
    }
}
#[no_mangle]
pub unsafe extern "C" fn stats_term() {
    let mut a: *mut statsnode = ::core::ptr::null_mut::<statsnode>();
    let mut an: *mut statsnode = ::core::ptr::null_mut::<statsnode>();
    a = firstnode;
    while !a.is_null() {
        an = (*a).nextsibling as *mut statsnode;
        stats_free(a);
        free(a as *mut ::core::ffi::c_void);
        a = an;
    }
}
