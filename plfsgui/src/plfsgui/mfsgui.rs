// allow: SIZE_OK — exported C ABI and process/syscall seams must remain one translation unit.

#[path = "mfsgui_core.rs"]
mod core;

use ::core::cell::{Cell, RefCell};
use ::core::ffi::{VaList, c_char, c_double, c_int, c_void};
use plfscommon::md5::{md5_final, md5_init, md5_update, md5ctx};
use std::ffi::{CStr, CString, OsString};
use std::fs::{self, File};
use std::io::{self, Read};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

unsafe extern "C" {
    fn cfg_getstr(name: *const c_char, default: *const c_char) -> *mut c_char;
    fn cfg_getuint32(name: *const c_char, default: u32) -> u32;
    fn main_destruct_register_fname(callback: Option<unsafe extern "C" fn()>, name: *const c_char);
    fn main_reload_register_fname(callback: Option<unsafe extern "C" fn()>, name: *const c_char);
    fn main_chld_register_fname(
        pid: libc::pid_t,
        callback: Option<unsafe extern "C" fn(libc::pid_t, c_int)>,
        name: *const c_char,
    );
    fn main_poll_register_fname(
        desc: Option<unsafe extern "C" fn(*mut libc::pollfd, *mut u32)>,
        serve: Option<unsafe extern "C" fn(*mut libc::pollfd)>,
        desc_name: *const c_char,
        serve_name: *const c_char,
    );
    fn main_time_register_fname(
        seconds: u32,
        offset: u32,
        callback: Option<unsafe extern "C" fn()>,
        name: *const c_char,
    ) -> *mut c_void;
    fn monotonic_seconds() -> c_double;
    fn mfs_log(mode: c_int, priority: c_int, format: *const c_char, ...);
    fn tcpsocket() -> c_int;
    fn tcpresolve(
        hostname: *const c_char,
        service: *const c_char,
        ip: *mut u32,
        port: *mut u16,
        passive: c_int,
    ) -> c_int;
    fn tcpnonblock(socket: c_int) -> c_int;
    fn tcpsetacceptfilter(socket: c_int) -> c_int;
    fn tcpreuseaddr(socket: c_int) -> c_int;
    fn tcpnodelay(socket: c_int) -> c_int;
    fn tcpnumlisten(socket: c_int, ip: u32, port: u16, queue: u16) -> c_int;
    fn tcptowrite(
        socket: c_int,
        buffer: *const c_void,
        length: u32,
        partial_timeout_ms: u32,
        total_timeout_ms: u32,
    ) -> i32;
    fn tcpaccept(socket: c_int) -> c_int;
    fn tcpclose(socket: c_int) -> c_int;
    fn vsnprintf(
        output: *mut c_char,
        maximum: usize,
        format: *const c_char,
        arguments: VaList,
    ) -> c_int;
}

pub type pid_t = libc::pid_t;
pub type time_t = libc::time_t;
pub type pollfd = libc::pollfd;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct requests_str {
    pub rtype: u8,
    pub rl: u32,
    pub request: *mut c_char,
    pub fl: u32,
    pub fname: *mut c_char,
    pub extra: *mut c_char,
    pub mimetype: *const c_char,
    pub mtimestr: *mut c_char,
    pub etag: *mut c_char,
    pub mtime: time_t,
    pub fsize: u32,
    pub fdata: *mut u8,
    pub next: *mut requests_str,
}
pub type requests = requests_str;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct httphandle_str {
    pub sock: c_int,
    pub error: c_int,
    pub inputdata: *mut u8,
    pub inputdataleng: u32,
    pub outputdata: *mut u8,
    pub outputdataleng: u32,
    pub starttime: c_double,
    pub requrl: *mut c_char,
    pub reqargs: *mut c_char,
    pub httpver: u8,
    pub keepalive: u8,
    pub etagmatch: u8,
    pub mtimematch: u8,
    pub method: u8,
    pub status: u8,
    pub req: *mut requests,
}
pub type httphandle = httphandle_str;

pub const METHOD_NONE: u32 = 0;
pub const METHOD_GET: u32 = 1;
pub const METHOD_HEAD: u32 = 2;
pub const STATUS_NONE: u32 = 0;
pub const STATUS_FOUND: u32 = 1;
pub const STATUS_NOTFOUND: u32 = 2;
pub const STATUS_NOTMODIFIED: u32 = 4;
pub const STATUS_BADREQUEST: u32 = 5;
pub const STATUS_BADMETHOD: u32 = 6;
pub const STATUS_INTERROR: u32 = 7;
pub const MATCH_UNKNOWN: u32 = 0;
pub const MATCH_NO: u32 = 1;
pub const MATCH_YES: u32 = 2;
pub const HTTP_HEADERMAX: c_int = core::HTTP_HEADER_MAX as c_int;
pub const MFSLOG_INFO: c_int = 1;
pub const MFSLOG_NOTICE: c_int = 2;
pub const MFSLOG_WARNING: c_int = 3;
pub const MFSLOG_ERR: c_int = 4;
pub const MFSLOG_SYSLOG: c_int = 0;
pub const MFSLOG_SYSLOG_STDERR: c_int = 2;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: c_int = 3;

struct RequestMirror {
    raw: Box<requests>,
    route: CString,
    target: CString,
    extra: Option<CString>,
    modified: Option<CString>,
    etag: Option<CString>,
    data: Vec<u8>,
}

impl RequestMirror {
    fn new(request: &core::Request) -> Self {
        let route = CString::new(request.route.clone()).unwrap_or_default();
        let target = CString::new(request.target.clone()).unwrap_or_default();
        let extra = request
            .extra
            .clone()
            .and_then(|value| CString::new(value).ok());
        let modified = request
            .modified_text
            .clone()
            .and_then(|value| CString::new(value).ok());
        let etag = request
            .etag
            .map(|value| CString::new(value.to_vec()).unwrap_or_default());
        let mut data = if request.kind == core::RequestKind::Cgi {
            vec![0; core::FORWARD_BUFFER_SIZE]
        } else {
            request.data.clone()
        };
        let fdata =
            if request.kind == core::RequestKind::File || request.kind == core::RequestKind::Cgi {
                data.as_mut_ptr()
            } else {
                std::ptr::null_mut()
            };
        let raw = Box::new(requests {
            rtype: request.kind as u8,
            rl: request.route.len() as u32,
            request: route.as_ptr().cast_mut(),
            fl: request.target.len() as u32,
            fname: target.as_ptr().cast_mut(),
            extra: extra
                .as_ref()
                .map_or(std::ptr::null_mut(), |value| value.as_ptr().cast_mut()),
            mimetype: mime_c_pointer(request.mime),
            mtimestr: modified
                .as_ref()
                .map_or(std::ptr::null_mut(), |value| value.as_ptr().cast_mut()),
            etag: etag
                .as_ref()
                .map_or(std::ptr::null_mut(), |value| value.as_ptr().cast_mut()),
            mtime: request.modified_seconds as time_t,
            fsize: if request.kind == core::RequestKind::Cgi {
                0
            } else {
                request.data.len() as u32
            },
            fdata,
            next: std::ptr::null_mut(),
        });
        Self {
            raw,
            route,
            target,
            extra,
            modified,
            etag,
            data,
        }
    }
}

struct Runtime {
    state: core::ServerState,
    mirrors: Vec<RequestMirror>,
}

impl Runtime {
    fn rebuild_mirrors(&mut self) {
        let mut mirrors = self
            .state
            .requests
            .iter()
            .map(RequestMirror::new)
            .collect::<Vec<_>>();
        let mut next = std::ptr::null_mut();
        for mirror in mirrors.iter_mut().rev() {
            mirror.raw.next = next;
            next = (&mut *mirror.raw) as *mut requests;
        }
        self.mirrors = mirrors;
    }

    fn mirror_pointer(&mut self, index: usize) -> *mut requests {
        self.mirrors
            .get_mut(index)
            .map_or(std::ptr::null_mut(), |mirror| {
                (&mut *mirror.raw) as *mut requests
            })
    }

