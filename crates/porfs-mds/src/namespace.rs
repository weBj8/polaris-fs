//! Namespace operations: path-component resolution, streaming directory
//! listing, node creation (file/dir/symlink/special), hard links, removal,
//! and atomic rename. Every mutation commits as ONE redb write transaction
//! spanning all touched tables, so a crash never leaves a half-applied
//! namespace change.
//!
//! `dir_entries` keys are hash-ordered (`parent | xxhash64(name) | name` —
//! see `keys.rs`), so listings come back in hash order, not name order.
//! POSIX explicitly allows any `readdir` order.

use std::ops::Bound;

use redb::{ReadableDatabase, ReadableTable};

use crate::error::{MdsError, Result, dberr};
use crate::keys::{
    DIR_ENTRIES, FILE_EXTENTS, INODES, META, NEXT_INO_KEY, XATTRS, decode_dirent_name,
    dirent_bounds, dirent_key, encode_rec, ino_key,
};
use crate::mds::{
    Mds, dir_is_empty, entry_child, now_ts, rec_from, remove_extent_rows, require_dir,
    validate_name,
};
use crate::types::{
    DirEntry, Ino, InodeAttr, InodeRec, MAX_SYMLINK_LEN, NodeKind, NodeSpec, ROOT_INO, ReaddirBatch,
};

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

    /// One batch of a streaming directory listing: up to `limit` entries of
    /// directory `ino` in hash order, resuming strictly after the raw key
    /// `after` (`None` starts from the beginning). The returned
    /// [`ReaddirBatch::next_cookie`] is the raw key of the last entry —
    /// feed it back as `after` to continue; `None` means the scan is
    /// exhausted. Total work across a full walk is O(entries).
    pub fn readdir_batch(
        &self,
        ino: Ino,
        after: Option<&[u8]>,
        limit: usize,
    ) -> Result<ReaddirBatch> {
        let txn = self.db.begin_read().map_err(dberr)?;
        let inodes = txn.open_table(INODES).map_err(dberr)?;
        require_dir(&rec_from(&inodes, ino)?, ino)?;
        let entries = txn.open_table(DIR_ENTRIES).map_err(dberr)?;
        let (start, end) = dirent_bounds(ino);
        let mut out: Vec<crate::types::DirEntry> = Vec::new();
        let mut last_key: Option<Vec<u8>> = None;
        {
            let lower: Bound<&[u8]> = match after {
                None => Bound::Included(start.as_slice()),
                Some(cookie) => Bound::Excluded(cookie),
            };
            let upper: Bound<&[u8]> = Bound::Excluded(end.as_slice());
            let mut range = entries.range::<&[u8]>((lower, upper)).map_err(dberr)?;
            while out.len() < limit {
                let Some(item) = range.next() else { break };
                let (key, value) = item.map_err(dberr)?;
                let name = decode_dirent_name(key.value())?.to_string();
                let child = Ino::from_be_bytes(*value.value());
                let crec = rec_from(&inodes, child)?;
                let cookie = key.value().to_vec();
                last_key = Some(cookie.clone());
                out.push(DirEntry {
                    name,
                    ino: child,
                    kind: crec.kind,
                    cookie,
                });
            }
            // One more probe tells whether the walk is exhausted.
            if range.next().is_none() {
                last_key = None;
            }
        }
        Ok(ReaddirBatch {
            entries: out,
            next_cookie: last_key,
        })
    }

    /// List every entry of directory `ino`, in hash order (the B-tree scan
    /// order; POSIX allows any order). Collects the whole listing — callers
    /// serving paginated consumers (FUSE `readdir`) must use
    /// [`Mds::readdir_batch`] instead.
    pub fn readdir(&self, ino: Ino) -> Result<Vec<(String, Ino, NodeKind)>> {
        let mut all = Vec::new();
        let mut cookie = None;
        loop {
            let batch = self.readdir_batch(ino, cookie.as_deref(), 4096)?;
            let done = batch.next_cookie.is_none();
            all.extend(batch.entries.into_iter().map(|e| (e.name, e.ino, e.kind)));
            match batch.next_cookie {
                Some(next) => cookie = Some(next),
                None => {
                    debug_assert!(done);
                    return Ok(all);
                }
            }
        }
    }

    /// Create a regular file `name` under `parent` with permission bits
    /// `mode`, owned by `uid`/`gid`, and return its inode number.
    pub fn create(
        &mut self,
        parent: Ino,
        name: &str,
        mode: u32,
        uid: u32,
        gid: u32,
    ) -> Result<Ino> {
        self.create_node(
            parent,
            name,
            NodeSpec {
                kind: NodeKind::File,
                mode,
                rdev: 0,
                uid,
                gid,
            },
        )
    }

    /// Create a directory `name` under `parent` with permission bits `mode`,
    /// owned by `uid`/`gid`, and return its inode number.
    pub fn mkdir(&mut self, parent: Ino, name: &str, mode: u32, uid: u32, gid: u32) -> Result<Ino> {
        self.create_node(
            parent,
            name,
            NodeSpec {
                kind: NodeKind::Dir,
                mode,
                rdev: 0,
                uid,
                gid,
            },
        )
    }

    /// Create a special node `name` under `parent` from `spec`:
    /// [`NodeKind::Fifo`], [`NodeKind::Socket`], [`NodeKind::Chr`] or
    /// [`NodeKind::Blk`] (with device id `spec.rdev`), or a plain
    /// [`NodeKind::File`]. Directories and symlinks have their own
    /// constructors.
    pub fn mknod(&mut self, parent: Ino, name: &str, spec: NodeSpec) -> Result<Ino> {
        match spec.kind {
            NodeKind::Fifo | NodeKind::Socket | NodeKind::Chr | NodeKind::Blk | NodeKind::File => {}
            NodeKind::Dir | NodeKind::Symlink => {
                return Err(MdsError::InvalidOp(format!(
                    "mknod cannot create {:?}; use mkdir/symlink",
                    spec.kind
                )));
            }
        }
        let mut spec = spec;
        if !matches!(spec.kind, NodeKind::Chr | NodeKind::Blk) {
            spec.rdev = 0;
        }
        self.create_node(parent, name, spec)
    }

    /// Create a symlink `name` under `parent` pointing at `target` (stored
    /// verbatim in the inode record, at most [`MAX_SYMLINK_LEN`] bytes).
    /// The link gets mode 0o777, size = target length, nlink 1.
    pub fn symlink(
        &mut self,
        parent: Ino,
        name: &str,
        target: &str,
        uid: u32,
        gid: u32,
    ) -> Result<Ino> {
        validate_name(name)?;
        if target.is_empty() {
            return Err(MdsError::InvalidOp("empty symlink target".to_string()));
        }
        if target.len() > MAX_SYMLINK_LEN {
            return Err(MdsError::TooBig(format!(
                "symlink target is {} bytes, max {MAX_SYMLINK_LEN}",
                target.len()
            )));
        }
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
            ino = alloc_ino(&meta)?;
            meta.insert(NEXT_INO_KEY, ino + 1).map_err(dberr)?;
            let now = now_ts();
            let rec = InodeRec::new_symlink(target.as_bytes(), uid, gid, now);
            let bytes = encode_rec(&rec)?;
            inodes
                .insert(&ino_key(ino), bytes.as_slice())
                .map_err(dberr)?;
            entries
                .insert(key.as_slice(), &ino.to_be_bytes())
                .map_err(dberr)?;
            touch_parent(&mut inodes, parent, now)?;
        }
        txn.commit().map_err(dberr)?;
        Ok(ino)
    }

    /// Read the target of symlink `ino` ([`MdsError::InvalidOp`] for other
    /// kinds).
    pub fn readlink(&self, ino: Ino) -> Result<String> {
        let rec = self.get_rec(ino)?;
        if rec.kind != NodeKind::Symlink {
            return Err(MdsError::InvalidOp(format!("ino {ino} is not a symlink")));
        }
        let target = rec
            .link_target
            .ok_or_else(|| MdsError::Corrupt(format!("symlink ino {ino} has no target")))?;
        String::from_utf8(target)
            .map_err(|_| MdsError::Corrupt(format!("symlink ino {ino} target is not utf-8")))
    }

    /// Hard-link `name` under directory `parent` to the existing non-
    /// directory node `ino` (POSIX: hard links to directories are rejected
    /// with [`MdsError::IsDir`]).
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
            let now = now_ts();
            rec.nlink += 1;
            rec.ctime = now;
            let bytes = encode_rec(&rec)?;
            inodes
                .insert(&ino_key(ino), bytes.as_slice())
                .map_err(dberr)?;
            touch_parent(&mut inodes, parent, now)?;
        }
        txn.commit().map_err(dberr)?;
        Ok(())
    }

    /// Remove the non-directory node `name` from directory `parent`. When
    /// the inode's link count reaches zero the inode, its xattrs, its
    /// extent map, and its extents are dropped (the extents are tombstoned
    /// in the store after the metadata commit — the commit is the atomic
    /// point).
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
            let now = now_ts();
            if rec.nlink > 1 {
                let mut rec = rec;
                rec.nlink -= 1;
                rec.ctime = now;
                let bytes = encode_rec(&rec)?;
                inodes
                    .insert(&ino_key(child), bytes.as_slice())
                    .map_err(dberr)?;
            } else {
                inodes.remove(&ino_key(child)).map_err(dberr)?;
                remove_xattr_rows(&txn, child)?;
                let mut fext = txn.open_table(FILE_EXTENTS).map_err(dberr)?;
                dead_extents = remove_extent_rows(&mut fext, child)?;
            }
            touch_parent(&mut inodes, parent, now)?;
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
            remove_xattr_rows(&txn, child)?;
            touch_parent(&mut inodes, parent, now_ts())?;
        }
        txn.commit().map_err(dberr)?;
        Ok(())
    }

    /// Rename `old_name` under `old_parent` to `new_name` under
    /// `new_parent`, atomically (one redb commit). POSIX replacement rules:
    /// a non-directory target is replaced; a directory target must be an
    /// empty directory; directory-over-non-directory and vice versa fail;
    /// renaming a directory into its own subtree fails with
    /// [`MdsError::InvalidOp`] (POSIX `EINVAL`); renaming onto the same
    /// name (or onto another hard link of the same inode) is a no-op.
    /// Cross-directory renames update both parents' mtime/ctime; the
    /// renamed node's ctime is always refreshed. Open files keep working:
    /// the inode number never changes.
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
            let src = entry_child(&entries, old_parent, old_name)?;
            if src == ROOT_INO {
                return Err(MdsError::InvalidOp("cannot rename the root".to_string()));
            }
            let mut src_rec = rec_from(&inodes, src)?;
            if src_rec.kind == NodeKind::Dir
                && Self::subtree_contains(&entries, &inodes, src, new_parent)?
            {
                return Err(MdsError::InvalidOp(format!(
                    "rename: {new_name:?} is inside the subtree of {old_name:?}"
                )));
            }
            let old_key = dirent_key(old_parent, old_name);
            let new_key = dirent_key(new_parent, new_name);
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
                match (src_rec.kind.is_dir(), dst_rec.kind.is_dir()) {
                    (true, false) => {
                        return Err(MdsError::NotDir(format!(
                            "rename: target {new_name:?} is not a directory, source is"
                        )));
                    }
                    (false, true) => {
                        return Err(MdsError::IsDir(format!(
                            "rename: target {new_name:?} is a directory, source is not"
                        )));
                    }
                    (true, true) => {
                        if !dir_is_empty(&entries, dst)? {
                            return Err(MdsError::NotEmpty(format!(
                                "rename: target directory {new_name:?} is not empty"
                            )));
                        }
                    }
                    (false, false) => {}
                }
                entries.remove(new_key.as_slice()).map_err(dberr)?;
                if !dst_rec.kind.is_dir() && dst_rec.nlink > 1 {
                    let mut rec = dst_rec;
                    rec.nlink -= 1;
                    rec.ctime = now_ts();
                    let bytes = encode_rec(&rec)?;
                    inodes
                        .insert(&ino_key(dst), bytes.as_slice())
                        .map_err(dberr)?;
                } else {
                    inodes.remove(&ino_key(dst)).map_err(dberr)?;
                    remove_xattr_rows(&txn, dst)?;
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
            let now = now_ts();
            src_rec.ctime = now;
            let bytes = encode_rec(&src_rec)?;
            inodes
                .insert(&ino_key(src), bytes.as_slice())
                .map_err(dberr)?;
            touch_parent(&mut inodes, old_parent, now)?;
            if new_parent != old_parent {
                touch_parent(&mut inodes, new_parent, now)?;
            }
        }
        txn.commit().map_err(dberr)?;
        self.discard_tolerant(&dead_extents)?;
        Ok(())
    }

    /// Depth-first check whether `node` lies inside the directory subtree
    /// rooted at `root` (including `root` itself). Cyclic structures are
    /// impossible by construction (a directory always has exactly one
    /// parent entry).
    fn subtree_contains(
        entries: &impl ReadableTable<&'static [u8], &'static [u8; 8]>,
        inodes: &impl ReadableTable<&'static [u8; 8], &'static [u8]>,
        root: Ino,
        node: Ino,
    ) -> Result<bool> {
        let mut stack = vec![root];
        while let Some(dir) = stack.pop() {
            if dir == node {
                return Ok(true);
            }
            let (start, end) = dirent_bounds(dir);
            for item in entries
                .range(start.as_slice()..end.as_slice())
                .map_err(dberr)?
            {
                let (_key, value) = item.map_err(dberr)?;
                let child = Ino::from_be_bytes(*value.value());
                let crec = rec_from(inodes, child)?;
                if crec.kind == NodeKind::Dir {
                    stack.push(child);
                }
            }
        }
        Ok(false)
    }

    /// Shared create/mkdir/mknod body: validate, allocate an inode, insert
    /// the inode record and the directory entry in one transaction.
    fn create_node(&mut self, parent: Ino, name: &str, spec: NodeSpec) -> Result<Ino> {
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
            ino = alloc_ino(&meta)?;
            meta.insert(NEXT_INO_KEY, ino + 1).map_err(dberr)?;
            let now = now_ts();
            let mut rec = InodeRec::new(spec.kind, spec.mode, spec.uid, spec.gid, now);
            rec.rdev = spec.rdev;
            let bytes = encode_rec(&rec)?;
            inodes
                .insert(&ino_key(ino), bytes.as_slice())
                .map_err(dberr)?;
            entries
                .insert(key.as_slice(), &ino.to_be_bytes())
                .map_err(dberr)?;
            touch_parent(&mut inodes, parent, now)?;
        }
        txn.commit().map_err(dberr)?;
        Ok(ino)
    }
}

