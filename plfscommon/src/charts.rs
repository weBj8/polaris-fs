//! Charts (mfscommon/charts.c): per-minute statistics series kept for four
//! ranges, the binary stats file, PNG chart rendering and the raw data
//! export used by the CGI / CLI.
//!
//! Layout:
//! - `imp` (`#[deny(unsafe_code)]`): state, stats file, RPN calc
//!   definitions, data/monotonic exports;
//! - `render` (`#[deny(unsafe_code)]`): chart drawing, y-scale fitting,
//!   deflate (zlib via flate2, level 1 like `deflateInit(Z_BEST_SPEED)`)
//!   and PNG CRC patching;
//! - `sys`: libc time boundary (`time`, `localtime`, `gmtime`);
//! - the C ABI of charts.h, which only turns raw pointers into slices.
//!
//! Locking mirrors the C `glock`: the operations C ran under the mutex
//! (add, store, term, get, getdata, single-series makedata and the
//! make_png..get_png pair) serialize on a gate that `charts_make_png`
//! takes and `charts_get_png` releases; monotonic_data and multi-series
//! makedata stay unlocked as in C (they only read).
//!
//! Deviations, all where C is undefined or unobservable:
//! - an extended chart whose lower series is undefined but a higher one is
//!   (C dereferences NULL) treats the missing series as "no data";
//! - calls before `charts_init` act on an empty chart set instead of
//!   dereferencing NULL;
//! - the stats file is opened with O_CLOEXEC (std default).

#[path = "charts/imp.rs"]
mod imp;
#[path = "charts/render.rs"]
mod render;
#[path = "charts/tables.rs"]
mod tables;

use core::ffi::{CStr, c_char, c_double, c_int};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

use imp::{Charts, DEFS_END, EStatDef, MAXLENG, StatDef};
use tables::PNG_1X1;

pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

/// charts.h `statdef`.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _statdef {
    pub name: *mut c_char,
    pub statid: uint32_t,
    pub mode: uint8_t,
    pub percent: uint8_t,
    pub scale: uint8_t,
    pub multiplier: uint16_t,
    pub divisor: uint16_t,
}
pub type statdef = _statdef;

/// charts.h `estatdef`.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _estatdef {
    pub name: *mut c_char,
    pub statid: uint32_t,
    pub c1src: uint32_t,
    pub c2src: uint32_t,
    pub c3src: uint32_t,
    pub mode: uint8_t,
    pub percent: uint8_t,
    pub scale: uint8_t,
    pub multiplier: uint16_t,
    pub divisor: uint16_t,
}
pub type estatdef = _estatdef;

/// libc time boundary (the C used the non-reentrant calls; kept so TZ
/// handling, including re-reading `TZ` per call, is glibc's).
mod sys {
    /// The `struct tm` fields charts use.
    #[derive(Clone, Copy)]
    pub struct Tm {
        pub min: i32,
        pub hour: i32,
        pub mday: i32,
        pub mon: i32,
        pub year: i32,
        pub gmtoff: i64,
    }

    pub fn now() -> i64 {
        // SAFETY: time(NULL) has no pointer argument to honour.
        unsafe { libc::time(core::ptr::null_mut()) }
    }

    fn conv(f: unsafe extern "C" fn(*const libc::time_t) -> *mut libc::tm, t: i64) -> Tm {
        let t: libc::time_t = t;
        // SAFETY: `t` lives across the call; the result points at libc's
        // static buffer (or is NULL), copied out before returning.
        let p = unsafe { f(&t) };
        // SAFETY: non-NULL result is a valid `tm`.
        match unsafe { p.as_ref() } {
            Some(tm) => Tm {
                min: tm.tm_min,
                hour: tm.tm_hour,
                mday: tm.tm_mday,
                mon: tm.tm_mon,
                year: tm.tm_year,
                gmtoff: tm.tm_gmtoff,
            },
            // only for years beyond int range; C would dereference NULL
            None => Tm { min: 0, hour: 0, mday: 0, mon: 0, year: 0, gmtoff: 0 },
        }
    }

    pub fn localtime(t: i64) -> Tm {
        conv(libc::localtime, t)
    }

    pub fn gmtime(t: i64) -> Tm {
        conv(libc::gmtime, t)
    }
}

static STATE: Mutex<Option<Charts>> = Mutex::new(None);

/// C `glock`, held from `charts_make_png` until `charts_get_png`.
struct Gate {
    held: Mutex<bool>,
    cv: Condvar,
}

static GATE: Gate = Gate { held: Mutex::new(false), cv: Condvar::new() };

impl Gate {
    fn lock(&self) {
        let mut h = self.held.lock().unwrap_or_else(PoisonError::into_inner);
        while *h {
            h = self.cv.wait(h).unwrap_or_else(PoisonError::into_inner);
        }
        *h = true;
    }
    fn unlock(&self) {
        *self.held.lock().unwrap_or_else(PoisonError::into_inner) = false;
        self.cv.notify_one();
    }
}

fn state() -> MutexGuard<'static, Option<Charts>> {
    STATE.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Runs `f` inside the C critical section.
fn locked<R>(f: impl FnOnce(&mut Option<Charts>) -> R) -> R {
    GATE.lock();
    let r = f(&mut state());
    GATE.unlock();
    r
}

