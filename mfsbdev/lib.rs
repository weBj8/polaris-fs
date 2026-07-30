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
pub use mfscommon::{clocks, conncache, crc, delayrun, labelparser, lwthread, md5, mfslog, processname, sockets};

pub mod src {
    pub mod mfsclient {
        pub mod chunkrwlock;
        pub mod chunksdatacache;
        pub mod csdb;
        pub mod csorder;
        pub mod extrapackets;
        pub mod heapsorter;
        pub mod inoleng;
        pub mod mastercomm;
        pub mod mfsio;
        pub mod mfsioint;
        pub mod mfsioint_lookupcache;
        pub mod readdata;
        pub mod stats;
        pub mod truncate;
        pub mod writedata;
    } // mod mfsclient
    pub mod mfscommon {
        pub mod pcqueue;
        pub mod squeue;
        pub mod strerr;
        pub mod workers;
    } // mod mfscommon
} // mod src