    fn request_for_mirror(&self, pointer: *mut requests) -> Option<&core::Request> {
        self.mirrors
            .iter()
            .position(|mirror| {
                std::ptr::eq((&*mirror.raw) as *const requests, pointer.cast_const())
            })
            .and_then(|index| self.state.requests.get(index))
    }
}

thread_local! {
    static RUNTIME: RefCell<Option<Runtime>> = const { RefCell::new(None) };
    static CHILDREN: RefCell<Vec<pid_t>> = const { RefCell::new(Vec::new()) };
    static CGI_PATH: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static LISTENER: Cell<c_int> = const { Cell::new(-1) };
    static LISTENER_POLL_POSITION: Cell<u32> = const { Cell::new(0) };
}

fn with_runtime<R>(function: impl FnOnce(&Runtime) -> R) -> Option<R> {
    RUNTIME.with(|runtime| {
        let runtime = runtime.try_borrow().ok()?;
        runtime.as_ref().map(function)
    })
}

fn with_runtime_mut<R>(function: impl FnOnce(&mut Runtime) -> R) -> Option<R> {
    RUNTIME.with(|runtime| {
        let mut runtime = runtime.try_borrow_mut().ok()?;
        runtime.as_mut().map(function)
    })
}

fn mime_c_pointer(mime: Option<&[u8]>) -> *const c_char {
    match mime {
        Some(b"text/plain") => c"text/plain".as_ptr(),
        Some(b"text/html; charset=utf-8") => c"text/html; charset=utf-8".as_ptr(),
        Some(b"text/css") => c"text/css".as_ptr(),
        Some(b"text/javascript") => c"text/javascript".as_ptr(),
        Some(b"image/vnd.microsoft.icon") => c"image/vnd.microsoft.icon".as_ptr(),
        Some(b"image/gif") => c"image/gif".as_ptr(),
        Some(b"image/jpeg") => c"image/jpeg".as_ptr(),
        Some(b"image/png") => c"image/png".as_ptr(),
        Some(b"image/tiff") => c"image/tiff".as_ptr(),
        Some(b"image/bmp") => c"image/bmp".as_ptr(),
        Some(b"application/zip") => c"application/zip".as_ptr(),
        Some(b"text/xml") => c"text/xml".as_ptr(),
        Some(b"image/svg+xml") => c"image/svg+xml".as_ptr(),
        Some(b"application/pdf") => c"application/pdf".as_ptr(),
        Some(b"application/octet-stream") => c"application/octet-stream".as_ptr(),
        Some(_) | None => std::ptr::null(),
    }
}

fn log_message(mode: c_int, priority: c_int, message: &[u8]) {
    let Ok(message) = CString::new(message) else {
        return;
    };
    // SAFETY: Category 8 (FFI). Format is static; CString supplies one matching string operand.
    unsafe { mfs_log(mode, priority, c"%s".as_ptr(), message.as_ptr()) };
}

fn log_path(mode: c_int, priority: c_int, prefix: &[u8], path: &[u8]) {
    let mut message = Vec::with_capacity(prefix.len() + path.len());
    message.extend_from_slice(prefix);
    message.extend_from_slice(path);
    log_message(mode, priority, &message);
}

fn configured_string(name: &CStr, default: &CStr) -> Vec<u8> {
    // SAFETY: Category 8/12 (FFI/ownership). cfg returns a malloc-owned NUL-terminated string.
    unsafe {
        let pointer = cfg_getstr(name.as_ptr(), default.as_ptr());
        if pointer.is_null() {
            return default.to_bytes().to_vec();
        }
        let bytes = CStr::from_ptr(pointer).to_bytes().to_vec();
        libc::free(pointer.cast());
        bytes
    }
}

fn configured_timeout() -> u16 {
    // SAFETY: Category 8 (FFI). Static config key is NUL-terminated.
    let timeout = unsafe { cfg_getuint32(c"GUISERV_TIMEOUT".as_ptr(), 300) };
    core::clamp_timeout(timeout)
}

fn path_from_bytes(bytes: &[u8]) -> PathBuf {
    PathBuf::from(OsString::from_vec(bytes.to_vec()))
}

fn md5_hex(data: &[u8]) -> [u8; 32] {
    let mut context = md5ctx {
        state: [0; 4],
        count: [0; 2],
        buffer: [0; 64],
    };
    let mut digest = [0u8; 16];
    // SAFETY: Category 8 (FFI). Context/digest are uniquely writable; slice is readable for u32 length.
    unsafe {
        md5_init(&mut context);
        md5_update(&mut context, data.as_ptr(), data.len() as u32);
        md5_final(digest.as_mut_ptr(), &mut context);
    }
    let mut output = [0u8; 32];
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for (index, byte) in digest.into_iter().enumerate() {
        output[index * 2] = HEX[(byte >> 4) as usize];
        output[index * 2 + 1] = HEX[(byte & 0x0f) as usize];
    }
    output
}

fn format_modified(seconds: i64) -> Option<Vec<u8>> {
    let timestamp = seconds as libc::time_t;
    // SAFETY: Category 8 (FFI). Timestamp/format live through call; output holds 31 writable bytes.
    unsafe {
        let broken_down = libc::gmtime(&timestamp);
        if broken_down.is_null() {
            return None;
        }
        let mut output = [0 as c_char; 31];
        let length = libc::strftime(
            output.as_mut_ptr(),
            30,
            c"%a, %d %b %Y %T GMT".as_ptr(),
            broken_down,
        );
        (length > 0).then(|| output[..length].iter().map(|byte| *byte as u8).collect())
    }
}

fn load_file(request: &mut core::Request) -> io::Result<()> {
    let path = path_from_bytes(&request.target);
    let mut file = File::open(&path)?;
    let metadata = file.metadata()?;
    let size = metadata.len() as u32;
    let mut data = vec![0; size as usize];
    file.read_exact(&mut data)?;
    let modified = metadata.mtime();
    request.modified_seconds = modified;
    request.modified_text = format_modified(modified);
    request.etag = Some(md5_hex(&data));
    request.data = data;
    if request.extra.is_none() {
        request.mime =
            core::mime_for_path(&request.target).or_else(|| core::mime_for_path(&request.route));
    }
    Ok(())
}

fn load_request(mut request: core::Request) -> io::Result<core::Request> {
    if request.kind == core::RequestKind::File {
        load_file(&mut request)?;
    }
    Ok(request)
}

fn trimmed_lines(data: &[u8]) -> impl Iterator<Item = &[u8]> {
    data.split_inclusive(|byte| *byte == b'\n')
        .filter_map(|line| {
            let end = line
                .iter()
                .rposition(|byte| !matches!(byte, b'\r' | b'\n' | b'\t' | b' '))
                .map_or(0, |index| index + 1);
            (end > 0).then_some(&line[..end])
        })
}

fn rescan() {
    let Some((root, requests_file, had_requests)) = with_runtime(|runtime| {
        (
            runtime.state.config.root.clone(),
            runtime.state.config.requests_file.clone(),
            !runtime.state.requests.is_empty(),
        )
    }) else {
        return;
    };
    let data = match fs::read(path_from_bytes(&requests_file)) {
        Ok(data) => data,
        Err(error) => {
            let suffix = if error.kind() == io::ErrorKind::NotFound {
                if had_requests {
                    b") not found - requests not changed".as_slice()
                } else {
                    b") not found !!!".as_slice()
                }
            } else if had_requests {
                b") can't be opened - requests not changed".as_slice()
            } else {
                b") can't be opened - no requests !!!".as_slice()
            };
            let mut message = b"guiserv: requests file (".to_vec();
            message.extend_from_slice(&requests_file);
            message.extend_from_slice(suffix);
            log_message(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, &message);
            return;
        }
    };
    let root_path = path_from_bytes(&root);
    let mut requests = Vec::new();
    for line in trimmed_lines(&data) {
        match core::parse_config_line(line, &root_path) {
            Ok(Some(request)) => match load_request(request) {
                Ok(request) => requests.push(request),
                Err(_) => log_path(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"guiserv: can't read content for request: ",
                    line,
                ),
            },
            Ok(None) => {}
            Err(_) => log_path(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"guiserv: wrong request definition: ",
                line,
            ),
        }
    }
    with_runtime_mut(|runtime| {
        runtime.state.requests = requests;
        runtime.rebuild_mirrors();
    });
    log_message(
        MFSLOG_SYSLOG_STDERR,
        MFSLOG_NOTICE,
        b"guiserv: requests have been reloaded",
    );
}

