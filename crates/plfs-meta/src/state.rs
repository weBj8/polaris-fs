//! The redb-backed metadata state machine: tables, per-op transactions, and
//! read helpers. Table map (all keys big-endian for scan order):
//!
//! - `inodes`: ino u64 → bincode(Inode)
//! - `dentries`: parent_be(8) ++ name → child ino u64
//! - `layouts`: ino_be(8) ++ idx_be(8) → bincode(ChunkRef)
//! - `chunkrefs`: chunk_id(16) → refcount u64
//! - `snaps`: snap id u64 → bincode(SnapshotMeta)
//! - `gc`: seq u64 → bincode(GcEntry)
//! - `counters`: name &str → next value u64 (ino/snap/gc_seq)

use std::path::{Path, PathBuf};
use std::sync::Arc;

use redb::{Database, ReadableTable, ReadableTableMetadata, Table, TableDefinition};

use crate::{ChunkRef, GcEntry, Inode, Kind, MetaError, MetaOp, OpResult, SnapshotMeta, Time};

/// Root directory inode number.
pub const ROOT_INO: u64 = 1;

/// chunk_id → (version, replicas) reachability map (rollback diff, GC).
pub type ChunkMap = std::collections::HashMap<[u8; 16], (u64, Vec<String>)>;

const INODES: TableDefinition<u64, &[u8]> = TableDefinition::new("inodes");
const DENTRIES: TableDefinition<&[u8], u64> = TableDefinition::new("dentries");
const LAYOUTS: TableDefinition<&[u8], &[u8]> = TableDefinition::new("layouts");
const CHUNKREFS: TableDefinition<&[u8], u64> = TableDefinition::new("chunkrefs");
const SNAPS: TableDefinition<u64, &[u8]> = TableDefinition::new("snaps");
const GC: TableDefinition<u64, &[u8]> = TableDefinition::new("gc");
const COUNTERS: TableDefinition<&str, u64> = TableDefinition::new("counters");

fn storage(err: impl std::fmt::Display) -> MetaError {
    MetaError::Storage(err.to_string())
}

fn dkey(parent: u64, name: &str) -> Vec<u8> {
    let mut k = parent.to_be_bytes().to_vec();
    k.extend_from_slice(name.as_bytes());
    k
}

fn lkey(ino: u64, idx: u64) -> Vec<u8> {
    let mut k = ino.to_be_bytes().to_vec();
    k.extend_from_slice(&idx.to_be_bytes());
    k
}

fn enc<T: serde::Serialize>(v: &T) -> Result<Vec<u8>, MetaError> {
    bincode::serialize(v).map_err(|e| MetaError::Codec(e.to_string()))
}

fn dec<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, MetaError> {
    bincode::deserialize(bytes).map_err(|e| MetaError::Codec(e.to_string()))
}

// Read helpers, generic over the table type so read and write transactions
// share them.

fn read_inode(t: &impl ReadableTable<u64, &'static [u8]>, ino: u64) -> Result<Inode, MetaError> {
    match t.get(ino).map_err(storage)? {
        Some(v) => Ok(dec(v.value())?),
        None => Err(MetaError::NotFound),
    }
}

fn read_dentry(
    t: &impl ReadableTable<&'static [u8], u64>,
    parent: u64,
    name: &str,
) -> Result<Option<u64>, MetaError> {
    Ok(t.get(dkey(parent, name).as_slice())
        .map_err(storage)?
        .map(|v| v.value()))
}

fn scan_dentries(
    t: &impl ReadableTable<&'static [u8], u64>,
    ino: u64,
) -> Result<Vec<(String, u64)>, MetaError> {
    let prefix = ino.to_be_bytes();
    let mut out = Vec::new();
    for row in t.range(prefix.as_slice()..).map_err(storage)? {
        let (k, v) = row.map_err(storage)?;
        let key = k.value();
        if !key.starts_with(&prefix) {
            break;
        }
        let name = String::from_utf8(key[8..].to_vec())
            .map_err(|_| MetaError::Codec("non-utf8 name".into()))?;
        out.push((name, v.value()));
    }
    Ok(out)
}

fn scan_layout(
    t: &impl ReadableTable<&'static [u8], &'static [u8]>,
    ino: u64,
) -> Result<Vec<(u64, ChunkRef)>, MetaError> {
    let prefix = ino.to_be_bytes();
    let mut out = Vec::new();
    for row in t.range(prefix.as_slice()..).map_err(storage)? {
        let (k, v) = row.map_err(storage)?;
        let key = k.value();
        if !key.starts_with(&prefix) {
            break;
        }
        let idx = u64::from_be_bytes(
            key[8..16]
                .try_into()
                .map_err(|_| MetaError::Codec("bad layout key".into()))?,
        );
        out.push((idx, dec::<ChunkRef>(v.value())?));
    }
    Ok(out)
}

fn bump(counters: &mut Table<'_, &str, u64>, name: &'static str) -> Result<u64, MetaError> {
    let cur = counters
        .get(name)
        .map_err(storage)?
        .map_or(0, |v| v.value());
    counters.insert(name, cur + 1).map_err(storage)?;
    Ok(cur)
}

