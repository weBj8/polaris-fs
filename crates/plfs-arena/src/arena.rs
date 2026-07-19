//! Arena operations (contract §5–§7): mkfs, open (header scan + bitmap
//! reconcile), Put (header-last atomic publish), Get, Stat, Delete, List,
//! and lazy bitmap flush on close.

use std::fs::File;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use nix::fcntl::{FallocateFlags, fallocate};

use crate::bitmap::Bitmaps;
use crate::geom::{self, Geometry, MkfsConfig, MkfsReport, align_up};
use crate::header::{HEADER_LEN, SlotHeader};
use crate::index::{ChunkMeta, Index, SlotClass};
use crate::io::{AlignedBuf, IoEngine, IoPrep, blk_discard, block_device_size, is_block_device};
use crate::sb::{
    FLAG_BLOCK_DEVICE, FLAG_DISCARD_OK, FLAG_PUNCH_OK, FORMAT_VERSION, SB_A_OFFSET, SB_B_OFFSET,
    SB_LEN, Superblock,
};
use crate::{ArenaError, ChunkId, Result};

/// Sequential scan reads windows of about this size (whole slots per window).
const SCAN_WINDOW_TARGET: u64 = 4 << 20;

/// Golden-ratio multiplier spreading allocation start offsets.
const ALLOC_SPREAD: u64 = 0x9E37_79B9_7F4A_7C15;

/// Evidence of a duplicate chunk_id found at open (contract §5): should never
/// happen; the higher (class, slot_no) was kept, the other slot freed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorruptionEvidence {
    /// The duplicated chunk id.
    pub chunk_id: ChunkId,
    /// Slot that won and stays allocated.
    pub kept: (SlotClass, u64),
    /// Slot that lost and was freed.
    pub dropped: (SlotClass, u64),
}

/// Result of a successful [`Arena::put`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PutOutcome {
    /// Payload was written and sealed into a fresh slot.
    Stored,
    /// Idempotent no-op: same id, version, and payload crc already sealed.
    NoOp,
}

/// An open ChunkArena (one sparse image file or block device).
pub struct Arena {
    io: IoEngine,
    geom: Geometry,
    flags: u32,
    arena_uuid: [u8; 16],
    created_at: u64,
    index: Index,
    bitmaps: Bitmaps,
    alloc_seq: u64,
    corruptions: Vec<CorruptionEvidence>,
    closed: bool,
}

impl Arena {
    /// Create a fresh arena at `path` (contract §6): size the sparse file (or
    /// take the block device as-is), compute geometry by fixpoint, probe
    /// capabilities by actually punching/discarding, write zeroed bitmap
    /// copies, write superblock B then A, fdatasync. The slot region is not
    /// touched. `path` must not contain live data — stale slot bytes are
    /// never zeroed.
    pub fn mkfs(path: impl AsRef<Path>, cfg: &MkfsConfig) -> Result<MkfsReport> {
        let io = IoEngine::open(path.as_ref(), true)?;
        let is_block = is_block_device(&io.file);
        let mut cfg = cfg.clone();
        let mut flags = 0u32;
        if is_block {
            cfg.total_bytes = block_device_size(&io.file)?;
            flags |= FLAG_BLOCK_DEVICE;
        } else {
            io.file.set_len(cfg.total_bytes)?;
        }
        let geom = Geometry::compute(&cfg)?;
        if is_block {
            let off = geom.total_bytes.saturating_sub(1 << 20) & !4095;
            if blk_discard(&io.file, off, 1 << 20).is_ok() {
                flags |= FLAG_DISCARD_OK;
            }
        } else if punch_hole(&io.file, geom.slot_region_offset, 4096).is_ok() {
            flags |= FLAG_PUNCH_OK;
        }
        let bitmaps = Bitmaps::new(geom.l_slot_count, geom.s_slot_count);
        write_bitmap_copies(&io.file, &geom, &bitmaps)?;
        let arena_uuid = uuid::Uuid::now_v7().into_bytes();
        let created_at = unix_now();
        let sb = geom::superblock_for(&geom, flags, arena_uuid, created_at).encode();
        write_aligned(&io.file, &sb, SB_B_OFFSET)?;
        write_aligned(&io.file, &sb, SB_A_OFFSET)?;
        io.file.sync_data()?;
        Ok(MkfsReport {
            geometry: geom,
            arena_uuid,
            created_at,
            punch_ok: flags & FLAG_PUNCH_OK != 0,
            discard_ok: flags & FLAG_DISCARD_OK != 0,
            block_device: is_block,
        })
    }

