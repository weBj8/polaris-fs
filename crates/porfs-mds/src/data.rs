//! The file data path: log-structured copy-on-write on top of the extent
//! store. An overwrite never mutates an extent in place — the new bytes are
//! appended as new extents, the metadata commit repoints the map, and only
//! then are the replaced extents tombstoned.
//!
//! Durability order (phase contract): `store.append` first, then the redb
//! commit, with no per-write fdatasync. [`Mds::fsync`] is the barrier that
//! makes appended data durable (redb commits already are, per transaction).

use porfs_format::EXTENT_DATA_MAX;
use porfs_store::ExtentId;

use crate::error::{MdsError, Result, dberr};
use crate::keys::{FILE_EXTENTS, INODES, encode_extent_value, encode_rec, extent_key, ino_key};
use crate::mds::{ExtentRow, Mds, now_ts};
use crate::types::{Ino, InodeAttr, InodeRec, NodeKind, SetAttr};

impl Mds {
    /// Write `data` at `offset` of file `ino`; returns the number of bytes
    /// written (`data.len()`). The file grows to cover the write; unwritten
    /// gaps read back as zeros.
    ///
    /// Data is split into `EXTENT_DATA_MAX`-sized chunks. Per chunk, ranges
    /// of the map fully covered by the chunk are replaced; partial head/tail
    /// overlaps are read-modify-write merged (their kept bytes are rewritten
    /// as new extents). Each chunk commits as one redb transaction after its
    /// extents were appended.
    pub fn write(&mut self, ino: Ino, offset: u64, data: &[u8]) -> Result<usize> {
        let mut rec = self.get_rec(ino)?;
        if rec.kind != NodeKind::File {
            return Err(MdsError::IsDir(format!("ino {ino} is a directory")));
        }
        if data.is_empty() {
            return Ok(0);
        }
        let mut written = 0usize;
        for chunk in data.chunks(EXTENT_DATA_MAX as usize) {
            let cstart = offset.saturating_add(written as u64);
            self.write_chunk(ino, &mut rec, cstart, chunk)?;
            written += chunk.len();
        }
        Ok(written)
    }

    /// Read up to `len` bytes at `offset` of file `ino`, clamped to the file
    /// size (short reads at EOF, empty past it). Holes read back as zeros.
    pub fn read(&mut self, ino: Ino, offset: u64, len: u64) -> Result<Vec<u8>> {
        let rec = self.get_rec(ino)?;
        if rec.kind != NodeKind::File {
            return Err(MdsError::IsDir(format!("ino {ino} is a directory")));
        }
        if len == 0 || offset >= rec.size {
            return Ok(Vec::new());
        }
        let end = offset.saturating_add(len).min(rec.size);
        let mut out = vec![0u8; (end - offset) as usize];
        let rows = self.extent_rows(ino, offset, end)?;
        if rows.is_empty() {
            return Ok(out); // pure hole
        }
        let ids: Vec<ExtentId> = rows.iter().map(|row| row.id).collect();
        let bufs = self.store.read_batch(&ids)?;
        for (row, buf) in rows.iter().zip(bufs.iter()) {
            let clip_start = row.off.max(offset);
            let clip_end = row.end().min(end);
            let dst = (clip_start - offset) as usize;
            let src = (clip_start - row.off) as usize;
            let n = (clip_end - clip_start) as usize;
            out[dst..dst + n].copy_from_slice(&buf[src..src + n]);
        }
        Ok(out)
    }

