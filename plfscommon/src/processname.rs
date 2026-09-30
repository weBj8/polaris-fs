//! Process title (ps/top name) — port of mfscommon/processname.c
//! (the `!HAVE_SETPROCTITLE` Linux path).
//!
//! BOUNDARY MODULE: the title is rewritten in place inside the C runtime's
//! argv/environ memory block, and `environ` is replaced by a heap copy so
//! the old environment strings may be overwritten. No safe abstraction can
//! express writing into memory owned by the C runtime, so the raw pointer
//! work is confined to the annotated fns below. The region arithmetic is
//! safe code in `imp` and unit-tested.
//!
//! Behavior preserved from C, including its strdup-failure path: the C
//! cleanup loop frees `environ[i]` (the NULL slot) instead of `environ[j]`,
//! so already-copied strings leak; here they are left untouched likewise
//! and only the pointer array is freed. One deliberate deviation on that
//! same path: C keeps `myenvcpy` pointing at the freed array (a double free
//! if `processname_term` ran); it is cleared here.

use core::ffi::{c_char, c_int};
use std::sync::Mutex;

#[deny(unsafe_code)]
mod imp {
    /// C `lastpos` walk: `strings` are (address, strlen) of argv[0..argc]
    /// followed by the original environment. A string extends the region
    /// when it starts right after the previous region end's NUL. Returns
    /// C `argv_leng = (lastpos - argv_start) - 1` as `uint32_t`.
    pub fn region_len(argv: &[(usize, usize)], env: &[(usize, usize)]) -> u32 {
        let mut lastpos: Option<usize> = None;
        for &(p, l) in argv {
            if lastpos.is_none_or(|lp| lp.wrapping_add(1) == p) {
                lastpos = Some(p.wrapping_add(l));
            }
        }
        let mut lastpos = lastpos.unwrap_or(0);
        for &(p, l) in env {
            if lastpos.wrapping_add(1) == p {
                lastpos = p.wrapping_add(l);
            }
        }
        let start = argv.first().map_or(0, |a| a.0);
        (lastpos.wrapping_sub(start) as isize - 1) as u32
    }

    /// C `processname_set` length: bytes of `name` copied (the rest of the
    /// `argv_leng` window is zeroed).
    pub fn title_len(name_len: usize, argv_leng: u32) -> u32 {
        let l = name_len as u32;
        if l >= argv_leng { argv_leng - 1 } else { l }
    }
}

struct Region {
    start: usize,
    leng: u32,
    envcpy: usize,
}

/// C statics `argv_start`, `argv_leng`, `myenvcpy` (addresses as usize).
static REGION: Mutex<Region> = Mutex::new(Region { start: 0, leng: 0, envcpy: 0 });

unsafe extern "C" {
    static mut environ: *mut *mut c_char;
}

fn region() -> std::sync::MutexGuard<'static, Region> {
    REGION.lock().unwrap_or_else(|e| e.into_inner())
}

/// # Safety
/// `argv` must be the process argv (argc valid C strings) and must be
/// called before other threads read `environ` (process bootstrap).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn processname_init(argc: c_int, argv: *mut *mut c_char) {
    let mut r = region();
    r.start = 0;
    r.leng = 0;
    // SAFETY: per fn contract argv has argc entries; environ is the C
    // runtime's NULL-terminated array, replaced only here at bootstrap.
    unsafe {
        if argc == 0 || (*argv).is_null() {
            return;
        }
        let argp = environ;
        let mut n = 0usize;
        while !(*argp.add(n)).is_null() {
            n += 1;
        }
        let copy = libc::malloc((n + 1) * size_of::<*mut c_char>()) as *mut *mut c_char;
        r.envcpy = copy as usize;
        if copy.is_null() {
            return;
        }
        environ = copy;
        for i in 0..n {
            let d = libc::strdup(*argp.add(i));
            if d.is_null() {
                libc::free(copy as *mut libc::c_void);
                r.envcpy = 0;
                environ = argp;
                return;
            }
            *copy.add(i) = d;
        }
        *copy.add(n) = core::ptr::null_mut();
        let span = |p: *mut c_char| (p as usize, libc::strlen(p));
        let args: Vec<(usize, usize)> = (0..argc as usize).map(|i| span(*argv.add(i))).collect();
        let envs: Vec<(usize, usize)> = (0..n).map(|i| span(*argp.add(i))).collect();
        r.start = *argv as usize;
        r.leng = imp::region_len(&args, &envs);
    }
}

/// # Safety
/// `name` must be a valid C string; `processname_init` must have run (or
/// the region is empty and nothing is written).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn processname_set(name: *mut c_char) {
    let r = region();
    if r.leng == 0 {
        return;
    }
    // SAFETY: [start, start+leng) is the contiguous argv/environ block
    // measured in init, no longer referenced by environ (copied away).
    unsafe {
        let l = imp::title_len(libc::strlen(name), r.leng) as usize;
        let dst = r.start as *mut u8;
        if l > 0 {
            core::ptr::copy_nonoverlapping(name as *const u8, dst, l);
        }
        core::ptr::write_bytes(dst.add(l), 0, (r.leng as usize) - l);
    }
}

/// C: "DO NOT CALL" — only frees the environ pointer array for ASan.
#[unsafe(no_mangle)]
pub extern "C" fn processname_term() {
    let p = region().envcpy;
    if p != 0 {
        // SAFETY: allocated by malloc in processname_init.
        unsafe { libc::free(p as *mut libc::c_void) };
    }
}

#[cfg(test)]
mod tests {
    use super::imp::*;

    #[test]
    fn contiguous_block_spans_argv_and_env() {
        // "ab\0cde\0X=1\0" at 100: argv ab@100, cde@103; env X=1@107
        assert_eq!(region_len(&[(100, 2), (103, 3)], &[(107, 3)]), 9);
        // a gap stops the walk (env not adjacent)
        assert_eq!(region_len(&[(100, 2), (103, 3)], &[(200, 3)]), 5);
        // non-adjacent argv entry is skipped but later adjacent ones count
        assert_eq!(region_len(&[(100, 2), (50, 1), (103, 1)], &[]), 3);
    }

    #[test]
    fn title_len_clips_to_window() {
        assert_eq!(title_len(3, 10), 3);
        assert_eq!(title_len(10, 10), 9);
        assert_eq!(title_len(50, 10), 9);
    }
}
