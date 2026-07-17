//! redb table schemas and key/value codecs.
//!
//! Every multi-byte integer embedded in a key is **big-endian** so that byte
//! order equals numeric order — redb orders raw byte keys lexicographically,
//! and prefix range scans (`readdir`, per-file extent scans) rely on that.
//! Values are fixed-width big-endian where they are flat, bincode for the
//! (variable-ish) inode record.

use redb::TableDefinition;

use crate::error::{MdsError, Result};
use crate::types::{Ino, InodeRec};

/// `inodes`: `ino` (u64 BE) -> bincode [`InodeRec`].
pub(crate) const INODES: TableDefinition<&[u8; 8], &[u8]> = TableDefinition::new("inodes");
/// `dir_entries`: (`parent ino` BE ++ name bytes) -> child ino (u64 BE).
/// `readdir` is a prefix range scan over the parent's 8-byte prefix.
pub(crate) const DIR_ENTRIES: TableDefinition<&[u8], &[u8; 8]> =
    TableDefinition::new("dir_entries");
/// `file_extents`: (`ino` BE ++ logical offset BE) -> (`extent_id` BE ++ `len` BE).
/// An interval map of file data: row `[off, off + len)` is covered by extent
/// `extent_id`. Offsets are the extent's start; rows never overlap.
pub(crate) const FILE_EXTENTS: TableDefinition<&[u8], &[u8; 16]> =
    TableDefinition::new("file_extents");
/// `meta`: singleton keys, currently only [`NEXT_INO_KEY`] -> u64.
pub(crate) const META: TableDefinition<&str, u64> = TableDefinition::new("meta");

/// The `meta` row holding the next inode number to allocate.
pub(crate) const NEXT_INO_KEY: &str = "next_ino";

/// `inodes` table key for `ino`.
pub(crate) fn ino_key(ino: Ino) -> [u8; 8] {
    ino.to_be_bytes()
}

/// `dir_entries` key for `name` under `parent`.
pub(crate) fn dirent_key(parent: Ino, name: &str) -> Vec<u8> {
    let mut key = Vec::with_capacity(8 + name.len());
    key.extend_from_slice(&parent.to_be_bytes());
    key.extend_from_slice(name.as_bytes());
    key
}

/// Half-open key range covering every `dir_entries` row of `parent`:
/// `[parent_be, (parent + 1)_be)`. Inos come from a monotonically increasing
/// counter, so `parent + 1` never wraps in practice.
pub(crate) fn dirent_bounds(parent: Ino) -> ([u8; 8], [u8; 8]) {
    debug_assert!(parent < u64::MAX, "ino space exhausted");
    (parent.to_be_bytes(), (parent + 1).to_be_bytes())
}

/// `file_extents` key for the extent of `ino` starting at `offset`.
pub(crate) fn extent_key(ino: Ino, offset: u64) -> [u8; 16] {
    let mut key = [0u8; 16];
    key[..8].copy_from_slice(&ino.to_be_bytes());
    key[8..].copy_from_slice(&offset.to_be_bytes());
    key
}

/// Half-open key range covering every `file_extents` row of `ino`.
pub(crate) fn extent_bounds(ino: Ino) -> ([u8; 16], [u8; 16]) {
    debug_assert!(ino < u64::MAX, "ino space exhausted");
    (extent_key(ino, 0), extent_key(ino + 1, 0))
}

/// Decode the `(ino, offset)` pair out of a `file_extents` key read back
/// from the database; a wrong length is metadata corruption.
pub(crate) fn decode_extent_key(key: &[u8]) -> Result<(Ino, u64)> {
    if key.len() != 16 {
        return Err(MdsError::Corrupt(format!(
            "file_extents key is {} bytes, expected 16",
            key.len()
        )));
    }
    let ino =
        u64::from_be_bytes(key[..8].try_into().map_err(|_| {
            MdsError::Corrupt("file_extents key ino slice is not 8 bytes".to_string())
        })?);
    let offset = u64::from_be_bytes(key[8..].try_into().map_err(|_| {
        MdsError::Corrupt("file_extents key offset slice is not 8 bytes".to_string())
    })?);
    Ok((ino, offset))
}

/// Encode a `file_extents` value.
pub(crate) fn encode_extent_value(extent_id: u64, len: u64) -> [u8; 16] {
    let mut value = [0u8; 16];
    value[..8].copy_from_slice(&extent_id.to_be_bytes());
    value[8..].copy_from_slice(&len.to_be_bytes());
    value
}

/// Decode a `file_extents` value into `(extent_id, len)`; a wrong length is
/// metadata corruption.
pub(crate) fn decode_extent_value(value: &[u8]) -> Result<(u64, u64)> {
    if value.len() != 16 {
        return Err(MdsError::Corrupt(format!(
            "file_extents value is {} bytes, expected 16",
            value.len()
        )));
    }
    let extent_id = u64::from_be_bytes(value[..8].try_into().map_err(|_| {
        MdsError::Corrupt("file_extents value id slice is not 8 bytes".to_string())
    })?);
    let len = u64::from_be_bytes(value[8..].try_into().map_err(|_| {
        MdsError::Corrupt("file_extents value len slice is not 8 bytes".to_string())
    })?);
    Ok((extent_id, len))
}

/// Serialize an inode record (bincode). Serialization of this plain struct
/// is infallible in practice; a failure would be a bug, surfaced as
/// [`MdsError::Corrupt`] rather than a panic.
pub(crate) fn encode_rec(rec: &InodeRec) -> Result<Vec<u8>> {
    bincode::serialize(rec)
        .map_err(|err| MdsError::Corrupt(format!("inode record unencodable: {err}")))
}

/// Deserialize an inode record; any decode failure is metadata corruption.
pub(crate) fn decode_rec(bytes: &[u8]) -> Result<InodeRec> {
    bincode::deserialize(bytes)
        .map_err(|err| MdsError::Corrupt(format!("inode record undecodable: {err}")))
}
