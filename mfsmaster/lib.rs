#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
#![feature(strict_provenance)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod src {
    pub mod mfscommon {
        pub mod cfg;
        pub mod charts;
        pub mod clocks;
        pub mod cpuusage;
        pub mod crc;
        pub mod cuckoohash;
        pub mod dictionary;
        pub mod globengine;
        pub mod labelparser;
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
    pub mod mfsmaster {
        pub mod appendres;
        pub mod bgsaver;
        pub mod bio;
        pub mod changelog;
        pub mod chartsdata;
        pub mod chunkdelay;
        pub mod chunks;
        pub mod csdb;
        pub mod csipmap;
        pub mod datacachemgr;
        pub mod exports;
        pub mod filesystem;
        pub mod flocklocks;
        pub mod iptosesid;
        pub mod itree;
        pub mod matoclserv;
        pub mod matocsserv;
        pub mod matomlserv;
        pub mod merger;
        pub mod metadata;
        pub mod missinglog;
        pub mod multilan;
        pub mod openfiles;
        pub mod patterns;
        pub mod posixacl;
        pub mod posixlocks;
        pub mod restore;
        pub mod sessions;
        pub mod sharedpointer;
        pub mod storageclass;
        pub mod topology;
        pub mod xattr;
    } // mod mfsmaster
} // mod src
