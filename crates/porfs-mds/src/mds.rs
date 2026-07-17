//! The MDS core: two files per instance (a redb database for metadata, an
//! extent-store device for file payloads). This module owns the lifecycle
//! (`format` / `open` / mount-time reconcile), the shared row helpers the
//! operation modules build on, and the extent-map scan used by the data
//! path. Namespace operations live in `namespace.rs`, the file data path in
//! `data.rs`, consistency verification in `check.rs`.
//!
//! Namespace mutations commit as ONE redb write transaction (ACID — a rename
//! is a single commit). redb fdatasyncs on every commit, so committed
//! metadata is durable immediately; extent data becomes durable only at
//! [`Mds::fsync`] (the store's group commit). See `docs/format.md` §6 for the
//! durability-horizon model this builds on.

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use redb::{Database, ReadableDatabase, ReadableTable, ReadableTableMetadata};

use porfs_store::{ExtentId, ExtentStore, StoreError};

use crate::error::{MdsError, Result, dberr};
use crate::keys::{
    DIR_ENTRIES, FILE_EXTENTS, INODES, MDS_SCHEMA_VERSION, META, NEXT_INO_KEY, SCHEMA_VERSION_KEY,
    XATTRS, decode_dirent_name, decode_extent_key, decode_extent_value, decode_rec, dirent_bounds,
    dirent_hash_bounds, encode_rec, extent_bounds, extent_key, ino_key, name_hash,
};
use crate::types::{Ino, InodeRec, MAX_NAME_LEN, NodeKind, ROOT_INO, Statfs};

/// One row of a file's extent map: interval `[off, off + len)` is covered by
/// extent `id`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ExtentRow {
    pub(crate) off: u64,
    pub(crate) id: ExtentId,
    pub(crate) len: u64,
}

impl ExtentRow {
    /// End (exclusive) of the covered interval.
    pub(crate) fn end(&self) -> u64 {
        self.off + self.len
    }
}

/// Single-node PolarisFS metadata server (v0, library form).
///
/// Owns the redb metadata database and the extent store. All mutating
/// namespace operations take `&mut self` (single-writer; P6 turns this into
/// a networked service with its own concurrency story).
///
/// Crash model: a crash between a metadata commit and [`Mds::fsync`] can
/// leave `file_extents` rows pointing at extents the store salvaged away on
/// remount. [`Mds::open`] repairs exactly that before serving (see
/// [`Mds::last_reconcile_repairs`]).
pub struct Mds {
    pub(crate) db: Database,
    pub(crate) store: ExtentStore,
    pub(crate) last_reconcile_repairs: u64,
}

impl Mds {
    /// Create a fresh MDS instance: initialize the extent-store device and
    /// the redb database, inserting the root directory (ino [`ROOT_INO`],
    /// mode 0o755) and the inode counter.
    ///
    /// `data_size` is the device size in bytes (rounded down to a 4KiB
    /// multiple by the store). Fails with [`MdsError::Exists`] if the
    /// metadata file is already a formatted MDS database.
    pub fn format(
        meta_path: impl AsRef<Path>,
        data_path: impl AsRef<Path>,
        data_size: u64,
    ) -> Result<Self> {
        let store = ExtentStore::create(data_path, data_size)?;
        let db = Database::create(meta_path.as_ref()).map_err(dberr)?;
        let txn = db.begin_write().map_err(dberr)?;
        {
            let mut inodes = txn.open_table(INODES).map_err(dberr)?;
            let mut meta = txn.open_table(META).map_err(dberr)?;
            if meta.get(NEXT_INO_KEY).map_err(dberr)?.is_some() {
                return Err(MdsError::Exists(format!(
                    "metadata already formatted: {}",
                    meta_path.as_ref().display()
                )));
            }
            // Create the whole schema now: redb read transactions fail with
            // TableDoesNotExist on a table no committed write txn ever
            // opened, and every read path assumes the schema exists.
            txn.open_table(DIR_ENTRIES).map_err(dberr)?;
            txn.open_table(FILE_EXTENTS).map_err(dberr)?;
            txn.open_table(XATTRS).map_err(dberr)?;
            let root = InodeRec::new(NodeKind::Dir, 0o755, 0, 0, now_ts());
            let root_bytes = encode_rec(&root)?;
            inodes
                .insert(&ino_key(ROOT_INO), root_bytes.as_slice())
                .map_err(dberr)?;
            meta.insert(NEXT_INO_KEY, ROOT_INO + 1).map_err(dberr)?;
            meta.insert(SCHEMA_VERSION_KEY, MDS_SCHEMA_VERSION)
                .map_err(dberr)?;
        }
        txn.commit().map_err(dberr)?;
        Ok(Self {
            db,
            store,
            last_reconcile_repairs: 0,
        })
    }

