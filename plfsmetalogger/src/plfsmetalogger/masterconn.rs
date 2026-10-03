//! Metalogger connection with the master — port of mfsmetalogger/masterconn.c.
//!
//! One connection (C `masterconnsingleton`) to the master's metalogger port:
//! register, receive the changelog stream into `changelog_ml.0.mfs`
//! (rotated on the master's rotate marker), and periodically download the
//! metadata file plus the two last changelogs (`metadata_ml.mfs.back`,
//! `changelog_ml_back.{0,1}.mfs`).
//!
//! Layout: all logic is safe Rust on top of `plfscommon` (daemon registry,
//! `sockets::tcp`, cfg getters, clocks, crc32, mfslog). The only unsafe
//! site is `sys::close`, kept so a failing `close(2)` of the downloaded
//! file is still reported ("error closing metafile") as in C.
//!
//! Kept from C, step for step: packet framing and the 1.5 MB limit, the
//! 10 ms parse budget, writev batching (100 packets), NOP keepalive after
//! 1 s idle, timeouts, the download state machine (retry counter, block
//! size, renames, `oldmode`), the backward scan of `changelog_ml.0.back` in
//! `find_last_log_version` (including its quirks: a one-line file yields 0,
//! digits are read at most 32 bytes past a block end, the 200000-byte
//! window), cfg read order and clamps (BACK_META_KEEP_PREVIOUS is clamped
//! only on reload), log texts, modes and the errno channel.
//!
//! Deviations, none observable through files, protocol or logs:
//! - the changelog file is buffered by `BufWriter` (capacity = st_blksize,
//!   like glibc stdio) instead of a `FILE*`: content is identical at every
//!   flush point (1 s timer, rotation, close, exit), only the write(2)
//!   chunking between flushes may differ;
//! - files are opened with O_CLOEXEC (std default);
//! - C leaks the previous download fd when MATOAN_DOWNLOAD_INFO repeats
//!   during a download; here it is closed;
//! - `masterconn_stats` (unreferenced in C: commented out in
//!   masterconn.h) and its byte counters are not ported;
//! - callbacks after `masterconn_term` (none happen) are no-ops instead of
//!   NULL dereferences.
#![deny(unsafe_code)]

use core::ffi::c_int;
use std::collections::VecDeque;
use std::ffi::CString;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::{FileExt, MetadataExt};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};

use plfscommon::clocks::{monotonic_seconds, monotonic_useconds};
use plfscommon::cnum::fmt_fixed;
use plfscommon::crc::crc32;
use plfscommon::daemon::{self, TimerHandle};
use plfscommon::mfslog::{
    MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_INFO, MFSLOG_NOTICE, MFSLOG_SYSLOG, MFSLOG_SYSLOG_STDERR,
    MFSLOG_WARNING, log_bytes,
};
use plfscommon::sockets::{fmt_ip, fmt_ip_port, tcp};
use plfscommon::cfg;

// MFSCommunication.h (PROTO_BASE = 0)
const ANTOAN_NOP: u32 = 0;
const ANTOAN_UNKNOWN_COMMAND: u32 = 1;
const ANTOAN_BAD_COMMAND_SIZE: u32 = 2;
const ANTOAN_FORCE_TIMEOUT: u32 = 5;
const ANTOMA_REGISTER: u32 = 50;
const MATOAN_METACHANGES_LOG: u32 = 51;
const MATOAN_MASTER_ACK: u32 = 52;
const ANTOMA_DOWNLOAD_START: u32 = 60;
const MATOAN_DOWNLOAD_INFO: u32 = 61;
const ANTOMA_DOWNLOAD_REQUEST: u32 = 62;
const MATOAN_DOWNLOAD_DATA: u32 = 63;
const ANTOMA_DOWNLOAD_END: u32 = 64;
/// C `MaxPacketSize` (ANTOMA_MAXPACKETSIZE).
const MAX_PACKET_SIZE: u32 = 1_500_000;
/// C `META_DL_BLOCK`: min(MATOAN_MAXPACKETSIZE - 1000, 1000000).
const META_DL_BLOCK: u64 = 1_000_000;

const VERSMAJ: u16 = 4;
const VERSMID: u8 = 59;
const VERSMIN: u8 = 2 * 2;
const DEFAULT_MASTERNAME: &[u8] = b"mfsmaster";
const DEFAULT_MASTER_CONTROL_PORT: &[u8] = b"9419";
const MFSSIGNATURE: &[u8] = b"MFS";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    Free,
    Connecting,
    Data,
    Kill,
}

struct InPacket {
    ptype: u32,
    data: Vec<u8>,
}

/// Output packet: header + payload, `start` = bytes already written.
struct OutPacket {
    data: Vec<u8>,
    start: usize,
}

/// C `masterconn`.
struct Conn {
    mode: Mode,
    sock: c_int,
    pdescpos: i32,
    lastread: f64,
    lastwrite: f64,
    conntime: f64,
    input_hdr: [u8; 8],
    /// Write offset into the current input target (header or packet data).
    input_off: usize,
    input_bytesleft: u32,
    input_end: bool,
    input_packet: Option<InPacket>,
    inputq: VecDeque<InPacket>,
    outputq: VecDeque<OutPacket>,
    bindip: u32,
    masterip: u32,
    masterport: u16,
    timeout: u16,
    masteraddrvalid: u8,
    downloadretrycnt: u8,
    downloading: u8,
    oldmode: u8,
    logfd: Option<BufWriter<File>>,
    metafd: Option<File>,
    filesize: u64,
    dloffset: u64,
    dlstartuts: u64,
}

impl Conn {
    fn new(timeout: u16) -> Self {
        Conn {
            mode: Mode::Free,
            sock: -1,
            pdescpos: -1,
            lastread: 0.0,
            lastwrite: 0.0,
            conntime: 0.0,
            input_hdr: [0; 8],
            input_off: 0,
            input_bytesleft: 8,
            input_end: false,
            input_packet: None,
            inputq: VecDeque::new(),
            outputq: VecDeque::new(),
            bindip: 0,
            masterip: 0,
            masterport: 0,
            timeout,
            masteraddrvalid: 0,
            downloadretrycnt: 0,
            downloading: 0,
            oldmode: 0,
            logfd: None,
            metafd: None,
            filesize: 0,
            dloffset: 0,
            dlstartuts: 0,
        }
    }
}

/// C file statics + the singleton.
struct State {
    c: Conn,
    /// Directory every file name is relative to: "" (the daemon's working
    /// directory, as in C) in production, a scratch dir in tests.
    base: PathBuf,
    back_logs: u32,
    back_meta_copies: u32,
    master_host: CString,
    master_port: CString,
    bind_host: CString,
    timeout: u32,
    lastlogversion: u64,
    reconnect_hook: Option<TimerHandle>,
    download_hook: Option<TimerHandle>,
    readbuff: Vec<u8>,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn state() -> MutexGuard<'static, Option<State>> {
    STATE.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Syscall boundary.
mod sys {
    #![allow(unsafe_code)]
    use std::fs::File;
    use std::os::fd::IntoRawFd;

