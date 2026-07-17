//! Consistency verification over the metadata tables and the store index:
//! `self_check` is the offline-fsck-style pass the P3 gate runs after every
//! (clean or crashed) remount. It detects; it does not repair — the only
//! repair in v0 is the mount-time reconcile in `Mds::open`, whose count is
//! reported here as `orphans_repaired`.

use std::collections::HashMap;

use redb::{ReadableDatabase, ReadableTable};

use crate::error::{MdsError, Result, dberr};
use crate::keys::{
    DIR_ENTRIES, FILE_EXTENTS, INODES, decode_extent_key, decode_extent_value, decode_rec, ino_key,
};
use crate::mds::Mds;
use crate::types::{CheckReport, Ino, NodeKind, ROOT_INO};

impl Mds {
    /// Verify metadata consistency and count the live objects:
    ///
    /// 1. every directory entry's child inode exists;
    /// 2. every non-root directory has exactly one parent entry and
    ///    `nlink == 1`; every file's `nlink` equals its directory-entry count;
    /// 3. every `file_extents` row belongs to a live **file** inode;
    /// 4. every mapped extent exists in the store index and is not
    ///    tombstoned.
    ///
    /// The first failure returns [`MdsError::Corrupt`]; success returns the
    /// counts. Takes `&mut self` (rather than `&self`) because pass 4 probes
    /// extent liveness through the store's read path — the store exposes no
    /// metadata-only index query in v0.
    pub fn self_check(&mut self) -> Result<CheckReport> {
        let mut report = CheckReport {
            orphans_repaired: self.last_reconcile_repairs,
            ..CheckReport::default()
        };
        // Passes 1-3 run in one read snapshot; pass 4 needs `&mut self.store`
        // (read path), so the snapshot is dropped first.
        let mut mapped: Vec<(Ino, u64)> = Vec::new();
        {
            let txn = self.db.begin_read().map_err(dberr)?;
            let inodes = txn.open_table(INODES).map_err(dberr)?;
            let entries = txn.open_table(DIR_ENTRIES).map_err(dberr)?;
            let fext = txn.open_table(FILE_EXTENTS).map_err(dberr)?;

            let mut refs: HashMap<Ino, u32> = HashMap::new();
            for item in entries.iter().map_err(dberr)? {
                let (_key, value) = item.map_err(dberr)?;
                report.entries += 1;
                let child = Ino::from_be_bytes(*value.value());
                if inodes.get(&ino_key(child)).map_err(dberr)?.is_none() {
                    return Err(MdsError::Corrupt(format!(
                        "dir entry references missing ino {child}"
                    )));
                }
                *refs.entry(child).or_insert(0) += 1;
            }

            for item in inodes.iter().map_err(dberr)? {
                let (key, value) = item.map_err(dberr)?;
                report.inodes += 1;
                let ino = Ino::from_be_bytes(*key.value());
                let rec = decode_rec(value.value())?;
                if ino == ROOT_INO {
                    if rec.kind != NodeKind::Dir {
                        return Err(MdsError::Corrupt(
                            "root inode is not a directory".to_string(),
                        ));
                    }
                    continue; // the root has no parent entry by definition
                }
                let count = refs.get(&ino).copied().unwrap_or(0);
                match rec.kind {
                    NodeKind::Dir => {
                        if count != 1 {
                            return Err(MdsError::Corrupt(format!(
                                "dir ino {ino} has {count} parent entries, want 1"
                            )));
                        }
                        if rec.nlink != 1 {
                            return Err(MdsError::Corrupt(format!(
                                "dir ino {ino} has nlink {}, want 1",
                                rec.nlink
                            )));
                        }
                    }
                    NodeKind::File => {
                        if rec.nlink != count {
                            return Err(MdsError::Corrupt(format!(
                                "file ino {ino} has nlink {} but {count} dir entries",
                                rec.nlink
                            )));
                        }
                    }
                }
            }

            for item in fext.iter().map_err(dberr)? {
                let (key, value) = item.map_err(dberr)?;
                report.extents += 1;
                let (ino, _offset) = decode_extent_key(key.value())?;
                let (extent_id, _len) = decode_extent_value(value.value())?;
                match inodes.get(&ino_key(ino)).map_err(dberr)? {
                    None => {
                        return Err(MdsError::Corrupt(format!(
                            "extent map row for missing ino {ino}"
                        )));
                    }
                    Some(guard) => {
                        if decode_rec(guard.value())?.kind != NodeKind::File {
                            return Err(MdsError::Corrupt(format!(
                                "extent map row on non-file ino {ino}"
                            )));
                        }
                    }
                }
                mapped.push((ino, extent_id));
            }
        }

        for (ino, extent_id) in mapped {
            if !self.extent_live(extent_id)? {
                return Err(MdsError::Corrupt(format!(
                    "extent {extent_id} of ino {ino} missing or tombstoned in the store"
                )));
            }
        }
        Ok(report)
    }
}
