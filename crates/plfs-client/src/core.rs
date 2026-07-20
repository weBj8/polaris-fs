//! Client core (design doc §7.2): the single-writer write path, end to end
//! in-process:
//!
//! ```text
//! write(ino, off, buf):
//!   CoW: sealed chunks are never modified in place — every written chunk
//!   gets a fresh UUIDv7 chunk id; unwritten bytes are read-modify-written
//!   from the effective layout (pending overlay first, then the arena).
//!   Append one WAL record (chunks + layout-commit intent); fsync → return.
//!
//! flush() (fsync(fd) forces it; the background 1 s / 8 MiB flusher is S9+):
//!   Put chunks to replicas (RF=1: local arena; idempotent by chunk_id)
//!   → commit CommitLayout through the volume Raft group (seq-deduped)
//!   → truncate WAL prefix → drain the GC queue (exact-version Delete).
//!
//! replay (open): WAL records re-run through the same flush path; Put is
//! idempotent on (chunk_id, version, crc) and CommitLayout is deduped by
//! the client seq — acknowledged writes survive kill -9.
//! ```
//!
//! Middle-of-file writes: `CommitLayout` supersedes the layout tail from
//! `first_idx`, so a write carries the refs of every later chunk (`tail`)
//! — supersede/re-insert is refcount-neutral for untouched tail chunks.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use plfs_arena::{Arena, ArenaError, ChunkId};
use plfs_meta::raft::MetaRaft;
use plfs_meta::{ChunkRef, MetaError, MetaOp, MetaState, OpResult};
use serde::{Deserialize, Serialize};

use crate::wal::{Wal, WalError};

/// Logical chunk size of the write path: the arena L-slot payload capacity
/// (1 MiB slot − 64-byte header) — one chunk always fits one slot.
pub const CHUNK_SIZE: u64 = (1 << 20) - 64;

/// One WAL record: one `write()` call, its new chunk payloads, and the
/// layout-commit intent. Flushed/replayed as: Put chunks → CommitLayout.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WalRecord {
    seq: u64,
    ino: u64,
    first_idx: u64,
    new_size: u64,
    /// New chunks to Put: (per-chunk version, chunk_id, payload).
    chunks: Vec<(u64, [u8; 16], Vec<u8>)>,
    /// Full commit ref list starting at `first_idx` (in order); the merged
    /// flusher view and the per-write view share it.
    refs: Vec<ChunkRef>,
}

impl WalRecord {
    fn commit_chunks(&self) -> Vec<ChunkRef> {
        self.refs.clone()
    }
}

/// Data-plane destination for chunk Puts/Deletes. Standalone uses the
/// loopback gRPC ChunkStore (design doc §3.1: the exact same code path as
/// cluster mode); `Local` (direct arena) exists for tests and fuzz harnesses.
pub enum ChunkSink {
    /// A local ChunkArena (tests, in-process fuzz).
    Local(Box<Arena>),
    /// A ChunkStore gRPC endpoint (loopback in standalone, remote in S14).
    Grpc {
        /// The endpoint address (recorded as this chunk's replica).
        addr: String,
        /// The ChunkStore client.
        client:
            plfs_common::data::v1::chunk_store_client::ChunkStoreClient<tonic::transport::Channel>,
    },
    /// RF-replicated data plane over registry-discovered data nodes (§7.2).
    Cluster(Box<crate::cluster::ClusterSink>),
}

/// How the data plane is reached when booting a ClientCore.
#[derive(Debug, Clone)]
pub enum SinkConfig {
    /// Open the arena at this path directly.
    Local(std::path::PathBuf),
    /// Connect to a ChunkStore endpoint (e.g. `http://127.0.0.1:9100`).
    Grpc(String),
    /// RF-replicated data plane: discover data nodes via the registry.
    Cluster {
        /// Registry endpoint (host:port).
        registry: String,
        /// Replication factor (2 or 3).
        rf: usize,
    },
}

impl ChunkSink {
    /// Replica addresses for a NEW chunk (placement): Local/Grpc put the
    /// single endpoint; Cluster does rendezvous placement over live data
    /// nodes (design doc §7.2).
    async fn replica_addrs(&mut self, chunk_id: &[u8; 16]) -> Vec<String> {
        match self {
            ChunkSink::Local(_) => vec!["local".to_string()],
            ChunkSink::Grpc { addr, .. } => vec![addr.clone()],
            ChunkSink::Cluster(c) => {
                let _ = c.refresh_live().await;
                c.place(chunk_id)
            }
        }
    }

    /// Quorum Put to `replicas` (ignored by single-endpoint sinks).
    async fn put(
        &mut self,
        id: ChunkId,
        version: u64,
        payload: &[u8],
        replicas: &[String],
    ) -> Result<(), ClientError> {
        match self {
            ChunkSink::Local(arena) => {
                arena.put(id, version, payload)?;
                Ok(())
            }
            ChunkSink::Grpc { client, .. } => {
                let req = plfs_common::data::v1::PutRequest {
                    chunk_id: id.as_bytes().to_vec(),
                    version,
                    crc32c: crc32fast::hash(payload),
                    payload: payload.to_vec(),
                    seal: true,
                };
                client
                    .put(req)
                    .await
                    .map_err(|s| ClientError::Codec(format!("grpc put: {s}")))?;
                Ok(())
            }
            ChunkSink::Cluster(c) => c.put(*id.as_bytes(), version, payload, replicas).await,
        }
    }

