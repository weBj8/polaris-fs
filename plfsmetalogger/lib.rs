// Shared mfscommon modules (VC-05). The pub use keeps their #[no_mangle]
// extern "C" symbols reachable so LTO retains them for the C-ABI users
// that remain in the workspace.
pub use plfscommon::{cfg, clocks, crc, md5, mfslog, processname, sockets, timeparser};

pub mod src {
    #[path = "plfsmetalogger"]
    pub mod mfsmetalogger {
        pub mod masterconn;
    } // mod mfsmetalogger
} // mod src
