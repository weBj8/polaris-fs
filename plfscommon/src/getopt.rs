//! glibc `getopt(3)` (short options only) in safe Rust.
//!
//! MooseFS daemons parse their command line with `getopt()`; glibc's
//! default PERMUTE ordering lets options follow non-options
//! (`mfsmaster start -f`) and reorders `argv` so non-options end up last.
//! That reordering is observable (it is the argv the process-title code
//! later measures), so this reproduces glibc's algorithm exactly,
//! including `exchange()` block rotation, `--` handling and
//! `POSIXLY_CORRECT` (REQUIRE_ORDER). Verified against the host glibc by
//! the differential test below.
//!
//! Only the features MooseFS optstrings use are supported: plain, `x:` and
//! `x::` options; no leading `+`/`-`/`:` and no `W;`.
#![deny(unsafe_code)]

/// One `getopt()` result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Next {
    /// A valid option, with its argument for `x:`/`x::` options.
    Opt(u8, Option<Vec<u8>>),
    /// Unknown option (glibc prints "invalid option" and returns '?').
    Invalid(u8),
    /// `x:` at the end of argv (glibc prints "option requires an argument").
    MissingArg(u8),
    /// No more options (`getopt` returned -1).
    End,
}

pub struct Getopt<'a> {
    args: &'a [Vec<u8>],
    optstring: &'a [u8],
    /// Current argv order: `order[i]` is the original index of argv[i].
    pub order: Vec<usize>,
    /// C `optind`.
    pub optind: usize,
    /// `__nextchar`: (original arg index, byte offset); None = NULL.
    next: Option<(usize, usize)>,
    first_nonopt: usize,
    last_nonopt: usize,
    permute: bool,
}

impl<'a> Getopt<'a> {
    /// `posixly_correct`: the `POSIXLY_CORRECT` environment variable is set.
    pub fn new(args: &'a [Vec<u8>], optstring: &'a [u8], posixly_correct: bool) -> Self {
        debug_assert!(!matches!(optstring.first(), Some(b'+' | b'-' | b':')));
        debug_assert!(!optstring.windows(2).any(|w| w == b"W;"));
        Getopt {
            args,
            optstring,
            order: (0..args.len()).collect(),
            optind: 1,
            next: None,
            first_nonopt: 1,
            last_nonopt: 1,
            permute: !posixly_correct,
        }
    }

    fn arg(&self, i: usize) -> &'a [u8] {
        &self.args[self.order[i]]
    }

    fn nonoption(&self, i: usize) -> bool {
        let a = self.arg(i);
        a.first() != Some(&b'-') || a.len() == 1
    }

    /// glibc `exchange()`: move the skipped non-options after the options.
    fn exchange(&mut self) {
        let (bottom, middle, top) = (self.first_nonopt, self.last_nonopt, self.optind);
        self.order[bottom..top].rotate_left(middle - bottom);
        self.first_nonopt += self.optind - self.last_nonopt;
        self.last_nonopt = self.optind;
    }

    /// C `getopt(argc, argv, optstring)`.
    pub fn next(&mut self) -> Next {
        let argc = self.args.len();
        if self.optind > argc {
            self.optind = argc;
        }
        let at_end = match self.next {
            None => true,
            Some((a, off)) => off >= self.args[a].len(),
        };
        if at_end {
            if self.last_nonopt > self.optind {
                self.last_nonopt = self.optind;
            }
            if self.first_nonopt > self.optind {
                self.first_nonopt = self.optind;
            }
            if self.permute {
                if self.first_nonopt != self.last_nonopt && self.last_nonopt != self.optind {
                    self.exchange();
                } else if self.last_nonopt != self.optind {
                    self.first_nonopt = self.optind;
                }
                while self.optind < argc && self.nonoption(self.optind) {
                    self.optind += 1;
                }
                self.last_nonopt = self.optind;
            }
            if self.optind != argc && self.arg(self.optind) == b"--" {
                self.optind += 1;
                if self.first_nonopt != self.last_nonopt && self.last_nonopt != self.optind {
                    self.exchange();
                } else if self.first_nonopt == self.last_nonopt {
                    self.first_nonopt = self.optind;
                }
                self.last_nonopt = argc;
                self.optind = argc;
            }
            if self.optind == argc {
                if self.first_nonopt != self.last_nonopt {
                    self.optind = self.first_nonopt;
                }
                self.next = None;
                return Next::End;
            }
            if self.nonoption(self.optind) {
                // REQUIRE_ORDER: stop at the first non-option
                self.next = None;
                return Next::End;
            }
            self.next = Some((self.order[self.optind], 1));
        }
        let (ai, off) = self.next.expect("positioned inside an option cluster");
        let a: &'a [u8] = &self.args[ai];
        let c = a[off];
        let rest = off + 1;
        self.next = Some((ai, rest));
        if rest == a.len() {
            self.optind += 1;
        }
        let pos = match self.optstring.iter().position(|&x| x == c) {
            Some(p) if c != b':' && c != b';' => p,
            _ => return Next::Invalid(c),
        };
        if self.optstring.get(pos + 1) != Some(&b':') {
            return Next::Opt(c, None);
        }
        let optional = self.optstring.get(pos + 2) == Some(&b':');
        let arg = if rest < a.len() {
            self.optind += 1;
            Some(a[rest..].to_vec())
        } else if optional {
            None
        } else if self.optind == argc {
            self.next = None;
            return Next::MissingArg(c);
        } else {
            let v = self.arg(self.optind).to_vec();
            self.optind += 1;
            Some(v)
        };
        self.next = None;
        Next::Opt(c, arg)
    }
}

