//! PolarisFS FUSE client v0: a `fuser::Filesystem` implementation over
//! [`porfs_mds::Mds`], plus the mount configuration.
//!
//! Architecture: [`PorfsFs`] wraps the MDS in one `std::sync::Mutex` —
//! fuser dispatches requests from multiple threads, and v0 serializes
//! everything through a single global lock (the MDS is synchronous anyway).
//! Lock splitting is a P19 performance item; nothing here preempts it.
//!
//! Op wiring is deliberately boring: every FUSE op maps to the MDS call of
//! the same name, errors map to errnos (`map` module), and times/kinds map
//! mechanically. Semantics gaps of v0 (documented in the phase report):
//! no symlink/mknod (P5), no xattr (P5), no fsync-on-close (close-to-open
//! lands in P11), no permission checks (`access` allows all; ACLs land in
//! P24), and O_APPEND correctness under a stale attribute cache is
//! single-client-only until P17.

mod config;
mod fs;
mod map;
mod worker;

pub use config::MountConfig;
pub use fs::PorfsFs;
