//! Checkpoint slots: a consistent snapshot of the in-memory index at a log
//! position, so mount can skip the full log scan. Two slots ping-pong at
//! `CKPT_SLOT_A_OFF` / `CKPT_SLOT_B_OFF`; each slot is a 64-byte header plus
//! an entry array (40 bytes per entry, zero-padded to a 4KiB multiple).
//! See `docs/format.md` §4 for the field tables.

use bytemuck::{Pod, Zeroable};

use crate::{CKPT_MAGIC, FormatError, checksum};

/// Offset of the header CRC field; the checksum covers `header[0..60)`.
const HEADER_CRC_OFFSET: usize = 60;

/// The 64-byte checkpoint header (little-endian fields, zero padding).
///
/// `header_crc32c` covers bytes `[0..60)`; `payload_crc32c` covers the
/// `entry_count * 40` payload bytes (padding excluded).
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct CheckpointHeader {
    /// `CKPT_MAGIC` (`0x3150_4B43`).
    pub magic: u32,
    /// Header length in bytes (= 64).
    pub header_len: u16,
    /// Flags; currently always 0.
    pub flags: u16,
    /// Generation: monotonically increasing across both slots; highest wins.
    pub slot_gen: u64,
    /// Log offset this checkpoint's index covers; mount resumes scanning here.
    pub covered_tail: u64,
    /// Id the next append will receive, as of `covered_tail`.
    pub next_extent_id: u64,
    /// Number of live extents as of `covered_tail`.
    pub extent_count: u64,
    /// Sum of `data_len` over live extents as of `covered_tail`.
    pub live_bytes: u64,
    /// Number of 40-byte entries in the payload.
    pub entry_count: u32,
    /// CRC32C over the `entry_count * 40` payload bytes (padding excluded).
    pub payload_crc32c: u32,
    /// Reserved, must be zero.
    pub reserved: [u8; 4],
    /// CRC32C over `header[0..60)`.
    pub header_crc32c: u32,
}

const _: () = assert!(size_of::<CheckpointHeader>() == 64);

/// One live index entry in a checkpoint payload: where an extent lives and
/// what it belongs to. Tombstones are dropped at checkpoint time; only live
/// entries are stored.
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct CheckpointEntry {
    /// Extent id.
    pub extent_id: u64,
    /// Byte offset of the record in the log region.
    pub offset: u64,
    /// Total on-disk bytes of the record (a 4KiB multiple).
    pub disk_len: u32,
    /// Payload bytes (padding excluded).
    pub data_len: u32,
    /// Owning inode.
    pub inode: u64,
    /// Logical byte offset inside the file.
    pub logical_offset: u64,
}

const _: () = assert!(size_of::<CheckpointEntry>() == 40);

impl CheckpointHeader {
    /// Serialized header length in bytes.
    pub const LEN: usize = 64;

    /// Recompute and store the CRC32C over `header[0..60)`.
    pub fn seal(&mut self) {
        let crc = checksum(&bytemuck::bytes_of(self)[..HEADER_CRC_OFFSET]);
        self.header_crc32c = crc;
    }

    /// Validate magic, header length, and header CRC32C. The payload CRC and
    /// the slot-capacity bound are checked by the reader, which knows
    /// `entry_count` and the payload bytes.
    pub fn validate(&self) -> Result<(), FormatError> {
        if self.magic != CKPT_MAGIC {
            return Err(FormatError::BadCheckpointMagic);
        }
        if self.header_len != Self::LEN as u16 {
            return Err(FormatError::BadCheckpointHeaderLen(self.header_len));
        }
        if checksum(&bytemuck::bytes_of(self)[..HEADER_CRC_OFFSET]) != self.header_crc32c {
            return Err(FormatError::BadCheckpointHeaderCrc);
        }
        Ok(())
    }

    /// Copy a header out of a raw 64-byte image (alignment-agnostic).
    /// Does not validate; call [`CheckpointHeader::validate`] afterwards.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FormatError> {
        if bytes.len() != Self::LEN {
            return Err(FormatError::Truncated {
                need: Self::LEN,
                got: bytes.len(),
            });
        }
        Ok(bytemuck::pod_read_unaligned(bytes))
    }

    /// View the struct as its serialized bytes.
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