fn quick_rescan() {
    let Some((root, requests_file, previous_stamp)) = with_runtime(|runtime| {
        (
            runtime.state.config.root.clone(),
            runtime.state.config.requests_file.clone(),
            runtime.state.stamp,
        )
    }) else {
        return;
    };
    let root_metadata = fs::metadata(path_from_bytes(&root));
    let request_metadata = fs::metadata(path_from_bytes(&requests_file));
    if let (Ok(root_metadata), Ok(request_metadata)) = (root_metadata, request_metadata) {
        let stamp = core::ScanStamp {
            root_mtime: root_metadata.mtime(),
            requests_mtime: request_metadata.mtime(),
            requests_len: request_metadata.len() as i64,
        };
        if stamp != previous_stamp {
            with_runtime_mut(|runtime| runtime.state.stamp = stamp);
            rescan();
            return;
        }
    }
    with_runtime_mut(|runtime| {
        let mut changed = false;
        for request in &mut runtime.state.requests {
            if request.kind != core::RequestKind::File {
                continue;
            }
            let path = path_from_bytes(&request.target);
            match fs::metadata(&path) {
                Ok(metadata)
                    if request.modified_seconds != metadata.mtime()
                        || request.data.len() as u64 != metadata.len() =>
                {
                    if load_file(request).is_ok() {
                        log_path(
                            MFSLOG_SYSLOG,
                            MFSLOG_NOTICE,
                            b"guiserv: file reloaded: ",
                            &request.target,
                        );
                        changed = true;
                    }
                }
                Ok(_) => {}
                Err(_) => log_path(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"guiserv: can't stat file: ",
                    &request.target,
                ),
            }
        }
        if changed {
            runtime.rebuild_mirrors();
        }
    });
}

fn reload_common() {
    let timeout = configured_timeout();
    let root = configured_string(c"ROOT_DIR", c"/usr/local/share/plfscgi");
    let configured_requests = configured_string(c"REQUESTS_FILE", c"requests.cfg");
    let requests_file = core::requests_path(&path_from_bytes(&root), &configured_requests);
    let os_path = CGI_PATH.with(|path| path.borrow().clone());
    RUNTIME.with(|runtime| {
        let Ok(mut runtime) = runtime.try_borrow_mut() else {
            return;
        };
        if let Some(runtime) = runtime.as_mut() {
            runtime.state.config.root = root;
            runtime.state.config.requests_file = requests_file;
            runtime.state.config.timeout = timeout;
            runtime.state.config.os_path = os_path;
            runtime.state.stamp = core::ScanStamp::default();
        } else {
            *runtime = Some(Runtime {
                state: core::ServerState {
                    config: core::RuntimeConfig {
                        root,
                        requests_file,
                        timeout,
                        listen_host: b"*".to_vec(),
                        listen_port: b"9425".to_vec(),
                        os_path,
                    },
                    requests: Vec::new(),
                    stamp: core::ScanStamp::default(),
                },
                mirrors: Vec::new(),
            });
        }
    });
    quick_rescan();
}

#[derive(Clone, Copy)]
enum ListenerError {
    Socket,
    Resolve,
    Listen,
}

fn open_listener(host: &[u8], port: &[u8]) -> Result<(c_int, u32, u16), ListenerError> {
    let host = CString::new(host).map_err(|_| ListenerError::Resolve)?;
    let port = CString::new(port).map_err(|_| ListenerError::Resolve)?;
    // SAFETY: Category 8 (FFI). tcpsocket has no pointer arguments.
    let socket = unsafe { tcpsocket() };
    if socket < 0 {
        return Err(ListenerError::Socket);
    }
    // SAFETY: Category 8 (FFI). Socket is live; option helpers only mutate descriptor state.
    unsafe {
        tcpnonblock(socket);
        tcpnodelay(socket);
        tcpreuseaddr(socket);
    }
    let mut ip = 0;
    let mut numeric_port = 0;
    // SAFETY: Category 8 (FFI). CStrings and output pointers remain valid for call duration.
    let resolved =
        unsafe { tcpresolve(host.as_ptr(), port.as_ptr(), &mut ip, &mut numeric_port, 1) };
    if resolved < 0 {
        // SAFETY: Category 12 (resource ownership). Socket was created above and is not published.
        unsafe { tcpclose(socket) };
        return Err(ListenerError::Resolve);
    }
    // SAFETY: Category 8 (FFI). Socket and resolved address are valid; queue matches C reference.
    if unsafe { tcpnumlisten(socket, ip, numeric_port, 100) } < 0 {
        // SAFETY: Category 12 (resource ownership). Socket was created above and is not published.
        unsafe { tcpclose(socket) };
        return Err(ListenerError::Listen);
    }
    // SAFETY: Category 8 (FFI). Socket is listening; accept filter is optional.
    if unsafe { tcpsetacceptfilter(socket) } < 0 {
        let unsupported = io::Error::last_os_error().raw_os_error() == Some(libc::ENOTSUP);
        if !unsupported {
            log_message(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_NOTICE,
                b"guiserv: can't set accept filter",
            );
        }
    }
    Ok((socket, ip, numeric_port))
}

fn monotonic_now() -> f64 {
    // SAFETY: Category 8 (FFI). Clock function has no pointer arguments or ownership effects.
    unsafe { monotonic_seconds() }
}

fn remaining_millis(start: f64, timeout: u16) -> Option<u32> {
    let now = monotonic_now();
    let remaining = start + f64::from(timeout) - now;
    (remaining > 0.0).then(|| (remaining * 1_000.0) as u32)
}

fn retryable_errno(error: Option<c_int>) -> bool {
    matches!(error, Some(libc::EAGAIN) | Some(libc::EINTR))
}

fn write_tcp(socket: c_int, data: &[u8], start: f64, timeout: u16) -> bool {
    let Some(milliseconds) = remaining_millis(start, timeout) else {
        return false;
    };
    // SAFETY: Category 8 (FFI). Slice is readable for u32-bounded response length.
    unsafe {
        tcptowrite(
            socket,
            data.as_ptr().cast(),
            data.len() as u32,
            milliseconds,
            milliseconds,
        ) == data.len() as i32
    }
}

fn read_http_request(
    socket: c_int,
    start: f64,
    timeout: u16,
    requests: &[core::Request],
) -> Option<core::ParsedRequest> {
    let mut parser = core::RequestParser::new();
    let mut buffer = [0u8; 4_096];
    loop {
        // SAFETY: Category 8 (FFI). Stack buffer is writable for its declared length.
        let read = unsafe { libc::read(socket, buffer.as_mut_ptr().cast(), buffer.len()) };
        if read == 0 {
            return None;
        }
        if read > 0 {
            match parser.feed(&buffer[..read as usize], requests) {
                core::ParseProgress::Pending => {}
                core::ParseProgress::Ready(request) => return Some(request),
                core::ParseProgress::Closed => return None,
            }
        } else {
            let error = io::Error::last_os_error().raw_os_error();
            if !retryable_errno(error) {
                return None;
            }
        }
        let milliseconds = remaining_millis(start, timeout)?;
        let mut descriptor = libc::pollfd {
            fd: socket,
            events: libc::POLLIN,
            revents: 0,
        };
        // SAFETY: Category 8 (FFI). One valid pollfd is supplied for poll duration.
        let result = unsafe { libc::poll(&mut descriptor, 1, milliseconds as c_int) };
        if result < 0 {
            if io::Error::last_os_error().raw_os_error() != Some(libc::EINTR) {
                return None;
            }
        } else if result == 0
            || descriptor.revents & (libc::POLLERR | libc::POLLHUP) != 0
            || descriptor.revents & libc::POLLIN == 0
        {
            return None;
        }
    }
}

