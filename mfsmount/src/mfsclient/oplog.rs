extern "C" {
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
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
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __mode_t = ::core::ffi::c_uint;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type va_list = __gnuc_va_list;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
    pub __unused_initialized_1: ::core::ffi::c_uint,
    pub __unused_initialized_2: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2Rust_Unnamed_0 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2Rust_Unnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2Rust_Unnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2Rust_Unnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2Rust_Unnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2Rust_Unnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2Rust_Unnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2Rust_Unnamed_0 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2Rust_Unnamed_0 = 0;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_ctx {
    pub uid: uid_t,
    pub gid: gid_t,
    pub pid: pid_t,
    pub umask: mode_t,
}
pub type fhentry = _fhentry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fhentry {
    pub fh: ::core::ffi::c_ulong,
    pub readpos: uint64_t,
    pub refcount: uint32_t,
    pub next: *mut _fhentry,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __data: __pthread_cond_s {
        __wseq: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_ulonglong,
        },
        __g1_start: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_ulonglong,
        },
        __g_size: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        __g1_orig_size: 0 as ::core::ffi::c_uint,
        __wrefs: 0 as ::core::ffi::c_uint,
        __g_signals: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        __unused_initialized_1: 0 as ::core::ffi::c_uint,
        __unused_initialized_2: 0 as ::core::ffi::c_uint,
    },
};
pub const OPBUFFSIZE: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const LINELENG: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const MAXHISTORYSIZE: ::core::ffi::c_int = 0xf00000 as ::core::ffi::c_int;
static mut nextfh: ::core::ffi::c_ulong = 1 as ::core::ffi::c_ulong;
static mut fhhead: *mut fhentry = ::core::ptr::null_mut::<fhentry>();
static mut opbuff: [uint8_t; 16777216] = [0; 16777216];
static mut writepos: uint64_t = 0 as uint64_t;
static mut waiting: uint8_t = 0 as uint8_t;
static mut opbufflock: pthread_mutex_t = pthread_mutex_t {
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
static mut nodata: pthread_cond_t = PTHREAD_COND_INITIALIZER;
static mut convts: time_t = 0 as time_t;
static mut convtm: tm = tm {
    tm_sec: 0,
    tm_min: 0,
    tm_hour: 0,
    tm_mday: 0,
    tm_mon: 0,
    tm_year: 0,
    tm_wday: 0,
    tm_yday: 0,
    tm_isdst: 0,
    tm_gmtoff: 0,
    tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
};
static mut timelock: pthread_mutex_t = pthread_mutex_t {
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
#[inline]
unsafe extern "C" fn oplog_put(mut buff: *mut uint8_t, mut leng: uint32_t) {
    let mut bpos: uint32_t = 0;
    if leng > OPBUFFSIZE as uint32_t {
        buff = buff.offset(leng.wrapping_sub(OPBUFFSIZE as uint32_t) as isize);
        leng = OPBUFFSIZE as uint32_t;
    }
    pthread_mutex_lock(&raw mut opbufflock);
    bpos = writepos.wrapping_rem(OPBUFFSIZE as uint64_t) as uint32_t;
    writepos = writepos.wrapping_add(leng as uint64_t);
    if bpos.wrapping_add(leng) > OPBUFFSIZE as uint32_t {
        memcpy(
            (&raw mut opbuff as *mut uint8_t).offset(bpos as isize) as *mut ::core::ffi::c_void,
            buff as *const ::core::ffi::c_void,
            (OPBUFFSIZE as uint32_t).wrapping_sub(bpos) as size_t,
        );
        buff = buff.offset((OPBUFFSIZE as uint32_t).wrapping_sub(bpos) as isize);
        leng = leng.wrapping_sub((OPBUFFSIZE as uint32_t).wrapping_sub(bpos));
        bpos = 0 as uint32_t;
    }
    memcpy(
        (&raw mut opbuff as *mut uint8_t).offset(bpos as isize) as *mut ::core::ffi::c_void,
        buff as *const ::core::ffi::c_void,
        leng as size_t,
    );
    if waiting != 0 {
        pthread_cond_broadcast(&raw mut nodata);
        waiting = 0 as uint8_t;
    }
    pthread_mutex_unlock(&raw mut opbufflock);
}
#[no_mangle]
pub unsafe extern "C" fn oplog_printf(
    mut ctx: *const fuse_ctx,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buff: [::core::ffi::c_char; 1000] = [0; 1000];
    let mut leng: uint32_t = 0;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ltime: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    pthread_mutex_lock(&raw mut timelock);
    gettimeofday(&raw mut tv, NULL);
    if convts / 900 as time_t != tv.tv_sec / 900 as __time_t {
        convts = (tv.tv_sec / 900 as __time_t) as time_t;
        convts *= 900 as time_t;
        localtime_r(&raw mut convts, &raw mut convtm);
    }
    ltime = convtm;
    leng = (tv.tv_sec as time_t - convts) as uint32_t;
    ltime.tm_sec = (ltime.tm_sec as uint32_t).wrapping_add(leng.wrapping_rem(60 as uint32_t))
        as ::core::ffi::c_int;
    ltime.tm_min = (ltime.tm_min as uint32_t).wrapping_add(leng.wrapping_div(60 as uint32_t))
        as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut timelock);
    leng = snprintf(
        &raw mut buff as *mut ::core::ffi::c_char,
        LINELENG as size_t,
        b"%02u.%02u %02u:%02u:%02u.%06u: uid:%u gid:%u pid:%u cmd:\0".as_ptr()
            as *const ::core::ffi::c_char,
        ltime.tm_mon + 1 as ::core::ffi::c_int,
        ltime.tm_mday,
        ltime.tm_hour,
        ltime.tm_min,
        ltime.tm_sec,
        tv.tv_usec as ::core::ffi::c_uint,
        (*ctx).uid,
        (*ctx).gid,
        (*ctx).pid as ::core::ffi::c_uint,
    ) as uint32_t;
    if leng < LINELENG as uint32_t {
        ap = c2rust_args.clone();
        leng = leng.wrapping_add(vsnprintf(
            (&raw mut buff as *mut ::core::ffi::c_char).offset(leng as isize),
            (LINELENG as uint32_t).wrapping_sub(leng) as size_t,
            format,
            ap.as_va_list(),
        ) as uint32_t);
    }
    if leng >= LINELENG as uint32_t {
        leng = (LINELENG - 1 as ::core::ffi::c_int) as uint32_t;
    }
    let c2rust_fresh0 = leng;
    leng = leng.wrapping_add(1);
    buff[c2rust_fresh0 as usize] = '\n' as ::core::ffi::c_char;
    oplog_put(
        &raw mut buff as *mut ::core::ffi::c_char as *mut uint8_t,
        leng,
    );
}
#[no_mangle]
pub unsafe extern "C" fn oplog_msg(mut format: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buff: [::core::ffi::c_char; 1000] = [0; 1000];
    let mut leng: uint32_t = 0;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ltime: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    pthread_mutex_lock(&raw mut timelock);
    gettimeofday(&raw mut tv, NULL);
    if convts / 900 as time_t != tv.tv_sec / 900 as __time_t {
        convts = (tv.tv_sec / 900 as __time_t) as time_t;
        convts *= 900 as time_t;
        localtime_r(&raw mut convts, &raw mut convtm);
    }
    ltime = convtm;
    leng = (tv.tv_sec as time_t - convts) as uint32_t;
    ltime.tm_sec = (ltime.tm_sec as uint32_t).wrapping_add(leng.wrapping_rem(60 as uint32_t))
        as ::core::ffi::c_int;
    ltime.tm_min = (ltime.tm_min as uint32_t).wrapping_add(leng.wrapping_div(60 as uint32_t))
        as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut timelock);
    leng = snprintf(
        &raw mut buff as *mut ::core::ffi::c_char,
        LINELENG as size_t,
        b"%02u.%02u %02u:%02u:%02u.%06u: msg:\0".as_ptr() as *const ::core::ffi::c_char,
        ltime.tm_mon + 1 as ::core::ffi::c_int,
        ltime.tm_mday,
        ltime.tm_hour,
        ltime.tm_min,
        ltime.tm_sec,
        tv.tv_usec as ::core::ffi::c_uint,
    ) as uint32_t;
    if leng < LINELENG as uint32_t {
        ap = c2rust_args.clone();
        leng = leng.wrapping_add(vsnprintf(
            (&raw mut buff as *mut ::core::ffi::c_char).offset(leng as isize),
            (LINELENG as uint32_t).wrapping_sub(leng) as size_t,
            format,
            ap.as_va_list(),
        ) as uint32_t);
    }
    if leng >= LINELENG as uint32_t {
        leng = (LINELENG - 1 as ::core::ffi::c_int) as uint32_t;
    }
    let c2rust_fresh1 = leng;
    leng = leng.wrapping_add(1);
    buff[c2rust_fresh1 as usize] = '\n' as ::core::ffi::c_char;
    oplog_put(
        &raw mut buff as *mut ::core::ffi::c_char as *mut uint8_t,
        leng,
    );
}
#[no_mangle]
pub unsafe extern "C" fn oplog_newhandle(mut hflag: ::core::ffi::c_int) -> ::core::ffi::c_ulong {
    let mut fhptr: *mut fhentry = ::core::ptr::null_mut::<fhentry>();
    let mut bpos: uint32_t = 0;
    pthread_mutex_lock(&raw mut opbufflock);
    fhptr = malloc(::core::mem::size_of::<fhentry>()) as *mut fhentry;
    let c2rust_fresh2 = nextfh;
    nextfh = nextfh.wrapping_add(1);
    (*fhptr).fh = c2rust_fresh2;
    (*fhptr).refcount = 1 as uint32_t;
    if hflag != 0 {
        if writepos < MAXHISTORYSIZE as uint64_t {
            (*fhptr).readpos = 0 as uint64_t;
        } else {
            (*fhptr).readpos = writepos.wrapping_sub(MAXHISTORYSIZE as uint64_t);
            bpos = (*fhptr).readpos.wrapping_rem(OPBUFFSIZE as uint64_t) as uint32_t;
            while (*fhptr).readpos < writepos {
                if opbuff[bpos as usize] as ::core::ffi::c_int == '\n' as ::core::ffi::c_int {
                    break;
                }
                bpos = bpos.wrapping_add(1);
                bpos = bpos.wrapping_rem(OPBUFFSIZE as uint32_t);
                (*fhptr).readpos = (*fhptr).readpos.wrapping_add(1);
            }
            if (*fhptr).readpos < writepos {
                (*fhptr).readpos = (*fhptr).readpos.wrapping_add(1);
            }
        }
    } else {
        (*fhptr).readpos = writepos;
    }
    (*fhptr).next = fhhead as *mut _fhentry;
    fhhead = fhptr;
    pthread_mutex_unlock(&raw mut opbufflock);
    return (*fhptr).fh;
}
#[no_mangle]
pub unsafe extern "C" fn oplog_releasehandle(mut fh: ::core::ffi::c_ulong) {
    let mut fhpptr: *mut *mut fhentry = ::core::ptr::null_mut::<*mut fhentry>();
    let mut fhptr: *mut fhentry = ::core::ptr::null_mut::<fhentry>();
    pthread_mutex_lock(&raw mut opbufflock);
    fhpptr = &raw mut fhhead;
    loop {
        fhptr = *fhpptr;
        if fhptr.is_null() {
            break;
        }
        if (*fhptr).fh == fh {
            (*fhptr).refcount = (*fhptr).refcount.wrapping_sub(1);
            if (*fhptr).refcount == 0 as uint32_t {
                *fhpptr = (*fhptr).next as *mut fhentry;
                free(fhptr as *mut ::core::ffi::c_void);
            } else {
                fhpptr = &raw mut (*fhptr).next as *mut *mut fhentry;
            }
        } else {
            fhpptr = &raw mut (*fhptr).next as *mut *mut fhentry;
        }
    }
    pthread_mutex_unlock(&raw mut opbufflock);
}
#[no_mangle]
pub unsafe extern "C" fn oplog_getdata(
    mut fh: ::core::ffi::c_ulong,
    mut buff: *mut *mut uint8_t,
    mut leng: *mut uint32_t,
    mut maxleng: uint32_t,
) {
    let mut fhptr: *mut fhentry = ::core::ptr::null_mut::<fhentry>();
    let mut bpos: uint32_t = 0;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    pthread_mutex_lock(&raw mut opbufflock);
    fhptr = fhhead;
    while !fhptr.is_null() && (*fhptr).fh != fh {
        fhptr = (*fhptr).next as *mut fhentry;
    }
    if fhptr.is_null() {
        *buff = ::core::ptr::null_mut::<uint8_t>();
        *leng = 0 as uint32_t;
        return;
    }
    (*fhptr).refcount = (*fhptr).refcount.wrapping_add(1);
    while (*fhptr).readpos >= writepos {
        gettimeofday(&raw mut tv, NULL);
        ts.tv_sec = tv.tv_sec + 1 as __time_t;
        ts.tv_nsec = (tv.tv_usec * 1000 as __suseconds_t) as __syscall_slong_t;
        waiting = 1 as uint8_t;
        if pthread_cond_timedwait(&raw mut nodata, &raw mut opbufflock, &raw mut ts) == ETIMEDOUT {
            *buff = b"#\n\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t;
            *leng = 2 as uint32_t;
            return;
        }
    }
    bpos = (*fhptr).readpos.wrapping_rem(OPBUFFSIZE as uint64_t) as uint32_t;
    *leng = writepos.wrapping_sub((*fhptr).readpos) as uint32_t;
    *buff = (&raw mut opbuff as *mut uint8_t).offset(bpos as isize);
    if *leng > (OPBUFFSIZE as uint32_t).wrapping_sub(bpos) {
        *leng = (OPBUFFSIZE as uint32_t).wrapping_sub(bpos);
    }
    if *leng > maxleng {
        *leng = maxleng;
    }
    (*fhptr).readpos = (*fhptr).readpos.wrapping_add(*leng as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn oplog_releasedata(mut fh: ::core::ffi::c_ulong) {
    let mut fhpptr: *mut *mut fhentry = ::core::ptr::null_mut::<*mut fhentry>();
    let mut fhptr: *mut fhentry = ::core::ptr::null_mut::<fhentry>();
    fhpptr = &raw mut fhhead;
    loop {
        fhptr = *fhpptr;
        if fhptr.is_null() {
            break;
        }
        if (*fhptr).fh == fh {
            (*fhptr).refcount = (*fhptr).refcount.wrapping_sub(1);
            if (*fhptr).refcount == 0 as uint32_t {
                *fhpptr = (*fhptr).next as *mut fhentry;
                free(fhptr as *mut ::core::ffi::c_void);
            } else {
                fhpptr = &raw mut (*fhptr).next as *mut *mut fhentry;
            }
        } else {
            fhpptr = &raw mut (*fhptr).next as *mut *mut fhentry;
        }
    }
    pthread_mutex_unlock(&raw mut opbufflock);
}
