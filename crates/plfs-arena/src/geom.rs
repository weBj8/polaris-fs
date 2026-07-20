//! Arena geometry (contract §1, §6): derives the region layout from the mkfs
//! parameters by fixpoint iteration — counts -> bitmap size ->
//! `slot_region_offset` -> counts, at most 3 iterations with counts only
//! shrinking. Readers never recompute; they validate and use the superblock
//! values.

use crate::bitmap::Bitmaps;
use crate::index::SlotClass;
use crate::sb::{FORMAT_VERSION, Superblock};
use crate::{ArenaError, Result};

/// Offset of the bitmap region (right after both superblocks).
pub(crate) const BITMAP_REGION_OFFSET: u64 = 8192;
/// `slot_region_offset` alignment (contract §2).
const SLOT_REGION_ALIGN: u64 = 1 << 20;
/// Slot sizes must be page multiples so every slot start is O_DIRECT-safe.
const SLOT_SIZE_ALIGN: u64 = 4096;

/// mkfs parameters. Slot sizes must be multiples of 4096.
#[derive(Debug, Clone)]
pub struct MkfsConfig {
    /// Image size in bytes (ignored for block devices — the device is used
    /// as-is and its size becomes `total_bytes`).
    pub total_bytes: u64,
    /// Fraction of slot-region bytes assigned to L-slots (default 0.90).
    pub l_fraction: f64,
    /// L-slot size in bytes (default 1 MiB).
    pub l_slot_size: u32,
    /// S-slot size in bytes (default 64 KiB).
    pub s_slot_size: u32,
}

impl MkfsConfig {
    /// Config with the contract defaults for a given image size.
    pub fn new(total_bytes: u64) -> Self {
        Self {
            total_bytes,
            l_fraction: 0.90,
            l_slot_size: 1 << 20,
            s_slot_size: 1 << 16,
        }
    }
}

/// What mkfs decided (returned to the caller; everything is also persisted in
/// the superblock).
#[derive(Debug, Clone)]
pub struct MkfsReport {
    /// Final geometry as written to the superblock.
    pub geometry: Geometry,
    /// Arena UUID (UUIDv4 at mkfs).
    pub arena_uuid: [u8; 16],
    /// mkfs time, unix seconds.
    pub created_at: u64,
    /// Backend supports hole punching (`PUNCH_OK`).
    pub punch_ok: bool,
    /// Backend supports BLKDISCARD (`DISCARD_OK`).
    pub discard_ok: bool,
    /// Backend is a block device (`BLOCK_DEVICE`).
    pub block_device: bool,
}

/// Region layout; all offsets are superblock values (contract §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Geometry {
    /// L-slot size in bytes.
    pub l_slot_size: u32,
    /// S-slot size in bytes.
    pub s_slot_size: u32,
    /// Number of L-slots.
    pub l_slot_count: u64,
    /// Number of S-slots.
    pub s_slot_count: u64,
    /// Offset of bitmap copy A (copy B follows after `bitmap_copy_bytes`).
    pub bitmap_region_offset: u64,
    /// Bytes per bitmap copy, incl. crc and zero padding (multiple of 4096).
    pub bitmap_copy_bytes: u64,
    /// Offset of L-slot 0 (1 MiB aligned).
    pub slot_region_offset: u64,
    /// Image/device size at mkfs.
    pub total_bytes: u64,
}

