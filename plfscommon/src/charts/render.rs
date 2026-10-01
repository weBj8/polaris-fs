//! Chart rendering of mfscommon/charts.c: `charts_makechart` (bars, axes,
//! labels), `charts_fixmax` (y scale), the raw scanline buffer, deflate and
//! PNG CRC patching.
//!
//! Float conversions mirror what gcc emits on x86-64 for the C casts
//! (`cvttsd2si` semantics, including its out-of-range result), so charts
//! with degenerate scales (dmax = 0 / inf) render the same pixels.
#![deny(unsafe_code)]

use flate2::{FlushCompress, Status};

use super::imp::{
    Charts, MAXHEIGHT, MAXLENG, MAXXSIZE, MAXYSIZE, MINHEIGHT, MINLENG, MODE_ADD, NODATA, RANGES,
    XADD, XPOS, YADD, YPOS,
};
use super::tables::{FONT, PNG_1X1, PNG_HEADER, PNG_TAILER};
use crate::mfslog::{MFSLOG_SYSLOG, MFSLOG_WARNING, log_bytes};

const COLOR_BKG: u8 = 1;
const COLOR_AXIS: u8 = 2;
const COLOR_AUX: u8 = 3;
const COLOR_TEXT: u8 = 4;
const COLOR_NODATA: u8 = 5;
const COLOR_DATA_BEGIN: u32 = 50;
const COLOR_DATA_RANGE: u32 = 50;

const FDOT: u8 = 10;
const COLON: u8 = 11;
const KILO: u8 = 12;
const MEGA: u8 = 13;
const GIGA: u8 = 14;
const TERA: u8 = 15;
const PETA: u8 = 16;
const EXA: u8 = 17;
const ZETTA: u8 = 18;
const YOTTA: u8 = 19;
const MILI: u8 = 20;
const MICRO: u8 = 21;
const PERCENT: u8 = 22;
const SPACE: u8 = 23;
const SQUARE: u8 = 24;

/// x86-64 `cvttsd2si` (64-bit): truncation, `i64::MIN` when out of range.
fn cvtt64(x: f64) -> i64 {
    if x.is_nan() || !(-9223372036854775808.0..9223372036854775808.0).contains(&x) {
        i64::MIN
    } else {
        x as i64
    }
}

/// gcc's double -> uint64_t sequence (split at 2^63, then cvttsd2si).
pub(super) fn c_f64_to_u64(x: f64) -> u64 {
    if x >= 9223372036854775808.0 {
        (cvtt64(x - 9223372036854775808.0) as u64) ^ (1 << 63)
    } else {
        cvtt64(x) as u64
    }
}

/// gcc's double -> uint8_t: 32-bit `cvttsd2si`, low byte kept.
pub(super) fn c_f64_to_u8(x: f64) -> u8 {
    let v = if x.is_nan() || !(-2147483648.0..2147483648.0).contains(&x) {
        i32::MIN
    } else {
        x as i32
    };
    v as u8
}

/// Days in month (charts.c getmonleng); 0 for an invalid month.
pub(super) fn getmonleng(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 4 != 0 {
                28
            } else if year % 100 != 0 {
                29
            } else if year % 400 != 0 {
                28
            } else {
                29
            }
        }
        _ => 0,
    }
}