/// # Safety
/// `calcs` is CHARTS_DEFS_END-terminated; `stats` / `estats` are
/// terminated by an entry with `divisor == 0`; every `stats` name and
/// `filename` are valid C strings (charts.h contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_init(
    calcs: *const uint32_t,
    stats: *const statdef,
    estats: *const estatdef,
    filename: *const c_char,
    mode: uint8_t,
) -> c_int {
    let mut calcv = Vec::new();
    // SAFETY: terminated array per fn contract.
    unsafe {
        let mut p = calcs;
        while *p != DEFS_END {
            calcv.push(*p);
            p = p.add(1);
        }
    }
    let mut statv = Vec::new();
    // SAFETY: terminated array with valid names per fn contract.
    unsafe {
        let mut p = stats;
        while (*p).divisor != 0 {
            let s = &*p;
            statv.push(StatDef {
                name: CStr::from_ptr(s.name).to_bytes().to_vec(),
                statid: s.statid,
                mode: s.mode,
                percent: s.percent,
                scale: s.scale,
                multiplier: s.multiplier,
                divisor: s.divisor,
            });
            p = p.add(1);
        }
    }
    let mut estatv = Vec::new();
    // SAFETY: terminated array per fn contract.
    unsafe {
        let mut p = estats;
        while (*p).divisor != 0 {
            let e = &*p;
            estatv.push(EStatDef {
                statid: e.statid,
                c1src: e.c1src,
                c2src: e.c2src,
                c3src: e.c3src,
                mode: e.mode,
                percent: e.percent,
                scale: e.scale,
                multiplier: e.multiplier,
                divisor: e.divisor,
            });
            p = p.add(1);
        }
    }
    // SAFETY: valid C string per fn contract.
    let path = PathBuf::from(OsStr::from_bytes(unsafe { CStr::from_ptr(filename) }.to_bytes()));
    let (c, ret) = Charts::init(&calcv, statv, estatv, path, mode);
    *state() = Some(c);
    ret
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn charts_term() {
    locked(|s| *s = None);
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn charts_store() {
    locked(|s| {
        if let Some(c) = s {
            c.store()
        }
    });
}

/// # Safety
/// `data` is NULL or points at one value per configured stat.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_add(data: *mut uint64_t, datats: uint32_t) {
    locked(|s| {
        if let Some(c) = s {
            let n = c.stat_count();
            // SAFETY: per fn contract.
            let d = (!data.is_null()).then(|| unsafe { std::slice::from_raw_parts(data, n) });
            c.add(d, datats);
        }
    });
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn charts_get(r#type: uint32_t, numb: uint32_t) -> uint64_t {
    locked(|s| s.as_ref().map_or(0, |c| c.get(r#type, numb)))
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn charts_getmaxleng() -> uint32_t {
    MAXLENG
}

/// # Safety
/// `data` (if non-NULL) is writable for `charts_getmaxleng()` doubles;
/// `timestamp` / `rsec` are NULL or writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_getdata(
    data: *mut c_double,
    timestamp: *mut uint32_t,
    rsec: *mut uint32_t,
    number: uint32_t,
) {
    if data.is_null() || timestamp.is_null() {
        return;
    }
    let Some((ts, rs, values)) = locked(|s| s.as_ref().and_then(|c| c.getdata(number))) else {
        return;
    };
    // SAFETY: per fn contract (rsec is written whenever data is, as in C).
    unsafe {
        *timestamp = ts;
        *rsec = rs;
        std::ptr::copy_nonoverlapping(values.as_ptr(), data, values.len());
    }
}

/// # Safety
/// `buff` is NULL (size query) or writable for the size a NULL call
/// returned with the same arguments.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_makedata(
    buff: *mut uint8_t,
    number: uint32_t,
    maxentries: uint32_t,
    multimode: uint8_t,
) -> uint32_t {
    let run = |s: &mut Option<Charts>| {
        let Some(c) = s.as_ref() else { return 0 };
        if buff.is_null() {
            return c.makedata(None, number, maxentries, multimode != 0);
        }
        let len = c.makedata(None, number, maxentries, multimode != 0) as usize;
        // SAFETY: per fn contract the buffer holds the queried size.
        let out = unsafe { std::slice::from_raw_parts_mut(buff, len) };
        c.makedata(Some(out), number, maxentries, multimode != 0)
    };
    if multimode != 0 { run(&mut state()) } else { locked(run) }
}

/// # Safety
/// `buff` is NULL (size query) or writable for the returned size.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_monotonic_data(buff: *mut uint8_t) -> uint32_t {
    let s = state();
    let Some(c) = s.as_ref() else {
        if buff.is_null() {
            return 2;
        }
        // SAFETY: per fn contract (2 bytes: zero stat count).
        unsafe { std::ptr::write_bytes(buff, 0, 2) };
        return 0;
    };
    if buff.is_null() {
        return c.monotonic_data(None);
    }
    let len = c.monotonic_data(None) as usize;
    // SAFETY: per fn contract.
    c.monotonic_data(Some(unsafe { std::slice::from_raw_parts_mut(buff, len) }))
}

/// Takes the gate; `charts_get_png` releases it.
// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn charts_make_png(chartid: uint32_t, chartwidth: uint32_t, chartheight: uint32_t) -> uint32_t {
    GATE.lock();
    match state().as_mut() {
        Some(c) => c.make_png(chartid, chartwidth, chartheight),
        None => PNG_1X1.len() as u32,
    }
}

/// # Safety
/// `buff` is writable for the size the preceding `charts_make_png` returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_get_png(buff: *mut uint8_t) {
    {
        let mut s = state();
        match s.as_mut() {
            Some(c) => {
                let len = c.png_len() as usize;
                // SAFETY: per fn contract.
                c.get_png(unsafe { std::slice::from_raw_parts_mut(buff, len) });
            }
            // SAFETY: per fn contract (make_png returned the 1x1 size).
            None => unsafe { std::ptr::copy_nonoverlapping(PNG_1X1.as_ptr(), buff, PNG_1X1.len()) },
        }
    }
    GATE.unlock();
}