    /// Open an existing MDS instance and run the mount-time reconcile:
    /// `file_extents` rows whose extent is missing from the store index (or
    /// tombstoned) are deleted, turning the region into a hole. That is
    /// exactly the state a crash between the last metadata commit and the
    /// last [`Mds::fsync`] can leave behind (the store salvages unconfirmed
    /// tail writes on remount).
    pub fn open(meta_path: impl AsRef<Path>, data_path: impl AsRef<Path>) -> Result<Self> {
        let store = ExtentStore::open(data_path)?;
        let db = Database::open(meta_path.as_ref()).map_err(dberr)?;
        check_schema_version(&db, meta_path.as_ref())?;
        let mut mds = Self {
            db,
            store,
            last_reconcile_repairs: 0,
        };
        mds.get_rec(ROOT_INO).map_err(|err| match err {
            MdsError::NotFound(_) => MdsError::Corrupt(format!(
                "{} is not a formatted MDS database (no root inode)",
                meta_path.as_ref().display()
            )),
            other => other,
        })?;
        let repairs = mds.reconcile()?;
        mds.last_reconcile_repairs = repairs;
        Ok(mds)
    }

    /// Number of dangling extent-map rows the mount-time reconcile removed
    /// during the last [`Mds::open`] (always 0 after [`Mds::format`]).
    pub fn last_reconcile_repairs(&self) -> u64 {
        self.last_reconcile_repairs
    }

    /// Storage and namespace totals for `statfs`-style reporting.
    pub fn statfs(&self) -> Result<Statfs> {
        let txn = self.db.begin_read().map_err(dberr)?;
        let inodes = txn.open_table(INODES).map_err(dberr)?;
        Ok(Statfs {
            total_bytes: self.store.device_size(),
            free_bytes: self.store.device_size().saturating_sub(self.store.tail()),
            live_bytes: self.store.live_bytes(),
            inodes: inodes.len().map_err(dberr)?,
            extents: self.store.extent_count(),
        })
    }

    /// Read an inode record, mapping a missing row to [`MdsError::NotFound`].
    pub(crate) fn get_rec(&self, ino: Ino) -> Result<InodeRec> {
        let txn = self.db.begin_read().map_err(dberr)?;
        let inodes = txn.open_table(INODES).map_err(dberr)?;
        rec_from(&inodes, ino)
    }

    /// Rows of `ino`'s extent map intersecting `[start, end)`, in offset
    /// order. Shared by the data path and the truncate path.
    pub(crate) fn extent_rows(&self, ino: Ino, start: u64, end: u64) -> Result<Vec<ExtentRow>> {
        let txn = self.db.begin_read().map_err(dberr)?;
        let fext = txn.open_table(FILE_EXTENTS).map_err(dberr)?;
        // Every row of `ino` with offset < end is a candidate; keep those
        // whose interval reaches into [start, end). Rows are offset-ordered,
        // so the first row may be the one covering `start` (predecessor).
        let lo = extent_key(ino, 0);
        let hi = extent_key(ino, end);
        let mut rows = Vec::new();
        for item in fext.range(lo.as_slice()..hi.as_slice()).map_err(dberr)? {
            let (key, value) = item.map_err(dberr)?;
            let (_, offset) = decode_extent_key(key.value())?;
            let (extent_id, len) = decode_extent_value(value.value())?;
            if offset + len > start {
                rows.push(ExtentRow {
                    off: offset,
                    id: extent_id,
                    len,
                });
            }
        }
        Ok(rows)
    }