/// charts_fixmax: rounds `max` up to a label-friendly scale over `ypts`
/// grid lines. Updates `scale` (u8, wraps like C), sets `mode` (digits
/// after the dot) and `base` (label step), returns the chart top value.
pub(super) fn fixmax(max: u64, ypts: u32, scale: &mut u8, mode: &mut u8, base: &mut u16) -> f64 {
    let max = if max == 0 { 1 } else { max };
    if max <= 9 {
        *base = ((max * 100 + ypts as u64 - 1) / ypts as u64) as u16;
        if (*base as u32).wrapping_mul(ypts) < 1000 {
            *mode = 2;
            return ((*base as u32 * ypts) / 100) as f64;
        }
    }
    if max <= 99 {
        *base = ((max * 10 + ypts as u64 - 1) / ypts as u64) as u16;
        if (*base as u32).wrapping_mul(ypts) < 1000 {
            *mode = 1;
            return ((*base as u32 * ypts) / 10) as f64;
        }
    }
    let mut cpmax: u64 = 999;
    let mut cmode: u8 = 0;
    let mut ascale: u8 = 0;
    let mut factor: u64 = ypts as u64;
    loop {
        if max <= cpmax {
            *base = (max.wrapping_add(factor).wrapping_sub(1) / factor) as u16;
            if (*base as u32).wrapping_mul(ypts) < 1000 {
                *mode = cmode;
                *scale = scale.wrapping_add(ascale);
                return (*base as u64).wrapping_mul(factor) as f64;
            }
        }
        if cmode == 0 {
            cmode = 2;
            ascale = ascale.wrapping_add(1);
        } else {
            cmode -= 1;
        }
        factor = factor.wrapping_mul(10);
        if cpmax.wrapping_mul(10) > cpmax {
            cpmax *= 10;
        } else {
            let f10 = factor / 10;
            *base = if max.wrapping_add(9) < max {
                ((max / 10).wrapping_add(f10).wrapping_sub(1) / f10) as u16
            } else {
                ((max + 9) / 10).wrapping_add(f10).wrapping_sub(1).wrapping_div(f10) as u16
            };
            *mode = cmode;
            *scale = scale.wrapping_add(ascale);
            return *base as f64 * factor as f64;
        }
    }
}

/// Pixel write at signed chart coordinates (C indexes with the same math).
fn put(chart: &mut [u8], x: i64, y: i64, c: u8) {
    chart[(MAXXSIZE as i64 * y + x) as usize] = c;
}

#[allow(clippy::too_many_arguments)]
fn puttext(chart: &mut [u8], posx: i32, posy: i32, color: u8, text: &[u8], clip: (i32, i32, i32, i32)) {
    let (minx, maxx, miny, maxy) = clip;
    for (i, &ch) in text.iter().enumerate() {
        let px = (i as i32).wrapping_mul(6).wrapping_add(posx);
        let fp = ch.min(SQUARE) as usize;
        for fy in 0..9i32 {
            let mut fbits = FONT[fp][fy as usize];
            if fbits == 0 {
                continue;
            }
            for fx in 0..5i32 {
                let x = px.wrapping_add(fx);
                let y = posy.wrapping_add(fy);
                if fbits & 0x10 != 0 && x >= minx && x <= maxx && y >= miny && y <= maxy {
                    put(chart, x as i64, y as i64, color);
                }
                fbits <<= 1;
            }
        }
    }
}

const JTAB: [u8; 11] = [MICRO, MILI, SPACE, KILO, MEGA, GIGA, TERA, PETA, EXA, ZETTA, YOTTA];

