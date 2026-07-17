//! Namespace operations: path-component resolution, directory listing, node
//! creation, hard links, removal, and atomic rename. Every mutation commits
//! as ONE redb write transaction spanning all touched tables, so a crash
//! never leaves a half-applied namespace change.

use redb::{ReadableDatabase, ReadableTable};

use crate::error::{MdsError, Result, dberr};
use crate::keys::{
    DIR_ENTRIES, FILE_EXTENTS, INODES, META, NEXT_INO_KEY, dirent_bounds, dirent_key, encode_rec,
    ino_key,
};
use crate::mds::{
    Mds, dir_is_empty, entry_child, now_ts, rec_from, remove_extent_rows, require_dir,
    validate_name,
};
use crate::types::{Ino, InodeAttr, InodeRec, NodeKind};

impl Mds {
    /// Resolve `name` under directory `parent` to an inode number.
    pub fn lookup(&self, parent: Ino, name: &str) -> Result<Ino> {
        validate_name(name)?;
        let txn = self.db.begin_read().map_err(dberr)?;
        let inodes = txn.open_table(INODES).map_err(dberr)?;
        require_dir(&rec_from(&inodes, parent)?, parent)?;
        let entries = txn.open_table(DIR_ENTRIES).map_err(dberr)?;
        entry_child(&entries, parent, name)
    }

    /// Read the public attributes of `ino`.
    pub fn getattr(&self, ino: Ino) -> Result<InodeAttr> {
        Ok(self.get_rec(ino)?.attr(ino))
    }

    /// List the entries of directory `ino`, sorted by name (the B-tree stores
    /// keys in byte order, and all keys of one directory share its 8-byte
    /// ino prefix, so the range scan is already name-ordered).
    pub fn readdir(&self, ino: Ino) -> Result<Vec<(String, Ino, NodeKind)>> {
        let txn = self.db.begin_read().map_err(dberr)?;
        let inodes = txn.open_table(INODES).map_err(dberr)?;
        require_dir(&rec_from(&inodes, ino)?, ino)?;
        let entries = txn.open_table(DIR_ENTRIES).map_err(dberr)?;
        let (start, end) = dirent_bounds(ino);
        let mut out = Vec::new();
        for item in entries
            .range(start.as_slice()..end.as_slice())
            .map_err(dberr)?
        {
            let (key, value) = item.map_err(dberr)?;
            let name = std::str::from_utf8(&key.value()[8..])
                .map_err(|_| MdsError::Corrupt(format!("non-utf8 entry name under ino {ino}")))?
                .to_string();
            let child = Ino::from_be_bytes(*value.value());
            let crec = rec_from(&inodes, child)?;
            out.push((name, child, crec.kind));
        }
        Ok(out)
    }

    /// Create a regular file `name` under `parent` with permission bits
    /// `mode` and return its inode number.
    pub fn create(&mut self, parent: Ino, name: &str, mode: u32) -> Result<Ino> {
        self.create_node(parent, name, NodeKind::File, mode)
    }

    /// Create a directory `name` under `parent` with permission bits `mode`
    /// and return its inode number.
    pub fn mkdir(&mut self, parent: Ino, name: &str, mode: u32) -> Result<Ino> {
        self.create_node(parent, name, NodeKind::Dir, mode)
    }

    /// Hard-link `name` under directory `parent` to the existing file `ino`
    /// (POSIX: hard links to directories are rejected with [`MdsError::IsDir`]).
    pub fn link(&mut self, ino: Ino, parent: Ino, name: &str) -> Result<()> {
        validate_name(name)?;
        let txn = self.db.begin_write().map_err(dberr)?;
        {
            let mut inodes = txn.open_table(INODES).map_err(dberr)?;
            let mut entries = txn.open_table(DIR_ENTRIES).map_err(dberr)?;
            let mut rec = rec_from(&inodes, ino)?;
            if rec.kind == NodeKind::Dir {
                return Err(MdsError::IsDir(format!(
                    "ino {ino} is a directory; cannot hard-link"
                )));
            }
            require_dir(&rec_from(&inodes, parent)?, parent)?;
            let key = dirent_key(parent, name);
            if entries.get(key.as_slice()).map_err(dberr)?.is_some() {
                return Err(MdsError::Exists(format!("{name:?} under ino {parent}")));
            }
            entries
                .insert(key.as_slice(), &ino.to_be_bytes())
                .map_err(dberr)?;
            rec.nlink += 1;
            rec.ctime = now_ts();
            let bytes = encode_rec(&rec)?;
            inodes
                .insert(&ino_key(ino), bytes.as_slice())
                .map_err(dberr)?;
        }
        txn.commit().map_err(dberr)?;
        Ok(())
    }

