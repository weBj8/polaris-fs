//! Metalogger master protocol. Rust owns connection, packet, and file state;
//! raw pointers remain only at daemon, socket, config, CRC, and stdio ABI edges.
// allow: SIZE_OK — 33 frozen C ABI exports must share one runtime boundary.

#[path = "masterconn_core.rs"]
mod core;

use self::core::{Connection, Control, Mode, Packet, ProtocolError};
use crate::mfslog::{MFSLOG_ERR, MFSLOG_INFO, MFSLOG_NOTICE, MFSLOG_WARNING};
use libc::{c_char, c_double, c_int, c_void};
use std::ffi::{CStr, CString, OsStr};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::FileExt;
use std::os::unix::io::IntoRawFd;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

pub type FILE = c_void;

#[repr(C)]
pub struct masterconn {
    _private: [u8; 0],
}

pub type pollfd = libc::pollfd;

pub const ANTOAN_NOP: c_int = 0;
pub const ANTOAN_FORCE_TIMEOUT: u32 = 5;
pub const MATOAN_METACHANGES_LOG: u32 = 51;
pub const MATOAN_MASTER_ACK: u32 = 52;
pub const ANTOMA_DOWNLOAD_START: c_int = 60;
pub const MATOAN_DOWNLOAD_INFO: u32 = 61;
pub const ANTOMA_DOWNLOAD_REQUEST: c_int = 62;
pub const MATOAN_DOWNLOAD_DATA: u32 = 63;
pub const ANTOMA_DOWNLOAD_END: c_int = 64;
const MFSLOG_SYSLOG: c_int = 0;
const MFSLOG_SYSLOG_STDERR: c_int = 2;
const MFSLOG_ERRNO_SYSLOG_STDERR: c_int = 3;

unsafe extern "C" {
    fn cfg_getstr(name: *const c_char, default: *const c_char) -> *mut c_char;
    fn cfg_getuint32(name: *const c_char, default: u32) -> u32;
    fn main_destruct_register_fname(callback: Option<unsafe extern "C" fn()>, name: *const c_char);
    fn main_reload_register_fname(callback: Option<unsafe extern "C" fn()>, name: *const c_char);
    fn main_info_register_fname(
        callback: Option<unsafe extern "C" fn(*mut FILE)>,
        name: *const c_char,
    );
    fn main_poll_register_fname(
        desc: Option<unsafe extern "C" fn(*mut pollfd, *mut u32)>,
        serve: Option<unsafe extern "C" fn(*mut pollfd)>,
        desc_name: *const c_char,
        serve_name: *const c_char,
    );
    fn main_time_register_fname(
        seconds: u32,
        offset: u32,
        callback: Option<unsafe extern "C" fn()>,
        name: *const c_char,
    ) -> *mut c_void;
    fn main_time_change(handle: *mut c_void, seconds: u32, offset: u32) -> c_int;
    fn tcpsocket() -> c_int;
    fn tcpresolve(
        host: *const c_char,
        service: *const c_char,
        ip: *mut u32,
        port: *mut u16,
        passive: c_int,
    ) -> c_int;
    fn tcpnonblock(socket: c_int) -> c_int;
    fn tcpgetstatus(socket: c_int) -> c_int;
    fn tcpnodelay(socket: c_int) -> c_int;
    fn tcpnumbind(socket: c_int, ip: u32, port: u16) -> c_int;
    fn tcpnumconnect(socket: c_int, ip: u32, port: u16) -> c_int;
    fn tcpclose(socket: c_int) -> c_int;
    fn univmakestrip(buffer: *mut c_char, ip: u32);
    fn univmakestripport(buffer: *mut c_char, ip: u32, port: u16);
    fn monotonic_seconds() -> c_double;
    fn monotonic_useconds() -> u64;
    fn mycrc32(crc: u32, block: *const c_void, length: u32) -> u32;
    fn mfs_log(mode: c_int, priority: c_int, format: *const c_char, ...);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
}

fn log_message(mode: c_int, priority: c_int, message: &CStr) {
    // SAFETY: Message is a static or owned NUL-terminated format without operands.
    unsafe { mfs_log(mode, priority, message.as_ptr()) };
}

struct Config {
    host: CString,
    port: CString,
    bind_host: CString,
    timeout: u16,
    back_logs: u32,
    back_meta_copies: u32,
    reconnect_delay: u32,
    meta_frequency: u32,
}

struct Runtime {
    connection: Connection,
    config: Config,
    socket: c_int,
    poll_position: i32,
    bind_ip: u32,
    master_ip: u32,
    master_port: u16,
    address_valid: bool,
    downloading: u8,
    old_mode: bool,
    meta_file: Option<File>,
    log_file: Option<File>,
    file_size: u64,
    download_offset: u64,
    download_started: u64,
    download_retries: u8,
    last_log_version: u64,
    bytes_in: u32,
    bytes_out: u32,
    reconnect_hook: usize,
    download_hook: usize,
}

