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
    clocks, conncache, crc, delayrun, labelparser, lwthread, md5, mfslog, processname, sockets,
};
// Shared transpiled mfsclient modules (dedup, VC-06); pub use keeps their
// #[no_mangle] extern "C" symbols reachable so fat LTO retains them.
pub use plfsclient::{
    chunkrwlock, chunksdatacache, csdb, csorder, extrapackets, heapsorter, inoleng, mastercomm,
    readdata, stats, truncate, writedata,
};

pub mod src {
    // Mount-only (FUSE) client modules — the ones that diverged from
    // mfsbdev's NBD path in the C mfsclient/ tree (VC-06). Renamed from
    // `mfsclient` to `fuse_client` to stop colliding with the shared
    // `mfsclient` crate above.
    pub mod fuse_client {
        pub mod dentry_invalidator;
        pub mod dirattrcache;
        pub mod dirbuf;
        pub mod fdcache;
        pub mod getgroups;
        pub mod masterproxy;
        pub mod mfs_fuse;
        pub mod mfs_meta_fuse;
        pub mod negentrycache;
        pub mod oplog;
        pub mod sustained_inodes;
        pub mod sustained_parents;
        pub mod sustained_stats;
        pub mod symlinkcache;
        pub mod xattrcache;
    } // mod fuse_client
    #[path = "plfscommon"]
    pub mod mfscommon {
        pub mod pcqueue;
        pub mod strerr;
    } // mod mfscommon
} // mod src
