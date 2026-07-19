//! In-memory chunk index: `chunk_id -> ChunkMeta` for live (SEALED) slots.
//! Rebuilt from the slot-header scan at every open; the only structure
//! `stat`/`list` ever consult.

use std::collections::HashMap;

use crate::ChunkId;

/// Slot class. Declaration order defines the duplicate-resolution rank
/// (contract §5: the higher `(class, slot_no)` wins): `L > S`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SlotClass {
    /// Small slot (inline chunks).
    S,
    /// Large slot (normal chunks).
    L,
}

/// Metadata of one live chunk (index entry; mirrors the on-disk header).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkMeta {
    /// Chunk identifier.
    pub chunk_id: ChunkId,
    /// Caller metadata; immutable per chunk_id.
    pub version: u64,
    /// Payload length in bytes.
    pub payload_len: u64,
    /// crc32c of the payload bytes.
    pub payload_crc32c: u32,
    /// Owning slot class.
    pub class: SlotClass,
    /// Slot number within the class.
    pub slot_no: u64,
}

pub(crate) struct Index {
    map: HashMap<ChunkId, ChunkMeta>,
}

impl Index {
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, id: &ChunkId) -> Option<&ChunkMeta> {
        self.map.get(id)
    }

    pub(crate) fn insert(&mut self, meta: ChunkMeta) -> Option<ChunkMeta> {
        self.map.insert(meta.chunk_id, meta)
    }

    pub(crate) fn remove(&mut self, id: &ChunkId) -> Option<ChunkMeta> {
        self.map.remove(id)
    }

    pub(crate) fn len(&self) -> usize {
        self.map.len()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &ChunkMeta> {
        self.map.values()
    }
}
