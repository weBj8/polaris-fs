//! Main config file handling — port of mfscommon/cfg.c.
//!
//! Shared by all daemons (the four per-daemon c2rust copies were identical
//! apart from the embedded `__FILE__` path). The parser, option registry and
//! typed getters are safe Rust in `imp`; the C ABI exports below it only
//! convert pointers and allocate the `malloc`ed results the C callers
//! `free()`.
//!
//! Behavior preserved from the C original:
//! - line grammar byte for byte, including signed-`char` comparisons (bytes
//!   >= 0x80 end names/values), trailing-space-only trimming, comment only
//!   at column 0, duplicate-variable warning, `DANGEROUS_` detection;
//! - a failed reload keeps the previous parameters; a read error mid-file
//!   ends parsing like `getline` returning -1;
//! - getters record the effective value (`cfg_info` order = first use),
//!   use the C default formatting (`%hhd`, `%.6lf`, ...) truncated to 999
//!   bytes, and convert with glibc `strto*` semantics including the
//!   narrowing casts and the `errno = ERANGE` side effect;
//! - period getters return whatever `parse_*period` left in `*ret`, even
//!   for a bad default, narrowed to `uint16_t` for hperiod.
//!
//! Intentional non-differences: C leaks the previous lists/filename on
//! `cfg_load`; Rust frees them (not observable). Internal buffer OOM aborts
//! through Rust's allocator handler instead of `passert` (both abort).

use core::ffi::{CStr, c_char, c_double, c_int};

use crate::mfslog::oom_abort;

pub use imp::{
    ConvErr, dangerous_options, default_file, default_file_md5, default_str, get_double,
    get_hperiod, get_int8, get_int16, get_int32, get_int64, get_num, get_speriod, get_str,
    get_uint8, get_uint16, get_uint32, get_uint64, info_text, is_defined, load, reload, term,
    use_option,
};

/// C `__FILE__` for cfg.c in the upstream automake build.
const CFG_C: &str = "../mfscommon/cfg.c";

#[deny(unsafe_code)]
mod imp {
    use std::ffi::OsStr;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
    use std::os::unix::ffi::OsStrExt;
    use std::sync::{Mutex, MutexGuard};

    use crate::cnum;
    use crate::md5::{md5_final_imp, md5_init_imp, md5_update_imp, md5ctx};
    use crate::mfslog::{
        MFSLOG_ERR, MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_NOTICE, MFSLOG_SYSLOG_STDERR,
        MFSLOG_WARNING, log_bytes_errno,
    };
    use crate::timeparser::{TPARSE_UNEXPECTED_CHAR, TPARSE_VALUE_TOO_BIG, parse_period};

    struct Param {
        name: Vec<u8>,
        value: Vec<u8>,
    }

    struct State {
        cfgfname: Option<Vec<u8>>,
        params: Vec<Param>,
        used: Vec<Param>,
        logundefined: bool,
        dangerous: bool,
    }

    static STATE: Mutex<State> = Mutex::new(State {
        cfgfname: None,
        params: Vec::new(),
        used: Vec::new(),
        logundefined: false,
        dangerous: false,
    });

    /// Log records produced while the state lock is held; emitted after
    /// unlocking so a log sink may call back into cfg.
    #[derive(Default)]
    struct Logs(Vec<(i32, i32, Vec<u8>, i32)>);

    impl Logs {
        fn push(&mut self, mode: i32, pri: i32, text: Vec<u8>) {
            self.0.push((mode, pri, text, 0));
        }
        fn push_errno(&mut self, mode: i32, pri: i32, text: Vec<u8>, errno: i32) {
            self.0.push((mode, pri, text, errno));
        }
        fn emit(self) {
            for (mode, pri, text, errno) in self.0 {
                log_bytes_errno(mode, pri, &text, errno);
            }
        }
    }

    fn lock() -> MutexGuard<'static, State> {
        STATE.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn cat(parts: &[&[u8]]) -> Vec<u8> {
        parts.concat()
    }

