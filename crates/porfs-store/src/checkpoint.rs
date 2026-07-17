//! Checkpoint write and read paths (format.md §4/§5). A checkpoint serializes
//! the *confirmed* index state (as of `confirmed_tail`, the durability
//! horizon) into the older of the two ping-pong slots, so the next mount can
//! load the index and scan only the log tail past `covered_tail`.
//!
//! Only confirmed state may enter a checkpoint:
//! - extents appended but not yet synced (record at/past `confirmed_tail`) are
//!   excluded — their log records are not durable, and the tail scan rediscovers
//!   them if they survived;
//! - extents with a *pending* (unsynced) tombstone are included as live — the
//!   tombstone is not durable, so the extent is still live at the horizon, and
//!   the tail scan re-applies the tombstone if it survived;
//! - extents with a synced tombstone are dropped (dead at the horizon).

use std::os::unix::fs::FileExt;

use porfs_format::{
    BLOCK_SIZE, CKPT_SLOT_CAP, CheckpointEntry, CheckpointHeader, DATA_START, checksum,
};

use crate::StoreError;
use crate::aligned::AlignedBuf;
use crate::store::{ExtentMeta, ExtentStore};

/// One of the two checkpoint slots (ping-pong targets, format.md §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CkptSlot {
    A,
    B,
}

impl CkptSlot {
    pub(crate) fn offset(self) -> u64 {
        match self {
            Self::A => porfs_format::CKPT_SLOT_A_OFF,
            Self::B => porfs_format::CKPT_SLOT_B_OFF,
        }
    }

    pub(crate) fn other(self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}

/// Newest checkpoint slot known to this store (read at open or written since).
#[derive(Debug, Clone, Copy)]
pub(crate) struct CkptMeta {
    pub(crate) slot_gen: u64,
    pub(crate) covered_tail: u64,
    pub(crate) entry_count: u32,
    pub(crate) slot: CkptSlot,
}

impl ExtentStore {
    /// Durability horizon (format.md §6): the maximum extent id confirmed by
    /// the last successful sync; 0 if no sync has confirmed any extent yet.
    ///
    /// Every id `<= confirmed_id()` is guaranteed readable after a crash; ids
    /// above it may or may not survive (callers must not rely on them). In a
    /// live session this is `next_extent_id - 1` at the last sync. After
    /// `open` it is derived from recovered state: every id found up to the
    /// salvaged tail is confirmed, because the superblock tail — which bounds
    /// the mount scan — only ever advances past fdatasynced records.
    pub fn confirmed_id(&self) -> u64 {
        self.confirmed_next_id.saturating_sub(1)
    }

    /// Set the group-commit policy: automatically `sync` after `max_pending`
    /// appends since the last sync (checked inside `append`/`append_batch`).
    /// One `fdatasync` then amortizes a whole batch of appends. `0` disables
    /// auto-sync; disabled is the default, preserving P1 bench semantics.
    pub fn set_commit_policy(&mut self, max_pending: usize) {
        self.commit_max_pending = max_pending;
    }

    /// True when the last `open` loaded its index from a checkpoint slot
    /// (fast mount); false when it fell back to a full log scan.
    pub fn mount_used_checkpoint(&self) -> bool {
        self.mount_used_ckpt
    }

    /// Generation of the newest valid checkpoint slot known (0 if none).
    pub fn checkpoint_slot_gen(&self) -> u64 {
        self.last_ckpt.map_or(0, |c| c.slot_gen)
    }

    /// Log offset the newest checkpoint's index covers (0 if none).
    pub fn checkpoint_covered_tail(&self) -> u64 {
        self.last_ckpt.map_or(0, |c| c.covered_tail)
    }

    /// Entry count of the newest checkpoint (0 if none).
    pub fn checkpoint_entry_count(&self) -> u32 {
        self.last_ckpt.map_or(0, |c| c.entry_count)
    }

