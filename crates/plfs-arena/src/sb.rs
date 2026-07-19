//! Superblock encode/decode (contract §2): two identical 4096-byte copies at
//! offsets 0 and 4096, crc32c over bytes [0, 104). Written once at mkfs;
//! readers try A then B and reject unknown `format_version`.

pub(crate) const SB_LEN: usize = 4096;
pub(crate) const SB_A_OFFSET: u64 = 0;
pub(crate) const SB_B_OFFSET: u64 = 4096;
pub(crate) const SB_MAGIC: &[u8; 8] = b"GFARENA1";
pub(crate) const FORMAT_VERSION: u32 = 1;
pub(crate) const FLAG_PUNCH_OK: u32 = 1 << 0;
pub(crate) const FLAG_DISCARD_OK: u32 = 1 << 1;
pub(crate) const FLAG_BLOCK_DEVICE: u32 = 1 << 2;

const OFF_FORMAT_VERSION: usize = 8;
const OFF_FLAGS: usize = 12;
const OFF_L_SLOT_SIZE: usize = 16;
const OFF_S_SLOT_SIZE: usize = 20;
const OFF_L_SLOT_COUNT: usize = 24;
const OFF_S_SLOT_COUNT: usize = 32;
const OFF_BITMAP_REGION_OFFSET: usize = 40;
const OFF_BITMAP_COPY_BYTES: usize = 48;
const OFF_SLOT_REGION_OFFSET: usize = 56;
const OFF_TOTAL_BYTES: usize = 64;
const OFF_ARENA_UUID: usize = 72;
const OFF_CREATED_AT: usize = 88;
const OFF_CRC: usize = 104;

/// Decoded superblock (field meanings per contract §2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Superblock {
    pub(crate) format_version: u32,
    pub(crate) flags: u32,
    pub(crate) l_slot_size: u32,
    pub(crate) s_slot_size: u32,
    pub(crate) l_slot_count: u64,
    pub(crate) s_slot_count: u64,
    pub(crate) bitmap_region_offset: u64,
    pub(crate) bitmap_copy_bytes: u64,
    pub(crate) slot_region_offset: u64,
    pub(crate) total_bytes: u64,
    pub(crate) arena_uuid: [u8; 16],
    pub(crate) created_at: u64,
}

/// Why a single superblock copy failed to decode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SbError {
    BadMagic,
    BadCrc,
}

impl Superblock {
    /// Serialize to the full 4096-byte block (zero padding, crc over [0, 104)).
    pub(crate) fn encode(&self) -> [u8; SB_LEN] {
        let mut buf = [0u8; SB_LEN];
        buf[0..8].copy_from_slice(SB_MAGIC);
        put_u32(&mut buf, OFF_FORMAT_VERSION, self.format_version);
        put_u32(&mut buf, OFF_FLAGS, self.flags);
        put_u32(&mut buf, OFF_L_SLOT_SIZE, self.l_slot_size);
        put_u32(&mut buf, OFF_S_SLOT_SIZE, self.s_slot_size);
        put_u64(&mut buf, OFF_L_SLOT_COUNT, self.l_slot_count);
        put_u64(&mut buf, OFF_S_SLOT_COUNT, self.s_slot_count);
        put_u64(
            &mut buf,
            OFF_BITMAP_REGION_OFFSET,
            self.bitmap_region_offset,
        );
        put_u64(&mut buf, OFF_BITMAP_COPY_BYTES, self.bitmap_copy_bytes);
        put_u64(&mut buf, OFF_SLOT_REGION_OFFSET, self.slot_region_offset);
        put_u64(&mut buf, OFF_TOTAL_BYTES, self.total_bytes);
        buf[OFF_ARENA_UUID..OFF_ARENA_UUID + 16].copy_from_slice(&self.arena_uuid);
        put_u64(&mut buf, OFF_CREATED_AT, self.created_at);
        let crc = crc32fast::hash(&buf[..OFF_CRC]);
        put_u32(&mut buf, OFF_CRC, crc);
        buf
    }

    /// Parse and validate magic + crc. Version is returned, not checked here
    /// (the caller maps an unwanted version to `UnsupportedVersion`).
    pub(crate) fn decode(buf: &[u8]) -> std::result::Result<Self, SbError> {
        if buf.len() < SB_LEN {
            return Err(SbError::BadMagic);
        }
        if &buf[0..8] != SB_MAGIC {
            return Err(SbError::BadMagic);
        }
        let want = get_u32(buf, OFF_CRC);
        let got = crc32fast::hash(&buf[..OFF_CRC]);
        if want != got {
            return Err(SbError::BadCrc);
        }
        let mut arena_uuid = [0u8; 16];
        arena_uuid.copy_from_slice(&buf[OFF_ARENA_UUID..OFF_ARENA_UUID + 16]);
        Ok(Self {
            format_version: get_u32(buf, OFF_FORMAT_VERSION),
            flags: get_u32(buf, OFF_FLAGS),
            l_slot_size: get_u32(buf, OFF_L_SLOT_SIZE),
            s_slot_size: get_u32(buf, OFF_S_SLOT_SIZE),
            l_slot_count: get_u64(buf, OFF_L_SLOT_COUNT),
            s_slot_count: get_u64(buf, OFF_S_SLOT_COUNT),
            bitmap_region_offset: get_u64(buf, OFF_BITMAP_REGION_OFFSET),
            bitmap_copy_bytes: get_u64(buf, OFF_BITMAP_COPY_BYTES),
            slot_region_offset: get_u64(buf, OFF_SLOT_REGION_OFFSET),
            total_bytes: get_u64(buf, OFF_TOTAL_BYTES),
            arena_uuid,
            created_at: get_u64(buf, OFF_CREATED_AT),
        })
    }
}

fn put_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

fn put_u64(buf: &mut [u8], off: usize, v: u64) {
    buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

fn get_u32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(buf[off..off + 4].try_into().expect("slice len"))
}

fn get_u64(buf: &[u8], off: usize) -> u64 {
    u64::from_le_bytes(buf[off..off + 8].try_into().expect("slice len"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Superblock {
        Superblock {
            format_version: FORMAT_VERSION,
            flags: FLAG_PUNCH_OK,
            l_slot_size: 1 << 20,
            s_slot_size: 1 << 16,
            l_slot_count: 100,
            s_slot_count: 50,
            bitmap_region_offset: 8192,
            bitmap_copy_bytes: 4096,
            slot_region_offset: 1 << 20,
            total_bytes: 1 << 30,
            arena_uuid: [7; 16],
            created_at: 1_700_000_000,
        }
    }

    #[test]
    fn roundtrip() {
        let sb = sample();
        let buf = sb.encode();
        assert_eq!(Superblock::decode(&buf), Ok(sb));
    }

    #[test]
    fn bad_magic_rejected() {
        let mut buf = sample().encode();
        buf[0] ^= 0xFF;
        assert_eq!(Superblock::decode(&buf), Err(SbError::BadMagic));
    }

    #[test]
    fn bad_crc_rejected() {
        let mut buf = sample().encode();
        buf[40] ^= 0x01;
        assert_eq!(Superblock::decode(&buf), Err(SbError::BadCrc));
    }
}
