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
        pub mod clocks;
        pub mod conncache;
        pub mod crc;
        pub mod delayrun;
        pub mod labelparser;
        pub mod lwthread;
        pub mod md5;
        pub mod mfslog;
        pub mod pcqueue;
        pub mod processname;
        pub mod sockets;
        pub mod squeue;
        pub mod strerr;
        pub mod workers;
    } // mod mfscommon
} // mod src