    /// C `%s` of a buffer that may hold embedded NULs.
    fn cstr_prefix(b: &[u8]) -> &[u8] {
        &b[..b.iter().position(|&c| c == 0).unwrap_or(b.len())]
    }

    fn open(path: &[u8]) -> std::io::Result<File> {
        File::open(OsStr::from_bytes(path))
    }

    /// C `fseek(END); ftell; fseek(SET)`: an unseekable stream makes ftell
    /// return -1, i.e. `(unsigned long)-1`.
    fn file_size(f: &mut File) -> u64 {
        let size = f.seek(SeekFrom::End(0)).unwrap_or(u64::MAX);
        let _ = f.seek(SeekFrom::Start(0));
        size
    }

    /// C `fread(buf,1,n,fd)==n`: read exactly `n` bytes or fail.
    fn read_n(f: &mut File, n: u64) -> Option<Vec<u8>> {
        let mut buf = Vec::new();
        f.take(n).read_to_end(&mut buf).ok()?;
        (buf.len() as u64 == n).then_some(buf)
    }

    /// Iterate `getline` lines until EOF or read error.
    fn for_each_line(f: File, mut cb: impl FnMut(&[u8])) {
        let mut r = BufReader::new(f);
        let mut line = Vec::new();
        loop {
            line.clear();
            match r.read_until(b'\n', &mut line) {
                Ok(0) | Err(_) => break,
                Ok(_) => cb(&line),
            }
        }
    }

    /// Signed-char view of a line byte (0 past the end, like the NUL).
    fn sc(line: &[u8], i: usize) -> i8 {
        line.get(i).copied().unwrap_or(0) as i8
    }

    fn reload_locked(st: &mut State, logs: &mut Logs) -> bool {
        let fname = st.cfgfname.clone();
        let shown = fname.clone().unwrap_or_else(|| b"(null)".to_vec());
        let file = match &fname {
            Some(p) => open(p),
            // C: fopen(NULL) fails with EFAULT
            None => Err(std::io::Error::from_raw_os_error(libc::EFAULT)),
        };
        let file = match file {
            Ok(f) => f,
            Err(e) => {
                let errno = e.raw_os_error().unwrap_or(0);
                if errno == libc::ENOENT {
                    logs.push(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        cat(&[b"main config file (", &shown, b") not found"]),
                    );
                } else {
                    logs.push_errno(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        cat(&[b"can't load main config file (", &shown, b"), error"]),
                        errno,
                    );
                }
                return false;
            }
        };
        st.params.clear();
        for_each_line(file, |line| parse_line(st, logs, &shown, line));
        true
    }

    fn parse_line(st: &mut State, logs: &mut Logs, fname: &[u8], line: &[u8]) {
        let at = |i: usize| sc(line, i);
        let bad = |logs: &mut Logs| {
            logs.push(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                cat(&[b"bad definition in config file '", fname, b"': ", cstr_prefix(line)]),
            );
        };
        if at(0) == b'#' as i8 {
            return;
        }
        let blank = |i: &mut usize| {
            while at(*i) == b' ' as i8 || at(*i) == b'\t' as i8 {
                *i += 1;
            }
        };
        let mut i = 0usize;
        blank(&mut i);
        let nps = i;
        while at(i) > 32 && at(i) < 127 && at(i) != b'=' as i8 {
            i += 1;
        }
        let npe = i;
        blank(&mut i);
        if at(i) != b'=' as i8 || npe == nps {
            if at(i) > 32 {
                bad(logs);
            }
            return;
        }
        i += 1;
        blank(&mut i);
        let vps = i;
        while at(i) >= 32 {
            i += 1;
        }
        while i > vps && at(i - 1) == 32 {
            i -= 1;
        }
        let vpe = i;
        blank(&mut i);
        let t = at(i);
        if t != 0 && t != b'\r' as i8 && t != b'\n' as i8 && t != b'#' as i8 {
            bad(logs);
            return;
        }
        let name = &line[nps..npe];
        let value = &line[vps..vpe];
        if name.starts_with(b"DANGEROUS_") {
            st.dangerous = true;
        }
        if let Some(p) = st.params.iter_mut().find(|p| p.name == name) {
            logs.push(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                cat(&[
                    b"variable '",
                    &p.name,
                    b"' defined more than once in the config file (previous value: ",
                    &p.value,
                    b", current value: ",
                    value,
                    b")",
                ]),
            );
            p.value = value.to_vec();
        } else {
            st.params.push(Param { name: name.to_vec(), value: value.to_vec() });
        }
    }

