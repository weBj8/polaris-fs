#![feature(core_intrinsics)]
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
// c2rust transpiles each C TU as a standalone module that redeclares shared
// symbols (fprintf, FILE*, malloc...) against its own local opaque types.
// Same C ABI, different Rust types — inherent to the translation model.
#![allow(clashing_extern_declarations)]

// (macro_use removed: modules import ::c2rust_bitfields directly)
extern crate c2rust_bitfields;
extern crate libc;

// Shared transpiled mfscommon modules (dedup, VC-05). The pub use makes
// their #[no_mangle] extern "C" symbols reachable so LTO retains them; the
// daemon's extern blocks resolve to these definitions at link time.
pub use plfscommon::{
    charts, clocks, conncache, cpuusage, crc, lwthread, md5, memusage, mfslog, processname,
    sockets, timeparser,
};

pub mod src {
    #[path = "plfschunkserver"]
    pub mod mfschunkserver {
        pub mod bgjobs;
        pub mod busychunks;
        pub mod chartsdata;
        pub mod csserv;
        pub mod hddspacemgr;
        pub mod mainserv;
        pub mod masterconn;
        pub mod replicator;
    } // mod mfschunkserver
    #[path = "plfscommon"]
    pub mod mfscommon {
        pub mod cfg;
        pub mod ionice;
        pub mod pcqueue;
        pub mod random;
        pub mod strerr;
    } // mod mfscommon
} // mod src
