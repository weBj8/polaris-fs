//! /proc/self/statm memory usage, migrated to safe Rust (P1).
//! Layout: safe logic in `imp` (deny unsafe, one annotated escape hatch for
//! sysconf), C ABI out-param export at top level.

pub type uint64_t = u64;
pub type uint8_t = u8;

#[deny(unsafe_code)]
mod imp {
    /// pages of (total, rss) from /proc/self/statm; rss None if the second
    /// field is missing/unparseable (the C original set *virt but returned 0)
    pub fn statm() -> Option<(u64, Option<u64>)> {
        let buf = std::fs::read("/proc/self/statm").ok()?;
        let mut it = buf.split(|&b| b == b' ');
        let total = std::str::from_utf8(it.next()?).ok()?.parse().ok()?;
        let rss = it
            .next()
            .and_then(|f| std::str::from_utf8(f).ok()?.parse().ok());
        Some((total, rss))
    }

    // ponytail: sysconf has no std equivalent; error (-1) matches the C
    // original's unchecked getpagesize().
    #[allow(unsafe_code)]
    pub fn pagesize() -> u64 {
        // SAFETY: sysconf(_SC_PAGESIZE) reads process-global config; no
        // memory unsafety possible.
        unsafe { libc::sysconf(libc::_SC_PAGESIZE) as u64 }
    }
}

/// C ABI out-param wrapper — the six daemon crates call this by symbol.
///
/// # Safety
/// `rss` and `virt` must be valid writable `u64` pointers (C caller contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mem_used(mut rss: *mut uint64_t, mut virt: *mut uint64_t) -> uint8_t {
    // SAFETY: per fn contract; both written unconditionally, as the original did.
    unsafe {
        *rss = 0;
        *virt = 0;
        if let Some((t, r)) = imp::statm() {
            let ps = imp::pagesize();
            *virt = t.wrapping_mul(ps);
            if let Some(r) = r {
                *rss = r.wrapping_mul(ps);
                return 1;
            }
        }
    }
    0
}
