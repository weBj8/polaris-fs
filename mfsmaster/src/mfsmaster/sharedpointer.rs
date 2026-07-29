extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _shp {
    pub pointer: *mut ::core::ffi::c_void,
    pub freefn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub refcnt: uint32_t,
}
pub type shp = _shp;
#[no_mangle]
pub unsafe extern "C" fn shp_new(
    mut pointer: *mut ::core::ffi::c_void,
    mut freefn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> *mut ::core::ffi::c_void {
    let mut s: *mut shp = ::core::ptr::null_mut::<shp>();
    s = malloc(::core::mem::size_of::<shp>()) as *mut shp;
    (*s).pointer = pointer;
    (*s).freefn = freefn;
    (*s).refcnt = 1 as uint32_t;
    return s as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn shp_get(mut vs: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let mut s: *mut shp = vs as *mut shp;
    return (*s).pointer;
}
#[no_mangle]
pub unsafe extern "C" fn shp_inc(mut vs: *mut ::core::ffi::c_void) {
    let mut s: *mut shp = vs as *mut shp;
    (*s).refcnt = (*s).refcnt.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn shp_dec(mut vs: *mut ::core::ffi::c_void) {
    let mut s: *mut shp = vs as *mut shp;
    if (*s).refcnt > 0 as uint32_t {
        (*s).refcnt = (*s).refcnt.wrapping_sub(1);
    }
    if (*s).refcnt == 0 as uint32_t {
        (*s).freefn.expect("non-null function pointer")((*s).pointer);
        free(s as *mut ::core::ffi::c_void);
    }
}
