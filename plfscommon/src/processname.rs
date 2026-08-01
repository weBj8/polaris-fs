unsafe extern "C" {
    static mut environ: *mut *mut ::core::ffi::c_char;
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
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
pub type uint32_t = u32;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
// BOUNDARY MODULE (P1): processname mutates the process-global argv memory
// region and swaps `environ` so the new name shows in ps/top. This is
// inherently raw-pointer work; the module stays unsafe with annotated blocks
// rather than being made "safe" (there is no safe abstraction for rewriting
// another allocation's bytes — the C runtime's argv area).
static mut argv_start: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut argv_leng: uint32_t = 0;
static mut myenvcpy: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
// SAFETY (module): argv/environ pointers come from the C runtime and are
// valid for the process lifetime; malloc'd env copy is freed only in
// processname_term; all offset walks are bounded by NUL terminators the C
// runtime guarantees.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn processname_init(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    unsafe {
        unsafe extern "C" {
            #[link_name = "environ"]
            static mut environ_0: *mut *mut ::core::ffi::c_char;
        }
        let mut argp: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut lastpos: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        argv_start = ::core::ptr::null_mut::<::core::ffi::c_char>();
        argv_leng = 0 as uint32_t;
        if argc == 0 as ::core::ffi::c_int || (*argv.offset(0 as isize)).is_null() {
            return;
        }
        argp = environ;
        i = 0 as ::core::ffi::c_int;
        while !(*argp.offset(i as isize)).is_null() {
            i += 1;
        }
        environ = malloc(
            ((i + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
        ) as *mut *mut ::core::ffi::c_char;
        myenvcpy = environ;
        if environ.is_null() {
            environ = argp;
            return;
        }
        i = 0 as ::core::ffi::c_int;
        while !(*argp.offset(i as isize)).is_null() {
            *environ.offset(i as isize) = strdup(*argp.offset(i as isize));
            if (*environ.offset(i as isize)).is_null() {
                j = 0 as ::core::ffi::c_int;
                while j < i {
                    free(*environ.offset(i as isize) as *mut ::core::ffi::c_void);
                    j += 1;
                }
                free(environ as *mut ::core::ffi::c_void);
                environ = argp;
                return;
            }
            i += 1;
        }
        *environ.offset(i as isize) = ::core::ptr::null_mut::<::core::ffi::c_char>();
        lastpos = ::core::ptr::null_mut::<::core::ffi::c_char>();
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            if lastpos.is_null()
                || lastpos.offset(1 as ::core::ffi::c_int as isize) == *argv.offset(i as isize)
            {
                lastpos =
                    (*argv.offset(i as isize)).offset(strlen(*argv.offset(i as isize)) as isize);
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while !(*argp.offset(i as isize)).is_null() {
            if lastpos.offset(1 as ::core::ffi::c_int as isize) == *argp.offset(i as isize) {
                lastpos =
                    (*argp.offset(i as isize)).offset(strlen(*argp.offset(i as isize)) as isize);
            }
            i += 1;
        }
        argv_start = *argv.offset(0 as isize);
        argv_leng = (lastpos.offset_from(argv_start) - 1 as isize) as uint32_t;
    }
}
// SAFETY: argv_start/argv_leng bound the writable region (computed in init
// from contiguous argv+environ storage); name must be a valid NUL-terminated
// C string (C caller contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn processname_set(mut name: *mut ::core::ffi::c_char) {
    unsafe {
        let mut l: uint32_t = 0;
        if argv_leng > 0 as uint32_t {
            l = strlen(name) as uint32_t;
            if l >= argv_leng {
                l = argv_leng.wrapping_sub(1 as uint32_t);
            }
            if l > 0 as uint32_t {
                memcpy(
                    argv_start as *mut ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    l as size_t,
                );
            }
            memset(
                argv_start.offset(l as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                argv_leng.wrapping_sub(l) as size_t,
            );
        }
    }
}
// SAFETY: myenvcpy was allocated by malloc in init (or is null); freed once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn processname_term() {
    unsafe {
        if !myenvcpy.is_null() {
            free(myenvcpy as *mut ::core::ffi::c_void);
        }
    }
}