static RUNTIME: Mutex<Option<Runtime>> = Mutex::new(None);

fn runtime() -> MutexGuard<'static, Option<Runtime>> {
    match RUNTIME.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

unsafe fn config_string(name: &'static CStr, default: &'static CStr) -> CString {
    // SAFETY: Category 8 (FFI). cfg contract returns malloc-owned NUL-terminated string or null.
    let pointer = unsafe { cfg_getstr(name.as_ptr(), default.as_ptr()) };
    if pointer.is_null() {
        return default.to_owned();
    }
    // SAFETY: Category 8 (FFI). cfg contract guarantees valid C string until freed below.
    let value = unsafe { CStr::from_ptr(pointer) }.to_owned();
    // SAFETY: Category 8 (FFI). Pointer came from cfg_getstr's malloc-compatible allocator.
    unsafe { libc::free(pointer.cast()) };
    value
}

unsafe fn load_config() -> Config {
    // SAFETY: Category 8 (FFI). Constant names/defaults satisfy cfg API contracts.
    let host = unsafe { config_string(c"MASTER_HOST", c"mfsmaster") };
    // SAFETY: Category 8 (FFI). Constant names/defaults satisfy cfg API contracts.
    let port = unsafe { config_string(c"MASTER_PORT", c"9419") };
    // SAFETY: Category 8 (FFI). Constant names/defaults satisfy cfg API contracts.
    let bind_host = unsafe { config_string(c"BIND_HOST", c"*") };
    // SAFETY: Category 8 (FFI). cfg getters accept static NUL-terminated keys.
    let timeout = unsafe { cfg_getuint32(c"MASTER_TIMEOUT".as_ptr(), 10) };
    // SAFETY: Same cfg contract.
    let back_logs = unsafe { cfg_getuint32(c"BACK_LOGS".as_ptr(), 50) };
    // SAFETY: Same cfg contract.
    let back_meta_copies = unsafe { cfg_getuint32(c"BACK_META_KEEP_PREVIOUS".as_ptr(), 3) };
    // SAFETY: Same cfg contract.
    let reconnect_delay = unsafe { cfg_getuint32(c"MASTER_RECONNECTION_DELAY".as_ptr(), 5) };
    // SAFETY: Same cfg contract.
    let meta_frequency = unsafe { cfg_getuint32(c"META_DOWNLOAD_FREQ".as_ptr(), 24) };
    let (timeout, back_logs, meta_frequency) =
        core::clamp_config(timeout, back_logs, meta_frequency);
    Config {
        host,
        port,
        bind_host,
        timeout,
        back_logs,
        back_meta_copies,
        reconnect_delay,
        meta_frequency,
    }
}

fn find_last_log_version() -> u64 {
    let Ok(metadata) = fs::metadata("metadata_ml.mfs.back") else {
        return 0;
    };
    if !metadata.is_file() || metadata.len() == 0 {
        return 0;
    }
    let Ok(mut file) = OpenOptions::new()
        .read(true)
        .write(true)
        .open("changelog_ml.0.back")
    else {
        return 0;
    };
    let Ok(size) = file.metadata().map(|m| m.len()) else {
        return 0;
    };
    let start = size.saturating_sub(229_376);
    if file.seek(SeekFrom::Start(start)).is_err() {
        return 0;
    }
    let mut bytes = Vec::new();
    if file.read_to_end(&mut bytes).is_err() {
        return 0;
    }
    let Some(last) = bytes.iter().rposition(|byte| *byte == b'\n') else {
        return 0;
    };
    let Some(previous) = bytes[..last].iter().rposition(|byte| *byte == b'\n') else {
        return 0;
    };
    if start + last as u64 + 1 != size && file.set_len(start + last as u64 + 1).is_err() {
        return 0;
    }
    let line_start = previous + 1;
    let mut version = 0u64;
    let mut position = line_start;
    while position < bytes.len() && bytes[position].is_ascii_digit() {
        version = version
            .wrapping_mul(10)
            .wrapping_add(u64::from(bytes[position] - b'0'));
        position += 1;
    }
    if bytes.get(position) == Some(&b':') {
        version
    } else {
        0
    }
}

fn metadata_valid(path: &Path) -> bool {
    let Ok(mut file) = File::open(path) else {
        return false;
    };
    let mut header = [0u8; 8];
    if file.read_exact(&mut header).is_err() || &header == b"MFSM NEW" {
        return false;
    }
    if &header[..5] != b"MFSM "
        || !(b'1'..=b'9').contains(&header[5])
        || header[6] != b'.'
        || !header[7].is_ascii_digit()
    {
        return false;
    }
    let version = ((header[5] - b'0') << 4) + header[7] - b'0';
    if version >= 0x20 {
        let mut identity = [0u8; 16];
        if file.read_exact(&mut identity).is_err() {
            return false;
        }
    }
    if file.seek(SeekFrom::End(-16)).is_err() {
        return false;
    }
    let mut marker = [0u8; 16];
    if file.read_exact(&mut marker).is_err() {
        return false;
    }
    if version < 0x17 {
        marker == [0; 16]
    } else {
        &marker == b"[MFS EOF MARKER]"
    }
}

