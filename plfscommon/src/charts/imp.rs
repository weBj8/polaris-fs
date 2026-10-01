//! Safe core of mfscommon/charts.c: series storage, the stats file
//! (`charts_store` / `charts_load`), RPN calc evaluation and the data /
//! monotonic exports. Rendering lives in `render`.
//!
//! Integer semantics follow the C exactly: `uint32_t` arithmetic wraps,
//! `int32_t` deltas come from wrapped unsigned subtraction, sums of
//! `uint64_t` samples wrap.
#![deny(unsafe_code)]

use core::ffi::c_int;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use flate2::{Compress, Compression};

use super::sys;
use super::tables::{DATA_COLORS, PNG_HEADER};
use crate::mfslog::{
    MFSLOG_ERRNO_SYSLOG_STDERR, MFSLOG_INFO, MFSLOG_NOTICE, MFSLOG_SYSLOG_STDERR, MFSLOG_WARNING,
    log_bytes, log_bytes_errno,
};

pub(super) const MAXLENG: u32 = 4096;
pub(super) const MINLENG: u32 = 100;
pub(super) const MAXHEIGHT: u32 = 1000;
pub(super) const MINHEIGHT: u32 = 100;
pub(super) const XPOS: i32 = 43;
pub(super) const YPOS: i32 = 6;
pub(super) const XADD: u32 = 50;
pub(super) const YADD: u32 = 20;
pub(super) const MAXXSIZE: usize = MAXLENG as usize + XADD as usize;
pub(super) const MAXYSIZE: usize = MAXHEIGHT as usize + YADD as usize;
pub(super) const RANGES: usize = 4;
pub(super) const NODATA: u64 = u64::MAX;

pub(super) const MODE_ADD: u8 = 0;
const SCALE_MICRO: u8 = 0;
const SCALE_MILI: u8 = 1;
const SCALE_NONE: u8 = 2;
const SCALE_KILO: u8 = 3;
const SCALE_MEGA: u8 = 4;
const SCALE_GIGA: u8 = 5;

const OP_CONST: u32 = 1000;
const OP_ADD: u32 = 1001;
const OP_SUB: u32 = 1002;
const OP_MIN: u32 = 1003;
const OP_MAX: u32 = 1004;
const OP_MUL: u32 = 1005;
const OP_DIV: u32 = 1006;
const OP_NEG: u32 = 1007;
pub(super) const OP_END: u32 = 1999;
pub(super) const DEFS_END: u32 = 2000;
const DIRECT_START: u32 = 1;
const CALC_START: u32 = 1000;
const EXTENDED_START: u32 = 100;
const STACK_NODATA: i64 = i64::MIN;

const FILE_VERSION: u32 = 0x0001_0000;
/// Seconds per sample for SHORT / MEDIUM / LONG / VERYLONG range.
pub(super) const RANGE_SECONDS: [u32; RANGES] = [60, 60 * 6, 60 * 30, 60 * 60 * 24];

pub struct StatDef {
    pub name: Vec<u8>,
    pub statid: u32,
    pub mode: u8,
    pub percent: u8,
    pub scale: u8,
    pub multiplier: u16,
    pub divisor: u16,
}

/// `estatdef` without its name: C strdup's it but never reads it.
pub struct EStatDef {
    pub statid: u32,
    pub c1src: u32,
    pub c2src: u32,
    pub c3src: u32,
    pub mode: u8,
    pub percent: u8,
    pub scale: u8,
    pub multiplier: u16,
    pub divisor: u16,
}

/// Display parameters shared by statdef / estatdef.
#[derive(Clone, Copy)]
pub(super) struct DefParams {
    pub mode: u8,
    pub percent: u8,
    pub scale: u8,
    pub multiplier: u16,
    pub divisor: u16,
}

