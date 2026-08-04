use libc::{FILE, c_char, c_double, c_int};
use std::ffi::{CStr, CString, OsString};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use std::sync::{Mutex, MutexGuard};

const MFSLOG_NOTICE: c_int = 2;
const MFSLOG_WARNING: c_int = 3;
const MFSLOG_ERR: c_int = 4;
const MFSLOG_SYSLOG_STDERR: c_int = 2;
const MFSLOG_ERRNO_SYSLOG_STDERR: c_int = 3;
const TPARSE_OK: c_int = 0;
const TPARSE_UNEXPECTED_CHAR: c_int = -1;
const TPARSE_VALUE_TOO_BIG: c_int = -2;

unsafe extern "C" {
    fn mfs_log(mode: c_int, priority: c_int, format: *const c_char, ...);
    fn parse_speriod(value: *const c_char, result: *mut u32) -> c_int;
    fn parse_hperiod(value: *const c_char, result: *mut u32) -> c_int;
    fn md5_init(context: *mut Md5Context);
    fn md5_update(context: *mut Md5Context, data: *const u8, length: u32);
    fn md5_final(digest: *mut u8, context: *mut Md5Context);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn snprintf(buffer: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub struct cfg_buff {
    pub leng: u32,
    pub data: [u8; 1],
}

#[repr(C)]
struct Md5Context {
    state: [u32; 4],
    count: [u32; 2],
    buffer: [u8; 64],
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Entry {
    name: Vec<u8>,
    value: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq)]
enum ParsedLine {
    Ignore,
    Bad(Vec<u8>),
    Entry(Vec<u8>, Vec<u8>),
}

enum ParseWarning {
    Bad(Vec<u8>),
    Duplicate(Vec<u8>, Vec<u8>, Vec<u8>),
}

struct Config {
    filename: Option<Vec<u8>>,
    parsed: Vec<Entry>,
    used: Vec<Entry>,
    log_undefined: bool,
    dangerous: bool,
}

impl Config {
    const fn new() -> Self {
        Self {
            filename: None,
            parsed: Vec::new(),
            used: Vec::new(),
            log_undefined: false,
            dangerous: false,
        }
    }

    fn parse_line(line: &[u8]) -> ParsedLine {
        if line.first() == Some(&b'#') {
            return ParsedLine::Ignore;
        }
        let mut index = 0;
        while matches!(line.get(index), Some(b' ' | b'\t')) {
            index += 1;
        }
        let name_start = index;
        while matches!(line.get(index), Some(byte) if *byte > 32 && *byte < 127 && *byte != b'=') {
            index += 1;
        }
        let name_end = index;
        while matches!(line.get(index), Some(b' ' | b'\t')) {
            index += 1;
        }
        if line.get(index) != Some(&b'=') || name_start == name_end {
            return match line.get(index) {
                Some(byte) if *byte > 32 => ParsedLine::Bad(line.to_vec()),
                _ => ParsedLine::Ignore,
            };
        }
        index += 1;
        while matches!(line.get(index), Some(b' ' | b'\t')) {
            index += 1;
        }
        let value_start = index;
        while matches!(line.get(index), Some(byte) if (32..128).contains(byte)) {
            index += 1;
        }
        let mut value_end = index;
        while value_end > value_start && line[value_end - 1] == b' ' {
            value_end -= 1;
        }
        while matches!(line.get(index), Some(b' ' | b'\t')) {
            index += 1;
        }
        if !matches!(line.get(index), None | Some(0 | b'\r' | b'\n' | b'#')) {
            return ParsedLine::Bad(line.to_vec());
        }
        ParsedLine::Entry(
            line[name_start..name_end].to_vec(),
            line[value_start..value_end].to_vec(),
        )
    }

    fn parse(&mut self, data: &[u8]) -> Vec<ParseWarning> {
        self.parsed.clear();
        let mut warnings = Vec::new();
        for line in data.split_inclusive(|byte| *byte == b'\n') {
            match Self::parse_line(line) {
                ParsedLine::Ignore => {}
                ParsedLine::Bad(line) => warnings.push(ParseWarning::Bad(line)),
                ParsedLine::Entry(name, value) => {
                    if name.starts_with(b"DANGEROUS_") {
                        self.dangerous = true;
                    }
                    if let Some(entry) = self.parsed.iter_mut().find(|entry| entry.name == name) {
                        warnings.push(ParseWarning::Duplicate(
                            name,
                            entry.value.clone(),
                            value.clone(),
                        ));
                        entry.value = value;
                    } else {
                        self.parsed.insert(0, Entry { name, value });
                    }
                }
            }
        }
        warnings
    }

    fn value(&self, name: &[u8]) -> Option<&[u8]> {
        self.parsed
            .iter()
            .find(|entry| entry.name == name)
            .map(|entry| entry.value.as_slice())
    }

    fn default_value(&self, name: &[u8]) -> Option<Vec<u8>> {
        self.parsed
            .iter()
            .chain(self.used.iter())
            .find(|entry| entry.name == name)
            .map(|entry| entry.value.clone())
    }

    fn record(&mut self, name: &[u8], value: &[u8]) {
        if let Some(entry) = self.used.iter_mut().find(|entry| entry.name == name) {
            entry.value = value.to_vec();
        } else {
            self.used.push(Entry {
                name: name.to_vec(),
                value: value.to_vec(),
            });
        }
    }

    fn value_or_record(&mut self, name: &[u8], default: &[u8]) -> Vec<u8> {
        if let Some(value) = self.value(name).map(<[u8]>::to_vec) {
            self.record(name, &value);
            value
        } else {
            self.record(name, &default[..default.len().min(999)]);
            default.to_vec()
        }
    }
}

static CONFIG: Mutex<Config> = Mutex::new(Config::new());

fn config() -> MutexGuard<'static, Config> {
    match CONFIG.lock() {
        Ok(config) => config,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn cstring(bytes: &[u8]) -> CString {
    let end = bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len());
    CString::new(&bytes[..end]).unwrap_or_default()
}

/// # Safety
/// `pointer` must reference a readable NUL-terminated C string.
unsafe fn input_bytes(pointer: *const c_char) -> Vec<u8> {
    // SAFETY: Category 8 (FFI); exported caller contract supplies valid C string.
    unsafe { CStr::from_ptr(pointer) }.to_bytes().to_vec()
}

fn malloc_string(bytes: &[u8]) -> *mut c_char {
    // SAFETY: Category 8/10/12 (FFI, bounds, allocation); size includes terminator,
    // destination is fresh libc storage, and caller receives sole free() ownership.
    let result = unsafe { libc::malloc(bytes.len() + 1) }.cast::<c_char>();
    if result.is_null() {
        std::process::abort();
    }
    // SAFETY: Category 10 (bounds); allocation above has bytes.len()+1 writable bytes.
    unsafe { ptr::copy_nonoverlapping(bytes.as_ptr().cast::<c_char>(), result, bytes.len()) };
    // SAFETY: Category 10 (bounds); final allocated byte is reserved for NUL.
    unsafe { result.add(bytes.len()).write(0) };
    result
}

fn option(name: &[u8], default: &[u8]) -> (Vec<u8>, bool, bool) {
    let mut state = config();
    let configured = state.value(name).is_some();
    let log_default = state.log_undefined && !configured;
    (
        state.value_or_record(name, default),
        configured,
        log_default,
    )
}

fn log_default(name: &[u8], value: &[u8]) {
    let name = cstring(name);
    let value = cstring(value);
    // SAFETY: Category 8 (FFI); format and both CString arguments are valid for call.
    unsafe {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            c"config: using default value for option '%s' - '%s'".as_ptr(),
            name.as_ptr(),
            value.as_ptr(),
        )
    };
}

fn log_bad_number(value: &[u8]) {
    let value = cstring(value);
    // SAFETY: Category 8 (FFI); fixed format matches one valid CString argument.
    unsafe {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            c"config: number expected, got '%s'".as_ptr(),
            value.as_ptr(),
        )
    };
}

