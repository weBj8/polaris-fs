//! redb table schemas and key/value codecs.
//!
//! Every multi-byte integer embedded in a key is **big-endian** so that byte
//! order equals numeric order — redb orders raw byte keys lexicographically,
//! and prefix range scans (`readdir`, per-file extent scans) rely on that.
//! Values are fixed-width big-endian where they are flat, bincode for the
//! (variable-ish) inode record.
//!
//! `dir_entries` keys are (`parent ino` BE ++ `xxhash64(name)` BE ++ name):
//! hash-ordered within one directory, the GPFS extensible-hashing principle.
//! Act-2 (P7+) maps hash ranges to chunkservers, so the hash is the seam the
//! directory will be sharded along. Point lookups compute the hash and then
//! verify the full-name suffix, so same-hash different-name entries coexist.

use redb::TableDefinition;
use twox_hash::XxHash64;

use crate::error::{MdsError, Result};
use crate::types::{Ino, InodeRec};

/// `inodes`: `ino` (u64 BE) -> bincode [`InodeRec`].
pub(crate) const INODES: TableDefinition<&[u8; 8], &[u8]> = TableDefinition::new("inodes");
/// `dir_entries`: (`parent ino` BE ++ `xxhash64(name)` BE ++ name bytes) ->
/// child ino (u64 BE). `readdir` is a prefix range scan over the parent's
/// 8-byte prefix (hash order); lookups scan the 16-byte hash prefix and
/// compare the name suffix.
pub(crate) const DIR_ENTRIES: TableDefinition<&[u8], &[u8; 8]> =
    TableDefinition::new("dir_entries");
/// `file_extents`: (`ino` BE ++ logical offset BE) -> (`extent_id` BE ++ `len` BE).
/// An interval map of file data: row `[off, off + len)` is covered by extent
/// `extent_id`. Offsets are the extent's start; rows never overlap.
pub(crate) const FILE_EXTENTS: TableDefinition<&[u8], &[u8; 16]> =
    TableDefinition::new("file_extents");
/// `xattrs`: (`ino` BE ++ attribute name bytes) -> value bytes (<= 64 KiB).
pub(crate) const XATTRS: TableDefinition<&[u8], &[u8]> = TableDefinition::new("xattrs");
/// `meta`: singleton keys: [`NEXT_INO_KEY`], [`SCHEMA_VERSION_KEY`].
pub(crate) const META: TableDefinition<&str, u64> = TableDefinition::new("meta");

/// The `meta` row holding the next inode number to allocate.
pub(crate) const NEXT_INO_KEY: &str = "next_ino";
/// The `meta` row holding the MDS-internal metadata schema version. Absent
/// means a pre-P5 (legacy) database, which this code rejects on open — the
/// dev-box precedent, same as the v0->v2 extent-format rejection.
pub(crate) const SCHEMA_VERSION_KEY: &str = "schema_version";
/// Current metadata schema version: P5 introduced hash-ordered `dir_entries`
/// keys, the `xattrs` table, and symlink/special inode records.
pub(crate) const MDS_SCHEMA_VERSION: u64 = 1;

/// `inodes` table key for `ino`.
pub(crate) fn ino_key(ino: Ino) -> [u8; 8] {
    ino.to_be_bytes()
}

/// xxhash64 of a directory-entry name (seed 0): the shard key of the
/// big-directory index.
pub(crate) fn name_hash(name: &str) -> u64 {
    XxHash64::oneshot(0, name.as_bytes())
}

/// `dir_entries` key for `name` under `parent`.
pub(crate) fn dirent_key(parent: Ino, name: &str) -> Vec<u8> {
    let mut key = Vec::with_capacity(16 + name.len());
    key.extend_from_slice(&parent.to_be_bytes());
    key.extend_from_slice(&name_hash(name).to_be_bytes());
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

/// Half-open key range covering every `dir_entries` row of `parent` whose
/// name hashes to `hash` (the collision class): `[parent|hash, parent|hash+1)`.
pub(crate) fn dirent_hash_bounds(parent: Ino, hash: u64) -> ([u8; 16], [u8; 16]) {
    let mut start = [0u8; 16];
    start[..8].copy_from_slice(&parent.to_be_bytes());
    start[8..].copy_from_slice(&hash.to_be_bytes());
    // `hash + 1` overflows at u64::MAX; the end then becomes the first key
    // past the hash field entirely: (parent + 1) | 0.
    let (end_hi, end_lo) = match hash.checked_add(1) {
        Some(next) => (parent, next),
        None => (parent + 1, 0),
    };
    let mut end = [0u8; 16];
    end[..8].copy_from_slice(&end_hi.to_be_bytes());
    end[8..].copy_from_slice(&end_lo.to_be_bytes());
    (start, end)
}

/// Decode the entry name out of a `dir_entries` key read back from the
/// database; a short key is metadata corruption.
pub(crate) fn decode_dirent_name(key: &[u8]) -> Result<&str> {
    if key.len() < 16 {
        return Err(MdsError::Corrupt(format!(
            "dir_entries key is {} bytes, expected at least 16",
            key.len()
        )));
    }
    std::str::from_utf8(&key[16..])
        .map_err(|_| MdsError::Corrupt("non-utf8 dir_entries name".to_string()))
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

/// `xattrs` table key for attribute `name` on `ino`.
pub(crate) fn xattr_key(ino: Ino, name: &str) -> Vec<u8> {
    let mut key = Vec::with_capacity(8 + name.len());
    key.extend_from_slice(&ino.to_be_bytes());
    key.extend_from_slice(name.as_bytes());
    key
}

/// Half-open key range covering every `xattrs` row of `ino`.
pub(crate) fn xattr_bounds(ino: Ino) -> ([u8; 8], [u8; 8]) {
    debug_assert!(ino < u64::MAX, "ino space exhausted");
    (ino.to_be_bytes(), (ino + 1).to_be_bytes())
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