impl Charts {
    /// charts_makechart into `self.chart` (width/height without margins).
    fn makechart(&mut self, ty: u32, range: u32, width: u32, height: u32) {
        let mut chart = std::mem::take(&mut self.chart);
        chart.fill(0);
        let mut tabs: [Option<Vec<u64>>; 3] = [None, None, None];
        let mut colors = 0usize;
        for (k, tab) in tabs.iter_mut().enumerate() {
            if self.filltab(tab, range, ty, k as u32 + 1, width) {
                colors = k + 1;
            }
        }
        // C dereferences a NULL lower table when only a higher one filled;
        // here a missing table reads as "no data".
        let sample = |tabs: &[Option<Vec<u64>>; 3], k: usize, i: usize| -> Option<u64> {
            if colors > k {
                tabs[k].as_ref().map(|t| t[i]).filter(|&v| v != NODATA)
            } else {
                None
            }
        };
        let mut max: u64 = 0;
        for i in 0..width as usize {
            let mut d: u64 = 0;
            for k in 0..3 {
                if let Some(v) = sample(&tabs, k, i) {
                    d = d.wrapping_add(v);
                }
            }
            if d > max {
                max = d;
            }
        }
        let mut scale: u8;
        if max > 1_000_000_000_000_000_000 {
            for k in 0..colors {
                if let Some(t) = tabs[k].as_mut() {
                    for v in t[..width as usize].iter_mut().filter(|v| **v != NODATA) {
                        *v /= 1000;
                    }
                }
            }
            max /= 1000;
            scale = 1;
        } else {
            scale = 0;
        }
        let params = self.def_params(ty);
        let additive = params.is_some_and(|p| p.mode == MODE_ADD);
        if additive {
            match range {
                1 => max = max.wrapping_add(5) / 6,
                2 => max = max.wrapping_add(29) / 30,
                3 => max = max.wrapping_add(1439) / (24 * 60),
                _ => {}
            }
        }
        let mut percent = 0u8;
        if let Some(p) = params {
            scale = scale.wrapping_add(p.scale);
            percent = p.percent;
            max = max.wrapping_mul(p.multiplier as u64);
            max /= p.divisor as u64;
        }
        let ypts = height / 20;
        let mut mode = 0u8;
        let mut base = 0u16;
        let mut dmax = fixmax(max, ypts, &mut scale, &mut mode, &mut base);
        if let Some(p) = params {
            dmax *= p.divisor as f64;
            dmax /= p.multiplier as f64;
        }
        if additive {
            match range {
                1 => dmax *= 6.0,
                2 => dmax *= 30.0,
                3 => dmax *= (24 * 60) as f64,
                _ => {}
            }
        }

        let h64 = height as u64;
        let px = |chart: &mut Vec<u8>, i: i64, j: i64, c: u8| put(chart, i + XPOS as i64, j + YPOS as i64, c);
        for i in 0..width as usize {
            let mut any = false;
            let mut layer = |k: usize, below: u64| match sample(&tabs, k, i) {
                Some(v) => {
                    any = true;
                    below.wrapping_add(v)
                }
                None => below,
            };
            let mut c3d = layer(2, 0);
            let mut c2d = layer(1, c3d);
            let mut c1d = layer(0, c2d);
            let i64i = i as i64;
            if !any {
                for j in 0..height as i64 {
                    let c = if (j + i64i) % 3 != 0 { COLOR_BKG } else { COLOR_NODATA };
                    px(&mut chart, i64i, j, c);
                }
                continue;
            }
            let scale_to = |v: u64, div: f64| c_f64_to_u64(v.wrapping_mul(h64) as f64 / div);
            if c1d > 281_474_976_710_656 {
                c1d /= 65536;
                c2d /= 65536;
                c3d /= 65536;
                let div = dmax / 65536.0;
                c1d = scale_to(c1d, div);
                c2d = scale_to(c2d, div);
                c3d = scale_to(c3d, div);
            } else {
                c1d = scale_to(c1d, dmax);
                c2d = scale_to(c2d, dmax);
                c3d = scale_to(c3d, dmax);
            }
            let mut j: i64 = 0;
            while h64 >= c1d.wrapping_add(j as u64) {
                px(&mut chart, i64i, j, COLOR_BKG);
                j += 1;
            }
            // (height-1-j) is unsigned arithmetic in C
            let rem = |j: i64| (height.wrapping_sub(1).wrapping_sub(j as u32)) as f64;
            let remu = |j: i64| height.wrapping_sub(1).wrapping_sub(j as u32);
            while h64 >= c2d.wrapping_add(j as u64) {
                let c = match colors {
                    1 => Some((COLOR_DATA_BEGIN + remu(j).wrapping_mul(3 * COLOR_DATA_RANGE) / height) as u8),
                    2 => Some(c_f64_to_u8(
                        COLOR_DATA_BEGIN as f64 + rem(j) * 1.5 * COLOR_DATA_RANGE as f64 / height as f64,
                    )),
                    3 => Some((COLOR_DATA_BEGIN + remu(j).wrapping_mul(COLOR_DATA_RANGE) / height) as u8),
                    _ => None,
                };
                if let Some(c) = c {
                    px(&mut chart, i64i, j, c);
                }
                j += 1;
            }
            while h64 >= c3d.wrapping_add(j as u64) {
                let c = match colors {
                    2 => Some(c_f64_to_u8(
                        COLOR_DATA_BEGIN as f64
                            + 1.5 * COLOR_DATA_RANGE as f64
                            + rem(j) * 1.5 * COLOR_DATA_RANGE as f64 / height as f64,
                    )),
                    3 => Some(
                        (COLOR_DATA_BEGIN + COLOR_DATA_RANGE + remu(j).wrapping_mul(COLOR_DATA_RANGE) / height)
                            as u8,
                    ),
                    _ => None,
                };
                if let Some(c) = c {
                    px(&mut chart, i64i, j, c);
                }
                j += 1;
            }
            while (height as i32 as i64) > j {
                let c = COLOR_DATA_BEGIN + 2 * COLOR_DATA_RANGE + remu(j).wrapping_mul(COLOR_DATA_RANGE) / height;
                px(&mut chart, i64i, j, c as u8);
                j += 1;
            }
        }

        let (w, h) = (width as i64, height as i64);
        // axes
        for i in -3..w + 3 {
            px(&mut chart, i, h, COLOR_AXIS);
        }
        for i in -2..h + 5 {
            put(&mut chart, XPOS as i64 - 1, h - i + YPOS as i64, COLOR_AXIS);
            put(&mut chart, XPOS as i64 + w, h - i + YPOS as i64, COLOR_AXIS);
        }

        let label_y = YPOS + height as i32 + 4;
        let xclip = (XPOS, XPOS + width as i32 - 1, 0, MAXYSIZE as i32 - 1);
        let t = self.t;
        // x scale
        if range < 3 {
            let (xs, xoff, xbold, mut xh, mut xd, mut xm, mut xy);
            if range == 2 {
                xs = 12;
                xoff = t.lnghalfhour % 12;
                xbold = 4;
                xh = t.lnghalfhour / 12;
                xd = t.lngmday;
                xm = t.lngmonth;
                xy = t.lngyear;
            } else if range == 1 {
                xs = 10;
                xoff = t.medmin / 6;
                xbold = 6;
                xh = t.medhour;
                (xd, xm, xy) = (0, 0, 0);
            } else {
                xs = 60;
                xoff = t.shmin;
                xbold = 1;
                xh = t.shhour;
                (xd, xm, xy) = (0, 0, 0);
            }
            let mut i = width.wrapping_sub(xoff).wrapping_sub(1) as i32;
            while i >= 0 {
                let ys: i32;
                if xh % xbold == 0 {
                    ys = if (range == 0 && xh % 6 == 0) || (range == 1 && xh == 0) || (range == 2 && xd == 1) {
                        1
                    } else {
                        2
                    };
                    if range < 2 {
                        let text = [(xh / 10) as u8, (xh % 10) as u8, COLON, 0, 0];
                        puttext(&mut chart, XPOS + i - 14, label_y, COLOR_TEXT, &text, xclip);
                    } else {
                        let text = [(xm / 10) as u8, (xm % 10) as u8, FDOT, (xd / 10) as u8, (xd % 10) as u8];
                        puttext(&mut chart, XPOS + i + 10, label_y, COLOR_TEXT, &text, xclip);
                        xd = xd.wrapping_sub(1);
                        if xd == 0 {
                            xm = xm.wrapping_sub(1);
                            if xm == 0 {
                                xm = 12;
                                xy = xy.wrapping_sub(1);
                            }
                            xd = getmonleng(xy, xm);
                        }
                    }
                    px(&mut chart, i as i64, h + 1, COLOR_AXIS);
                    px(&mut chart, i as i64, h + 2, COLOR_AXIS);
                } else {
                    ys = 4;
                }
                let mut j = 0i32;
                while j < height as i32 {
                    if ys > 1 || j % 4 != 0 {
                        px(&mut chart, i as i64, j as i64, COLOR_AUX);
                    }
                    j += ys;
                }
                let wrap = if range < 2 { 23 } else { 3 };
                xh = if xh == 0 { wrap } else { xh - 1 };
                i -= xs;
            }
            if range == 2 {
                i = i.wrapping_sub(xs.wrapping_mul(xh as i32));
                let text = [(xm / 10) as u8, (xm % 10) as u8, FDOT, (xd / 10) as u8, (xd % 10) as u8];
                puttext(&mut chart, XPOS + i + 10, label_y, COLOR_TEXT, &text, xclip);
            }
        } else {
            let mut xy = t.lngyear;
            let mut xm = t.lngmonth;
            let mon_x = |i: i32, xy: u32, xm: u32| {
                (XPOS as u32)
                    .wrapping_add(i as u32)
                    .wrapping_add(getmonleng(xy, xm).wrapping_sub(11) / 2)
                    .wrapping_add(1) as i32
            };
            let mut i = width.wrapping_sub(t.lngmday) as i32;
            while i >= 0 {
                let text = [(xm / 10) as u8, (xm % 10) as u8];
                puttext(&mut chart, mon_x(i, xy, xm), label_y, COLOR_TEXT, &text, xclip);
                px(&mut chart, i as i64, h + 1, COLOR_AXIS);
                px(&mut chart, i as i64, h + 2, COLOR_AXIS);
                if xm != 1 {
                    for j in (0..height as i64).step_by(2) {
                        px(&mut chart, i as i64, j, COLOR_AUX);
                    }
                } else {
                    for j in (0..height as i64).filter(|j| j % 4 != 0) {
                        px(&mut chart, i as i64, j, COLOR_AUX);
                    }
                }
                xm = xm.wrapping_sub(1);
                if xm == 0 {
                    xm = 12;
                    xy = xy.wrapping_sub(1);
                }
                i = (i as u32).wrapping_sub(getmonleng(xy, xm)) as i32;
            }
            let text = [(xm / 10) as u8, (xm % 10) as u8];
            puttext(&mut chart, mon_x(i, xy, xm), label_y, COLOR_TEXT, &text, xclip);
        }

        // y scale
        let full = (0, MAXXSIZE as i32 - 1, 0, MAXYSIZE as i32 - 1);
        for i in 0..=ypts as i64 {
            let mut d: u64 = (base as i64 * i) as u64;
            let mut text = [0u8; 6];
            let mut j = 0usize;
            let mut push = |text: &mut [u8; 6], v: u8| {
                text[j] = v;
                j += 1;
            };
            match mode {
                0 => {
                    if d >= 10 {
                        if d >= 100 {
                            push(&mut text, (d / 100) as u8);
                            d %= 100;
                        }
                        push(&mut text, (d / 10) as u8);
                    }
                    push(&mut text, (d % 10) as u8);
                }
                1 => {
                    if d >= 100 {
                        push(&mut text, (d / 100) as u8);
                        d %= 100;
                    }
                    push(&mut text, (d / 10) as u8);
                    push(&mut text, FDOT);
                    push(&mut text, (d % 10) as u8);
                }
                2 => {
                    push(&mut text, (d / 100) as u8);
                    d %= 100;
                    push(&mut text, FDOT);
                    push(&mut text, (d / 10) as u8);
                    push(&mut text, (d % 10) as u8);
                }
                _ => {}
            }
            if scale < 11 {
                if JTAB[scale as usize] != SPACE {
                    push(&mut text, JTAB[scale as usize]);
                }
            } else {
                push(&mut text, SQUARE);
            }
            if percent != 0 {
                push(&mut text, PERCENT);
            }
            let row = h - 20 * i;
            puttext(
                &mut chart,
                XPOS - 4 - (j as i32 * 6),
                (YPOS as i64 + row - 3) as i32,
                COLOR_TEXT,
                &text[..j],
                full,
            );
            put(&mut chart, XPOS as i64 - 2, YPOS as i64 + row, COLOR_AXIS);
            put(&mut chart, XPOS as i64 - 3, YPOS as i64 + row, COLOR_AXIS);
            if i > 0 {
                for x in (1..w).step_by(2) {
                    px(&mut chart, x, row, COLOR_AUX);
                }
            }
        }
        self.chart = chart;
    }

