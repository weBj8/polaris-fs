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
        pub mod charts;
        pub mod clocks;
        pub mod conncache;
        pub mod cpuusage;
        pub mod crc;
        pub mod ionice;
        pub mod lwthread;
        pub mod md5;
        pub mod memusage;
        pub mod mfslog;
        pub mod pcqueue;
        pub mod processname;
        pub mod random;
        pub mod sockets;
        pub mod strerr;
        pub mod timeparser;
    } // mod mfscommon
} // mod src