    /// `close(2)` with its result (std's `drop` discards close errors).
    pub fn close(f: File) -> libc::c_int {
        let fd = f.into_raw_fd();
        // `fd` was just released by `into_raw_fd`: owned here, closed once.
        // SAFETY: valid, owned descriptor (see above).
        unsafe { libc::close(fd) }
    }
}

fn log(mode: c_int, pri: c_int, text: &str) {
    log_bytes(mode, pri, text.as_bytes());
}

fn be32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

fn be64(b: &[u8]) -> u64 {
    u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

/// C `%s` of a cfg value: text up to the first NUL.
fn cstring(v: Vec<u8>) -> CString {
    let end = v.iter().position(|&b| b == 0).unwrap_or(v.len());
    CString::new(&v[..end]).expect("NUL removed")
}

fn cfg_str(name: &[u8], def: &[u8]) -> CString {
    cstring(cfg::get_str(name, def))
}

fn cfg_u32(name: &[u8], def: u32) -> u32 {
    let (v, e) = cfg::get_uint32(name, def);
    cfg::set_erange(e);
    v
}

/// `%s` of a `char*` that may stop early at an embedded NUL.
fn until_nul(b: &[u8]) -> &[u8] {
    &b[..b.iter().position(|&c| c == 0).unwrap_or(b.len())]
}

/// C `masterconn_findlastlogversion`: version of the last complete line of
/// `changelog_ml.0.back` (trailing garbage after the last newline is
/// truncated), searched backwards in 32 KiB blocks within the last
/// 200000 bytes; 0 if there is no usable metadata backup or line.
fn find_last_log_version(base: &Path) -> u64 {
    let Ok(meta) = fs::metadata(base.join("metadata_ml.mfs.back")) else { return 0 };
    if meta.len() == 0 || !meta.file_type().is_file() {
        return 0;
    }
    let Ok(mut f) = OpenOptions::new().read(true).write(true).open(base.join("changelog_ml.0.back")) else {
        return 0;
    };
    // C ignores an fstat failure (st keeps the metadata file's values)
    let st_size = f.metadata().map_or(meta.len(), |m| m.len());
    let mut buff = [0u8; 32800];
    let mut size = st_size;
    let mut lastnewline: u64 = 0;
    let mut version: u64 = 0;
    while size > 0 && size + 200_000 > st_size {
        let mut buffpos: usize;
        if size > 32768 {
            buff.copy_within(0..32, 32768);
            size -= 32768;
            let _ = f.seek(SeekFrom::Start(size));
            if !matches!(f.read(&mut buff[..32768]), Ok(32768)) {
                return 0;
            }
            buffpos = 32768;
        } else {
            let s = size as usize;
            buff.copy_within(0..32, s);
            let _ = f.seek(SeekFrom::Start(0));
            if !matches!(f.read(&mut buff[..s]), Ok(n) if n == s) {
                return 0;
            }
            buffpos = s;
            size = 0;
        }
        while buffpos > 0 {
            buffpos -= 1;
            if buff[buffpos] != b'\n' {
                continue;
            }
            if lastnewline == 0 {
                lastnewline = size + buffpos as u64;
                continue;
            }
            if lastnewline + 1 != st_size && f.set_len(lastnewline + 1).is_err() {
                return 0;
            }
            buffpos += 1;
            while buffpos < 32800 && buff[buffpos].is_ascii_digit() {
                version = version.wrapping_mul(10).wrapping_add((buff[buffpos] - b'0') as u64);
                buffpos += 1;
            }
            if buffpos == 32800 || buff[buffpos] != b':' {
                version = 0;
            }
            return version;
        }
    }
    0
}

/// C `masterconn_metadata_check`: 0 if the downloaded file has a known
/// signature and its EOF marker, -1 otherwise.
fn metadata_check(name: &Path) -> c_int {
    let warn = |m: &str| log(MFSLOG_SYSLOG, MFSLOG_WARNING, m);
    let Ok(mut f) = File::open(name) else {
        warn("can't open downloaded metadata");
        return -1;
    };
    let mut chk = [0u8; 16];
    if !matches!(f.read(&mut chk[..8]), Ok(8)) {
        warn("can't read downloaded metadata");
        return -1;
    }
    if &chk[..8] == b"MFSM NEW" {
        return -1;
    }
    let eofmark: [u8; 16];
    if &chk[..3] == MFSSIGNATURE
        && &chk[3..5] == b"M "
        && (b'1'..=b'9').contains(&chk[5])
        && chk[6] == b'.'
        && chk[7].is_ascii_digit()
    {
        let fver = ((chk[5] - b'0') << 4).wrapping_add(chk[7] - b'0');
        if fver < 0x17 {
            eofmark = [0; 16];
        } else {
            eofmark = *b"[MFS EOF MARKER]";
            if fver >= 0x20 {
                if !matches!(f.read(&mut chk), Ok(16)) {
                    warn("can't read downloaded metadata");
                    return -1;
                }
                let (metaversion, metaid) = (be64(&chk[..8]), be64(&chk[8..]));
                log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    &format!("meta data version: {metaversion}, meta data id: 0x{metaid:016X}"),
                );
            }
        }
    } else {
        warn("bad metadata file format");
        return -1;
    }
    // lseek(fd,-16,SEEK_END): on a short file the position stays put
    let _ = f.seek(SeekFrom::End(-16));
    if !matches!(f.read(&mut chk), Ok(16)) {
        warn("can't read downloaded metadata");
        return -1;
    }
    drop(f);
    if chk != eofmark {
        warn("truncated metadata file !!!");
        return -1;
    }
    0
}

impl State {
    fn path(&self, name: &str) -> PathBuf {
        self.base.join(name)
    }

    fn rename(&self, from: &str, to: &str) -> bool {
        fs::rename(self.path(from), self.path(to)).is_ok()
    }

    fn unlink(&self, name: &str) {
        let _ = fs::remove_file(self.path(name));
    }

    /// C `masterconn_createpacket` + the payload writes.
    fn createpacket(&mut self, ptype: u32, payload: &[u8]) {
        let mut data = Vec::with_capacity(8 + payload.len());
        data.extend_from_slice(&ptype.to_be_bytes());
        data.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        data.extend_from_slice(payload);
        self.c.outputq.push_back(OutPacket { data, start: 0 });
    }

    fn sendregister(&mut self) {
        let c = &mut self.c;
        c.downloading = 0;
        c.metafd = None;
        c.logfd = None;
        let mut p = Vec::with_capacity(15);
        p.push(if self.lastlogversion > 0 { 2 } else { 1 });
        p.extend_from_slice(&VERSMAJ.to_be_bytes());
        p.push(VERSMID);
        p.push(VERSMIN);
        p.extend_from_slice(&c.timeout.to_be_bytes());
        if self.lastlogversion > 0 {
            p.extend_from_slice(&self.lastlogversion.wrapping_add(1).to_be_bytes());
        }
        self.createpacket(ANTOMA_REGISTER, &p);
    }