    /// Open an existing arena (contract §5): read superblock A then B, scan
    /// every slot header through the I/O pipeline, rebuild the index and
    /// bitmaps from the headers (headers are truth), then rewrite and sync
    /// both bitmap copies.
    pub fn open(path: impl AsRef<Path>) -> Result<Arena> {
        let io = IoEngine::open(path.as_ref(), false)?;
        let mut block = AlignedBuf::zeroed(SB_LEN)?;
        let mut found = None;
        for off in [SB_A_OFFSET, SB_B_OFFSET] {
            if io.file.read_exact_at(&mut block[..], off).is_err() {
                continue;
            }
            if let Ok(sb) = Superblock::decode(&block[..]) {
                found = Some(sb);
                break;
            }
        }
        let sb = found.ok_or(ArenaError::BadSuperblock)?;
        if sb.format_version != FORMAT_VERSION {
            return Err(ArenaError::UnsupportedVersion(sb.format_version));
        }
        let geom = Geometry::from_sb(&sb)?;
        let mut arena = Arena {
            io,
            geom,
            flags: sb.flags,
            arena_uuid: sb.arena_uuid,
            created_at: sb.created_at,
            index: Index::new(),
            bitmaps: Bitmaps::new(geom.l_slot_count, geom.s_slot_count),
            alloc_seq: 0,
            corruptions: Vec::new(),
            closed: false,
        };
        arena.scan_class(SlotClass::L)?;
        arena.scan_class(SlotClass::S)?;
        arena.io.reset_pool();
        arena.flush_bitmap()?;
        Ok(arena)
    }

    /// Scan one class region in sequential windows; classify each slot per
    /// contract §5 (valid SEALED header => allocated, everything else free)
    /// and resolve duplicate chunk_ids (higher (class, slot_no) wins).
    fn scan_class(&mut self, class: SlotClass) -> Result<()> {
        let count = self.geom.slot_count(class);
        if count == 0 {
            return Ok(());
        }
        let slot_size = u64::from(self.geom.slot_size(class));
        let spw = (SCAN_WINDOW_TARGET / slot_size).max(1);
        let window = spw * slot_size;
        let n = count.div_ceil(spw) as usize;
        let capacity = self.geom.capacity(class);
        let region_start = self.geom.class_region_offset(class);
        let index = &mut self.index;
        let bitmaps = &mut self.bitmaps;
        let corruptions = &mut self.corruptions;
        self.io.drive(
            n,
            window as usize,
            |op, _buf| {
                let first = op as u64 * spw;
                let slots = (count - first).min(spw);
                Ok(IoPrep {
                    offset: region_start + first * slot_size,
                    len: (slots * slot_size) as usize,
                    write: false,
                })
            },
            |op, buf| {
                let first = op as u64 * spw;
                let slots = (count - first).min(spw);
                for k in 0..slots {
                    let slot_no = first + k;
                    let base = (k * slot_size) as usize;
                    let Some(h) = SlotHeader::decode_valid(&buf[base..base + HEADER_LEN], capacity)
                    else {
                        continue;
                    };
                    if !h.sealed {
                        continue;
                    }
                    let meta = ChunkMeta {
                        chunk_id: h.chunk_id,
                        version: h.version,
                        payload_len: h.payload_len,
                        payload_crc32c: h.payload_crc32c,
                        class,
                        slot_no,
                    };
                    match index.get(&h.chunk_id) {
                        None => {
                            index.insert(meta);
                            bitmaps.set(class, slot_no);
                        }
                        Some(existing)
                            if (existing.class, existing.slot_no) >= (class, slot_no) =>
                        {
                            corruptions.push(CorruptionEvidence {
                                chunk_id: h.chunk_id,
                                kept: (existing.class, existing.slot_no),
                                dropped: (class, slot_no),
                            });
                        }
                        Some(existing) => {
                            corruptions.push(CorruptionEvidence {
                                chunk_id: h.chunk_id,
                                kept: (class, slot_no),
                                dropped: (existing.class, existing.slot_no),
                            });
                            bitmaps.clear(existing.class, existing.slot_no);
                            index.insert(meta);
                            bitmaps.set(class, slot_no);
                        }
                    }
                }
                Ok(())
            },
        )
    }