/// Chart subscript clock (C `shhour,shmin,medhour,...,vlngyear`).
#[derive(Clone, Copy, Default)]
pub(super) struct Times {
    pub shhour: u32,
    pub shmin: u32,
    pub medhour: u32,
    pub medmin: u32,
    pub lnghalfhour: u32,
    pub lngmday: u32,
    pub lngmonth: u32,
    pub lngyear: u32,
    pub vlngmday: u32,
    pub vlngmonth: u32,
    pub vlngyear: u32,
}

pub struct Charts {
    pub(super) calcdefs: Vec<u32>,
    pub(super) calcstart: Vec<usize>,
    pub(super) statdefs: Vec<StatDef>,
    pub(super) estatdefs: Vec<EStatDef>,
    pub(super) filename: PathBuf,
    pub(super) series: Vec<[Vec<u64>; RANGES]>,
    pub(super) pointers: [u32; RANGES],
    pub(super) timepoint: [u32; RANGES],
    pub(super) monotonic: Vec<u64>,
    pub(super) t: Times,
    pub(super) chart: Vec<u8>,
    /// Whole buffer is deflated, including bytes beyond the current image
    /// left by earlier (larger) charts, exactly as C does.
    pub(super) rawchart: Vec<u8>,
    pub(super) compbuff: Vec<u8>,
    pub(super) compsize: u32,
    /// None until init succeeds: C returns from a failed `charts_load`
    /// before `deflateInit`, so every later PNG degrades to the 1x1 image.
    pub(super) zstr: Option<Compress>,
    pub(super) png_header: [u8; PNG_HEADER.len()],
}

/// Big-endian writer over a caller buffer (datapack.h put*bit).
pub(super) struct Put<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> Put<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Put { buf, pos: 0 }
    }
    fn bytes(&mut self, b: &[u8]) {
        self.buf[self.pos..self.pos + b.len()].copy_from_slice(b);
        self.pos += b.len();
    }
    pub fn u8(&mut self, v: u8) {
        self.bytes(&[v]);
    }
    pub fn u16(&mut self, v: u16) {
        self.bytes(&v.to_be_bytes());
    }
    pub fn u32(&mut self, v: u32) {
        self.bytes(&v.to_be_bytes());
    }
    pub fn u64(&mut self, v: u64) {
        self.bytes(&v.to_be_bytes());
    }
}

fn be32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

fn os_errno(e: &io::Error) -> c_int {
    e.raw_os_error().unwrap_or(0)
}