    fn master_ack(&mut self, data: &[u8]) {
        if data.len() != 5 {
            log(MFSLOG_SYSLOG, MFSLOG_WARNING, &format!("MATOAN_MASTER_ACK - wrong size ({}/5)", data.len()));
            self.c.mode = Mode::Kill;
            return;
        }
        if data[0] >= 2 {
            self.c.mode = Mode::Kill;
        }
    }

    fn force_timeout(&mut self, data: &[u8]) {
        if data.len() != 2 {
            log(MFSLOG_SYSLOG, MFSLOG_WARNING, &format!("ANTOAN_FORCE_TIMEOUT - wrong size ({}/2)", data.len()));
            self.c.mode = Mode::Kill;
            return;
        }
        self.c.timeout = u16::from_be_bytes([data[0], data[1]]).max(10);
    }

    fn close_logfd(&mut self) {
        if let Some(mut l) = self.c.logfd.take() {
            let _ = l.flush();
        }
    }

    fn metachanges_log(&mut self, data: &[u8]) {
        let length = data.len() as u32;
        if length == 1 && data[0] == 0x55 {
            self.close_logfd();
            if self.back_logs > 0 {
                for i in (1..=self.back_logs).rev() {
                    self.rename(&format!("changelog_ml.{}.mfs", i - 1), &format!("changelog_ml.{i}.mfs"));
                }
            } else {
                self.unlink("changelog_ml.0.mfs");
            }
            return;
        }
        let kill = |s: &mut Self, m: &str| {
            log(MFSLOG_SYSLOG, MFSLOG_WARNING, m);
            s.c.mode = Mode::Kill;
        };
        if length < 10 {
            return kill(self, &format!("MATOAN_METACHANGES_LOG - wrong size ({length}/9+data)"));
        }
        if data[0] != 0xFF {
            return kill(self, "MATOAN_METACHANGES_LOG - wrong packet");
        }
        if data[data.len() - 1] != 0 {
            return kill(self, "MATOAN_METACHANGES_LOG - invalid string");
        }
        let version = be64(&data[1..9]);
        let text = until_nul(&data[9..]);
        if self.lastlogversion > 0 && version != self.lastlogversion.wrapping_add(1) {
            log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                &format!(
                    "some changes lost: [{}-{}], download metadata again",
                    self.lastlogversion,
                    version.wrapping_sub(1)
                ),
            );
            self.close_logfd();
            for i in 0..=self.back_logs {
                self.unlink(&format!("changelog_ml.{i}.mfs"));
            }
            self.lastlogversion = 0;
            self.c.mode = Mode::Kill;
            return;
        }
        if self.c.logfd.is_none() {
            let path = self.path("changelog_ml.0.mfs");
            self.c.logfd = OpenOptions::new().append(true).create(true).open(path).ok().map(|f| {
                // glibc stdio buffers with st_blksize (BUFSIZ if unknown)
                let cap = f.metadata().map_or(0, |m| m.blksize() as usize);
                BufWriter::with_capacity(if cap > 0 { cap } else { 8192 }, f)
            });
        }
        if let Some(l) = self.c.logfd.as_mut() {
            let mut line = format!("{version}: ").into_bytes();
            line.extend_from_slice(text);
            line.push(b'\n');
            let _ = l.write_all(&line);
            self.lastlogversion = version;
        } else {
            let mut m = format!("lost MFS change {version}: ").into_bytes();
            m.extend_from_slice(text);
            log_bytes(MFSLOG_SYSLOG, MFSLOG_WARNING, &m);
        }
    }