    /// Store a chunk (contract §6): class select, free-slot bitmap scan from
    /// a pseudo-random start, payload write, single 64-byte SEALED header
    /// write (atomic publish), fdatasync, index insert, lazy bitmap bit.
    ///
    /// Idempotent: same id + version + payload crc => `NoOp`; same id with a
    /// different (version, crc) => `AlreadyExists`.
    pub fn put(&mut self, id: ChunkId, version: u64, payload: &[u8]) -> Result<PutOutcome> {
        if id.is_nil() {
            return Err(invalid_input("nil chunk id is forbidden"));
        }
        let crc = crc32fast::hash(payload);
        if let Some(m) = self.index.get(&id) {
            if m.version == version && m.payload_crc32c == crc {
                return Ok(PutOutcome::NoOp);
            }
            return Err(ArenaError::AlreadyExists(id));
        }
        let len = payload.len() as u64;
        let class = if len <= self.geom.capacity(SlotClass::S) {
            SlotClass::S
        } else if len <= self.geom.capacity(SlotClass::L) {
            SlotClass::L
        } else {
            return Err(ArenaError::Oversize {
                len: payload.len(),
                max: self.geom.capacity(SlotClass::L) as usize,
            });
        };
        let count = self.geom.slot_count(class);
        let id_hash = u64::from_le_bytes(id.as_bytes()[0..8].try_into().expect("16 bytes"));
        let start = (id_hash ^ self.alloc_seq.wrapping_mul(ALLOC_SPREAD)) % count.max(1);
        self.alloc_seq = self.alloc_seq.wrapping_add(1);
        let slot_no = self
            .bitmaps
            .find_free_from(class, start)
            .ok_or(ArenaError::OutOfSpace { class })?;
        let slot_off = self.geom.slot_offset(class, slot_no);
        let header = SlotHeader {
            sealed: true,
            chunk_id: id,
            version,
            payload_len: len,
            payload_crc32c: crc,
        }
        .encode();
        if self.io.is_direct() {
            // O_DIRECT alignment: offsets/lengths must be 512-aligned, so
            // publish as two writes — [zero pad(64) | payload | tail pad]
            // over [0, align_up(64+len, 512)), then 512 bytes of
            // [header(64) | payload[..448]] at slot+0 (payload bytes are
            // identical to the first write).
            let len1 = align_up(64 + len, 512) as usize;
            let buf_len1 = align_up(len1 as u64, 4096) as usize;
            self.io.drive(
                1,
                buf_len1,
                |_, buf| {
                    buf.zero_range(0, 64);
                    buf.write_bytes(64, payload);
                    buf.zero_range(64 + payload.len(), len1 - 64 - payload.len());
                    Ok(IoPrep {
                        offset: slot_off,
                        len: len1,
                        write: true,
                    })
                },
                |_, _| Ok(()),
            )?;
            let prefix = payload.len().min(448);
            self.io.drive(
                1,
                4096,
                |_, buf| {
                    buf.write_bytes(0, &header);
                    buf.write_bytes(64, &payload[..prefix]);
                    buf.zero_range(64 + prefix, 448 - prefix);
                    Ok(IoPrep {
                        offset: slot_off,
                        len: 512,
                        write: true,
                    })
                },
                |_, _| Ok(()),
            )?;
        } else {
            self.io.file.write_all_at(payload, slot_off + 64)?;
            self.io.file.write_all_at(&header, slot_off)?;
        }
        self.io.file.sync_data()?;
        self.index.insert(ChunkMeta {
            chunk_id: id,
            version,
            payload_len: len,
            payload_crc32c: crc,
            class,
            slot_no,
        });
        self.bitmaps.set(class, slot_no);
        Ok(PutOutcome::Stored)
    }

