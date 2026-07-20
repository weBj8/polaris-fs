//! Checkpoint region codec (contract §3.5): the clean-close index + bitmap
//! snapshot for bounded boot. Any validation failure falls back to the full
//! slot-header scan.

use crate::ChunkId;
use crate::index::{ChunkMeta, SlotClass};

/// Header page length (64 bytes used; page-aligned for O_DIRECT).
pub(crate) const HEADER_LEN: usize = 4096;
/// One index entry on disk.
pub(crate) const ENTRY_LEN: usize = 48;
const MAGIC: &[u8; 8] = b"GFCPNT01";

pub(crate) struct Header {
    pub(crate) generation: u64,
    pub(crate) entry_count: u64,
    pub(crate) bitmap_len: u32,
    pub(crate) payload_crc32c: u32,
}

pub(crate) fn encode_header(h: &Header) -> [u8; HEADER_LEN] {
    let mut buf = [0u8; HEADER_LEN];
    buf[0..8].copy_from_slice(MAGIC);
    buf[8..16].copy_from_slice(&h.generation.to_le_bytes());
    buf[16..24].copy_from_slice(&h.entry_count.to_le_bytes());
    buf[24..28].copy_from_slice(&h.bitmap_len.to_le_bytes());
    buf[28..32].copy_from_slice(&h.payload_crc32c.to_le_bytes());
    let crc = crc32fast::hash(&buf[..32]);
    buf[32..36].copy_from_slice(&crc.to_le_bytes());
    buf
}

pub(crate) fn decode_header(buf: &[u8]) -> Option<Header> {
    if buf.len() < 64 || &buf[0..8] != MAGIC {
        return None;
    }
    let want = u32::from_le_bytes(buf[32..36].try_into().ok()?);
    if crc32fast::hash(&buf[..32]) != want {
        return None;
    }
    Some(Header {
        generation: u64::from_le_bytes(buf[8..16].try_into().ok()?),
        entry_count: u64::from_le_bytes(buf[16..24].try_into().ok()?),
        bitmap_len: u32::from_le_bytes(buf[24..28].try_into().ok()?),
        payload_crc32c: u32::from_le_bytes(buf[28..32].try_into().ok()?),
    })
}

pub(crate) fn encode_entry(m: &ChunkMeta, out: &mut [u8]) {
    debug_assert_eq!(out.len(), ENTRY_LEN);
    out[0..16].copy_from_slice(m.chunk_id.as_bytes());
    out[16..24].copy_from_slice(&m.version.to_le_bytes());
    out[24..32].copy_from_slice(&m.payload_len.to_le_bytes());
    out[32..36].copy_from_slice(&m.payload_crc32c.to_le_bytes());
    out[36] = match m.class {
        SlotClass::S => 0,
        SlotClass::L => 1,
    };
    out[40..48].copy_from_slice(&m.slot_no.to_le_bytes());
}

pub(crate) fn decode_entry(buf: &[u8]) -> Option<ChunkMeta> {
    if buf.len() < ENTRY_LEN {
        return None;
    }
    let mut id = [0u8; 16];
    id.copy_from_slice(&buf[0..16]);
    let class = match buf[36] {
        0 => SlotClass::S,
        1 => SlotClass::L,
        _ => return None,
    };
    Some(ChunkMeta {
        chunk_id: ChunkId::from(id),
        version: u64::from_le_bytes(buf[16..24].try_into().ok()?),
        payload_len: u64::from_le_bytes(buf[24..32].try_into().ok()?),
        payload_crc32c: u32::from_le_bytes(buf[32..36].try_into().ok()?),
        class,
        slot_no: u64::from_le_bytes(buf[40..48].try_into().ok()?),
    })
}