fn parse_integer(bytes: &[u8]) -> (i64, usize) {
    let value = cstring(bytes);
    let mut end = ptr::null_mut();
    // SAFETY: Category 8 (FFI); CString is readable and end receives an interior pointer.
    let result = unsafe { libc::strtoll(value.as_ptr(), &mut end, 0) };
    // SAFETY: Category 11 (provenance); strtoll returns a pointer within the same CString.
    let offset = unsafe { end.offset_from(value.as_ptr()) };
    (result, usize::try_from(offset).unwrap_or(0))
}

fn parse_unsigned(bytes: &[u8]) -> (u64, usize) {
    let value = cstring(bytes);
    let mut end = ptr::null_mut();
    // SAFETY: Category 8 (FFI); CString is readable and end receives an interior pointer.
    let result = unsafe { libc::strtoull(value.as_ptr(), &mut end, 0) };
    // SAFETY: Category 11 (provenance); strtoull returns a pointer within the same CString.
    let offset = unsafe { end.offset_from(value.as_ptr()) };
    (result, usize::try_from(offset).unwrap_or(0))
}

fn warn_numeric_tail(bytes: &[u8], end: usize) {
    if matches!(bytes.get(end), Some(byte) if *byte != b' ' && *byte != b'\t') {
        log_bad_number(bytes);
    }
}