impl Geometry {
    /// Fixpoint per contract §6: start from a one-page bitmap estimate and
    /// iterate counts -> bitmap size -> slot_region_offset -> counts; the
    /// bitmap estimate only grows, so counts only shrink.
    pub(crate) fn compute(cfg: &MkfsConfig) -> Result<Self> {
        if !(0.0..=1.0).contains(&cfg.l_fraction) {
            return Err(invalid_input("l_fraction must be within [0.0, 1.0]"));
        }
        let l_size = u64::from(cfg.l_slot_size);
        let s_size = u64::from(cfg.s_slot_size);
        if !l_size.is_multiple_of(SLOT_SIZE_ALIGN) || l_size < SLOT_SIZE_ALIGN {
            return Err(invalid_input("l_slot_size must be a multiple of 4096"));
        }
        if !s_size.is_multiple_of(SLOT_SIZE_ALIGN) || s_size < SLOT_SIZE_ALIGN {
            return Err(invalid_input("s_slot_size must be a multiple of 4096"));
        }
        if s_size > l_size {
            return Err(invalid_input("s_slot_size must not exceed l_slot_size"));
        }
        let mut copy_bytes: u64 = 4096;
        for _ in 0..3 {
            let (_, l_count, s_count) = counts_for(cfg, copy_bytes)?;
            let need = bitmap_copy_len(l_count, s_count)?;
            if need <= copy_bytes {
                break;
            }
            copy_bytes = need;
        }
        // Recompute once with the final bitmap size; counts can only shrink,
        // and the bitmap sized above still fits the shrunk counts.
        let (slot_region_offset, l_count, s_count) = counts_for(cfg, copy_bytes)?;
        if l_count == 0 && s_count == 0 {
            return Err(invalid_input("total_bytes too small for any slot"));
        }
        Ok(Self {
            l_slot_size: cfg.l_slot_size,
            s_slot_size: cfg.s_slot_size,
            l_slot_count: l_count,
            s_slot_count: s_count,
            bitmap_region_offset: BITMAP_REGION_OFFSET,
            bitmap_copy_bytes: copy_bytes,
            slot_region_offset,
            total_bytes: cfg.total_bytes,
        })
    }

    /// Validate superblock geometry fields and adopt them (readers use the
    /// superblock values, never recomputed constants).
    pub(crate) fn from_sb(sb: &Superblock) -> Result<Self> {
        let bad = || ArenaError::BadSuperblock;
        let l_size = u64::from(sb.l_slot_size);
        let s_size = u64::from(sb.s_slot_size);
        if l_size < 576
            || s_size < 576
            || !l_size.is_multiple_of(512)
            || !s_size.is_multiple_of(512)
            || s_size > l_size
        {
            return Err(bad());
        }
        if sb.bitmap_copy_bytes == 0
            || !sb.bitmap_copy_bytes.is_multiple_of(4096)
            || sb.bitmap_region_offset < SB_END
        {
            return Err(bad());
        }
        let bitmap_end = sb
            .bitmap_region_offset
            .checked_add(2u64.checked_mul(sb.bitmap_copy_bytes).ok_or_else(bad)?)
            .ok_or_else(bad)?;
        if sb.slot_region_offset < bitmap_end {
            return Err(bad());
        }
        let l_span = l_size.checked_mul(sb.l_slot_count).ok_or_else(bad)?;
        let s_span = s_size.checked_mul(sb.s_slot_count).ok_or_else(bad)?;
        let slots_end = sb
            .slot_region_offset
            .checked_add(l_span)
            .and_then(|v| v.checked_add(s_span))
            .ok_or_else(bad)?;
        if slots_end > sb.total_bytes {
            return Err(bad());
        }
        let payload = Bitmaps::payload_len(sb.l_slot_count, sb.s_slot_count) as u64;
        if payload + 4 > sb.bitmap_copy_bytes {
            return Err(bad());
        }
        Ok(Self {
            l_slot_size: sb.l_slot_size,
            s_slot_size: sb.s_slot_size,
            l_slot_count: sb.l_slot_count,
            s_slot_count: sb.s_slot_count,
            bitmap_region_offset: sb.bitmap_region_offset,
            bitmap_copy_bytes: sb.bitmap_copy_bytes,
            slot_region_offset: sb.slot_region_offset,
            total_bytes: sb.total_bytes,
        })
    }

    /// Payload capacity of a slot class (contract §4: slot_size - 64).
    pub fn capacity(&self, class: SlotClass) -> u64 {
        u64::from(self.slot_size(class)) - 64
    }