    fn use_option_locked(st: &mut State, name: &[u8], value: &[u8]) {
        if let Some(p) = st.used.iter_mut().find(|p| p.name == name) {
            p.value = value.to_vec();
        } else {
            st.used.push(Param { name: name.to_vec(), value: value.to_vec() });
        }
    }

    fn find<'a>(list: &'a [Param], name: &[u8]) -> Option<&'a Param> {
        list.iter().find(|p| p.name == name)
    }

    /// C: `cfg_load` (returns `cfg_reload()`).
    pub fn load(fname: &[u8], logundefined: bool) -> bool {
        let mut logs = Logs::default();
        let ok = {
            let mut st = lock();
            st.params.clear();
            st.used.clear();
            st.logundefined = logundefined;
            st.dangerous = false;
            st.cfgfname = Some(fname.to_vec());
            reload_locked(&mut st, &mut logs)
        };
        logs.emit();
        ok
    }

    /// C: `cfg_reload`.
    pub fn reload() -> bool {
        let mut logs = Logs::default();
        let ok = reload_locked(&mut lock(), &mut logs);
        logs.emit();
        ok
    }

    /// C: `cfg_term` (flags keep their values, as in C).
    pub fn term() {
        let mut st = lock();
        st.params.clear();
        st.used.clear();
        st.cfgfname = None;
    }

    /// C: `cfg_dangerous_options`.
    pub fn dangerous_options() -> bool {
        lock().dangerous
    }

    /// C: `cfg_isdefined` (file parameters only).
    pub fn is_defined(name: &[u8]) -> bool {
        find(&lock().params, name).is_some()
    }

    /// C: `cfg_use_option`.
    pub fn use_option(name: &[u8], value: &[u8]) {
        use_option_locked(&mut lock(), name, value);
    }

    /// C: `cfg_info` output text.
    pub fn info_text() -> Vec<u8> {
        let st = lock();
        let mut out = b"[config]\n".to_vec();
        for p in &st.used {
            out.extend_from_slice(&cat(&[&p.name, b" = ", &p.value, b"\n"]));
        }
        out.push(b'\n');
        out
    }

    /// Value of a file parameter, else of a used option (C `cfg_getdefault*`).
    fn default_value(name: &[u8]) -> Option<Vec<u8>> {
        let st = lock();
        find(&st.params, name).or_else(|| find(&st.used, name)).map(|p| p.value.clone())
    }

    /// C: `cfg_getdefaultstr`.
    pub fn default_str(name: &[u8]) -> Option<Vec<u8>> {
        default_value(name)
    }

    /// C: `cfg_getdefaultfile` — whole file contents when size <= maxleng.
    pub fn default_file(name: &[u8], maxleng: u32) -> Option<Vec<u8>> {
        let path = default_value(name)?;
        let mut f = open(&path).ok()?;
        let fsize = file_size(&mut f);
        if fsize > maxleng as u64 {
            return None;
        }
        read_n(&mut f, fsize)
    }

    /// C: `cfg_getdefaultfilemd5` — Err(()) is the C `-1`.
    pub fn default_file_md5(name: &[u8], txtmode: bool) -> Result<[u8; 16], ()> {
        let path = default_value(name).ok_or(())?;
        let mut f = open(&path).map_err(|_| ())?;
        let mut ctx = md5ctx { state: [0; 4], count: [0; 2], buffer: [0; 64] };
        md5_init_imp(&mut ctx);
        if txtmode {
            for_each_line(f, |line| {
                let line = cstr_prefix(line);
                let mut s = line.len();
                while s > 0 && matches!(line[s - 1], b'\r' | b'\n' | b'\t' | b' ') {
                    s -= 1;
                }
                if s > 0 {
                    let mut p = 0;
                    while matches!(line[p], b' ' | b'\t') {
                        p += 1;
                    }
                    if line[p] != b'#' {
                        md5_update_imp(&mut ctx, &line[p..s]);
                    }
                }
            });
        } else {
            let mut fsize = file_size(&mut f);
            while fsize > 65536 {
                let chunk = read_n(&mut f, 65536).ok_or(())?;
                md5_update_imp(&mut ctx, &chunk);
                fsize -= 65536;
            }
            if fsize > 0 {
                let chunk = read_n(&mut f, fsize).ok_or(())?;
                md5_update_imp(&mut ctx, &chunk);
            }
        }
        let mut digest = [0u8; 16];
        md5_final_imp(&mut digest, &mut ctx);
        Ok(digest)
    }

    /// C `snprintf(usedvalue,1000,...)`: at most 999 bytes kept.
    fn clip999(v: &[u8]) -> &[u8] {
        &v[..v.len().min(999)]
    }

    /// Shared body of `_CONFIG_GEN_FUNCTION`: Some(file value) if defined,
    /// else records/logs the formatted default and returns None.
    fn lookup(name: &[u8], def_text: &[u8]) -> Option<Vec<u8>> {
        let mut logs = Logs::default();
        let found = {
            let mut st = lock();
            match find(&st.params, name).map(|p| p.value.clone()) {
                Some(v) => {
                    use_option_locked(&mut st, name, &v);
                    Some(v)
                }
                None => {
                    if st.logundefined {
                        logs.push(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            cat(&[
                                b"config: using default value for option '",
                                name,
                                b"' - '",
                                def_text,
                                b"'",
                            ]),
                        );
                    }
                    use_option_locked(&mut st, name, clip999(def_text));
                    None
                }
            }
        };
        logs.emit();
        found
    }

    /// `ERANGE` from a glibc conversion; the C boundary applies it to errno.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ConvErr {
        pub erange: bool,
    }

    /// C `str_to_*`: warn when the number is followed by anything but
    /// NUL, tab or space.
    fn check_tail(value: &[u8], end: usize) {
        let e = value.get(end).copied().unwrap_or(0);
        if e != 0 && e != b'\t' && e != b' ' {
            log_bytes_errno(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                &cat(&[b"config: number expected, got '", value, b"'"]),
                0,
            );
        }
    }

    fn conv_i(value: &[u8]) -> (i64, ConvErr) {
        let (v, end, erange) = cnum::strtol(value);
        check_tail(value, end);
        (v, ConvErr { erange })
    }

    fn conv_u(value: &[u8]) -> (u64, ConvErr) {
        let (v, end, erange) = cnum::strtoul(value);
        check_tail(value, end);
        (v, ConvErr { erange })
    }

    const OK: ConvErr = ConvErr { erange: false };

    /// C: `cfg_getstr` (value or the default).
    pub fn get_str(name: &[u8], def: &[u8]) -> Vec<u8> {
        lookup(name, def).unwrap_or_else(|| def.to_vec())
    }

    macro_rules! int_getter {
        ($(#[$m:meta])* $fname:ident, $t:ty, $conv:ident, $mid:ty) => {
            $(#[$m])*
            pub fn $fname(name: &[u8], def: $t) -> ($t, ConvErr) {
                match lookup(name, def.to_string().as_bytes()) {
                    Some(v) => {
                        let (x, e) = $conv(&v);
                        (x as $mid as $t, e)
                    }
                    None => (def, OK),
                }
            }
        };
    }

    int_getter!(/// C: `cfg_getnum` (strtol → int).
        get_num, i32, conv_i, i32);
    int_getter!(/// C: `cfg_getint8` (strtol → int32 → int8).
        get_int8, i8, conv_i, i32);
    int_getter!(/// C: `cfg_getuint8` (strtoul → uint32 → uint8).
        get_uint8, u8, conv_u, u32);
    int_getter!(/// C: `cfg_getint16`.
        get_int16, i16, conv_i, i32);
    int_getter!(/// C: `cfg_getuint16`.
        get_uint16, u16, conv_u, u32);
    int_getter!(/// C: `cfg_getint32`.
        get_int32, i32, conv_i, i32);
    int_getter!(/// C: `cfg_getuint32`.
        get_uint32, u32, conv_u, u32);
    int_getter!(/// C: `cfg_getint64` (strtoll).
        get_int64, i64, conv_i, i64);
    int_getter!(/// C: `cfg_getuint64` (strtoull).
        get_uint64, u64, conv_u, u64);

    /// C: `cfg_getdouble` (strtod; default printed as `%.6lf`).
    pub fn get_double(name: &[u8], def: f64) -> (f64, ConvErr) {
        match lookup(name, cnum::fmt_f6(def).as_bytes()) {
            Some(v) => {
                let (x, end, erange) = cnum::strtod(&v);
                check_tail(&v, end);
                (x, ConvErr { erange })
            }
            None => (def, OK),
        }
    }

    /// Shared body of `_CONFIG_GEN_PERIOD_FUNCTION`.
    fn get_period(name: &[u8], def: &[u8], hmode: bool) -> u32 {
        let mut logs = Logs::default();
        let def = cstr_prefix(def);
        let ret = {
            let mut st = lock();
            let mut result = None;
            if let Some(v) = find(&st.params, name).map(|p| p.value.clone()) {
                let sv = |t: &[u8]| cat(&[t, b" in '", name, b" = ", &v, b"' - using defaults"]);
                match parse_period(&v, hmode) {
                    Ok(r) => {
                        use_option_locked(&mut st, name, &v);
                        result = Some(r);
                    }
                    Err((code, r)) if code == TPARSE_UNEXPECTED_CHAR => {
                        let t = if r != 0 {
                            sv(&cat(&[b"config: unexpected char '", &[r as u8], b"'"]))
                        } else {
                            sv(b"config: unexpected end")
                        };
                        logs.push(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, t);
                    }
                    Err((code, r)) if code == TPARSE_VALUE_TOO_BIG => {
                        let t = if r != 0 {
                            sv(&cat(&[b"config: value too big in section '", &[r as u8], b"'"]))
                        } else {
                            sv(b"config: parsed value too big")
                        };
                        logs.push(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, t);
                    }
                    Err(_) => {}
                }
            }
            match result {
                Some(r) => r,
                None => {
                    if st.logundefined {
                        logs.push(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            cat(&[
                                b"config: using default value for option '",
                                name,
                                b"' - '",
                                def,
                                b"'",
                            ]),
                        );
                    }
                    use_option_locked(&mut st, name, clip999(def));
                    match parse_period(def, hmode) {
                        Ok(r) => r,
                        Err((_, r)) => {
                            logs.push(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                cat(&[
                                    b"config: wrong default value for option '",
                                    name,
                                    b"' - '",
                                    def,
                                    b"' !!!",
                                ]),
                            );
                            r
                        }
                    }
                }
            }
        };
        logs.emit();
        ret
    }

    /// C: `cfg_getsperiod` (seconds).
    pub fn get_speriod(name: &[u8], def: &[u8]) -> u32 {
        get_period(name, def, false)
    }

    /// C: `cfg_gethperiod` (hours; `uint16_t` result).
    pub fn get_hperiod(name: &[u8], def: &[u8]) -> u16 {
        get_period(name, def, true) as u16
    }

    #[cfg(test)]
    pub(super) fn test_lock() -> MutexGuard<'static, ()> {
        static T: Mutex<()> = Mutex::new(());
        T.lock().unwrap_or_else(|e| e.into_inner())
    }
}

