//! The MDS core: a redb database for metadata plus a data plane for file
//! payloads (one local extent device, or a cluster of chunkservers). This
//! module owns the lifecycle (`format` / `format_cluster` / `open` /
//! mount-time reconcile), the shared row helpers the operation modules
//! build on, and the extent-map scan used by the data path. Namespace
//! operations live in `namespace.rs`, the file data path in `data.rs`,
//! consistency verification in `check.rs`, the data plane in `plane.rs`.
//!
//! Namespace mutations commit as ONE redb write transaction (ACID — a rename
//! is a single commit). redb fdatasyncs on every commit, so committed
//! metadata is durable immediately; extent data becomes durable only at
//! [`Mds::fsync`] (the store's group commit / the cluster-wide sync
//! broadcast). See `docs/format.md` §6 for the durability-horizon model
//! this builds on.

use std::net::SocketAddr;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use redb::{Database, ReadableDatabase, ReadableTable, ReadableTableMetadata};

use porfs_store::ExtentStore;

use crate::error::{MdsError, Result, dberr};
use crate::keys::{
    CLUSTER_CONFIG, CLUSTER_MEMBERSHIP_KEY, CLUSTER_MODE_KEY, DIR_ENTRIES, ExtentRef, FILE_EXTENTS,
    FILE_EXTENTS_V2, INODES, MDS_SCHEMA_VERSION, META, MemberSpec, NEXT_INO_KEY, REPLICAS_KEY,
    SCHEMA_VERSION_KEY, XATTRS, decode_dirent_name, decode_extent_key, decode_extent_value,
    decode_membership, decode_rec, dirent_bounds, dirent_hash_bounds, encode_extent_value,
    encode_membership, encode_rec, extent_bounds, extent_key, ino_key, name_hash,
};
use crate::plane::{ClusterTimeouts, DataPlane, Probe, RemoteStore, learn_membership};
use crate::types::{Ino, InodeRec, MAX_NAME_LEN, NodeKind, ROOT_INO, Statfs};