fn inc_ref(chunkrefs: &mut Table<'_, &[u8], u64>, chunk: &ChunkRef) -> Result<(), MetaError> {
    let cur = chunkrefs
        .get(chunk.chunk_id.as_slice())
        .map_err(storage)?
        .map_or(0, |v| v.value());
    chunkrefs
        .insert(chunk.chunk_id.as_slice(), cur + 1)
        .map_err(storage)?;
    Ok(())
}

fn dec_ref(
    chunkrefs: &mut Table<'_, &[u8], u64>,
    gc: &mut Table<'_, u64, &[u8]>,
    counters: &mut Table<'_, &str, u64>,
    chunk: &ChunkRef,
) -> Result<(), MetaError> {
    let cur = chunkrefs
        .get(chunk.chunk_id.as_slice())
        .map_err(storage)?
        .map_or(0, |v| v.value());
    if cur <= 1 {
        chunkrefs
            .remove(chunk.chunk_id.as_slice())
            .map_err(storage)?;
        let seq = bump(counters, "next_gc_seq")?;
        gc.insert(
            seq,
            enc(&GcEntry {
                chunk_id: chunk.chunk_id,
                version: chunk.version,
                replicas: chunk.replicas.clone(),
            })?
            .as_slice(),
        )
        .map_err(storage)?;
    } else {
        chunkrefs
            .insert(chunk.chunk_id.as_slice(), cur - 1)
            .map_err(storage)?;
    }
    Ok(())
}

/// The metadata state machine for one volume (design doc §7.3). All
/// mutations go through [`MetaState::apply_at`] — one redb write transaction
/// per op, so multi-table ops (rename, layout commit + refcounts) are
/// atomic by construction.
#[derive(Clone)]
pub struct MetaState {
    db: Arc<Database>,
    db_path: PathBuf,
}

impl MetaState {
    /// Create a fresh metadata store with the root directory at
    /// [`ROOT_INO`].
    pub fn create(path: impl AsRef<Path>, now: Time) -> Result<Self, MetaError> {
        let path = path.as_ref();
        let db = Arc::new(Database::create(path).map_err(storage)?);
        let state = Self {
            db,
            db_path: path.to_path_buf(),
        };
        let txn = state.db.begin_write().map_err(storage)?;
        {
            let mut inodes = txn.open_table(INODES).map_err(storage)?;
            inodes
                .insert(
                    ROOT_INO,
                    enc(&Inode {
                        kind: Kind::Dir,
                        mode: 0o755,
                        uid: 0,
                        gid: 0,
                        size: 0,
                        atime: now,
                        mtime: now,
                        ctime: now,
                        nlink: 2,
                        symlink_target: None,
                    })?
                    .as_slice(),
                )
                .map_err(storage)?;
            let mut counters = txn.open_table(COUNTERS).map_err(storage)?;
            counters.insert("next_ino", ROOT_INO + 1).map_err(storage)?;
            counters.insert("next_snap", 1u64).map_err(storage)?;
            counters.insert("next_gc_seq", 1u64).map_err(storage)?;
            // Create the remaining tables so read paths never race a
            // missing table on a fresh store.
            txn.open_table(DENTRIES).map_err(storage)?;
            txn.open_table(LAYOUTS).map_err(storage)?;
            txn.open_table(CHUNKREFS).map_err(storage)?;
            txn.open_table(SNAPS).map_err(storage)?;
            txn.open_table(GC).map_err(storage)?;
        }
        txn.commit().map_err(storage)?;
        Ok(state)
    }