// ---------------------------------------------------------------------------
// C ABI boundary. Callers link by symbol; signatures match cfg.h.
// ---------------------------------------------------------------------------

/// C `struct cfg_buff { uint32_t leng; uint8_t data[1]; }`.
#[repr(C)]
pub struct cfg_buff {
    pub leng: u32,
    pub data: [u8; 1],
}

/// # Safety
// SAFETY: caller guarantees `p` is a valid NUL-terminated C string.
unsafe fn bytes<'a>(p: *const c_char) -> &'a [u8] {
    // SAFETY: per fn contract.
    unsafe { CStr::from_ptr(p) }.to_bytes()
}

/// `malloc`ed NUL-terminated copy (the C `strdup`); null on OOM unless
/// `passert_line` asks for the C `passert` abort.
fn c_strdup(v: &[u8], passert_line: Option<u32>) -> *mut c_char {
    // SAFETY: fresh allocation of v.len()+1 bytes, fully initialized below.
    unsafe {
        let p = libc::malloc(v.len() + 1) as *mut u8;
        if p.is_null() {
            if let Some(line) = passert_line {
                oom_abort(CFG_C, line, "_cfg_ret_tmp");
            }
            return core::ptr::null_mut();
        }
        core::ptr::copy_nonoverlapping(v.as_ptr(), p, v.len());
        *p.add(v.len()) = 0;
        p as *mut c_char
    }
}

