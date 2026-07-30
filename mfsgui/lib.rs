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
pub use mfscommon::{clocks, crc, md5, mfslog, processname, sockets, timeparser};

pub mod src {
    pub mod mfscommon {
        pub mod cfg;
        pub mod strerr;
    } // mod mfscommon
    pub mod mfsgui {
        pub mod mfsgui;
    } // mod mfsgui
} // mod src