fn queue_download_start(rt: &mut Runtime, file_number: u8) {
    if rt.connection.mode == Mode::Data && rt.downloading == 0 {
        rt.connection
            .queue_packet(ANTOMA_DOWNLOAD_START as u32, &[file_number]);
        rt.downloading = file_number;
    }
}

fn download_end(rt: &mut Runtime) -> c_int {
    rt.downloading = 0;
    rt.connection.queue_packet(ANTOMA_DOWNLOAD_END as u32, &[]);
    if let Some(file) = rt.meta_file.take() {
        // SAFETY: Category 8 (FFI). into_raw_fd transfers sole descriptor ownership to close.
        if unsafe { libc::close(file.into_raw_fd()) } < 0 {
            log_message(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                c"error closing metafile",
            );
            return -1;
        }
    }
    0
}

fn download_next(rt: &mut Runtime) {
    if rt.download_offset < rt.file_size {
        let length = (rt.file_size - rt.download_offset).min(u64::from(core::META_DL_BLOCK)) as u32;
        let mut payload = Vec::with_capacity(12);
        payload.extend_from_slice(&rt.download_offset.to_be_bytes());
        payload.extend_from_slice(&length.to_be_bytes());
        rt.connection
            .queue_packet(ANTOMA_DOWNLOAD_REQUEST as u32, &payload);
        return;
    }
    let file_number = rt.downloading;
    if download_end(rt) < 0 {
        return;
    }
    // SAFETY: Monotonic clock takes no arguments and returns a value.
    let elapsed = unsafe { monotonic_useconds() }
        .wrapping_sub(rt.download_started)
        .max(1);
    let file_name = match file_number {
        1 => c"metadata",
        11 => c"changelog_0",
        12 => c"changelog_1",
        _ => c"???",
    };
    // SAFETY: Static format matches file name, byte/time values, and throughput double.
    unsafe {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            c"%s downloaded %luB/%lu.%06us (%.3lf MB/s)".as_ptr(),
            file_name.as_ptr(),
            rt.file_size,
            elapsed / 1_000_000,
            (elapsed % 1_000_000) as u32,
            rt.file_size as f64 / elapsed as f64,
        )
    };
    match file_number {
        1 => {
            if metadata_valid(Path::new("metadata_ml.tmp")) {
                if rt.config.back_meta_copies > 0 {
                    for index in (1..rt.config.back_meta_copies).rev() {
                        let _ = fs::rename(
                            format!("metadata_ml.mfs.back.{index}"),
                            format!("metadata_ml.mfs.back.{}", index + 1),
                        );
                    }
                    let _ = fs::rename("metadata_ml.mfs.back", "metadata_ml.mfs.back.1");
                }
                let _ = fs::rename("metadata_ml.tmp", "metadata_ml.mfs.back");
            }
            if !rt.old_mode {
                queue_download_start(rt, 11);
            }
        }
        11 => {
            let _ = fs::rename("changelog_ml.tmp", "changelog_ml_back.0.mfs");
            queue_download_start(rt, 12);
        }
        12 => {
            let _ = fs::rename("changelog_ml.tmp", "changelog_ml_back.1.mfs");
        }
        _ => {}
    }
}

fn before_close(rt: &mut Runtime) {
    if matches!(rt.downloading, 11 | 12) {
        log_message(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            c"old master detected - please upgrade your master server and then restart metalogger",
        );
        rt.old_mode = true;
    }
    if rt.meta_file.take().is_some() {
        let _ = fs::remove_file("metadata_ml.tmp");
        let _ = fs::remove_file("changelog_ml.tmp");
    }
    rt.log_file.take();
}

fn dispatch_packet(rt: &mut Runtime, packet: Packet) {
    match rt
        .connection
        .apply_control(packet.packet_type, &packet.payload)
    {
        Ok(Control::Handled) => return,
        Err(error) => {
            let message = match (error, packet.packet_type) {
                (ProtocolError::WrongSize, MATOAN_MASTER_ACK) => {
                    c"MATOAN_MASTER_ACK - wrong size"
                }
                (ProtocolError::WrongSize, _) => c"master packet - wrong size",
                (ProtocolError::UnknownCommand, _) => c"unknown packet from master",
            };
            log_message(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, message);
            return;
        }
        Ok(Control::Dispatch) => {}
    }
    match packet.packet_type {
        MATOAN_METACHANGES_LOG => metachange(rt, &packet.payload),
        MATOAN_DOWNLOAD_INFO => download_info(rt, &packet.payload),
        MATOAN_DOWNLOAD_DATA => download_data(rt, &packet.payload),
        _ => rt.connection.mode = Mode::Kill,
    }
}