/// Apply a conversion's `errno = ERANGE` side effect (strto* semantics).
fn set_erange(e: imp::ConvErr) {
    if e.erange {
        // SAFETY: thread-local errno location is always valid.
        unsafe { *libc::__errno_location() = libc::ERANGE };
    }
}

/// # Safety
/// `fname` must be a valid C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_load(fname: *const c_char, logundefined: c_int) -> c_int {
    // SAFETY: per fn contract.
    imp::load(unsafe { bytes(fname) }, logundefined != 0) as c_int
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn cfg_reload() -> c_int {
    imp::reload() as c_int
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn cfg_term() {
    imp::term()
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn cfg_dangerous_options() -> c_int {
    imp::dangerous_options() as c_int
}

/// # Safety
/// `name` must be a valid C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_isdefined(name: *const c_char) -> c_int {
    // SAFETY: per fn contract.
    imp::is_defined(unsafe { bytes(name) }) as c_int
}

/// # Safety
/// `name` and `value` must be valid C strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_use_option(name: *const c_char, value: *const c_char) {
    // SAFETY: per fn contract.
    unsafe { imp::use_option(bytes(name), bytes(value)) }
}

/// # Safety
/// `fd` must be a valid open `FILE*`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_info(fd: *mut libc::FILE) {
    let text = imp::info_text();
    // SAFETY: per fn contract; text is a valid buffer of its length.
    unsafe { libc::fwrite(text.as_ptr() as *const _, 1, text.len(), fd) };
}

