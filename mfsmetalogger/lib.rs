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

pub mod src {
    pub mod mfscommon {
        pub mod cfg;
        pub mod clocks;
        pub mod crc;
        pub mod md5;
        pub mod mfslog;
        pub mod processname;
        pub mod sockets;
        pub mod strerr;
        pub mod timeparser;
    } // mod mfscommon
    pub mod mfsmetalogger {
        pub mod masterconn;
    } // mod mfsmetalogger
} // mod src