    /// Slot size in bytes for a class.
    pub fn slot_size(&self, class: SlotClass) -> u32 {
        match class {
            SlotClass::L => self.l_slot_size,
            SlotClass::S => self.s_slot_size,
        }
    }

    /// Slot count for a class.
    pub fn slot_count(&self, class: SlotClass) -> u64 {
        match class {
            SlotClass::L => self.l_slot_count,
            SlotClass::S => self.s_slot_count,
        }
    }

    /// Absolute offset of slot `idx` of `class` (contract §1 table).
    pub(crate) fn slot_offset(&self, class: SlotClass, idx: u64) -> u64 {
        match class {
            SlotClass::L => self.slot_region_offset + idx * u64::from(self.l_slot_size),
            SlotClass::S => {
                self.slot_region_offset
                    + self.l_slot_count * u64::from(self.l_slot_size)
                    + idx * u64::from(self.s_slot_size)
            }
        }
    }

    /// Start of a class region (for sequential scans).
    pub(crate) fn class_region_offset(&self, class: SlotClass) -> u64 {
        self.slot_offset(class, 0)
    }

    /// Checkpoint region offset (contract §3.5: right after bitmap copy B).
    pub(crate) fn checkpoint_offset(&self) -> u64 {
        self.bitmap_region_offset + 2 * self.bitmap_copy_bytes
    }

    /// Checkpoint region size (contract §3.5).
    pub(crate) fn checkpoint_bytes(&self) -> u64 {
        checkpoint_len(self.total_bytes)
    }
}

const SB_END: u64 = 8192;

/// Slot counts for a given bitmap-copy size (one fixpoint step).
fn counts_for(cfg: &MkfsConfig, copy_bytes: u64) -> Result<(u64, u64, u64)> {
    let l_size = u64::from(cfg.l_slot_size);
    let s_size = u64::from(cfg.s_slot_size);
    let slot_region_offset = align_up(
        BITMAP_REGION_OFFSET + 2 * copy_bytes + checkpoint_len(cfg.total_bytes),
        SLOT_REGION_ALIGN,
    );
    if slot_region_offset >= cfg.total_bytes {
        return Err(invalid_input("total_bytes too small for any slot"));
    }
    let region = cfg.total_bytes - slot_region_offset;
    let l_bytes = (region as f64 * cfg.l_fraction) as u64;
    let l_count = l_bytes / l_size;
    let s_count = (region - l_count * l_size) / s_size;
    Ok((slot_region_offset, l_count, s_count))
}

/// Checkpoint region size (contract §3.5): max(1 MiB, total/1024).
pub(crate) fn checkpoint_len(total_bytes: u64) -> u64 {
    align_up((total_bytes / 1024).max(1 << 20), 4096)
}

pub(crate) fn align_up(v: u64, align: u64) -> u64 {
    v.div_ceil(align) * align
}

fn bitmap_copy_len(l_count: u64, s_count: u64) -> Result<u64> {
    let payload = Bitmaps::payload_len(l_count, s_count) as u64;
    Ok(align_up(payload + 4, 4096))
}

fn invalid_input(msg: &'static str) -> ArenaError {
    ArenaError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, msg))
}