/// # Safety
/// `name` must be a valid C string. Result is `malloc`ed (caller frees).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_getdefaultstr(name: *const c_char) -> *mut c_char {
    // SAFETY: per fn contract.
    match imp::default_str(unsafe { bytes(name) }) {
        Some(v) => c_strdup(&v, None),
        None => core::ptr::null_mut(),
    }
}

/// # Safety
/// `name` must be a valid C string. Result is `malloc`ed (caller frees).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_getdefaultfile(name: *const c_char, maxleng: u32) -> *mut cfg_buff {
    // SAFETY: per fn contract.
    let Some(data) = imp::default_file(unsafe { bytes(name) }, maxleng) else {
        return core::ptr::null_mut();
    };
    // leng/data are within the offsetof(cfg_buff,data)+len allocation.
    // SAFETY: fresh allocation, initialized below (data may be empty).
    unsafe {
        let ret = libc::malloc(4 + data.len()) as *mut cfg_buff;
        if ret.is_null() {
            oom_abort(CFG_C, 261, "ret");
        }
        (*ret).leng = data.len() as u32;
        let dst = (ret as *mut u8).add(4);
        core::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
        ret
    }
}

/// # Safety
/// `name` must be a valid C string; `digest` writable for 16 bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_getdefaultfilemd5(
    name: *const c_char,
    txtmode: u8,
    digest: *mut u8,
) -> c_int {
    // SAFETY: per fn contract.
    match imp::default_file_md5(unsafe { bytes(name) }, txtmode != 0) {
        Ok(d) => {
            // SAFETY: per fn contract.
            unsafe { core::ptr::copy_nonoverlapping(d.as_ptr(), digest, 16) };
            0
        }
        Err(()) => -1,
    }
}