    /// Failover Get across `replicas` (ignored by single-endpoint sinks).
    async fn get(
        &mut self,
        id: &ChunkId,
        replicas: &[String],
    ) -> Result<(u64, Vec<u8>), ClientError> {
        match self {
            ChunkSink::Local(arena) => Ok(arena.get(id)?),
            ChunkSink::Grpc { client, .. } => {
                let req = plfs_common::data::v1::GetRequest {
                    chunk_id: id.as_bytes().to_vec(),
                    if_version: 0,
                };
                let mut stream = client
                    .get(req)
                    .await
                    .map_err(|s| grpc_status(&s))?
                    .into_inner();
                let mut version = 0;
                let mut data = Vec::new();
                while let Some(frame) = stream
                    .message()
                    .await
                    .map_err(|s| ClientError::Codec(format!("grpc get stream: {s}")))?
                {
                    version = frame.version;
                    data.extend_from_slice(&frame.data);
                }
                Ok((version, data))
            }
            ChunkSink::Cluster(c) => c.get(*id.as_bytes(), replicas).await,
        }
    }

    /// Delete on all `replicas` (single-endpoint sinks use their endpoint).
    async fn delete(
        &mut self,
        id: &ChunkId,
        version: u64,
        replicas: &[String],
    ) -> Result<(), ClientError> {
        match self {
            ChunkSink::Local(arena) => {
                arena.delete(id, version)?;
                Ok(())
            }
            ChunkSink::Grpc { client, .. } => {
                let req = plfs_common::data::v1::DeleteRequest {
                    chunk_id: id.as_bytes().to_vec(),
                    version,
                };
                client
                    .delete(req)
                    .await
                    .map_err(|s| ClientError::Codec(format!("grpc delete: {s}")))?;
                Ok(())
            }
            ChunkSink::Cluster(c) => c.delete(*id.as_bytes(), version, replicas).await,
        }
    }
}

fn grpc_status(s: &tonic::Status) -> ClientError {
    match s.code() {
        tonic::Code::NotFound => ClientError::Arena(ArenaError::NotFound(ChunkId::from([0; 16]))),
        tonic::Code::ResourceExhausted => ClientError::Arena(ArenaError::OutOfSpace {
            class: plfs_arena::SlotClass::L,
        }),
        _ => ClientError::Codec(format!("grpc: {s}")),
    }
}

/// statfs payload-byte view of the arena.
#[derive(Debug, Clone, Copy)]
pub struct Statfs {
    /// Total payload capacity across both slot classes.
    pub total_bytes: u64,
    /// Currently allocatable payload bytes.
    pub free_bytes: u64,
}