    /// Test-only hook: shrink the slot capacity check of `checkpoint()` so the
    /// capacity fallback is exercisable without writing ~800k extents. The
    /// mount-side validation always uses the real `CKPT_SLOT_CAP`.
    #[doc(hidden)]
    pub fn set_ckpt_slot_cap(&mut self, cap: u64) {
        self.ckpt_slot_cap = cap;
    }
    /// Write a checkpoint of the confirmed index state into the older slot.
    ///
    /// The slot image (64-byte header + `entry_count * 40` payload, zero-padded
    /// to a 4KiB multiple) is written with one 4KiB-aligned write followed by
    /// `fdatasync`. Returns `Ok(false)` — writing nothing — when the entries do
    /// not fit the slot capacity (mount then falls back to a full scan, which
    /// is always correct).
    pub fn checkpoint(&mut self) -> Result<bool, StoreError> {
        let mut entries: Vec<CheckpointEntry> = Vec::with_capacity(self.index.len());
        let mut live_bytes = 0u64;
        for (&id, meta) in &self.index {
            if meta.offset + u64::from(meta.disk_len) > self.confirmed_tail {
                continue; // unsynced append: not durable at the horizon
            }
            if meta.tombstoned && !self.pending_tombs.contains(&id) {
                continue; // synced tombstone: dead at the horizon
            }
            live_bytes += u64::from(meta.data_len);
            entries.push(CheckpointEntry {
                extent_id: id,
                offset: meta.offset,
                disk_len: meta.disk_len,
                data_len: meta.data_len,
                inode: meta.inode,
                logical_offset: meta.logical_offset,
            });
        }
        entries.sort_unstable_by_key(|e| e.extent_id);
        let payload_len = entries.len() * CheckpointEntry::LEN;
        if payload_len as u64 + CheckpointHeader::LEN as u64 > self.ckpt_slot_cap {
            return Ok(false);
        }
        let slot_gen = self.last_ckpt.map_or(1, |c| c.slot_gen + 1);
        let slot = match self.last_ckpt {
            Some(c) => c.slot.other(),
            None => CkptSlot::A,
        };
        let total = (CheckpointHeader::LEN + payload_len).div_ceil(BLOCK_SIZE as usize)
            * BLOCK_SIZE as usize;
        let mut buf = AlignedBuf::zeroed(total)?;
        for (i, entry) in entries.iter().enumerate() {
            buf.write_bytes(
                CheckpointHeader::LEN + i * CheckpointEntry::LEN,
                entry.as_bytes(),
            );
        }
        let mut header = CheckpointHeader {
            magic: porfs_format::CKPT_MAGIC,
            header_len: CheckpointHeader::LEN as u16,
            flags: 0,
            slot_gen,
            covered_tail: self.confirmed_tail,
            next_extent_id: self.confirmed_next_id,
            extent_count: entries.len() as u64,
            live_bytes,
            entry_count: entries.len() as u32,
            payload_crc32c: checksum(
                &buf[CheckpointHeader::LEN..CheckpointHeader::LEN + payload_len],
            ),
            reserved: [0; 4],
            header_crc32c: 0,
        };
        header.seal();
        buf.write_bytes(0, header.as_bytes());
        self.file.write_all_at(&buf[..], slot.offset())?;
        self.file.sync_data()?;
        self.last_ckpt = Some(CkptMeta {
            slot_gen,
            covered_tail: self.confirmed_tail,
            entry_count: header.entry_count,
            slot,
        });
        Ok(true)
    }

    /// Read and validate one checkpoint slot. Returns `Ok(None)` for any
    /// validation failure (absent, torn, or corrupted slot): checkpoint
    /// candidates are advisory, the log is the source of truth.
    pub(crate) fn read_checkpoint(
        &self,
        slot: CkptSlot,
    ) -> Result<Option<(CheckpointHeader, Vec<CheckpointEntry>)>, StoreError> {
        let mut head = AlignedBuf::zeroed(BLOCK_SIZE as usize)?;
        self.file.read_exact_at(&mut head[..], slot.offset())?;
        let header = match CheckpointHeader::from_bytes(&head[..CheckpointHeader::LEN])
            .and_then(|h| h.validate().map(|()| h))
        {
            Ok(h) => h,
            Err(_) => return Ok(None),
        };
        let payload_len = header.entry_count as usize * CheckpointEntry::LEN;
        if payload_len as u64 + CheckpointHeader::LEN as u64 > CKPT_SLOT_CAP {
            return Ok(None);
        }
        let total = (CheckpointHeader::LEN + payload_len).div_ceil(BLOCK_SIZE as usize)
            * BLOCK_SIZE as usize;
        let mut buf = AlignedBuf::zeroed(total)?;
        buf[..BLOCK_SIZE as usize].copy_from_slice(&head[..]);
        if total > BLOCK_SIZE as usize {
            self.file
                .read_exact_at(&mut buf[BLOCK_SIZE as usize..], slot.offset() + BLOCK_SIZE)?;
        }
        let payload = &buf[CheckpointHeader::LEN..CheckpointHeader::LEN + payload_len];
        if checksum(payload) != header.payload_crc32c {
            return Ok(None);
        }
        let mut entries = Vec::with_capacity(header.entry_count as usize);
        for chunk in payload.chunks_exact(CheckpointEntry::LEN) {
            entries.push(CheckpointEntry::from_bytes(chunk)?);
        }
        Ok(Some((header, entries)))
    }

    /// A checkpoint is usable for a fast mount only if the log range it skips
    /// is inside the confirmed region: `covered_tail` must be aligned and
    /// between `DATA_START` and the superblock's confirmed tail.
    pub(crate) fn checkpoint_usable(&self, header: &CheckpointHeader, sb_tail: u64) -> bool {
        header.covered_tail >= DATA_START
            && header.covered_tail % BLOCK_SIZE == 0
            && header.covered_tail <= sb_tail
    }

    /// Load checkpoint entries into the in-memory index as live extents.
    pub(crate) fn load_checkpoint(&mut self, entries: &[CheckpointEntry]) {
        for e in entries {
            self.index.insert(
                e.extent_id,
                ExtentMeta {
                    offset: e.offset,
                    disk_len: e.disk_len,
                    data_len: e.data_len,
                    inode: e.inode,
                    logical_offset: e.logical_offset,
                    tombstoned: false,
                },
            );
        }
    }
}
