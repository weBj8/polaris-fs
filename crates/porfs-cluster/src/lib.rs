//! Cluster layer: static membership and CRUSH-style striping.
//!
//! Placement of a file's chunks on chunkservers is a **pure function** of
//! `(inode, chunk_index)` and the membership — rendezvous hashing (the
//! straw2 family CRUSH uses), so any client computes the same layout
//! without a central lookup. The per-chunk extent ids are server-assigned
//! and remain *metadata*: the writer returns them as a [`Layout`]; P9/P12
//! moves layouts into the MDS.

pub mod io;

use std::net::SocketAddr;

/// Logical bytes per stripe chunk (the unit placed on one chunkserver).
/// Matches EXTENT_DATA_MAX: one chunk = one extent, and per-op transport
/// overhead amortizes over the largest frame the wire allows.
pub const STRIPE_UNIT: u64 = 4 << 20;

/// Static cluster membership (P7): server index = identity.
#[derive(Debug, Clone)]
pub struct Membership {
    addrs: Vec<SocketAddr>,
}

impl Membership {
    /// Membership from server addresses in index order.
    pub fn new(addrs: Vec<SocketAddr>) -> Self {
        assert!(!addrs.is_empty(), "membership must not be empty");
        Self { addrs }
    }

    /// Number of servers in the pool.
    pub fn len(&self) -> usize {
        self.addrs.len()
    }

    /// Always false: membership is non-empty by construction.
    pub fn is_empty(&self) -> bool {
        self.addrs.is_empty()
    }

    /// Address of server `index`.
    pub fn addr(&self, index: usize) -> SocketAddr {
        self.addrs[index]
    }

    /// All addresses, in index order.
    pub fn addrs(&self) -> &[SocketAddr] {
        &self.addrs
    }
}

/// CRUSH-style placement: for every `(inode, chunk_index)` the server
/// with the highest rendezvous score `hash(inode, chunk, server)` wins.
/// Adding a server moves only the ~1/(N+1) of chunks that score higher
/// on it; removing one moves only its own.
#[derive(Debug, Clone)]
pub struct StripeMap {
    members: usize,
}

impl StripeMap {
    /// A placement function over `members` servers.
    pub fn new(members: usize) -> Self {
        assert!(members > 0, "stripe map needs at least one member");
        Self { members }
    }

    /// Number of members this map places onto.
    pub fn members(&self) -> usize {
        self.members
    }

    /// The chunkserver index holding chunk `chunk_index` of `inode`.
    pub fn place(&self, inode: u64, chunk_index: u64) -> usize {
        let mut best = 0usize;
        let mut best_score = 0u64;
        for server in 0..self.members {
            let score = score(inode, chunk_index, server as u64);
            if score > best_score {
                best_score = score;
                best = server;
            }
        }
        best
    }

    /// Number of chunks covering `len` bytes.
    pub fn chunk_count(len: u64) -> u64 {
        len.div_ceil(STRIPE_UNIT)
    }
}

/// The rendezvous score of one (inode, chunk, server) triple.
fn score(inode: u64, chunk_index: u64, server: u64) -> u64 {
    use std::hash::Hasher;
    let mut hasher = twox_hash::XxHash64::with_seed(0x7091_5A1D_5EED_0001);
    hasher.write_u64(inode);
    hasher.write_u64(chunk_index);
    hasher.write_u64(server);
    hasher.finish()
}

/// Where one chunk lives: server index (rendezvous) + the extent id the
/// server assigned on write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkLoc {
    /// Server index within the membership.
    pub server: usize,
    /// Server-local extent id of the chunk's payload.
    pub extent_id: u64,
}

/// The placement record of one striped file: `chunks[i]` locates chunk
/// `i` (`logical offset = i * STRIPE_UNIT`). Produced by the writer;
/// required by the reader; owned by the metadata layer from P9 on.
#[derive(Debug, Clone)]
pub struct Layout {
    /// Inode of the striped file.
    pub inode: u64,
    /// Total logical size in bytes (last chunk may be short).
    pub len: u64,
    /// One entry per chunk, in chunk-index order.
    pub chunks: Vec<ChunkLoc>,
}

impl Layout {
    /// Total number of chunks.
    pub fn chunk_count(&self) -> u64 {
        self.chunks.len() as u64
    }
}
