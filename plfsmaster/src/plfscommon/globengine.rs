//! Glob pattern engine with a 16-entry cache — port of mfscommon/globengine.c.
//!
//! Used by plfsmaster (patterns.c storage-class patterns; filesystem.c
//! name-glob listings). Parsing and matching are safe Rust in `imp`,
//! mirroring the C algorithm step by step — including its quirks:
//! - `\x` escapes apply outside brackets only; an unclosed `[` is literal;
//! - consecutive `*` collapse; `[!..]` negates; `a-b` ranges;
//! - the middle-subpattern advance uses subpattern 0's length
//!   (`name += pos+p->sptab[0].leng`), a C quirk kept as is.
//!
//! Two deliberate deviations where C has undefined behavior, both
//! documented in the rewrite plan:
//! - a range ending in byte 0xFF (`[a-\xff]`) loops forever in C (uint8_t
//!   counter); here it terminates with the range filled up to 0xFF;
//! - when that middle advance overshoots the remaining name, C reads past
//!   the name buffer; here the name simply does not match.
//!
//! The cache lives in the plfsmaster lib because `glob_cache_init`
//! registers its destructor with the daemon's `main_destruct_register`.

use core::ffi::{c_char, c_int, c_void};
use std::sync::{Mutex, MutexGuard};

use plfscommon::clocks::monotonic_seconds;

pub use imp::Pattern;

const GLOB_CACHE_SIZE: usize = 16;

#[deny(unsafe_code)]
mod imp {
    enum Atom {
        Str(Vec<u8>),
        QMark,
        Range([u32; 8]),
    }

    enum Token {
        Star,
        Atom(Atom),
    }

    struct Sub {
        atoms: Vec<Atom>,
        /// C `uint8_t leng`.
        leng: u8,
    }

    pub struct Pattern {
        subs: Vec<Sub>,
        minleng: u32,
        first_asterisk: bool,
        last_asterisk: bool,
    }

    fn at(s: &[u8], i: usize) -> u8 {
        s.get(i).copied().unwrap_or(0)
    }

    fn set_bit(bits: &mut [u32; 8], c: u8, neg: bool) {
        let mask = 1u32 << (c & 0x1F);
        if neg {
            bits[(c >> 5) as usize] &= !mask;
        } else {
            bits[(c >> 5) as usize] |= mask;
        }
    }

    /// C `parse_range` over the bytes between `[` and `]`.
    fn parse_range(s: &[u8]) -> [u32; 8] {
        let mut k = 0;
        let neg = s.first() == Some(&b'!');
        if neg {
            k = 1;
        }
        let mut bits = [if neg { u32::MAX } else { 0 }; 8];
        while k < s.len() {
            if k + 2 < s.len() && s[k + 1] == b'-' {
                for c in s[k]..=s[k + 2] {
                    set_bit(&mut bits, c, neg);
                }
                k += 3;
            } else {
                set_bit(&mut bits, s[k], neg);
                k += 1;
            }
        }
        bits
    }

    /// C `unescape_string`: literal run starting at `start`, returns the
    /// unescaped bytes and the end index.
    fn unescape(g: &[u8], start: usize) -> (Vec<u8>, usize) {
        let mut out = Vec::new();
        let mut r = start;
        while at(g, r) != 0 {
            let c = at(g, r);
            if c == b'\\' {
                if at(g, r + 1) != 0 {
                    out.push(at(g, r + 1));
                    r += 2;
                } else {
                    out.push(c);
                    r += 1;
                }
            } else if c == b'[' || c == b'*' || c == b'?' {
                break;
            } else {
                out.push(c);
                r += 1;
            }
        }
        (out, r)
    }