    /// Fetch a chunk (contract §6): index lookup, one aligned read of
    /// 64+payload_len, verify header valid + SEALED + payload crc.
    pub fn get(&mut self, id: &ChunkId) -> Result<(u64, Vec<u8>)> {
        let meta = self
            .index
            .get(id)
            .cloned()
            .ok_or(ArenaError::NotFound(*id))?;
        let slot_off = self.geom.slot_offset(meta.class, meta.slot_no);
        let need = 64 + meta.payload_len as usize;
        let mut raw = Vec::with_capacity(need);
        if self.io.is_direct() {
            let read_len = align_up(need as u64, 512) as usize;
            let buf_len = align_up(need as u64, 4096) as usize;
            self.io.drive(
                1,
                buf_len,
                |_, _| {
                    Ok(IoPrep {
                        offset: slot_off,
                        len: read_len,
                        write: false,
                    })
                },
                |_, buf| {
                    raw.extend_from_slice(&buf[..need]);
                    Ok(())
                },
            )?;
        } else {
            raw.resize(need, 0);
            self.io.file.read_exact_at(&mut raw, slot_off)?;
        }
        let capacity = self.geom.capacity(meta.class);
        let corrupt = |detail| ArenaError::Corrupt {
            chunk_id: *id,
            detail,
        };
        let h = SlotHeader::decode_valid(&raw[..HEADER_LEN], capacity)
            .ok_or_else(|| corrupt("invalid slot header"))?;
        if !h.sealed {
            return Err(corrupt("header not sealed"));
        }
        if h.chunk_id != *id {
            return Err(corrupt("header chunk id mismatch"));
        }
        if h.version != meta.version {
            return Err(corrupt("header version mismatch"));
        }
        if h.payload_len != meta.payload_len {
            return Err(corrupt("header payload length mismatch"));
        }
        let payload = raw[HEADER_LEN..].to_vec();
        if crc32fast::hash(&payload) != h.payload_crc32c {
            return Err(corrupt("payload crc32c mismatch"));
        }
        Ok((h.version, payload))
    }

    /// Index-only metadata lookup; no disk I/O.
    pub fn stat(&self, id: &ChunkId) -> Option<ChunkMeta> {
        self.index.get(id).cloned()
    }

    /// Exact-version delete (contract §6): absent or version mismatch =>
    /// `Ok(false)`; else index remove + lazy bit clear + hole punch /
    /// BLKDISCARD / bitmap-only free. No fsync on this path.
    pub fn delete(&mut self, id: &ChunkId, version: u64) -> Result<bool> {
        let Some(meta) = self.index.get(id).cloned() else {
            return Ok(false);
        };
        if meta.version != version {
            return Ok(false);
        }
        self.index.remove(id);
        self.bitmaps.clear(meta.class, meta.slot_no);
        let slot_off = self.geom.slot_offset(meta.class, meta.slot_no);
        let slot_size = u64::from(self.geom.slot_size(meta.class));
        if self.flags & FLAG_PUNCH_OK != 0 {
            punch_hole(&self.io.file, slot_off, slot_size)?;
        } else if self.flags & FLAG_DISCARD_OK != 0 {
            blk_discard(&self.io.file, slot_off, slot_size)?;
        }
        Ok(true)
    }

    /// All live chunks (index order is unspecified). No disk I/O.
    pub fn list(&self) -> Vec<ChunkMeta> {
        self.index.iter().cloned().collect()
    }

    /// Number of live chunks.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// True when no live chunks are indexed.
    pub fn is_empty(&self) -> bool {
        self.index.len() == 0
    }

    /// Free slots of a class per the in-memory bitmap (hint, kept exact by
    /// every put/delete and rebuilt from headers at open).
    pub fn free_slots(&self, class: SlotClass) -> u64 {
        self.bitmaps.free_count(class)
    }

    /// Geometry as read from (or written by mkfs into) the superblock.
    pub fn geometry(&self) -> &Geometry {
        &self.geom
    }

    /// Arena UUID from the superblock.
    pub fn arena_uuid(&self) -> [u8; 16] {
        self.arena_uuid
    }

    /// mkfs time, unix seconds.
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    /// True when running O_DIRECT + io_uring (false = buffered syscalls).
    pub fn is_direct(&self) -> bool {
        self.io.is_direct()
    }

    /// Backend supports hole punching (`PUNCH_OK`).
    pub fn punch_ok(&self) -> bool {
        self.flags & FLAG_PUNCH_OK != 0
    }

    /// Backend supports BLKDISCARD (`DISCARD_OK`).
    pub fn discard_ok(&self) -> bool {
        self.flags & FLAG_DISCARD_OK != 0
    }

    /// Backend is a block device (`BLOCK_DEVICE`).
    pub fn block_device(&self) -> bool {
        self.flags & FLAG_BLOCK_DEVICE != 0
    }

    /// Duplicate-chunkId corruption evidence collected during open's scan.
    pub fn corruption_evidence(&self) -> &[CorruptionEvidence] {
        &self.corruptions
    }

    /// Write both bitmap copies (+crc) and fdatasync them.
    pub fn sync_bitmap(&mut self) -> Result<()> {
        self.flush_bitmap()
    }

    fn flush_bitmap(&mut self) -> Result<()> {
        write_bitmap_copies(&self.io.file, &self.geom, &self.bitmaps)?;
        self.io.file.sync_data()?;
        Ok(())
    }