fn metachange(rt: &mut Runtime, payload: &[u8]) {
    if payload == [0x55] {
        rt.log_file.take();
        for index in (1..=rt.config.back_logs).rev() {
            let _ = fs::rename(
                format!("changelog_ml.{}.mfs", index - 1),
                format!("changelog_ml.{index}.mfs"),
            );
        }
        return;
    }
    if payload.len() < 10 || payload[0] != 0xff || payload.last() != Some(&0) {
        log_message(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            c"MATOAN_METACHANGES_LOG - wrong size",
        );
        rt.connection.mode = Mode::Kill;
        return;
    }
    let version = u64::from_be_bytes(payload[1..9].try_into().unwrap_or([0; 8]));
    if rt.last_log_version > 0 && version != rt.last_log_version.wrapping_add(1) {
        // SAFETY: Static format matches inclusive lost-version bounds.
        unsafe {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                c"some changes lost: [%lu-%lu], download metadata again".as_ptr(),
                rt.last_log_version,
                version.wrapping_sub(1),
            )
        };
        rt.log_file.take();
        for index in 0..=rt.config.back_logs {
            let _ = fs::remove_file(format!("changelog_ml.{index}.mfs"));
        }
        rt.last_log_version = 0;
        rt.connection.mode = Mode::Kill;
        return;
    }
    if rt.log_file.is_none() {
        rt.log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("changelog_ml.0.mfs")
            .ok();
    }
    if let Some(file) = rt.log_file.as_mut() {
        let written = write!(file, "{version}: ")
            .and_then(|()| file.write_all(&payload[9..payload.len() - 1]))
            .and_then(|()| file.write_all(b"\n"));
        if written.is_ok() {
            rt.last_log_version = version;
        }
    }
}

fn download_info(rt: &mut Runtime, payload: &[u8]) {
    if payload.len() == 1 {
        rt.downloading = 0;
        return;
    }
    if payload.len() != 8 {
        log_message(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            c"MATOAN_DOWNLOAD_INFO - wrong size",
        );
        rt.connection.mode = Mode::Kill;
        return;
    }
    rt.file_size = u64::from_be_bytes(payload.try_into().unwrap_or([0; 8]));
    rt.download_offset = 0;
    rt.download_retries = 0;
    // SAFETY: Category 8 (FFI). Monotonic clock takes no arguments and returns value.
    rt.download_started = unsafe { monotonic_useconds() };
    let path = match rt.downloading {
        1 => "metadata_ml.tmp",
        11 | 12 => "changelog_ml.tmp",
        _ => {
            rt.connection.mode = Mode::Kill;
            return;
        }
    };
    rt.meta_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .ok();
    if rt.meta_file.is_none() {
        log_message(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_ERR,
            c"can't open metadata download file",
        );
        download_end(rt);
    } else {
        download_next(rt);
    }
}

fn download_data(rt: &mut Runtime, payload: &[u8]) {
    let Ok(block) = core::parse_download_block(payload) else {
        rt.connection.mode = Mode::Kill;
        return;
    };
    if rt.meta_file.is_none()
        || block.offset != rt.download_offset
        || block.offset.wrapping_add(block.data.len() as u64) > rt.file_size
    {
        rt.connection.mode = Mode::Kill;
        return;
    }
    let write_ok = rt
        .meta_file
        .as_ref()
        .is_some_and(|file| file.write_at(block.data, block.offset).ok() == Some(block.data.len()));
    let crc_matches = write_ok && {
        // SAFETY: Category 8 (FFI). Slice pointer covers exactly length bytes for duration of call.
        (unsafe { mycrc32(0, block.data.as_ptr().cast(), block.data.len() as u32) }) == block.crc
    };
    if !crc_matches
        || rt
            .meta_file
            .as_ref()
            .is_none_or(|file| file.sync_all().is_err())
    {
        if rt.download_retries >= 5 {
            download_end(rt);
        } else {
            rt.download_retries += 1;
            download_next(rt);
        }
        return;
    }
    rt.download_offset = rt.download_offset.wrapping_add(block.data.len() as u64);
    rt.download_retries = 0;
    download_next(rt);
}

fn connected(rt: &mut Runtime) {
    // SAFETY: Category 8 (FFI). Runtime owns live socket descriptor.
    unsafe { tcpnodelay(rt.socket) };
    // SAFETY: Category 8 (FFI). Monotonic clock has no preconditions.
    let now = unsafe { monotonic_seconds() };
    rt.connection = Connection::new(rt.config.timeout);
    rt.connection.connected(now, rt.last_log_version);
    rt.downloading = 0;
    rt.meta_file = None;
    rt.log_file = None;
    if rt.last_log_version == 0 {
        queue_download_start(rt, 1);
    }
}

