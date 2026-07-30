// Shared c2rust-transpiled MooseFS mfscommon modules (deduplicated P0).
// These files were byte-identical across all daemon crates (dedup-map.md).
#![feature(c_variadic)]
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

// Test-only symbol stubs: strerr lives in the per-daemon (divergent)
// strerr.rs, so the mfscommon test binary has no provider. Daemons link
// their own; this stub exists solely for `cargo test`.
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn strerr(_error: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    b"test-stub\0".as_ptr() as *const ::core::ffi::c_char
}

pub mod charts;
pub mod clocks;
pub mod conncache;
pub mod cpuusage;
pub mod crc;
pub mod delayrun;
pub mod labelparser;
pub mod lwthread;
pub mod md5;
pub mod memusage;
pub mod mfslog;
pub mod processname;
pub mod sockets;
pub mod timeparser;