    /// Apply the field-optional attribute update. `size` resizes the file
    /// (truncate semantics: shrink discards trailing extents, growth reads
    /// back as zeros). `ctime` is always refreshed. Returns the new
    /// attributes.
    pub fn setattr(&mut self, ino: Ino, set: SetAttr) -> Result<InodeAttr> {
        let mut rec = self.get_rec(ino)?;
        if let Some(size) = set.size {
            if rec.kind == NodeKind::Dir {
                return Err(MdsError::IsDir(format!(
                    "ino {ino} is a directory; cannot resize"
                )));
            }
            if rec.kind != NodeKind::File {
                return Err(MdsError::InvalidOp(format!(
                    "ino {ino} is not a regular file; cannot resize"
                )));
            }
            if size != rec.size {
                self.truncate(ino, &mut rec, size)?;
            }
        }
        if let Some(mode) = set.mode {
            rec.mode = mode;
        }
        if let Some(uid) = set.uid {
            rec.uid = uid;
        }
        if let Some(gid) = set.gid {
            rec.gid = gid;
        }
        if let Some(atime) = set.atime {
            rec.atime = atime;
        }
        if let Some(mtime) = set.mtime {
            rec.mtime = mtime;
        }
        rec.ctime = now_ts();
        let txn = self.db.begin_write().map_err(dberr)?;
        {
            let mut inodes = txn.open_table(INODES).map_err(dberr)?;
            let bytes = encode_rec(&rec)?;
            inodes
                .insert(&ino_key(ino), bytes.as_slice())
                .map_err(dberr)?;
        }
        txn.commit().map_err(dberr)?;
        Ok(rec.attr(ino))
    }

    /// Durability barrier for `ino`: confirms every extent appended so far
    /// (the store's group commit — fdatasync of the data region plus the
    /// dual-superblock update). redb commits are already durable per
    /// transaction, so after `fsync` returns, both the namespace state and
    /// the file data of `ino` survive a crash. Legal on any node kind —
    /// fsync on a directory is a metadata barrier.
    pub fn fsync(&mut self, ino: Ino) -> Result<()> {
        self.get_rec(ino)?; // fsync on a missing inode is an error
        self.store.sync()?;
        Ok(())
    }

    /// Data-only durability barrier (POSIX `fdatasync`): confirms appended
    /// extents like [`Mds::fsync`], but treats the inode as data-only —
    /// redb metadata commits are durable per transaction, so there is no
    /// separate metadata barrier to skip on this implementation; the
    /// distinction matters for the API contract (P6 RPC) and for rejecting
    /// non-file nodes ([`MdsError::InvalidOp`]).
    pub fn fdatasync(&mut self, ino: Ino) -> Result<()> {
        let rec = self.get_rec(ino)?;
        if rec.kind != NodeKind::File {
            return Err(MdsError::InvalidOp(format!(
                "fdatasync on non-file ino {ino}"
            )));
        }
        self.store.sync()?;
        Ok(())
    }

    /// Seek within the extent map (POSIX `SEEK_DATA` / `SEEK_HOLE`):
    /// - data: the smallest offset `>= pos` covered by an extent row —
    ///   `pos` itself when it falls inside a row; [`MdsError::OutOfRange`]
    ///   (POSIX `ENXIO`) when no data remains at or past `pos`;
    /// - hole: the smallest offset `>= pos` NOT covered by any row,
    ///   clamped to the file size (the implicit hole at EOF);
    ///   [`MdsError::OutOfRange`] when `pos` is already at/past EOF.
    pub fn seek(&mut self, ino: Ino, pos: u64, data: bool) -> Result<u64> {
        let rec = self.get_rec(ino)?;
        if rec.kind != NodeKind::File {
            return Err(MdsError::InvalidOp(format!("seek on non-file ino {ino}")));
        }
        if pos >= rec.size {
            return Err(MdsError::OutOfRange(format!(
                "seek from {pos} at/past EOF {}",
                rec.size
            )));
        }
        let rows = self.extent_rows(ino, pos, rec.size)?;
        if data {
            // rows all intersect [pos, size), so the first row is the answer.
            match rows.first() {
                Some(row) if row.off <= pos => Ok(pos),
                Some(row) => Ok(row.off),
                None => Err(MdsError::OutOfRange(format!("no data at or past {pos}"))),
            }
        } else {
            for row in &rows {
                if row.off > pos {
                    return Ok(pos); // pos is in a hole before the next row
                }
                if row.end() > pos {
                    return Ok(row.end().min(rec.size)); // skip past the row
                }
            }
            Ok(pos) // no rows at/past pos: pos itself is in a hole
        }
    }