    /// C `pattern_to_atoms_list` (input ends at the first NUL).
    fn tokenize(g: &[u8]) -> Vec<Token> {
        let mut toks = Vec::new();
        let mut last_asterisk = false;
        let mut p = 0usize;
        loop {
            let c = at(g, p);
            p += 1;
            if c == 0 {
                break;
            }
            match c {
                b'*' => {
                    if !last_asterisk {
                        toks.push(Token::Star);
                        last_asterisk = true;
                    }
                    continue;
                }
                b'?' => {
                    toks.push(Token::Atom(Atom::QMark));
                    last_asterisk = false;
                    continue;
                }
                b'[' => {
                    let mut r = p;
                    while at(g, r) != 0 && at(g, r) != b']' {
                        r += 1;
                    }
                    if at(g, r) == b']' {
                        toks.push(Token::Atom(Atom::Range(parse_range(&g[p..r]))));
                        p = r + 1;
                        last_asterisk = false;
                        continue;
                    }
                }
                _ => {}
            }
            let s = if c == b'[' {
                let (rest, end) = unescape(g, p);
                p = end;
                let mut s = vec![b'['];
                s.extend_from_slice(&rest);
                s
            } else {
                let (s, end) = unescape(g, p - 1);
                p = end;
                s
            };
            toks.push(Token::Atom(Atom::Str(s)));
            last_asterisk = false;
        }
        toks
    }

    impl Pattern {
        /// C `glob_new` (pattern bytes up to the first NUL).
        pub fn new(globstr: &[u8]) -> Self {
            let mut toks = tokenize(globstr).into_iter().peekable();
            let mut p = Pattern {
                subs: Vec::new(),
                minleng: 0,
                first_asterisk: false,
                last_asterisk: false,
            };
            if toks.peek().is_none() {
                return p;
            }
            if matches!(toks.peek(), Some(Token::Star)) {
                toks.next();
                p.first_asterisk = true;
                if toks.peek().is_none() {
                    p.last_asterisk = true;
                    return p;
                }
            }
            let mut cur = Sub { atoms: Vec::new(), leng: 0 };
            while let Some(t) = toks.next() {
                match t {
                    Token::Star => {
                        if toks.peek().is_some() {
                            p.subs.push(std::mem::replace(
                                &mut cur,
                                Sub { atoms: Vec::new(), leng: 0 },
                            ));
                        } else {
                            p.last_asterisk = true;
                        }
                    }
                    Token::Atom(a) => {
                        let l = match &a {
                            Atom::Str(s) => s.len() as u8,
                            _ => 1,
                        };
                        cur.leng = cur.leng.wrapping_add(l);
                        cur.atoms.push(a);
                    }
                }
            }
            p.subs.push(cur);
            p.minleng = p.subs.iter().map(|s| s.leng as u32).sum();
            p
        }

        /// C `glob_match` / `pattern_match`.
        pub fn matches(&self, name: &[u8]) -> bool {
            let nleng = name.len();
            if (nleng as u32) < self.minleng {
                return false;
            }
            let subs = &self.subs;
            let (first, last) = (self.first_asterisk, self.last_asterisk);
            if subs.is_empty() {
                return first || last;
            }
            if subs.len() == 1 {
                let s = &subs[0];
                let sl = s.leng as usize;
                if sl > nleng {
                    return false;
                }
                if first && last {
                    return closest(s, name).is_some();
                }
                if last {
                    return exact(s, name);
                }
                if first {
                    return exact(s, &name[nleng - sl..]);
                }
                if sl != nleng {
                    return false;
                }
                return exact(s, name);
            }
            let mut cur = name;
            let adv0 = subs[0].leng as usize;
            if first {
                let Some(pos) = closest(&subs[0], cur) else { return false };
                cur = &cur[pos + adv0..];
            } else {
                if !exact(&subs[0], cur) {
                    return false;
                }
                cur = &cur[adv0..];
            }
            let lasti = subs.len() - 1;
            for s in &subs[1..lasti] {
                let Some(pos) = closest(s, cur) else { return false };
                // C advances by subpattern 0's length (quirk kept)
                let adv = pos + adv0;
                if adv > cur.len() {
                    return false; // C: reads past the name (UB)
                }
                cur = &cur[adv..];
            }
            let s = &subs[lasti];
            if last {
                closest(s, cur).is_some()
            } else {
                let sl = s.leng as usize;
                if cur.len() < sl {
                    return false;
                }
                exact(s, &cur[cur.len() - sl..])
            }
        }
    }