    /// Open an existing metadata store.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, MetaError> {
        let path = path.as_ref();
        Ok(Self {
            db: Arc::new(Database::open(path).map_err(storage)?),
            db_path: path.to_path_buf(),
        })
    }

    /// Wrap an already-open redb handle (the Raft storage impls share the
    /// state machine's database — one file, one lock).
    pub(crate) fn with_db(db: Arc<Database>, db_path: PathBuf) -> Self {
        Self { db, db_path }
    }

    /// Clone the underlying redb handle (single lock per file).
    pub(crate) fn db_handle(&self) -> Arc<Database> {
        self.db.clone()
    }

    /// The metadata store file this state runs on.
    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    /// Directory holding snapshot checkpoint copies (`<volume>/snaps`).
    pub fn snaps_dir(&self) -> PathBuf {
        self.db_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("snaps")
    }

    /// The checkpoint file for snapshot `id`.
    pub fn checkpoint_path(&self, id: u64) -> PathBuf {
        self.snaps_dir().join(format!("{id}.redb"))
    }

    /// O(1) snapshot content (design doc §9): byte-copy the store between
    /// transactions. The held write txn blocks concurrent raft-log commits
    /// for the duration — without it a log append landing mid-copy could
    /// tear the image. Called from the raft apply loop right after the
    /// CreateSnap row commits; sequential applies are the write-barrier.
    pub fn checkpoint_copy(&self, id: u64) -> Result<(), MetaError> {
        let dir = self.snaps_dir();
        std::fs::create_dir_all(&dir).map_err(storage)?;
        {
            let _barrier = self.db.begin_write().map_err(storage)?;
            std::fs::copy(&self.db_path, self.checkpoint_path(id)).map_err(storage)?;
        }
        let copy = std::fs::File::open(self.checkpoint_path(id)).map_err(storage)?;
        copy.sync_all().map_err(storage)?;
        drop(copy);
        std::fs::File::open(&dir)
            .map_err(storage)?
            .sync_all()
            .map_err(storage)?;
        Ok(())
    }

    /// Remove the checkpoint file; a missing file is not an error.
    pub fn delete_checkpoint_file(&self, id: u64) -> Result<(), MetaError> {
        match std::fs::remove_file(self.checkpoint_path(id)) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(storage(e)),
        }
    }

    /// Open a snapshot checkpoint for read-only `.snapshots` resolution.
    pub fn open_checkpoint(&self, id: u64) -> Result<Self, MetaError> {
        Self::open(self.checkpoint_path(id))
    }

    /// Apply a Raft-committed op, recording the applied log id in the SAME
    /// transaction. The log id advances even when the op fails semantically
    /// (the error is the response, not a re-apply reason).
    pub(crate) fn apply_logged(
        &mut self,
        op: &MetaOp,
        index: u64,
        term: u64,
        node_id: u64,
        now: Time,
    ) -> Result<Result<OpResult, MetaError>, MetaError> {
        let txn = self.db.begin_write().map_err(storage)?;
        let result = self.apply_in(&txn, op, now);
        {
            let mut kv = txn.open_table(crate::raft::RAFT_KV).map_err(storage)?;
            kv.insert(
                "last_applied",
                crate::raft::enc_bytes(&(index, term, node_id))?.as_slice(),
            )
            .map_err(storage)?;
        }
        txn.commit().map_err(storage)?;
        Ok(result)
    }

    /// Apply one op with the wall clock (see [`MetaState::apply_at`]).
    pub fn apply(&mut self, op: &MetaOp) -> Result<OpResult, MetaError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or((0, 0), |d| (d.as_secs() as i64, d.subsec_nanos()));
        self.apply_at(op, now)
    }

    /// Apply one op in a single redb write transaction.
    pub fn apply_at(&mut self, op: &MetaOp, now: Time) -> Result<OpResult, MetaError> {
        let txn = self.db.begin_write().map_err(storage)?;
        let result = self.apply_in(&txn, op, now);
        match result {
            Ok(r) => {
                txn.commit().map_err(storage)?;
                Ok(r)
            }
            Err(e) => Err(e), // txn dropped ⇒ rolled back
        }
    }

    pub(crate) fn apply_in(
        &self,
        txn: &redb::WriteTransaction,
        op: &MetaOp,
        now: Time,
    ) -> Result<OpResult, MetaError> {
        match op {
            MetaOp::Mkdir {
                parent,
                name,
                mode,
                uid,
                gid,
            } => {
                let ino = self.alloc_ino(txn)?;
                self.insert_dentry(txn, *parent, name, ino, now)?;
                self.put_inode(
                    txn,
                    ino,
                    &Inode {
                        kind: Kind::Dir,
                        mode: *mode,
                        uid: *uid,
                        gid: *gid,
                        size: 0,
                        atime: now,
                        mtime: now,
                        ctime: now,
                        nlink: 2,
                        symlink_target: None,
                    },
                )?;
                self.bump_nlink(txn, *parent, 1, now)?;
                Ok(OpResult::Ino(ino))
            }
            MetaOp::CreateFile {
                parent,
                name,
                mode,
                uid,
                gid,
            } => {
                let ino = self.alloc_ino(txn)?;
                self.insert_dentry(txn, *parent, name, ino, now)?;
                self.put_inode(
                    txn,
                    ino,
                    &Inode {
                        kind: Kind::File,
                        mode: *mode,
                        uid: *uid,
                        gid: *gid,
                        size: 0,
                        atime: now,
                        mtime: now,
                        ctime: now,
                        nlink: 1,
                        symlink_target: None,
                    },
                )?;
                Ok(OpResult::Ino(ino))
            }
            MetaOp::Symlink {
                parent,
                name,
                target,
                uid,
                gid,
            } => {
                let ino = self.alloc_ino(txn)?;
                self.insert_dentry(txn, *parent, name, ino, now)?;
                self.put_inode(
                    txn,
                    ino,
                    &Inode {
                        kind: Kind::Symlink,
                        mode: 0o777,
                        uid: *uid,
                        gid: *gid,
                        size: target.len() as u64,
                        atime: now,
                        mtime: now,
                        ctime: now,
                        nlink: 1,
                        symlink_target: Some(target.clone()),
                    },
                )?;
                Ok(OpResult::Ino(ino))
            }
            MetaOp::Link { parent, name, ino } => {
                let mut target = self.get_inode(txn, *ino)?;
                if target.kind == Kind::Dir {
                    return Err(MetaError::IsDir);
                }
                self.insert_dentry(txn, *parent, name, *ino, now)?;
                target.nlink += 1;
                target.ctime = now;
                self.put_inode(txn, *ino, &target)?;
                Ok(OpResult::None)
            }
            MetaOp::Unlink { parent, name } => {
                let ino = self.take_dentry(txn, *parent, name)?;
                let mut target = self.get_inode(txn, ino)?;
                if target.kind == Kind::Dir {
                    return Err(MetaError::IsDir);
                }
                target.nlink -= 1;
                target.ctime = now;
                if target.nlink == 0 {
                    self.remove_inode(txn, ino)?;
                } else {
                    self.put_inode(txn, ino, &target)?;
                }
                Ok(OpResult::None)
            }
            MetaOp::Rmdir { parent, name } => {
                let ino = self.take_dentry(txn, *parent, name)?;
                let target = self.get_inode(txn, ino)?;
                if target.kind != Kind::Dir {
                    return Err(MetaError::NotDir);
                }
                if !scan_dentries(&txn.open_table(DENTRIES).map_err(storage)?, ino)?.is_empty() {
                    return Err(MetaError::NotEmpty);
                }
                let mut inodes = txn.open_table(INODES).map_err(storage)?;
                inodes.remove(ino).map_err(storage)?;
                drop(inodes);
                self.bump_nlink(txn, *parent, -1, now)?;
                Ok(OpResult::None)
            }
            MetaOp::Rename {
                src_parent,
                src_name,
                dst_parent,
                dst_name,
            } => {
                let src_ino = read_dentry(
                    &txn.open_table(DENTRIES).map_err(storage)?,
                    *src_parent,
                    src_name,
                )?
                .ok_or(MetaError::NotFound)?;
                let src = self.get_inode(txn, src_ino)?;
                let dst_ino = read_dentry(
                    &txn.open_table(DENTRIES).map_err(storage)?,
                    *dst_parent,
                    dst_name,
                )?;
                if let Some(dst_ino) = dst_ino {
                    if dst_ino == src_ino {
                        return Ok(OpResult::None); // rename onto itself
                    }
                    let dst = self.get_inode(txn, dst_ino)?;
                    match (src.kind == Kind::Dir, dst.kind == Kind::Dir) {
                        (true, false) => return Err(MetaError::NotDir),
                        (false, true) => return Err(MetaError::IsDir),
                        (true, true) => {
                            let empty = scan_dentries(
                                &txn.open_table(DENTRIES).map_err(storage)?,
                                dst_ino,
                            )?
                            .is_empty();
                            if !empty {
                                return Err(MetaError::NotEmpty);
                            }
                            self.remove_dentry(txn, *dst_parent, dst_name)?;
                            let mut inodes = txn.open_table(INODES).map_err(storage)?;
                            inodes.remove(dst_ino).map_err(storage)?;
                            drop(inodes);
                            self.bump_nlink(txn, *dst_parent, -1, now)?;
                        }
                        (false, false) => {
                            self.remove_dentry(txn, *dst_parent, dst_name)?;
                            let mut dst = dst;
                            dst.nlink -= 1;
                            if dst.nlink == 0 {
                                self.remove_inode(txn, dst_ino)?;
                            } else {
                                dst.ctime = now;
                                self.put_inode(txn, dst_ino, &dst)?;
                            }
                        }
                    }
                }
                self.remove_dentry(txn, *src_parent, src_name)?;
                let mut dentries = txn.open_table(DENTRIES).map_err(storage)?;
                dentries
                    .insert(dkey(*dst_parent, dst_name).as_slice(), src_ino)
                    .map_err(storage)?;
                drop(dentries);
                if src.kind == Kind::Dir && src_parent != dst_parent {
                    self.bump_nlink(txn, *src_parent, -1, now)?;
                    self.bump_nlink(txn, *dst_parent, 1, now)?;
                }
                Ok(OpResult::None)
            }
            MetaOp::SetAttr {
                ino,
                size,
                mode,
                uid,
                gid,
                atime,
                mtime,
            } => {
                let mut target = self.get_inode(txn, *ino)?;
                if let Some(size) = size {
                    if *size < target.size && target.kind == Kind::File {
                        self.truncate_layout(txn, *ino, *size)?;
                    }
                    target.size = *size;
                    target.mtime = now;
                }
                if let Some(mode) = mode {
                    target.mode = *mode;
                }
                if let Some(uid) = uid {
                    target.uid = *uid;
                }
                if let Some(gid) = gid {
                    target.gid = *gid;
                }
                if let Some(atime) = atime {
                    target.atime = *atime;
                }
                if let Some(mtime) = mtime {
                    target.mtime = *mtime;
                }
                target.ctime = now;
                self.put_inode(txn, *ino, &target)?;
                Ok(OpResult::None)
            }
            MetaOp::CommitLayout {
                ino,
                first_idx,
                chunks,
                new_size,
                seq,
            } => {
                {
                    let mut counters = txn.open_table(COUNTERS).map_err(storage)?;
                    let last = counters
                        .get("commit_seq")
                        .map_err(storage)?
                        .map_or(0, |v| v.value());
                    if *seq <= last {
                        return Ok(OpResult::None); // replayed duplicate: skip
                    }
                    counters.insert("commit_seq", *seq).map_err(storage)?;
                }
                let mut target = self.get_inode(txn, *ino)?;
                if target.kind != Kind::File {
                    return Err(MetaError::IsDir);
                }
                let existing = scan_layout(&txn.open_table(LAYOUTS).map_err(storage)?, *ino)?;
                if *first_idx > existing.len() as u64 {
                    return Err(MetaError::Invalid("layout commit leaves a gap".into()));
                }
                {
                    let mut layouts = txn.open_table(LAYOUTS).map_err(storage)?;
                    let mut chunkrefs = txn.open_table(CHUNKREFS).map_err(storage)?;
                    let mut gc = txn.open_table(GC).map_err(storage)?;
                    let mut counters = txn.open_table(COUNTERS).map_err(storage)?;
                    // Net refcount accounting per chunk_id: superseded rows
                    // and re-committed tail refs cancel out — only chunks
                    // with a negative NET delta may reach zero (and be
                    // GC-queued); a transient zero would wrongly queue a
                    // live chunk for deletion.
                    let mut net: std::collections::BTreeMap<[u8; 16], (i64, ChunkRef)> =
                        std::collections::BTreeMap::new();
                    for (idx, old) in existing.iter().skip(*first_idx as usize) {
                        layouts
                            .remove(lkey(*ino, *idx).as_slice())
                            .map_err(storage)?;
                        net.entry(old.chunk_id).or_insert((0, old.clone())).0 -= 1;
                    }
                    for (i, chunk) in chunks.iter().enumerate() {
                        layouts
                            .insert(
                                lkey(*ino, first_idx + i as u64).as_slice(),
                                enc(chunk)?.as_slice(),
                            )
                            .map_err(storage)?;
                        net.entry(chunk.chunk_id).or_insert((0, chunk.clone())).0 += 1;
                    }
                    for (delta, chunk) in net.into_values() {
                        if delta < 0 {
                            for _ in 0..-delta {
                                dec_ref(&mut chunkrefs, &mut gc, &mut counters, &chunk)?;
                            }
                        } else {
                            for _ in 0..delta {
                                inc_ref(&mut chunkrefs, &chunk)?;
                            }
                        }
                    }
                }
                target.size = *new_size;
                target.mtime = now;
                target.ctime = now;
                self.put_inode(txn, *ino, &target)?;
                Ok(OpResult::None)
            }
            MetaOp::GcTake { max } => {
                let gc = txn.open_table(GC).map_err(storage)?;
                let mut out = Vec::new();
                for row in gc.iter().map_err(storage)?.take(*max as usize) {
                    let (seq, bytes) = row.map_err(storage)?;
                    out.push((seq.value(), dec::<GcEntry>(bytes.value())?));
                }
                Ok(OpResult::GcBatch(out))
            }
            MetaOp::GcDone { seqs } => {
                let mut gc = txn.open_table(GC).map_err(storage)?;
                for seq in seqs {
                    gc.remove(*seq).map_err(storage)?;
                }
                Ok(OpResult::None)
            }
            MetaOp::CreateSnap { name } => {
                let id = {
                    let mut counters = txn.open_table(COUNTERS).map_err(storage)?;
                    bump(&mut counters, "next_snap")?
                };
                let mut snaps = txn.open_table(SNAPS).map_err(storage)?;
                snaps
                    .insert(
                        id,
                        enc(&SnapshotMeta {
                            id,
                            name: name.clone(),
                            created_at: now,
                        })?
                        .as_slice(),
                    )
                    .map_err(storage)?;
                Ok(OpResult::SnapId(id))
            }
            MetaOp::RepairChunk {
                chunk_id,
                version,
                new_replicas,
            } => {
                let inos = txn
                    .open_table(INODES)
                    .map_err(storage)?
                    .iter()
                    .map_err(storage)?
                    .filter_map(|r| r.ok().map(|(k, _)| k.value()))
                    .collect::<Vec<_>>();
                let mut layouts = txn.open_table(LAYOUTS).map_err(storage)?;
                for ino in inos {
                    let rows = scan_layout(&txn.open_table(LAYOUTS).map_err(storage)?, ino)?;
                    for (idx, mut chunk) in rows {
                        if chunk.chunk_id == *chunk_id && chunk.version == *version {
                            chunk.replicas = new_replicas.clone();
                            layouts
                                .insert(lkey(ino, idx).as_slice(), enc(&chunk)?.as_slice())
                                .map_err(storage)?;
                        }
                    }
                }
                Ok(OpResult::None)
            }
            MetaOp::DeleteSnap { id } => {
                let mut snaps = txn.open_table(SNAPS).map_err(storage)?;
                if snaps.remove(*id).map_err(storage)?.is_none() {
                    return Err(MetaError::NotFound);
                }
                Ok(OpResult::None)
            }
            MetaOp::RestoreSnap { id } => {
                let cp = self.open_checkpoint(*id)?;
                // This read txn sees the last commit — the pre-rollback
                // namespace — which is exactly what the orphan diff needs.
                let live_chunks = self.chunk_map()?;
                let rtx = cp.db.begin_read().map_err(storage)?;
                let mut inodes = Vec::new();
                for row in rtx
                    .open_table(INODES)
                    .map_err(storage)?
                    .iter()
                    .map_err(storage)?
                {
                    let (k, v) = row.map_err(storage)?;
                    inodes.push((k.value(), v.value().to_vec()));
                }
                let mut dentries = Vec::new();
                for row in rtx
                    .open_table(DENTRIES)
                    .map_err(storage)?
                    .iter()
                    .map_err(storage)?
                {
                    let (k, v) = row.map_err(storage)?;
                    dentries.push((k.value().to_vec(), v.value()));
                }
                let mut layouts = Vec::new();
                for row in rtx
                    .open_table(LAYOUTS)
                    .map_err(storage)?
                    .iter()
                    .map_err(storage)?
                {
                    let (k, v) = row.map_err(storage)?;
                    layouts.push((k.value().to_vec(), v.value().to_vec()));
                }
                let mut chunkrefs = Vec::new();
                for row in rtx
                    .open_table(CHUNKREFS)
                    .map_err(storage)?
                    .iter()
                    .map_err(storage)?
                {
                    let (k, v) = row.map_err(storage)?;
                    chunkrefs.push((k.value().to_vec(), v.value()));
                }
                let (cp_next_ino, cp_next_snap) = {
                    let counters = rtx.open_table(COUNTERS).map_err(storage)?;
                    (
                        counters
                            .get("next_ino")
                            .map_err(storage)?
                            .map_or(ROOT_INO + 1, |v| v.value()),
                        counters
                            .get("next_snap")
                            .map_err(storage)?
                            .map_or(1, |v| v.value()),
                    )
                };
                drop(rtx);
                let cp_refs: std::collections::HashSet<[u8; 16]> = chunkrefs
                    .iter()
                    .map(|(k, _)| {
                        <[u8; 16]>::try_from(k.as_slice())
                            .map_err(|_| MetaError::Codec("bad chunkref key".into()))
                    })
                    .collect::<Result<_, _>>()?;
                {
                    let mut t = txn.open_table(INODES).map_err(storage)?;
                    let keys: Vec<u64> = t
                        .iter()
                        .map_err(storage)?
                        .filter_map(|r| r.ok().map(|(k, _)| k.value()))
                        .collect();
                    for k in keys {
                        t.remove(k).map_err(storage)?;
                    }
                    for (k, v) in inodes {
                        t.insert(k, v.as_slice()).map_err(storage)?;
                    }
                }
                {
                    let mut t = txn.open_table(DENTRIES).map_err(storage)?;
                    let keys: Vec<Vec<u8>> = t
                        .iter()
                        .map_err(storage)?
                        .filter_map(|r| r.ok().map(|(k, _)| k.value().to_vec()))
                        .collect();
                    for k in keys {
                        t.remove(k.as_slice()).map_err(storage)?;
                    }
                    for (k, v) in dentries {
                        t.insert(k.as_slice(), v).map_err(storage)?;
                    }
                }
                {
                    let mut t = txn.open_table(LAYOUTS).map_err(storage)?;
                    let keys: Vec<Vec<u8>> = t
                        .iter()
                        .map_err(storage)?
                        .filter_map(|r| r.ok().map(|(k, _)| k.value().to_vec()))
                        .collect();
                    for k in keys {
                        t.remove(k.as_slice()).map_err(storage)?;
                    }
                    for (k, v) in layouts {
                        t.insert(k.as_slice(), v.as_slice()).map_err(storage)?;
                    }
                }
                {
                    let mut t = txn.open_table(CHUNKREFS).map_err(storage)?;
                    let keys: Vec<Vec<u8>> = t
                        .iter()
                        .map_err(storage)?
                        .filter_map(|r| r.ok().map(|(k, _)| k.value().to_vec()))
                        .collect();
                    for k in keys {
                        t.remove(k.as_slice()).map_err(storage)?;
                    }
                    for (k, v) in chunkrefs {
                        t.insert(k.as_slice(), v).map_err(storage)?;
                    }
                }
                // Counters never move backwards: snap ids name checkpoint
                // files (reuse would clobber one), and a reused ino could
                // collide with entries in the other snapshots.
                {
                    let mut counters = txn.open_table(COUNTERS).map_err(storage)?;
                    let live_ino = counters
                        .get("next_ino")
                        .map_err(storage)?
                        .map_or(ROOT_INO + 1, |v| v.value());
                    counters
                        .insert("next_ino", live_ino.max(cp_next_ino))
                        .map_err(storage)?;
                    let live_snap = counters
                        .get("next_snap")
                        .map_err(storage)?
                        .map_or(1, |v| v.value());
                    counters
                        .insert("next_snap", live_snap.max(cp_next_snap))
                        .map_err(storage)?;
                }
                {
                    let mut gc = txn.open_table(GC).map_err(storage)?;
                    let mut counters = txn.open_table(COUNTERS).map_err(storage)?;
                    for (chunk_id, (version, replicas)) in live_chunks {
                        if !cp_refs.contains(&chunk_id) {
                            let seq = bump(&mut counters, "next_gc_seq")?;
                            gc.insert(
                                seq,
                                enc(&GcEntry {
                                    chunk_id,
                                    version,
                                    replicas,
                                })?
                                .as_slice(),
                            )
                            .map_err(storage)?;
                        }
                    }
                }
                Ok(OpResult::None)
            }
            MetaOp::GcEnqueue { entries } => {
                let mut gc = txn.open_table(GC).map_err(storage)?;
                let mut queued = std::collections::HashSet::new();
                for row in gc.iter().map_err(storage)? {
                    let (_, v) = row.map_err(storage)?;
                    queued.insert(dec::<GcEntry>(v.value())?.chunk_id);
                }
                let mut counters = txn.open_table(COUNTERS).map_err(storage)?;
                for entry in entries {
                    if queued.insert(entry.chunk_id) {
                        let seq = bump(&mut counters, "next_gc_seq")?;
                        gc.insert(seq, enc(entry)?.as_slice()).map_err(storage)?;
                    }
                }
                Ok(OpResult::None)
            }
        }
    }

    // ---- public read helpers (leader-local; S8 proxies followers) ----

    /// Resolve a directory entry.
    pub fn lookup(&self, parent: u64, name: &str) -> Result<Option<u64>, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        read_dentry(&txn.open_table(DENTRIES).map_err(storage)?, parent, name)
    }

    /// Fetch inode attributes.
    pub fn getattr(&self, ino: u64) -> Result<Option<Inode>, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        match txn
            .open_table(INODES)
            .map_err(storage)?
            .get(ino)
            .map_err(storage)?
        {
            Some(v) => Ok(Some(dec(v.value())?)),
            None => Ok(None),
        }
    }

    /// List a directory (name, child ino) in byte order.
    pub fn listdir(&self, ino: u64) -> Result<Vec<(String, u64)>, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        scan_dentries(&txn.open_table(DENTRIES).map_err(storage)?, ino)
    }

    /// The file's layout (idx, chunk) in order.
    pub fn layout(&self, ino: u64) -> Result<Vec<(u64, ChunkRef)>, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        scan_layout(&txn.open_table(LAYOUTS).map_err(storage)?, ino)
    }

    /// Layout rows from `from_idx` onward (range scan — O(rows returned),
    /// used by the read path to skip the prefix walk).
    pub fn layout_from(&self, ino: u64, from_idx: u64) -> Result<Vec<(u64, ChunkRef)>, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let layouts = txn.open_table(LAYOUTS).map_err(storage)?;
        let start = lkey(ino, from_idx);
        let prefix = ino.to_be_bytes();
        let mut out = Vec::new();
        for row in layouts.range(start.as_slice()..).map_err(storage)? {
            let (k, v) = row.map_err(storage)?;
            let key = k.value();
            if !key.starts_with(&prefix) {
                break;
            }
            let idx = u64::from_be_bytes(
                key[8..16]
                    .try_into()
                    .map_err(|_| MetaError::Codec("bad layout key".into()))?,
            );
            out.push((idx, dec::<ChunkRef>(v.value())?));
        }
        Ok(out)
    }

    /// Layout math: which chunk covers `offset` (idx, chunk, intra-chunk
    /// offset). `None` beyond the covered prefix (caller serves zeros up to
    /// the inode size, EOF at the size).
    pub fn chunk_at(
        &self,
        ino: u64,
        offset: u64,
    ) -> Result<Option<(u64, ChunkRef, u64)>, MetaError> {
        let mut base = 0u64;
        for (idx, chunk) in self.layout(ino)? {
            if offset < base + chunk.len {
                return Ok(Some((idx, chunk, offset - base)));
            }
            base += chunk.len;
        }
        Ok(None)
    }

    /// All layout chunks whose replica set contains `addr` (§7.4 repair:
    /// the degraded set on a dead data node).
    pub fn chunks_with_replica(&self, addr: &str) -> Result<Vec<ChunkRef>, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let inodes = txn.open_table(INODES).map_err(storage)?;
        let mut out = Vec::new();
        let ino_list: Vec<u64> = inodes
            .iter()
            .map_err(storage)?
            .filter_map(|r| r.ok().map(|(k, _)| k.value()))
            .collect();
        for ino in ino_list {
            for (_, chunk) in scan_layout(&txn.open_table(LAYOUTS).map_err(storage)?, ino)? {
                if chunk.replicas.iter().any(|a| a == addr) {
                    out.push(chunk);
                }
            }
        }
        Ok(out)
    }

    /// Snapshot rows in id order.
    pub fn list_snaps(&self) -> Result<Vec<SnapshotMeta>, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let snaps = txn.open_table(SNAPS).map_err(storage)?;
        let mut out = Vec::new();
        for row in snaps.iter().map_err(storage)? {
            out.push(dec::<SnapshotMeta>(row.map_err(storage)?.1.value())?);
        }
        Ok(out)
    }

    /// Live chunk refcount (0 when dead).
    pub fn chunk_refcount(&self, chunk_id: &[u8; 16]) -> Result<u64, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let chunkrefs = txn.open_table(CHUNKREFS).map_err(storage)?;
        Ok(chunkrefs
            .get(chunk_id.as_slice())
            .map_err(storage)?
            .map_or(0, |v| v.value()))
    }

    /// All layout chunks: chunk_id → (version, replicas).
    pub fn chunk_map(&self) -> Result<ChunkMap, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let layouts = txn.open_table(LAYOUTS).map_err(storage)?;
        let mut out = std::collections::HashMap::new();
        for row in layouts.iter().map_err(storage)? {
            let (_, v) = row.map_err(storage)?;
            let chunk = dec::<ChunkRef>(v.value())?;
            out.insert(chunk.chunk_id, (chunk.version, chunk.replicas));
        }
        Ok(out)
    }

    /// Chunk ids currently queued for GC (orphan-sweep exclusion).
    pub fn gc_chunk_ids(&self) -> Result<std::collections::HashSet<[u8; 16]>, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let gc = txn.open_table(GC).map_err(storage)?;
        let mut out = std::collections::HashSet::new();
        for row in gc.iter().map_err(storage)? {
            let (_, v) = row.map_err(storage)?;
            out.insert(dec::<GcEntry>(v.value())?.chunk_id);
        }
        Ok(out)
    }

    /// Last applied write-path commit sequence (idempotent re-commit
    /// horizon for WAL replay).
    pub fn commit_seq(&self) -> Result<u64, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let counters = txn.open_table(COUNTERS).map_err(storage)?;
        Ok(counters
            .get("commit_seq")
            .map_err(storage)?
            .map_or(0, |v| v.value()))
    }

    /// Number of queued GC entries.
    pub fn gc_len(&self) -> Result<u64, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        txn.open_table(GC).map_err(storage)?.len().map_err(storage)
    }

    // ---- write-transaction helpers ----

    fn alloc_ino(&self, txn: &redb::WriteTransaction) -> Result<u64, MetaError> {
        let mut counters = txn.open_table(COUNTERS).map_err(storage)?;
        bump(&mut counters, "next_ino")
    }

    fn get_inode(&self, txn: &redb::WriteTransaction, ino: u64) -> Result<Inode, MetaError> {
        read_inode(&txn.open_table(INODES).map_err(storage)?, ino)
    }

    fn put_inode(
        &self,
        txn: &redb::WriteTransaction,
        ino: u64,
        inode: &Inode,
    ) -> Result<(), MetaError> {
        let mut inodes = txn.open_table(INODES).map_err(storage)?;
        inodes
            .insert(ino, enc(inode)?.as_slice())
            .map_err(storage)?;
        Ok(())
    }

    fn remove_inode(&self, txn: &redb::WriteTransaction, ino: u64) -> Result<(), MetaError> {
        let rows = scan_layout(&txn.open_table(LAYOUTS).map_err(storage)?, ino)?;
        for (idx, chunk) in rows {
            let mut layouts = txn.open_table(LAYOUTS).map_err(storage)?;
            layouts.remove(lkey(ino, idx).as_slice()).map_err(storage)?;
            drop(layouts);
            let mut chunkrefs = txn.open_table(CHUNKREFS).map_err(storage)?;
            let mut gc = txn.open_table(GC).map_err(storage)?;
            let mut counters = txn.open_table(COUNTERS).map_err(storage)?;
            dec_ref(&mut chunkrefs, &mut gc, &mut counters, &chunk)?;
        }
        let mut inodes = txn.open_table(INODES).map_err(storage)?;
        inodes.remove(ino).map_err(storage)?;
        Ok(())
    }

    fn insert_dentry(
        &self,
        txn: &redb::WriteTransaction,
        parent: u64,
        name: &str,
        ino: u64,
        now: Time,
    ) -> Result<(), MetaError> {
        let parent_inode = self.get_inode(txn, parent)?;
        if parent_inode.kind != Kind::Dir {
            return Err(MetaError::NotDir);
        }
        if name.is_empty() || name.contains('/') {
            return Err(MetaError::Invalid("bad name".into()));
        }
        if read_dentry(&txn.open_table(DENTRIES).map_err(storage)?, parent, name)?.is_some() {
            return Err(MetaError::Exists);
        }
        let mut dentries = txn.open_table(DENTRIES).map_err(storage)?;
        dentries
            .insert(dkey(parent, name).as_slice(), ino)
            .map_err(storage)?;
        drop(dentries);
        let mut parent_inode = parent_inode;
        parent_inode.mtime = now;
        parent_inode.ctime = now;
        self.put_inode(txn, parent, &parent_inode)
    }

    fn take_dentry(
        &self,
        txn: &redb::WriteTransaction,
        parent: u64,
        name: &str,
    ) -> Result<u64, MetaError> {
        let ino = read_dentry(&txn.open_table(DENTRIES).map_err(storage)?, parent, name)?
            .ok_or(MetaError::NotFound)?;
        self.remove_dentry(txn, parent, name)?;
        Ok(ino)
    }

    fn remove_dentry(
        &self,
        txn: &redb::WriteTransaction,
        parent: u64,
        name: &str,
    ) -> Result<(), MetaError> {
        let mut dentries = txn.open_table(DENTRIES).map_err(storage)?;
        dentries
            .remove(dkey(parent, name).as_slice())
            .map_err(storage)?;
        Ok(())
    }

    fn bump_nlink(
        &self,
        txn: &redb::WriteTransaction,
        ino: u64,
        delta: i32,
        now: Time,
    ) -> Result<(), MetaError> {
        let mut inode = self.get_inode(txn, ino)?;
        inode.nlink = inode.nlink.saturating_add_signed(delta);
        inode.ctime = now;
        self.put_inode(txn, ino, &inode)
    }

    /// Drop layout rows fully beyond `size` (a straddling chunk stays
    /// sealed and whole; reads are clamped by the inode size).
    fn truncate_layout(
        &self,
        txn: &redb::WriteTransaction,
        ino: u64,
        size: u64,
    ) -> Result<(), MetaError> {
        let rows = scan_layout(&txn.open_table(LAYOUTS).map_err(storage)?, ino)?;
        let mut base = 0u64;
        let mut layouts = txn.open_table(LAYOUTS).map_err(storage)?;
        let mut chunkrefs = txn.open_table(CHUNKREFS).map_err(storage)?;
        let mut gc = txn.open_table(GC).map_err(storage)?;
        let mut counters = txn.open_table(COUNTERS).map_err(storage)?;
        for (idx, chunk) in rows {
            if base >= size {
                layouts.remove(lkey(ino, idx).as_slice()).map_err(storage)?;
                dec_ref(&mut chunkrefs, &mut gc, &mut counters, &chunk)?;
            }
            base += chunk.len;
        }
        Ok(())
    }
}