fn init_connect(rt: &mut Runtime) -> c_int {
    // SAFETY: Category 8 (FFI). CString pointers and output references remain valid for calls.
    unsafe {
        if !rt.address_valid {
            if tcpresolve(
                rt.config.bind_host.as_ptr(),
                std::ptr::null(),
                &mut rt.bind_ip,
                std::ptr::null_mut(),
                1,
            ) < 0
            {
                rt.bind_ip = 0;
            }
            if tcpresolve(
                rt.config.host.as_ptr(),
                rt.config.port.as_ptr(),
                &mut rt.master_ip,
                &mut rt.master_port,
                0,
            ) < 0
            {
                return -1;
            }
            rt.address_valid = true;
        }
        rt.socket = tcpsocket();
        if rt.socket < 0 || tcpnonblock(rt.socket) < 0 {
            if rt.socket >= 0 {
                tcpclose(rt.socket);
            }
            rt.socket = -1;
            return -1;
        }
        if rt.bind_ip > 0 && tcpnumbind(rt.socket, rt.bind_ip, 0) < 0 {
            tcpclose(rt.socket);
            rt.socket = -1;
            return -1;
        }
        let status = tcpnumconnect(rt.socket, rt.master_ip, rt.master_port);
        if status < 0 {
            tcpclose(rt.socket);
            rt.socket = -1;
            rt.address_valid = false;
            return -1;
        }
        if status == 0 {
            connected(rt);
        } else {
            rt.connection.mode = Mode::Connecting;
            rt.connection.connect_time = monotonic_seconds();
        }
    }
    0
}

fn disconnect_if_killed(rt: &mut Runtime) {
    if rt.connection.mode != Mode::Kill {
        return;
    }
    before_close(rt);
    if rt.socket >= 0 {
        // SAFETY: Category 8 (FFI). Runtime exclusively owns descriptor.
        unsafe { tcpclose(rt.socket) };
    }
    rt.socket = -1;
    rt.connection.disconnect();
}