/// # Safety
/// `name` must be a valid C string; `def` a valid C string (C faults on a
/// NULL default for an undefined option). Result is `malloc`ed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_getstr(name: *const c_char, def: *const c_char) -> *mut c_char {
    // SAFETY: per fn contract.
    let name = unsafe { bytes(name) };
    let defined = imp::is_defined(name);
    if def.is_null() && !defined {
        // C: strdup(NULL) on the default path — a crash; abort instead of UB.
        std::process::abort();
    }
    // SAFETY: def is non-null here unless the option is defined (then unused).
    let def_b: &[u8] = if def.is_null() { b"" } else { unsafe { bytes(def) } };
    let v = imp::get_str(name, def_b);
    c_strdup(&v, Some(if defined { 380 } else { 455 }))
}

macro_rules! export_int_getter {
    ($cname:ident, $f:ident, $t:ty) => {
        /// # Safety
        /// `name` must be a valid C string.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $cname(name: *const c_char, def: $t) -> $t {
            // SAFETY: per fn contract.
            let (v, e) = imp::$f(unsafe { bytes(name) }, def);
            set_erange(e);
            v
        }
    };
}

export_int_getter!(cfg_getnum, get_num, c_int);
export_int_getter!(cfg_getint8, get_int8, i8);
export_int_getter!(cfg_getuint8, get_uint8, u8);
export_int_getter!(cfg_getint16, get_int16, i16);
export_int_getter!(cfg_getuint16, get_uint16, u16);
export_int_getter!(cfg_getint32, get_int32, i32);
export_int_getter!(cfg_getuint32, get_uint32, u32);
export_int_getter!(cfg_getint64, get_int64, i64);
export_int_getter!(cfg_getuint64, get_uint64, u64);
export_int_getter!(cfg_getdouble, get_double, c_double);

/// # Safety
/// `name` and `def` must be valid C strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_getsperiod(name: *const c_char, def: *const c_char) -> u32 {
    // SAFETY: per fn contract.
    unsafe { imp::get_speriod(bytes(name), bytes(def)) }
}

/// # Safety
/// `name` and `def` must be valid C strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_gethperiod(name: *const c_char, def: *const c_char) -> u16 {
    // SAFETY: per fn contract.
    unsafe { imp::get_hperiod(bytes(name), bytes(def)) }
}

#[cfg(test)]
mod tests {
    use super::imp::*;
    use std::io::Write;

    fn rm(p: &[u8]) {
        std::fs::remove_file(std::str::from_utf8(p).unwrap()).unwrap();
    }

    fn write_cfg(tag: &str, body: &[u8]) -> Vec<u8> {
        let dir = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/cfgtest"));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join(format!("cfgtest-{tag}-{}.cfg", std::process::id()));
        std::fs::File::create(&p).unwrap().write_all(body).unwrap();
        p.into_os_string().into_encoded_bytes()
    }

    #[test]
    fn grammar_matches_c() {
        let _g = test_lock();
        let f = write_cfg(
            "grammar",
            b"# comment\n\
              A = 1\n\
              \tB=two words   \n\
              C =  x # trailing comment\n\
              D = bad\tvalue\n\
              E = caf\xc3\xa9\n\
              \x20 # indented comment\n\
              =nokey\n\
              F = \n\
              G = v1\n\
              G = v2\n\
              DANGEROUS_X = 1\n\
              H = nul\0junk\n\
              I = last",
        );
        assert!(load(&f, false));
        let get = |n: &[u8]| get_str(n, b"<def>");
        assert_eq!(get(b"A"), b"1");
        assert_eq!(get(b"B"), b"two words");
        assert_eq!(get(b"C"), b"x # trailing comment"); // '#' only at column 0
        assert!(!is_defined(b"D")); // tab inside a value is rejected
        assert!(!is_defined(b"E")); // bytes >= 0x80 end the value (signed char)
        assert_eq!(get(b"F"), b"");
        assert_eq!(get(b"G"), b"v2"); // later definition wins
        assert_eq!(get(b"H"), b"nul");
        assert_eq!(get(b"I"), b"last");
        assert!(dangerous_options());
        assert!(!is_defined(b"#"));
        term();
        rm(&f);
    }

