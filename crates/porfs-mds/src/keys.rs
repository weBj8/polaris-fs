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
/// `file_extents` (legacy, schema 1): (`ino` BE ++ logical offset BE) ->
/// (`extent_id` BE ++ `len` BE). Kept only as the migration source of the
/// schema 1 -> 2 upgrade at open; never written by current code.
pub(crate) const FILE_EXTENTS: TableDefinition<&[u8], &[u8; 16]> =
    TableDefinition::new("file_extents");
/// `file_extents_v2` (schema 2): same key as the legacy table; the value is
/// a length-discriminated [`ExtentRef`] codec — 16 bytes for a local extent
/// (byte-identical to the legacy encoding) or 32 bytes for a cluster extent
/// (see [`encode_extent_value`]). redb pins the value type per table, so the
/// codec change required a NEW table rather than a type change.
pub(crate) const FILE_EXTENTS_V2: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new("file_extents_v2");
/// `xattrs`: (`ino` BE ++ attribute name bytes) -> value bytes (<= 64 KiB).
pub(crate) const XATTRS: TableDefinition<&[u8], &[u8]> = TableDefinition::new("xattrs");
/// `meta`: singleton keys: [`NEXT_INO_KEY`], [`SCHEMA_VERSION_KEY`],
/// [`CLUSTER_MODE_KEY`], [`REPLICAS_KEY`].
pub(crate) const META: TableDefinition<&str, u64> = TableDefinition::new("meta");
/// `cluster_config`: [`CLUSTER_MEMBERSHIP_KEY`] -> bincode-2
/// (`config::standard()`) serialized `Vec<MemberSpec>`. Present only on
/// cluster-mode databases.
pub(crate) const CLUSTER_CONFIG: TableDefinition<&str, &[u8]> =
    TableDefinition::new("cluster_config");

/// The `meta` row holding the next inode number to allocate.
pub(crate) const NEXT_INO_KEY: &str = "next_ino";
/// The `meta` row holding the MDS-internal metadata schema version. Absent
/// means a pre-P5 (legacy) database, which this code rejects on open — the
/// dev-box precedent, same as the v0->v2 extent-format rejection.
pub(crate) const SCHEMA_VERSION_KEY: &str = "schema_version";
/// The `meta` row holding the data-plane mode: absent/0 = local extent
/// device, 1 = cluster of chunkservers.
pub(crate) const CLUSTER_MODE_KEY: &str = "cluster_mode";
/// The `meta` row holding the cluster replica count (cluster mode only).
pub(crate) const REPLICAS_KEY: &str = "replicas";
/// The `cluster_config` row holding the serialized cluster membership.
pub(crate) const CLUSTER_MEMBERSHIP_KEY: &str = "membership";
/// Current metadata schema version: P5 introduced hash-ordered `dir_entries`
/// keys, the `xattrs` table, and symlink/special inode records (schema 1);
/// P12.5 replaced `file_extents` with `file_extents_v2` and added
/// `cluster_config` (schema 2). Schema 1 opens via the migration path.
pub(crate) const MDS_SCHEMA_VERSION: u64 = 2;

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

/// Where one extent's payload lives: on the local extent device
/// (`Local`: a device extent id) or on chunkservers (`Remote`: a
/// primary copy plus, with two replicas, a secondary copy — each a
/// `(server index, server-local extent id)` pair into the persisted
/// cluster membership).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExtentRef {
    /// Extent id on the local extent device.
    Local(u64),
    /// Primary copy `(server, extent)` and optional replica copy on the
    /// cluster's chunkservers.
    Remote {
        /// Membership index of the primary chunkserver.
        server: u32,
        /// Server-local extent id of the primary copy.
        extent: u64,
        /// Replica copy `(server, extent)` when replicas = 2.
        replica: Option<(u32, u64)>,
    },
}

impl ExtentRef {
    /// Number of independently addressable copies of the payload.
    pub(crate) fn copies(self) -> usize {
        match self {
            ExtentRef::Local(_) => 1,
            ExtentRef::Remote { replica, .. } => 1 + usize::from(replica.is_some()),
        }
    }
}

