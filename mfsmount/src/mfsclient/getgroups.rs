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
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    unsafe fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
}
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
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
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct groups {
    pub lcnt: uint32_t,
    pub gidcnt: uint32_t,
    pub gidtab: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grcache {
    pub time: ::core::ffi::c_double,
    pub pid: pid_t,
    pub uid: uid_t,
    pub gid: gid_t,
    pub g: *mut groups,
    pub next: *mut grcache,
    pub prev: *mut *mut grcache,
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut ::core::ffi::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    unsafe {
        return __getdelim(__lineptr, __n, '\n' as ::core::ffi::c_int, __stream);
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn portable_usleep(mut usec: uint64_t) {
    unsafe {
        let mut req: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut rem: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut s: ::core::ffi::c_int = 0;
        req.tv_sec = usec.wrapping_div(1000000 as uint64_t) as __time_t;
        req.tv_nsec = usec
            .wrapping_rem(1000000 as uint64_t)
            .wrapping_mul(1000 as uint64_t) as __syscall_slong_t;
        loop {
            s = nanosleep(&raw mut req, &raw mut rem);
            if s < 0 as ::core::ffi::c_int {
                req = rem;
            }
            if s >= 0 as ::core::ffi::c_int {
                break;
            }
        }
    }
}
static mut main_thread: pthread_t = 0;
static mut keep_alive: ::core::ffi::c_int = 0;
pub const HASHSIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
static mut groups_hashtab: *mut *mut grcache = ::core::ptr::null_mut::<*mut grcache>();
static mut to: ::core::ffi::c_double = 0.;
static mut glock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __glibc_reserved: 0,
        __list: __pthread_list_t {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut debug_mode: ::core::ffi::c_int = 0;
#[inline]
unsafe extern "C" fn make_groups(mut gid: gid_t, mut gidcnt: uint32_t) -> *mut groups {
    unsafe {
        let mut ret: *mut groups = ::core::ptr::null_mut::<groups>();
        ret = malloc(
            ::core::mem::size_of::<groups>()
                .wrapping_add(::core::mem::size_of::<uint32_t>().wrapping_mul(gidcnt as size_t)),
        ) as *mut groups;
        (*ret).lcnt = 1 as uint32_t;
        (*ret).gidcnt = gidcnt;
        if gidcnt > 0 as uint32_t {
            (*ret).gidtab = ret.offset(1 as ::core::ffi::c_int as isize) as *mut uint32_t;
            *(*ret).gidtab.offset(0 as isize) = gid as uint32_t;
        } else {
            (*ret).gidtab = ::core::ptr::null_mut::<uint32_t>();
        }
        return ret;
    }
}
#[inline]
unsafe extern "C" fn get_groups(mut pid: pid_t, mut gid: gid_t) -> *mut groups {
    unsafe {
        let mut ret: *mut groups = ::core::ptr::null_mut::<groups>();
        let mut proc_filename: [::core::ffi::c_char; 50] = [0; 50];
        let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut gcount: uint32_t = 0;
        let mut n: uint32_t = 0;
        let mut g: gid_t = 0;
        let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut linebuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lbsize: size_t = 0;
        snprintf(
            &raw mut proc_filename as *mut ::core::ffi::c_char,
            50 as size_t,
            b"/proc/%d/status\0".as_ptr() as *const ::core::ffi::c_char,
            pid,
        );
        fd = fopen(
            &raw mut proc_filename as *mut ::core::ffi::c_char,
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if fd.is_null() {
            return make_groups(gid, 1 as uint32_t);
        }
        linebuff = malloc(1024 as size_t) as *mut ::core::ffi::c_char;
        lbsize = 1024 as size_t;
        while getline(&raw mut linebuff, &raw mut lbsize, fd) != -1 as __ssize_t {
            if strncmp(
                linebuff,
                b"Groups:\0".as_ptr() as *const ::core::ffi::c_char,
                7 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                gcount = 1 as uint32_t;
                ptr = linebuff.offset(7 as ::core::ffi::c_int as isize);
                loop {
                    while *ptr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                        || *ptr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(1);
                    }
                    if *ptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && *ptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        g = strtoul(ptr, &raw mut ptr, 10 as ::core::ffi::c_int) as gid_t;
                        if g != gid {
                            gcount = gcount.wrapping_add(1);
                        }
                    }
                    if !(*ptr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                        || *ptr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int)
                    {
                        break;
                    }
                }
                ret = make_groups(gid, gcount);
                n = 1 as uint32_t;
                ptr = linebuff.offset(7 as ::core::ffi::c_int as isize);
                loop {
                    while *ptr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                        || *ptr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(1);
                    }
                    if *ptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && *ptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        g = strtoul(ptr, &raw mut ptr, 10 as ::core::ffi::c_int) as gid_t;
                        if g != gid {
                            *(*ret).gidtab.offset(n as isize) = g as uint32_t;
                            n = n.wrapping_add(1);
                        }
                    }
                    if !((*ptr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                        || *ptr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int)
                        && n < gcount)
                    {
                        break;
                    }
                }
                fclose(fd);
                free(linebuff as *mut ::core::ffi::c_void);
                return ret;
            }
        }
        fclose(fd);
        free(linebuff as *mut ::core::ffi::c_void);
        return make_groups(gid, 1 as uint32_t);
    }
}
#[inline]
unsafe extern "C" fn groups_dump(mut g: *mut groups) {
    unsafe {
        let mut h: uint32_t = 0;
        h = 0 as uint32_t;
        while h < (*g).gidcnt {
            fprintf(
                stderr,
                b"%c%u\0".as_ptr() as *const ::core::ffi::c_char,
                if h == 0 as uint32_t {
                    '(' as ::core::ffi::c_int
                } else {
                    ',' as ::core::ffi::c_int
                },
                *(*g).gidtab.offset(h as isize),
            );
            h = h.wrapping_add(1);
        }
        if (*g).gidcnt == 0 as uint32_t {
            fprintf(stderr, b"EMPTY\n\0".as_ptr() as *const ::core::ffi::c_char);
        } else {
            fprintf(stderr, b")\n\0".as_ptr() as *const ::core::ffi::c_char);
        };
    }
}
#[inline]
unsafe extern "C" fn groups_decref(mut g: *mut groups) {
    unsafe {
        if (*g).lcnt > 0 as uint32_t {
            (*g).lcnt = (*g).lcnt.wrapping_sub(1);
        }
        if (*g).lcnt == 0 as uint32_t {
            free(g as *mut ::core::ffi::c_void);
        }
    }
}
#[inline]
unsafe extern "C" fn groups_remove(mut gc: *mut grcache) {
    unsafe {
        *(*gc).prev = (*gc).next;
        if !(*gc).next.is_null() {
            (*(*gc).next).prev = (*gc).prev;
        }
        groups_decref((*gc).g);
        free(gc as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_get_common(
    mut pid: pid_t,
    mut uid: uid_t,
    mut gid: gid_t,
    mut cacheonly: uint8_t,
) -> *mut groups {
    unsafe {
        let mut t: ::core::ffi::c_double = 0.;
        let mut h: uint32_t = 0;
        let mut g: *mut groups = ::core::ptr::null_mut::<groups>();
        let mut gf: *mut groups = ::core::ptr::null_mut::<groups>();
        let mut gc: *mut grcache = ::core::ptr::null_mut::<grcache>();
        let mut gcn: *mut grcache = ::core::ptr::null_mut::<grcache>();
        let mut gcf: *mut grcache = ::core::ptr::null_mut::<grcache>();
        if debug_mode != 0 {
            fprintf(
                stderr,
                b"groups_get(pid=%u,uid=%u,gid=%u)\n\0".as_ptr() as *const ::core::ffi::c_char,
                pid as uint32_t,
                uid as uint32_t,
                gid as uint32_t,
            );
        }
        t = monotonic_seconds();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut glock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    323 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    323 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    323 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    323 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    323 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    323 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        h = ((pid as ::core::ffi::c_int * 0x74bf4863 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_add(uid as ::core::ffi::c_uint)
            .wrapping_mul(0xb435c489 as ::core::ffi::c_uint)
            .wrapping_add(gid as ::core::ffi::c_uint)
            .wrapping_rem(HASHSIZE as ::core::ffi::c_uint) as uint32_t;
        gcf = ::core::ptr::null_mut::<grcache>();
        gc = *groups_hashtab.offset(h as isize);
        while !gc.is_null() {
            gcn = (*gc).next as *mut grcache;
            if (*gc).time + to < t && cacheonly as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                groups_remove(gc);
            } else if (*gc).pid == pid && (*gc).uid == uid && (*gc).gid == gid {
                gcf = gc;
            }
            gc = gcn;
        }
        if !gcf.is_null() {
            gf = (*gcf).g;
            (*gf).lcnt = (*gf).lcnt.wrapping_add(1);
        } else {
            gf = ::core::ptr::null_mut::<groups>();
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        if cacheonly != 0 {
            if !gf.is_null() {
                if debug_mode != 0 {
                    fprintf(
                        stderr,
                        b"groups_get(pid=%u,uid=%u,gid=%u):\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        pid as uint32_t,
                        uid as uint32_t,
                        gid as uint32_t,
                    );
                    groups_dump(gf);
                }
                return gf;
            } else {
                if debug_mode != 0 {
                    fprintf(
                        stderr,
                        b"groups_get(pid=%u,uid=%u,gid=%u) - emergency mode\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        pid as uint32_t,
                        uid as uint32_t,
                        gid as uint32_t,
                    );
                }
                return make_groups(gid, 1 as uint32_t);
            }
        } else if !gf.is_null() && uid != 0 as uid_t {
            if debug_mode != 0 {
                fprintf(
                    stderr,
                    b"groups_get(pid=%u,uid=%u,gid=%u):\0".as_ptr() as *const ::core::ffi::c_char,
                    pid as uint32_t,
                    uid as uint32_t,
                    gid as uint32_t,
                );
                groups_dump(gf);
            }
            return gf;
        }
        g = get_groups(pid, gid);
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_lock(&raw mut glock);
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    367 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    367 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    367 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    367 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    367 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    367 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        if !gf.is_null() {
            groups_decref(gf);
        }
        gcf = ::core::ptr::null_mut::<grcache>();
        gc = *groups_hashtab.offset(h as isize);
        while !gc.is_null() {
            if (*gc).pid == pid && (*gc).uid == uid && (*gc).gid == gid {
                gcf = gc;
            }
            gc = (*gc).next as *mut grcache;
        }
        if !gcf.is_null() {
            groups_decref((*gcf).g);
            (*gcf).g = g;
            (*g).lcnt = (*g).lcnt.wrapping_add(1);
        } else {
            gc = malloc(::core::mem::size_of::<grcache>()) as *mut grcache;
            (*gc).time = t;
            (*gc).pid = pid;
            (*gc).uid = uid;
            (*gc).gid = gid;
            (*gc).g = g;
            (*g).lcnt = (*g).lcnt.wrapping_add(1);
            (*gc).next = *groups_hashtab.offset(h as isize) as *mut grcache;
            if !(*gc).next.is_null() {
                (*(*gc).next).prev = &raw mut (*gc).next;
            }
            (*gc).prev = groups_hashtab.offset(h as isize) as *mut *mut grcache;
            *groups_hashtab.offset(h as isize) = gc;
        }
        let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
        if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    399 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    399 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
            } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_6: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    399 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    399 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
            } else {
                let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    399 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    399 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
        if debug_mode != 0 {
            fprintf(
                stderr,
                b"groups_get(pid=%u,uid=%u,gid=%u):\0".as_ptr() as *const ::core::ffi::c_char,
                pid as uint32_t,
                uid as uint32_t,
                gid as uint32_t,
            );
            groups_dump(g);
        }
        // caller reference: the cache insert above only adds the cache's own
        // reference. Without a separate caller reference, the caller's
        // groups_rel() drops lcnt to 0 and frees the object while the cache
        // entry still points at it (use-after-free, hit reliably by root
        // because the uid==0 path reallocates on every call).
        // Latent bug inherited from the C source; fixed here, not in callers.
        (*g).lcnt = (*g).lcnt.wrapping_add(1);
        return g;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_rel(mut g: *mut groups) {
    unsafe {
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut glock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        groups_decref(g);
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    410 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    410 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    410 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    410 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    410 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    410 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_cleanup_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        static mut h: uint32_t = 0 as uint32_t;
        let mut i: uint32_t = 0;
        let mut t: ::core::ffi::c_double = 0.;
        let mut gc: *mut grcache = ::core::ptr::null_mut::<grcache>();
        let mut gcn: *mut grcache = ::core::ptr::null_mut::<grcache>();
        let mut ka: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while ka != 0 {
            let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut glock);
            if _mfs_assert_ret != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        420 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        420 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        420 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        420 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                } else {
                    let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        420 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        420 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                }
                abort();
            }
            t = monotonic_seconds();
            i = 0 as uint32_t;
            while i < 16 as uint32_t {
                gc = *groups_hashtab.offset(h as isize);
                while !gc.is_null() {
                    gcn = (*gc).next as *mut grcache;
                    if (*gc).time + to < t {
                        groups_remove(gc);
                    }
                    gc = gcn;
                }
                h = h.wrapping_add(1);
                h = h.wrapping_rem(HASHSIZE as uint32_t);
                i = i.wrapping_add(1);
            }
            ka = keep_alive;
            let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
            if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        433 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        433 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        433 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        433 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                } else {
                    let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        433 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        433 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
            portable_usleep(10000 as uint64_t);
        }
        return arg;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_term() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut glock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    479 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    479 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    479 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    479 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    479 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    479 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        keep_alive = 0 as ::core::ffi::c_int;
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    481 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    481 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    481 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    481 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    481 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    481 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        pthread_join(
            main_thread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_lock(&raw mut glock);
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    486 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    486 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    486 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    486 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    486 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    486 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        i = 0 as uint32_t;
        while i < HASHSIZE as uint32_t {
            while !(*groups_hashtab.offset(i as isize)).is_null() {
                groups_remove(*groups_hashtab.offset(i as isize));
            }
            i = i.wrapping_add(1);
        }
        free(groups_hashtab as *mut ::core::ffi::c_void);
        let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
        if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    500 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    500 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
            } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_6: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    500 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    500 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
            } else {
                let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    500 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    500 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_3: ::core::ffi::c_int = pthread_mutex_destroy(&raw mut glock);
        if _mfs_assert_ret_3 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_3 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_7: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    501 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    501 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_7,
                );
            } else if _mfs_assert_ret_3 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_8: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_3);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    501 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_8,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    501 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_8,
                );
            } else {
                let mut _mfs_errorstring_err_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_3: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_3);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    501 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    501 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn groups_init(mut _to: ::core::ffi::c_double, mut dm: ::core::ffi::c_int) {
    unsafe {
        let mut i: uint32_t = 0;
        debug_mode = dm;
        let mut _mfs_assert_ret: ::core::ffi::c_int =
            pthread_mutex_init(&raw mut glock, ::core::ptr::null::<pthread_mutexattr_t>());
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    507 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    507 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    507 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    507 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    507 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    507 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        groups_hashtab =
            malloc(::core::mem::size_of::<*mut groups>().wrapping_mul(HASHSIZE as size_t))
                as *mut *mut grcache;
        if groups_hashtab.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr() as *const ::core::ffi::c_char,
                509 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"groups_hashtab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr() as *const ::core::ffi::c_char,
                509 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"groups_hashtab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if groups_hashtab
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut grcache
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr() as *const ::core::ffi::c_char,
                509 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"groups_hashtab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/getgroups.c\0".as_ptr() as *const ::core::ffi::c_char,
                509 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"groups_hashtab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_1,
            );
            abort();
        }
        i = 0 as uint32_t;
        while i < HASHSIZE as uint32_t {
            *groups_hashtab.offset(i as isize) = ::core::ptr::null_mut::<grcache>();
            i = i.wrapping_add(1);
        }
        to = _to;
        keep_alive = 1 as ::core::ffi::c_int;
        pthread_create(
            &raw mut main_thread,
            ::core::ptr::null::<pthread_attr_t>(),
            Some(
                groups_cleanup_thread
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            NULL,
        );
    }
}