/// errno as the C code would see it after a short (non-failing) read/write.
fn cur_errno() -> c_int {
    io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

/// One read(2); C treats any count other than the full length as an error.
fn read_once(f: &mut File, buf: &mut [u8]) -> Result<(), c_int> {
    match f.read(buf) {
        Ok(n) if n == buf.len() => Ok(()),
        Ok(_) => Err(cur_errno()),
        Err(e) => Err(os_errno(&e)),
    }
}

/// One write(2), same convention as `read_once`.
fn write_once(f: &mut File, buf: &[u8]) -> Result<(), c_int> {
    match f.write(buf) {
        Ok(n) if n == buf.len() => Ok(()),
        Ok(_) => Err(cur_errno()),
        Err(e) => Err(os_errno(&e)),
    }
}

/// `fprintf(stderr, "<prefix>%s\n", strerr(errno))`.
fn stderr_errno(prefix: &[u8], errno: c_int) {
    let mut line = prefix.to_vec();
    line.extend_from_slice(&crate::strerr::message(errno));
    line.push(b'\n');
    let _ = io::stderr().write_all(&line);
}

/// PLTE entries 50..200: three linear ramps (png_make_palette).
fn make_palette(header: &mut [u8; PNG_HEADER.len()]) {
    for rng in 0..3 {
        let mut r = DATA_COLORS[rng * 6] as f64;
        let mut g = DATA_COLORS[rng * 6 + 1] as f64;
        let mut b = DATA_COLORS[rng * 6 + 2] as f64;
        let mut dr = DATA_COLORS[rng * 6 + 3] as f64;
        let mut dg = DATA_COLORS[rng * 6 + 4] as f64;
        let mut db = DATA_COLORS[rng * 6 + 5] as f64;
        dr -= r;
        dg -= g;
        db -= b;
        dr /= 50.0;
        dg /= 50.0;
        db /= 50.0;
        r += 0.5;
        g += 0.5;
        b += 0.5;
        for indx in 0..50 {
            let o = 41 + (50 + rng * 50 + indx) * 3;
            header[o] = super::render::c_f64_to_u8(r);
            header[o + 1] = super::render::c_f64_to_u8(g);
            header[o + 2] = super::render::c_f64_to_u8(b);
            r += dr;
            g += dg;
            b += db;
        }
    }
}

impl Charts {
    /// charts_init: `calcs` excludes the CHARTS_DEFS_END terminator.
    pub fn init(
        calcs: &[u32],
        statdefs: Vec<StatDef>,
        estatdefs: Vec<EStatDef>,
        filename: PathBuf,
        mode: u8,
    ) -> (Charts, c_int) {
        let rawchartsize = (1 + MAXXSIZE) * MAXYSIZE;
        let compbuffsize = (rawchartsize as u64 * 1001 / 1000 + 16) as usize;
        let calccount = calcs.iter().filter(|&&op| op == OP_END).count();
        let (calcdefs, calcstart) = if !calcs.is_empty() && calccount > 0 {
            let mut starts = vec![0usize];
            for (i, &op) in calcs.iter().enumerate() {
                if op == OP_END && starts.len() < calccount {
                    starts.push(i + 1);
                }
            }
            (calcs.to_vec(), starts)
        } else {
            (Vec::new(), Vec::new())
        };
        let n = statdefs.len();
        let mut header = PNG_HEADER;
        make_palette(&mut header);
        let mut c = Charts {
            calcdefs,
            calcstart,
            statdefs,
            estatdefs,
            filename,
            series: (0..n)
                .map(|_| std::array::from_fn(|_| vec![NODATA; MAXLENG as usize]))
                .collect(),
            pointers: [0; RANGES],
            timepoint: [0; RANGES],
            monotonic: vec![0; n],
            t: Times::default(),
            chart: vec![0; MAXXSIZE * MAXYSIZE],
            rawchart: vec![0; rawchartsize],
            compbuff: vec![0; compbuffsize],
            compsize: 0,
            zstr: None,
            png_header: header,
        };
        if c.load(mode) < 0 {
            return (c, -1);
        }
        c.inittimepointers();
        if mode == 0 {
            c.add(None, sys::now() as u32);
        }
        c.zstr = Some(Compress::new(Compression::new(1), true));
        (c, 0)
    }

    pub fn stat_count(&self) -> usize {
        self.statdefs.len()
    }

    fn statcnt(&self) -> u32 {
        self.statdefs.len() as u32
    }

    pub(super) fn is_direct_stat(&self, x: u32) -> bool {
        x < self.statcnt()
    }

    pub(super) fn is_extended_stat(&self, x: u32) -> bool {
        x >= EXTENDED_START && x < EXTENDED_START + self.estatdefs.len() as u32
    }

    fn def_is_direct(&self, x: u32) -> bool {
        x >= DIRECT_START && x < DIRECT_START + self.statcnt()
    }

    fn def_is_calc(&self, x: u32) -> bool {
        x >= CALC_START && x < CALC_START + self.calcstart.len() as u32
    }

    pub(super) fn def_params(&self, ty: u32) -> Option<DefParams> {
        if self.is_direct_stat(ty) {
            let s = &self.statdefs[ty as usize];
            Some(DefParams {
                mode: s.mode,
                percent: s.percent,
                scale: s.scale,
                multiplier: s.multiplier,
                divisor: s.divisor,
            })
        } else if self.is_extended_stat(ty) {
            let e = &self.estatdefs[(ty - EXTENDED_START) as usize];
            Some(DefParams {
                mode: e.mode,
                percent: e.percent,
                scale: e.scale,
                multiplier: e.multiplier,
                divisor: e.divisor,
            })
        } else {
            None
        }
    }

    pub fn store(&self) {
        let mut f = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .mode(0o666)
            .open(&self.filename)
        {
            Ok(f) => f,
            Err(e) => {
                log_bytes_errno(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"error creating charts data file",
                    os_errno(&e),
                );
                return;
            }
        };
        let werr = |errno: c_int| {
            log_bytes_errno(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error writing charts data file",
                errno,
            )
        };
        let mut hdr = [0u8; 16];
        let mut p = Put::new(&mut hdr);
        p.u32(FILE_VERSION);
        p.u32(MAXLENG);
        p.u32(self.statcnt());
        p.u32(self.timepoint[0]);
        if let Err(e) = write_once(&mut f, &hdr) {
            return werr(e);
        }
        let mut data = vec![0u8; 8 * MAXLENG as usize];
        for (i, sd) in self.statdefs.iter().enumerate() {
            let mut namehdr = [0u8; 100];
            let s = sd.name.len().min(100);
            namehdr[..s].copy_from_slice(&sd.name[..s]);
            if let Err(e) = write_once(&mut f, &namehdr) {
                return werr(e);
            }
            for j in 0..RANGES {
                let tab = &self.series[i][j];
                let p0 = self.pointers[j].wrapping_add(1);
                let mut p = Put::new(&mut data);
                for s in 0..MAXLENG {
                    p.u64(tab[(p0.wrapping_add(s) % MAXLENG) as usize]);
                }
                if let Err(e) = write_once(&mut f, &data) {
                    return werr(e);
                }
            }
        }
    }

    fn load(&mut self, mode: u8) -> c_int {
        let mut f = match File::open(&self.filename) {
            Ok(f) => f,
            Err(e) => {
                let errno = os_errno(&e);
                if mode == 1 {
                    stderr_errno(b"file loading error: ", errno);
                    return -1;
                }
                if errno != libc::ENOENT {
                    log_bytes_errno(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"error reading charts data file",
                        errno,
                    );
                } else {
                    log_bytes(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"no charts data file - initializing empty charts",
                    );
                }
                return 0;
            }
        };
        let read_err = |errno: c_int| -> c_int {
            if mode == 1 {
                stderr_errno(b"error reading charts data file: ", errno);
                return -1;
            }
            log_bytes_errno(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error reading charts data file",
                errno,
            );
            0
        };
        let mut hdr = [0u8; 16];
        if let Err(e) = read_once(&mut f, &mut hdr) {
            return read_err(e);
        }
        if be32(&hdr[0..4]) != FILE_VERSION {
            if mode == 1 {
                let _ = io::stderr().write_all(b"unrecognized charts data file format\n");
                return -1;
            }
            log_bytes(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"unrecognized charts data file format - initializing empty charts",
            );
            return 0;
        }
        let fleng = be32(&hdr[4..8]);
        let fcharts = be32(&hdr[8..12]);
        self.timepoint[0] = be32(&hdr[12..16]);
        self.pointers = [MAXLENG - 1; RANGES];
        for _ in 0..fcharts {
            let mut namehdr = [0u8; 100];
            if let Err(e) = read_once(&mut f, &mut namehdr) {
                return read_err(e);
            }
            let name = &namehdr[..namehdr.iter().position(|&b| b == 0).unwrap_or(100)];
            let Some(j) = self.statdefs.iter().position(|s| s.name == name) else {
                // unknown chart: skip its data (C computes the offset in uint32_t)
                let skip = 4u32.wrapping_mul(fleng).wrapping_mul(8);
                let _ = f.seek(SeekFrom::Current(skip as i64));
                continue;
            };
            for k in 0..RANGES {
                if fleng > MAXLENG {
                    let _ = f.seek(SeekFrom::Current(((fleng - MAXLENG) as u64 * 8) as i64));
                }
                let n = fleng.min(MAXLENG) as usize;
                let mut data = vec![0u8; 8 * n];
                if let Err(e) = read_once(&mut f, &mut data) {
                    return read_err(e);
                }
                let tab = &mut self.series[j][k];
                for (slot, v) in tab[MAXLENG as usize - n..].iter_mut().zip(data.chunks_exact(8)) {
                    *slot = u64::from_be_bytes(v.try_into().expect("chunk of 8"));
                }
            }
        }
        if mode == 1 {
            return 0;
        }
        log_bytes(MFSLOG_SYSLOG_STDERR, MFSLOG_INFO, b"stats file has been loaded");
        0
    }

    fn set_times(&mut self, r: usize, tm: &sys::Tm) {
        let t = &mut self.t;
        match r {
            0 => {
                t.shmin = tm.min as u32;
                t.shhour = tm.hour as u32;
            }
            1 => {
                t.medmin = tm.min as u32;
                t.medhour = tm.hour as u32;
            }
            2 => {
                t.lnghalfhour = (tm.hour as u32).wrapping_mul(2);
                if tm.min >= 30 {
                    t.lnghalfhour = t.lnghalfhour.wrapping_add(1);
                }
                t.lngmday = tm.mday as u32;
                t.lngmonth = (tm.mon + 1) as u32;
                t.lngyear = (tm.year + 1900) as u32;
            }
            _ => {
                t.vlngmday = tm.mday as u32;
                t.vlngmonth = (tm.mon + 1) as u32;
                t.vlngyear = (tm.year + 1900) as u32;
            }
        }
    }

    fn inittimepointers(&mut self) {
        let (tm, local) = if self.timepoint[0] == 0 {
            let now = sys::now();
            let tm = sys::localtime(now);
            (tm, now.wrapping_add(tm.gmtoff) as i32)
        } else {
            // C: `now = timepoint*60` is computed in uint32_t
            let now = self.timepoint[0].wrapping_mul(60) as i64;
            (sys::gmtime(now), now as i32)
        };
        for r in 0..RANGES {
            self.timepoint[r] = (local / RANGE_SECONDS[r] as i32) as u32;
            self.set_times(r, &tm);
        }
    }

    pub fn add(&mut self, data: Option<&[u64]>, datats: u32) {
        if let Some(d) = data {
            for (m, &v) in self.monotonic.iter_mut().zip(d) {
                *m = m.wrapping_add(v);
            }
        }
        let now = datats as i64;
        let tm = sys::localtime(now);
        let local = now.wrapping_add(tm.gmtoff) as i32;
        for r in 0..RANGES {
            let nowtime = local / RANGE_SECONDS[r] as i32;
            let mut delta = (nowtime as u32).wrapping_sub(self.timepoint[r]) as i32;
            if delta > 0 {
                if delta > MAXLENG as i32 {
                    delta = MAXLENG as i32;
                }
                while delta > 0 {
                    self.pointers[r] = (self.pointers[r] + 1) % MAXLENG;
                    let p = self.pointers[r] as usize;
                    for s in self.series.iter_mut() {
                        s[r][p] = NODATA;
                    }
                    delta -= 1;
                }
                self.timepoint[r] = nowtime as u32;
                self.set_times(r, &tm);
            }
            if let Some(d) = data
                && delta <= 0
                && delta > -(MAXLENG as i32)
            {
                let i = (self.pointers[r].wrapping_add(MAXLENG).wrapping_add(delta as u32) % MAXLENG)
                    as usize;
                for (j, sd) in self.statdefs.iter().enumerate() {
                    let v = &mut self.series[j][r][i];
                    if *v == NODATA {
                        *v = d[j];
                    } else if sd.mode == MODE_ADD {
                        *v = v.wrapping_add(d[j]);
                    } else if d[j] > *v {
                        *v = d[j];
                    }
                }
            }
        }
    }

    pub fn get(&self, ty: u32, numb: u32) -> u64 {
        let mut result: u64 = 0;
        if numb == 0 || numb > MAXLENG || !self.is_direct_stat(ty) {
            return result;
        }
        let tab = &self.series[ty as usize][0];
        let mut j = (self.pointers[0] % MAXLENG) as usize;
        let add = self.statdefs[ty as usize].mode == MODE_ADD;
        let mut cnt: u64 = 0;
        for _ in 0..numb {
            let v = tab[j];
            if add {
                if v != NODATA {
                    result = result.wrapping_add(v);
                    cnt += 1;
                }
            } else if v != NODATA && v > result {
                result = v;
            }
            j = if j > 0 { j - 1 } else { MAXLENG as usize - 1 };
        }
        if add && cnt > 0 {
            result /= cnt;
        }
        result
    }

    pub(super) fn seriescnt(&self, ty: u32) -> u8 {
        if self.is_direct_stat(ty) {
            return 1;
        }
        if !self.is_extended_stat(ty) {
            return 0;
        }
        let e = &self.estatdefs[(ty - EXTENDED_START) as usize];
        let mut s = 0;
        for src in [e.c1src, e.c2src, e.c3src] {
            if !(self.def_is_direct(src) || self.def_is_calc(src)) {
                break;
            }
            s += 1;
        }
        s
    }

    /// Evaluates one RPN calc definition at sample `j` of range `r`.
    fn eval(&self, start: usize, r: usize, j: usize) -> u64 {
        let ops = &self.calcdefs;
        let mut stack = [0i64; 50];
        let mut sp = 0usize;
        let mut k = start;
        // `get` stops at the array end: only reachable with a malformed
        // definition (C reads past the copy there).
        while let Some(&op) = ops.get(k).filter(|&&op| op != OP_END) {
            if self.is_direct_stat(op) {
                if sp < 50 {
                    let v = self.series[op as usize][r][j];
                    stack[sp] = if v == NODATA { STACK_NODATA } else { v as i64 };
                    sp += 1;
                }
            } else if op == OP_CONST {
                k += 1;
                if sp < 50 {
                    stack[sp] = ops.get(k).copied().unwrap_or(0) as i64;
                    sp += 1;
                }
            } else if op == OP_NEG {
                if sp >= 1 && stack[sp - 1] != STACK_NODATA {
                    stack[sp - 1] = stack[sp - 1].wrapping_neg();
                }
            } else if (OP_ADD..=OP_DIV).contains(&op) {
                if sp >= 2 {
                    let (a, b) = (stack[sp - 2], stack[sp - 1]);
                    stack[sp - 2] = if a == STACK_NODATA
                        || b == STACK_NODATA
                        || (op == OP_DIV && b == 0)
                    {
                        STACK_NODATA
                    } else {
                        match op {
                            OP_ADD => a.wrapping_add(b),
                            OP_SUB => a.wrapping_sub(b),
                            OP_MIN => a.min(b),
                            OP_MAX => a.max(b),
                            OP_MUL => a.wrapping_mul(b),
                            _ => a.wrapping_div(b),
                        }
                    };
                    sp -= 1;
                }
            }
            k += 1;
        }
        // STACK_NODATA < 0, so `>= 0` also rejects it
        if sp >= 1 && stack[sp - 1] >= 0 {
            stack[sp - 1] as u64
        } else {
            NODATA
        }
    }

    /// charts_filltab: the last `width` samples of series `cno` (1..3) of
    /// chart `ty`, oldest first. `buf` is allocated on first success.
    pub(super) fn filltab(
        &self,
        buf: &mut Option<Vec<u64>>,
        range: u32,
        ty: u32,
        cno: u32,
        width: u32,
    ) -> bool {
        if range >= RANGES as u32 || cno == 0 || cno > 3 {
            return false;
        }
        let r = range as usize;
        let pointer = self.pointers[r];
        let idx = |i: u32| {
            (MAXLENG
                .wrapping_sub(width)
                .wrapping_add(1)
                .wrapping_add(pointer)
                .wrapping_add(i)
                % MAXLENG) as usize
        };
        let src = if self.is_direct_stat(ty) {
            if cno != 1 {
                return false;
            }
            ty + DIRECT_START
        } else if self.is_extended_stat(ty) {
            let e = &self.estatdefs[(ty - EXTENDED_START) as usize];
            match cno {
                1 => e.c1src,
                2 => e.c2src,
                _ => e.c3src,
            }
        } else {
            return false;
        };
        if self.def_is_direct(src) {
            let tab = buf.get_or_insert_with(|| vec![0; MAXLENG as usize]);
            let s = &self.series[(src - DIRECT_START) as usize][r];
            for i in 0..width {
                tab[i as usize] = s[idx(i)];
            }
            true
        } else if self.def_is_calc(src) {
            let start = self.calcstart[(src - CALC_START) as usize];
            let tab = buf.get_or_insert_with(|| vec![0; MAXLENG as usize]);
            for i in 0..width {
                tab[i as usize] = self.eval(start, r, idx(i));
            }
            true
        } else {
            false
        }
    }

    /// charts_statid_converter: numeric `type*10+range`, or a 4-char id
    /// whose letter case selects the range (lowercase bit per byte).
    pub(super) fn statid_converter(&self, number: u32) -> (u32, u32) {
        if number < 0x1000000 {
            return (number / 10, number % 10);
        }
        let rmask = number & 0x2020_2020;
        let rmask =
            ((rmask >> 5) & 1) | ((rmask >> 12) & 2) | ((rmask >> 19) & 4) | ((rmask >> 26) & 8);
        let number = number & 0xDFDF_DFDF;
        if let Some(i) = self.statdefs.iter().position(|s| s.statid & 0xDFDF_DFDF == number) {
            return (i as u32, rmask);
        }
        if let Some(i) = self.estatdefs.iter().position(|s| s.statid & 0xDFDF_DFDF == number) {
            return (EXTENDED_START + i as u32, rmask);
        }
        (u32::MAX, u32::MAX)
    }

    fn data_multiplier(&self, ty: u32, range: u32) -> f64 {
        let mut ret = 1.0;
        let Some(p) = self.def_params(ty) else {
            return ret;
        };
        if p.mode == MODE_ADD {
            match range {
                1 => ret /= 6.0,
                2 => ret /= 30.0,
                3 => ret /= 1440.0,
                _ => {}
            }
        }
        ret *= p.multiplier as f64;
        ret /= p.divisor as f64;
        match p.scale {
            SCALE_MICRO => ret /= 1_000_000.0,
            SCALE_MILI => ret /= 1000.0,
            SCALE_KILO => ret *= 1000.0,
            SCALE_MEGA => ret *= 1_000_000.0,
            SCALE_GIGA => ret *= 1_000_000_000.0,
            _ => {}
        }
        ret
    }

    /// charts_getdata: (timestamp, seconds per sample, MAXLENG values with
    /// -1.0 for "no data"), or None for an unknown id.
    pub fn getdata(&self, number: u32) -> Option<(u32, u32, Vec<f64>)> {
        let (ty, range) = self.statid_converter(number);
        if range >= RANGES as u32 || !(self.is_direct_stat(ty) || self.is_extended_stat(ty)) {
            return None;
        }
        let mul = self.data_multiplier(ty, range);
        let mut tabs: [Option<Vec<u64>>; 3] = [None, None, None];
        let mut c = 0;
        for (k, tab) in tabs.iter_mut().enumerate() {
            if self.filltab(tab, range, ty, k as u32 + 1, MAXLENG) {
                c = k + 1;
            }
        }
        let rsec = RANGE_SECONDS[range as usize];
        let ts = self.timepoint[range as usize].wrapping_mul(rsec);
        let data = (0..MAXLENG as usize)
            .map(|i| {
                let mut d: u64 = 0;
                let mut nd = true;
                // a missing lower table (C: NULL dereference) reads as no data
                for tab in tabs[..c].iter().flatten() {
                    if tab[i] != NODATA {
                        d = d.wrapping_add(tab[i]);
                        nd = false;
                    }
                }
                if nd { -1.0 } else { d as f64 * mul }
            })
            .collect();
        Some((ts, rsec, data))
    }

    /// charts_makedata: `out == None` returns the required size.
    pub fn makedata(&self, out: Option<&mut [u8]>, number: u32, maxentries: u32, multimode: bool) -> u32 {
        let (chtype, chrange) = self.statid_converter(number);
        let maxentries = maxentries.min(MAXLENG);
        if multimode {
            let scnt = self.seriescnt(chtype);
            if scnt == 0 {
                return 0;
            }
            let (from, to) = if chrange == 9 {
                (0u32, RANGES as u32)
            } else if chrange < RANGES as u32 {
                (chrange, chrange + 1)
            } else {
                return 0;
            };
            let Some(buf) = out else {
                return 8 + (to - from) * (13 + scnt as u32 * maxentries * 8);
            };
            let p = self.def_params(chtype).unwrap_or(DefParams {
                mode: MODE_ADD + 1,
                percent: 0,
                scale: SCALE_NONE,
                multiplier: 1,
                divisor: 1,
            });
            let additive = p.mode == MODE_ADD;
            let mut w = Put::new(buf);
            w.u8((to - from) as u8);
            w.u8(scnt);
            w.u32(maxentries);
            w.u8(p.percent);
            w.u8(p.scale);
            let mut tab = None;
            for r in from..to {
                w.u8(r as u8);
                w.u32(self.timepoint[r as usize].wrapping_mul(RANGE_SECONDS[r as usize]));
                let mut rdiv = p.divisor as u32;
                if additive {
                    rdiv = rdiv.wrapping_mul([1, 6, 30, 1440][r as usize]);
                }
                w.u32(p.multiplier as u32);
                w.u32(rdiv);
                for i in 0..scnt {
                    if self.filltab(&mut tab, r, chtype, (scnt - i) as u32, maxentries) {
                        let t = tab.as_ref().expect("filled");
                        for j in 0..maxentries {
                            w.u64(t[(maxentries - 1 - j) as usize]);
                        }
                    } else {
                        for _ in 0..maxentries {
                            w.u64(0);
                        }
                    }
                }
            }
            return 0;
        }
        let valid = chrange < RANGES as u32 && self.is_direct_stat(chtype);
        let Some(buf) = out else {
            return if valid { maxentries * 8 + 8 } else { 0 };
        };
        if valid {
            let r = chrange as usize;
            let tab = &self.series[chtype as usize][r];
            let mut j = (self.pointers[r] % MAXLENG) as usize;
            let mut w = Put::new(buf);
            w.u32(self.timepoint[r].wrapping_mul(RANGE_SECONDS[r]));
            w.u32(maxentries);
            for _ in 0..maxentries {
                w.u64(tab[j]);
                j = if j > 0 { j - 1 } else { MAXLENG as usize - 1 };
            }
        }
        0
    }

    /// charts_monotonic_data: `out == None` returns the required size.
    pub fn monotonic_data(&self, out: Option<&mut [u8]>) -> u32 {
        let Some(buf) = out else {
            return 2 + 8 * self.statcnt();
        };
        let mut w = Put::new(buf);
        w.u16(self.statcnt() as u16);
        for &m in &self.monotonic {
            w.u64(m);
        }
        0
    }
}