    #[test]
    fn getters_convert_and_record_like_c() {
        let _g = test_lock();
        let f = write_cfg(
            "getters",
            b"N = 0x10\nU8 = 300\nI8 = -129\nNEG = -1\nBIG = 99999999999999999999\n\
              D = 1.5e3\nP = 1h30m\nPB = 2x\nHP = 1w\n",
        );
        assert!(load(&f, false));
        assert_eq!(get_num(b"N", 0).0, 16);
        assert_eq!(get_uint8(b"U8", 0).0, 44); // 300 as uint32 → uint8
        assert_eq!(get_int8(b"I8", 0).0, 127); // -129 as int32 → int8
        assert_eq!(get_uint32(b"NEG", 0).0, u32::MAX);
        assert_eq!(get_num(b"BIG", 0), (-1, ConvErr { erange: true }));
        assert_eq!(get_uint64(b"BIG", 0), (u64::MAX, ConvErr { erange: true }));
        assert_eq!(get_double(b"D", 0.0).0, 1500.0);
        assert_eq!(get_speriod(b"P", b"0"), 5400);
        assert_eq!(get_speriod(b"PB", b"7s"), 7); // bad value → default
        assert_eq!(get_hperiod(b"HP", b"1h"), 168);
        assert_eq!(get_speriod(b"MISSING", b"x"), b'x' as u32); // C returns *ret
        assert_eq!(get_int16(b"MISSING16", -5).0, -5);
        assert_eq!(get_double(b"MISSINGD", 0.25).0, 0.25);
        let info = String::from_utf8(info_text()).unwrap();
        assert_eq!(
            info,
            "[config]\nN = 0x10\nU8 = 300\nI8 = -129\nNEG = -1\nBIG = 99999999999999999999\n\
             D = 1.5e3\nP = 1h30m\nPB = 7s\nHP = 1w\nMISSING = x\nMISSING16 = -5\n\
             MISSINGD = 0.250000\n\n"
        );
        term();
        rm(&f);
    }

    #[test]
    fn failed_reload_keeps_params_and_defaults_files() {
        let _g = test_lock();
        let f = write_cfg("keep", b"K = v\n");
        assert!(load(&f, false));
        rm(&f);
        assert!(!reload());
        assert!(is_defined(b"K"));
        // the file-open failure path above already removed `f`
        let data = write_cfg("data", b"hello\n  # c\n  x  \r\n");
        use_option(b"DATA", &data);
        assert_eq!(default_file(b"DATA", 100).unwrap(), b"hello\n  # c\n  x  \r\n");
        assert!(default_file(b"DATA", 3).is_none());
        let md5 = |d: &[u8]| {
            let mut c = crate::md5::md5ctx { state: [0; 4], count: [0; 2], buffer: [0; 64] };
            crate::md5::md5_init_imp(&mut c);
            crate::md5::md5_update_imp(&mut c, d);
            let mut o = [0u8; 16];
            crate::md5::md5_final_imp(&mut o, &mut c);
            o
        };
        assert_eq!(default_file_md5(b"DATA", true).unwrap(), md5(b"hellox"));
        assert_eq!(default_file_md5(b"DATA", false).unwrap(), md5(b"hello\n  # c\n  x  \r\n"));
        assert_eq!(default_file_md5(b"NOPE", false), Err(()));
        term();
        rm(&data);
    }
}
