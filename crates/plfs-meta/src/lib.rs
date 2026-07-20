//! Metadata state machine (design doc §7.3): the per-volume namespace on
//! redb — inodes, directory entries, file layouts, chunk refcounts, snapshot
//! rows, and the GC queue. Every mutation is a Raft-log-shaped [`MetaOp`]
//! applied in ONE redb write transaction (atomic rename for free); S8 wraps
//! this machine in openraft, S15 reuses redb savepoints for snapshots.

mod state;

pub mod raft;
pub mod service;
pub mod transport;

pub use state::{MetaState, ROOT_INO};

use serde::{Deserialize, Serialize};

/// Seconds + nanoseconds timestamp.
pub type Time = (i64, u32);

/// Inode kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    /// Regular file.
    File,
    /// Directory.
    Dir,
    /// Symbolic link (target stored in the inode).
    Symlink,
}

/// Inode attributes (POSIX surface, design doc §8.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inode {
    /// File/directory/symlink.
    pub kind: Kind,
    /// Permission bits.
    pub mode: u32,
    /// Owner uid.
    pub uid: u32,
    /// Owner gid.
    pub gid: u32,
    /// Logical file size in bytes (authoritative for reads; a chunk
    /// straddling the size stays sealed and whole, never read past `size`).
    pub size: u64,
    /// Access time (relatime semantics at the FUSE layer).
    pub atime: Time,
    /// Modification time.
    pub mtime: Time,
    /// Change time (bumped by every mutation).
    pub ctime: Time,
    /// Hard-link count; the inode is freed when it reaches 0.
    pub nlink: u32,
    /// Symlink target (only for `Kind::Symlink`).
    pub symlink_target: Option<String>,
}

/// One chunk of a file layout (§7.2 CoW: sealed, never mutated in place).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkRef {
    /// ChunkArena chunk id (UUIDv7).
    pub chunk_id: [u8; 16],
    /// Caller version metadata (exact-version delete protects GC races).
    pub version: u64,
    /// Bytes of file content carried by this chunk.
    pub len: u64,
    /// Data-node addresses holding this chunk's replicas (RF = len).
    pub replicas: Vec<String>,
}

/// A dead chunk awaiting data-node Delete (exact-version, idempotent).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GcEntry {
    /// Chunk to delete.
    pub chunk_id: [u8; 16],
    /// Version it was deleted with.
    pub version: u64,
    /// Replica addresses to delete it from.
    pub replicas: Vec<String>,
}

/// Snapshot row (schema only at S7; S15 adds the checkpoint machinery).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotMeta {
    /// Snapshot id.
    pub id: u64,
    /// User-visible name.
    pub name: String,
    /// Creation time.
    pub created_at: Time,
}