    /// Tombstone extents in the store after their map rows committed out.
    /// Tolerates UnknownExtent/Tombstoned: after a crash the store may
    /// already have salvaged the extent away (the committed metadata is the
    /// source of truth; unreachable extents are GC material for P27).
    pub(crate) fn discard_tolerant(&mut self, ids: &[ExtentId]) -> Result<()> {
        for &id in ids {
            match self.store.discard(id) {
                Ok(()) | Err(StoreError::UnknownExtent(_)) | Err(StoreError::Tombstoned(_)) => {}
                Err(err) => return Err(MdsError::Store(err)),
            }
        }
        Ok(())
    }

    /// v0 extent-liveness probe: reads the payload (the store exposes no
    /// metadata-only index query), which doubles as a CRC check. Missing or
    /// tombstoned maps to `false`; any other error propagates.
    pub(crate) fn extent_live(&mut self, extent_id: ExtentId) -> Result<bool> {
        match self.store.read(extent_id) {
            Ok(_) => Ok(true),
            Err(StoreError::UnknownExtent(_)) | Err(StoreError::Tombstoned(_)) => Ok(false),
            Err(err) => Err(MdsError::Store(err)),
        }
    }

    /// The mount-time reconcile behind [`Mds::open`]; returns the number of
    /// dangling map rows removed (each turns its region into a hole).
    fn reconcile(&mut self) -> Result<u64> {
        let rows: Vec<(Ino, u64, ExtentId)> = {
            let txn = self.db.begin_read().map_err(dberr)?;
            let fext = txn.open_table(FILE_EXTENTS).map_err(dberr)?;
            let mut rows = Vec::new();
            for item in fext.iter().map_err(dberr)? {
                let (key, value) = item.map_err(dberr)?;
                let (ino, offset) = decode_extent_key(key.value())?;
                let (extent_id, _) = decode_extent_value(value.value())?;
                rows.push((ino, offset, extent_id));
            }
            rows
        };
        let mut dead = Vec::new();
        for (ino, offset, extent_id) in rows {
            if !self.extent_live(extent_id)? {
                dead.push(extent_key(ino, offset));
            }
        }
        let repairs = dead.len() as u64;
        if !dead.is_empty() {
            let txn = self.db.begin_write().map_err(dberr)?;
            {
                let mut fext = txn.open_table(FILE_EXTENTS).map_err(dberr)?;
                for key in &dead {
                    fext.remove(key.as_slice()).map_err(dberr)?;
                }
            }
            txn.commit().map_err(dberr)?;
        }
        Ok(repairs)
    }
}

/// Current time as `(secs, nsecs)` since the epoch; `(0, 0)` before it.
pub(crate) fn now_ts() -> (i64, u32) {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or((0, 0), |d| (d.as_secs() as i64, d.subsec_nanos()))
}

/// Enforce the metadata schema version on open: an absent version key marks
/// a pre-P5 legacy database, and any other value marks a newer/older
/// incompatible schema. Both are rejected with a clear error (dev-box
/// precedent, same as the v0->v2 extent-format rejection).
fn check_schema_version(db: &Database, meta_path: &Path) -> Result<()> {
    let txn = db.begin_read().map_err(dberr)?;
    let meta = txn.open_table(META).map_err(dberr)?;
    let version = meta
        .get(SCHEMA_VERSION_KEY)
        .map_err(dberr)?
        .map(|v| v.value());
    match version {
        None => Err(MdsError::UnsupportedSchema(format!(
            "{} has no schema_version row: legacy (pre-P5) metadata layout; \
             re-format (the dev-box layout changed: hash-ordered dir entries, \
             xattrs, symlink/special inodes)",
            meta_path.display()
        ))),
        Some(v) if v != MDS_SCHEMA_VERSION => Err(MdsError::UnsupportedSchema(format!(
            "{} has metadata schema version {v}, this build supports {MDS_SCHEMA_VERSION}; \
             re-format",
            meta_path.display()
        ))),
        Some(_) => Ok(()),
    }
}

