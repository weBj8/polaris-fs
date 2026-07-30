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
// Shared transpiled mfsclient modules (dedup, VC-06); pub use keeps their
// #[no_mangle] extern "C" symbols reachable so fat LTO retains them.
pub use mfsclient::{chunkrwlock, chunksdatacache, csdb, csorder, extrapackets, heapsorter, inoleng, mastercomm, readdata, stats, truncate, writedata};

pub mod src {
    pub mod mfsclient {
        pub mod mfsio;
        pub mod mfsioint;
        pub mod mfsioint_lookupcache;
    } // mod mfsclient
    pub mod mfscommon {
        pub mod pcqueue;
        pub mod squeue;
        pub mod strerr;
        pub mod workers;
    } // mod mfscommon
} // mod src