/// Allocate the next inode number from the `meta` table.
fn alloc_ino(meta: &impl ReadableTable<&'static str, u64>) -> Result<Ino> {
    meta.get(NEXT_INO_KEY)
        .map_err(dberr)?
        .map(|guard| guard.value())
        .ok_or_else(|| MdsError::Corrupt("meta.next_ino row missing".to_string()))
}

/// POSIX directory-side effect of a namespace mutation: bump the parent
/// directory's mtime and ctime.
fn touch_parent(
    inodes: &mut redb::Table<'_, &'static [u8; 8], &'static [u8]>,
    parent: Ino,
    now: (i64, u32),
) -> Result<()> {
    let mut rec = rec_from(inodes, parent)?;
    rec.mtime = now;
    rec.ctime = now;
    let bytes = encode_rec(&rec)?;
    inodes
        .insert(&ino_key(parent), bytes.as_slice())
        .map_err(dberr)?;
    Ok(())
}

/// Delete every `xattrs` row of `ino` inside the current write transaction.
pub(crate) fn remove_xattr_rows(txn: &redb::WriteTransaction, ino: Ino) -> Result<u64> {
    let mut xattrs = txn.open_table(XATTRS).map_err(dberr)?;
    let (start, end) = crate::keys::xattr_bounds(ino);
    let mut keys = Vec::new();
    for item in xattrs
        .range(start.as_slice()..end.as_slice())
        .map_err(dberr)?
    {
        let (key, _value) = item.map_err(dberr)?;
        keys.push(key.value().to_vec());
    }
    let count = keys.len() as u64;
    for key in &keys {
        xattrs.remove(key.as_slice()).map_err(dberr)?;
    }
    Ok(count)
}