    fn download_end(&mut self) -> c_int {
        self.c.downloading = 0;
        self.createpacket(ANTOMA_DOWNLOAD_END, &[]);
        if let Some(f) = self.c.metafd.take()
            && sys::close(f) < 0
        {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, "error closing metafile");
            return -1;
        }
        0
    }

    fn download_init(&mut self, filenum: u8) {
        if self.c.mode == Mode::Data && self.c.downloading == 0 {
            self.createpacket(ANTOMA_DOWNLOAD_START, &[filenum]);
            self.c.downloading = filenum;
        }
    }

    fn download_next(&mut self) {
        if self.c.dloffset < self.c.filesize {
            let left = self.c.filesize - self.c.dloffset;
            let mut p = Vec::with_capacity(12);
            p.extend_from_slice(&self.c.dloffset.to_be_bytes());
            p.extend_from_slice(&(left.min(META_DL_BLOCK) as u32).to_be_bytes());
            self.createpacket(ANTOMA_DOWNLOAD_REQUEST, &p);
            return;
        }
        let filenum = self.c.downloading;
        if self.download_end() < 0 {
            return;
        }
        let mut dltime = monotonic_useconds().wrapping_sub(self.c.dlstartuts) as i64;
        if dltime <= 0 {
            dltime = 1;
        }
        let what = match filenum {
            1 => "metadata",
            11 => "changelog_0",
            12 => "changelog_1",
            _ => "???",
        };
        let fs = self.c.filesize;
        log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            &format!(
                "{what} downloaded {fs}B/{}.{:06}s ({} MB/s)",
                dltime / 1_000_000,
                (dltime % 1_000_000) as u32,
                fmt_fixed(fs as f64 / dltime as f64, 3)
            ),
        );
        match filenum {
            1 => {
                if metadata_check(&self.path("metadata_ml.tmp")) == 0 {
                    if self.back_meta_copies > 0 {
                        // C: `int i = BackMetaCopies-1` (wraps for huge values)
                        for i in (1..=self.back_meta_copies.wrapping_sub(1) as i32).rev() {
                            self.rename(&format!("metadata_ml.mfs.back.{i}"), &format!("metadata_ml.mfs.back.{}", i + 1));
                        }
                        self.rename("metadata_ml.mfs.back", "metadata_ml.mfs.back.1");
                    }
                    if !self.rename("metadata_ml.tmp", "metadata_ml.mfs.back") {
                        log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            "can't rename downloaded metadata - do it manually before next download",
                        );
                    }
                }
                if self.c.oldmode == 0 {
                    self.download_init(11);
                }
            }
            11 | 12 => {
                let to = if filenum == 11 { "changelog_ml_back.0.mfs" } else { "changelog_ml_back.1.mfs" };
                if !self.rename("changelog_ml.tmp", to) {
                    log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        "can't rename downloaded changelog - do it manually before next download",
                    );
                }
                if filenum == 11 {
                    self.download_init(12);
                }
            }
            _ => {}
        }
    }

    fn download_info(&mut self, data: &[u8]) {
        let length = data.len();
        if length != 1 && length != 8 {
            log(MFSLOG_SYSLOG, MFSLOG_WARNING, &format!("MATOAN_DOWNLOAD_INFO - wrong size ({length}/1|8)"));
            self.c.mode = Mode::Kill;
            return;
        }
        if length == 1 {
            self.c.downloading = 0;
            log(MFSLOG_SYSLOG, MFSLOG_WARNING, "download start error");
            return;
        }
        self.c.filesize = be64(data);
        self.c.dloffset = 0;
        self.c.downloadretrycnt = 0;
        self.c.dlstartuts = monotonic_useconds();
        let name = match self.c.downloading {
            1 => "metadata_ml.tmp",
            11 | 12 => "changelog_ml.tmp",
            _ => {
                log(MFSLOG_SYSLOG, MFSLOG_WARNING, "unexpected MATOAN_DOWNLOAD_INFO packet");
                self.c.mode = Mode::Kill;
                return;
            }
        };
        let path = self.path(name);
        self.c.metafd = OpenOptions::new().write(true).truncate(true).create(true).open(path).ok();
        if self.c.metafd.is_none() {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, "error opening metafile");
            self.download_end();
            return;
        }
        self.download_next();
    }

    /// Shared failure path of a download block: retry up to 5 times.
    fn download_retry(&mut self) {
        if self.c.downloadretrycnt >= 5 {
            self.download_end();
        } else {
            self.c.downloadretrycnt += 1;
            self.download_next();
        }
    }

    fn download_data(&mut self, data: &[u8]) {
        let length = data.len() as u32;
        let kill = |s: &mut Self, m: &str| {
            log(MFSLOG_SYSLOG, MFSLOG_WARNING, m);
            s.c.mode = Mode::Kill;
        };
        if self.c.metafd.is_none() {
            return kill(self, "MATOAN_DOWNLOAD_DATA - file not opened");
        }
        if length < 16 {
            return kill(self, &format!("MATOAN_DOWNLOAD_DATA - wrong size ({length}/16+data)"));
        }
        let offset = be64(&data[..8]);
        let leng = be32(&data[8..12]);
        let crc = be32(&data[12..16]);
        if leng.wrapping_add(16) != length {
            return kill(self, &format!("MATOAN_DOWNLOAD_DATA - wrong size ({length}/16+{leng})"));
        }
        if offset != self.c.dloffset {
            return kill(
                self,
                &format!("MATOAN_DOWNLOAD_DATA - unexpected file offset ({offset}/{})", self.c.dloffset),
            );
        }
        let end = offset.wrapping_add(leng as u64);
        if end > self.c.filesize {
            return kill(self, &format!("MATOAN_DOWNLOAD_DATA - unexpected file size ({end}/{})", self.c.filesize));
        }
        let block = &data[16..];
        let f = self.c.metafd.as_ref().expect("checked above");
        if !matches!(f.write_at(block, offset), Ok(n) if n == leng as usize) {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, "error writing metafile");
            return self.download_retry();
        }
        if crc != crc32(0, block) {
            log(MFSLOG_SYSLOG, MFSLOG_WARNING, "metafile data crc error");
            return self.download_retry();
        }
        if f.sync_all().is_err() {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, "error syncing metafile");
            return self.download_retry();
        }
        self.c.dloffset += leng as u64;
        self.c.downloadretrycnt = 0;
        self.download_next();
    }

    fn beforeclose(&mut self) {
        if self.c.downloading == 11 || self.c.downloading == 12 {
            log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                "old master detected - please upgrade your master server and then restart metalogger",
            );
            self.c.oldmode = 1;
        }
        if let Some(f) = self.c.metafd.take() {
            sys::close(f);
            self.unlink("metadata_ml.tmp");
            self.unlink("changelog_ml.tmp");
        }
        self.close_logfd();
    }

    fn gotpacket(&mut self, ptype: u32, data: &[u8]) {
        match ptype {
            ANTOAN_NOP | ANTOAN_UNKNOWN_COMMAND | ANTOAN_BAD_COMMAND_SIZE => {}
            MATOAN_METACHANGES_LOG => self.metachanges_log(data),
            ANTOAN_FORCE_TIMEOUT => self.force_timeout(data),
            MATOAN_MASTER_ACK => self.master_ack(data),
            MATOAN_DOWNLOAD_INFO => self.download_info(data),
            MATOAN_DOWNLOAD_DATA => self.download_data(data),
            _ => {
                log(MFSLOG_SYSLOG, MFSLOG_WARNING, &format!("got unknown message (type:{ptype})"));
                self.c.mode = Mode::Kill;
            }
        }
    }

    fn connected(&mut self) {
        let now = monotonic_seconds();
        tcp::nodelay(self.c.sock);
        let c = &mut self.c;
        c.mode = Mode::Data;
        c.lastread = now;
        c.lastwrite = now;
        c.input_bytesleft = 8;
        c.input_off = 0;
        c.input_end = false;
        c.input_packet = None;
        c.inputq.clear();
        c.outputq.clear();
        c.timeout = self.timeout as u16;
        self.sendregister();
        if self.lastlogversion == 0 {
            self.download_init(1);
        }
    }

    fn initconnect(&mut self) -> c_int {
        if self.c.masteraddrvalid == 0 {
            self.c.bindip = tcp::resolve(Some(self.bind_host.as_c_str()), None, true).map_or(0, |(ip, _)| ip);
            match tcp::resolve(Some(self.master_host.as_c_str()), Some(self.master_port.as_c_str()), false) {
                Some((mip, mport)) => {
                    self.c.masterip = mip;
                    self.c.masterport = mport;
                    self.c.masteraddrvalid = 1;
                }
                None => {
                    let mut m = b"can't resolve master host/port (".to_vec();
                    m.extend_from_slice(self.master_host.to_bytes());
                    m.push(b':');
                    m.extend_from_slice(self.master_port.to_bytes());
                    m.push(b')');
                    log_bytes(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, &m);
                    return -1;
                }
            }
        }
        let c = &mut self.c;
        c.sock = tcp::socket();
        if c.sock < 0 {
            log(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_WARNING, "create socket, error");
            return -1;
        }
        if tcp::nonblock(c.sock) < 0 {
            log(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_WARNING, "set nonblock, error");
            tcp::close(c.sock);
            c.sock = -1;
            return -1;
        }
        if c.bindip > 0 && tcp::numbind(c.sock, c.bindip, 0) < 0 {
            log(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_WARNING, "can't bind socket to given ip");
            tcp::close(c.sock);
            c.sock = -1;
            return -1;
        }
        let status = tcp::numconnect(c.sock, c.masterip, c.masterport);
        if status < 0 {
            log(MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_WARNING, "connect failed, error");
            tcp::close(c.sock);
            c.sock = -1;
            c.masteraddrvalid = 0;
            return -1;
        }
        if status == 0 {
            log(MFSLOG_SYSLOG, MFSLOG_INFO, "connected to Master immediately");
            self.connected();
        } else {
            c.mode = Mode::Connecting;
            c.conntime = monotonic_seconds();
            log(MFSLOG_SYSLOG, MFSLOG_INFO, "connecting ...");
        }
        0
    }

    fn connect_failed(&mut self) {
        tcp::close(self.c.sock);
        self.c.sock = -1;
        self.c.mode = Mode::Free;
        self.c.masteraddrvalid = 0;
    }

    fn connecttest(&mut self) {
        if tcp::getstatus(self.c.sock) != 0 {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, "connection failed, error");
            self.connect_failed();
        } else {
            log(MFSLOG_SYSLOG, MFSLOG_INFO, "connected to Master");
            self.connected();
        }
    }

    /// Input framing over freshly read bytes. Returns false when a packet
    /// header exceeds the limit (C returns from `masterconn_read` there,
    /// skipping the hup/error handling).
    fn feed(&mut self, buf: &[u8]) -> bool {
        let c = &mut self.c;
        let mut rbpos = 0usize;
        while rbpos < buf.len() {
            let n = (buf.len() - rbpos).min(c.input_bytesleft as usize);
            let src = &buf[rbpos..rbpos + n];
            match c.input_packet.as_mut() {
                Some(p) => p.data[c.input_off..c.input_off + n].copy_from_slice(src),
                None => c.input_hdr[c.input_off..c.input_off + n].copy_from_slice(src),
            }
            rbpos += n;
            c.input_off += n;
            c.input_bytesleft -= n as u32;
            if c.input_bytesleft > 0 {
                break;
            }
            if c.input_packet.is_none() {
                let ptype = be32(&c.input_hdr[..4]);
                let leng = be32(&c.input_hdr[4..]);
                if leng > MAX_PACKET_SIZE {
                    log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        &format!("Master packet too long ({leng}/{MAX_PACKET_SIZE}) ; command:{ptype}"),
                    );
                    c.input_end = true;
                    return false;
                }
                c.input_packet = Some(InPacket { ptype, data: vec![0; leng as usize] });
                c.input_off = 0;
                c.input_bytesleft = leng;
            }
            if c.input_bytesleft > 0 {
                continue;
            }
            if let Some(p) = c.input_packet.take() {
                c.inputq.push_back(p);
                c.input_bytesleft = 8;
                c.input_off = 0;
            }
        }
        true
    }

    fn read(&mut self, now: f64) {
        if self.readbuff.is_empty() {
            self.readbuff = vec![0; 65536];
        }
        let mut buf = std::mem::take(&mut self.readbuff);
        let (mut rbleng, mut err, mut hup) = (0usize, false, false);
        loop {
            let i = tcp::read(self.c.sock, &mut buf[rbleng..]);
            if i == 0 {
                hup = true;
                break;
            } else if i < 0 {
                if tcp::errno() != libc::EAGAIN {
                    err = true;
                }
                break;
            }
            rbleng += i as usize;
            if rbleng == buf.len() {
                let n = buf.len() * 2;
                buf.resize(n, 0);
            } else {
                break;
            }
        }
        if rbleng > 0 {
            self.c.lastread = now;
        }
        let complete = self.feed(&buf[..rbleng]);
        self.readbuff = buf;
        if !complete {
            return;
        }
        if hup {
            log(MFSLOG_SYSLOG, MFSLOG_NOTICE, "connection was reset by Master");
            self.c.input_end = true;
        } else if err {
            log(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, "read from Master error");
            self.c.input_end = true;
        }
    }

    fn parse(&mut self) {
        let starttime = monotonic_useconds();
        let mut currtime = starttime;
        while self.c.mode == Mode::Data && !self.c.inputq.is_empty() && starttime + 10000 > currtime {
            let p = self.c.inputq.pop_front().expect("non-empty");
            self.gotpacket(p.ptype, &p.data);
            if !self.c.inputq.is_empty() {
                currtime = monotonic_useconds();
            }
        }
        if self.c.mode == Mode::Data && self.c.inputq.is_empty() && self.c.input_end {
            self.c.mode = Mode::Kill;
        }
    }

    fn write(&mut self, now: f64) {
        let c = &mut self.c;
        loop {
            let bufs: Vec<&[u8]> = c.outputq.iter().take(100).map(|o| &o.data[o.start..]).collect();
            if bufs.is_empty() {
                return;
            }
            let leng = bufs.iter().fold(0u32, |a, b| a.wrapping_add(b.len() as u32));
            let i = tcp::writev(c.sock, &bufs);
            let e = tcp::errno();
            drop(bufs);
            if i < 0 {
                if e != libc::EAGAIN {
                    log(MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING, "write to Master error");
                    c.mode = Mode::Kill;
                }
                return;
            }
            if i > 0 {
                c.lastwrite = now;
            }
            let mut left = i as u32;
            while left > 0 {
                let Some(o) = c.outputq.front_mut() else { break };
                let bl = (o.data.len() - o.start) as u32;
                if bl > left {
                    o.start += left as usize;
                    left = 0;
                } else {
                    left -= bl;
                    c.outputq.pop_front();
                }
            }
            if (i as u32) < leng {
                return;
            }
        }
    }

    fn desc(&mut self, pdesc: &mut [libc::pollfd], ndesc: &mut u32) {
        let mut pos = *ndesc as usize;
        let c = &mut self.c;
        c.pdescpos = -1;
        if c.mode == Mode::Free || c.sock < 0 {
            return;
        }
        let mut events = 0;
        if c.mode == Mode::Data && !c.input_end {
            events |= libc::POLLIN;
        }
        if (c.mode == Mode::Data && !c.outputq.is_empty()) || c.mode == Mode::Connecting {
            events |= libc::POLLOUT;
        }
        pdesc[pos].events = events;
        if events != 0 {
            pdesc[pos].fd = c.sock;
            c.pdescpos = pos as i32;
            pos += 1;
        }
        *ndesc = pos as u32;
    }

    fn disconnection_check(&mut self) {
        if self.c.mode == Mode::Kill {
            self.beforeclose();
            tcp::close(self.c.sock);
            self.c.input_packet = None;
            self.c.inputq.clear();
            self.c.outputq.clear();
            self.c.mode = Mode::Free;
        }
    }

    fn serve(&mut self, pdesc: &[libc::pollfd]) {
        let now = monotonic_seconds();
        let slot = (self.c.pdescpos >= 0).then(|| pdesc[self.c.pdescpos as usize]);
        if self.c.mode == Mode::Connecting {
            if self.c.sock >= 0
                && slot.is_some_and(|p| p.revents & (libc::POLLOUT | libc::POLLHUP | libc::POLLERR) != 0)
            {
                self.connecttest();
            } else if self.c.conntime + 1.0 < now {
                log(MFSLOG_SYSLOG, MFSLOG_WARNING, "connection timed out");
                self.connect_failed();
            }
        } else {
            if let Some(p) = slot {
                if p.revents & (libc::POLLERR | libc::POLLIN) == libc::POLLIN && self.c.mode == Mode::Data {
                    self.read(now);
                }
                if p.revents & (libc::POLLERR | libc::POLLHUP) != 0 {
                    self.c.input_end = true;
                }
                self.parse();
            }
            if self.c.mode == Mode::Data && self.c.lastwrite + 1.0 < now && self.c.outputq.is_empty() {
                self.createpacket(ANTOAN_NOP, &[]);
            }
            if let Some(p) = slot
                && ((p.events & libc::POLLOUT == 0 && !self.c.outputq.is_empty()) || p.revents & libc::POLLOUT != 0)
                && self.c.mode == Mode::Data
            {
                self.write(now);
            }
            if self.c.mode == Mode::Data && self.c.lastread + (self.c.timeout as f64) < now {
                self.c.mode = Mode::Kill;
            }
        }
        self.disconnection_check();
    }

    fn info(&self) -> Vec<u8> {
        let c = &self.c;
        let mode = match c.mode {
            Mode::Free => "NOT CONNECTED",
            Mode::Connecting => "CONNECTING IN PROGRESS",
            Mode::Data => "CONNECTED",
            Mode::Kill => "DISCONNECTING",
        };
        let mut s = format!(
            "[master connection]\nmaster address is valid: {}\nworking timeout: {}\nsocket bind ip: {}\nresolved ip:port number: {}\nsocket mode: {mode}\n",
            c.masteraddrvalid,
            c.timeout,
            fmt_ip(c.bindip),
            fmt_ip_port(c.masterip, c.masterport),
        );
        if c.downloading == 1 {
            s.push_str("downloading metadata in progress\n");
        } else if c.downloading == 11 || c.downloading == 12 {
            s.push_str("downloading last changelogs in progress\n");
        }
        if c.downloading > 0 {
            let dltime = monotonic_useconds().wrapping_sub(c.dlstartuts);
            s.push_str(&format!(
                "downloading progress: {}/{}\ndownloading try counter: {}\ndownloading time: {}.{:06}s",
                c.dloffset,
                c.filesize,
                c.downloadretrycnt,
                dltime / 1_000_000,
                (dltime % 1_000_000) as u32
            ));
        }
        s.push('\n');
        s.into_bytes()
    }

    /// Config values shared by init and reload (cfg_info records the read
    /// order, so the callers keep C's order of `cfg_get*` calls).
    fn clamp_common(&mut self, metadlfreq: u32) -> u32 {
        self.timeout = self.timeout.clamp(10, 65535);
        self.back_logs = self.back_logs.clamp(5, 10000);
        metadlfreq.min(self.back_logs / 2)
    }
}