fn resource_from_directory(
    request: &core::Request,
    url: &[u8],
) -> Result<core::Resource, core::Status> {
    let base = path_from_bytes(&request.target);
    let resolved = core::resolve_directory_path(&base, &request.route, url)?;
    let mut file = File::open(&resolved).map_err(|error| match error.kind() {
        io::ErrorKind::NotFound => core::Status::NotFound,
        io::ErrorKind::PermissionDenied => core::Status::Forbidden,
        _ => core::Status::InternalError,
    })?;
    let metadata = file.metadata().map_err(|_| core::Status::InternalError)?;
    if !metadata.file_type().is_file() {
        return Err(core::Status::Forbidden);
    }
    let mut data = vec![0; metadata.len() as u32 as usize];
    file.read_exact(&mut data)
        .map_err(|_| core::Status::InternalError)?;
    Ok(core::Resource {
        mime: core::mime_for_path(resolved.as_os_str().as_bytes()),
        data,
    })
}

fn pipe_read(
    file_descriptor: c_int,
    maximum: usize,
    start: f64,
    timeout: u16,
) -> io::Result<(Vec<u8>, bool)> {
    let mut data = vec![0; maximum];
    let mut length = 0;
    loop {
        // SAFETY: Category 8 (FFI). Remaining Vec region is initialized and writable.
        let read = unsafe {
            libc::read(
                file_descriptor,
                data[length..].as_mut_ptr().cast(),
                maximum - length,
            )
        };
        if read == 0 {
            data.truncate(length);
            return Ok((data, false));
        }
        if read > 0 {
            length += read as usize;
            if length >= maximum {
                data.truncate(length);
                return Ok((data, true));
            }
        } else {
            let error = io::Error::last_os_error();
            if !retryable_errno(error.raw_os_error()) {
                return Err(error);
            }
        }
        let milliseconds = remaining_millis(start, timeout)
            .ok_or_else(|| io::Error::from(io::ErrorKind::TimedOut))?;
        let mut descriptor = libc::pollfd {
            fd: file_descriptor,
            events: libc::POLLIN,
            revents: 0,
        };
        // SAFETY: Category 8 (FFI). One valid pollfd is supplied for poll duration.
        let result = unsafe { libc::poll(&mut descriptor, 1, milliseconds as c_int) };
        if result < 0 && io::Error::last_os_error().raw_os_error() != Some(libc::EINTR) {
            return Err(io::Error::last_os_error());
        }
        if descriptor.revents & libc::POLLERR != 0 {
            return Err(io::Error::other("CGI pipe poll error"));
        }
    }
}

fn stream_cgi(socket: c_int, pipe: c_int, start: f64, timeout: u16) -> bool {
    // SAFETY: Category 8 (FFI). Pipe fd is valid and owned by parent branch.
    unsafe { libc::fcntl(pipe, libc::F_SETFL, libc::O_NONBLOCK) };
    let Ok((first, mut continues)) = pipe_read(pipe, core::FORWARD_BUFFER_SIZE - 1, start, timeout)
    else {
        return false;
    };
    let parsed = core::prepare_cgi_headers(&first);
    if !write_tcp(socket, &parsed.headers, start, timeout)
        || !write_tcp(socket, &first[parsed.body_offset..], start, timeout)
    {
        return false;
    }
    while continues {
        let Ok((chunk, next)) = pipe_read(pipe, core::FORWARD_BUFFER_SIZE, start, timeout) else {
            return false;
        };
        continues = next;
        if !chunk.is_empty() && !write_tcp(socket, &chunk, start, timeout) {
            return false;
        }
    }
    true
}

fn run_cgi(
    socket: c_int,
    request: &core::Request,
    args: Option<&[u8]>,
    os_path: &[u8],
    start: f64,
    timeout: u16,
) -> bool {
    let Ok(executable) = CString::new(request.target.clone()) else {
        return false;
    };
    let Ok(path_environment) = CString::new(os_path) else {
        return false;
    };
    let Ok(query_environment) = CString::new(core::query_string(args, request.extra.as_deref()))
    else {
        return false;
    };
    let mut pipe_descriptors = [-1; 2];
    // SAFETY: Category 8 (FFI). Two-element array is valid pipe output storage.
    if unsafe { libc::pipe(pipe_descriptors.as_mut_ptr()) } < 0 {
        return false;
    }
    let argv = [executable.as_ptr(), std::ptr::null()];
    let environment = [
        c"GATEWAY_INTERFACE=CGI/1.1".as_ptr(),
        c"SERVER_PROTOCOL=HTTP/1.1".as_ptr(),
        c"REQUEST_METHOD=GET".as_ptr(),
        c"SERVER_NAME=mfsgui".as_ptr(),
        path_environment.as_ptr(),
        query_environment.as_ptr(),
        std::ptr::null(),
    ];
    // SAFETY: Category 8 (FFI). Fork occurs in single-threaded connection child with no borrow held.
    let child = unsafe { libc::fork() };
    if child == 0 {
        // SAFETY: Category 8 (FFI). Child mirrors C descriptor/exec sequence; argv/env storage stays live.
        unsafe {
            let null = libc::open(c"/dev/null".as_ptr(), libc::O_RDWR, 0);
            libc::close(libc::STDIN_FILENO);
            libc::dup(null);
            libc::close(libc::STDOUT_FILENO);
            libc::dup(pipe_descriptors[1]);
            libc::close(libc::STDERR_FILENO);
            libc::dup(pipe_descriptors[1]);
            for descriptor in 3..1024 {
                libc::close(descriptor);
            }
            libc::execve(executable.as_ptr(), argv.as_ptr(), environment.as_ptr());
            libc::exit(0);
        }
    }
    if child < 0 {
        // SAFETY: Category 12 (resource ownership). Parent owns both unpublished pipe ends.
        unsafe {
            libc::close(pipe_descriptors[0]);
            libc::close(pipe_descriptors[1]);
        }
        return false;
    }
    // SAFETY: Category 12 (resource ownership). Parent no longer writes CGI pipe.
    unsafe { libc::close(pipe_descriptors[1]) };
    let result = stream_cgi(socket, pipe_descriptors[0], start, timeout);
    // SAFETY: Category 12 (resource ownership). Parent owns read end through stream completion.
    unsafe { libc::close(pipe_descriptors[0]) };
    result
}

fn serve_parsed(
    socket: c_int,
    parsed: &core::ParsedRequest,
    requests: &[core::Request],
    os_path: &[u8],
    start: f64,
    timeout: u16,
) -> bool {
    let request = parsed.route_index.and_then(|index| requests.get(index));
    if let Some(request) = request {
        match request.kind {
            core::RequestKind::Cgi if parsed.method == core::Method::Get => {
                run_cgi(
                    socket,
                    request,
                    parsed.args.as_deref(),
                    os_path,
                    start,
                    timeout,
                );
                return false;
            }
            core::RequestKind::Directory => {
                let resource = resource_from_directory(request, &parsed.url);
                let mut adjusted = parsed.clone();
                let response = match resource {
                    Ok(resource) => {
                        core::prepare_response(&adjusted, Some(request), Some(&resource))
                    }
                    Err(status) => {
                        adjusted.status = status;
                        core::prepare_response(&adjusted, Some(request), None)
                    }
                };
                return write_tcp(socket, &response.headers, start, timeout)
                    && write_tcp(socket, &response.body, start, timeout)
                    && response.keep_alive;
            }
            core::RequestKind::File | core::RequestKind::Redirect | core::RequestKind::Cgi => {}
        }
    }
    let response = core::prepare_response(parsed, request, None);
    write_tcp(socket, &response.headers, start, timeout)
        && write_tcp(socket, &response.body, start, timeout)
        && response.keep_alive
}

fn handle_http_connection(socket: c_int) -> c_int {
    let Some((requests, timeout, os_path)) = with_runtime(|runtime| {
        (
            runtime.state.requests.clone(),
            runtime.state.config.timeout,
            runtime.state.config.os_path.clone(),
        )
    }) else {
        return 1;
    };
    loop {
        let start = monotonic_now();
        let Some(parsed) = read_http_request(socket, start, timeout, &requests) else {
            return 1;
        };
        if !serve_parsed(socket, &parsed, &requests, &os_path, start, timeout) {
            return 0;
        }
    }
}

