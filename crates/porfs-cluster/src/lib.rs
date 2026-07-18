//! Cluster layer: static topology-aware membership and CRUSH-style striping.
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

/// A chunkserver's physical failure domain.
///
/// Rack is the mandatory replica-separation boundary. Chassis is retained as
/// part of the topology so placement policy can become more granular without
/// changing membership's input shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FailureDomain {
    /// Rack containing the server and its disk.
    pub rack: u64,
    /// Chassis containing the server and its disk.
    pub chassis: u64,
}

impl FailureDomain {
    /// Construct a physical failure domain.
    pub const fn new(rack: u64, chassis: u64) -> Self {
        Self { rack, chassis }
    }
}

/// A chunkserver endpoint and its physical topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Member {
    /// Network endpoint of the chunkserver.
    pub addr: SocketAddr,
    /// Failure domain of the server and its disk.
    pub failure_domain: FailureDomain,
}

/// Static cluster membership (P7): server index = identity.
#[derive(Debug, Clone)]
pub struct Membership {
    members: Vec<Member>,
    addrs: Vec<SocketAddr>,
}

impl Membership {
    /// Membership from server addresses in index order.
    ///
    /// This compatibility constructor assigns every server a distinct rack and
    /// chassis. Production callers must use [`Self::with_topology`] so
    /// replicated placement receives real failure-domain information.
    pub fn new(addrs: Vec<SocketAddr>) -> Self {
        Self::with_topology(
            addrs
                .into_iter()
                .enumerate()
                .map(|(index, addr)| Member {
                    addr,
                    failure_domain: FailureDomain::new(index as u64, index as u64),
                })
                .collect(),
        )
    }

    /// Membership from endpoints tagged with their physical topology.
    pub fn with_topology(members: Vec<Member>) -> Self {
        assert!(!members.is_empty(), "membership must not be empty");
        let addrs = members.iter().map(|member| member.addr).collect();
        Self { members, addrs }
    }

    /// Number of servers in the pool.
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Always false: membership is non-empty by construction.
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Address of server `index`.
    pub fn addr(&self, index: usize) -> SocketAddr {
        self.members[index].addr
    }

    /// All addresses, in index order.
    pub fn addrs(&self) -> &[SocketAddr] {
        &self.addrs
    }

    /// All members, in index order.
    pub fn members(&self) -> &[Member] {
        &self.members
    }

    /// Failure domain of server `index`.
    pub fn failure_domain(&self, index: usize) -> FailureDomain {
        self.members[index].failure_domain
    }
}

/// CRUSH-style placement: for every `(inode, chunk_index)` the server
/// with the highest rendezvous score `hash(inode, chunk, server)` wins.
/// Adding a server moves only the ~1/(N+1) of chunks that score higher
/// on it; removing one moves only its own.
#[derive(Debug, Clone)]
pub struct StripeMap {
    failure_domains: Vec<FailureDomain>,
}

impl StripeMap {
    /// A placement function over `members` servers.
    pub fn new(members: usize) -> Self {
        assert!(members > 0, "stripe map needs at least one member");
        Self {
            failure_domains: (0..members)
                .map(|index| FailureDomain::new(index as u64, index as u64))
                .collect(),
        }
    }

    /// A placement function over a topology-tagged membership.
    pub fn from_membership(membership: &Membership) -> Self {
        Self {
            failure_domains: membership
                .members()
                .iter()
                .map(|member| member.failure_domain)
                .collect(),
        }
    }

    /// Number of members this map places onto.
    pub fn members(&self) -> usize {
        self.failure_domains.len()
    }

    /// The chunkserver index holding chunk `chunk_index` of `inode`.
    pub fn place(&self, inode: u64, chunk_index: u64) -> usize {
        let mut best = 0usize;
        let mut best_score = 0u64;
        for server in 0..self.members() {
            let score = score(inode, chunk_index, server as u64);
            if score > best_score {
                best_score = score;
                best = server;
            }
        }
        best
    }

    /// Primary and secondary for a chunk, ranked by rendezvous score and
    /// separated across racks.
    ///
    /// Requires at least two racks because P9 always keeps two copies.
    pub fn place_replicas(&self, inode: u64, chunk_index: u64) -> (usize, usize) {
        assert!(
            self.members() >= 2,
            "replicated placement needs at least two members"
        );
        let mut ranked: Vec<(u64, usize)> = (0..self.members())
            .map(|server| (score(inode, chunk_index, server as u64), server))
            .collect();
        ranked.sort_unstable_by(|a, b| b.cmp(a));
        let primary = ranked[0].1;
        let secondary = ranked
            .iter()
            .skip(1)
            .find_map(|&(_, server)| {
                (self.failure_domains[server].rack != self.failure_domains[primary].rack)
                    .then_some(server)
            })
            .expect("replicated placement requires at least two racks");
        (primary, secondary)
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

/// Two independently-addressable copies of one immutable stripe chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplicaLoc {
    /// Primary copy, which coordinates the initial chain write.
    pub primary: ChunkLoc,
    /// Secondary copy, used for failover reads.
    pub secondary: ChunkLoc,
}

/// Placement record for a two-way replicated striped file.
#[derive(Debug, Clone)]
pub struct ReplicatedLayout {
    /// Inode of the striped file.
    pub inode: u64,
    /// Monotonically increasing content generation used in write IDs.
    pub generation: u64,
    /// Total logical size in bytes.
    pub len: u64,
    /// One primary/secondary pair per chunk.
    pub chunks: Vec<ReplicaLoc>,
}

impl Layout {
    /// Total number of chunks.
    pub fn chunk_count(&self) -> u64 {
        self.chunks.len() as u64
    }
}
