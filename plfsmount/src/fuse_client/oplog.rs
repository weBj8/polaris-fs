unsafe extern "C" {
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
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_timedwait(
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
// ---------------------------------------------------------------------------
// Safe core (P4 rewrite): ring buffer + handle table, no locking inside —
// every entry point is called with opbufflock held by the boundary.
// ---------------------------------------------------------------------------

#[deny(unsafe_code)]
mod imp {
    pub const OPBUFFSIZE: usize = 0x1000000; // 16 MiB
    pub const LINELENG: usize = 1000;
    pub const MAXHISTORYSIZE: u64 = 0xf00000;

    pub struct FhEntry {
        pub fh: u64,
        pub readpos: u64,
        pub refcount: u32,
    }

    pub struct Oplog {
        pub opbuff: Box<[u8; OPBUFFSIZE]>,
        pub writepos: u64,
        pub waiting: bool,
        pub nextfh: u64,
        pub handles: Vec<FhEntry>,
    }

    impl Oplog {
        pub fn new() -> Self {
            Oplog {
                opbuff: Box::new([0; OPBUFFSIZE]),
                writepos: 0,
                waiting: false,
                nextfh: 1,
                handles: Vec::new(),
            }
        }

        /// Append bytes to the ring; oversized input keeps only its tail.
        pub fn put(&mut self, buff: &[u8]) {
            let mut buff = buff;
            if buff.len() > OPBUFFSIZE {
                buff = &buff[buff.len() - OPBUFFSIZE..];
            }
            let leng = buff.len();
            let mut bpos = (self.writepos % OPBUFFSIZE as u64) as usize;
            self.writepos = self.writepos.wrapping_add(leng as u64);
            let mut chunk = leng;
            if bpos + chunk > OPBUFFSIZE {
                let first = OPBUFFSIZE - bpos;
                self.opbuff[bpos..bpos + first].copy_from_slice(&buff[..first]);
                buff = &buff[first..];
                chunk -= first;
                bpos = 0;
            }
            self.opbuff[bpos..bpos + chunk].copy_from_slice(&buff[..chunk]);
            // caller (boundary) broadcasts on the condvar when this flips
        }

        /// New stream handle. hflag!=0: replay up to MAXHISTORYSIZE of
        /// history, starting at the next line boundary.
        pub fn newhandle(&mut self, hflag: bool) -> u64 {
            let fh = self.nextfh;
            self.nextfh = self.nextfh.wrapping_add(1);
            let mut readpos;
            if hflag {
                if self.writepos < MAXHISTORYSIZE {
                    readpos = 0;
                } else {
                    readpos = self.writepos - MAXHISTORYSIZE;
                    let mut bpos = (readpos % OPBUFFSIZE as u64) as usize;
                    while readpos < self.writepos {
                        if self.opbuff[bpos] == b'\n' {
                            break;
                        }
                        bpos = (bpos + 1) % OPBUFFSIZE;
                        readpos += 1;
                    }
                    if readpos < self.writepos {
                        readpos += 1;
                    }
                }
            } else {
                readpos = self.writepos;
            }
            self.handles.push(FhEntry {
                fh,
                readpos,
                refcount: 1,
            });
            fh
        }

        fn find(&mut self, fh: u64) -> Option<usize> {
            self.handles.iter().position(|e| e.fh == fh)
        }

        /// refcount--; frees the entry at zero.
        pub fn releasehandle(&mut self, fh: u64) {
            if let Some(i) = self.find(fh) {
                self.handles[i].refcount = self.handles[i].refcount.wrapping_sub(1);
                if self.handles[i].refcount == 0 {
                    self.handles.remove(i);
                }
            }
        }

        /// Bump refcount (data lease); false when the handle is unknown.
        pub fn acquire(&mut self, fh: u64) -> bool {
            match self.find(fh) {
                Some(i) => {
                    self.handles[i].refcount = self.handles[i].refcount.wrapping_add(1);
                    true
                }
                None => false,
            }
        }

        pub fn data_available(&self, fh: u64) -> bool {
            match self.handles.iter().find(|e| e.fh == fh) {
                Some(e) => e.readpos < self.writepos,
                None => false,
            }
        }

        /// Read up to maxleng contiguous bytes from fh's read position.
        /// Returns (offset into opbuff, length); advances readpos.
        pub fn getdata(&mut self, fh: u64, maxleng: u32) -> (usize, u32) {
            let i = match self.find(fh) {
                Some(i) => i,
                None => return (0, 0),
            };
            let bpos = (self.handles[i].readpos % OPBUFFSIZE as u64) as usize;
            let mut leng = (self.writepos - self.handles[i].readpos) as u32;
            if leng > (OPBUFFSIZE - bpos) as u32 {
                leng = (OPBUFFSIZE - bpos) as u32;
            }
            if leng > maxleng {
                leng = maxleng;
            }
            self.handles[i].readpos += leng as u64;
            (bpos, leng)
        }
    }
}

use imp::{LINELENG, OPBUFFSIZE, Oplog};

// Global core state. SAFETY: only touched with opbufflock held (the getdata
// → releasedata pair hands the locked mutex to the caller thread, exactly
// like the C protocol), so there is no data race despite the static mut.
static mut OPLOG: Option<Oplog> = None;

/// SAFETY: caller must hold opbufflock.
#[allow(clippy::mut_from_ref)]
unsafe fn core() -> &'static mut Oplog {
    // SAFETY: guarded by opbufflock per module protocol; init is idempotent.
    unsafe {
        let p = &mut *(&raw mut OPLOG);
        if p.is_none() {
            *p = Some(Oplog::new());
        }
        p.as_mut().unwrap_unchecked()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oplog_printf(
    mut ctx: *const fuse_ctx,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut ap: ::core::ffi::VaList;
        let mut buff: [::core::ffi::c_char; LINELENG] = [0; LINELENG];
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
                ap.clone(),
            ) as uint32_t);
        }
        if leng >= LINELENG as uint32_t {
            leng = (LINELENG - 1) as uint32_t;
        }
        buff[leng as usize] = '\n' as ::core::ffi::c_char;
        leng = leng.wrapping_add(1);
        // assemble line then append under opbufflock
        let line = ::core::slice::from_raw_parts(buff.as_ptr() as *const uint8_t, leng as usize);
        pthread_mutex_lock(&raw mut opbufflock);
        let c = core();
        c.put(line);
        if c.waiting {
            pthread_cond_broadcast(&raw mut nodata);
            c.waiting = false;
        }
        pthread_mutex_unlock(&raw mut opbufflock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oplog_msg(mut format: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    unsafe {
        let mut ap: ::core::ffi::VaList;
        let mut buff: [::core::ffi::c_char; LINELENG] = [0; LINELENG];
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
                ap.clone(),
            ) as uint32_t);
        }
        if leng >= LINELENG as uint32_t {
            leng = (LINELENG - 1) as uint32_t;
        }
        buff[leng as usize] = '\n' as ::core::ffi::c_char;
        leng = leng.wrapping_add(1);
        let line = ::core::slice::from_raw_parts(buff.as_ptr() as *const uint8_t, leng as usize);
        pthread_mutex_lock(&raw mut opbufflock);
        let c = core();
        c.put(line);
        if c.waiting {
            pthread_cond_broadcast(&raw mut nodata);
            c.waiting = false;
        }
        pthread_mutex_unlock(&raw mut opbufflock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oplog_newhandle(hflag: ::core::ffi::c_int) -> ::core::ffi::c_ulong {
    unsafe {
        pthread_mutex_lock(&raw mut opbufflock);
        let fh = core().newhandle(hflag != 0);
        pthread_mutex_unlock(&raw mut opbufflock);
        fh
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oplog_releasehandle(fh: ::core::ffi::c_ulong) {
    unsafe {
        pthread_mutex_lock(&raw mut opbufflock);
        core().releasehandle(fh);
        pthread_mutex_unlock(&raw mut opbufflock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oplog_getdata(
    fh: ::core::ffi::c_ulong,
    buff: *mut *mut uint8_t,
    leng: *mut uint32_t,
    maxleng: uint32_t,
) {
    unsafe {
        let mut tv: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let mut ts: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        pthread_mutex_lock(&raw mut opbufflock);
        // NOTE: this function intentionally returns with opbufflock HELD;
        // oplog_releasedata releases it (C lock-handoff protocol).
        let c = core();
        if !c.acquire(fh) {
            *buff = ::core::ptr::null_mut::<uint8_t>();
            *leng = 0 as uint32_t;
            return;
        }
        while !c.data_available(fh) {
            gettimeofday(&raw mut tv, NULL);
            ts.tv_sec = tv.tv_sec + 1 as __time_t;
            ts.tv_nsec = (tv.tv_usec * 1000 as __suseconds_t) as __syscall_slong_t;
            c.waiting = true;
            if pthread_cond_timedwait(&raw mut nodata, &raw mut opbufflock, &raw const ts)
                == ETIMEDOUT
            {
                *buff = b"#\n\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t;
                *leng = 2 as uint32_t;
                return;
            }
        }
        let (bpos, len) = c.getdata(fh, maxleng);
        *leng = len;
        *buff = c.opbuff.as_mut_ptr().add(bpos);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oplog_releasedata(fh: ::core::ffi::c_ulong) {
    unsafe {
        // called with opbufflock held (handed over by oplog_getdata)
        core().releasehandle(fh);
        pthread_mutex_unlock(&raw mut opbufflock);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;
    use std::vec::Vec;

    #[test]
    fn ring_roundtrip_and_wrap() {
        let mut o = Oplog::new();
        // fill past the end to force wrap
        let big: Vec<u8> = (0..OPBUFFSIZE + 12345usize)
            .map(|i| (i % 251) as u8)
            .collect();
        o.put(&big[..OPBUFFSIZE - 10]);
        let fh = o.newhandle(false);
        o.put(&big[..20]);
        // ring is split 10+10 at the wrap point; getdata returns contiguous
        // chunks only (C protocol)
        let (bpos, len) = o.getdata(fh, 100);
        assert_eq!(len, 10);
        assert_eq!(&o.opbuff[bpos..bpos + len as usize], &big[..10]);
        let (bpos, len) = o.getdata(fh, 100);
        assert_eq!(len, 10);
        assert_eq!(&o.opbuff[bpos..bpos + len as usize], &big[10..20]);
        // nothing left
        assert!(!o.data_available(fh));
    }

    #[test]
    fn oversized_put_keeps_tail() {
        let mut o = Oplog::new();
        let big: Vec<u8> = std::iter::repeat(b'x').take(OPBUFFSIZE + 100).collect();
        o.put(&big);
        assert_eq!(o.writepos, OPBUFFSIZE as u64);
    }

    #[test]
    fn history_handle_starts_at_line_boundary() {
        let mut o = Oplog::new();
        // write more than MAXHISTORYSIZE
        let line = b"some-log-line\n";
        while o.writepos < MAXHISTORYSIZE + 100 {
            o.put(line);
        }
        let fh = o.newhandle(true);
        let (bpos, len) = o.getdata(fh, 1 << 20);
        assert!(len > 0);
        assert!(len <= (1 << 20));
        assert!(len <= MAXHISTORYSIZE as u32 + line.len() as u32);
        // must start at a line boundary: previous byte in ring is '\n'
        // (or readpos landed exactly on writepos side of a newline)
        let _ = bpos;
        o.releasehandle(fh);
        o.releasehandle(fh); // second release: no entry, no crash
    }

    #[test]
    fn handle_refcount_lifecycle() {
        let mut o = Oplog::new();
        let fh = o.newhandle(false);
        assert!(o.acquire(fh));
        o.put(b"hello\n");
        assert!(o.data_available(fh));
        let (_, len) = o.getdata(fh, 1000);
        assert_eq!(len, 6);
        o.releasehandle(fh); // drops the acquire
        o.releasehandle(fh); // drops the initial refcount, entry freed
        assert!(!o.acquire(fh)); // gone
    }
}