/// Client-core errors.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// Inode not found.
    #[error("not found")]
    NotFound,
    /// Arena failure.
    #[error("arena: {0}")]
    Arena(#[from] ArenaError),
    /// Metadata failure.
    #[error("meta: {0}")]
    Meta(#[from] MetaError),
    /// WAL failure.
    #[error("wal: {0}")]
    Wal(#[from] WalError),
    /// Codec failure.
    #[error("codec: {0}")]
    Codec(String),
}

/// The single-writer client core (§7.2). One per mounted volume.
pub struct ClientCore {
    wal: Wal,
    sink: ChunkSink,
    meta: MetaRaft,
    state: MetaState,
    next_seq: u64,
    pending: BTreeMap<u64, WalRecord>,
    cache: Option<crate::cache::ReadCache>,
    last_chunk: Option<([u8; 16], Vec<u8>)>,
    /// Chunks re-replicated by the background repair worker (§7.4).
    repair_count: std::sync::Arc<std::sync::atomic::AtomicU64>,
    checkpoints: std::sync::Mutex<std::collections::HashMap<u64, MetaState>>,
}

fn volume_paths(dir: &Path) -> (PathBuf, PathBuf, PathBuf) {
    (
        dir.join("arena.img"),
        dir.join("meta.redb"),
        dir.join("wal"),
    )
}

impl ClientCore {
    /// Create a fresh standalone volume (arena + meta + wal) in `dir`.
    /// A previous create interrupted before its first commit (zero-byte
    /// meta store) is wiped and retried; create on a live volume fails.
    pub async fn create(dir: impl AsRef<Path>, arena_bytes: u64) -> Result<Self, ClientError> {
        let dir = dir.as_ref();
        std::fs::create_dir_all(dir).map_err(|e| ClientError::Codec(e.to_string()))?;
        let (arena_path, meta_path, wal_dir) = volume_paths(dir);
        if meta_path.exists()
            && std::fs::metadata(&meta_path)
                .map_err(|e| ClientError::Codec(e.to_string()))?
                .len()
                == 0
        {
            std::fs::remove_file(&meta_path).map_err(|e| ClientError::Codec(e.to_string()))?;
        }
        Arena::mkfs(
            &arena_path,
            &plfs_arena::MkfsConfig {
                total_bytes: arena_bytes,
                l_fraction: 0.90,
                l_slot_size: 1 << 20,
                s_slot_size: 64 << 10,
            },
        )?;
        Self::format_volume(dir, arena_bytes)?;
        let state = MetaState::open(&meta_path)?;
        Self::boot(state, SinkConfig::Local(arena_path), wal_dir).await
    }

    /// Format a fresh volume at `dir` (arena image + metadata store), without
    /// booting — the standalone flow starts the data node between format and
    /// open.
    pub fn format_volume(dir: impl AsRef<Path>, arena_bytes: u64) -> Result<(), ClientError> {
        let dir = dir.as_ref();
        std::fs::create_dir_all(dir).map_err(|e| ClientError::Codec(e.to_string()))?;
        let (arena_path, meta_path, _) = volume_paths(dir);
        if meta_path.exists()
            && std::fs::metadata(&meta_path)
                .map_err(|e| ClientError::Codec(e.to_string()))?
                .len()
                == 0
        {
            std::fs::remove_file(&meta_path).map_err(|e| ClientError::Codec(e.to_string()))?;
        }
        Arena::mkfs(
            &arena_path,
            &plfs_arena::MkfsConfig {
                total_bytes: arena_bytes,
                l_fraction: 0.90,
                l_slot_size: 1 << 20,
                s_slot_size: 64 << 10,
            },
        )?;
        MetaState::create(&meta_path, now())?;
        Ok(())
    }

    /// Format a fresh volume at `dir` with an explicit data-plane sink
    /// (standalone: loopback gRPC, design doc §3.1).
    pub async fn create_with_sink(
        dir: impl AsRef<Path>,
        arena_bytes: u64,
        sink: SinkConfig,
    ) -> Result<Self, ClientError> {
        let dir = dir.as_ref();
        Self::format_volume(dir, arena_bytes)?;
        let (_, meta_path, wal_dir) = volume_paths(dir);
        let state = MetaState::open(&meta_path)?;
        Self::boot(state, sink, wal_dir).await
    }

    /// Open an existing volume: replay the WAL through the flush path
    /// (idempotent Puts + seq-deduped commits), then continue the sequence.
    pub async fn open(dir: impl AsRef<Path>) -> Result<Self, ClientError> {
        let dir = dir.as_ref();
        let (arena_path, meta_path, wal_dir) = volume_paths(dir);
        let state = MetaState::open(&meta_path)?;
        Self::boot(state, SinkConfig::Local(arena_path), wal_dir).await
    }

    /// Open an existing volume with an explicit data-plane sink.
    pub async fn open_with_sink(
        dir: impl AsRef<Path>,
        sink: SinkConfig,
    ) -> Result<Self, ClientError> {
        let dir = dir.as_ref();
        let (_, meta_path, wal_dir) = volume_paths(dir);
        let state = MetaState::open(&meta_path)?;
        Self::boot(state, sink, wal_dir).await
    }

    async fn boot(
        state: MetaState,
        sink: SinkConfig,
        wal_dir: PathBuf,
    ) -> Result<Self, ClientError> {
        let repair_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
        let raft = MetaRaft::bootstrap(&state, 1).await?;
        let sink = match sink {
            SinkConfig::Local(path) => ChunkSink::Local(Box::new(Arena::open(path)?)),
            SinkConfig::Grpc(addr) => ChunkSink::Grpc {
                addr: addr.clone(),
                client: plfs_common::data::v1::chunk_store_client::ChunkStoreClient::connect(addr)
                    .await
                    .map_err(|e| ClientError::Codec(format!("grpc connect: {e}")))?,
            },
            SinkConfig::Cluster { registry, rf } => {
                let mut c = crate::cluster::ClusterSink::new(&registry, rf);
                c.refresh_live().await?;
                let queue = c.degraded_handle();
                let repair_registry = registry.clone();
                let repair_state = state.clone();
                let repair_raft = raft.raft().clone();
                let repair_counter = repair_count.clone();
                tokio::spawn(crate::cluster::repair_loop(
                    queue,
                    repair_registry,
                    repair_state,
                    repair_raft,
                    repair_counter,
                ));
                let scrub_interval = std::env::var("PLFS_SCRUB_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(7 * 24 * 3600);
                tokio::spawn(crate::scrub::scrub_loop(
                    registry.clone(),
                    state.clone(),
                    scrub_interval,
                ));
                ChunkSink::Cluster(Box::new(c))
            }
        };
        let retention_cfg = wal_dir
            .parent()
            .map(|d| d.join("snapshots.toml"))
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|s| toml::from_str::<crate::scheduler::RetentionConfig>(&s).ok());
        if let Some(cfg) = retention_cfg {
            tokio::spawn(crate::scheduler::retention_loop(
                cfg,
                raft.raft().clone(),
                state.clone(),
            ));
        }
        let (wal, replay) = Wal::open(&wal_dir, crate::wal::DEFAULT_SEGMENT_LEN)?;
        let cache_dir = wal_dir.parent().map(std::path::Path::to_path_buf);
        let cache = match &cache_dir {
            Some(dir) => {
                match crate::cache::ReadCache::open(dir, crate::cache::CacheConfig::default()) {
                    Ok(c) => Some(c),
                    Err(e) => {
                        eprintln!("read cache disabled: {e}");
                        None
                    }
                }
            }
            None => None,
        };
        let mut core = ClientCore {
            wal,
            sink,
            meta: raft,
            state,
            next_seq: 1,
            pending: BTreeMap::new(),
            cache,
            last_chunk: None,
            repair_count,
            checkpoints: std::sync::Mutex::new(std::collections::HashMap::new()),
        };
        let mut records = Vec::new();
        let mut max_seq = 0;
        for rec in replay {
            let record: WalRecord = bincode::deserialize(&rec.payload)
                .map_err(|e| ClientError::Codec(e.to_string()))?;
            max_seq = max_seq.max(record.seq);
            records.push(record);
        }
        let merged = merge_records(&core.state, records)?;
        core.flush_records(&merged).await?;
        core.wal.truncate_prefix(core.wal.append_pos())?;
        core.next_seq = core.state.commit_seq()?.max(max_seq) + 1;
        Ok(core)
    }

    /// Write at `offset` (CoW; see module docs). Durable in the WAL when it
    /// returns (§7.2 group commit); `fsync` flushes to replicas + metadata.
    pub async fn write(&mut self, ino: u64, offset: u64, buf: &[u8]) -> Result<usize, ClientError> {
        if buf.is_empty() {
            return Ok(0);
        }
        let (size, layout) = self.effective_layout(ino)?;
        // Dense-prefix layouts cannot express holes: a write past the
        // covered prefix zero-fills from the coverage end (the RMW path
        // turns it into real chunks; all chunks stay full except the last).
        let coverage_end: u64 = layout.iter().map(|(_, c)| c.len).sum();
        let (start, data) = if offset > coverage_end {
            let mut data = vec![0u8; (offset - coverage_end) as usize];
            data.extend_from_slice(buf);
            (coverage_end, data)
        } else {
            (offset, buf.to_vec())
        };
        let data_len = data.len();
        let first_idx = start / CHUNK_SIZE;
        let end = start + data_len as u64;
        let last_idx = (end - 1) / CHUNK_SIZE;
        let mut chunks: Vec<([u8; 16], Vec<u8>)> = Vec::new();
        for idx in first_idx..=last_idx {
            let chunk_start = idx * CHUNK_SIZE;
            let intra = start.saturating_sub(chunk_start) as usize;
            let take = ((end - chunk_start).min(CHUNK_SIZE) - intra as u64) as usize;
            let old = layout
                .iter()
                .find(|(i, _)| *i == idx)
                .map(|(_, c)| c.clone());
            let mut content = match old {
                Some(c) if intra > 0 || take < c.len as usize => {
                    let mut v = self.read_chunk(&c).await?;
                    v.resize(c.len as usize, 0);
                    v
                }
                Some(c)
                    if take < CHUNK_SIZE as usize
                        && (idx != last_idx || end < chunk_start + c.len) =>
                {
                    let mut v = self.read_chunk(&c).await?;
                    v.resize(c.len as usize, 0);
                    v
                }
                _ => Vec::new(),
            };
            let new_len = (intra + take).max(content.len());
            content.resize(new_len, 0);
            content[intra..intra + take]
                .copy_from_slice(&data[(chunk_start + intra as u64 - start) as usize..][..take]);
            chunks.push((ChunkId::new_v7().into(), content));
        }
        let old_tail: Vec<ChunkRef> = layout
            .into_iter()
            .filter(|(i, _)| *i > last_idx)
            .map(|(_, c)| c)
            .collect();
        // One WAL record per chunk (segment-sized): commits apply per chunk
        // (POSIX needs no cross-chunk atomicity), each record's tail carries
        // the refs of this write's newer chunks and the old tail.
        let n = chunks.len();
        let seqs: Vec<u64> = (0..n).map(|i| self.next_seq + i as u64).collect();
        self.next_seq += n as u64;
        let mut new_refs: Vec<ChunkRef> = Vec::with_capacity(chunks.len());
        for (i, (id, payload)) in chunks.iter().enumerate() {
            new_refs.push(ChunkRef {
                chunk_id: *id,
                version: seqs[i],
                len: payload.len() as u64,
                replicas: self.sink.replica_addrs(id).await,
            });
        }
        for (i, (id, payload)) in chunks.into_iter().enumerate() {
            let idx = first_idx + i as u64;
            let refs: Vec<ChunkRef> = std::iter::once(new_refs[i].clone())
                .chain(new_refs[i + 1..].iter().cloned())
                .chain(old_tail.iter().cloned())
                .collect();
            let record = WalRecord {
                seq: seqs[i],
                ino,
                first_idx: idx,
                new_size: size.max(end),
                chunks: vec![(seqs[i], id, payload)],
                refs,
            };
            let bytes =
                bincode::serialize(&record).map_err(|e| ClientError::Codec(e.to_string()))?;
            self.wal.append(&bytes)?;
            self.pending.insert(seqs[i], record);
        }
        self.wal.sync()?;
        Ok(buf.len())
    }

    /// Read `[offset, offset+len)` (clamped to the effective size; sparse
    /// tail reads as zeros).
    pub async fn read(&mut self, ino: u64, offset: u64, len: u64) -> Result<Vec<u8>, ClientError> {
        let (size, layout) = self.effective_layout_range(ino, offset / CHUNK_SIZE)?;
        if offset >= size {
            return Ok(Vec::new());
        }
        let end = (offset + len).min(size);
        let mut out = vec![0u8; (end - offset) as usize];
        let mut base = (offset / CHUNK_SIZE) * CHUNK_SIZE;
        for (_, chunk) in layout {
            let cstart = base;
            let cend = base + chunk.len;
            base = cend;
            if cend <= offset || cstart >= end {
                continue;
            }
            let data = self.read_chunk(&chunk).await?;
            let from = offset.max(cstart) - cstart;
            let to = end.min(cend) - cstart;
            let dst = (cstart.max(offset) - offset) as usize;
            out[dst..dst + (to - from) as usize].copy_from_slice(&data[from as usize..to as usize]);
        }
        Ok(out)
    }

    /// fsync: force a flush of all dirty chunks and wait for the replica
    /// quorum + Raft metadata commit (the §10.1 durability line). Single
    /// writer ⇒ flushing everything is the same line.
    pub async fn fsync(&mut self) -> Result<(), ClientError> {
        self.flush().await
    }

    /// Flush all pending records (Put → CommitLayout → WAL truncate → GC).
    pub async fn flush(&mut self) -> Result<(), ClientError> {
        let records: Vec<WalRecord> = std::mem::take(&mut self.pending).into_values().collect();
        let merged = merge_records(&self.state, records)?;
        self.flush_records(&merged).await?;
        let pos = self.wal.append_pos();
        self.wal.truncate_prefix(pos)?;
        self.gc_drain().await
    }

    async fn flush_records(&mut self, records: &[WalRecord]) -> Result<(), ClientError> {
        // Ascending seq order is required: CommitLayout dedups on the
        // client seq, and a per-ino commit list (BTreeMap order) could
        // otherwise land a high seq first and have later, lower-seq
        // commits silently skipped — losing those files' writes.
        let mut records: Vec<&WalRecord> = records.iter().collect();
        records.sort_by_key(|r| r.seq);
        for rec in records {
            for (version, id, payload) in &rec.chunks {
                if let Err(e) = self
                    .sink
                    .put(
                        ChunkId::from(*id),
                        *version,
                        payload,
                        &rec.refs
                            .first()
                            .map(|r| r.replicas.clone())
                            .unwrap_or_default(),
                    )
                    .await
                {
                    eprintln!(
                        "[fsync-err] put seq={} ino={} idx={}: {e}",
                        rec.seq, rec.ino, rec.first_idx
                    );
                    return Err(e);
                }
            }
            let op = MetaOp::CommitLayout {
                ino: rec.ino,
                first_idx: rec.first_idx,
                chunks: rec.commit_chunks(),
                new_size: rec.new_size,
                seq: rec.seq,
            };
            match self.meta.write(op).await? {
                Err(e) if !matches!(e, MetaError::NotFound) => {
                    eprintln!(
                        "[fsync-err] commit seq={} ino={} idx={}: {e}",
                        rec.seq, rec.ino, rec.first_idx
                    );
                    return Err(ClientError::Meta(e));
                }
                Ok(_) => {}
                Err(MetaError::NotFound) => {
                    // Unlinked before this commit landed: the chunks never
                    // got a refcount, so GC would never reclaim them.
                    for (version, id, _) in &rec.chunks {
                        self.sink
                            .delete(
                                &ChunkId::from(*id),
                                *version,
                                &rec.refs
                                    .first()
                                    .map(|r| r.replicas.clone())
                                    .unwrap_or_default(),
                            )
                            .await?;
                    }
                }
                Err(e) => return Err(ClientError::Meta(e)),
            }
        }
        Ok(())
    }

    /// Drain the GC queue: exact-version Deletes (idempotent re-drive on
    /// crash between Delete and GcDone).
    async fn gc_drain(&mut self) -> Result<(), ClientError> {
        loop {
            let batch = match self.meta.write(MetaOp::GcTake { max: 256 }).await? {
                Ok(OpResult::GcBatch(v)) => v,
                Ok(_) => unreachable!("GcTake returns GcBatch"),
                Err(e) => return Err(ClientError::Meta(e)),
            };
            if batch.is_empty() {
                return Ok(());
            }
            let mut seqs = Vec::new();
            for (seq, entry) in batch {
                // Snapshot-pinned chunks leave the queue with their data
                // intact; S16 re-enqueues them when the pinning snapshot
                // is deleted.
                if !self.chunk_pinned_by_snapshot(&entry.chunk_id)? {
                    self.sink
                        .delete(
                            &ChunkId::from(entry.chunk_id),
                            entry.version,
                            &entry.replicas,
                        )
                        .await?;
                }
                seqs.push(seq);
            }
            match self.meta.write(MetaOp::GcDone { seqs }).await? {
                Ok(_) => {}
                Err(e) => return Err(ClientError::Meta(e)),
            }
        }
    }

    /// Namespace ops go straight through the Raft group (durable when the
    /// call returns; §7.3).
    pub async fn meta_op(&self, op: MetaOp) -> Result<Result<OpResult, MetaError>, ClientError> {
        Ok(self.meta.write(op).await?)
    }

    /// Read-only metadata access (leader-local).
    pub fn state(&self) -> &MetaState {
        &self.state
    }

    /// Create a volume snapshot (design doc §9): fsync first so the
    /// checkpoint carries every acknowledged write.
    pub async fn snapshot_create(&mut self, name: &str) -> Result<u64, ClientError> {
        self.fsync().await?;
        match self
            .meta_op(MetaOp::CreateSnap {
                name: name.to_string(),
            })
            .await?
        {
            Ok(OpResult::SnapId(id)) => Ok(id),
            Ok(other) => Err(ClientError::Codec(format!("CreateSnap: {other:?}"))),
            Err(e) => Err(ClientError::Meta(e)),
        }
    }

    /// Snapshot rows in id order.
    pub fn snapshot_list(&self) -> Result<Vec<plfs_meta::SnapshotMeta>, ClientError> {
        Ok(self.state.list_snaps()?)
    }

    /// Delete a snapshot.
    pub async fn snapshot_delete(&mut self, id: u64) -> Result<(), ClientError> {
        delete_snap_with_reclaim(&self.state, self.meta.raft(), id).await?;
        self.checkpoints.lock().expect("checkpoints").remove(&id);
        self.gc_drain().await
    }

    /// Claim the volume writer epoch (§4.3 fencing primitive).
    pub async fn claim_writer(&self, client: &str) -> Result<u64, ClientError> {
        match self
            .meta_op(MetaOp::ClaimWriter {
                client: client.to_string(),
            })
            .await?
        {
            Ok(OpResult::WriterEpoch(epoch)) => Ok(epoch),
            Ok(other) => Err(ClientError::Codec(format!("ClaimWriter: {other:?}"))),
            Err(e) => Err(ClientError::Meta(e)),
        }
    }

    /// Serve this volume's metadata over gRPC (§8.2): foreign clients read
    /// the namespace through this endpoint (leader-local reads).
    pub fn serve_meta(&self, addr: &str) -> Result<(), ClientError> {
        let svc = plfs_meta::service::MetaOpsSvc::new(
            self.meta.raft().clone(),
            self.state.clone(),
            1,
            std::collections::BTreeMap::from([(1u64, "loopback".to_string())]),
        );
        let addr: std::net::SocketAddr = addr
            .parse()
            .map_err(|e| ClientError::Codec(format!("addr: {e}")))?;
        tokio::spawn(async move {
            if let Err(e) = tonic::transport::Server::builder()
                .add_service(svc.into_server())
                .serve(addr)
                .await
            {
                tracing::warn!("serve_meta {addr}: {e}");
            }
        });
        Ok(())
    }

    /// Roll the volume back to a snapshot (§9, reversible via the implicit
    /// pre-rollback snapshot); returns the pre-rollback snapshot id.
    pub async fn snapshot_rollback(&mut self, snap: u64) -> Result<u64, ClientError> {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let pre = self.snapshot_create(&format!("pre-rollback-{ts}")).await?;
        match self.meta_op(MetaOp::RestoreSnap { id: snap }).await? {
            Ok(_) => Ok(pre),
            Err(e) => Err(ClientError::Meta(e)),
        }
    }

    fn checkpoint(&self, id: u64) -> Result<MetaState, ClientError> {
        let mut map = self.checkpoints.lock().expect("checkpoints");
        if let Some(cp) = map.get(&id) {
            return Ok(cp.clone());
        }
        let cp = self.state.open_checkpoint(id)?;
        map.insert(id, cp.clone());
        Ok(cp)
    }

    /// GC rule (design doc §9): a chunk is collectable only when no
    /// snapshot checkpoint references it anymore.
    fn chunk_pinned_by_snapshot(&self, chunk_id: &[u8; 16]) -> Result<bool, ClientError> {
        pinned_by_any_snapshot(&self.state, chunk_id)
    }

    /// Resolve a directory entry inside a snapshot.
    pub fn snapshot_lookup(
        &self,
        snap: u64,
        parent: u64,
        name: &str,
    ) -> Result<Option<u64>, ClientError> {
        Ok(self.checkpoint(snap)?.lookup(parent, name)?)
    }

    /// Inode attributes inside a snapshot.
    pub fn snapshot_getattr(
        &self,
        snap: u64,
        ino: u64,
    ) -> Result<Option<plfs_meta::Inode>, ClientError> {
        Ok(self.checkpoint(snap)?.getattr(ino)?)
    }

    /// List a directory inside a snapshot.
    pub fn snapshot_listdir(&self, snap: u64, ino: u64) -> Result<Vec<(String, u64)>, ClientError> {
        Ok(self.checkpoint(snap)?.listdir(ino)?)
    }

    /// Read file bytes as of a snapshot; chunk fetches share the live
    /// `(chunk_id, version)` SSD cache (§9: unchanged data is near-free).
    pub async fn snapshot_read(
        &mut self,
        snap: u64,
        ino: u64,
        offset: u64,
        len: u64,
    ) -> Result<Vec<u8>, ClientError> {
        let cp = self.checkpoint(snap)?;
        let size = cp
            .getattr(ino)?
            .ok_or(ClientError::Meta(MetaError::NotFound))?
            .size;
        if offset >= size {
            return Ok(Vec::new());
        }
        let end = (offset + len).min(size);
        let layout = cp.layout_from(ino, offset / CHUNK_SIZE)?;
        let mut out = vec![0u8; (end - offset) as usize];
        let mut base = (offset / CHUNK_SIZE) * CHUNK_SIZE;
        for (_, chunk) in layout {
            let cstart = base;
            let cend = base + chunk.len;
            base = cend;
            if cend <= offset || cstart >= end {
                continue;
            }
            let data = self.read_chunk(&chunk).await?;
            let from = offset.max(cstart) - cstart;
            let to = end.min(cend) - cstart;
            let dst = (cstart.max(offset) - offset) as usize;
            out[dst..dst + (to - from) as usize].copy_from_slice(&data[from as usize..to as usize]);
        }
        Ok(out)
    }

    /// Capacity stats for statfs: total/free payload bytes across both slot
    /// classes (exact per the in-memory bitmap, which open reconciled).
    /// Grpc sinks report elastic sparse capacity until S13 aggregates statfs
    /// through the Registry.
    pub async fn statfs(&self) -> Result<Statfs, ClientError> {
        match &self.sink {
            ChunkSink::Local(arena) => {
                let geom = arena.geometry();
                let l_cap = u64::from(geom.l_slot_size) - 64;
                let s_cap = u64::from(geom.s_slot_size) - 64;
                Ok(Statfs {
                    total_bytes: geom.l_slot_count * l_cap + geom.s_slot_count * s_cap,
                    free_bytes: arena.free_slots(plfs_arena::SlotClass::L) * l_cap
                        + arena.free_slots(plfs_arena::SlotClass::S) * s_cap,
                })
            }
            ChunkSink::Grpc { client, .. } => {
                let mut client = client.clone();
                let mut stream = client
                    .list(plfs_common::data::v1::ListRequest {})
                    .await
                    .map_err(|s| ClientError::Codec(format!("grpc list: {s}")))?
                    .into_inner();
                let mut used = 0u64;
                while let Some(item) = stream
                    .message()
                    .await
                    .map_err(|s| ClientError::Codec(format!("grpc list stream: {s}")))?
                {
                    used += item.len;
                }
                const HEADROOM: u64 = 1 << 40; // sparse backend, elastic
                Ok(Statfs {
                    total_bytes: used + HEADROOM,
                    free_bytes: HEADROOM,
                })
            }
            ChunkSink::Cluster(c) => {
                let (total, used) = c.cluster_stats().await?;
                Ok(Statfs {
                    total_bytes: total,
                    free_bytes: total.saturating_sub(used),
                })
            }
        }
    }

    /// Committed layout ⊕ pending commits (read-your-writes overlay), and
    /// the effective file size.
    /// Committed layout rows from `from_idx` ⊕ pending commits, and the
    /// effective file size — chunks are full except the last, so reads can
    /// start at `offset / CHUNK_SIZE` without the prefix walk.
    fn effective_layout_range(
        &self,
        ino: u64,
        from_idx: u64,
    ) -> Result<(u64, Vec<(u64, ChunkRef)>), ClientError> {
        let inode = self.state.getattr(ino)?.ok_or(ClientError::NotFound)?;
        let mut layout = self.state.layout_from(ino, from_idx)?;
        let mut size = inode.size;
        // Prefilter before cloning: records that can't intersect the range.
        for rec in self
            .pending
            .values()
            .filter(|r| r.ino == ino && r.first_idx + r.chunks.len() as u64 > from_idx)
        {
            layout.retain(|(i, _)| *i < rec.first_idx);
            layout.extend(
                rec.commit_chunks()
                    .into_iter()
                    .enumerate()
                    .map(|(i, c)| (rec.first_idx + i as u64, c)),
            );
            size = size.max(rec.new_size);
        }
        Ok((size, layout))
    }

    /// Committed layout ⊕ pending commits (read-your-writes overlay), and
    /// the effective file size.
    fn effective_layout(&self, ino: u64) -> Result<(u64, Vec<(u64, ChunkRef)>), ClientError> {
        self.effective_layout_range(ino, 0)
    }

    /// Chunk payload: pending overlay first, then the SSD cache, then the
    /// data plane. Cache hits need no validation in the single-writer
    /// model: the layout comes from our own leader-local redb (always
    /// fresh) and sealed chunks are immutable, so a `(chunk_id, version)`
    /// hit from the current layout is valid by construction. The Stat-based
    /// validation machinery in `ReadCache` is for foreign volumes (S18).
    async fn read_chunk(&mut self, chunk: &ChunkRef) -> Result<Vec<u8>, ClientError> {
        for rec in self.pending.values() {
            for (_, id, payload) in &rec.chunks {
                if *id == chunk.chunk_id {
                    return Ok(payload.clone());
                }
            }
        }
        let id = chunk.chunk_id;
        // One-entry MRU payload cache: the kernel reads sequentially in
        // sub-chunk slices, so the last chunk serves the next few reads
        // without another cache-file pass.
        if let Some((last_id, last_data)) = &self.last_chunk
            && *last_id == id
        {
            if let Some(cache) = &self.cache {
                cache.record(true);
            }
            return Ok(last_data.clone());
        }
        let data = if let Some(cache) = &self.cache {
            match cache.probe(&id, chunk.version) {
                Some(bytes) => {
                    cache.record(true);
                    bytes
                }
                None => self.fetch_and_cache(&id, &chunk.replicas).await?,
            }
        } else {
            self.fetch_and_cache(&id, &chunk.replicas).await?
        };
        self.last_chunk = Some((id, data.clone()));
        Ok(data)
    }

    async fn fetch_and_cache(
        &mut self,
        id: &[u8; 16],
        replicas: &[String],
    ) -> Result<Vec<u8>, ClientError> {
        let (version, data) = self.sink.get(&ChunkId::from(*id), replicas).await?;
        if let Some(cache) = &self.cache {
            cache.record(false);
            cache.insert(id, version, &data);
        }
        Ok(data)
    }

    /// Read-cache statistics (hits, misses, used bytes) for the metrics
    /// endpoint and gates.
    pub fn cache_stats(&self) -> Option<crate::cache::CacheStats> {
        self.cache.as_ref().map(|c| c.stats())
    }

    /// Chunks re-replicated by the background repair worker (§7.4).
    pub fn repair_count(&self) -> u64 {
        self.repair_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Run one scrub pass over the cluster data plane (§10.3); `rate`
    /// paces I/O in bytes/sec (0 = unthrottled). Non-cluster sinks hold a
    /// single copy — nothing to compare — so this is a no-op there.
    pub async fn scrub(&mut self, rate: u64) -> Result<crate::scrub::ScrubStats, ClientError> {
        match &mut self.sink {
            ChunkSink::Cluster(c) => crate::scrub::scrub_once(c, &self.state, rate).await,
            _ => Ok(crate::scrub::ScrubStats::default()),
        }
    }

    /// Stop the raft core (before dropping, so the redb handle is freed).
    pub async fn shutdown(&self) -> Result<(), ClientError> {
        self.meta.shutdown().await?;
        Ok(())
    }
}

/// Delete a snapshot and re-enqueue the chunks it exclusively referenced
/// for GC (design doc §9). Shared by the client API and the retention
/// scheduler.
pub(crate) async fn delete_snap_with_reclaim(
    state: &MetaState,
    raft: &openraft::Raft<plfs_meta::raft::MetaRaftConfig>,
    id: u64,
) -> Result<(), ClientError> {
    let chunks = state.open_checkpoint(id)?.chunk_map()?;
    let resp = raft
        .client_write(MetaOp::DeleteSnap { id })
        .await
        .map_err(|e| ClientError::Codec(format!("raft: {e}")))?;
    match resp.data {
        Ok(_) => {}
        Err(e) => return Err(ClientError::Meta(e)),
    }
    let mut entries = Vec::new();
    for (chunk_id, (version, replicas)) in chunks {
        if state.chunk_refcount(&chunk_id)? == 0 && !pinned_by_any_snapshot(state, &chunk_id)? {
            entries.push(plfs_meta::GcEntry {
                chunk_id,
                version,
                replicas,
            });
        }
    }
    if !entries.is_empty() {
        raft.client_write(MetaOp::GcEnqueue { entries })
            .await
            .map_err(|e| ClientError::Codec(format!("raft: {e}")))?;
    }
    Ok(())
}

fn pinned_by_any_snapshot(state: &MetaState, chunk_id: &[u8; 16]) -> Result<bool, ClientError> {
    for snap in state.list_snaps()? {
        if state.open_checkpoint(snap.id)?.chunk_refcount(chunk_id)? > 0 {
            return Ok(true);
        }
    }
    Ok(false)
}

fn now() -> (i64, u32) {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or((0, 0), |d| (d.as_secs() as i64, d.subsec_nanos()))
}

type PendingChunk = (u64, [u8; 16], Vec<u8>, Vec<String>);

/// §7.2 flusher grouping: within one flush, superseded work collapses —
/// only the latest chunk per (ino, idx) is Put, and one CommitLayout per
/// ino covers the whole merged run (gaps filled from the committed layout).
/// Without this, a small-write stream costs one Put + one Raft commit + one
/// GC entry per write instead of per chunk.
fn merge_records(
    state: &MetaState,
    records: Vec<WalRecord>,
) -> Result<Vec<WalRecord>, ClientError> {
    let mut by_ino: BTreeMap<u64, Vec<WalRecord>> = BTreeMap::new();
    for rec in records {
        by_ino.entry(rec.ino).or_default().push(rec);
    }
    by_ino
        .into_values()
        .map(|mut recs| {
            recs.sort_by_key(|r| r.seq);
            let ino = recs[0].ino;
            let first_idx = recs.iter().map(|r| r.first_idx).min().expect("nonempty");
            let new_size = recs.iter().map(|r| r.new_size).max().expect("nonempty");
            let seq = recs.iter().map(|r| r.seq).max().expect("nonempty");
            let committed: BTreeMap<u64, ChunkRef> = state.layout(ino)?.into_iter().collect();
            let mut by_idx: BTreeMap<u64, PendingChunk> = BTreeMap::new();
            for rec in &recs {
                for (version, id, payload) in &rec.chunks {
                    by_idx.insert(
                        rec.first_idx,
                        (
                            *version,
                            *id,
                            payload.clone(),
                            rec.refs
                                .first()
                                .map(|r| r.replicas.clone())
                                .unwrap_or_default(),
                        ),
                    );
                }
            }
            let max_idx = *by_idx.keys().max().expect("nonempty");
            // The tail beyond the merged run comes from the record AT
            // max_idx: its own refs after its chunks. Taking the latest
            // record's tail instead would duplicate refs for indexes that
            // the merge itself covers (out-of-order writes).
            let tail: Vec<ChunkRef> = recs
                .iter()
                .rev()
                .find(|r| r.first_idx == max_idx)
                .map(|r| r.refs[r.chunks.len()..].to_vec())
                .unwrap_or_default();
            let mut chunks: Vec<(u64, [u8; 16], Vec<u8>)> = Vec::new();
            let mut refs: Vec<ChunkRef> = Vec::new();
            for idx in first_idx..=max_idx {
                match by_idx.get(&idx) {
                    Some((version, id, payload, replicas)) => {
                        refs.push(ChunkRef {
                            chunk_id: *id,
                            version: *version,
                            len: payload.len() as u64,
                            replicas: replicas.clone(),
                        });
                        chunks.push((*version, *id, payload.clone()));
                    }
                    None => {
                        refs.push(committed.get(&idx).ok_or(ClientError::NotFound)?.clone());
                    }
                }
            }
            refs.extend(tail);
            Ok(WalRecord {
                seq,
                ino,
                first_idx,
                new_size,
                chunks,
                refs,
            })
        })
        .collect()
}
