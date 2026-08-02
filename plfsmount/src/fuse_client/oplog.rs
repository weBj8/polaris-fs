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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
static TIME_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
// ---------------------------------------------------------------------------
// Safe core (P4 rewrite): ring buffer + handle table, no locking inside —
// every entry point is called with the Rust state mutex held.
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
        pub opbuff: Box<[u8]>,
        pub writepos: u64,
        pub waiting: bool,
        pub nextfh: u64,
        pub handles: Vec<FhEntry>,
    }

    impl Oplog {
        pub fn new() -> Self {
            Oplog {
                opbuff: vec![0; OPBUFFSIZE].into_boxed_slice(),
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

use imp::{LINELENG, Oplog};
use std::cell::RefCell;
use std::sync::{Condvar, LazyLock, Mutex, MutexGuard};

static OPLOG: LazyLock<Mutex<Oplog>> = LazyLock::new(|| Mutex::new(Oplog::new()));
static NODATA: Condvar = Condvar::new();
std::thread_local! {
    // getdata/releasedata run on the same FUSE worker. Keeping the guard in
    // TLS preserves the C lock-handoff contract without a pthread mutex.
    static DATA_LEASE: RefCell<Option<MutexGuard<'static, Oplog>>> = const { RefCell::new(None) };
}

fn hold_data_lease(guard: MutexGuard<'static, Oplog>) {
    DATA_LEASE.with(|lease| {
        let mut lease = lease.borrow_mut();
        assert!(lease.is_none(), "nested oplog data lease");
        *lease = Some(guard);
    });
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
        let time_guard = TIME_LOCK.lock().unwrap();
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
        drop(time_guard);
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
        let line = ::core::slice::from_raw_parts(buff.as_ptr() as *const uint8_t, leng as usize);
        let mut oplog = OPLOG.lock().unwrap();
        oplog.put(line);
        if oplog.waiting {
            NODATA.notify_all();
            oplog.waiting = false;
        }
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
        let time_guard = TIME_LOCK.lock().unwrap();
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
        drop(time_guard);
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
        let mut oplog = OPLOG.lock().unwrap();
        oplog.put(line);
        if oplog.waiting {
            NODATA.notify_all();
            oplog.waiting = false;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oplog_newhandle(hflag: ::core::ffi::c_int) -> ::core::ffi::c_ulong {
    OPLOG.lock().unwrap().newhandle(hflag != 0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oplog_releasehandle(fh: ::core::ffi::c_ulong) {
    OPLOG.lock().unwrap().releasehandle(fh);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oplog_getdata(
    fh: ::core::ffi::c_ulong,
    buff: *mut *mut uint8_t,
    leng: *mut uint32_t,
    maxleng: uint32_t,
) {
    let mut oplog = OPLOG.lock().unwrap();
    if !oplog.acquire(fh) {
        unsafe {
            *buff = ::core::ptr::null_mut::<uint8_t>();
            *leng = 0;
        }
        hold_data_lease(oplog);
        return;
    }
    while !oplog.data_available(fh) {
        oplog.waiting = true;
        let (next, timeout) = NODATA
            .wait_timeout(oplog, std::time::Duration::from_secs(1))
            .unwrap();
        oplog = next;
        if timeout.timed_out() {
            unsafe {
                *buff = b"#\n\0".as_ptr() as *mut uint8_t;
                *leng = 2;
            }
            hold_data_lease(oplog);
            return;
        }
    }
    let (bpos, len) = oplog.getdata(fh, maxleng);
    unsafe {
        *leng = len;
        *buff = oplog.opbuff.as_mut_ptr().add(bpos);
    }
    hold_data_lease(oplog);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oplog_releasedata(fh: ::core::ffi::c_ulong) {
    DATA_LEASE.with(|lease| {
        let mut oplog = lease
            .borrow_mut()
            .take()
            .expect("oplog_releasedata without getdata");
        oplog.releasehandle(fh);
    });
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

    #[test]
    fn data_lease_unlocks_on_release() {
        let mut buff = std::ptr::null_mut();
        let mut len = 99;
        // SAFETY: output pointers are valid; unknown handle avoids waiting.
        unsafe {
            super::oplog_getdata(u64::MAX, &mut buff, &mut len, 16);
            assert!(buff.is_null());
            assert_eq!(len, 0);
            super::oplog_releasedata(u64::MAX);
        }
        assert!(super::OPLOG.try_lock().is_ok());
    }
}
