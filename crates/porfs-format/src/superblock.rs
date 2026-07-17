//! Superblock: exactly one 4KiB block, stored in two copies (A at offset 0,
//! B at offset 4KiB). See `docs/format.md` §2 for the field table.

use bytemuck::{Pod, Zeroable};

use crate::{FORMAT_VERSION, FormatError, SB_MAGIC, checksum};

/// Offset of the CRC field; the checksum covers `[0..SB_CRC_OFFSET)`.
const SB_CRC_OFFSET: usize = 4092;

/// The superblock, exactly one 4KiB block.
///
/// Serialized form is the struct itself (little-endian fields, zero padding).
/// `sb_crc32c` covers bytes `[0..4092)` of the serialized image.
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Superblock {
    /// `SB_MAGIC` at offset 0.
    pub magic: [u8; 8],
    /// `FORMAT_VERSION` (= 2).
    pub format_version: u32,
    /// Flags; bit0 = clean unmount (`SB_FLAG_CLEAN`).
    pub flags: u32,
    /// Total device size in bytes.
    pub device_size: u64,
    /// Start of the extent log region (= `DATA_START` = 64MiB).
    pub data_start: u64,
    /// Next append offset; only confirmed (synced) extents are covered.
    pub tail: u64,
    /// Number of live extents (tombstones excluded).
    pub extent_count: u64,
    /// Monotonic sync counter; on open the valid copy with the highest value wins.
    pub sync_seq: u64,
    /// Filesystem UUID.
    pub uuid: [u8; 16],
    /// Creation time, unix seconds.
    pub created_at: u64,
    /// Last sync time, unix seconds.
    pub last_sync_at: u64,
    /// Reserved, must be zero.
    pub reserved: [u8; 4004],
    /// CRC32C over bytes `[0..4092)` of the serialized image.
    pub sb_crc32c: u32,
}

const _: () = assert!(size_of::<Superblock>() == crate::BLOCK_SIZE as usize);

impl Superblock {
    /// Serialized length in bytes (one block).
    pub const LEN: usize = crate::BLOCK_SIZE as usize;

    /// Recompute and store the CRC32C over bytes `[0..4092)`.
    pub fn seal(&mut self) {
        let crc = checksum(&bytemuck::bytes_of(self)[..SB_CRC_OFFSET]);
        self.sb_crc32c = crc;
    }

    /// Validate magic, format version, and CRC32C.
    pub fn validate(&self) -> Result<(), FormatError> {
        if self.magic != *SB_MAGIC {
            return Err(FormatError::BadSuperblockMagic);
        }
        if self.format_version != FORMAT_VERSION {
            return Err(FormatError::UnsupportedVersion(self.format_version));
        }
        if checksum(&bytemuck::bytes_of(self)[..SB_CRC_OFFSET]) != self.sb_crc32c {
            return Err(FormatError::BadSuperblockCrc);
        }
        Ok(())
    }

    /// Copy a superblock out of a raw 4096-byte image (alignment-agnostic).
    /// Does not validate; call [`Superblock::validate`] afterwards.
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
    use crate::DATA_START;
    use std::mem::offset_of;

    fn sample() -> Superblock {
        let mut sb = Superblock {
            magic: *SB_MAGIC,
            format_version: FORMAT_VERSION,
            flags: crate::SB_FLAG_CLEAN,
            device_size: 1 << 30,
            data_start: DATA_START,
            tail: DATA_START,
            extent_count: 0,
            sync_seq: 1,
            uuid: [0xAB; 16],
            created_at: 1_700_000_000,
            last_sync_at: 1_700_000_000,
            reserved: [0; 4004],
            sb_crc32c: 0,
        };
        sb.seal();
        sb
    }

    #[test]
    fn serialized_layout_matches_spec() {
        assert_eq!(Superblock::LEN, 4096);
        assert_eq!(offset_of!(Superblock, magic), 0);
        assert_eq!(offset_of!(Superblock, format_version), 8);
        assert_eq!(offset_of!(Superblock, flags), 12);
        assert_eq!(offset_of!(Superblock, device_size), 16);
        assert_eq!(offset_of!(Superblock, data_start), 24);
        assert_eq!(offset_of!(Superblock, tail), 32);
        assert_eq!(offset_of!(Superblock, extent_count), 40);
        assert_eq!(offset_of!(Superblock, sync_seq), 48);
        assert_eq!(offset_of!(Superblock, uuid), 56);
        assert_eq!(offset_of!(Superblock, created_at), 72);
        assert_eq!(offset_of!(Superblock, last_sync_at), 80);
        assert_eq!(offset_of!(Superblock, reserved), 88);
        assert_eq!(offset_of!(Superblock, sb_crc32c), 4092);
    }

    #[test]
    fn seal_validate_roundtrip() {
        let sb = sample();
        assert!(sb.validate().is_ok());
        let decoded = Superblock::from_bytes(sb.as_bytes()).unwrap();
        assert!(decoded.validate().is_ok());
        assert_eq!(decoded.sync_seq, 1);
        assert_eq!(decoded.uuid, [0xAB; 16]);
    }

    #[test]
    fn validate_rejects_bit_flip() {
        let sb = sample();
        let mut image = [0u8; Superblock::LEN];
        image.copy_from_slice(sb.as_bytes());
        // Flip one payload bit; CRC must catch it.
        image[100] ^= 0x01;
        let decoded = Superblock::from_bytes(&image).unwrap();
        assert!(matches!(
            decoded.validate(),
            Err(FormatError::BadSuperblockCrc)
        ));
    }

    #[test]
    fn validate_rejects_bad_magic_and_version() {
        let mut sb = sample();
        sb.magic = *b"XXXXXXX!";
        sb.seal();
        assert!(matches!(
            sb.validate(),
            Err(FormatError::BadSuperblockMagic)
        ));
        let mut sb = sample();
        sb.format_version = 99;
        sb.seal();
        assert!(matches!(
            sb.validate(),
            Err(FormatError::UnsupportedVersion(99))
        ));
    }

    #[test]
    fn from_bytes_rejects_wrong_length() {
        assert!(matches!(
            Superblock::from_bytes(&[0u8; 100]),
            Err(FormatError::Truncated { .. })
        ));
    }
}
