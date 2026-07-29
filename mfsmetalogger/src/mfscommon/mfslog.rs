pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    static mut stderr: *mut FILE;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn closelog();
    unsafe fn openlog(
        __ident: *const ::core::ffi::c_char,
        __option: ::core::ffi::c_int,
        __facility: ::core::ffi::c_int,
    );
    unsafe fn syslog(__pri: ::core::ffi::c_int, __fmt: *const ::core::ffi::c_char, ...);
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn backtrace(
        __array: *mut *mut ::core::ffi::c_void,
        __size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn backtrace_symbols(
        __array: *const *mut ::core::ffi::c_void,
        __size: ::core::ffi::c_int,
    ) -> *mut *mut ::core::ffi::c_char;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type va_list = __gnuc_va_list;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LOG_ERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LOG_WARNING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LOG_NOTICE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LOG_INFO: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LOG_DEBUG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LOG_USER: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_DAEMON: ::core::ffi::c_int = (3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_PID: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const LOG_NDELAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MFSLOG_DEBUG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LOGBUFFSIZE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const MSGBUFFSIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
static mut mfs_log_sink: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()> = None;
static mut force_stderr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut use_colors: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut stderr_active: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut mfs_log_min_level: ::core::ffi::c_int = MFSLOG_INFO;
static mut mfs_log_elevate_to: ::core::ffi::c_int = MFSLOG_NOTICE;
static mut syslog_open: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut mfs_log_priority_strings: [*const ::core::ffi::c_char; 5] = [
    b"debug\0".as_ptr() as *const ::core::ffi::c_char,
    b"info\0".as_ptr() as *const ::core::ffi::c_char,
    b"notice\0".as_ptr() as *const ::core::ffi::c_char,
    b"warning\0".as_ptr() as *const ::core::ffi::c_char,
    b"error\0".as_ptr() as *const ::core::ffi::c_char,
];
pub const MFSLOG_PRI_MIN: ::core::ffi::c_int = MFSLOG_DEBUG;
pub const MFSLOG_PRI_MAX: ::core::ffi::c_int = MFSLOG_ERR;
unsafe extern "C" fn mfs_log_pri_to_str(
    mut priority: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        if priority >= MFSLOG_PRI_MIN && priority <= MFSLOG_PRI_MAX {
            return mfs_log_priority_strings[priority as usize];
        }
        return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log_str_to_pri(
    mut pristr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: ::core::ffi::c_char = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut fpri: ::core::ffi::c_int = 0;
        let mut fpristr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        fpristr = ::core::ptr::null::<::core::ffi::c_char>();
        fpri = -1 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while *pristr.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            c = *pristr.offset(i as isize);
            if c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            {
                c = (c as ::core::ffi::c_int
                    + ('a' as ::core::ffi::c_int - 'A' as ::core::ffi::c_int))
                    as ::core::ffi::c_char;
            }
            if c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
            {
                if !fpristr.is_null() {
                    if *fpristr.offset(i as isize) as ::core::ffi::c_int != c as ::core::ffi::c_int
                    {
                        return -1 as ::core::ffi::c_int;
                    }
                } else {
                    j = MFSLOG_PRI_MIN;
                    while j <= MFSLOG_PRI_MAX {
                        if *mfs_log_priority_strings[j as usize].offset(i as isize)
                            as ::core::ffi::c_int
                            == c as ::core::ffi::c_int
                        {
                            fpristr = mfs_log_priority_strings[j as usize];
                            fpri = j;
                        }
                        j += 1;
                    }
                    if fpristr.is_null() {
                        return -1 as ::core::ffi::c_int;
                    }
                }
            } else {
                return -1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        return fpri;
    }
}
pub const COLOR_DEBUG: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"\x1B[0;90m\0") };
pub const COLOR_INFO: [::core::ffi::c_char; 1] =
    unsafe { ::core::mem::transmute::<[u8; 1], [::core::ffi::c_char; 1]>(*b"\0") };
pub const COLOR_NOTICE: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"\x1B[1;97m\0") };
pub const COLOR_WARNING: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"\x1B[1;93m\0") };
pub const COLOR_ERROR: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"\x1B[1;31m\0") };
pub const COLOR_CLEAR: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"\x1B(B\x1B[m\0") };
unsafe extern "C" fn mfs_log_priority_convert(
    mut priority: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut elevated_priority: ::core::ffi::c_int = 0;
        if priority < mfs_log_elevate_to {
            elevated_priority = mfs_log_elevate_to;
        } else {
            elevated_priority = priority;
        }
        match elevated_priority {
            MFSLOG_DEBUG => return LOG_DEBUG,
            MFSLOG_INFO => return LOG_INFO,
            MFSLOG_NOTICE => return LOG_NOTICE,
            MFSLOG_WARNING => return LOG_WARNING,
            _ => return LOG_ERR,
        };
    }
}
unsafe extern "C" fn mfs_log_pri_to_colorstr(
    mut priority: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        match priority {
            MFSLOG_DEBUG => return COLOR_DEBUG.as_ptr(),
            MFSLOG_INFO => return COLOR_INFO.as_ptr(),
            MFSLOG_NOTICE => return COLOR_NOTICE.as_ptr(),
            MFSLOG_WARNING => return COLOR_WARNING.as_ptr(),
            MFSLOG_ERR => return COLOR_ERROR.as_ptr(),
            _ => {}
        }
        return b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_file_log(
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut func: *const ::core::ffi::c_char,
    mut bt: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut btbuf: [*mut ::core::ffi::c_void; 100] =
            [::core::ptr::null_mut::<::core::ffi::c_void>(); 100];
        let mut btstr: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut ap: ::core::ffi::VaList;
        static mut lfd: *mut FILE = ::core::ptr::null_mut::<FILE>();
        if fmt.is_null() {
            if !lfd.is_null() {
                fclose(lfd);
                lfd = ::core::ptr::null_mut::<FILE>();
            }
            return;
        }
        if lfd.is_null() {
            lfd = fopen(
                b"mfsdebug.txt\0".as_ptr() as *const ::core::ffi::c_char,
                b"a\0".as_ptr() as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if lfd.is_null() {
                return;
            }
        }
        fprintf(
            lfd,
            b"%s:%d (%s):\0".as_ptr() as *const ::core::ffi::c_char,
            file,
            line,
            func,
        );
        ap = c2rust_args.clone();
        vfprintf(lfd, fmt, ap.clone());
        fprintf(lfd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        if bt != 0 {
            n = backtrace(&raw mut btbuf as *mut *mut ::core::ffi::c_void, BT_BUF_SIZE);
            btstr = backtrace_symbols(&raw mut btbuf as *mut *mut ::core::ffi::c_void, n);
            if !btstr.is_null() {
                i = 1 as ::core::ffi::c_int;
                while i < n {
                    fprintf(
                        lfd,
                        b"\t%u: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        i,
                        *btstr.offset(i as isize),
                    );
                    i += 1;
                }
                free(btstr as *mut ::core::ffi::c_void);
            } else {
                i = 1 as ::core::ffi::c_int;
                while i < n {
                    fprintf(
                        lfd,
                        b"\t%u: [%p]\n\0".as_ptr() as *const ::core::ffi::c_char,
                        i,
                        btbuf[i as usize],
                    );
                    i += 1;
                }
            }
        }
    }
}
pub const BT_BUF_SIZE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log(
    mut mode: ::core::ffi::c_int,
    mut priority: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut msg: [::core::ffi::c_char; 4096] = [0; 4096];
        let mut p: [::core::ffi::c_char; 2048] = [0; 2048];
        let mut n: ::core::ffi::c_int = 0;
        let mut ap: ::core::ffi::VaList;
        let mut _mfs_errstring: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        if priority < mfs_log_min_level {
            return;
        }
        if mode & 1 as ::core::ffi::c_int != 0 {
            _mfs_errstring = strerr(*__errno_location());
        } else {
            _mfs_errstring = ::core::ptr::null::<::core::ffi::c_char>();
        }
        ap = c2rust_args.clone();
        n = vsnprintf(
            &raw mut p as *mut ::core::ffi::c_char,
            LOGBUFFSIZE as size_t,
            fmt,
            ap.clone(),
        );
        if n < 0 as ::core::ffi::c_int {
            return;
        }
        p[(LOGBUFFSIZE - 1 as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_char;
        if mode & 1 as ::core::ffi::c_int != 0 {
            snprintf(
                &raw mut msg as *mut ::core::ffi::c_char,
                MSGBUFFSIZE as size_t,
                b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut p as *mut ::core::ffi::c_char,
                _mfs_errstring,
            );
        } else {
            snprintf(
                &raw mut msg as *mut ::core::ffi::c_char,
                MSGBUFFSIZE as size_t,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut p as *mut ::core::ffi::c_char,
            );
        }
        msg[(MSGBUFFSIZE - 1 as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_char;
        let sink = *&raw const mfs_log_sink;
        if sink.is_some() {
            sink.expect("non-null function pointer")(
                &raw mut msg as *mut ::core::ffi::c_char,
            );
        }
        if syslog_open != 0 {
            syslog(
                mfs_log_priority_convert(priority),
                b"[%s] %s\0".as_ptr() as *const ::core::ffi::c_char,
                mfs_log_pri_to_str(priority),
                &raw mut msg as *mut ::core::ffi::c_char,
            );
        }
        if stderr_active == 0 as ::core::ffi::c_int {
            return;
        }
        if syslog_open == 0 as ::core::ffi::c_int
            || force_stderr != 0
            || mode & 2 as ::core::ffi::c_int != 0
        {
            if use_colors != 0 {
                fprintf(
                    stderr,
                    b"%s%s%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    mfs_log_pri_to_colorstr(priority),
                    &raw mut msg as *mut ::core::ffi::c_char,
                    COLOR_CLEAR.as_ptr(),
                );
            } else {
                fprintf(
                    stderr,
                    b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut msg as *mut ::core::ffi::c_char,
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log_set_min_level(mut minlevel: ::core::ffi::c_int) {
    unsafe {
        mfs_log_min_level = minlevel;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log_set_elevate_to(mut elevateto: ::core::ffi::c_int) {
    unsafe {
        mfs_log_elevate_to = elevateto;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log_set_sink_function(
    mut s: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>,
) {
    unsafe {
        mfs_log_sink = s;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log_detach_stderr() {
    unsafe {
        stderr_active = 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log_detach_syslog() {
    unsafe {
        if syslog_open != 0 {
            closelog();
        }
        syslog_open = 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log_term() {
    unsafe {
        if syslog_open != 0 {
            closelog();
        }
        mfs_file_log(
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_log_init(
    mut ident: *const ::core::ffi::c_char,
    mut daemonflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if !ident.is_null() {
            if daemonflag != 0 {
                openlog(ident, LOG_PID | LOG_NDELAY, LOG_DAEMON);
            } else {
                openlog(ident, LOG_PID | LOG_NDELAY, LOG_USER);
            }
            syslog_open = 1 as ::core::ffi::c_int;
        }
        force_stderr = if daemonflag != 0 {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        use_colors = if isatty(STDERR_FILENO) != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        stderr_active = 1 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
}
