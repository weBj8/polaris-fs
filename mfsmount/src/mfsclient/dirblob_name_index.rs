extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_index {
    pub old_hashtab: *mut *mut uint8_t,
    pub new_hashtab: *mut *mut uint8_t,
    pub old_size: size_t,
    pub new_size: size_t,
    pub count: size_t,
    pub rehash_pos: size_t,
    pub is_rehashing: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const REHASH_STEP: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CLUSTER_SIZE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
unsafe extern "C" fn hash_function(mut str: *const uint8_t, mut len: uint8_t) -> uint32_t {
    let mut h: uint32_t = 5381 as uint32_t;
    let mut i: uint8_t = 0 as uint8_t;
    while (i as ::core::ffi::c_int) < len as ::core::ffi::c_int {
        h = (h << 5 as ::core::ffi::c_int)
            .wrapping_add(h)
            .wrapping_add(*str.offset(i as isize) as uint32_t);
        i = i.wrapping_add(1);
    }
    return h;
}
unsafe extern "C" fn hash_insert(
    mut hashtab: *mut *mut uint8_t,
    mut hashsize: uint32_t,
    mut ptr: *mut uint8_t,
    mut hash: uint32_t,
) {
    let mut i: uint32_t = 0;
    let mut disp: uint32_t = 0;
    let mut hashmask: uint32_t = 0;
    hashmask = hashsize.wrapping_sub(1 as uint32_t);
    disp = hash.wrapping_mul(0x53b23891 as uint32_t) & hashmask | 1 as uint32_t;
    loop {
        i = 0 as uint32_t;
        while i < CLUSTER_SIZE as uint32_t {
            if (*hashtab.offset((hash.wrapping_add(i) & hashmask) as isize)).is_null() {
                *hashtab.offset((hash.wrapping_add(i) & hashmask) as isize) = ptr;
                return;
            }
            i = i.wrapping_add(1);
        }
        hash = hash.wrapping_add(disp);
    }
}
unsafe extern "C" fn hash_find(
    mut hashtab: *mut *mut uint8_t,
    mut hashsize: uint32_t,
    mut name: *const uint8_t,
    mut len: uint8_t,
    mut hash: uint32_t,
) -> *mut uint8_t {
    let mut i: uint32_t = 0;
    let mut disp: uint32_t = 0;
    let mut hashmask: uint32_t = 0;
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    hashmask = hashsize.wrapping_sub(1 as uint32_t);
    disp = hash.wrapping_mul(0x53b23891 as uint32_t) & hashmask | 1 as uint32_t;
    loop {
        i = 0 as uint32_t;
        while i < CLUSTER_SIZE as uint32_t {
            ptr = *hashtab.offset((hash.wrapping_add(i) & hashmask) as isize);
            if ptr.is_null() {
                return ::core::ptr::null_mut::<uint8_t>();
            }
            if *ptr as ::core::ffi::c_int == len as ::core::ffi::c_int
                && memcmp(
                    ptr.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    len as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                return ptr;
            }
            i = i.wrapping_add(1);
        }
        hash = hash.wrapping_add(disp);
    }
}
unsafe extern "C" fn do_incremental_rehash(mut idx: *mut name_index) {
    let mut steps: ::core::ffi::c_int = 0;
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut len: uint8_t = 0;
    let mut str: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut hash: uint32_t = 0;
    if (*idx).is_rehashing == 0 {
        return;
    }
    steps = REHASH_STEP;
    while steps > 0 as ::core::ffi::c_int && (*idx).rehash_pos < (*idx).old_size {
        ptr = *(*idx).old_hashtab.offset((*idx).rehash_pos as isize);
        if !ptr.is_null() {
            len = *ptr.offset(0 as isize);
            str = ptr.offset(1 as ::core::ffi::c_int as isize);
            hash = hash_function(str, len);
            hash_insert((*idx).new_hashtab, (*idx).new_size as uint32_t, ptr, hash);
        }
        (*idx).rehash_pos = (*idx).rehash_pos.wrapping_add(1);
        steps -= 1;
    }
    if (*idx).rehash_pos >= (*idx).old_size {
        free((*idx).old_hashtab as *mut ::core::ffi::c_void);
        (*idx).old_hashtab = (*idx).new_hashtab;
        (*idx).old_size = (*idx).new_size;
        (*idx).new_hashtab = ::core::ptr::null_mut::<*mut uint8_t>();
        (*idx).new_size = 0 as size_t;
        (*idx).is_rehashing = 0 as ::core::ffi::c_int;
        (*idx).rehash_pos = 0 as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn name_index_create(mut minelements: uint32_t) -> *mut ::core::ffi::c_void {
    let mut idx: *mut name_index = ::core::ptr::null_mut::<name_index>();
    let mut initial_size: uint32_t = 0;
    initial_size = minelements
        .wrapping_mul(5 as uint32_t)
        .wrapping_div(3 as uint32_t);
    initial_size |= initial_size >> 1 as ::core::ffi::c_int;
    initial_size |= initial_size >> 2 as ::core::ffi::c_int;
    initial_size |= initial_size >> 4 as ::core::ffi::c_int;
    initial_size |= initial_size >> 8 as ::core::ffi::c_int;
    initial_size |= initial_size >> 16 as ::core::ffi::c_int;
    initial_size = initial_size.wrapping_add(1);
    if initial_size < 256 as uint32_t {
        initial_size = 256 as uint32_t;
    }
    idx = malloc(::core::mem::size_of::<name_index>()) as *mut name_index;
    (*idx).old_hashtab = calloc(
        initial_size as size_t,
        ::core::mem::size_of::<*mut uint8_t>(),
    ) as *mut *mut uint8_t;
    (*idx).new_hashtab = ::core::ptr::null_mut::<*mut uint8_t>();
    (*idx).old_size = initial_size as size_t;
    (*idx).new_size = 0 as size_t;
    (*idx).count = 0 as size_t;
    (*idx).rehash_pos = 0 as size_t;
    (*idx).is_rehashing = 0 as ::core::ffi::c_int;
    return idx as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn name_index_destroy(mut vidx: *mut ::core::ffi::c_void) {
    let mut idx: *mut name_index = vidx as *mut name_index;
    free((*idx).old_hashtab as *mut ::core::ffi::c_void);
    if !(*idx).new_hashtab.is_null() {
        free((*idx).new_hashtab as *mut ::core::ffi::c_void);
    }
    free(idx as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn name_index_add(mut vidx: *mut ::core::ffi::c_void, mut ptr: *mut uint8_t) {
    let mut len: uint8_t = 0;
    let mut str: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut hash: uint32_t = 0;
    let mut idx: *mut name_index = vidx as *mut name_index;
    do_incremental_rehash(idx);
    if (*idx).is_rehashing == 0
        && (*idx).count.wrapping_mul(5 as size_t) > (*idx).old_size.wrapping_mul(3 as size_t)
    {
        (*idx).new_size = (*idx).old_size.wrapping_mul(2 as size_t);
        (*idx).new_hashtab =
            calloc((*idx).new_size, ::core::mem::size_of::<*mut uint8_t>()) as *mut *mut uint8_t;
        (*idx).is_rehashing = 1 as ::core::ffi::c_int;
        (*idx).rehash_pos = 0 as size_t;
    }
    len = *ptr.offset(0 as isize);
    str = ptr.offset(1 as ::core::ffi::c_int as isize);
    hash = hash_function(str, len);
    if (*idx).is_rehashing != 0 {
        hash_insert((*idx).new_hashtab, (*idx).new_size as uint32_t, ptr, hash);
    } else {
        hash_insert((*idx).old_hashtab, (*idx).old_size as uint32_t, ptr, hash);
    }
    (*idx).count = (*idx).count.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn name_index_find(
    mut vidx: *mut ::core::ffi::c_void,
    mut str: *const uint8_t,
    mut len: uint8_t,
) -> *mut uint8_t {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut hash: uint32_t = 0;
    let mut idx: *mut name_index = vidx as *mut name_index;
    do_incremental_rehash(idx);
    hash = hash_function(str, len);
    if (*idx).is_rehashing != 0 {
        ptr = hash_find(
            (*idx).new_hashtab,
            (*idx).new_size as uint32_t,
            str,
            len,
            hash,
        );
        if !ptr.is_null() {
            return ptr;
        }
    }
    return hash_find(
        (*idx).old_hashtab,
        (*idx).old_size as uint32_t,
        str,
        len,
        hash,
    );
}
