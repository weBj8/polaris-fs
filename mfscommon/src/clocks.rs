//! Monotonic clock helpers (CLOCK_MONOTONIC), migrated to safe Rust (P1).
//! Semantics preserved from the c2rust original: values are raw
//! CLOCK_MONOTONIC timestamps (boot-relative), not `std::time::Instant`.
//!
//! Layout per porting.md: safe logic in `imp` (deny unsafe, one annotated
//! escape hatch), C ABI exports at top level (no module-level deny:
//! `#[unsafe(no_mangle)]` is itself an unsafe attribute in edition 2024).

pub type uint32_t = u32;
pub type uint64_t = u64;

#[deny(unsafe_code)]
mod imp {
    /// nanoseconds since boot (CLOCK_MONOTONIC)
    // ponytail: this one libc call is why the module isn't 100% safe —
    // std exposes no raw CLOCK_MONOTONIC value (Instant is opaque).
    #[allow(unsafe_code)]
    pub fn nseconds() -> u64 {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        // SAFETY: ts is a valid pointer to a timespec we own;
        // clock_gettime(CLOCK_MONOTONIC) cannot fail.
        unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
        (ts.tv_sec as u64)
            .wrapping_mul(1_000_000_000)
            .wrapping_add(ts.tv_nsec as u64)
    }
}

pub use imp::nseconds;

#[unsafe(no_mangle)]
pub extern "C" fn monotonic_seconds() -> ::core::ffi::c_double {
    let ns = nseconds();
    (ns / 1_000_000_000) as ::core::ffi::c_double
        + (ns % 1_000_000_000) as ::core::ffi::c_double * 0.000000001f64
}

#[unsafe(no_mangle)]
pub extern "C" fn monotonic_nseconds() -> uint64_t {
    nseconds()
}

#[unsafe(no_mangle)]
pub extern "C" fn monotonic_useconds() -> uint64_t {
    nseconds().wrapping_div(1000)
}

#[unsafe(no_mangle)]
pub extern "C" fn monotonic_method() -> *const ::core::ffi::c_char {
    b"clock_gettime\0".as_ptr() as *const ::core::ffi::c_char
}

#[unsafe(no_mangle)]
pub extern "C" fn monotonic_speed() -> uint32_t {
    let mut i: uint32_t = 0;
    let st = nseconds().wrapping_add(10_000_000);
    loop {
        let en = nseconds();
        i = i.wrapping_add(1);
        if en >= st {
            break;
        }
    }
    i
}