    /// Flush the bitmaps and close (best-effort flush also happens on Drop).
    pub fn close(mut self) -> Result<()> {
        self.flush_bitmap()?;
        self.closed = true;
        Ok(())
    }

    /// Abandon the arena WITHOUT the Drop-time bitmap flush, simulating a
    /// crash (kill -9 / power loss): everything fdatasynced stays durable,
    /// the lazy bitmap flush is lost. Test/simulation support for the
    /// crash-consistency gates — production code should use [`Arena::close`].
    /// Leaks the file descriptor (process exit reclaims it).
    pub fn abandon(self) {
        std::mem::forget(self);
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        if !self.closed {
            let _ = self.flush_bitmap();
        }
    }
}

fn punch_hole(file: &File, offset: u64, len: u64) -> Result<()> {
    fallocate(
        file,
        FallocateFlags::FALLOC_FL_PUNCH_HOLE | FallocateFlags::FALLOC_FL_KEEP_SIZE,
        offset as i64,
        len as i64,
    )
    .map_err(std::io::Error::from)?;
    Ok(())
}

/// Write `bytes` (length a multiple of 4096) at `offset` through an aligned
/// bounce buffer so O_DIRECT constraints hold on every path.
fn write_aligned(file: &File, bytes: &[u8], offset: u64) -> Result<()> {
    let mut buf = AlignedBuf::zeroed(bytes.len())?;
    buf.write_bytes(0, bytes);
    file.write_all_at(&buf[..], offset)?;
    Ok(())
}

fn write_bitmap_copies(file: &File, geom: &Geometry, bitmaps: &Bitmaps) -> Result<()> {
    let mut payload = bitmaps.encode();
    payload.resize(geom.bitmap_copy_bytes as usize, 0);
    write_aligned(file, &payload, geom.bitmap_region_offset)?;
    write_aligned(
        file,
        &payload,
        geom.bitmap_region_offset + geom.bitmap_copy_bytes,
    )?;
    Ok(())
}

