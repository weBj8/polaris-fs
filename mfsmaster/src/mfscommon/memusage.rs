extern "C" {
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn getpagesize() -> ::core::ffi::c_int;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
}
pub type size_t = usize;
pub type ssize_t = isize;
pub type uint8_t = u8;
pub type uint64_t = u64;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn mem_used(mut rss: *mut uint64_t, mut virt: *mut uint64_t) -> uint8_t {
    let mut fd: ::core::ffi::c_int = open(
        b"/proc/self/statm\0".as_ptr() as *const ::core::ffi::c_char,
        O_RDONLY,
    );
    let mut statbuff: [::core::ffi::c_char; 1000] = [0; 1000];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut l: ::core::ffi::c_int = 0;
    let mut e: ::core::ffi::c_int = 0;
    e = 0 as ::core::ffi::c_int;
    *rss = 0 as uint64_t;
    *virt = 0 as uint64_t;
    if fd >= 0 as ::core::ffi::c_int {
        l = read(
            fd,
            &raw mut statbuff as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            1000 as size_t,
        ) as ::core::ffi::c_int;
        if l < 1000 as ::core::ffi::c_int && l > 0 as ::core::ffi::c_int {
            statbuff[l as usize] = 0 as ::core::ffi::c_char;
            *virt = strtoul(
                &raw mut statbuff as *mut ::core::ffi::c_char,
                &raw mut p,
                10 as ::core::ffi::c_int,
            )
            .wrapping_mul(getpagesize() as ::core::ffi::c_ulong) as uint64_t;
            if *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
                *rss = strtoul(
                    p.offset(1 as ::core::ffi::c_int as isize),
                    &raw mut p,
                    10 as ::core::ffi::c_int,
                )
                .wrapping_mul(getpagesize() as ::core::ffi::c_ulong)
                    as uint64_t;
                if *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                    || *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                {
                    e = 1 as ::core::ffi::c_int;
                }
            }
        }
        close(fd);
    }
    return e as uint8_t;
}