/// Build the superblock to persist for a fresh arena.
pub(crate) fn superblock_for(
    geom: &Geometry,
    flags: u32,
    arena_uuid: [u8; 16],
    created_at: u64,
) -> Superblock {
    Superblock {
        format_version: FORMAT_VERSION,
        flags,
        l_slot_size: geom.l_slot_size,
        s_slot_size: geom.s_slot_size,
        l_slot_count: geom.l_slot_count,
        s_slot_count: geom.s_slot_count,
        bitmap_region_offset: geom.bitmap_region_offset,
        bitmap_copy_bytes: geom.bitmap_copy_bytes,
        slot_region_offset: geom.slot_region_offset,
        total_bytes: geom.total_bytes,
        arena_uuid,
        created_at,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_geometry_fits_and_honors_fraction() {
        let cfg = MkfsConfig::new(1 << 30);
        let g = Geometry::compute(&cfg).expect("geometry");
        assert_eq!(g.bitmap_region_offset, 8192);
        assert_eq!(g.slot_region_offset % SLOT_REGION_ALIGN, 0);
        assert!(g.bitmap_copy_bytes.is_multiple_of(4096));
        assert!(
            g.bitmap_copy_bytes >= Bitmaps::payload_len(g.l_slot_count, g.s_slot_count) as u64 + 4
        );
        let slots_end = g.slot_offset(SlotClass::S, g.s_slot_count);
        assert!(slots_end <= g.total_bytes);
        let region = g.total_bytes - g.slot_region_offset;
        let l_bytes = g.l_slot_count * u64::from(g.l_slot_size);
        let frac = l_bytes as f64 / region as f64;
        let slot_slack = u64::from(g.l_slot_size) as f64 / region as f64;
        assert!(
            (frac - 0.90).abs() <= slot_slack,
            "l_fraction {frac} off by more than one slot ({slot_slack})"
        );
        assert!(g.l_slot_count > 0 && g.s_slot_count > 0);
    }

    #[test]
    fn small_custom_geometry() {
        let cfg = MkfsConfig {
            total_bytes: 16 << 20,
            l_fraction: 0.90,
            l_slot_size: 256 << 10,
            s_slot_size: 16 << 10,
        };
        let g = Geometry::compute(&cfg).expect("geometry");
        assert_eq!(g.capacity(SlotClass::L), (256 << 10) - 64);
        assert_eq!(g.capacity(SlotClass::S), (16 << 10) - 64);
        let slots_end = g.slot_offset(SlotClass::S, g.s_slot_count);
        assert!(slots_end <= g.total_bytes);
        assert_eq!(g.slot_offset(SlotClass::L, 0), g.slot_region_offset);
        assert_eq!(
            g.slot_offset(SlotClass::S, 0),
            g.slot_region_offset + g.l_slot_count * u64::from(g.l_slot_size)
        );
    }

    #[test]
    fn fixpoint_is_deterministic_and_counts_shrink() {
        let cfg = MkfsConfig::new(64 << 20);
        let a = Geometry::compute(&cfg).expect("a");
        let b = Geometry::compute(&cfg).expect("b");
        assert_eq!(a, b);
        // Naive estimate (single bitmap page) can only overestimate counts.
        let slot_off = align_up(8192 + 2 * 4096, SLOT_REGION_ALIGN);
        let region = cfg.total_bytes - slot_off;
        let naive_l = ((region as f64 * 0.9) as u64) / u64::from(a.l_slot_size);
        assert!(a.l_slot_count <= naive_l);
    }

    #[test]
    fn rejects_bad_configs() {
        assert!(Geometry::compute(&MkfsConfig::new(1 << 20)).is_err());
        let mut cfg = MkfsConfig::new(1 << 30);
        cfg.l_fraction = 1.5;
        assert!(Geometry::compute(&cfg).is_err());
        let mut cfg = MkfsConfig::new(1 << 30);
        cfg.s_slot_size = 1000;
        assert!(Geometry::compute(&cfg).is_err());
        let mut cfg = MkfsConfig::new(1 << 30);
        cfg.s_slot_size = 2 << 20;
        assert!(Geometry::compute(&cfg).is_err());
    }

    #[test]
    fn from_sb_roundtrip_and_validation() {
        let cfg = MkfsConfig::new(1 << 30);
        let g = Geometry::compute(&cfg).expect("geometry");
        let sb = superblock_for(&g, 0, [1; 16], 7);
        assert_eq!(Geometry::from_sb(&sb).expect("from_sb"), g);
        let mut bad = sb.clone();
        bad.slot_region_offset = 4096;
        assert!(Geometry::from_sb(&bad).is_err());
        let mut bad = sb.clone();
        bad.l_slot_count = u64::MAX / 2;
        assert!(Geometry::from_sb(&bad).is_err());
        let mut bad = sb;
        bad.bitmap_copy_bytes = 1234;
        assert!(Geometry::from_sb(&bad).is_err());
    }
}