    /// Deallocate the range `[offset, offset + len)` without changing the
    /// file size (POSIX `fallocate(FALLOC_FL_PUNCH_HOLE|FALLOC_FL_KEEP_SIZE)`).
    /// Map rows fully covered by the range are dropped (the region becomes
    /// a hole and reads back as zeros); rows straddling the range edge have
    /// their punched bytes zero-filled in place (rewritten as a new extent
    /// of the same span) — v1 keeps rows unsplit, correctness first.
    pub fn punch_hole(&mut self, ino: Ino, offset: u64, len: u64) -> Result<()> {
        let mut rec = self.get_rec(ino)?;
        if rec.kind != NodeKind::File {
            return Err(MdsError::InvalidOp(format!(
                "punch_hole on non-file ino {ino}"
            )));
        }
        if len == 0 {
            return Ok(());
        }
        let end = offset.saturating_add(len);
        if offset >= rec.size {
            return Ok(()); // past EOF: nothing to punch
        }
        let end = end.min(rec.size);
        let rows = self.extent_rows(ino, offset, end)?;
        if rows.is_empty() {
            return Ok(());
        }
        // Rewrite straddlers with zeroed punch bytes BEFORE the metadata
        // commit; fully covered rows just die.
        let mut replacements: Vec<(u64, Vec<u8>)> = Vec::new();
        let mut dead: Vec<ExtentId> = Vec::new();
        for row in &rows {
            let covered = row.off >= offset && row.end() <= end;
            let old = self.store.read(row.id)?;
            dead.push(row.id);
            if covered {
                continue;
            }
            let mut bytes = old;
            let zstart = offset.max(row.off) - row.off;
            let zend = end.min(row.end()) - row.off;
            bytes[zstart as usize..zend as usize].fill(0);
            replacements.push((row.off, bytes));
        }
        let items: Vec<(u64, u64, &[u8])> = replacements
            .iter()
            .map(|(off, bytes)| (ino, *off, bytes.as_slice()))
            .collect();
        let new_ids = self.store.append_batch(&items)?;
        let txn = self.db.begin_write().map_err(dberr)?;
        {
            let mut fext = txn.open_table(FILE_EXTENTS).map_err(dberr)?;
            let mut inodes = txn.open_table(INODES).map_err(dberr)?;
            for row in &rows {
                fext.remove(extent_key(ino, row.off).as_slice())
                    .map_err(dberr)?;
            }
            for ((off, bytes), id) in replacements.iter().zip(new_ids.iter()) {
                let value = encode_extent_value(*id, bytes.len() as u64);
                fext.insert(extent_key(ino, *off).as_slice(), &value)
                    .map_err(dberr)?;
            }
            let now = now_ts();
            rec.mtime = now;
            rec.ctime = now;
            let bytes = encode_rec(&rec)?;
            inodes
                .insert(&ino_key(ino), bytes.as_slice())
                .map_err(dberr)?;
        }
        txn.commit().map_err(dberr)?;
        self.discard_tolerant(&dead)?;
        Ok(())
    }

    /// One `<= EXTENT_DATA_MAX` chunk of [`Mds::write`]: merge boundary
    /// overlaps, append the new extents, commit the map, tombstone the
    /// replaced extents.
    fn write_chunk(
        &mut self,
        ino: Ino,
        rec: &mut InodeRec,
        cstart: u64,
        chunk: &[u8],
    ) -> Result<()> {
        debug_assert!(!chunk.is_empty() && chunk.len() as u64 <= EXTENT_DATA_MAX);
        let cend = cstart.saturating_add(chunk.len() as u64);
        let rows = self.extent_rows(ino, cstart, cend)?;
        // RMW the partial head/tail overlaps: the kept bytes of the boundary
        // extents are rewritten as new extents. Fully covered extents are
        // simply replaced (discarded below, no piece rewritten).
        let mut pieces: Vec<(u64, Vec<u8>)> = Vec::with_capacity(3);
        let mut replaced: Vec<ExtentId> = Vec::with_capacity(rows.len());
        for row in &rows {
            let head = row.off < cstart;
            let tail = row.end() > cend;
            if head || tail {
                let old = self.store.read(row.id)?;
                if head {
                    pieces.push((row.off, old[..(cstart - row.off) as usize].to_vec()));
                }
                if tail {
                    pieces.push((cend, old[(cend - row.off) as usize..].to_vec()));
                }
            }
            replaced.push(row.id);
        }
        pieces.push((cstart, chunk.to_vec()));
        pieces.sort_by_key(|piece| piece.0);
        // Durability order: extent data is appended BEFORE the metadata
        // commit points at it.
        let items: Vec<(u64, u64, &[u8])> = pieces
            .iter()
            .map(|(off, bytes)| (ino, *off, bytes.as_slice()))
            .collect();
        let ids = self.store.append_batch(&items)?;
        let txn = self.db.begin_write().map_err(dberr)?;
        {
            let mut fext = txn.open_table(FILE_EXTENTS).map_err(dberr)?;
            let mut inodes = txn.open_table(INODES).map_err(dberr)?;
            for row in &rows {
                fext.remove(extent_key(ino, row.off).as_slice())
                    .map_err(dberr)?;
            }
            for ((off, bytes), id) in pieces.iter().zip(ids.iter()) {
                let value = encode_extent_value(*id, bytes.len() as u64);
                fext.insert(extent_key(ino, *off).as_slice(), &value)
                    .map_err(dberr)?;
            }
            rec.size = rec.size.max(cend);
            let now = now_ts();
            rec.mtime = now;
            rec.ctime = now;
            let rec_bytes = encode_rec(rec)?;
            inodes
                .insert(&ino_key(ino), rec_bytes.as_slice())
                .map_err(dberr)?;
        }
        txn.commit().map_err(dberr)?;
        // Tombstone replaced extents AFTER the commit: a crash in between
        // leaks them (GC material for P27), it never corrupts the file.
        self.discard_tolerant(&replaced)?;
        Ok(())
    }