    /// Remove the file `name` from directory `parent`. When the inode's link
    /// count reaches zero the inode, its extent map, and its extents are
    /// dropped (the extents are tombstoned in the store after the metadata
    /// commit — the commit is the atomic point).
    pub fn unlink(&mut self, parent: Ino, name: &str) -> Result<()> {
        validate_name(name)?;
        let txn = self.db.begin_write().map_err(dberr)?;
        let mut dead_extents = Vec::new();
        {
            let mut inodes = txn.open_table(INODES).map_err(dberr)?;
            let mut entries = txn.open_table(DIR_ENTRIES).map_err(dberr)?;
            require_dir(&rec_from(&inodes, parent)?, parent)?;
            let key = dirent_key(parent, name);
            let child = entry_child(&entries, parent, name)?;
            let rec = rec_from(&inodes, child)?;
            if rec.kind == NodeKind::Dir {
                return Err(MdsError::IsDir(format!(
                    "{name:?} under ino {parent} is a directory; use rmdir"
                )));
            }
            entries.remove(key.as_slice()).map_err(dberr)?;
            if rec.nlink > 1 {
                let mut rec = rec;
                rec.nlink -= 1;
                rec.ctime = now_ts();
                let bytes = encode_rec(&rec)?;
                inodes
                    .insert(&ino_key(child), bytes.as_slice())
                    .map_err(dberr)?;
            } else {
                inodes.remove(&ino_key(child)).map_err(dberr)?;
                let mut fext = txn.open_table(FILE_EXTENTS).map_err(dberr)?;
                dead_extents = remove_extent_rows(&mut fext, child)?;
            }
        }
        txn.commit().map_err(dberr)?;
        self.discard_tolerant(&dead_extents)?;
        Ok(())
    }

    /// Remove the empty directory `name` from `parent`
    /// ([`MdsError::NotEmpty`] if it still has entries).
    pub fn rmdir(&mut self, parent: Ino, name: &str) -> Result<()> {
        validate_name(name)?;
        let txn = self.db.begin_write().map_err(dberr)?;
        {
            let mut inodes = txn.open_table(INODES).map_err(dberr)?;
            let mut entries = txn.open_table(DIR_ENTRIES).map_err(dberr)?;
            require_dir(&rec_from(&inodes, parent)?, parent)?;
            let key = dirent_key(parent, name);
            let child = entry_child(&entries, parent, name)?;
            let rec = rec_from(&inodes, child)?;
            if rec.kind != NodeKind::Dir {
                return Err(MdsError::NotDir(format!(
                    "{name:?} under ino {parent} is not a directory"
                )));
            }
            if !dir_is_empty(&entries, child)? {
                return Err(MdsError::NotEmpty(format!("{name:?} under ino {parent}")));
            }
            entries.remove(key.as_slice()).map_err(dberr)?;
            inodes.remove(&ino_key(child)).map_err(dberr)?;
        }
        txn.commit().map_err(dberr)?;
        Ok(())
    }