fn append_abi_line(handle: &mut httphandle, line: &[u8]) {
    let required = line.len().saturating_add(2);
    if handle.outputdata.is_null()
        || handle.outputdataleng as usize + required > core::HTTP_HEADER_MAX
    {
        handle.error = 1;
        return;
    }
    // SAFETY: Category 10 (bounds). Capacity check above proves destination range within 16 KiB ABI buffer.
    unsafe {
        std::ptr::copy_nonoverlapping(
            line.as_ptr(),
            handle.outputdata.add(handle.outputdataleng as usize),
            line.len(),
        );
        handle.outputdataleng += line.len() as u32;
        *handle.outputdata.add(handle.outputdataleng as usize) = b'\r';
        handle.outputdataleng += 1;
        *handle.outputdata.add(handle.outputdataleng as usize) = b'\n';
        handle.outputdataleng += 1;
    }
}

fn abi_status(status: u8) -> core::Status {
    match status {
        0 => core::Status::Ok,
        1 => core::Status::Found,
        2 => core::Status::NotFound,
        3 => core::Status::Forbidden,
        4 => core::Status::NotModified,
        5 => core::Status::BadRequest,
        6 => core::Status::MethodNotAllowed,
        _ => core::Status::InternalError,
    }
}

fn children_kill() {
    CHILDREN.with(|children| {
        for pid in children.borrow_mut().drain(..) {
            // SAFETY: Category 8 (FFI). PID came from successful fork and signal is valid.
            unsafe { libc::kill(pid, libc::SIGKILL) };
        }
    });
}

/// # Safety
/// `line` must point to a writable NUL-terminated configuration line.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_parse_cfgline(line: *mut c_char) -> c_int {
    if line.is_null() {
        return -1;
    }
    // SAFETY: Category 8 (FFI). Caller contract supplies NUL-terminated line.
    let bytes = unsafe { CStr::from_ptr(line) }.to_bytes();
    let Some(root) = with_runtime(|runtime| runtime.state.config.root.clone()) else {
        return -1;
    };
    let request = match core::parse_config_line(bytes, &path_from_bytes(&root)) {
        Ok(Some(request)) => request,
        Ok(None) => return 0,
        Err(_) => return -1,
    };
    let Ok(request) = load_request(request) else {
        return -1;
    };
    with_runtime_mut(|runtime| {
        runtime.state.requests.push(request);
        runtime.rebuild_mirrors();
    });
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_rescan() {
    rescan();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_quick_rescan() {
    quick_rescan();
}

/// # Safety
/// Both pointers must reference readable NUL-terminated strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_expect_str(
    string: *const c_char,
    expected: *const c_char,
) -> c_int {
    if string.is_null() || expected.is_null() {
        return -1;
    }
    // SAFETY: Category 8 (FFI). Caller contract supplies NUL-terminated strings.
    let string = unsafe { CStr::from_ptr(string) }.to_bytes();
    // SAFETY: Category 8 (FFI). Caller contract supplies NUL-terminated strings.
    let expected = unsafe { CStr::from_ptr(expected) }.to_bytes();
    if core::starts_ascii_case_insensitive(string, expected) {
        expected.len() as c_int
    } else {
        -1
    }
}

/// # Safety
/// Both pointers must reference readable NUL-terminated strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_check_header(
    string: *const c_char,
    header: *const c_char,
) -> c_int {
    if string.is_null() || header.is_null() {
        return -1;
    }
    // SAFETY: Category 8 (FFI). Caller contract supplies NUL-terminated strings.
    let string = unsafe { CStr::from_ptr(string) }.to_bytes();
    // SAFETY: Category 8 (FFI). Caller contract supplies NUL-terminated strings.
    let header = unsafe { CStr::from_ptr(header) }.to_bytes();
    core::header_prefix_len(string, header).map_or(-1, |length| length as c_int)
}

unsafe fn apply_first_abi_line(handle: &mut httphandle, line: *mut c_char) {
    // SAFETY: Category 8 (FFI). Exported caller supplies writable NUL-terminated header line.
    let bytes = unsafe { CStr::from_ptr(line) }.to_bytes();
    let requests = with_runtime(|runtime| runtime.state.requests.clone()).unwrap_or_default();
    let mut framed = bytes.to_vec();
    framed.extend_from_slice(b"\r\n\r\n");
    let mut parser = core::RequestParser::new();
    let parsed = match parser.feed(&framed, &requests) {
        core::ParseProgress::Ready(parsed) => parsed,
        core::ParseProgress::Pending | core::ParseProgress::Closed => {
            handle.status = STATUS_BADREQUEST as u8;
            return;
        }
    };
    handle.httpver = parsed.http_version;
    handle.keepalive = parsed.keep_alive as u8;
    handle.status = parsed.status as u8;
    handle.method = parsed.method as u8;
    handle.req = with_runtime_mut(|runtime| {
        parsed
            .route_index
            .map_or(std::ptr::null_mut(), |index| runtime.mirror_pointer(index))
    })
    .unwrap_or(std::ptr::null_mut());
    let prefix = match parsed.method {
        core::Method::Get => 5,
        core::Method::Head => 6,
        core::Method::None => return,
    };
    handle.requrl = line.wrapping_add(prefix);
    let path_end = parsed.url.len();
    // SAFETY: Category 10 (bounds). Parsed offsets were derived from this NUL-terminated line.
    unsafe { *handle.requrl.add(path_end) = 0 };
    let delimiter = bytes.get(prefix + path_end).copied();
    if matches!(delimiter, Some(b'?' | b'&')) {
        handle.reqargs = line.wrapping_add(prefix + path_end + 1);
        if let Some(arguments) = &parsed.args {
            // SAFETY: Category 10 (bounds). Parsed query length lies within same writable line.
            unsafe { *handle.reqargs.add(arguments.len()) = 0 };
        }
    } else {
        handle.reqargs = std::ptr::null_mut();
    }
}

unsafe fn apply_abi_header(handle: &mut httphandle, line: *mut c_char) {
    // SAFETY: Category 8 (FFI). Exported caller supplies NUL-terminated header line.
    let bytes = unsafe { CStr::from_ptr(line) }.to_bytes();
    let Some(separator) = bytes.iter().position(|byte| *byte == b':') else {
        return;
    };
    let name = &bytes[..separator];
    let value = bytes[separator + 1..].trim_ascii_start();
    let request = with_runtime(|runtime| runtime.request_for_mirror(handle.req).cloned()).flatten();
    if let Some(request) = request {
        if request.kind == core::RequestKind::File && name.eq_ignore_ascii_case(b"If-None-Match") {
            if let Some(etag) = request.etag {
                handle.etagmatch = match value.strip_prefix(b"\"") {
                    Some(candidate)
                        if candidate.len() >= 33
                            && candidate[..32] == etag
                            && candidate[32] == b'\"' =>
                    {
                        MATCH_YES as u8
                    }
                    Some(_) | None => MATCH_NO as u8,
                };
            }
        }
        if request.kind == core::RequestKind::File
            && name.eq_ignore_ascii_case(b"If-Modified-Since")
        {
            if let Some(modified) = request.modified_text {
                handle.mtimematch = if value == modified {
                    MATCH_YES as u8
                } else {
                    MATCH_NO as u8
                };
            }
        }
    }
    if name.eq_ignore_ascii_case(b"Connection") {
        if core::starts_ascii_case_insensitive(value, b"close") {
            handle.keepalive = 0;
        }
        if core::starts_ascii_case_insensitive(value, b"keep-alive") {
            handle.keepalive = 1;
        }
    }
}

/// # Safety
/// `handle` and `line` must point to valid writable ABI values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_parse_hdrline(
    handle: *mut httphandle,
    line_number: u32,
    line: *mut c_char,
) {
    if handle.is_null() || line.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Null checks establish valid handle/line for selected ABI adapter.
    unsafe {
        let handle = &mut *handle;
        if line_number == 0 {
            apply_first_abi_line(handle, line);
        } else {
            apply_abi_header(handle, line);
        }
    }
}

