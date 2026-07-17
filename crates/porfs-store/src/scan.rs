//! Mount path (format.md §5): open the device, pick the winning superblock,
//! rebuild the in-memory index from the newest usable checkpoint plus a scan
//! of the log tail it does not cover — or a full scan from `DATA_START` when
//! no checkpoint is usable. Scans are bounded by the superblock's confirmed
//! tail and stop (salvage-truncate) at the first bad record.

use std::collections::{HashMap, HashSet};
use std::os::unix::fs::FileExt;
use std::path::Path;

use porfs_format::{
    BLOCK_SIZE, CKPT_SLOT_CAP, DATA_START, EXTENT_DATA_MAX, ExtentHeader, FormatError, SB_A_OFFSET,
    SB_B_OFFSET, Superblock,
};

use crate::StoreError;
use crate::aligned::AlignedBuf;
use crate::checkpoint::{CkptMeta, CkptSlot};
use crate::engine;
use crate::store::{DEFAULT_QUEUE_DEPTH, ExtentMeta, ExtentStore};

impl ExtentStore {
    /// Open an existing device (format.md §5): validate both superblocks
    /// (magic, version, CRC), pick the valid copy with the highest `sync_seq`,
    /// then rebuild the in-memory index — from the newest usable checkpoint
    /// plus a scan of the log tail it does not cover, or a full scan from
    /// `DATA_START` when no checkpoint is usable.
    ///
    /// The scan is bounded by the superblock tail (only synced state) and
    /// stops at the first record with a bad magic, bad header CRC, or invalid
    /// `disk_len`; the tail is salvaged to that record's start and everything
    /// past it is treated as an unconfirmed write and dropped. v1 devices are
    /// rejected with [`FormatError::UnsupportedVersion`].
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let (file, direct) = engine::open_device(path.as_ref(), false)?;
        let device_size = file.metadata()?.len() & !(BLOCK_SIZE - 1);
        let mut block = AlignedBuf::zeroed(BLOCK_SIZE as usize)?;
        let mut best: Option<Superblock> = None;
        let mut version_seen: Option<u32> = None;
        for offset in [SB_A_OFFSET, SB_B_OFFSET] {
            let candidate = file
                .read_exact_at(&mut block[..], offset)
                .ok()
                .and_then(|()| Superblock::from_bytes(&block[..]).ok())
                .and_then(|sb| {
                    sb.validate()
                        .map(|()| sb)
                        .map_err(|e| {
                            if let FormatError::UnsupportedVersion(v) = e {
                                version_seen = Some(v);
                            }
                        })
                        .ok()
                });
            match (candidate, &best) {
                (Some(sb), None) => best = Some(sb),
                (Some(sb), Some(prev)) if sb.sync_seq > prev.sync_seq => best = Some(sb),
                _ => {}
            }
        }
        let sb = match (best, version_seen) {
            (Some(sb), _) => sb,
            (None, Some(v)) => return Err(FormatError::UnsupportedVersion(v).into()),
            (None, None) => return Err(StoreError::NoValidSuperblock),
        };
        let mut store = Self {
            file,
            device_size,
            tail: DATA_START,
            next_extent_id: 0,
            index: HashMap::new(),
            extent_count: 0,
            live_bytes: 0,
            sync_seq: sb.sync_seq,
            uuid: sb.uuid,
            created_at: sb.created_at,
            last_sync_at: sb.last_sync_at,
            confirmed_tail: DATA_START,
            confirmed_count: 0,
            confirmed_next_id: 0,
            commit_max_pending: 0,
            pending_appends: 0,
            pending_tombs: HashSet::new(),
            syncs_since_ckpt: 0,
            ckpt_slot_cap: CKPT_SLOT_CAP,
            last_ckpt: None,
            mount_used_ckpt: false,
            queue_depth: DEFAULT_QUEUE_DEPTH,
            direct,
            ring: engine::new_ring(direct, DEFAULT_QUEUE_DEPTH),
            pool: Vec::new(),
            pool_buf_len: 0,
            pool_dirty: false,
        };
        // Newest valid checkpoint wins the fast path; generation is tracked
        // even when the slot is unusable so the next checkpoint stays monotonic.
        let ck_a = store.read_checkpoint(CkptSlot::A)?;
        let ck_b = store.read_checkpoint(CkptSlot::B)?;
        let newest = match (&ck_a, &ck_b) {
            (Some(a), Some(b)) => {
                if a.0.slot_gen >= b.0.slot_gen {
                    Some((CkptSlot::A, a))
                } else {
                    Some((CkptSlot::B, b))
                }
            }
            (Some(a), None) => Some((CkptSlot::A, a)),
            (None, Some(b)) => Some((CkptSlot::B, b)),
            (None, None) => None,
        };
        let mut scan_start = DATA_START;
        if let Some((slot, (header, entries))) = newest {
            store.last_ckpt = Some(CkptMeta {
                slot_gen: header.slot_gen,
                covered_tail: header.covered_tail,
                entry_count: header.entry_count,
                slot,
            });
            if store.checkpoint_usable(header, sb.tail) {
                store.load_checkpoint(entries);
                store.next_extent_id = header.next_extent_id;
                scan_start = header.covered_tail;
                store.mount_used_ckpt = true;
            }
        }
        store.scan_from(scan_start, sb.tail)?;
        // Everything found by the bounded scan is confirmed by definition: the
        // superblock tail only ever advances past fdatasynced records, and the
        // salvaged tail never exceeds it. The durability horizon therefore
        // covers every id seen during recovery.
        store.confirmed_tail = store.tail;
        store.confirmed_count = store.extent_count;
        store.confirmed_next_id = store.next_extent_id;
        // Mark the filesystem as mounted: clear the clean-unmount flag on disk.
        store.write_superblocks(false)?;
        Ok(store)
    }

    /// Rebuild the in-memory index by scanning the log in `[start, limit)`,
    /// where `limit` is the superblock's confirmed tail (only synced state is
    /// ever scanned). Sets `tail` to the end of the last valid record
    /// (salvage truncation at the first bad record). The index is NOT cleared:
    /// a checkpoint fast-mount pre-loads it before scanning the tail region.
    pub(crate) fn scan_from(&mut self, start: u64, limit: u64) -> Result<(), StoreError> {
        let limit = limit.min(self.device_size);
        let mut block = AlignedBuf::zeroed(BLOCK_SIZE as usize)?;
        let mut offset = start;
        while offset + BLOCK_SIZE <= limit {
            self.file.read_exact_at(&mut block[..], offset)?;
            let header = match ExtentHeader::from_bytes(&block[..ExtentHeader::LEN])
                .and_then(|h| h.validate().map(|()| h))
            {
                Ok(h) => h,
                Err(_) => break, // first bad record: salvage-stop
            };
            let disk_len = u64::from(header.disk_len);
            if disk_len < BLOCK_SIZE
                || disk_len % BLOCK_SIZE != 0
                || u64::from(header.data_len) > EXTENT_DATA_MAX
                || u64::from(header.data_len) + ExtentHeader::LEN as u64 > disk_len
                || offset + disk_len > limit
                || header.extent_id == u64::MAX
            {
                break;
            }
            self.next_extent_id = self.next_extent_id.max(header.extent_id + 1);
            let meta = ExtentMeta {
                offset,
                disk_len: header.disk_len,
                data_len: header.data_len,
                inode: header.inode,
                logical_offset: header.logical_offset,
                tombstoned: header.is_tombstone(),
            };
            if header.is_tombstone() {
                match self.index.get_mut(&header.extent_id) {
                    Some(entry) => entry.tombstoned = true,
                    None => {
                        self.index.insert(header.extent_id, meta);
                    }
                }
            } else {
                if self.index.contains_key(&header.extent_id) {
                    break; // duplicate live id: treat as corruption, salvage-stop
                }
                self.index.insert(header.extent_id, meta);
            }
            offset += disk_len;
        }
        self.tail = offset;
        self.extent_count = 0;
        self.live_bytes = 0;
        for meta in self.index.values() {
            if !meta.tombstoned {
                self.extent_count += 1;
                self.live_bytes += u64::from(meta.data_len);
            }
        }
        Ok(())
    }
}
