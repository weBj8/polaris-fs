//! Slot header encode/decode (contract §4): the first 64 bytes of every slot.
//! A header is valid iff magic + `header_version` == 1 match, `header_crc32c`
//! over [0, 44) matches, `payload_len` fits the slot capacity, and the
//! chunk_id is non-nil. Reserved bytes are written zero and not checked.

use crate::ChunkId;

pub(crate) const HEADER_LEN: usize = 64;
pub(crate) const HEADER_MAGIC: u32 = 0x4753_4631;
pub(crate) const HEADER_VERSION: u16 = 1;
pub(crate) const FLAG_SEALED: u16 = 1 << 0;

const OFF_MAGIC: usize = 0;
const OFF_VERSION: usize = 4;
const OFF_FLAGS: usize = 6;
const OFF_CHUNK_ID: usize = 8;
const OFF_VERSION_U64: usize = 24;
const OFF_PAYLOAD_LEN: usize = 32;
const OFF_PAYLOAD_CRC: usize = 40;
const OFF_HEADER_CRC: usize = 44;

/// Decoded slot header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SlotHeader {
    pub(crate) sealed: bool,
    pub(crate) chunk_id: ChunkId,
    pub(crate) version: u64,
    pub(crate) payload_len: u64,
    pub(crate) payload_crc32c: u32,
}

impl SlotHeader {
    /// Serialize to 64 bytes (reserved zeroed, crc over [0, 44)).
    pub(crate) fn encode(&self) -> [u8; HEADER_LEN] {
        let mut buf = [0u8; HEADER_LEN];
        buf[OFF_MAGIC..OFF_MAGIC + 4].copy_from_slice(&HEADER_MAGIC.to_le_bytes());
        buf[OFF_VERSION..OFF_VERSION + 2].copy_from_slice(&HEADER_VERSION.to_le_bytes());
        let flags = if self.sealed { FLAG_SEALED } else { 0 };
        buf[OFF_FLAGS..OFF_FLAGS + 2].copy_from_slice(&flags.to_le_bytes());
        buf[OFF_CHUNK_ID..OFF_CHUNK_ID + 16].copy_from_slice(self.chunk_id.as_bytes());
        buf[OFF_VERSION_U64..OFF_VERSION_U64 + 8].copy_from_slice(&self.version.to_le_bytes());
        buf[OFF_PAYLOAD_LEN..OFF_PAYLOAD_LEN + 8].copy_from_slice(&self.payload_len.to_le_bytes());
        buf[OFF_PAYLOAD_CRC..OFF_PAYLOAD_CRC + 4]
            .copy_from_slice(&self.payload_crc32c.to_le_bytes());
        let crc = crc32fast::hash(&buf[..OFF_HEADER_CRC]);
        buf[OFF_HEADER_CRC..OFF_HEADER_CRC + 4].copy_from_slice(&crc.to_le_bytes());
        buf
    }

    /// Parse and fully validate against the slot payload `capacity`.
    /// Returns `None` for any invalid header (slot is then treated as free).
    pub(crate) fn decode_valid(buf: &[u8], capacity: u64) -> Option<Self> {
        if buf.len() < HEADER_LEN {
            return None;
        }
        let magic = u32::from_le_bytes(buf[OFF_MAGIC..OFF_MAGIC + 4].try_into().ok()?);
        if magic != HEADER_MAGIC {
            return None;
        }
        let version = u16::from_le_bytes(buf[OFF_VERSION..OFF_VERSION + 2].try_into().ok()?);
        if version != HEADER_VERSION {
            return None;
        }
        let want = u32::from_le_bytes(buf[OFF_HEADER_CRC..OFF_HEADER_CRC + 4].try_into().ok()?);
        if crc32fast::hash(&buf[..OFF_HEADER_CRC]) != want {
            return None;
        }
        let payload_len =
            u64::from_le_bytes(buf[OFF_PAYLOAD_LEN..OFF_PAYLOAD_LEN + 8].try_into().ok()?);
        if payload_len > capacity {
            return None;
        }
        let mut id = [0u8; 16];
        id.copy_from_slice(&buf[OFF_CHUNK_ID..OFF_CHUNK_ID + 16]);
        let chunk_id = ChunkId::from(id);
        if chunk_id.is_nil() {
            return None;
        }
        let flags = u16::from_le_bytes(buf[OFF_FLAGS..OFF_FLAGS + 2].try_into().ok()?);
        let payload_crc32c =
            u32::from_le_bytes(buf[OFF_PAYLOAD_CRC..OFF_PAYLOAD_CRC + 4].try_into().ok()?);
        Some(Self {
            sealed: flags & FLAG_SEALED != 0,
            chunk_id,
            version: u64::from_le_bytes(buf[OFF_VERSION_U64..OFF_VERSION_U64 + 8].try_into().ok()?),
            payload_len,
            payload_crc32c,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(sealed: bool) -> SlotHeader {
        SlotHeader {
            sealed,
            chunk_id: ChunkId::from([9; 16]),
            version: 42,
            payload_len: 1000,
            payload_crc32c: 0xDEAD_BEEF,
        }
    }

    #[test]
    fn roundtrip() {
        let h = sample(true);
        let buf = h.encode();
        assert_eq!(SlotHeader::decode_valid(&buf, 1000), Some(h));
    }

    #[test]
    fn unsealed_roundtrip() {
        let h = sample(false);
        let buf = h.encode();
        let got = SlotHeader::decode_valid(&buf, 65536).expect("valid");
        assert!(!got.sealed);
        assert_eq!(got, h);
    }

    #[test]
    fn bad_magic_rejected() {
        let mut buf = sample(true).encode();
        buf[0] ^= 0xFF;
        assert_eq!(SlotHeader::decode_valid(&buf, 65536), None);
    }

    #[test]
    fn bad_crc_rejected() {
        let mut buf = sample(true).encode();
        buf[30] ^= 0x01;
        assert_eq!(SlotHeader::decode_valid(&buf, 65536), None);
    }

    #[test]
    fn oversize_payload_len_rejected() {
        let h = sample(true);
        let buf = h.encode();
        assert_eq!(SlotHeader::decode_valid(&buf, 999), None);
    }

    #[test]
    fn nil_chunk_id_rejected() {
        let h = SlotHeader {
            chunk_id: ChunkId::from([0; 16]),
            ..sample(true)
        };
        let buf = h.encode();
        assert_eq!(SlotHeader::decode_valid(&buf, 65536), None);
    }
}