/// # Safety
/// `handle` must own a writable NUL-terminated request buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_parse_headers(handle: *mut httphandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Caller contract establishes valid handle.
    let handle_ref = unsafe { &mut *handle };
    if handle_ref.inputdata.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). read_request NUL-terminates initialized input within 16 KiB allocation.
    let initialized = unsafe { CStr::from_ptr(handle_ref.inputdata.cast()) }
        .to_bytes()
        .len();
    // SAFETY: Category 10 (bounds). CStr scan above proves initialized bytes plus terminator are in allocation.
    let data = unsafe { std::slice::from_raw_parts_mut(handle_ref.inputdata, initialized + 1) };
    let mut start = 0;
    let mut line_number = 0;
    while let Some(relative) = data[start..].windows(2).position(|bytes| bytes == b"\r\n") {
        let end = start + relative;
        data[end] = 0;
        // SAFETY: Category 10 (bounds). start is within writable request allocation and now NUL-terminated.
        let pointer = unsafe { handle_ref.inputdata.add(start).cast::<c_char>() };
        // SAFETY: Category 8 (FFI). Local pointers satisfy exported adapter contract.
        unsafe { mfscgiserv_parse_hdrline(handle, line_number, pointer) };
        start = end + 2;
        line_number = line_number.wrapping_add(1);
    }
}

/// Variadic C leaf retained for ABI compatibility; Rust callers use fixed-signature builders.
///
/// # Safety
/// `handle` must own a 16 KiB output buffer and `format`/arguments must obey C printf ABI.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_printf(
    handle: *mut httphandle,
    format: *const c_char,
    arguments: ...
) {
    if handle.is_null() || format.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Caller contract establishes valid mutable handle.
    let handle = unsafe { &mut *handle };
    if handle.outputdata.is_null() || handle.outputdataleng.wrapping_add(2) > HTTP_HEADERMAX as u32
    {
        handle.error = 1;
        return;
    }
    let maximum = (HTTP_HEADERMAX as u32).wrapping_sub(handle.outputdataleng.wrapping_add(2));
    // SAFETY: Category 8 (FFI). Remaining output span and variadic argument list match vsnprintf contract.
    let length = unsafe {
        vsnprintf(
            handle.outputdata.add(handle.outputdataleng as usize).cast(),
            maximum as usize,
            format,
            arguments.clone(),
        )
    };
    if handle
        .outputdataleng
        .wrapping_add(length as u32)
        .wrapping_add(2)
        > HTTP_HEADERMAX as u32
    {
        handle.error = 1;
        return;
    }
    handle.outputdataleng = handle.outputdataleng.wrapping_add(length as u32);
    // SAFETY: Category 10 (bounds). Combined-length check proves CRLF fits output allocation.
    unsafe {
        *handle.outputdata.add(handle.outputdataleng as usize) = b'\r';
        handle.outputdataleng = handle.outputdataleng.wrapping_add(1);
        *handle.outputdata.add(handle.outputdataleng as usize) = b'\n';
        handle.outputdataleng = handle.outputdataleng.wrapping_add(1);
    }
}

/// # Safety
/// `handle` must own a writable 16 KiB input buffer and valid socket.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_read_request(handle: *mut httphandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Caller contract establishes valid mutable handle.
    let handle = unsafe { &mut *handle };
    let timeout = with_runtime(|runtime| runtime.state.config.timeout).unwrap_or(300);
    let mut descriptor = libc::pollfd {
        fd: handle.sock,
        events: libc::POLLIN,
        revents: 0,
    };
    loop {
        if handle.inputdata.is_null() || handle.inputdataleng as usize >= core::HTTP_HEADER_MAX {
            handle.error = 1;
            return;
        }
        let remaining = core::HTTP_HEADER_MAX - handle.inputdataleng as usize;
        // SAFETY: Category 10 (bounds). Remaining length is within caller-provided 16 KiB buffer.
        let read = unsafe {
            libc::read(
                handle.sock,
                handle.inputdata.add(handle.inputdataleng as usize).cast(),
                remaining,
            )
        };
        if read == 0 {
            handle.error = 1;
            return;
        }
        if read < 0 {
            let error = io::Error::last_os_error().raw_os_error();
            if !retryable_errno(error) {
                handle.error = 1;
                return;
            }
        } else {
            let previous = handle.inputdataleng as usize;
            handle.inputdataleng = handle.inputdataleng.wrapping_add(read as u32);
            if handle.inputdataleng as usize >= core::HTTP_HEADER_MAX {
                handle.error = 1;
                return;
            }
            // SAFETY: Category 10 (bounds). Length remains below 16 KiB after check.
            unsafe { *handle.inputdata.add(handle.inputdataleng as usize) = 0 };
            if handle.inputdataleng >= 8 && handle.method == METHOD_NONE as u8 {
                // SAFETY: Category 10 (bounds). At least eight initialized bytes make prefixes readable.
                let prefix = unsafe { std::slice::from_raw_parts(handle.inputdata, 5) };
                if prefix.starts_with(b"GET ") {
                    handle.method = METHOD_GET as u8;
                } else if prefix.starts_with(b"HEAD ") {
                    handle.method = METHOD_HEAD as u8;
                } else {
                    handle.status = STATUS_BADMETHOD as u8;
                    return;
                }
            }
            let search = previous.saturating_sub(3);
            // SAFETY: Category 10 (bounds). Search slice lies within initialized request bytes.
            let data = unsafe {
                std::slice::from_raw_parts_mut(
                    handle.inputdata.add(search),
                    handle.inputdataleng as usize - search,
                )
            };
            if let Some(end) = data.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                data[end + 2] = 0;
                return;
            }
        }
        let Some(milliseconds) = remaining_millis(handle.starttime, timeout) else {
            handle.error = 1;
            return;
        };
        descriptor.revents = 0;
        // SAFETY: Category 8 (FFI). One valid pollfd is supplied for poll duration.
        let result = unsafe { libc::poll(&mut descriptor, 1, milliseconds as c_int) };
        if result < 0 && io::Error::last_os_error().raw_os_error() != Some(libc::EINTR)
            || result == 0
            || descriptor.revents & libc::POLLERR != 0
            || descriptor.revents & libc::POLLIN == 0
        {
            handle.error = 1;
            return;
        }
    }
}

/// # Safety
/// `handle.req` and its data must remain valid for write duration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_write_data(handle: *mut httphandle, skip: u32) {
    if handle.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Caller contract establishes valid handle and selected request.
    let handle = unsafe { &mut *handle };
    if handle.req.is_null() {
        handle.error = 1;
        return;
    }
    // SAFETY: Category 8 (FFI). Selected request remains valid for connection transaction.
    let request = unsafe { &*handle.req };
    if request.fdata.is_null() || skip > request.fsize {
        handle.error = 1;
        return;
    }
    // SAFETY: Category 10 (bounds). skip <= fsize proves remaining raw slice within request data.
    let data = unsafe {
        std::slice::from_raw_parts(
            request.fdata.add(skip as usize),
            request.fsize.wrapping_sub(skip) as usize,
        )
    };
    let timeout = with_runtime(|runtime| runtime.state.config.timeout).unwrap_or(300);
    if !write_tcp(handle.sock, data, handle.starttime, timeout) {
        handle.error = 1;
    }
}

/// # Safety
/// `handle` and its output buffer must remain valid for write duration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_write_headers(handle: *mut httphandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Caller contract establishes valid handle.
    let handle = unsafe { &mut *handle };
    if handle.outputdata.is_null() {
        handle.error = 1;
        return;
    }
    // SAFETY: Category 10 (bounds). outputdataleng is maintained within 16 KiB buffer.
    let data =
        unsafe { std::slice::from_raw_parts(handle.outputdata, handle.outputdataleng as usize) };
    let timeout = with_runtime(|runtime| runtime.state.config.timeout).unwrap_or(300);
    if !write_tcp(handle.sock, data, handle.starttime, timeout) {
        handle.error = 1;
    }
}

/// # Safety
/// `handle` must point to a valid writable ABI handle and output buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_status(handle: *mut httphandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Caller contract establishes valid mutable handle.
    let handle = unsafe { &mut *handle };
    append_abi_line(handle, abi_status(handle.status).line());
}