    /// Resize `ino` to `new_size` (`rec.size` is the current size). Shrink
    /// drops trailing map rows and discards their extents; a straddling
    /// extent's kept prefix is rewritten as a new extent at the same offset
    /// and the old one discarded. Growth only bumps the size.
    fn truncate(&mut self, ino: Ino, rec: &mut InodeRec, new_size: u64) -> Result<()> {
        let now = now_ts();
        if new_size >= rec.size {
            rec.size = new_size;
            rec.mtime = now;
            rec.ctime = now;
            let txn = self.db.begin_write().map_err(dberr)?;
            {
                let mut inodes = txn.open_table(INODES).map_err(dberr)?;
                let bytes = encode_rec(rec)?;
                inodes
                    .insert(&ino_key(ino), bytes.as_slice())
                    .map_err(dberr)?;
            }
            txn.commit().map_err(dberr)?;
            return Ok(());
        }
        let rows = self.extent_rows(ino, 0, rec.size)?;
        let mut dead: Vec<ExtentRow> = Vec::new();
        let mut straddler: Option<ExtentRow> = None;
        for row in rows {
            if row.off >= new_size {
                dead.push(row);
            } else if row.end() > new_size {
                straddler = Some(row);
            }
        }
        // Rewrite the straddler's kept prefix BEFORE the metadata commit.
        let mut replacement: Option<(u64, ExtentId, u64)> = None;
        if let Some(row) = straddler {
            let keep = (new_size - row.off) as usize;
            let old = self.store.read(row.id)?;
            let new_id = self.store.append(ino, row.off, &old[..keep])?;
            replacement = Some((row.off, new_id, keep as u64));
            dead.push(row); // the old straddler extent is discarded as well
        }
        let txn = self.db.begin_write().map_err(dberr)?;
        {
            let mut fext = txn.open_table(FILE_EXTENTS).map_err(dberr)?;
            let mut inodes = txn.open_table(INODES).map_err(dberr)?;
            for row in &dead {
                fext.remove(extent_key(ino, row.off).as_slice())
                    .map_err(dberr)?;
            }
            if let Some((off, id, len)) = replacement {
                let value = encode_extent_value(id, len);
                fext.insert(extent_key(ino, off).as_slice(), &value)
                    .map_err(dberr)?;
            }
            rec.size = new_size;
            rec.mtime = now;
            rec.ctime = now;
            let bytes = encode_rec(rec)?;
            inodes
                .insert(&ino_key(ino), bytes.as_slice())
                .map_err(dberr)?;
        }
        txn.commit().map_err(dberr)?;
        let dead_ids: Vec<ExtentId> = dead.iter().map(|row| row.id).collect();
        self.discard_tolerant(&dead_ids)?;
        Ok(())
    }
}