fn reset_connection_attempt(rt: &mut Runtime) {
    if rt.socket >= 0 {
        // SAFETY: Category 8 (FFI). Runtime exclusively owns descriptor.
        unsafe { tcpclose(rt.socket) };
    }
    rt.socket = -1;
    rt.address_valid = false;
    rt.connection.mode = Mode::Free;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_stats(input: *mut u32, output: *mut u32) {
    let mut guard = runtime();
    let Some(rt) = guard.as_mut() else { return };
    // SAFETY: Category 8 (FFI). C contract requires both output pointers writable.
    unsafe {
        if !input.is_null() {
            *input = rt.bytes_in;
        }
        if !output.is_null() {
            *output = rt.bytes_out;
        }
    }
    rt.bytes_in = 0;
    rt.bytes_out = 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_findlastlogversion() {
    if let Some(rt) = runtime().as_mut() {
        rt.last_log_version = find_last_log_version();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_createpacket(
    _connection: *mut masterconn,
    packet_type: u32,
    size: u32,
) -> *mut u8 {
    let mut guard = runtime();
    let Some(rt) = guard.as_mut() else {
        return std::ptr::null_mut();
    };
    let mut bytes = Packet::framed(packet_type, &vec![0; size as usize]);
    let payload = bytes.as_mut_ptr().wrapping_add(8);
    rt.connection.queue(bytes);
    payload
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_sendregister(_connection: *mut masterconn) {
    if let Some(rt) = runtime().as_mut() {
        rt.downloading = 0;
        rt.meta_file = None;
        rt.log_file = None;
        rt.connection.queue(core::register_packet(
            rt.config.timeout,
            rt.last_log_version,
        ));
    }
}

unsafe fn packet_slice<'a>(data: *const u8, length: u32) -> Option<&'a [u8]> {
    if length == 0 {
        return Some(&[]);
    }
    if data.is_null() && length != 0 {
        return None;
    }
    // SAFETY: Category 8 (FFI). Caller contract provides readable packet buffer of length bytes.
    Some(unsafe { std::slice::from_raw_parts(data, length as usize) })
}

macro_rules! packet_export {
    ($name:ident, $kind:expr) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(_connection: *mut masterconn, data: *const u8, length: u32) {
            // SAFETY: Forwarding same packet-buffer contract from exported callback.
            let Some(payload) = (unsafe { packet_slice(data, length) }) else {
                return;
            };
            if let Some(rt) = runtime().as_mut() {
                dispatch_packet(rt, Packet::new($kind, payload.to_vec()));
            }
        }
    };
}
packet_export!(masterconn_master_ack, MATOAN_MASTER_ACK);
packet_export!(masterconn_force_timeout, ANTOAN_FORCE_TIMEOUT);
packet_export!(masterconn_metachanges_log, MATOAN_METACHANGES_LOG);
packet_export!(masterconn_download_info, MATOAN_DOWNLOAD_INFO);
packet_export!(masterconn_download_data, MATOAN_DOWNLOAD_DATA);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_metachanges_flush() {
    if let Some(file) = runtime().as_mut().and_then(|rt| rt.log_file.as_mut()) {
        let _ = file.flush();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_download_end(_connection: *mut masterconn) -> c_int {
    runtime().as_mut().map_or(-1, download_end)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_download_init(_connection: *mut masterconn, number: u8) {
    if let Some(rt) = runtime().as_mut() {
        queue_download_start(rt, number);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_metadownloadinit() {
    if let Some(rt) = runtime().as_mut() {
        queue_download_start(rt, 1);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_metadata_check(name: *mut c_char) -> c_int {
    if name.is_null() {
        return -1;
    }
    // SAFETY: Category 8 (FFI). C contract supplies NUL-terminated path.
    let bytes = unsafe { CStr::from_ptr(name) }.to_bytes();
    i32::from(!metadata_valid(Path::new(OsStr::from_bytes(bytes)))) * -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_download_next(_connection: *mut masterconn) {
    if let Some(rt) = runtime().as_mut() {
        download_next(rt);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_beforeclose(_connection: *mut masterconn) {
    if let Some(rt) = runtime().as_mut() {
        before_close(rt);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_gotpacket(
    _connection: *mut masterconn,
    kind: u32,
    data: *const u8,
    length: u32,
) {
    let Some(payload) = (unsafe { packet_slice(data, length) }) else {
        return;
    };
    if let Some(rt) = runtime().as_mut() {
        dispatch_packet(rt, Packet::new(kind, payload.to_vec()));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_connected(_connection: *mut masterconn) {
    if let Some(rt) = runtime().as_mut() {
        connected(rt);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_initconnect(_connection: *mut masterconn) -> c_int {
    runtime().as_mut().map_or(-1, init_connect)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_connecttimeout(_connection: *mut masterconn) {
    if let Some(rt) = runtime().as_mut() {
        reset_connection_attempt(rt);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_connecttest(_connection: *mut masterconn) {
    if let Some(rt) = runtime().as_mut() {
        let failed = unsafe { tcpgetstatus(rt.socket) } != 0;
        if failed {
            reset_connection_attempt(rt);
        } else {
            connected(rt);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_read(connection: *mut masterconn, now: c_double) {
    if connection.is_null() {
        return;
    }
    if let Some(rt) = runtime().as_mut() {
        read_runtime(rt, now);
    }
}

fn read_runtime(rt: &mut Runtime, now: c_double) {
    let mut buffer = vec![0u8; 65_536];
    let mut used = 0usize;
    loop {
        // SAFETY: Category 8 (FFI). Buffer tail is writable and socket is runtime-owned.
        let count = unsafe {
            libc::read(
                rt.socket,
                buffer[used..].as_mut_ptr().cast(),
                buffer.len() - used,
            )
        };
        if count == 0 {
            log_message(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                c"connection was reset by Master",
            );
            rt.connection.input_end = true;
            break;
        }
        if count < 0 {
            let error = std::io::Error::last_os_error().raw_os_error();
            if !matches!(error, Some(code) if code == libc::EAGAIN || code == libc::EINTR) {
                log_message(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    c"read from Master error",
                );
                rt.connection.input_end = true;
            }
            break;
        }
        used += count as usize;
        rt.bytes_in = rt.bytes_in.wrapping_add(count as u32);
        if used < buffer.len() {
            break;
        }
        buffer.resize(buffer.len().saturating_mul(2), 0);
    }
    if used > 0 {
        rt.connection.last_read = now;
        if rt.connection.decoder.feed(&buffer[..used]).is_err() {
            rt.connection.input_end = true;
        }
    }
}

fn parse_runtime(rt: &mut Runtime) {
    // SAFETY: Category 8 (FFI). Monotonic clock has no preconditions.
    let start = unsafe { monotonic_useconds() };
    while rt.connection.mode == Mode::Data {
        let Some(packet) = rt.connection.decoder.pop() else {
            break;
        };
        dispatch_packet(rt, packet);
        // SAFETY: Same monotonic clock contract.
        if unsafe { monotonic_useconds() }.wrapping_sub(start) >= 10_000 {
            break;
        }
    }
    if rt.connection.mode == Mode::Data
        && rt.connection.decoder.is_empty()
        && rt.connection.input_end
    {
        rt.connection.mode = Mode::Kill;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_parse(_connection: *mut masterconn) {
    if let Some(rt) = runtime().as_mut() {
        parse_runtime(rt);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_write(_connection: *mut masterconn, now: c_double) {
    if let Some(rt) = runtime().as_mut() {
        write_runtime(rt, now);
    }
}

fn write_runtime(rt: &mut Runtime, now: c_double) {
    loop {
        let Some(packet) = rt.connection.output.front_mut() else {
            return;
        };
        let remaining = packet.bytes.len() - packet.sent;
        // SAFETY: Category 8 (FFI). Remaining Vec range is readable; socket is runtime-owned.
        let count = unsafe {
            libc::write(
                rt.socket,
                packet.bytes[packet.sent..].as_ptr().cast(),
                remaining,
            )
        };
        if count < 0 {
            let error = std::io::Error::last_os_error().raw_os_error();
            if !matches!(error, Some(code) if code == libc::EAGAIN || code == libc::EINTR) {
                rt.connection.mode = Mode::Kill;
            }
            return;
        }
        if count > 0 {
            rt.connection.last_write = now;
            rt.bytes_out = rt.bytes_out.wrapping_add(count as u32);
            packet.sent += count as usize;
        }
        if count as usize != remaining {
            return;
        }
        rt.connection.output.pop_front();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_desc(descriptors: *mut pollfd, count: *mut u32) {
    if descriptors.is_null() || count.is_null() {
        return;
    }
    let mut guard = runtime();
    let Some(rt) = guard.as_mut() else { return };
    rt.poll_position = -1;
    if rt.connection.mode == Mode::Free || rt.socket < 0 {
        return;
    }
    // SAFETY: Category 8 (FFI). Daemon supplies writable descriptor at current count and count pointer.
    unsafe {
        let position = *count as isize;
        let descriptor = &mut *descriptors.offset(position);
        descriptor.events = 0;
        if rt.connection.mode == Mode::Data && !rt.connection.input_end {
            descriptor.events |= libc::POLLIN;
        }
        if (rt.connection.mode == Mode::Data && !rt.connection.output.is_empty())
            || rt.connection.mode == Mode::Connecting
        {
            descriptor.events |= libc::POLLOUT;
        }
        if descriptor.events != 0 {
            descriptor.fd = rt.socket;
            rt.poll_position = *count as i32;
            *count = (*count).wrapping_add(1);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_disconnection_check() {
    if let Some(rt) = runtime().as_mut() {
        disconnect_if_killed(rt);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_serve(descriptors: *mut pollfd) {
    let mut guard = runtime();
    let Some(rt) = guard.as_mut() else { return };
    // SAFETY: Category 8 (FFI). Monotonic clock has no preconditions.
    let now = unsafe { monotonic_seconds() };
    let (events, revents) = if rt.poll_position >= 0 && !descriptors.is_null() {
        // SAFETY: Category 8 (FFI). poll_position was assigned by desc for same daemon poll array.
        unsafe {
            (
                (*descriptors.offset(rt.poll_position as isize)).events as c_int,
                (*descriptors.offset(rt.poll_position as isize)).revents as c_int,
            )
        }
    } else {
        (0, 0)
    };
    if rt.connection.mode == Mode::Connecting {
        if rt.socket >= 0
            && rt.poll_position >= 0
            && revents & c_int::from(libc::POLLOUT | libc::POLLHUP | libc::POLLERR) != 0
        {
            if unsafe { tcpgetstatus(rt.socket) } != 0 {
                reset_connection_attempt(rt);
            } else {
                connected(rt);
            }
        } else if rt.connection.connect_time + 1.0 < now {
            reset_connection_attempt(rt);
        }
    } else {
        if rt.poll_position >= 0
            && revents & c_int::from(libc::POLLERR | libc::POLLIN) == c_int::from(libc::POLLIN)
        {
            read_runtime(rt, now);
        }
        if revents & c_int::from(libc::POLLERR | libc::POLLHUP) != 0 {
            rt.connection.input_end = true;
        }
        parse_runtime(rt);
        if rt.connection.mode == Mode::Data
            && rt.connection.last_write + 1.0 < now
            && rt.connection.output.is_empty()
        {
            rt.connection.queue_packet(ANTOAN_NOP as u32, &[]);
        }
        if rt.poll_position >= 0
            && rt.connection.mode == Mode::Data
            && ((events & c_int::from(libc::POLLOUT) == 0 && !rt.connection.output.is_empty())
                || revents & c_int::from(libc::POLLOUT) != 0)
        {
            write_runtime(rt, now);
        }
        if rt.connection.mode == Mode::Data
            && rt.connection.last_read + f64::from(rt.connection.timeout) < now
        {
            rt.connection.mode = Mode::Kill;
        }
    }
    disconnect_if_killed(rt);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_reconnect() {
    if let Some(rt) = runtime().as_mut() {
        if rt.connection.mode == Mode::Free {
            init_connect(rt);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_socketmode(mode: u8) -> *const c_char {
    let label = match mode {
        0 => Mode::Free,
        1 => Mode::Connecting,
        2 => Mode::Data,
        3 => Mode::Kill,
        _ => return c"???".as_ptr(),
    };
    label.label().as_ptr().cast()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_info(file: *mut FILE) {
    if file.is_null() {
        return;
    }
    let guard = runtime();
    let Some(rt) = guard.as_ref() else { return };
    let mut bind = [0 as c_char; 16];
    let mut master = [0 as c_char; 32];
    // SAFETY: Category 8 (FFI). Daemon supplies valid FILE pointer; formats match promoted argument types.
    unsafe {
        univmakestrip(bind.as_mut_ptr(), rt.bind_ip);
        univmakestripport(master.as_mut_ptr(), rt.master_ip, rt.master_port);
        fprintf(file, c"[master connection]\nmaster address is valid: %u\nworking timeout: %u\nsocket bind ip: %s\nresolved ip:port number: %s\nsocket mode: %s\n".as_ptr(), rt.address_valid as c_int, rt.connection.timeout as c_int, bind.as_ptr(), master.as_ptr(), masterconn_socketmode(rt.connection.mode as u8));
        if rt.downloading == 1 {
            fprintf(file, c"downloading metadata in progress\n".as_ptr());
        } else if matches!(rt.downloading, 11 | 12) {
            fprintf(file, c"downloading last changelogs in progress\n".as_ptr());
        }
        if rt.downloading > 0 {
            let elapsed = monotonic_useconds().wrapping_sub(rt.download_started);
            fprintf(file, c"downloading progress: %lu/%lu\ndownloading try counter: %u\ndownloading time: %lu.%06us".as_ptr(), rt.download_offset, rt.file_size, rt.download_retries as c_int, elapsed / 1_000_000, (elapsed % 1_000_000) as u32);
        }
        fprintf(file, c"\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_term() {
    let mut guard = runtime();
    if let Some(mut rt) = guard.take() {
        before_close(&mut rt);
        if rt.socket >= 0 {
            unsafe { tcpclose(rt.socket) };
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_reload() {
    // SAFETY: Category 8 (FFI). Config keys/defaults are static valid C strings.
    let config = unsafe { load_config() };
    let mut guard = runtime();
    let Some(rt) = guard.as_mut() else { return };
    rt.config = config;
    rt.config.back_meta_copies = rt.config.back_meta_copies.min(99);
    rt.address_valid = false;
    if rt.connection.mode != Mode::Free {
        rt.connection.mode = Mode::Kill;
    }
    // SAFETY: Category 8 (FFI). Handles originate from timer registration and remain registry-owned.
    unsafe {
        main_time_change(
            rt.reconnect_hook as *mut c_void,
            rt.config.reconnect_delay,
            0,
        );
        main_time_change(
            rt.download_hook as *mut c_void,
            rt.config.meta_frequency.wrapping_mul(3600),
            630,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_init() -> c_int {
    // SAFETY: Category 8 (FFI). Config keys/defaults are static valid C strings.
    let config = unsafe { load_config() };
    let timeout = config.timeout;
    let mut rt = Runtime {
        connection: Connection::new(timeout),
        config,
        socket: -1,
        poll_position: -1,
        bind_ip: 0,
        master_ip: 0,
        master_port: 0,
        address_valid: false,
        downloading: 0,
        old_mode: false,
        meta_file: None,
        log_file: None,
        file_size: 0,
        download_offset: 0,
        download_started: 0,
        download_retries: 0,
        last_log_version: find_last_log_version(),
        bytes_in: 0,
        bytes_out: 0,
        reconnect_hook: 0,
        download_hook: 0,
    };
    if init_connect(&mut rt) < 0 {
        *runtime() = Some(rt);
        return -1;
    }
    // SAFETY: Category 8 (FFI). Registered callbacks and names have static process lifetime.
    unsafe {
        rt.reconnect_hook = main_time_register_fname(
            rt.config.reconnect_delay,
            0,
            Some(masterconn_reconnect),
            c"masterconn_reconnect".as_ptr(),
        ) as usize;
        rt.download_hook = main_time_register_fname(
            rt.config.meta_frequency.wrapping_mul(3600),
            630,
            Some(masterconn_metadownloadinit),
            c"masterconn_metadownloadinit".as_ptr(),
        ) as usize;
        main_destruct_register_fname(Some(masterconn_term), c"masterconn_term".as_ptr());
        main_poll_register_fname(
            Some(masterconn_desc),
            Some(masterconn_serve),
            c"masterconn_desc".as_ptr(),
            c"masterconn_serve".as_ptr(),
        );
        main_reload_register_fname(Some(masterconn_reload), c"masterconn_reload".as_ptr());
        main_time_register_fname(
            1,
            0,
            Some(masterconn_metachanges_flush),
            c"masterconn_metachanges_flush".as_ptr(),
        );
        main_info_register_fname(Some(masterconn_info), c"masterconn_info".as_ptr());
    }
    *runtime() = Some(rt);
    0
}