impl CheckpointEntry {
    /// Serialized entry length in bytes.
    pub const LEN: usize = 40;

    /// Copy an entry out of a raw 40-byte image (alignment-agnostic).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FormatError> {
        if bytes.len() != Self::LEN {
            return Err(FormatError::Truncated {
                need: Self::LEN,
                got: bytes.len(),
            });
        }
        Ok(bytemuck::pod_read_unaligned(bytes))
    }

    /// View the struct as its serialized bytes.
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::offset_of;

    fn sample() -> CheckpointHeader {
        let mut h = CheckpointHeader {
            magic: CKPT_MAGIC,
            header_len: CheckpointHeader::LEN as u16,
            flags: 0,
            slot_gen: 7,
            covered_tail: crate::DATA_START + 3 * crate::BLOCK_SIZE,
            next_extent_id: 9,
            extent_count: 8,
            live_bytes: 123_456,
            entry_count: 8,
            payload_crc32c: 0xDEAD_BEEF,
            reserved: [0; 4],
            header_crc32c: 0,
        };
        h.seal();
        h
    }

    #[test]
    fn header_layout_matches_spec() {
        assert_eq!(CheckpointHeader::LEN, 64);
        assert_eq!(offset_of!(CheckpointHeader, magic), 0);
        assert_eq!(offset_of!(CheckpointHeader, header_len), 4);
        assert_eq!(offset_of!(CheckpointHeader, flags), 6);
        assert_eq!(offset_of!(CheckpointHeader, slot_gen), 8);
        assert_eq!(offset_of!(CheckpointHeader, covered_tail), 16);
        assert_eq!(offset_of!(CheckpointHeader, next_extent_id), 24);
        assert_eq!(offset_of!(CheckpointHeader, extent_count), 32);
        assert_eq!(offset_of!(CheckpointHeader, live_bytes), 40);
        assert_eq!(offset_of!(CheckpointHeader, entry_count), 48);
        assert_eq!(offset_of!(CheckpointHeader, payload_crc32c), 52);
        assert_eq!(offset_of!(CheckpointHeader, reserved), 56);
        assert_eq!(offset_of!(CheckpointHeader, header_crc32c), 60);
    }

    #[test]
    fn entry_layout_matches_spec() {
        assert_eq!(CheckpointEntry::LEN, 40);
        assert_eq!(offset_of!(CheckpointEntry, extent_id), 0);
        assert_eq!(offset_of!(CheckpointEntry, offset), 8);
        assert_eq!(offset_of!(CheckpointEntry, disk_len), 16);
        assert_eq!(offset_of!(CheckpointEntry, data_len), 20);
        assert_eq!(offset_of!(CheckpointEntry, inode), 24);
        assert_eq!(offset_of!(CheckpointEntry, logical_offset), 32);
    }

    #[test]
    fn seal_validate_roundtrip() {
        let h = sample();
        assert!(h.validate().is_ok());
        let decoded = CheckpointHeader::from_bytes(h.as_bytes()).unwrap();
        assert!(decoded.validate().is_ok());
        assert_eq!(decoded.slot_gen, 7);
        assert_eq!(decoded.entry_count, 8);
    }

    #[test]
    fn validate_rejects_corruption() {
        let h = sample();
        let mut image = [0u8; CheckpointHeader::LEN];
        image.copy_from_slice(h.as_bytes());
        image[20] ^= 0x01; // inside covered_tail
        let decoded = CheckpointHeader::from_bytes(&image).unwrap();
        assert!(matches!(
            decoded.validate(),
            Err(FormatError::BadCheckpointHeaderCrc)
        ));

        let mut bad = h;
        bad.magic = 0;
        bad.seal();
        assert!(matches!(
            bad.validate(),
            Err(FormatError::BadCheckpointMagic)
        ));

        let mut bad = h;
        bad.header_len = 32;
        bad.seal();
        assert!(matches!(
            bad.validate(),
            Err(FormatError::BadCheckpointHeaderLen(32))
        ));
    }
}
