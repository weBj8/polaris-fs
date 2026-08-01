unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type uint16_t = u16;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _csdbentry {
    pub ip: uint32_t,
    pub port: uint16_t,
    pub readopcnt: uint32_t,
    pub writeopcnt: uint32_t,
    pub next: *mut _csdbentry,
}
pub type csdbentry = _csdbentry;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const CSDB_HASHSIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
static mut csdbhtab: [*mut csdbentry; 256] = [::core::ptr::null_mut::<csdbentry>(); 256];
static mut csdblock: *mut pthread_mutex_t = ::core::ptr::null_mut::<pthread_mutex_t>();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csdb_init() {
    unsafe {
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < CSDB_HASHSIZE as uint32_t {
            csdbhtab[i as usize] = ::core::ptr::null_mut::<csdbentry>();
            i = i.wrapping_add(1);
        }
        csdblock = malloc(::core::mem::size_of::<pthread_mutex_t>()) as *mut pthread_mutex_t;
        pthread_mutex_init(csdblock, ::core::ptr::null::<pthread_mutexattr_t>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csdb_term() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut cs: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
        let mut csn: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
        pthread_mutex_destroy(csdblock);
        free(csdblock as *mut ::core::ffi::c_void);
        i = 0 as uint32_t;
        while i < CSDB_HASHSIZE as uint32_t {
            cs = csdbhtab[i as usize];
            while !cs.is_null() {
                csn = (*cs).next as *mut csdbentry;
                free(cs as *mut ::core::ffi::c_void);
                cs = csn;
            }
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csdb_getreadcnt(mut ip: uint32_t, mut port: uint16_t) -> uint32_t {
    unsafe {
        let mut hash: uint32_t = ip
            .wrapping_mul(0x7b348943 as uint32_t)
            .wrapping_add(port as uint32_t)
            .wrapping_rem(CSDB_HASHSIZE as uint32_t);
        let mut result: uint32_t = 0 as uint32_t;
        let mut e: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
        pthread_mutex_lock(csdblock);
        e = csdbhtab[hash as usize];
        while !e.is_null() {
            if (*e).ip == ip && (*e).port as ::core::ffi::c_int == port as ::core::ffi::c_int {
                result = (*e).readopcnt;
                break;
            } else {
                e = (*e).next as *mut csdbentry;
            }
        }
        pthread_mutex_unlock(csdblock);
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csdb_getwritecnt(mut ip: uint32_t, mut port: uint16_t) -> uint32_t {
    unsafe {
        let mut hash: uint32_t = ip
            .wrapping_mul(0x7b348943 as uint32_t)
            .wrapping_add(port as uint32_t)
            .wrapping_rem(CSDB_HASHSIZE as uint32_t);
        let mut result: uint32_t = 0 as uint32_t;
        let mut e: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
        pthread_mutex_lock(csdblock);
        e = csdbhtab[hash as usize];
        while !e.is_null() {
            if (*e).ip == ip && (*e).port as ::core::ffi::c_int == port as ::core::ffi::c_int {
                result = (*e).writeopcnt;
                break;
            } else {
                e = (*e).next as *mut csdbentry;
            }
        }
        pthread_mutex_unlock(csdblock);
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csdb_getopcnt(mut ip: uint32_t, mut port: uint16_t) -> uint32_t {
    unsafe {
        let mut hash: uint32_t = ip
            .wrapping_mul(0x7b348943 as uint32_t)
            .wrapping_add(port as uint32_t)
            .wrapping_rem(CSDB_HASHSIZE as uint32_t);
        let mut result: uint32_t = 0 as uint32_t;
        let mut e: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
        pthread_mutex_lock(csdblock);
        e = csdbhtab[hash as usize];
        while !e.is_null() {
            if (*e).ip == ip && (*e).port as ::core::ffi::c_int == port as ::core::ffi::c_int {
                result = (*e).readopcnt.wrapping_add((*e).writeopcnt);
                break;
            } else {
                e = (*e).next as *mut csdbentry;
            }
        }
        pthread_mutex_unlock(csdblock);
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csdb_readinc(mut ip: uint32_t, mut port: uint16_t) {
    unsafe {
        let mut hash: uint32_t = ip
            .wrapping_mul(0x7b348943 as uint32_t)
            .wrapping_add(port as uint32_t)
            .wrapping_rem(CSDB_HASHSIZE as uint32_t);
        let mut e: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
        pthread_mutex_lock(csdblock);
        e = csdbhtab[hash as usize];
        while !e.is_null() {
            if (*e).ip == ip && (*e).port as ::core::ffi::c_int == port as ::core::ffi::c_int {
                (*e).readopcnt = (*e).readopcnt.wrapping_add(1);
                pthread_mutex_unlock(csdblock);
                return;
            }
            e = (*e).next as *mut csdbentry;
        }
        e = malloc(::core::mem::size_of::<csdbentry>()) as *mut csdbentry;
        (*e).ip = ip;
        (*e).port = port;
        (*e).readopcnt = 1 as uint32_t;
        (*e).writeopcnt = 0 as uint32_t;
        (*e).next = csdbhtab[hash as usize] as *mut _csdbentry;
        csdbhtab[hash as usize] = e;
        pthread_mutex_unlock(csdblock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csdb_readdec(mut ip: uint32_t, mut port: uint16_t) {
    unsafe {
        let mut hash: uint32_t = ip
            .wrapping_mul(0x7b348943 as uint32_t)
            .wrapping_add(port as uint32_t)
            .wrapping_rem(CSDB_HASHSIZE as uint32_t);
        let mut e: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
        pthread_mutex_lock(csdblock);
        e = csdbhtab[hash as usize];
        while !e.is_null() {
            if (*e).ip == ip && (*e).port as ::core::ffi::c_int == port as ::core::ffi::c_int {
                (*e).readopcnt = (*e).readopcnt.wrapping_sub(1);
                pthread_mutex_unlock(csdblock);
                return;
            }
            e = (*e).next as *mut csdbentry;
        }
        pthread_mutex_unlock(csdblock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csdb_writeinc(mut ip: uint32_t, mut port: uint16_t) {
    unsafe {
        let mut hash: uint32_t = ip
            .wrapping_mul(0x7b348943 as uint32_t)
            .wrapping_add(port as uint32_t)
            .wrapping_rem(CSDB_HASHSIZE as uint32_t);
        let mut e: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
        pthread_mutex_lock(csdblock);
        e = csdbhtab[hash as usize];
        while !e.is_null() {
            if (*e).ip == ip && (*e).port as ::core::ffi::c_int == port as ::core::ffi::c_int {
                (*e).writeopcnt = (*e).writeopcnt.wrapping_add(1);
                pthread_mutex_unlock(csdblock);
                return;
            }
            e = (*e).next as *mut csdbentry;
        }
        e = malloc(::core::mem::size_of::<csdbentry>()) as *mut csdbentry;
        (*e).ip = ip;
        (*e).port = port;
        (*e).readopcnt = 0 as uint32_t;
        (*e).writeopcnt = 1 as uint32_t;
        (*e).next = csdbhtab[hash as usize] as *mut _csdbentry;
        csdbhtab[hash as usize] = e;
        pthread_mutex_unlock(csdblock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csdb_writedec(mut ip: uint32_t, mut port: uint16_t) {
    unsafe {
        let mut hash: uint32_t = ip
            .wrapping_mul(0x7b348943 as uint32_t)
            .wrapping_add(port as uint32_t)
            .wrapping_rem(CSDB_HASHSIZE as uint32_t);
        let mut e: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
        pthread_mutex_lock(csdblock);
        e = csdbhtab[hash as usize];
        while !e.is_null() {
            if (*e).ip == ip && (*e).port as ::core::ffi::c_int == port as ::core::ffi::c_int {
                (*e).writeopcnt = (*e).writeopcnt.wrapping_sub(1);
                pthread_mutex_unlock(csdblock);
                return;
            }
            e = (*e).next as *mut csdbentry;
        }
        pthread_mutex_unlock(csdblock);
    }
}