fn parse_double(bytes: &[u8]) -> (f64, usize) {
    let value = cstring(bytes);
    let mut end = ptr::null_mut();
    // SAFETY: Category 8 (FFI); CString is readable and end receives interior pointer.
    let result = unsafe { libc::strtod(value.as_ptr(), &mut end) };
    // SAFETY: Category 11 (provenance); strtod returns pointer within same CString.
    let offset = unsafe { end.offset_from(value.as_ptr()) };
    (result, usize::try_from(offset).unwrap_or(0))
}

fn format_double(value: f64) -> Vec<u8> {
    let mut buffer = [0; 1000];
    // SAFETY: Category 8/10 (FFI/bounds); fixed format writes at most buffer length.
    unsafe { snprintf(buffer.as_mut_ptr(), buffer.len(), c"%.6lf".as_ptr(), value) };
    let length = buffer
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(buffer.len());
    buffer[..length].iter().map(|byte| *byte as u8).collect()
}

fn parse_period(bytes: &[u8], hours: bool) -> (c_int, u32) {
    let value = cstring(bytes);
    let mut result = 0;
    // SAFETY: Category 8 (FFI); CString and initialized output pointer match parser ABI.
    let status = unsafe {
        if hours {
            parse_hperiod(value.as_ptr(), &mut result)
        } else {
            parse_speriod(value.as_ptr(), &mut result)
        }
    };
    (status, result)
}

fn log_period_error(name: &[u8], value: &[u8], status: c_int, detail: u32) {
    let name = cstring(name);
    let value = cstring(value);
    let format = match (status, detail) {
        (TPARSE_UNEXPECTED_CHAR, 0) => c"config: unexpected end in '%s = %s' - using defaults",
        (TPARSE_UNEXPECTED_CHAR, _) => {
            c"config: unexpected char '%c' in '%s = %s' - using defaults"
        }
        (TPARSE_VALUE_TOO_BIG, 0) => c"config: parsed value too big in '%s = %s' - using defaults",
        (TPARSE_VALUE_TOO_BIG, _) => {
            c"config: value too big in section '%c' in '%s = %s' - using defaults"
        }
        _ => return,
    };
    // SAFETY: Category 8 (FFI); selected format's arguments match branch arity/types.
    unsafe {
        if detail == 0 {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                format.as_ptr(),
                name.as_ptr(),
                value.as_ptr(),
            );
        } else {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                format.as_ptr(),
                detail as c_int,
                name.as_ptr(),
                value.as_ptr(),
            );
        }
    }
}