/// One chunkserver endpoint of the persisted cluster membership: network
/// address, physical topology tags (placement input), and the extent-store
/// UUID pinned at format time (the server-identity guard of `Mds::open`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct MemberSpec {
    /// Network endpoint of the chunkserver.
    pub(crate) addr: std::net::SocketAddr,
    /// Rack tag (replica-separation boundary).
    pub(crate) rack: String,
    /// Chassis tag (finer-grained topology, reserved for future policy).
    pub(crate) chassis: String,
    /// Extent-store UUID learned from the first `HelloAck` at format time.
    pub(crate) uuid: [u8; 16],
}

/// Marker for "no replica" in the 32-byte cluster value form.
const NO_REPLICA: u32 = u32::MAX;

/// Encode a `file_extents_v2` value, length-discriminated:
/// - 16 bytes, local: `extent id BE ++ len BE` — byte-identical to the
///   legacy schema-1 encoding, so migrated rows need no rewrite;
/// - 32 bytes, cluster: `server u32 LE ++ extent u64 LE ++ replica server
///   u32 LE (u32::MAX = none) ++ replica extent u64 LE ++ len u64 LE`.
pub(crate) fn encode_extent_value(eref: &ExtentRef, len: u64) -> Vec<u8> {
    match *eref {
        ExtentRef::Local(extent_id) => {
            let mut value = Vec::with_capacity(16);
            value.extend_from_slice(&extent_id.to_be_bytes());
            value.extend_from_slice(&len.to_be_bytes());
            value
        }
        ExtentRef::Remote {
            server,
            extent,
            replica,
        } => {
            let (replica_server, replica_extent) = replica.unwrap_or((NO_REPLICA, 0));
            let mut value = Vec::with_capacity(32);
            value.extend_from_slice(&server.to_le_bytes());
            value.extend_from_slice(&extent.to_le_bytes());
            value.extend_from_slice(&replica_server.to_le_bytes());
            value.extend_from_slice(&replica_extent.to_le_bytes());
            value.extend_from_slice(&len.to_le_bytes());
            value
        }
    }
}

/// Decode a `file_extents_v2` value into `(ExtentRef, len)`; any length
/// other than 16 or 32 bytes is metadata corruption.
pub(crate) fn decode_extent_value(value: &[u8]) -> Result<(ExtentRef, u64)> {
    match value.len() {
        16 => {
            let extent_id = u64::from_be_bytes(value[..8].try_into().map_err(|_| {
                MdsError::Corrupt("file_extents value id slice is not 8 bytes".to_string())
            })?);
            let len = u64::from_be_bytes(value[8..].try_into().map_err(|_| {
                MdsError::Corrupt("file_extents value len slice is not 8 bytes".to_string())
            })?);
            Ok((ExtentRef::Local(extent_id), len))
        }
        32 => {
            let server = u32::from_le_bytes(value[0..4].try_into().map_err(|_| {
                MdsError::Corrupt("file_extents value server slice is not 4 bytes".to_string())
            })?);
            let extent = u64::from_le_bytes(value[4..12].try_into().map_err(|_| {
                MdsError::Corrupt("file_extents value extent slice is not 8 bytes".to_string())
            })?);
            let replica_server = u32::from_le_bytes(value[12..16].try_into().map_err(|_| {
                MdsError::Corrupt(
                    "file_extents value replica server slice is not 4 bytes".to_string(),
                )
            })?);
            let replica_extent = u64::from_le_bytes(value[16..24].try_into().map_err(|_| {
                MdsError::Corrupt(
                    "file_extents value replica extent slice is not 8 bytes".to_string(),
                )
            })?);
            let len = u64::from_le_bytes(value[24..32].try_into().map_err(|_| {
                MdsError::Corrupt("file_extents value len slice is not 8 bytes".to_string())
            })?);
            let replica =
                (replica_server != NO_REPLICA).then_some((replica_server, replica_extent));
            Ok((
                ExtentRef::Remote {
                    server,
                    extent,
                    replica,
                },
                len,
            ))
        }
        other => Err(MdsError::Corrupt(format!(
            "file_extents value is {other} bytes, expected 16 or 32"
        ))),
    }
}