/// One row of a file's extent map: interval `[off, off + len)` is covered by
/// the extent(s) `id` locates (one local extent, or one copy per cluster
/// chunkserver).
#[derive(Debug, Clone, Copy)]
pub(crate) struct ExtentRow {
    pub(crate) off: u64,
    pub(crate) id: ExtentRef,
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
/// Owns the redb metadata database and the data plane ([`DataPlane::Local`]
/// for a local extent device, [`DataPlane::Remote`] for a chunkserver
/// cluster). All mutating namespace operations take `&mut self`
/// (single-writer; P6 turns this into a networked service with its own
/// concurrency story).
///
/// Crash model: a crash between a metadata commit and [`Mds::fsync`] can
/// leave `file_extents_v2` rows pointing at extents the store(s) salvaged
/// away on remount. [`Mds::open`] repairs exactly that before serving (see
/// [`Mds::last_reconcile_repairs`]). Cluster mode repairs a row ONLY when
/// every copy answers `NotFound` from a reachable, identity-verified
/// server — an unreachable server fails the mount instead.
pub struct Mds {
    pub(crate) db: Database,
    pub(crate) store: DataPlane,
    pub(crate) last_reconcile_repairs: u64,
}

impl Mds {
    /// Create a fresh local-mode MDS instance: initialize the extent-store
    /// device and the redb database, inserting the root directory (ino
    /// [`ROOT_INO`], mode 0o755) and the inode counter.
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
            txn.open_table(FILE_EXTENTS_V2).map_err(dberr)?;
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
            store: DataPlane::Local(Box::new(store)),
            last_reconcile_repairs: 0,
        })
    }

    /// Create a fresh cluster-mode MDS instance: file payloads live on the
    /// given chunkservers (`(addr, rack, chassis)` per member) instead of a
    /// local device. Every member is contacted at format time; its store
    /// UUID is learned and pinned into `cluster_config`, so a later open
    /// refuses a reordered or reformatted server (the identity guard).
    ///
    /// `replicas` is 1 (single copy) or 2 (chain-replicated; requires
    /// members in at least two racks).
    pub fn format_cluster(
        meta_path: impl AsRef<Path>,
        members: Vec<(SocketAddr, String, String)>,
        replicas: usize,
    ) -> Result<Self> {
        Self::format_cluster_with_timeouts(meta_path, members, replicas, ClusterTimeouts::default())
    }

    /// [`Mds::format_cluster`] with explicit network timeouts (loopback
    /// tests shorten the failure paths; production uses the defaults).
    #[doc(hidden)]
    pub fn format_cluster_with_timeouts(
        meta_path: impl AsRef<Path>,
        members: Vec<(SocketAddr, String, String)>,
        replicas: usize,
        timeouts: ClusterTimeouts,
    ) -> Result<Self> {
        let specs = learn_membership(&members, timeouts)?;
        let plane = DataPlane::Remote(Box::new(RemoteStore::connect(&specs, replicas, timeouts)?));
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
            txn.open_table(DIR_ENTRIES).map_err(dberr)?;
            txn.open_table(FILE_EXTENTS_V2).map_err(dberr)?;
            txn.open_table(XATTRS).map_err(dberr)?;
            let mut cluster_config = txn.open_table(CLUSTER_CONFIG).map_err(dberr)?;
            let membership = encode_membership(&specs)?;
            cluster_config
                .insert(CLUSTER_MEMBERSHIP_KEY, membership.as_slice())
                .map_err(dberr)?;
            let root = InodeRec::new(NodeKind::Dir, 0o755, 0, 0, now_ts());
            let root_bytes = encode_rec(&root)?;
            inodes
                .insert(&ino_key(ROOT_INO), root_bytes.as_slice())
                .map_err(dberr)?;
            meta.insert(NEXT_INO_KEY, ROOT_INO + 1).map_err(dberr)?;
            meta.insert(SCHEMA_VERSION_KEY, MDS_SCHEMA_VERSION)
                .map_err(dberr)?;
            meta.insert(CLUSTER_MODE_KEY, 1).map_err(dberr)?;
            meta.insert(REPLICAS_KEY, replicas as u64).map_err(dberr)?;
        }
        txn.commit().map_err(dberr)?;
        Ok(Self {
            db,
            store: plane,
            last_reconcile_repairs: 0,
        })
    }

    /// Open an existing local-mode MDS instance and run the mount-time
    /// reconcile: `file_extents_v2` rows whose extent is missing from the
    /// store index (or tombstoned) are deleted, turning the region into a
    /// hole. That is exactly the state a crash between the last metadata
    /// commit and the last [`Mds::fsync`] can leave behind (the store
    /// salvages unconfirmed tail writes on remount).
    pub fn open(meta_path: impl AsRef<Path>, data_path: impl AsRef<Path>) -> Result<Self> {
        Self::open_any(
            meta_path.as_ref(),
            Some(data_path.as_ref()),
            ClusterTimeouts::default(),
        )
    }

    /// Open an existing cluster-mode MDS instance (membership, UUID pins,
    /// and replica count come from the persisted `cluster_config` — no
    /// flags needed at remount) and run the mount-time reconcile.
    pub fn open_cluster(meta_path: impl AsRef<Path>) -> Result<Self> {
        Self::open_any(meta_path.as_ref(), None, ClusterTimeouts::default())
    }

    /// [`Mds::open_cluster`] with explicit network timeouts.
    #[doc(hidden)]
    pub fn open_cluster_with_timeouts(
        meta_path: impl AsRef<Path>,
        timeouts: ClusterTimeouts,
    ) -> Result<Self> {
        Self::open_any(meta_path.as_ref(), None, timeouts)
    }

    /// Shared open path: schema check (+ schema 1 -> 2 migration), data
    /// plane construction (local device or cluster reconnect with identity
    /// verification), root inode check, mount-time reconcile.
    fn open_any(
        meta_path: &Path,
        data_path: Option<&Path>,
        timeouts: ClusterTimeouts,
    ) -> Result<Self> {
        let db = Database::open(meta_path).map_err(dberr)?;
        let version = check_schema_version(&db, meta_path)?;
        let cluster_mode = meta_counter(&db, CLUSTER_MODE_KEY)?.unwrap_or(0);
        if version == 1 {
            // A schema-1 database is local by definition (cluster mode did
            // not exist before schema 2).
            if cluster_mode != 0 {
                return Err(MdsError::Corrupt(format!(
                    "{} has metadata schema 1 but cluster_mode {cluster_mode}",
                    meta_path.display()
                )));
            }
            migrate_schema_1_to_2(&db)?;
        }
        let plane = match cluster_mode {
            0 => {
                let data_path = data_path.ok_or_else(|| {
                    MdsError::InvalidOp(format!(
                        "{} is a local-mode database; its extent device path is required",
                        meta_path.display()
                    ))
                })?;
                DataPlane::Local(Box::new(ExtentStore::open(data_path)?))
            }
            1 => {
                let members = read_cluster_membership(&db)?;
                let replicas = meta_counter(&db, REPLICAS_KEY)?
                    .ok_or_else(|| {
                        MdsError::Corrupt(format!(
                            "{} is cluster-mode but has no replicas row",
                            meta_path.display()
                        ))
                    })?
                    .try_into()
                    .map_err(|_| {
                        MdsError::Corrupt(format!(
                            "{} has an out-of-range replicas row",
                            meta_path.display()
                        ))
                    })?;
                DataPlane::Remote(Box::new(RemoteStore::connect(
                    &members, replicas, timeouts,
                )?))
            }
            other => {
                return Err(MdsError::Corrupt(format!(
                    "{} has unknown cluster_mode {other}",
                    meta_path.display()
                )));
            }
        };
        let mut mds = Self {
            db,
            store: plane,
            last_reconcile_repairs: 0,
        };
        mds.get_rec(ROOT_INO).map_err(|err| match err {
            MdsError::NotFound(_) => MdsError::Corrupt(format!(
                "{} is not a formatted MDS database (no root inode)",
                meta_path.display()
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

    /// Storage and namespace totals for `statfs`-style reporting. Cluster
    /// mode sums every server's counters; an unreachable server is an
    /// error, not a silently skipped term.
    pub fn statfs(&self) -> Result<Statfs> {
        let txn = self.db.begin_read().map_err(dberr)?;
        let inodes = txn.open_table(INODES).map_err(dberr)?;
        let device_size = self.store.device_size()?;
        Ok(Statfs {
            total_bytes: device_size,
            free_bytes: device_size.saturating_sub(self.store.tail()?),
            live_bytes: self.store.live_bytes()?,
            inodes: inodes.len().map_err(dberr)?,
            extents: self.store.extent_count()?,
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
        let fext = txn.open_table(FILE_EXTENTS_V2).map_err(dberr)?;
        // Every row of `ino` with offset < end is a candidate; keep those
        // whose interval reaches into [start, end). Rows are offset-ordered,
        // so the first row may be the one covering `start` (predecessor).
        let lo = extent_key(ino, 0);
        let hi = extent_key(ino, end);
        let mut rows = Vec::new();
        for item in fext.range(lo.as_slice()..hi.as_slice()).map_err(dberr)? {
            let (key, value) = item.map_err(dberr)?;
            let (_, offset) = decode_extent_key(key.value())?;
            let (eref, len) = decode_extent_value(value.value())?;
            if offset + len > start {
                rows.push(ExtentRow {
                    off: offset,
                    id: eref,
                    len,
                });
            }
        }
        Ok(rows)
    }

    /// Tombstone extents after their map rows committed out. Tolerates
    /// "already gone": after a crash the store(s) may already have salvaged
    /// the extent (the committed metadata is the source of truth;
    /// unreachable extents are GC material for P27).
    pub(crate) fn discard_tolerant(&mut self, refs: &[ExtentRef]) -> Result<()> {
        self.store.discard_tolerant(refs)
    }

    /// The mount-time reconcile behind [`Mds::open`]; returns the number of
    /// dangling map rows removed (each turns its region into a hole).
    ///
    /// A row is dead ONLY when every copy of its extent probes `NotFound`
    /// from a reachable server. `Live` on any copy keeps the row. Any
    /// `Unreachable` probe aborts the mount with a loud error naming the
    /// server — reconcile never deletes on ambiguity (a partition answering
    /// silence must not cost the namespace its data).
    fn reconcile(&mut self) -> Result<u64> {
        let rows: Vec<(Vec<u8>, ExtentRef)> = {
            let txn = self.db.begin_read().map_err(dberr)?;
            let fext = txn.open_table(FILE_EXTENTS_V2).map_err(dberr)?;
            let mut rows = Vec::new();
            for item in fext.iter().map_err(dberr)? {
                let (key, value) = item.map_err(dberr)?;
                // Decoding validates the key/value shapes (corruption is
                // caught here, not mid-reconcile); deletion needs the raw key.
                decode_extent_key(key.value())?;
                let (eref, _) = decode_extent_value(value.value())?;
                rows.push((key.value().to_vec(), eref));
            }
            rows
        };
        if rows.is_empty() {
            return Ok(0);
        }
        let refs: Vec<ExtentRef> = rows.iter().map(|(_, eref)| *eref).collect();
        let probes = self.store.probe(&refs)?;
        let mut dead = Vec::new();
        let mut cursor = 0;
        for (key, eref) in &rows {
            let copies = &probes[cursor..cursor + eref.copies()];
            cursor += eref.copies();
            if copies.iter().all(|probe| *probe == Probe::NotFound) {
                dead.push(key.clone());
                continue;
            }
            if let Some(server) = first_unreachable(eref, copies) {
                let addr = self
                    .store
                    .member_addr(server)
                    .map(|addr| addr.to_string())
                    .unwrap_or_else(|| format!("#{server}"));
                return Err(MdsError::Cluster(format!(
                    "chunkserver {addr} is unreachable; refusing to mount rather than \
                     reconcile against unknown extent state (no rows were deleted)"
                )));
            }
        }
        let repairs = dead.len() as u64;
        if !dead.is_empty() {
            let txn = self.db.begin_write().map_err(dberr)?;
            {
                let mut fext = txn.open_table(FILE_EXTENTS_V2).map_err(dberr)?;
                for key in &dead {
                    fext.remove(key.as_slice()).map_err(dberr)?;
                }
            }
            txn.commit().map_err(dberr)?;
        }
        Ok(repairs)
    }
}

/// The server index of the first `Unreachable` copy of `eref` (`None` when
/// every copy resolved). Copy 0 is the primary, copy 1 the replica.
fn first_unreachable(eref: &ExtentRef, copies: &[Probe]) -> Option<u32> {
    let ExtentRef::Remote {
        server, replica, ..
    } = *eref
    else {
        return None; // a local probe never yields Unreachable
    };
    if copies.first() == Some(&Probe::Unreachable) {
        return Some(server);
    }
    if copies.get(1) == Some(&Probe::Unreachable) {
        return replica.map(|(server, _)| server);
    }
    None
}

/// Read one `meta` counter (`Ok(None)` when the row is absent).
fn meta_counter(db: &Database, key: &str) -> Result<Option<u64>> {
    let txn = db.begin_read().map_err(dberr)?;
    let meta = txn.open_table(META).map_err(dberr)?;
    Ok(meta.get(key).map_err(dberr)?.map(|v| v.value()))
}

/// Read and decode the persisted cluster membership.
fn read_cluster_membership(db: &Database) -> Result<Vec<MemberSpec>> {
    let txn = db.begin_read().map_err(dberr)?;
    let cluster_config = txn.open_table(CLUSTER_CONFIG).map_err(dberr)?;
    let blob = cluster_config
        .get(CLUSTER_MEMBERSHIP_KEY)
        .map_err(dberr)?
        .ok_or_else(|| {
            MdsError::Corrupt("cluster-mode database without a membership row".to_string())
        })?;
    decode_membership(blob.value())
}

/// The schema 1 -> 2 upgrade: copy every `file_extents` row into
/// `file_extents_v2` verbatim (the 16-byte local value form is
/// byte-identical between the tables), drop the legacy table, and stamp
/// the new schema version — all in one transaction.
fn migrate_schema_1_to_2(db: &Database) -> Result<()> {
    let txn = db.begin_write().map_err(dberr)?;
    {
        let legacy = txn.open_table(FILE_EXTENTS).map_err(dberr)?;
        let mut current = txn.open_table(FILE_EXTENTS_V2).map_err(dberr)?;
        for item in legacy.iter().map_err(dberr)? {
            let (key, value) = item.map_err(dberr)?;
            // Decode + re-encode validates the legacy row; for the local
            // form the codec reproduces the identical 16 bytes.
            let (eref, len) = decode_extent_value(value.value())?;
            let encoded = encode_extent_value(&eref, len);
            current
                .insert(key.value(), encoded.as_slice())
                .map_err(dberr)?;
        }
        let mut meta = txn.open_table(META).map_err(dberr)?;
        meta.insert(SCHEMA_VERSION_KEY, MDS_SCHEMA_VERSION)
            .map_err(dberr)?;
    }
    if !txn.delete_table(FILE_EXTENTS).map_err(dberr)? {
        return Err(MdsError::Corrupt(
            "schema 1 database without a file_extents table".to_string(),
        ));
    }
    txn.commit().map_err(dberr)?;
    Ok(())
}

/// Current time as `(secs, nsecs)` since the epoch; `(0, 0)` before it.
pub(crate) fn now_ts() -> (i64, u32) {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or((0, 0), |d| (d.as_secs() as i64, d.subsec_nanos()))
}

/// Enforce the metadata schema version on open: an absent version key marks
/// a pre-P5 legacy database (rejected). Schema 1 opens and migrates to 2
/// in place; schema 2 is current; anything else is from an incompatible
/// build. Returns the version found (1 or 2).
fn check_schema_version(db: &Database, meta_path: &Path) -> Result<u64> {
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
        Some(v @ (1 | MDS_SCHEMA_VERSION)) => Ok(v),
        Some(v) => Err(MdsError::UnsupportedSchema(format!(
            "{} has metadata schema version {v}, this build supports 1 (migrated) \
             and {MDS_SCHEMA_VERSION}; re-format",
            meta_path.display()
        ))),
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

/// Delete every `file_extents_v2` row of `ino` inside the current write
/// transaction, returning the extent refs to tombstone in the data plane
/// after the commit.
pub(crate) fn remove_extent_rows(
    fext: &mut redb::Table<'_, &'static [u8], &'static [u8]>,
    ino: Ino,
) -> Result<Vec<ExtentRef>> {
    let (start, end) = extent_bounds(ino);
    let mut rows = Vec::new();
    for item in fext
        .range(start.as_slice()..end.as_slice())
        .map_err(dberr)?
    {
        let (key, value) = item.map_err(dberr)?;
        let (_, offset) = decode_extent_key(key.value())?;
        let (eref, _) = decode_extent_value(value.value())?;
        rows.push((offset, eref));
    }
    for (offset, _) in &rows {
        fext.remove(extent_key(ino, *offset).as_slice())
            .map_err(dberr)?;
    }
    Ok(rows.into_iter().map(|(_, eref)| eref).collect())
}