fn with_state(f: impl FnOnce(&mut State)) {
    if let Some(s) = state().as_mut() {
        f(s);
    }
}

fn masterconn_reconnect() {
    with_state(|s| {
        if s.c.mode == Mode::Free {
            s.initconnect();
        }
    });
}

fn masterconn_metadownloadinit() {
    with_state(|s| s.download_init(1));
}

fn masterconn_metachanges_flush() {
    with_state(|s| {
        if let Some(l) = s.c.logfd.as_mut() {
            let _ = l.flush();
        }
    });
}

fn masterconn_desc(pdesc: &mut [libc::pollfd], ndesc: &mut u32) {
    with_state(|s| s.desc(pdesc, ndesc));
}

fn masterconn_serve(pdesc: &[libc::pollfd]) {
    with_state(|s| s.serve(pdesc));
}

fn masterconn_info() -> Vec<u8> {
    state().as_ref().map_or_else(Vec::new, State::info)
}

/// C `masterconn_term`: closes the connection and drops all state (the
/// changelog buffer is flushed here; C's stdio flushed it at exit()).
fn masterconn_term() {
    let Some(s) = state().take() else { return };
    if s.c.mode != Mode::Free {
        tcp::close(s.c.sock);
    }
}

fn masterconn_reload() {
    with_state(|s| {
        s.master_host = cfg_str(b"MASTER_HOST", DEFAULT_MASTERNAME);
        s.master_port = cfg_str(b"MASTER_PORT", DEFAULT_MASTER_CONTROL_PORT);
        s.bind_host = cfg_str(b"BIND_HOST", b"*");
        s.c.masteraddrvalid = 0;
        if s.c.mode != Mode::Free {
            s.c.mode = Mode::Kill;
        }
        s.timeout = cfg_u32(b"MASTER_TIMEOUT", 10);
        s.back_logs = cfg_u32(b"BACK_LOGS", 50);
        s.back_meta_copies = cfg_u32(b"BACK_META_KEEP_PREVIOUS", 3);
        let reconnection_delay = cfg_u32(b"MASTER_RECONNECTION_DELAY", 5);
        let metadlfreq = cfg_u32(b"META_DOWNLOAD_FREQ", 24);
        let metadlfreq = s.clamp_common(metadlfreq);
        s.back_meta_copies = s.back_meta_copies.min(99);
        daemon::time_change(s.reconnect_hook, reconnection_delay, 0);
        daemon::time_change(s.download_hook, metadlfreq.wrapping_mul(3600), 630);
    });
}