    /// Rename `old_name` under `old_parent` to `new_name` under
    /// `new_parent`, atomically (one redb commit). POSIX replacement rules:
    /// a file target is replaced; a directory target must be an empty
    /// directory; file-over-directory and directory-over-file fail.
    ///
    /// v0: POSIX also forbids renaming a directory into one of its own
    /// descendants (e.g. `rename(a, a/b)`); detecting it needs an ancestor
    /// walk and is deferred to P5 (rename edge cases) — noted here per the
    /// phase contract.
    pub fn rename(
        &mut self,
        old_parent: Ino,
        old_name: &str,
        new_parent: Ino,
        new_name: &str,
    ) -> Result<()> {
        validate_name(old_name)?;
        validate_name(new_name)?;
        if old_parent == new_parent && old_name == new_name {
            return Ok(()); // POSIX: rename onto the same name is a no-op
        }
        let txn = self.db.begin_write().map_err(dberr)?;
        let mut dead_extents = Vec::new();
        {
            let mut inodes = txn.open_table(INODES).map_err(dberr)?;
            let mut entries = txn.open_table(DIR_ENTRIES).map_err(dberr)?;
            require_dir(&rec_from(&inodes, old_parent)?, old_parent)?;
            require_dir(&rec_from(&inodes, new_parent)?, new_parent)?;
            let old_key = dirent_key(old_parent, old_name);
            let new_key = dirent_key(new_parent, new_name);
            let src = entry_child(&entries, old_parent, old_name)?;
            let mut src_rec = rec_from(&inodes, src)?;
            let dst = entries
                .get(new_key.as_slice())
                .map_err(dberr)?
                .map(|guard| Ino::from_be_bytes(*guard.value()));
            if let Some(dst) = dst {
                if dst == src {
                    // POSIX: renaming a hard link onto another name of the
                    // same inode is a no-op; nothing mutated so far, and the
                    // transaction aborts by dropping uncommitted.
                    return Ok(());
                }
                let dst_rec = rec_from(&inodes, dst)?;
                match (src_rec.kind, dst_rec.kind) {
                    (NodeKind::Dir, NodeKind::File) => {
                        return Err(MdsError::NotDir(format!(
                            "rename: target {new_name:?} is a file, source is a directory"
                        )));
                    }
                    (NodeKind::File, NodeKind::Dir) => {
                        return Err(MdsError::IsDir(format!(
                            "rename: target {new_name:?} is a directory, source is a file"
                        )));
                    }
                    (NodeKind::Dir, NodeKind::Dir) => {
                        if !dir_is_empty(&entries, dst)? {
                            return Err(MdsError::NotEmpty(format!(
                                "rename: target directory {new_name:?} is not empty"
                            )));
                        }
                    }
                    (NodeKind::File, NodeKind::File) => {}
                }
                entries.remove(new_key.as_slice()).map_err(dberr)?;
                if dst_rec.kind == NodeKind::File && dst_rec.nlink > 1 {
                    let mut rec = dst_rec;
                    rec.nlink -= 1;
                    rec.ctime = now_ts();
                    let bytes = encode_rec(&rec)?;
                    inodes
                        .insert(&ino_key(dst), bytes.as_slice())
                        .map_err(dberr)?;
                } else {
                    inodes.remove(&ino_key(dst)).map_err(dberr)?;
                    if dst_rec.kind == NodeKind::File {
                        let mut fext = txn.open_table(FILE_EXTENTS).map_err(dberr)?;
                        dead_extents = remove_extent_rows(&mut fext, dst)?;
                    }
                }
            }
            entries.remove(old_key.as_slice()).map_err(dberr)?;
            entries
                .insert(new_key.as_slice(), &src.to_be_bytes())
                .map_err(dberr)?;
            src_rec.ctime = now_ts();
            let bytes = encode_rec(&src_rec)?;
            inodes
                .insert(&ino_key(src), bytes.as_slice())
                .map_err(dberr)?;
        }
        txn.commit().map_err(dberr)?;
        self.discard_tolerant(&dead_extents)?;
        Ok(())
    }

    /// Shared create/mkdir body: validate, allocate an inode, insert the
    /// inode record and the directory entry in one transaction.
    fn create_node(&mut self, parent: Ino, name: &str, kind: NodeKind, mode: u32) -> Result<Ino> {
        validate_name(name)?;
        let txn = self.db.begin_write().map_err(dberr)?;
        let ino;
        {
            let mut inodes = txn.open_table(INODES).map_err(dberr)?;
            let mut entries = txn.open_table(DIR_ENTRIES).map_err(dberr)?;
            let mut meta = txn.open_table(META).map_err(dberr)?;
            require_dir(&rec_from(&inodes, parent)?, parent)?;
            let key = dirent_key(parent, name);
            if entries.get(key.as_slice()).map_err(dberr)?.is_some() {
                return Err(MdsError::Exists(format!("{name:?} under ino {parent}")));
            }
            ino = meta
                .get(NEXT_INO_KEY)
                .map_err(dberr)?
                .ok_or_else(|| MdsError::Corrupt("meta.next_ino row missing".to_string()))?
                .value();
            meta.insert(NEXT_INO_KEY, ino + 1).map_err(dberr)?;
            let rec = InodeRec::new(kind, mode, now_ts());
            let bytes = encode_rec(&rec)?;
            inodes
                .insert(&ino_key(ino), bytes.as_slice())
                .map_err(dberr)?;
            entries
                .insert(key.as_slice(), &ino.to_be_bytes())
                .map_err(dberr)?;
        }
        txn.commit().map_err(dberr)?;
        Ok(ino)
    }
}
