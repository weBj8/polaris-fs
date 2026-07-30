// Shared c2rust-transpiled MooseFS mfsclient modules (deduplicated P0).
// These 12 files were byte-identical between mfsmount and mfsbdev
// (dedup-map.md, VC-06). FUSE- vs NBD-specific modules stay per-daemon.
#![feature(core_intrinsics)]
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
// c2rust transpiles each C TU as a standalone module that redeclares shared
// symbols against its own local opaque types. Same C ABI — model-inherent.
#![allow(clashing_extern_declarations)]

// (macro_use removed: modules import ::c2rust_bitfields directly)
extern crate c2rust_bitfields;
extern crate libc;

pub mod chunkrwlock;
pub mod chunksdatacache;
pub mod csdb;
pub mod csorder;
pub mod extrapackets;
pub mod heapsorter;
pub mod inoleng;
pub mod mastercomm;
pub mod readdata;
pub mod stats;
pub mod truncate;
pub mod writedata;
