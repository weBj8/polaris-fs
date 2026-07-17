//! Extent record header: 64 bytes, followed by `data_len` payload bytes and
//! zero padding to a 4KiB multiple. See `docs/format.md` §3 for the field table.

use bytemuck::{Pod, Zeroable};

use crate::{EXT_FLAG_TOMBSTONE, EXT_MAGIC, FormatError, checksum};

/// Offset of the CRC field; the checksum covers `header[0..60)`.
const HEADER_CRC_OFFSET: usize = 60;

/// The 64-byte extent record header (little-endian fields, zero padding).
///
/// `header_crc32c` covers bytes `[0..60)` of the serialized header.
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct ExtentHeader {
    /// `EXT_MAGIC` (`0x4558_5431`).
    pub magic: u32,
    /// Header length in bytes (= 64).
    pub header_len: u16,
    /// Flags; bit0 = tombstone (`EXT_FLAG_TOMBSTONE`).
    pub flags: u16,
    /// Extent id: monotonically increasing, unique per device.
    pub extent_id: u64,
    /// Owning inode (pass-through record in P1).
    pub inode: u64,
    /// Logical byte offset inside the file.
    pub logical_offset: u64,
    /// Payload bytes (padding excluded); 0 for tombstones.
    pub data_len: u32,
    /// CRC32C over `data[0..data_len)`.
    pub data_crc32c: u32,
    /// Total on-disk bytes: header + data + pad (a 4KiB multiple).
    pub disk_len: u32,
    /// Reserved, must be zero.
    pub reserved: [u8; 16],
    /// CRC32C over `header[0..60)`.
    pub header_crc32c: u32,
}

const _: () = assert!(size_of::<ExtentHeader>() == 64);

impl ExtentHeader {
    /// Serialized header length in bytes.
    pub const LEN: usize = 64;

    /// Recompute and store the CRC32C over `header[0..60)`.
    pub fn seal(&mut self) {
        let crc = checksum(&bytemuck::bytes_of(self)[..HEADER_CRC_OFFSET]);
        self.header_crc32c = crc;
    }

    /// Validate magic, header length, and CRC32C.
    pub fn validate(&self) -> Result<(), FormatError> {
        if self.magic != EXT_MAGIC {
            return Err(FormatError::BadExtentMagic);
        }
        if self.header_len != Self::LEN as u16 {
            return Err(FormatError::BadHeaderLen(self.header_len));
        }
        if checksum(&bytemuck::bytes_of(self)[..HEADER_CRC_OFFSET]) != self.header_crc32c {
            return Err(FormatError::BadHeaderCrc);
        }
        Ok(())
    }

    /// True if this record is a tombstone (logical delete of `extent_id`).
    pub fn is_tombstone(&self) -> bool {
        self.flags & EXT_FLAG_TOMBSTONE != 0
    }

    /// Copy a header out of a raw 64-byte image (alignment-agnostic).
    /// Does not validate; call [`ExtentHeader::validate`] afterwards.
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
    use crate::{BLOCK_SIZE, extent_disk_len};
    use std::mem::offset_of;

    fn sample(data: &[u8], disk_len: u32) -> ExtentHeader {
        let mut h = ExtentHeader {
            magic: EXT_MAGIC,
            header_len: ExtentHeader::LEN as u16,
            flags: 0,
            extent_id: 7,
            inode: 42,
            logical_offset: 4096,
            data_len: data.len() as u32,
            data_crc32c: checksum(data),
            disk_len,
            reserved: [0; 16],
            header_crc32c: 0,
        };
        h.seal();
        h
    }

    #[test]
    fn serialized_layout_matches_spec() {
        assert_eq!(ExtentHeader::LEN, 64);
        assert_eq!(offset_of!(ExtentHeader, magic), 0);
        assert_eq!(offset_of!(ExtentHeader, header_len), 4);
        assert_eq!(offset_of!(ExtentHeader, flags), 6);
        assert_eq!(offset_of!(ExtentHeader, extent_id), 8);
        assert_eq!(offset_of!(ExtentHeader, inode), 16);
        assert_eq!(offset_of!(ExtentHeader, logical_offset), 24);
        assert_eq!(offset_of!(ExtentHeader, data_len), 32);
        assert_eq!(offset_of!(ExtentHeader, data_crc32c), 36);
        assert_eq!(offset_of!(ExtentHeader, disk_len), 40);
        assert_eq!(offset_of!(ExtentHeader, reserved), 44);
        assert_eq!(offset_of!(ExtentHeader, header_crc32c), 60);
    }

    #[test]
    fn seal_validate_roundtrip() {
        let h = sample(b"hello", 4096);
        assert!(h.validate().is_ok());
        let decoded = ExtentHeader::from_bytes(h.as_bytes()).unwrap();
        assert!(decoded.validate().is_ok());
        assert_eq!(decoded.extent_id, 7);
        assert!(!decoded.is_tombstone());
    }

    #[test]
    fn validate_rejects_corruption() {
        let h = sample(b"data", 4096);
        let mut image = [0u8; ExtentHeader::LEN];
        image.copy_from_slice(h.as_bytes());
        image[10] ^= 0x01; // inside extent_id
        let decoded = ExtentHeader::from_bytes(&image).unwrap();
        assert!(matches!(decoded.validate(), Err(FormatError::BadHeaderCrc)));

        let mut bad = h;
        bad.magic = 0;
        bad.seal();
        assert!(matches!(bad.validate(), Err(FormatError::BadExtentMagic)));

        let mut bad = h;
        bad.header_len = 32;
        bad.seal();
        assert!(matches!(bad.validate(), Err(FormatError::BadHeaderLen(32))));
    }

    #[test]
    fn tombstone_flag() {
        let mut h = sample(b"", BLOCK_SIZE as u32);
        assert!(!h.is_tombstone());
        h.flags |= EXT_FLAG_TOMBSTONE;
        h.seal();
        assert!(h.is_tombstone());
        assert!(h.validate().is_ok());
    }

    #[test]
    fn disk_len_rounds_up_to_block() {
        assert_eq!(extent_disk_len(0), BLOCK_SIZE);
        assert_eq!(extent_disk_len(1), BLOCK_SIZE);
        assert_eq!(extent_disk_len(BLOCK_SIZE - 64), BLOCK_SIZE);
        assert_eq!(extent_disk_len(BLOCK_SIZE - 63), 2 * BLOCK_SIZE);
        assert_eq!(
            extent_disk_len(4 * 1024 * 1024),
            4 * 1024 * 1024 + BLOCK_SIZE
        );
    }
}