fn period(name: &[u8], default: &[u8], hours: bool) -> u32 {
    let configured_value = config().value(name).map(<[u8]>::to_vec);
    if let Some(value) = configured_value {
        let (status, result) = parse_period(&value, hours);
        if status == TPARSE_OK {
            config().record(name, &value);
            return result;
        }
        log_period_error(name, &value, status, result);
    }
    let log_undefined = config().log_undefined;
    if log_undefined {
        log_default(name, default);
    }
    config().record(name, &default[..default.len().min(999)]);
    let (status, result) = parse_period(default, hours);
    if status != TPARSE_OK {
        let name = cstring(name);
        let default = cstring(default);
        // SAFETY: Category 8 (FFI); fixed format matches two valid CString arguments.
        unsafe {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                c"config: wrong default value for option '%s' - '%s' !!!".as_ptr(),
                name.as_ptr(),
                default.as_ptr(),
            )
        };
    }
    result
}

fn log_parse_warnings(filename: &[u8], warnings: Vec<ParseWarning>) {
    let filename = cstring(filename);
    for warning in warnings {
        match warning {
            ParseWarning::Bad(line) => {
                let line = cstring(&line);
                // SAFETY: Category 8 (FFI); fixed format matches two CStrings.
                unsafe {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        c"bad definition in config file '%s': %s".as_ptr(),
                        filename.as_ptr(),
                        line.as_ptr(),
                    )
                };
            }
            ParseWarning::Duplicate(name, old, new) => {
                let name = cstring(&name);
                let old = cstring(&old);
                let new = cstring(&new);
                // SAFETY: Category 8 (FFI); fixed format matches three CStrings.
                unsafe {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        c"variable '%s' defined more than once in the config file (previous value: %s, current value: %s)".as_ptr(),
                        name.as_ptr(),
                        old.as_ptr(),
                        new.as_ptr(),
                    )
                };
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_load(filename: *const c_char, log_undefined: c_int) -> c_int {
    // SAFETY: Category 8 (FFI); cfg_load C contract requires valid filename string.
    let filename = unsafe { input_bytes(filename) };
    {
        let mut state = config();
        state.filename = Some(filename);
        state.parsed.clear();
        state.used.clear();
        state.log_undefined = log_undefined != 0;
        state.dangerous = false;
    }
    // SAFETY: cfg_load established initialized filename/state required by cfg_reload.
    unsafe { cfg_reload() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_reload() -> c_int {
    let Some(filename) = config().filename.clone() else {
        return 0;
    };
    let path = PathBuf::from(OsString::from_vec(filename.clone()));
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            let filename = cstring(&filename);
            // SAFETY: Category 8 (FFI); fixed format matches valid filename CString.
            unsafe {
                mfs_log(
                    if error.kind() == std::io::ErrorKind::NotFound {
                        MFSLOG_SYSLOG_STDERR
                    } else {
                        MFSLOG_ERRNO_SYSLOG_STDERR
                    },
                    MFSLOG_ERR,
                    if error.kind() == std::io::ErrorKind::NotFound {
                        c"main config file (%s) not found".as_ptr()
                    } else {
                        c"can't load main config file (%s), error".as_ptr()
                    },
                    filename.as_ptr(),
                )
            };
            return 0;
        }
    };
    let mut data = Vec::new();
    let _ = file.read_to_end(&mut data);
    let warnings = config().parse(&data);
    log_parse_warnings(&filename, warnings);
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_use_option(name: *const c_char, value: *const c_char) {
    // SAFETY: Category 8 (FFI); caller supplies valid option-name C string.
    let name = unsafe { input_bytes(name) };
    // SAFETY: Category 8 (FFI); caller supplies valid option-value C string.
    let value = unsafe { input_bytes(value) };
    config().record(&name, &value);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_info(stream: *mut FILE) {
    // SAFETY: Category 8 (FFI); caller owns valid FILE and format needs no arguments.
    unsafe { fprintf(stream, c"[config]\n".as_ptr()) };
    for entry in &config().used {
        let name = cstring(&entry.name);
        let value = cstring(&entry.value);
        // SAFETY: Category 8 (FFI); caller owns valid FILE and format matches CStrings.
        unsafe { fprintf(stream, c"%s = %s\n".as_ptr(), name.as_ptr(), value.as_ptr()) };
    }
    // SAFETY: Category 8 (FFI); caller owns valid FILE and format needs no arguments.
    unsafe { fprintf(stream, c"\n".as_ptr()) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_dangerous_options() -> c_int {
    c_int::from(config().dangerous)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_isdefined(name: *const c_char) -> c_int {
    // SAFETY: Category 8 (FFI); caller supplies valid option-name C string.
    let name = unsafe { input_bytes(name) };
    c_int::from(config().value(&name).is_some())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_term() {
    let mut state = config();
    state.filename = None;
    state.parsed.clear();
    state.used.clear();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_getdefaultstr(name: *const c_char) -> *mut c_char {
    // SAFETY: Category 8 (FFI); caller supplies valid option-name C string.
    let name = unsafe { input_bytes(name) };
    config()
        .default_value(&name)
        .map_or(ptr::null_mut(), |value| malloc_string(&value))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_getdefaultfile(name: *const c_char, max_length: u32) -> *mut cfg_buff {
    // SAFETY: Category 8 (FFI); caller supplies valid option-name C string.
    let name = unsafe { input_bytes(name) };
    let Some(filename) = config().default_value(&name) else {
        return ptr::null_mut();
    };
    let Ok(mut file) = File::open(PathBuf::from(OsString::from_vec(filename))) else {
        return ptr::null_mut();
    };
    let Ok(length) = file.seek(SeekFrom::End(0)) else {
        return ptr::null_mut();
    };
    if length > u64::from(max_length) || file.seek(SeekFrom::Start(0)).is_err() {
        return ptr::null_mut();
    }
    let mut data = vec![0; length as usize];
    if file.read_exact(&mut data).is_err() {
        return ptr::null_mut();
    }
    let offset = std::mem::offset_of!(cfg_buff, data);
    // SAFETY: Category 8/10/12; exact C layout size allocated by libc for caller free().
    let result = unsafe { libc::malloc(offset + data.len()) }.cast::<cfg_buff>();
    if result.is_null() {
        std::process::abort();
    }
    // SAFETY: Category 10; malloc alignment covers u32 and allocation covers header.
    unsafe { ptr::addr_of_mut!((*result).leng).write(data.len() as u32) };
    // SAFETY: Category 10; destination starts at data offset with data.len() capacity.
    unsafe { ptr::copy_nonoverlapping(data.as_ptr(), result.cast::<u8>().add(offset), data.len()) };
    result
}

fn md5_start() -> Md5Context {
    let mut context = Md5Context {
        state: [0; 4],
        count: [0; 2],
        buffer: [0; 64],
    };
    // SAFETY: Category 8 (FFI); initialized context matches md5_init ABI.
    unsafe { md5_init(&mut context) };
    context
}

fn md5_add(context: &mut Md5Context, data: &[u8]) {
    // SAFETY: Category 8/10 (FFI/bounds); slice pointer covers supplied u32 length.
    unsafe { md5_update(context, data.as_ptr(), data.len() as u32) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_getdefaultfilemd5(
    name: *const c_char,
    text_mode: u8,
    digest: *mut u8,
) -> c_int {
    // SAFETY: Category 8 (FFI); caller supplies valid option-name C string.
    let name = unsafe { input_bytes(name) };
    let Some(filename) = config().default_value(&name) else {
        return -1;
    };
    let Ok(file) = File::open(PathBuf::from(OsString::from_vec(filename))) else {
        return -1;
    };
    let mut context = md5_start();
    if text_mode != 0 {
        let mut reader = BufReader::new(file);
        let mut line = Vec::new();
        loop {
            line.clear();
            match reader.read_until(b'\n', &mut line) {
                Ok(0) => break,
                Ok(_) => {
                    while matches!(line.last(), Some(b'\r' | b'\n' | b'\t' | b' ')) {
                        line.pop();
                    }
                    let start = line
                        .iter()
                        .position(|byte| !matches!(byte, b' ' | b'\t'))
                        .unwrap_or(line.len());
                    if line.get(start).is_some_and(|byte| *byte != b'#') {
                        md5_add(&mut context, &line[start..]);
                    }
                }
                Err(_) => return -1,
            }
        }
    } else {
        let mut reader = BufReader::with_capacity(65_536, file);
        loop {
            let buffer = match reader.fill_buf() {
                Ok([]) => break,
                Ok(buffer) => buffer,
                Err(_) => return -1,
            };
            let length = buffer.len();
            md5_add(&mut context, buffer);
            reader.consume(length);
        }
    }
    // SAFETY: Category 8/10 (FFI/bounds); caller contract provides writable 16-byte digest.
    unsafe { md5_final(digest, &mut context) };
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_getstr(name: *const c_char, default: *const c_char) -> *mut c_char {
    // SAFETY: Category 8 (FFI); caller supplies valid name/default C strings.
    let name = unsafe { input_bytes(name) };
    // SAFETY: Category 8 (FFI); caller supplies valid default C string.
    let default = unsafe { input_bytes(default) };
    let (value, _, log) = option(&name, &default);
    if log {
        log_default(&name, &default);
    }
    malloc_string(&value)
}

macro_rules! integer_getter {
    ($function:ident, $type:ty, $parser:ident) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $function(name: *const c_char, default: $type) -> $type {
            // SAFETY: Category 8 (FFI); caller supplies valid option-name C string.
            let name = unsafe { input_bytes(name) };
            let rendered = default.to_string();
            let (value, configured, log) = option(&name, rendered.as_bytes());
            if log {
                log_default(&name, rendered.as_bytes());
            }
            if !configured {
                return default;
            }
            let (result, end) = $parser(&value);
            warn_numeric_tail(&value, end);
            result as $type
        }
    };
}

integer_getter!(cfg_getnum, c_int, parse_integer);
integer_getter!(cfg_getint8, i8, parse_integer);
integer_getter!(cfg_getuint8, u8, parse_unsigned);
integer_getter!(cfg_getint16, i16, parse_integer);
integer_getter!(cfg_getuint16, u16, parse_unsigned);
integer_getter!(cfg_getint32, i32, parse_integer);
integer_getter!(cfg_getuint32, u32, parse_unsigned);
integer_getter!(cfg_getint64, i64, parse_integer);
integer_getter!(cfg_getuint64, u64, parse_unsigned);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_getdouble(name: *const c_char, default: c_double) -> c_double {
    // SAFETY: Category 8 (FFI); caller supplies valid option-name C string.
    let name = unsafe { input_bytes(name) };
    let rendered = format_double(default);
    let (value, configured, log) = option(&name, &rendered);
    if log {
        log_default(&name, &rendered);
    }
    if !configured {
        return default;
    }
    let (result, end) = parse_double(&value);
    warn_numeric_tail(&value, end);
    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_getsperiod(name: *const c_char, default: *const c_char) -> u32 {
    // SAFETY: Category 8 (FFI); caller supplies valid name/default C strings.
    let name = unsafe { input_bytes(name) };
    // SAFETY: Category 8 (FFI); caller supplies valid default C string.
    let default = unsafe { input_bytes(default) };
    period(&name, &default, false)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cfg_gethperiod(name: *const c_char, default: *const c_char) -> u16 {
    // SAFETY: Category 8 (FFI); caller supplies valid name/default C strings.
    let name = unsafe { input_bytes(name) };
    // SAFETY: Category 8 (FFI); caller supplies valid default C string.
    let default = unsafe { input_bytes(default) };
    period(&name, &default, true) as u16
}

#[cfg(test)]
mod tests {
    use super::{Config, ParsedLine, parse_integer, parse_unsigned};
    use std::ffi::{CStr, CString};
    use std::os::unix::ffi::OsStrExt;

    #[test]
    fn parser_preserves_c_whitespace_and_comment_rules() {
        assert_eq!(Config::parse_line(b"# whole line\n"), ParsedLine::Ignore);
        assert_eq!(
            Config::parse_line(b" \tNAME \t=\t value   \n"),
            ParsedLine::Entry(b"NAME".to_vec(), b"value".to_vec())
        );
        assert_eq!(
            Config::parse_line(b"INLINE=value # remains value\n"),
            ParsedLine::Entry(b"INLINE".to_vec(), b"value # remains value".to_vec())
        );
        assert_eq!(
            Config::parse_line(b"TAB=value\t# comment\n"),
            ParsedLine::Entry(b"TAB".to_vec(), b"value".to_vec())
        );
        assert_eq!(
            Config::parse_line(b"NONASCII=\x80\n"),
            ParsedLine::Bad(b"NONASCII=\x80\n".to_vec())
        );
    }

    #[test]
    fn parser_replaces_duplicate_and_getter_records_default() {
        let mut config = Config::new();
        config.parse(b"A=first\nB=other\nA=last\n");
        assert_eq!(config.value(b"A"), Some(b"last".as_slice()));
        assert_eq!(config.value_or_record(b"MISSING", b"fallback"), b"fallback");
        assert_eq!(config.default_value(b"MISSING"), Some(b"fallback".to_vec()));
    }

    #[test]
    fn integer_parser_matches_c_base_zero_and_prefix_rules() {
        assert_eq!(parse_integer(b"010"), (8, 3));
        assert_eq!(parse_integer(b"0x10"), (16, 4));
        assert_eq!(parse_integer(b"-1"), (-1, 2));
        assert_eq!(parse_integer(b"12junk"), (12, 2));
        assert_eq!(parse_integer(b"0x"), (0, 1));
    }

    #[test]
    fn integer_parser_matches_libc_edges() {
        for input in [
            "0",
            "08",
            "  +17tail",
            "-9223372036854775808",
            "9223372036854775808",
            "18446744073709551616",
            "-18446744073709551616",
            "xyz",
        ] {
            let value = CString::new(input).unwrap();
            let mut signed_end = std::ptr::null_mut();
            let mut unsigned_end = std::ptr::null_mut();
            // SAFETY: CStrings are valid; libc writes only end pointers into same allocations.
            let signed = unsafe { libc::strtoll(value.as_ptr(), &mut signed_end, 0) };
            // SAFETY: same valid CString/end-pointer contract as signed conversion.
            let unsigned = unsafe { libc::strtoull(value.as_ptr(), &mut unsigned_end, 0) };
            // SAFETY: libc end pointers originate inside value's CString allocation.
            let signed_offset = unsafe { signed_end.offset_from(value.as_ptr()) } as usize;
            // SAFETY: libc end pointers originate inside value's CString allocation.
            let unsigned_offset = unsafe { unsigned_end.offset_from(value.as_ptr()) } as usize;
            assert_eq!(parse_integer(input.as_bytes()), (signed, signed_offset));
            assert_eq!(
                parse_unsigned(input.as_bytes()),
                (unsigned, unsigned_offset)
            );
        }
    }

    #[test]
    fn abi_loads_gets_and_returns_libc_owned_string() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
            "../target/cfg-test-{}-{}",
            env!("CARGO_PKG_NAME"),
            std::process::id()
        ));
        std::fs::write(&path, b"NAME = value\nNUMBER = 010\n").unwrap();
        let filename = CString::new(path.as_os_str().as_bytes()).unwrap();

        // SAFETY: test supplies valid C strings and frees cfg_getstr's libc allocation once.
        unsafe {
            assert_eq!(super::cfg_load(filename.as_ptr(), 0), 1);
            let value = super::cfg_getstr(c"NAME".as_ptr(), c"default".as_ptr());
            assert_eq!(CStr::from_ptr(value).to_bytes(), b"value");
            libc::free(value.cast());
            assert_eq!(super::cfg_getuint32(c"NUMBER".as_ptr(), 0), 8);
            super::cfg_term();
        }
        std::fs::remove_file(path).unwrap();
    }
}