/// POSIX name rules: nonempty, at most 255 bytes, no `/`, not `.` or `..`.
fn name_is_valid(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= MAX_NAME_LEN
        && name != "."
        && name != ".."
        && !name.contains('/')
}

/// Validate a single path component.
pub(crate) fn validate_name(name: &str) -> Result<()> {
    if name.len() > MAX_NAME_LEN {
        return Err(MdsError::NameTooLong(name.len()));
    }
    if name_is_valid(name) {
        Ok(())
    } else {
        Err(MdsError::InvalidName(name.to_string()))
    }
}

/// Read an inode record out of an `inodes` table (read or write txn).
pub(crate) fn rec_from(
    t: &impl ReadableTable<&'static [u8; 8], &'static [u8]>,
    ino: Ino,
) -> Result<InodeRec> {
    let guard = t
        .get(&ino_key(ino))
        .map_err(dberr)?
        .ok_or_else(|| MdsError::NotFound(format!("ino {ino}")))?;
    decode_rec(guard.value())
}

/// Error unless `rec` is a directory.
pub(crate) fn require_dir(rec: &InodeRec, ino: Ino) -> Result<()> {
    if rec.kind == NodeKind::Dir {
        Ok(())
    } else {
        Err(MdsError::NotDir(format!("ino {ino}")))
    }
}

/// Resolve a directory entry to its child inode. The key is
/// `parent | hash(name) | name`; the lookup scans the hash prefix (the
/// collision class) and compares the full-name suffix, so same-hash
/// different-name entries coexist.
pub(crate) fn entry_child(
    entries: &impl ReadableTable<&'static [u8], &'static [u8; 8]>,
    parent: Ino,
    name: &str,
) -> Result<Ino> {
    let (start, end) = dirent_hash_bounds(parent, name_hash(name));
    let range = entries
        .range(start.as_slice()..end.as_slice())
        .map_err(dberr)?;
    for item in range {
        let (key, value) = item.map_err(dberr)?;
        if decode_dirent_name(key.value())? == name {
            return Ok(Ino::from_be_bytes(*value.value()));
        }
    }
    Err(MdsError::NotFound(format!("{name:?} under ino {parent}")))
}

/// True when directory `ino` has no entries.
pub(crate) fn dir_is_empty(
    entries: &impl ReadableTable<&'static [u8], &'static [u8; 8]>,
    ino: Ino,
) -> Result<bool> {
    let (start, end) = dirent_bounds(ino);
    let mut range = entries
        .range(start.as_slice()..end.as_slice())
        .map_err(dberr)?;
    match range.next() {
        None => Ok(true),
        Some(item) => {
            item.map_err(dberr)?;
            Ok(false)
        }
    }
}

/// Delete every `file_extents` row of `ino` inside the current write
/// transaction, returning the extent ids to tombstone in the store after the
/// commit.
pub(crate) fn remove_extent_rows(
    fext: &mut redb::Table<'_, &'static [u8], &'static [u8; 16]>,
    ino: Ino,
) -> Result<Vec<ExtentId>> {
    let (start, end) = extent_bounds(ino);
    let mut rows = Vec::new();
    for item in fext
        .range(start.as_slice()..end.as_slice())
        .map_err(dberr)?
    {
        let (key, value) = item.map_err(dberr)?;
        let (_, offset) = decode_extent_key(key.value())?;
        let (extent_id, _) = decode_extent_value(value.value())?;
        rows.push((offset, extent_id));
    }
    for (offset, _) in &rows {
        fext.remove(extent_key(ino, *offset).as_slice())
            .map_err(dberr)?;
    }
    Ok(rows.into_iter().map(|(_, id)| id).collect())
}
