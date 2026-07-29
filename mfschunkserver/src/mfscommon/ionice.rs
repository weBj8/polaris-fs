extern "C" {
    fn setpriority(
        __which: __priority_which_t,
        __who: id_t,
        __prio: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn syscall(__sysno: ::core::ffi::c_long, ...) -> ::core::ffi::c_long;
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
pub type __id_t = ::core::ffi::c_uint;
pub type __priority_which = ::core::ffi::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type id_t = __id_t;
pub type __priority_which_t = __priority_which;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const IOPRIO_CLASS_IDLE: C2Rust_Unnamed = 3;
pub const IOPRIO_CLASS_BE: C2Rust_Unnamed = 2;
pub const IOPRIO_CLASS_RT: C2Rust_Unnamed = 1;
pub const IOPRIO_CLASS_NONE: C2Rust_Unnamed = 0;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const IOPRIO_WHO_USER: C2Rust_Unnamed_0 = 3;
pub const IOPRIO_WHO_PGRP: C2Rust_Unnamed_0 = 2;
pub const IOPRIO_WHO_PROCESS: C2Rust_Unnamed_0 = 1;
pub const __NR_ioprio_set: ::core::ffi::c_int = 251 as ::core::ffi::c_int;
pub const SYS_ioprio_set: ::core::ffi::c_int = __NR_ioprio_set;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn ioprio_set(
    mut which: ::core::ffi::c_int,
    mut who: ::core::ffi::c_int,
    mut ioprio: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return syscall(SYS_ioprio_set as ::core::ffi::c_long, which, who, ioprio)
        as ::core::ffi::c_int;
}
pub const IOPRIO_CLASS_SHIFT: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ionice_test() {
    mfs_log(
        MFSLOG_SYSLOG,
        MFSLOG_INFO,
        b"using linux ioprio (ionice) syscalls for I/O priority\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ionice_high() {
    ioprio_set(
        IOPRIO_WHO_PROCESS as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        (IOPRIO_CLASS_RT as ::core::ffi::c_int) << IOPRIO_CLASS_SHIFT | 0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ionice_medium() {
    ioprio_set(
        IOPRIO_WHO_PROCESS as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        (IOPRIO_CLASS_BE as ::core::ffi::c_int) << IOPRIO_CLASS_SHIFT | 0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ionice_low() {
    setpriority(PRIO_PROCESS, 0 as id_t, 19 as ::core::ffi::c_int);
    ioprio_set(
        IOPRIO_WHO_PROCESS as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        (IOPRIO_CLASS_IDLE as ::core::ffi::c_int) << IOPRIO_CLASS_SHIFT | 0 as ::core::ffi::c_int,
    );
}
