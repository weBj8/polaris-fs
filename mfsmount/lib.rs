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
        pub mod dentry_invalidator;
        pub mod dirattrcache;
        pub mod dirblob_name_index;
        pub mod dirblob_node_index;
        pub mod extrapackets;
        pub mod fdcache;
        pub mod getgroups;
        pub mod heapsorter;
        pub mod inoleng;
        pub mod mastercomm;
        pub mod masterproxy;
        pub mod mfs_fuse;
        pub mod mfs_meta_fuse;
        pub mod negentrycache;
        pub mod oplog;
        pub mod readdata;
        pub mod stats;
        pub mod sustained_inodes;
        pub mod sustained_parents;
        pub mod sustained_stats;
        pub mod symlinkcache;
        pub mod truncate;
        pub mod writedata;
        pub mod xattrcache;
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
        pub mod strerr;
    } // mod mfscommon
} // mod src