    /// charts_make_png: renders and compresses; returns the PNG size.
    /// The image is held until `get_png` (C held its global lock between
    /// the two calls for the same reason).
    pub fn make_png(&mut self, number: u32, chartwidth: u32, chartheight: u32) -> u32 {
        self.compsize = 0;
        let (chtype, chrange) = self.statid_converter(number);
        if chrange >= RANGES as u32 || !(self.is_direct_stat(chtype) || self.is_extended_stat(chtype)) {
            return PNG_1X1.len() as u32;
        }
        let (mut w, mut h) = (chartwidth, chartheight);
        if w == 0 && h == 0 {
            w = 950 + XADD;
            h = 100 + YADD;
        }
        h = h.clamp(MINHEIGHT + YADD, MAXHEIGHT + YADD);
        h -= (h - YADD) % 20;
        w = w.clamp(MINLENG + XADD, MAXLENG + XADD);
        self.makechart(chtype, chrange, w - XADD, h - YADD);
        // chart_to_rawchart: filter byte 0 + row; bytes past the image keep
        // whatever earlier charts left (the whole buffer is compressed)
        for y in 0..h as usize {
            let o = y * (1 + w as usize);
            self.rawchart[o] = 0;
            self.rawchart[o + 1..o + 1 + w as usize].copy_from_slice(&self.chart[y * MAXXSIZE..y * MAXXSIZE + w as usize]);
        }
        self.png_header[16..20].copy_from_slice(&w.to_be_bytes());
        self.png_header[20..24].copy_from_slice(&h.to_be_bytes());
        let Some(z) = self.zstr.as_mut() else {
            return PNG_1X1.len() as u32;
        };
        z.reset();
        match z.compress(&self.rawchart, &mut self.compbuff, FlushCompress::Finish) {
            Ok(Status::StreamEnd) => {}
            _ => return PNG_1X1.len() as u32,
        }
        self.compsize = z.total_out() as u32;
        (PNG_HEADER.len() + self.compsize as usize + PNG_TAILER.len()) as u32
    }