#[cfg(test)]
#[allow(unsafe_code)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::sync::Mutex;

    static GLIBC: Mutex<()> = Mutex::new(());

    unsafe extern "C" {
        static mut optind: libc::c_int;
        static mut opterr: libc::c_int;
        static mut optarg: *mut libc::c_char;
    }

    /// Run glibc getopt: (results, final optind, final argv order).
    fn glibc(args: &[Vec<u8>], optstring: &[u8]) -> (Vec<Next>, usize, Vec<usize>) {
        let _g = GLIBC.lock().unwrap_or_else(|e| e.into_inner());
        let cs: Vec<CString> = args.iter().map(|a| CString::new(a.clone()).unwrap()).collect();
        let mut ptrs: Vec<*mut libc::c_char> = cs.iter().map(|c| c.as_ptr() as *mut _).collect();
        let base = ptrs.clone();
        ptrs.push(std::ptr::null_mut());
        let os = CString::new(optstring).unwrap();
        let mut out = Vec::new();
        // glibc state is reset by optind=0 and serialized by GLIBC.
        // SAFETY: argv/optstring stay valid for all getopt calls.
        unsafe {
            optind = 0;
            opterr = 0;
            loop {
                let c = libc::getopt(args.len() as libc::c_int, ptrs.as_mut_ptr(), os.as_ptr());
                if c == -1 {
                    out.push(Next::End);
                    break;
                }
                let c = c as u8;
                if c == b'?' {
                    // error return: glibc sets optopt to the offending char
                    let oo = *libc_optopt() as u8;
                    let p = optstring.iter().position(|&x| x == oo);
                    if p.is_some_and(|p| optstring.get(p + 1) == Some(&b':')) {
                        out.push(Next::MissingArg(oo));
                    } else {
                        out.push(Next::Invalid(oo));
                    }
                } else {
                    let arg = if optarg.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(optarg).to_bytes().to_vec())
                    };
                    let p = optstring.iter().position(|&x| x == c).unwrap();
                    let arg = if optstring.get(p + 1) == Some(&b':') { arg } else { None };
                    out.push(Next::Opt(c, arg));
                }
                optarg = std::ptr::null_mut();
            }
            let order = ptrs[..args.len()]
                .iter()
                .map(|p| base.iter().position(|b| b == p).unwrap())
                .collect();
            (out, optind as usize, order)
        }
    }

    unsafe extern "C" {
        static mut optopt: libc::c_int;
    }
    fn libc_optopt() -> *mut libc::c_int {
        &raw mut optopt
    }

    fn ours(args: &[Vec<u8>], optstring: &[u8]) -> (Vec<Next>, usize, Vec<usize>) {
        let mut g = Getopt::new(args, optstring, false);
        let mut out = Vec::new();
        loop {
            let n = g.next();
            let end = n == Next::End;
            out.push(n);
            if end {
                break;
            }
        }
        (out, g.optind, g.order)
    }

    fn check(args: &[&str], optstring: &str) {
        let args: Vec<Vec<u8>> = args.iter().map(|s| s.as_bytes().to_vec()).collect();
        assert_eq!(ours(&args, optstring.as_bytes()), glibc(&args, optstring.as_bytes()), "{args:?}");
    }

    /// glibc returns '?' both for errors and for a listed '?' option, so
    /// the differential tests use an optstring without '?'.
    #[test]
    fn question_mark_listed_is_an_option() {
        let args: Vec<Vec<u8>> = vec![b"p".to_vec(), b"-?".to_vec(), b"-y".to_vec()];
        let mut g = Getopt::new(&args, b"h?", false);
        assert_eq!(g.next(), Next::Opt(b'?', None));
        assert_eq!(g.next(), Next::Invalid(b'y'));
        assert_eq!(g.next(), Next::End);
    }

    #[test]
    fn fixed_cases_match_glibc() {
        let o = "nuvfdc:t:hiax";
        check(&["p"], o);
        check(&["p", "-f"], o);
        check(&["p", "start", "-f"], o);
        check(&["p", "-c", "x.cfg", "restart", "-fd"], o);
        check(&["p", "-cx.cfg", "-t5", "stop"], o);
        check(&["p", "a", "b", "-f", "c", "-u", "--", "-d", "e"], o);
        check(&["p", "--", "-f"], o);
        check(&["p", "-", "-f"], o);
        check(&["p", "-q"], o);
        check(&["p", "-c"], o);
        check(&["p", "-fc"], o);
        check(&["p", "--foo"], o);
        check(&["p", "-?"], o);
        check(&["p", "-f", "-y", "a"], o);
        check(&["p", "x", "-", "y", "-xx"], o);
    }

    #[test]
    fn fuzz_matches_glibc() {
        let o = "nuvfdc:t:hiax";
        let pieces = ["-f", "-c", "cfg", "start", "-", "--", "-fd", "-t9", "-q", "-cX", "x", "-x", "-ax"];
        let mut seed: u64 = 0x1234_5678;
        for _ in 0..20_000 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let n = (seed % 7) as usize;
            let mut s = seed;
            let mut v = vec!["prog"];
            for _ in 0..n {
                s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
                v.push(pieces[(s >> 33) as usize % pieces.len()]);
            }
            check(&v, o);
        }
    }
}
