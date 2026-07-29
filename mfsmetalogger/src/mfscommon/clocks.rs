extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
}
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn monotonic_seconds() -> ::core::ffi::c_double {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut ts);
    return ts.tv_sec as ::core::ffi::c_double
        + ts.tv_nsec as ::core::ffi::c_double * 0.000000001f64;
}
#[no_mangle]
pub unsafe extern "C" fn monotonic_nseconds() -> uint64_t {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut ts);
    return (ts.tv_sec as uint64_t)
        .wrapping_mul(1000000000 as uint64_t)
        .wrapping_add(ts.tv_nsec as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn monotonic_useconds() -> uint64_t {
    return monotonic_nseconds().wrapping_div(1000 as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn monotonic_method() -> *const ::core::ffi::c_char {
    return b"clock_gettime\0".as_ptr() as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn monotonic_speed() -> uint32_t {
    let mut i: uint32_t = 0;
    let mut st: uint64_t = 0;
    let mut en: uint64_t = 0;
    i = 0 as uint32_t;
    st = monotonic_nseconds().wrapping_add(10000000 as uint64_t);
    loop {
        en = monotonic_nseconds();
        i = i.wrapping_add(1);
        if en >= st {
            break;
        }
    }
    return i;
}