    /// C `subpattern_match_exact`: atoms match a prefix of `name`.
    fn exact(sp: &Sub, name: &[u8]) -> bool {
        let mut n = name;
        for a in &sp.atoms {
            match a {
                Atom::Str(s) => {
                    if s.len() > n.len() || &n[..s.len()] != s.as_slice() {
                        return false;
                    }
                    n = &n[s.len()..];
                }
                Atom::QMark => {
                    if n.is_empty() {
                        return false;
                    }
                    n = &n[1..];
                }
                Atom::Range(bits) => {
                    let Some(&c) = n.first() else { return false };
                    if bits[(c >> 5) as usize] & (1u32 << (c & 0x1F)) == 0 {
                        return false;
                    }
                    n = &n[1..];
                }
            }
        }
        true
    }

    /// C `subpattern_closest_match`: first offset where `sp` matches.
    fn closest(sp: &Sub, name: &[u8]) -> Option<usize> {
        let sl = sp.leng as usize;
        let mut pos = 0;
        while pos <= name.len() && sl <= name.len() - pos {
            if exact(sp, &name[pos..]) {
                return Some(pos);
            }
            pos += 1;
        }
        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn m(g: &str, n: &str) -> bool {
            Pattern::new(g.as_bytes()).matches(n.as_bytes())
        }

        #[test]
        fn basic_globs() {
            assert!(m("*", ""));
            assert!(m("*", "abc"));
            assert!(!m("", ""));
            assert!(m("abc", "abc"));
            assert!(!m("abc", "abcd"));
            assert!(m("a*", "abcd"));
            assert!(m("*d", "abcd"));
            assert!(!m("*d", "abce"));
            assert!(m("*bc*", "abcd"));
            assert!(m("a?c", "abc"));
            assert!(!m("a?c", "ac"));
            assert!(m("[a-c]x", "bx"));
            assert!(!m("[!a-c]x", "bx"));
            assert!(m("[!a-c]x", "dx"));
            assert!(m("a**b", "ab"));
            assert!(m("a*b*c", "aXbYc"));
            assert!(!m("a*b*c", "aXbY"));
            assert!(m("*.txt", "notes.txt"));
        }

        #[test]
        fn escapes_and_unclosed_brackets() {
            assert!(m("a\\*b", "a*b"));
            assert!(!m("a\\*b", "axb"));
            assert!(m("[ab", "[ab"));
            assert!(m("x\\", "x\\"));
            assert!(m("[\\]]", "\\]")); // bracket body is not unescaped
        }

        #[test]
        fn middle_advance_quirk_matches_c() {
            // "a*bc*c": C advances by len("a")=1 after finding "bc", so the
            // final "c" may reuse the 'c' of "bc".
            assert!(!m("a*bc*c", "abc")); // minleng 4 > 3
            assert!(m("a*bcd*d", "abcdd"));
            // overshoot (C UB) → no match: sptab[0]="abc", middle "x"
            assert!(!m("abc*x*y", "abcxy"));
        }

        #[test]
        fn range_to_ff_terminates() {
            let p = Pattern::new(b"[\xf0-\xff]");
            assert!(p.matches(b"\xff"));
            assert!(p.matches(b"\xf0"));
            assert!(!p.matches(b"\xef"));
        }
    }
}

// ---------------------------------------------------------------------------
// C ABI boundary (globengine.h). Handles are `Pattern` addresses.
// ---------------------------------------------------------------------------

unsafe extern "C" {
    // plfsmaster bin (main.c): destructor registry.
    fn main_destruct_register_fname(fun: Option<unsafe extern "C" fn()>, fname: *const c_char);
}

struct CacheEntry {
    glob: Option<Box<Pattern>>,
    gname: Vec<u8>,
    mt: f64,
}

static CACHE: Mutex<Vec<CacheEntry>> = Mutex::new(Vec::new());

fn cache() -> MutexGuard<'static, Vec<CacheEntry>> {
    let mut c = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if c.is_empty() {
        c.extend((0..GLOB_CACHE_SIZE).map(|_| CacheEntry { glob: None, gname: Vec::new(), mt: 0.0 }));
    }
    c
}

/// Pattern bytes as C `glob_new` sees them (up to the first NUL).
fn until_nul(b: &[u8]) -> &[u8] {
    &b[..b.iter().position(|&c| c == 0).unwrap_or(b.len())]
}

