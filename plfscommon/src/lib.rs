// Shared c2rust-transpiled MooseFS mfscommon modules (deduplicated P0).
// These files were byte-identical across all daemon crates (dedup-map.md).
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

// (macro_use removed: modules import ::c2rust_bitfields directly)
extern crate c2rust_bitfields;
extern crate libc;

// Test-only provider: production daemons compile this shared source locally
// to avoid exporting duplicate C symbols to unrelated workspace binaries.
#[cfg(test)]
#[path = "strerr.rs"]
mod strerr;

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
