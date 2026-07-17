//! Extended attributes: per-inode name/value pairs in the `xattrs` table,
//! with exact Linux semantics — `XATTR_CREATE` fails with `EEXIST` when the
//! attribute exists, `XATTR_REPLACE` fails with `ENODATA` when it does not,
//! a missing attribute reads/removes as `ENODATA`, names are capped at 255
//! bytes and values at 64 KiB. setxattr/removexattr bump the inode's ctime
//! (Linux does).

use redb::{ReadableDatabase, ReadableTable};

use crate::error::{MdsError, Result, dberr};
use crate::keys::{XATTRS, encode_rec, ino_key, xattr_bounds, xattr_key};
use crate::mds::{Mds, now_ts, rec_from};
use crate::types::{Ino, MAX_XATTR_NAME_LEN, MAX_XATTR_VALUE_LEN, XATTR_CREATE, XATTR_REPLACE};

impl Mds {
    /// Read the value of extended attribute `name` on `ino`
    /// ([`MdsError::NoAttr`] when absent).
    pub fn getxattr(&self, ino: Ino, name: &str) -> Result<Vec<u8>> {
        validate_xattr_name(name)?;
        let txn = self.db.begin_read().map_err(dberr)?;
        let inodes = txn.open_table(crate::keys::INODES).map_err(dberr)?;
        rec_from(&inodes, ino)?;
        let xattrs = txn.open_table(XATTRS).map_err(dberr)?;
        let key = xattr_key(ino, name);
        let guard = xattrs
            .get(key.as_slice())
            .map_err(dberr)?
            .ok_or_else(|| MdsError::NoAttr(format!("{name:?} on ino {ino}")))?;
        Ok(guard.value().to_vec())
    }

    /// Set extended attribute `name` on `ino` to `value`, honoring
    /// [`XATTR_CREATE`]/[`XATTR_REPLACE`] in `flags` (both together are
    /// invalid).
    pub fn setxattr(&mut self, ino: Ino, name: &str, value: &[u8], flags: u32) -> Result<()> {
        validate_xattr_name(name)?;
        if value.len() > MAX_XATTR_VALUE_LEN {
            return Err(MdsError::TooBig(format!(
                "xattr value is {} bytes, max {MAX_XATTR_VALUE_LEN}",
                value.len()
            )));
        }
        if flags & !(XATTR_CREATE | XATTR_REPLACE) != 0
            || flags & XATTR_CREATE != 0 && flags & XATTR_REPLACE != 0
        {
            return Err(MdsError::InvalidOp(format!(
                "invalid xattr flags {flags:#x}"
            )));
        }
        let txn = self.db.begin_write().map_err(dberr)?;
        {
            let mut inodes = txn.open_table(crate::keys::INODES).map_err(dberr)?;
            let mut rec = rec_from(&inodes, ino)?;
            let mut xattrs = txn.open_table(XATTRS).map_err(dberr)?;
            let key = xattr_key(ino, name);
            let present = xattrs.get(key.as_slice()).map_err(dberr)?.is_some();
            if flags & XATTR_CREATE != 0 && present {
                return Err(MdsError::Exists(format!("xattr {name:?} on ino {ino}")));
            }
            if flags & XATTR_REPLACE != 0 && !present {
                return Err(MdsError::NoAttr(format!("xattr {name:?} on ino {ino}")));
            }
            xattrs.insert(key.as_slice(), value).map_err(dberr)?;
            rec.ctime = now_ts();
            let bytes = encode_rec(&rec)?;
            inodes
                .insert(&ino_key(ino), bytes.as_slice())
                .map_err(dberr)?;
        }
        txn.commit().map_err(dberr)?;
        Ok(())
    }

    /// List the extended-attribute names of `ino` (empty when it has none).
    pub fn listxattr(&self, ino: Ino) -> Result<Vec<String>> {
        let txn = self.db.begin_read().map_err(dberr)?;
        let inodes = txn.open_table(crate::keys::INODES).map_err(dberr)?;
        rec_from(&inodes, ino)?;
        let xattrs = txn.open_table(XATTRS).map_err(dberr)?;
        let (start, end) = xattr_bounds(ino);
        let mut out = Vec::new();
        for item in xattrs
            .range(start.as_slice()..end.as_slice())
            .map_err(dberr)?
        {
            let (key, _value) = item.map_err(dberr)?;
            let name = std::str::from_utf8(&key.value()[8..])
                .map_err(|_| MdsError::Corrupt(format!("non-utf8 xattr name on ino {ino}")))?;
            out.push(name.to_string());
        }
        Ok(out)
    }

    /// Remove extended attribute `name` from `ino` ([`MdsError::NoAttr`]
    /// when absent).
    pub fn removexattr(&mut self, ino: Ino, name: &str) -> Result<()> {
        validate_xattr_name(name)?;
        let txn = self.db.begin_write().map_err(dberr)?;
        {
            let mut inodes = txn.open_table(crate::keys::INODES).map_err(dberr)?;
            let mut rec = rec_from(&inodes, ino)?;
            let mut xattrs = txn.open_table(XATTRS).map_err(dberr)?;
            let key = xattr_key(ino, name);
            if xattrs.remove(key.as_slice()).map_err(dberr)?.is_none() {
                return Err(MdsError::NoAttr(format!("xattr {name:?} on ino {ino}")));
            }
            rec.ctime = now_ts();
            let bytes = encode_rec(&rec)?;
            inodes
                .insert(&ino_key(ino), bytes.as_slice())
                .map_err(dberr)?;
        }
        txn.commit().map_err(dberr)?;
        Ok(())
    }
}

/// Linux xattr name rules: nonempty, at most 255 bytes.
fn validate_xattr_name(name: &str) -> Result<()> {
    if name.is_empty() || name.len() > MAX_XATTR_NAME_LEN {
        return Err(MdsError::OutOfRange(format!(
            "xattr name is {} bytes, want 1..={MAX_XATTR_NAME_LEN}",
            name.len()
        )));
    }
    Ok(())
}