/// # Safety
/// `globstr` must be a valid NUL-terminated string. Free with `glob_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glob_new(globstr: *const u8) -> *mut c_void {
    // SAFETY: per fn contract.
    let g = unsafe { core::ffi::CStr::from_ptr(globstr as *const c_char) }.to_bytes();
    Box::into_raw(Box::new(Pattern::new(g))) as *mut c_void
}

/// # Safety
/// `glob` from `glob_new` (not from the cache); invalid afterwards.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glob_free(glob: *mut c_void) {
    // SAFETY: per fn contract this is the last use of the Box.
    drop(unsafe { Box::from_raw(glob as *mut Pattern) });
}

/// # Safety
/// `glob` a live pattern handle; `name` readable for `nleng` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glob_match(glob: *mut c_void, name: *const u8, nleng: u8) -> u8 {
    // SAFETY: per fn contract.
    let (p, n) = unsafe {
        let n = if nleng == 0 { &[][..] } else { std::slice::from_raw_parts(name, nleng as usize) };
        (&*(glob as *const Pattern), n)
    };
    p.matches(n) as u8
}

/// # Safety
/// `gname` readable for `gnleng` bytes. The handle stays valid until the
/// entry is evicted (16 newer distinct patterns) or the cache terminates.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn glob_cache_get(gnleng: u8, gname: *const u8) -> *mut c_void {
    // SAFETY: per fn contract.
    let key = unsafe {
        if gnleng == 0 { &[][..] } else { std::slice::from_raw_parts(gname, gnleng as usize) }
    };
    let mut c = cache();
    let mut j = 0usize;
    for i in 0..GLOB_CACHE_SIZE {
        if let Some(g) = &c[i].glob {
            if c[i].gname == key {
                let ptr = &**g as *const Pattern as *mut c_void;
                c[i].mt = monotonic_seconds();
                return ptr;
            }
        }
        if c[i].glob.is_none() || c[i].mt < c[j].mt {
            j = i;
        }
    }
    let glob = Box::new(Pattern::new(until_nul(key)));
    let ptr = &*glob as *const Pattern as *mut c_void;
    c[j] = CacheEntry { glob: Some(glob), gname: key.to_vec(), mt: monotonic_seconds() };
    ptr
}

/// C: `glob_cache_term` (registered as a main destructor).
#[unsafe(no_mangle)]
pub extern "C" fn glob_cache_term() {
    for e in cache().iter_mut() {
        *e = CacheEntry { glob: None, gname: Vec::new(), mt: 0.0 };
    }
}

/// C: `glob_cache_init`.
#[unsafe(no_mangle)]
pub extern "C" fn glob_cache_init() -> c_int {
    glob_cache_term();
    // SAFETY: main_destruct_register_fname stores the fn pointer and the
    // static name for the daemon's shutdown sequence.
    unsafe {
        main_destruct_register_fname(Some(glob_cache_term_c), c"glob_cache_term".as_ptr());
    }
    0
}

unsafe extern "C" fn glob_cache_term_c() {
    glob_cache_term()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_hits_and_lru_eviction() {
        // SAFETY: literal buffers; handles used while cached.
        unsafe {
            let a = glob_cache_get(3, b"*.c".as_ptr());
            assert_eq!(glob_cache_get(3, b"*.c".as_ptr()), a);
            assert_eq!(glob_match(a, b"x.c".as_ptr(), 3), 1);
            assert_eq!(glob_match(a, b"x.h".as_ptr(), 3), 0);
            let e = glob_cache_get(0, core::ptr::null());
            assert_eq!(glob_match(e, b"".as_ptr(), 0), 0);
            // embedded NUL: cache key is all bytes, pattern stops at NUL
            let z = glob_cache_get(3, b"a\0b".as_ptr());
            assert_eq!(glob_match(z, b"a".as_ptr(), 1), 1);
            let d = glob_new(c"a?".as_ptr() as *const u8);
            assert_eq!(glob_match(d, b"ab".as_ptr(), 2), 1);
            glob_free(d);
        }
        glob_cache_term();
    }
}