/// C `masterconn_init` (init.h RunTab entry).
pub extern "C" fn masterconn_init() -> c_int {
    let reconnection_delay = cfg_u32(b"MASTER_RECONNECTION_DELAY", 5);
    let master_host = cfg_str(b"MASTER_HOST", DEFAULT_MASTERNAME);
    let master_port = cfg_str(b"MASTER_PORT", DEFAULT_MASTER_CONTROL_PORT);
    let bind_host = cfg_str(b"BIND_HOST", b"*");
    let timeout = cfg_u32(b"MASTER_TIMEOUT", 10);
    let back_logs = cfg_u32(b"BACK_LOGS", 50);
    let back_meta_copies = cfg_u32(b"BACK_META_KEEP_PREVIOUS", 3);
    let metadlfreq = cfg_u32(b"META_DOWNLOAD_FREQ", 24);
    let mut s = State {
        c: Conn::new(0),
        base: PathBuf::new(),
        back_logs,
        back_meta_copies,
        master_host,
        master_port,
        bind_host,
        timeout,
        lastlogversion: 0,
        reconnect_hook: None,
        download_hook: None,
        readbuff: Vec::new(),
    };
    let metadlfreq = s.clamp_common(metadlfreq);
    s.c.timeout = s.timeout as u16;
    s.lastlogversion = find_last_log_version(&s.base);
    let mut guard = state();
    let s = guard.insert(s);
    if s.initconnect() < 0 {
        return -1;
    }
    s.reconnect_hook = daemon::time_register(reconnection_delay, 0, masterconn_reconnect, "masterconn_reconnect");
    s.download_hook = daemon::time_register(
        metadlfreq.wrapping_mul(3600),
        630,
        masterconn_metadownloadinit,
        "masterconn_metadownloadinit",
    );
    daemon::destruct_register(masterconn_term, "masterconn_term");
    daemon::poll_register(masterconn_desc, masterconn_serve, "masterconn_desc", "masterconn_serve");
    daemon::reload_register(masterconn_reload, "masterconn_reload");
    daemon::time_register(1, 0, masterconn_metachanges_flush, "masterconn_metachanges_flush");
    daemon::info_register(masterconn_info, "masterconn_info");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Scratch dir under target/ (never /tmp), removed by `Drop`.
    struct Scratch(PathBuf);

    impl Scratch {
        fn new(name: &str) -> Self {
            let p = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/mltest")).join(name);
            let _ = fs::remove_dir_all(&p);
            fs::create_dir_all(&p).unwrap();
            Scratch(p)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn st(base: &Path) -> State {
        State {
            c: Conn::new(10),
            base: base.to_path_buf(),
            back_logs: 5,
            back_meta_copies: 3,
            master_host: c"mfsmaster".into(),
            master_port: c"9419".into(),
            bind_host: c"*".into(),
            timeout: 10,
            lastlogversion: 0,
            reconnect_hook: None,
            download_hook: None,
            readbuff: Vec::new(),
        }
    }

    fn pkt(ptype: u32, data: &[u8]) -> Vec<u8> {
        let mut v = ptype.to_be_bytes().to_vec();
        v.extend_from_slice(&(data.len() as u32).to_be_bytes());
        v.extend_from_slice(data);
        v
    }

    fn sent(s: &mut State) -> Vec<Vec<u8>> {
        s.c.outputq.drain(..).map(|o| o.data).collect()
    }

    fn lastlog(dir: &Path, changelog: &[u8]) -> (u64, Vec<u8>) {
        fs::write(dir.join("metadata_ml.mfs.back"), b"x").unwrap();
        fs::write(dir.join("changelog_ml.0.back"), changelog).unwrap();
        let v = find_last_log_version(dir);
        (v, fs::read(dir.join("changelog_ml.0.back")).unwrap())
    }

    #[test]
    fn last_log_version_basic_cases() {
        let d = Scratch::new("lastlog");
        assert_eq!(find_last_log_version(&d.0), 0, "no metadata backup");
        fs::write(d.0.join("metadata_ml.mfs.back"), b"").unwrap();
        fs::write(d.0.join("changelog_ml.0.back"), b"1: a\n2: b\n").unwrap();
        assert_eq!(find_last_log_version(&d.0), 0, "empty metadata backup");
        assert_eq!(lastlog(&d.0, b"5: a\n6: b\n"), (6, b"5: a\n6: b\n".to_vec()));
        // one complete line: C finds no second newline
        assert_eq!(lastlog(&d.0, b"5: a\n").0, 0);
        // garbage after the last newline is truncated
        assert_eq!(lastlog(&d.0, b"5: a\n6: b\ngarb"), (6, b"5: a\n6: b\n".to_vec()));
        // line not starting with digits+':'
        assert_eq!(lastlog(&d.0, b"5: a\nx6: b\n").0, 0);
        assert_eq!(lastlog(&d.0, b"5: a\n6 b\n").0, 0);
    }

    #[test]
    fn last_log_version_block_carry() {
        let d = Scratch::new("lastlog_carry");
        // the version digits start exactly at a 32 KiB block boundary
        // (blocks are aligned from the end of the file)
        let mut tail = b"123456789: ".to_vec();
        tail.resize(32767, b'y');
        tail.push(b'\n');
        for filler in [100usize, 40000] {
            let mut f = vec![b'x'; filler];
            f.push(b'\n');
            f.extend_from_slice(&tail);
            assert_eq!(lastlog(&d.0, &f).0, 123456789, "filler {filler}");
        }
        // digits running past the 32-byte carry: C stops at 32800 (0)
        let mut tail = vec![b'7'; 40];
        tail.extend_from_slice(b": ");
        tail.resize(32767, b'y');
        tail.push(b'\n');
        let mut f = vec![b'x'; 40000];
        f.push(b'\n');
        f.extend_from_slice(&tail);
        assert_eq!(lastlog(&d.0, &f).0, 0);
    }

    #[test]
    fn last_log_version_window() {
        let d = Scratch::new("lastlog_window");
        let mut f = b"1: a\n".to_vec();
        f.resize(f.len() + 250_000, b'x');
        f.push(b'\n');
        let (v, after) = lastlog(&d.0, &f);
        assert_eq!(v, 0, "penultimate newline outside the 200000-byte window");
        assert_eq!(after, f, "no truncation");
    }

    #[test]
    fn framing_any_split() {
        let d = Scratch::new("framing");
        let stream: Vec<u8> = [pkt(0, &[]), pkt(52, &[1, 0, 0, 0, 0]), pkt(7, &[9; 300])].concat();
        for chunk in 1..=stream.len() {
            let mut s = st(&d.0);
            for c in stream.chunks(chunk) {
                assert!(s.feed(c));
            }
            let got: Vec<(u32, usize)> = s.c.inputq.iter().map(|p| (p.ptype, p.data.len())).collect();
            assert_eq!(got, vec![(0, 0), (52, 5), (7, 300)], "chunk {chunk}");
            assert_eq!(s.c.input_bytesleft, 8);
        }
        let mut s = st(&d.0);
        let mut big = 9u32.to_be_bytes().to_vec();
        big.extend_from_slice(&(MAX_PACKET_SIZE + 1).to_be_bytes());
        assert!(!s.feed(&big));
        assert!(s.c.input_end);
        assert!(s.c.inputq.is_empty());
    }

    #[test]
    fn register_and_acks() {
        let d = Scratch::new("register");
        let mut s = st(&d.0);
        s.c.timeout = 60;
        s.sendregister();
        s.lastlogversion = 41;
        s.sendregister();
        assert_eq!(
            sent(&mut s),
            vec![
                pkt(ANTOMA_REGISTER, &[1, 0, 4, 59, 4, 0, 60]),
                pkt(ANTOMA_REGISTER, &[2, 0, 4, 59, 4, 0, 60, 0, 0, 0, 0, 0, 0, 0, 42]),
            ]
        );
        s.c.mode = Mode::Data;
        s.gotpacket(ANTOAN_FORCE_TIMEOUT, &[0, 3]);
        assert_eq!(s.c.timeout, 10);
        s.gotpacket(MATOAN_MASTER_ACK, &[1, 0, 0, 0, 0]);
        assert_eq!(s.c.mode, Mode::Data);
        s.gotpacket(MATOAN_MASTER_ACK, &[2, 0, 0, 0, 0]);
        assert_eq!(s.c.mode, Mode::Kill);
        s.c.mode = Mode::Data;
        s.gotpacket(999, &[]);
        assert_eq!(s.c.mode, Mode::Kill);
    }

    fn change(version: u64, text: &[u8]) -> Vec<u8> {
        let mut v = vec![0xFF];
        v.extend_from_slice(&version.to_be_bytes());
        v.extend_from_slice(text);
        v.push(0);
        v
    }

    #[test]
    fn changelog_append_rotate_and_loss() {
        let d = Scratch::new("changelog");
        let mut s = st(&d.0);
        s.c.mode = Mode::Data;
        s.gotpacket(MATOAN_METACHANGES_LOG, &change(10, b"CREATE(1)"));
        s.gotpacket(MATOAN_METACHANGES_LOG, &change(11, b"A\0hidden"));
        masterconn_metachanges_flush_state(&mut s);
        assert_eq!(fs::read(d.0.join("changelog_ml.0.mfs")).unwrap(), b"10: CREATE(1)\n11: A\n");
        assert_eq!(s.lastlogversion, 11);
        // rotation marker: .0 -> .1
        s.gotpacket(MATOAN_METACHANGES_LOG, &[0x55]);
        assert!(!d.0.join("changelog_ml.0.mfs").exists());
        assert_eq!(fs::read(d.0.join("changelog_ml.1.mfs")).unwrap(), b"10: CREATE(1)\n11: A\n");
        s.gotpacket(MATOAN_METACHANGES_LOG, &change(12, b"X"));
        // gap: everything is removed and the connection is killed
        s.gotpacket(MATOAN_METACHANGES_LOG, &change(20, b"Y"));
        assert_eq!(s.c.mode, Mode::Kill);
        assert_eq!(s.lastlogversion, 0);
        assert!(!d.0.join("changelog_ml.0.mfs").exists());
        assert!(!d.0.join("changelog_ml.1.mfs").exists());
        // malformed packets
        for bad in [&b"\xFF\0\0\0\0\0\0\0\x01"[..], &b"\xFE\0\0\0\0\0\0\0\x01x\0"[..], &b"\xFF\0\0\0\0\0\0\0\x01xy"[..]] {
            s.c.mode = Mode::Data;
            s.gotpacket(MATOAN_METACHANGES_LOG, bad);
            assert_eq!(s.c.mode, Mode::Kill);
        }
    }

    fn masterconn_metachanges_flush_state(s: &mut State) {
        if let Some(l) = s.c.logfd.as_mut() {
            l.flush().unwrap();
        }
    }

    fn metadata_file(body: &[u8]) -> Vec<u8> {
        let mut v = b"MFSM 2.0".to_vec();
        v.extend_from_slice(&7u64.to_be_bytes());
        v.extend_from_slice(&0xABCDu64.to_be_bytes());
        v.extend_from_slice(body);
        v.extend_from_slice(b"[MFS EOF MARKER]");
        v
    }

    fn data_pkt(offset: u64, block: &[u8], crc: u32) -> Vec<u8> {
        let mut v = offset.to_be_bytes().to_vec();
        v.extend_from_slice(&(block.len() as u32).to_be_bytes());
        v.extend_from_slice(&crc.to_be_bytes());
        v.extend_from_slice(block);
        v
    }

    #[test]
    fn download_metadata_then_changelogs() {
        let d = Scratch::new("download");
        let mut s = st(&d.0);
        s.c.mode = Mode::Data;
        fs::write(d.0.join("metadata_ml.mfs.back"), b"old").unwrap();
        s.download_init(1);
        assert_eq!(sent(&mut s), vec![pkt(ANTOMA_DOWNLOAD_START, &[1])]);
        let file = metadata_file(&vec![5u8; 1_500_000]);
        s.gotpacket(MATOAN_DOWNLOAD_INFO, &(file.len() as u64).to_be_bytes());
        let mut off = 0u64;
        let mut retried = false;
        loop {
            let out = sent(&mut s);
            if be32(&out[0]) != ANTOMA_DOWNLOAD_REQUEST {
                assert_eq!(out, vec![pkt(ANTOMA_DOWNLOAD_END, &[]), pkt(ANTOMA_DOWNLOAD_START, &[11])]);
                break;
            }
            assert_eq!(out.len(), 1);
            let p = &out[0];
            assert_eq!(be64(&p[8..16]), off);
            let n = be32(&p[16..20]) as usize;
            assert_eq!(n as u64, (file.len() as u64 - off).min(META_DL_BLOCK));
            let block = &file[off as usize..off as usize + n];
            if !retried {
                // bad crc: same block requested again
                retried = true;
                s.gotpacket(MATOAN_DOWNLOAD_DATA, &data_pkt(off, block, 0));
                assert_eq!(s.c.downloadretrycnt, 1);
                continue;
            }
            s.gotpacket(MATOAN_DOWNLOAD_DATA, &data_pkt(off, block, crc32(0, block)));
            off += n as u64;
        }
        assert_eq!(fs::read(d.0.join("metadata_ml.mfs.back")).unwrap(), file);
        assert_eq!(fs::read(d.0.join("metadata_ml.mfs.back.1")).unwrap(), b"old");
        assert_eq!(s.c.downloading, 11);
        // changelog_0 download (empty file), then changelog_1
        s.gotpacket(MATOAN_DOWNLOAD_INFO, &0u64.to_be_bytes());
        assert_eq!(sent(&mut s), vec![pkt(ANTOMA_DOWNLOAD_END, &[]), pkt(ANTOMA_DOWNLOAD_START, &[12])]);
        assert!(d.0.join("changelog_ml_back.0.mfs").exists());
        s.gotpacket(MATOAN_DOWNLOAD_INFO, &[0]);
        assert_eq!(s.c.downloading, 0);
        assert_eq!(s.c.mode, Mode::Data);
    }

    #[test]
    fn download_data_checks() {
        let d = Scratch::new("download_checks");
        let mut s = st(&d.0);
        s.c.mode = Mode::Data;
        s.gotpacket(MATOAN_DOWNLOAD_DATA, &data_pkt(0, b"ab", 0));
        assert_eq!(s.c.mode, Mode::Kill, "file not opened");
        s.c.mode = Mode::Data;
        s.download_init(1);
        s.gotpacket(MATOAN_DOWNLOAD_INFO, &10u64.to_be_bytes());
        for bad in [data_pkt(1, b"ab", 0), data_pkt(0, &[0; 11], 0), data_pkt(0, b"ab", 0)[..15].to_vec()] {
            s.c.mode = Mode::Data;
            s.gotpacket(MATOAN_DOWNLOAD_DATA, &bad);
            assert_eq!(s.c.mode, Mode::Kill);
        }
        // closing mid-download removes the temp file
        s.beforeclose();
        assert!(!d.0.join("metadata_ml.tmp").exists());
    }

    #[test]
    fn metadata_check_formats() {
        let d = Scratch::new("mcheck");
        let p = d.0.join("m");
        let check = |body: &[u8]| {
            fs::write(&p, body).unwrap();
            metadata_check(&p)
        };
        assert_eq!(check(&metadata_file(b"zz")), 0);
        assert_eq!(check(b"MFSM NEW"), -1);
        assert_eq!(check(b"XFSM 2.0........"), -1);
        let mut trunc = metadata_file(b"zz");
        trunc.pop();
        assert_eq!(check(&trunc), -1);
        // pre-1.7 format: 16 zero bytes as EOF marker
        let mut old = b"MFSM 1.6".to_vec();
        old.extend_from_slice(&[1; 20]);
        old.extend_from_slice(&[0; 16]);
        assert_eq!(check(&old), 0);
        assert_eq!(check(b"MFSM 1.6"), -1, "short file: lseek fails, read hits EOF");
    }

    #[test]
    fn info_text() {
        let d = Scratch::new("info");
        let mut s = st(&d.0);
        s.c.bindip = 0x7F00_0001;
        s.c.masterip = 0x0A00_0002;
        s.c.masterport = 9419;
        s.c.masteraddrvalid = 1;
        assert_eq!(
            String::from_utf8(s.info()).unwrap(),
            "[master connection]\nmaster address is valid: 1\nworking timeout: 10\nsocket bind ip: 127.0.0.1\nresolved ip:port number: 10.0.0.2:9419\nsocket mode: NOT CONNECTED\n\n"
        );
        s.c.downloading = 11;
        s.c.dloffset = 5;
        s.c.filesize = 9;
        let t = String::from_utf8(s.info()).unwrap();
        assert!(t.contains("downloading last changelogs in progress\ndownloading progress: 5/9\ndownloading try counter: 0\ndownloading time: "));
        assert!(t.ends_with("s\n"));
    }
}
