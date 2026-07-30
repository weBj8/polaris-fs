#![feature(core_intrinsics)]
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

// Shared transpiled mfscommon modules (dedup, VC-05). The pub use makes
// their #[no_mangle] extern "C" symbols reachable so LTO retains them; the
// daemon's extern blocks resolve to these definitions at link time.
pub use mfscommon::{charts, clocks, conncache, cpuusage, crc, lwthread, md5, memusage, mfslog, processname, sockets, timeparser};

pub mod src {
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
    pub mod mfscommon {
        pub mod cfg;
        pub mod ionice;
        pub mod pcqueue;
        pub mod random;
        pub mod strerr;
    } // mod mfscommon
} // mod src