/// A metadata mutation — the Raft log entry shape (§7.3). Each op applies in
/// one redb write transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetaOp {
    /// Create a directory.
    Mkdir {
        /// Parent inode.
        parent: u64,
        /// Entry name.
        name: String,
        /// Permission bits.
        mode: u32,
        /// Owner uid.
        uid: u32,
        /// Owner gid.
        gid: u32,
    },
    /// Create a regular file.
    CreateFile {
        /// Parent inode.
        parent: u64,
        /// Entry name.
        name: String,
        /// Permission bits.
        mode: u32,
        /// Owner uid.
        uid: u32,
        /// Owner gid.
        gid: u32,
    },
    /// Create a symlink.
    Symlink {
        /// Parent inode.
        parent: u64,
        /// Entry name.
        name: String,
        /// Link target.
        target: String,
        /// Owner uid.
        uid: u32,
        /// Owner gid.
        gid: u32,
    },
    /// Hard-link `name` in `parent` to an existing inode.
    Link {
        /// Parent inode.
        parent: u64,
        /// Entry name.
        name: String,
        /// Target inode (must not be a directory).
        ino: u64,
    },
    /// Remove a name (file/symlink); frees the inode at nlink 0.
    Unlink {
        /// Parent inode.
        parent: u64,
        /// Entry name.
        name: String,
    },
    /// Remove an empty directory.
    Rmdir {
        /// Parent inode.
        parent: u64,
        /// Entry name.
        name: String,
    },
    /// Atomic same-volume rename (one txn; replaces a same-kind empty/file
    /// target per POSIX).
    Rename {
        /// Source parent.
        src_parent: u64,
        /// Source name.
        src_name: String,
        /// Destination parent.
        dst_parent: u64,
        /// Destination name.
        dst_name: String,
    },
    /// Attribute changes; `size` truncates (drops chunks fully beyond the
    /// new size, keeps a straddling chunk sealed).
    SetAttr {
        /// Target inode.
        ino: u64,
        /// New size (truncate/extend).
        size: Option<u64>,
        /// New mode.
        mode: Option<u32>,
        /// New uid.
        uid: Option<u32>,
        /// New gid.
        gid: Option<u32>,
        /// New atime.
        atime: Option<Time>,
        /// New mtime.
        mtime: Option<Time>,
    },
    /// Commit a CoW layout tail: `chunks` replace layout rows from
    /// `first_idx` onward (superseded rows' refcounts drop → GC), and the
    /// file size advances to `new_size`. One txn with the refcount
    /// accounting — this is the write path's commit point (§7.2).
    CommitLayout {
        /// Target inode.
        ino: u64,
        /// Index of the first chunk in `chunks`.
        first_idx: u64,
        /// New chunks (indexes `first_idx + i`).
        chunks: Vec<ChunkRef>,
        /// New logical file size.
        new_size: u64,
        /// Client write-path sequence number: commits apply only when
        /// `seq` exceeds the last applied one, so WAL-replay re-commits are
        /// idempotent (single writer per volume ⇒ one global sequence).
        seq: u64,
    },
    /// Take up to `max` entries from the GC queue (oldest first).
    GcTake {
        /// Maximum entries to return.
        max: u32,
    },
    /// Acknowledge processed GC entries (removed from the queue).
    GcDone {
        /// Queue sequence numbers completed.
        seqs: Vec<u64>,
    },
    /// Re-point a chunk's replica set after re-replication (§7.4 repair):
    /// every layout row holding (chunk_id, version) adopts `new_replicas`.
    RepairChunk {
        /// The chunk being repaired.
        chunk_id: [u8; 16],
        /// Its version (exact-version guard).
        version: u64,
        /// The new replica set.
        new_replicas: Vec<String>,
    },
    /// Insert a snapshot row (S15 adds the checkpoint).
    CreateSnap {
        /// Snapshot name.
        name: String,
    },
    /// Remove a snapshot row.
    DeleteSnap {
        /// Snapshot id.
        id: u64,
    },
    /// Roll back the volume namespace to a snapshot (§9): the apply replays
    /// the checkpoint's tables into the live store in this one transaction —
    /// never a DB-file restore, so the raft log, GC state and the snapshot
    /// catalog are untouched.
    RestoreSnap {
        /// Snapshot id to restore from.
        id: u64,
    },
    /// Enqueue dead chunks for data-node deletion (dedup against the queue;
    /// used when a snapshot delete releases exclusively-referenced chunks).
    GcEnqueue {
        /// Chunks to collect.
        entries: Vec<GcEntry>,
    },
    /// Claim the volume writer role (§4.3 fencing primitive): bumps and
    /// returns the writer epoch. The Registry lease only nominates a writer;
    /// this op is what fences, committed in the volume's own raft group.
    /// CAS enforcement on every mutating op lands with the shared-meta
    /// architecture (post-v0.2).
    ClaimWriter {
        /// Writer identity (for the raft log record).
        client: String,
    },
}

/// Result of an applied [`MetaOp`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OpResult {
    /// No payload.
    None,
    /// A newly allocated inode number.
    Ino(u64),
    /// A newly allocated snapshot id.
    SnapId(u64),
    /// GC entries taken (queue sequence number + entry).
    GcBatch(Vec<(u64, GcEntry)>),
    /// The new writer epoch after a ClaimWriter.
    WriterEpoch(u64),
}

/// Metadata errors (mapped to errno at the FUSE layer).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum MetaError {
    /// Name or inode not found.
    #[error("not found")]
    NotFound,
    /// Name already exists.
    #[error("already exists")]
    Exists,
    /// Directory not empty.
    #[error("directory not empty")]
    NotEmpty,
    /// Expected a directory.
    #[error("not a directory")]
    NotDir,
    /// Expected a non-directory.
    #[error("is a directory")]
    IsDir,
    /// Malformed op (e.g. layout commit leaving a gap).
    #[error("invalid op: {0}")]
    Invalid(String),
    /// redb storage failure.
    #[error("storage: {0}")]
    Storage(String),
    /// bincode codec failure.
    #[error("codec: {0}")]
    Codec(String),
}