    /// Size `make_png` reported for the image currently held.
    pub fn png_len(&self) -> u32 {
        if self.compsize == 0 {
            PNG_1X1.len() as u32
        } else {
            (PNG_HEADER.len() + self.compsize as usize + PNG_TAILER.len()) as u32
        }
    }

    /// charts_get_png: `buf` is exactly the size `make_png` returned.
    pub fn get_png(&mut self, buf: &mut [u8]) {
        if self.compsize == 0 {
            buf[..PNG_1X1.len()].copy_from_slice(&PNG_1X1);
        } else {
            let hl = PNG_HEADER.len();
            let cs = self.compsize as usize;
            buf[..hl].copy_from_slice(&self.png_header);
            buf[hl - 8..hl - 4].copy_from_slice(&self.compsize.to_be_bytes());
            buf[hl..hl + cs].copy_from_slice(&self.compbuff[..cs]);
            buf[hl + cs..hl + cs + PNG_TAILER.len()].copy_from_slice(&PNG_TAILER);
            fill_crc(&mut buf[..hl + cs + PNG_TAILER.len()]);
        }
        self.compsize = 0;
    }
}

/// charts_fill_crc: walks the chunks after the signature and replaces each
/// `CRC#` placeholder with the CRC of chunk type + data.
fn fill_crc(buf: &mut [u8]) {
    let end = buf.len();
    let mut p = 8usize;
    while p + 4 <= end {
        let chleng = u32::from_be_bytes([buf[p], buf[p + 1], buf[p + 2], buf[p + 3]]) as usize;
        p += 4;
        if p + 8 + chleng <= end {
            let crc = crate::crc::crc32(0, &buf[p..p + chleng + 4]);
            p += chleng + 4;
            if &buf[p..p + 4] == b"CRC#" {
                buf[p..p + 4].copy_from_slice(&crc.to_be_bytes());
                p += 4;
            } else {
                log_bytes(MFSLOG_SYSLOG, MFSLOG_WARNING, b"charts: unexpected data in generated png stream");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f64_conversions_match_gcc_x86_64() {
        // (a, b, (uint64_t)(a / b)) produced by gcc -O2 on x86-64
        let cases: &[(u64, f64, u64)] = &[
            (0, 0.0, 9223372036854775808),
            (0, f64::NAN, 9223372036854775808),
            (1, 0.5, 2),
            (1, -1.0, 18446744073709551615),
            (1, 0.0, 0),
            (1, -0.0, 9223372036854775808),
            (7, -3.0, 18446744073709551614),
            (9223372036854775807, 1.0, 9223372036854775808),
            (9223372036854775807, -3.0, 15372286728091293184),
            (18446744073709551615, 1.0, 0),
            (18446744073709551615, -3.0, 12297829382473034752),
            (12345678901234567890, 1.0, 12345678901234567168),
            (12345678901234567890, 3.0, 4115226300411522560),
            (12345678901234567890, -1.0, 9223372036854775808),
            (12345678901234567890, f64::INFINITY, 0),
        ];
        for &(a, b, want) in cases {
            assert_eq!(c_f64_to_u64(a as f64 / b), want, "{a} / {b}");
        }
        assert_eq!(c_f64_to_u8(199.9), 199);
        assert_eq!(c_f64_to_u8(f64::NAN), 0);
        assert_eq!(c_f64_to_u8(300.0), 44);
    }

    #[test]
    fn monleng() {
        assert_eq!(getmonleng(2024, 2), 29);
        assert_eq!(getmonleng(1900, 2), 28);
        assert_eq!(getmonleng(2000, 2), 29);
        assert_eq!(getmonleng(2023, 4), 30);
        assert_eq!(getmonleng(2023, 0), 0);
    }
}