/// # Safety
/// `handle` and selected request pointers must remain valid for call duration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_prepare_headers(handle: *mut httphandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Caller contract establishes valid mutable handle.
    let handle = unsafe { &mut *handle };
    if handle.req.is_null() {
        handle.status = STATUS_NOTFOUND as u8;
    }
    if handle.status == STATUS_NONE as u8
        && (handle.etagmatch == MATCH_YES as u8
            || handle.etagmatch == MATCH_UNKNOWN as u8 && handle.mtimematch == MATCH_YES as u8)
    {
        handle.status = STATUS_NOTMODIFIED as u8;
    }
    if handle.status != STATUS_NONE as u8 {
        handle.keepalive = 0;
    }
    append_abi_line(handle, abi_status(handle.status).line());
    append_abi_line(handle, b"Server: mfsgui");
    append_abi_line(
        handle,
        if handle.keepalive != 0 {
            b"Connection: keep-alive"
        } else {
            b"Connection: close"
        },
    );
    if !handle.req.is_null() {
        // SAFETY: Category 8 (FFI). Selected request remains valid for transaction.
        let request = unsafe { &*handle.req };
        if handle.status == STATUS_NONE as u8 {
            let content_type = if !request.extra.is_null() {
                // SAFETY: Category 8 (FFI). Request mirror owns NUL-terminated extra.
                unsafe { CStr::from_ptr(request.extra) }.to_bytes()
            } else if !request.mimetype.is_null() {
                // SAFETY: Category 8 (FFI). MIME pointer targets static NUL-terminated string.
                unsafe { CStr::from_ptr(request.mimetype) }.to_bytes()
            } else {
                b"text/plain"
            };
            let mut line = b"Content-Type: ".to_vec();
            line.extend_from_slice(content_type);
            append_abi_line(handle, &line);
            append_abi_line(
                handle,
                format!("Content-Length: {}", request.fsize).as_bytes(),
            );
        }
        if handle.status == STATUS_FOUND as u8 {
            // SAFETY: Category 8 (FFI). Redirect target is mirror-owned NUL-terminated string.
            let target = unsafe { CStr::from_ptr(request.fname) }.to_bytes();
            let args = if handle.reqargs.is_null() {
                None
            } else {
                // SAFETY: Category 8 (FFI). reqargs points into writable NUL-terminated request line.
                Some(unsafe { CStr::from_ptr(handle.reqargs) }.to_bytes())
            };
            let extra = if request.extra.is_null() {
                None
            } else {
                // SAFETY: Category 8 (FFI). Request mirror owns NUL-terminated extra.
                Some(unsafe { CStr::from_ptr(request.extra) }.to_bytes())
            };
            let mut location = b"Location: /".to_vec();
            location.extend_from_slice(target);
            if let Some(args) = args {
                location.push(b'?');
                location.extend_from_slice(args);
            }
            if let Some(extra) = extra {
                location.push(if args.is_some() { b'&' } else { b'?' });
                location.extend_from_slice(extra);
            }
            append_abi_line(handle, &location);
        } else {
            append_abi_line(handle, b"Cache-Control: public,max-age=0");
            if !request.etag.is_null() {
                // SAFETY: Category 8 (FFI). Request mirror owns NUL-terminated ETag.
                let etag = unsafe { CStr::from_ptr(request.etag) }.to_bytes();
                let mut line = b"ETag: \"".to_vec();
                line.extend_from_slice(etag);
                line.push(b'\"');
                append_abi_line(handle, &line);
            }
            if !request.mtimestr.is_null() {
                // SAFETY: Category 8 (FFI). Request mirror owns NUL-terminated timestamp.
                let modified = unsafe { CStr::from_ptr(request.mtimestr) }.to_bytes();
                let mut line = b"Last-Modified: ".to_vec();
                line.extend_from_slice(modified);
                append_abi_line(handle, &line);
            }
        }
    }
    append_abi_line(handle, b"");
}

/// # Safety
/// `handle.req` must own a forwarding buffer of at least `maximum` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_piperead(
    handle: *mut httphandle,
    file_descriptor: c_int,
    maximum: u32,
) -> c_int {
    if handle.is_null() {
        return 0;
    }
    // SAFETY: Category 8 (FFI). Caller contract establishes valid mutable handle.
    let handle = unsafe { &mut *handle };
    if handle.req.is_null() {
        handle.error = 1;
        return 0;
    }
    let timeout = with_runtime(|runtime| runtime.state.config.timeout).unwrap_or(300);
    let result = pipe_read(file_descriptor, maximum as usize, handle.starttime, timeout);
    let Ok((data, continues)) = result else {
        handle.error = 1;
        return 0;
    };
    // SAFETY: Category 8 (FFI). Selected CGI request owns maximum-sized forwarding buffer.
    let request = unsafe { &mut *handle.req };
    if request.fdata.is_null() {
        handle.error = 1;
        return 0;
    }
    // SAFETY: Category 10 (bounds). pipe_read caps data at caller-provided maximum.
    unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), request.fdata, data.len()) };
    request.fsize = data.len() as u32;
    continues as c_int
}

/// # Safety
/// `handle` must reference a live socket transaction; `file_descriptor` is readable CGI pipe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_handle_cgioutput(
    handle: *mut httphandle,
    file_descriptor: c_int,
) {
    if handle.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Caller contract establishes valid mutable handle.
    let handle = unsafe { &mut *handle };
    let timeout = with_runtime(|runtime| runtime.state.config.timeout).unwrap_or(300);
    if !stream_cgi(handle.sock, file_descriptor, handle.starttime, timeout) {
        handle.error = 1;
    }
}

/// # Safety
/// `handle.req` must reference a CGI request and handle socket must be live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_handle_cgi(handle: *mut httphandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Caller contract establishes valid mutable handle.
    let handle = unsafe { &mut *handle };
    let request = with_runtime(|runtime| runtime.request_for_mirror(handle.req).cloned()).flatten();
    let Some(request) = request else {
        handle.error = 1;
        return;
    };
    let args = if handle.reqargs.is_null() {
        None
    } else {
        // SAFETY: Category 8 (FFI). reqargs points into NUL-terminated request buffer.
        Some(unsafe { CStr::from_ptr(handle.reqargs) }.to_bytes())
    };
    let Some((path, timeout)) = with_runtime(|runtime| {
        (
            runtime.state.config.os_path.clone(),
            runtime.state.config.timeout,
        )
    }) else {
        handle.error = 1;
        return;
    };
    if !run_cgi(
        handle.sock,
        &request,
        args,
        &path,
        handle.starttime,
        timeout,
    ) {
        handle.error = 1;
    }
}