fn invalid_input(msg: &'static str) -> ArenaError {
    ArenaError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, msg))
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT: AtomicU64 = AtomicU64::new(0);

    struct Cleanup(std::path::PathBuf);

    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    fn temp_path(tag: &str) -> (std::path::PathBuf, Cleanup) {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let p = crate::test_tmpdir().join(format!("plfs-arena-{tag}-{}-{n}", std::process::id()));
        (p.clone(), Cleanup(p))
    }

    fn small_cfg(total: u64) -> MkfsConfig {
        MkfsConfig {
            total_bytes: total,
            l_fraction: 0.90,
            l_slot_size: 256 << 10,
            s_slot_size: 16 << 10,
        }
    }

    fn mk_arena(tag: &str) -> (Arena, std::path::PathBuf, Cleanup) {
        let (path, guard) = temp_path(tag);
        Arena::mkfs(&path, &small_cfg(8 << 20)).expect("mkfs");
        let arena = Arena::open(&path).expect("open");
        (arena, path, guard)
    }

    fn fill(byte: u8, len: usize) -> Vec<u8> {
        (0..len).map(|i| byte.wrapping_add(i as u8)).collect()
    }

    /// Write a raw slot (header + payload) directly, bypassing the arena.
    fn write_raw_slot(
        path: &Path,
        geom: &Geometry,
        class: SlotClass,
        slot_no: u64,
        h: &SlotHeader,
        payload: &[u8],
    ) {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .expect("raw open");
        let off = geom.slot_offset(class, slot_no);
        file.write_all_at(payload, off + 64).expect("raw payload");
        file.write_all_at(&h.encode(), off).expect("raw header");
        file.sync_data().expect("raw sync");
    }

    fn raw_header(id: ChunkId, version: u64, sealed: bool, payload: &[u8]) -> SlotHeader {
        SlotHeader {
            sealed,
            chunk_id: id,
            version,
            payload_len: payload.len() as u64,
            payload_crc32c: crc32fast::hash(payload),
        }
    }

    /// Read bitmap copy A from disk and decode it.
    fn read_bitmap(path: &Path, geom: &Geometry) -> Bitmaps {
        let file = std::fs::File::open(path).expect("bitmap open");
        let payload_len = Bitmaps::payload_len(geom.l_slot_count, geom.s_slot_count);
        let mut buf = vec![0u8; payload_len + 4];
        file.read_exact_at(&mut buf, geom.bitmap_region_offset)
            .expect("bitmap read");
        Bitmaps::decode(&buf, geom.l_slot_count, geom.s_slot_count).expect("bitmap decode")
    }

    /// Overwrite both on-disk bitmap copies with a crafted map.
    fn write_bitmap(path: &Path, geom: &Geometry, bitmaps: &Bitmaps) {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .expect("bitmap open");
        write_bitmap_copies(&file, geom, bitmaps).expect("bitmap write");
        file.sync_data().expect("bitmap sync");
    }

    #[test]
    fn mkfs_then_open_empty() {
        let (path, _g) = temp_path("mkfs");
        let report = Arena::mkfs(&path, &small_cfg(8 << 20)).expect("mkfs");
        assert!(report.geometry.l_slot_count > 0);
        assert!(report.geometry.s_slot_count > 0);
        let arena = Arena::open(&path).expect("open");
        assert_eq!(arena.len(), 0);
        assert!(arena.is_empty());
        assert_eq!(arena.arena_uuid(), report.arena_uuid);
        assert_eq!(arena.free_slots(SlotClass::L), report.geometry.l_slot_count);
        assert_eq!(arena.free_slots(SlotClass::S), report.geometry.s_slot_count);
        assert_eq!(arena.list(), Vec::new());
    }

    #[test]
    fn put_get_byte_exact() {
        let (mut arena, _p, _g) = mk_arena("putget");
        for (len, version) in [
            (0usize, 1u64),
            (1, 2),
            (447, 3),
            (448, 4),
            (449, 5),
            (16_000, 6),
            (100_000, 7),
        ] {
            let id = ChunkId::new_v7();
            let payload = fill(0x11, len);
            assert_eq!(
                arena.put(id, version, &payload).expect("put"),
                PutOutcome::Stored
            );
            let (v, got) = arena.get(&id).expect("get");
            assert_eq!(v, version);
            assert_eq!(got, payload);
        }
    }

    #[test]
    fn class_boundary_selection() {
        let (mut arena, _p, _g) = mk_arena("class");
        let s_cap = arena.geometry().capacity(SlotClass::S) as usize;
        let l_cap = arena.geometry().capacity(SlotClass::L) as usize;
        let id_s = ChunkId::new_v7();
        arena.put(id_s, 1, &fill(1, s_cap)).expect("put S");
        assert_eq!(arena.stat(&id_s).expect("stat").class, SlotClass::S);
        let id_l = ChunkId::new_v7();
        arena.put(id_l, 1, &fill(2, s_cap + 1)).expect("put L");
        assert_eq!(arena.stat(&id_l).expect("stat").class, SlotClass::L);
        let id_big = ChunkId::new_v7();
        match arena.put(id_big, 1, &fill(3, l_cap + 1)) {
            Err(ArenaError::Oversize { len, max }) => {
                assert_eq!(len, l_cap + 1);
                assert_eq!(max, l_cap);
            }
            other => panic!("expected Oversize, got {other:?}"),
        }
    }

    #[test]
    fn idempotent_put_matrix() {
        let (mut arena, path, _g) = mk_arena("idem");
        let id = ChunkId::new_v7();
        let payload = fill(7, 1000);
        assert_eq!(arena.put(id, 5, &payload).expect("put"), PutOutcome::Stored);
        assert_eq!(arena.put(id, 5, &payload).expect("noop"), PutOutcome::NoOp);
        assert!(matches!(
            arena.put(id, 6, &payload),
            Err(ArenaError::AlreadyExists(_))
        ));
        assert!(matches!(
            arena.put(id, 5, &fill(8, 1000)),
            Err(ArenaError::AlreadyExists(_))
        ));
        assert_eq!(arena.len(), 1);
        drop(arena);
        let mut arena = Arena::open(&path).expect("reopen");
        assert_eq!(arena.put(id, 5, &payload).expect("noop"), PutOutcome::NoOp);
        assert!(matches!(
            arena.put(id, 9, &payload),
            Err(ArenaError::AlreadyExists(_))
        ));
    }

    #[test]
    fn delete_exact_version_matrix() {
        let (mut arena, path, _g) = mk_arena("del");
        let punch = arena.punch_ok();
        let id = ChunkId::new_v7();
        let payload = fill(3, 500);
        assert!(!arena.delete(&id, 1).expect("absent"));
        arena.put(id, 7, &payload).expect("put");
        assert!(!arena.delete(&id, 8).expect("version mismatch"));
        assert!(arena.stat(&id).is_some());
        assert!(arena.delete(&id, 7).expect("exact"));
        assert!(!arena.delete(&id, 7).expect("again"));
        assert!(arena.stat(&id).is_none());
        assert!(matches!(arena.get(&id), Err(ArenaError::NotFound(_))));
        arena.close().expect("close");
        if punch {
            // Punched header cannot resurrect (contract §7); without punch a
            // resurrected chunk would be legal and re-deleted by GC.
            let mut arena = Arena::open(&path).expect("reopen");
            assert!(matches!(arena.get(&id), Err(ArenaError::NotFound(_))));
            assert!(arena.list().is_empty());
        }
    }

    #[test]
    fn unknown_id_paths() {
        let (mut arena, _p, _g) = mk_arena("unknown");
        let id = ChunkId::new_v7();
        assert!(arena.stat(&id).is_none());
        assert!(matches!(arena.get(&id), Err(ArenaError::NotFound(_))));
        assert!(!arena.delete(&id, 0).expect("delete"));
    }

    #[test]
    fn out_of_space_when_class_full() {
        let (mut arena, _p, _g) = mk_arena("oos");
        let s_cap = arena.geometry().capacity(SlotClass::S) as usize;
        let s_count = arena.geometry().s_slot_count;
        for i in 0..s_count {
            let id = ChunkId::new_v7();
            arena.put(id, i, &fill(1, 100)).expect("fill");
        }
        assert_eq!(arena.free_slots(SlotClass::S), 0);
        let err = arena.put(ChunkId::new_v7(), 0, &fill(1, 100));
        assert!(matches!(
            err,
            Err(ArenaError::OutOfSpace {
                class: SlotClass::S
            })
        ));
        let l_free = arena.free_slots(SlotClass::L);
        assert_eq!(l_free, arena.geometry().l_slot_count);
        let big = fill(2, s_cap + 1);
        arena.put(ChunkId::new_v7(), 0, &big).expect("L still open");
    }

    #[test]
    fn reopen_persistence_byte_exact() {
        let (mut arena, path, _g) = mk_arena("reopen");
        let mut expected = Vec::new();
        for i in 0..8u64 {
            let id = ChunkId::new_v7();
            let payload = fill(i as u8, (i as usize) * 1000 + 17);
            arena.put(id, i + 100, &payload).expect("put");
            expected.push((id, i + 100, payload));
        }
        arena.close().expect("close");
        let mut arena = Arena::open(&path).expect("reopen");
        assert_eq!(arena.len(), expected.len());
        for (id, version, payload) in &expected {
            let (v, got) = arena.get(id).expect("get");
            assert_eq!(&v, version);
            assert_eq!(&got, payload);
        }
        let mut listed: Vec<ChunkId> = arena.list().iter().map(|m| m.chunk_id).collect();
        listed.sort();
        let mut want: Vec<ChunkId> = expected.iter().map(|e| e.0).collect();
        want.sort();
        assert_eq!(listed, want);
    }

    #[test]
    fn reconcile_cleared_bit_valid_header_allocates() {
        let (mut arena, path, _g) = mk_arena("recon1");
        let id = ChunkId::new_v7();
        let payload = fill(9, 1234);
        arena.put(id, 1, &payload).expect("put");
        let meta = arena.stat(&id).expect("stat");
        let geom = *arena.geometry();
        arena.close().expect("close");
        // Craft a bitmap with the chunk's bit cleared (as if the lazy flush
        // never ran); the SEALED header must still win at open.
        let bitmaps = Bitmaps::new(geom.l_slot_count, geom.s_slot_count);
        write_bitmap(&path, &geom, &bitmaps);
        let mut arena = Arena::open(&path).expect("reopen");
        let (v, got) = arena.get(&id).expect("get");
        assert_eq!(v, 1);
        assert_eq!(got, payload);
        assert_eq!(
            arena.free_slots(meta.class),
            arena.geometry().slot_count(meta.class) - 1
        );
        let on_disk = read_bitmap(&path, &geom);
        assert!(on_disk.test(meta.class, meta.slot_no));
    }

    #[test]
    fn reconcile_stale_bit_without_header_is_cleared() {
        let (arena, path, _g) = mk_arena("recon2");
        let geom = *arena.geometry();
        assert!(arena.is_empty());
        drop(arena);
        // Craft a bitmap with a stale allocated bit for a never-written slot.
        let mut bitmaps = Bitmaps::new(geom.l_slot_count, geom.s_slot_count);
        bitmaps.set(SlotClass::L, 3);
        bitmaps.set(SlotClass::S, 2);
        write_bitmap(&path, &geom, &bitmaps);
        let mut arena = Arena::open(&path).expect("reopen");
        assert!(arena.is_empty());
        assert_eq!(arena.free_slots(SlotClass::L), geom.l_slot_count);
        assert_eq!(arena.free_slots(SlotClass::S), geom.s_slot_count);
        let on_disk = read_bitmap(&path, &geom);
        assert!(!on_disk.test(SlotClass::L, 3));
        assert!(!on_disk.test(SlotClass::S, 2));
        // The stale-marked slots are allocatable again.
        let id = ChunkId::new_v7();
        arena
            .put(id, 1, &fill(1, 100))
            .expect("put after reconcile");
    }

    #[test]
    fn duplicate_chunk_id_higher_slot_wins() {
        let (path, _g) = temp_path("dup");
        let report = Arena::mkfs(&path, &small_cfg(8 << 20)).expect("mkfs");
        let geom = report.geometry;
        let id = ChunkId::new_v7();
        let loser = fill(1, 300);
        let winner = fill(2, 400);
        write_raw_slot(
            &path,
            &geom,
            SlotClass::L,
            0,
            &raw_header(id, 1, true, &loser),
            &loser,
        );
        write_raw_slot(
            &path,
            &geom,
            SlotClass::L,
            1,
            &raw_header(id, 2, true, &winner),
            &winner,
        );
        let mut arena = Arena::open(&path).expect("open");
        assert_eq!(arena.len(), 1);
        let (v, got) = arena.get(&id).expect("get");
        assert_eq!(v, 2);
        assert_eq!(got, winner);
        assert_eq!(arena.stat(&id).expect("stat").slot_no, 1);
        assert_eq!(arena.corruption_evidence().len(), 1);
        assert_eq!(
            arena.corruption_evidence()[0],
            CorruptionEvidence {
                chunk_id: id,
                kept: (SlotClass::L, 1),
                dropped: (SlotClass::L, 0),
            }
        );
        assert_eq!(arena.free_slots(SlotClass::L), geom.l_slot_count - 1);
    }

    #[test]
    fn unsealed_header_is_free_at_boot() {
        let (path, _g) = temp_path("unsealed");
        let report = Arena::mkfs(&path, &small_cfg(8 << 20)).expect("mkfs");
        let geom = report.geometry;
        let id = ChunkId::new_v7();
        let payload = fill(5, 200);
        write_raw_slot(
            &path,
            &geom,
            SlotClass::L,
            0,
            &raw_header(id, 1, false, &payload),
            &payload,
        );
        let mut arena = Arena::open(&path).expect("open");
        assert!(arena.is_empty());
        assert!(matches!(arena.get(&id), Err(ArenaError::NotFound(_))));
        assert_eq!(arena.free_slots(SlotClass::L), geom.l_slot_count);
    }

    #[test]
    fn corrupt_payload_detected_on_get() {
        let (mut arena, path, _g) = mk_arena("corrupt");
        let id = ChunkId::new_v7();
        let payload = fill(4, 1000);
        arena.put(id, 1, &payload).expect("put");
        let meta = arena.stat(&id).expect("stat");
        let geom = *arena.geometry();
        arena.close().expect("close");
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .expect("raw open");
        let off = geom.slot_offset(meta.class, meta.slot_no) + 64;
        file.write_all_at(&[0xFF], off).expect("flip");
        file.sync_data().expect("sync");
        drop(file);
        let mut arena = Arena::open(&path).expect("reopen");
        assert!(matches!(arena.get(&id), Err(ArenaError::Corrupt { .. })));
    }

    #[test]
    fn nil_chunk_id_rejected() {
        let (mut arena, _p, _g) = mk_arena("nil");
        assert!(arena.put(ChunkId::from([0; 16]), 1, b"x").is_err());
    }

    #[test]
    fn sync_bitmap_persists_bits() {
        let (mut arena, path, _g) = mk_arena("syncbm");
        let id = ChunkId::new_v7();
        arena.put(id, 1, &fill(6, 100)).expect("put");
        let meta = arena.stat(&id).expect("stat");
        arena.sync_bitmap().expect("sync");
        let geom = *arena.geometry();
        arena.close().expect("close");
        let on_disk = read_bitmap(&path, &geom);
        assert!(on_disk.test(meta.class, meta.slot_no));
        let clear_count = if meta.class == SlotClass::L {
            geom.l_slot_count
        } else {
            geom.s_slot_count
        };
        assert_eq!(on_disk.free_count(meta.class), clear_count - 1);
    }
}