/// Encode the cluster membership for `cluster_config` (bincode 2, standard
/// config — the same codec family the wire protocol uses).
pub(crate) fn encode_membership(members: &[MemberSpec]) -> Result<Vec<u8>> {
    bincode2::serde::encode_to_vec(members, bincode2::config::standard())
        .map_err(|err| MdsError::Corrupt(format!("cluster membership unencodable: {err}")))
}

/// Decode the cluster membership out of `cluster_config`; any decode
/// failure is metadata corruption.
pub(crate) fn decode_membership(bytes: &[u8]) -> Result<Vec<MemberSpec>> {
    let (members, read): (Vec<MemberSpec>, usize) =
        bincode2::serde::decode_from_slice(bytes, bincode2::config::standard())
            .map_err(|err| MdsError::Corrupt(format!("cluster membership undecodable: {err}")))?;
    if read != bytes.len() {
        return Err(MdsError::Corrupt(format!(
            "trailing {} bytes after cluster membership",
            bytes.len() - read
        )));
    }
    Ok(members)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_value_roundtrip_matches_legacy_layout() {
        // The 16-byte local form is byte-identical to the legacy schema-1
        // encoding (id BE ++ len BE), so migrated rows need no rewrite.
        let legacy = {
            let mut value = [0u8; 16];
            value[..8].copy_from_slice(&0x0102_0304_0506_0708u64.to_be_bytes());
            value[8..].copy_from_slice(&4096u64.to_be_bytes());
            value
        };
        let encoded = encode_extent_value(&ExtentRef::Local(0x0102_0304_0506_0708), 4096);
        assert_eq!(encoded, legacy);
        let (eref, len) = decode_extent_value(&legacy).unwrap();
        assert_eq!(eref, ExtentRef::Local(0x0102_0304_0506_0708));
        assert_eq!(len, 4096);
    }

    #[test]
    fn remote_value_roundtrip() {
        for replica in [None, Some((3, 0xDEAD_BEEF))] {
            let eref = ExtentRef::Remote {
                server: 1,
                extent: 42,
                replica,
            };
            let encoded = encode_extent_value(&eref, 1 << 20);
            assert_eq!(encoded.len(), 32);
            let (back, len) = decode_extent_value(&encoded).unwrap();
            assert_eq!(back, eref);
            assert_eq!(len, 1 << 20);
        }
        // The no-replica marker round-trips as None, never as a server id.
        let encoded = encode_extent_value(
            &ExtentRef::Remote {
                server: 0,
                extent: 7,
                replica: None,
            },
            8,
        );
        assert_eq!(&encoded[12..16], &u32::MAX.to_le_bytes());
    }

    #[test]
    fn garbage_value_lengths_are_corrupt() {
        for len in [0usize, 8, 15, 17, 31, 33, 64] {
            let garbage = vec![0u8; len];
            assert!(
                matches!(decode_extent_value(&garbage), Err(MdsError::Corrupt(_))),
                "length {len} must be rejected"
            );
        }
    }

    #[test]
    fn membership_roundtrip() {
        let members = vec![
            MemberSpec {
                addr: "127.0.0.1:9100".parse().unwrap(),
                rack: "r1".to_string(),
                chassis: "c1".to_string(),
                uuid: [1; 16],
            },
            MemberSpec {
                addr: "[::1]:9101".parse().unwrap(),
                rack: "r2".to_string(),
                chassis: "c2".to_string(),
                uuid: [0xFF; 16],
            },
        ];
        let encoded = encode_membership(&members).unwrap();
        assert_eq!(decode_membership(&encoded).unwrap(), members);
        let mut trailing = encoded;
        trailing.push(0);
        assert!(matches!(
            decode_membership(&trailing),
            Err(MdsError::Corrupt(_))
        ));
    }
}