/// # Safety
/// `handle.req` must reference a directory request and remain writable for loaded file mirror.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_handle_dir(handle: *mut httphandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Caller contract establishes valid mutable handle.
    let handle = unsafe { &mut *handle };
    let request = with_runtime(|runtime| runtime.request_for_mirror(handle.req).cloned()).flatten();
    let Some(request) = request else {
        handle.status = STATUS_INTERROR as u8;
        return;
    };
    let url = if handle.requrl.is_null() {
        Vec::new()
    } else {
        // SAFETY: Category 8 (FFI). requrl points into NUL-terminated request buffer.
        unsafe { CStr::from_ptr(handle.requrl) }.to_bytes().to_vec()
    };
    let resource = match resource_from_directory(&request, &url) {
        Ok(resource) => resource,
        Err(status) => {
            handle.status = status as u8;
            return;
        }
    };
    // SAFETY: Category 8 (FFI). Selected mirror request is uniquely mutable in connection child.
    let raw = unsafe { &mut *handle.req };
    // SAFETY: Category 8 (FFI). malloc returns C-compatible storage for exported ABI buffer.
    let data = unsafe { libc::malloc(resource.data.len().max(1)) }.cast::<u8>();
    if data.is_null() {
        handle.status = STATUS_INTERROR as u8;
        return;
    }
    if !resource.data.is_empty() {
        // SAFETY: Category 10 (bounds). malloc size is at least source length.
        unsafe { std::ptr::copy_nonoverlapping(resource.data.as_ptr(), data, resource.data.len()) };
    }
    raw.fdata = data;
    raw.fsize = resource.data.len() as u32;
    raw.mimetype = mime_c_pointer(resource.mime);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_handle_httpconn(socket: c_int) {
    let status = handle_http_connection(socket);
    // SAFETY: Category 12 (resource ownership). Connection child exclusively owns accepted socket.
    unsafe { tcpclose(socket) };
    // SAFETY: Category 8 (FFI). This ABI function runs only in forked connection child.
    unsafe { libc::exit(status) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_chldend(pid: pid_t, _status: c_int) {
    CHILDREN.with(|children| children.borrow_mut().retain(|child| *child != pid));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_handle_newconn(socket: c_int) {
    // SAFETY: Category 8 (FFI). Daemon event loop is single-threaded and holds no state borrow.
    let child = unsafe { libc::fork() };
    if child == 0 {
        LISTENER.with(|listener| {
            // SAFETY: Category 12 (resource ownership). Child closes inherited listener copy only.
            unsafe { libc::close(listener.get()) };
        });
        // SAFETY: Category 8 (FFI). Child owns accepted socket and function exits process.
        unsafe { mfscgiserv_handle_httpconn(socket) };
    } else if child < 0 {
        log_message(MFSLOG_SYSLOG, MFSLOG_WARNING, b"guiserv: fork error");
        // SAFETY: Category 12 (resource ownership). Failed fork leaves accepted socket in parent.
        unsafe { libc::close(socket) };
    } else {
        CHILDREN.with(|children| children.borrow_mut().push(child));
        // SAFETY: Category 8 (FFI). PID and callback remain valid until child reap.
        unsafe {
            main_chld_register_fname(
                child,
                Some(mfscgiserv_chldend),
                c"mfscgiserv_chldend".as_ptr(),
            )
        };
        // SAFETY: Category 12 (resource ownership). Parent delegates accepted socket to child.
        unsafe { libc::close(socket) };
    }
}

/// # Safety
/// `descriptors` must have capacity at `*count`; pointers remain valid for callback duration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_desc(descriptors: *mut pollfd, count: *mut u32) {
    if descriptors.is_null() || count.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Caller contract supplies writable count.
    let position = unsafe { *count };
    LISTENER_POLL_POSITION.with(|stored| stored.set(position));
    // SAFETY: Category 10 (bounds). Daemon poll registry guarantees append capacity.
    unsafe {
        *descriptors.add(position as usize) = libc::pollfd {
            fd: LISTENER.with(Cell::get),
            events: libc::POLLIN,
            revents: 0,
        };
        *count = position.wrapping_add(1);
    }
}

/// # Safety
/// `descriptors` must reference poll array previously populated by `mfscgiserv_desc`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_serve(descriptors: *mut pollfd) {
    if descriptors.is_null() {
        return;
    }
    let position = LISTENER_POLL_POSITION.with(Cell::get);
    // SAFETY: Category 10 (bounds). Position was recorded during matching desc callback.
    let ready = unsafe { (*descriptors.add(position as usize)).revents & libc::POLLIN != 0 };
    if !ready {
        return;
    }
    let listener = LISTENER.with(Cell::get);
    // SAFETY: Category 8 (FFI). Listener is live daemon socket.
    let socket = unsafe { tcpaccept(listener) };
    if socket < 0 {
        log_message(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"guiserv: accept error",
        );
        return;
    }
    // SAFETY: Category 8 (FFI). Accepted socket is live and exclusively owned here.
    unsafe {
        tcpnonblock(socket);
        tcpnodelay(socket);
        mfscgiserv_handle_newconn(socket);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_reload_common() {
    reload_common();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_reload() {
    reload_common();
    let new_host = configured_string(c"GUISERV_LISTEN_HOST", c"*");
    let new_port = configured_string(c"GUISERV_LISTEN_PORT", c"9425");
    let Some((old_host, old_port)) = with_runtime(|runtime| {
        (
            runtime.state.config.listen_host.clone(),
            runtime.state.config.listen_port.clone(),
        )
    }) else {
        return;
    };
    if old_host == new_host && old_port == new_port {
        let mut message = b"guiserv: socket address hasn't changed (".to_vec();
        message.extend_from_slice(&new_host);
        message.push(b':');
        message.extend_from_slice(&new_port);
        message.push(b')');
        log_message(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, &message);
        return;
    }
    let Ok((new_listener, _, _)) = open_listener(&new_host, &new_port) else {
        log_message(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"guiserv: socket address changed, but new listener failed",
        );
        return;
    };
    with_runtime_mut(|runtime| {
        runtime.state.config.listen_host = new_host.clone();
        runtime.state.config.listen_port = new_port.clone();
    });
    LISTENER.with(|listener| {
        let old = listener.replace(new_listener);
        if old >= 0 {
            // SAFETY: Category 12 (resource ownership). Atomic replacement transfers old listener close ownership.
            unsafe { tcpclose(old) };
        }
    });
    let mut message = b"guiserv: socket address changed, now listen on ".to_vec();
    message.extend_from_slice(&new_host);
    message.push(b':');
    message.extend_from_slice(&new_port);
    log_message(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, &message);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_term() {
    let address = with_runtime(|runtime| {
        let mut address = runtime.state.config.listen_host.clone();
        address.push(b':');
        address.extend_from_slice(&runtime.state.config.listen_port);
        address
    })
    .unwrap_or_default();
    log_path(MFSLOG_SYSLOG, MFSLOG_INFO, b"guiserv: closing ", &address);
    LISTENER.with(|listener| {
        let socket = listener.replace(-1);
        if socket >= 0 {
            // SAFETY: Category 12 (resource ownership). Listener cell transfers sole close ownership.
            unsafe { tcpclose(socket) };
        }
    });
    RUNTIME.with(|runtime| *runtime.borrow_mut() = None);
    children_kill();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_prepare_path() {
    let mut path = b"PATH=".to_vec();
    if let Some(value) = std::env::var_os("PATH") {
        path.extend_from_slice(value.as_os_str().as_bytes());
    }
    CGI_PATH.with(|stored| *stored.borrow_mut() = path.clone());
    with_runtime_mut(|runtime| runtime.state.config.os_path = path);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_init() -> c_int {
    // SAFETY: Category 8 (FFI). Initialization is single-threaded and prepares process-owned CGI environment.
    unsafe { mfscgiserv_prepare_path() };
    reload_common();
    let host = configured_string(c"GUISERV_LISTEN_HOST", c"*");
    let port = configured_string(c"GUISERV_LISTEN_PORT", c"9425");
    let listener = match open_listener(&host, &port) {
        Ok((listener, _, _)) => listener,
        Err(error) => {
            let message = match error {
                ListenerError::Socket => b"guiserv: can't create socket".as_slice(),
                ListenerError::Resolve => b"guiserv: can't resolve listen address".as_slice(),
                ListenerError::Listen => b"guiserv: can't listen on socket".as_slice(),
            };
            log_message(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_ERR, message);
            return -1;
        }
    };
    with_runtime_mut(|runtime| {
        runtime.state.config.listen_host = host.clone();
        runtime.state.config.listen_port = port.clone();
    });
    LISTENER.with(|stored| stored.set(listener));
    let mut message = b"guiserv: listen on ".to_vec();
    message.extend_from_slice(&host);
    message.push(b':');
    message.extend_from_slice(&port);
    log_message(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, &message);
    // SAFETY: Category 8 (FFI). Exported callbacks and static names remain valid for daemon lifetime.
    unsafe {
        main_reload_register_fname(Some(mfscgiserv_reload), c"mfscgiserv_reload".as_ptr());
        main_destruct_register_fname(Some(mfscgiserv_term), c"mfscgiserv_term".as_ptr());
        main_poll_register_fname(
            Some(mfscgiserv_desc),
            Some(mfscgiserv_serve),
            c"mfscgiserv_desc".as_ptr(),
            c"mfscgiserv_serve".as_ptr(),
        );
        main_time_register_fname(
            1,
            0,
            Some(mfscgiserv_quick_rescan),
            c"mfscgiserv_quick_rescan".as_ptr(),
        );
    }
    0
}
