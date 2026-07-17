//! The extent store itself: device create/open, superblock state management,
//! and the durability ordering mandated by `docs/format.md` (extent data
//! fdatasynced first, superblock copy B then copy A afterwards).

use std::collections::HashMap;
use std::fs::File;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use io_uring::IoUring;
use porfs_format::{
    BLOCK_SIZE, DATA_START, FORMAT_VERSION, SB_A_OFFSET, SB_B_OFFSET, SB_FLAG_CLEAN, SB_MAGIC,
    Superblock,
};

use crate::aligned::AlignedBuf;
use crate::engine::{self};
use crate::{ExtentId, StoreError};

/// Default io_uring queue depth.
pub const DEFAULT_QUEUE_DEPTH: u32 = 32;

/// In-memory index entry for one extent id.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ExtentMeta {
    pub(crate) offset: u64,
    pub(crate) disk_len: u32,
    pub(crate) data_len: u32,
    pub(crate) inode: u64,
    pub(crate) logical_offset: u64,
    pub(crate) tombstoned: bool,
}

/// Synchronous append-only extent store. See crate-level docs for the model.
pub struct ExtentStore {
    pub(crate) file: File,
    pub(crate) device_size: u64,
    /// Next append offset (in-memory; may lead the confirmed tail until sync).
    pub(crate) tail: u64,
    pub(crate) next_extent_id: u64,
    pub(crate) index: HashMap<ExtentId, ExtentMeta>,
    pub(crate) extent_count: u64,
    pub(crate) live_bytes: u64,
    pub(crate) sync_seq: u64,
    pub(crate) uuid: [u8; 16],
    pub(crate) created_at: u64,
    pub(crate) last_sync_at: u64,
    /// Values written into the superblock: only fdatasync-confirmed state.
    pub(crate) confirmed_tail: u64,
    pub(crate) confirmed_count: u64,
    pub(crate) queue_depth: u32,
    pub(crate) direct: bool,
    pub(crate) ring: Option<IoUring>,
    /// Idle buffer pool: one buffer per pipeline slot. Buffers are only checked
    /// out inside `drive`, and `drive` reaps every completion before returning,
    /// so the pool is always idle between batch calls and can be rebuilt freely.
    pub(crate) pool: Vec<AlignedBuf>,
    /// Byte length of each pooled buffer (largest disk_len seen so far).
    pub(crate) pool_buf_len: usize,
    /// Set when the queue depth changed and the pool must be rebuilt.
    pub(crate) pool_dirty: bool,
}

impl ExtentStore {
    /// Initialize a device with the v0 format: size it, terminate any stale
    /// log, and write both superblocks (sync_seq = 1, clean unmount flag set).
    ///
    /// `size_bytes` is rounded down to a 4KiB multiple. For regular files the
    /// device is (re)sized with `ftruncate`; other file types (e.g. block
    /// devices) must already be at least `size_bytes` large.
    pub fn create(path: impl AsRef<Path>, size_bytes: u64) -> Result<Self, StoreError> {
        let (file, direct) = engine::open_device(path.as_ref(), true)?;
        let min_size = DATA_START + BLOCK_SIZE;
        let mut device_size = size_bytes & !(BLOCK_SIZE - 1);
        if device_size < min_size {
            return Err(StoreError::InvalidDeviceSize {
                size: size_bytes,
                min: min_size,
            });
        }
        if file.set_len(device_size).is_err() {
            // Not a regular file (e.g. a block device): accept if already large enough.
            let current = file.metadata()?.len() & !(BLOCK_SIZE - 1);
            if current < device_size {
                return Err(StoreError::InvalidDeviceSize {
                    size: current,
                    min: device_size,
                });
            }
            device_size = current;
        }
        let now = unix_now();
        let store = Self {
            file,
            device_size,
            tail: DATA_START,
            next_extent_id: 0,
            index: HashMap::new(),
            extent_count: 0,
            live_bytes: 0,
            sync_seq: 1,
            uuid: uuid::Uuid::new_v4().into_bytes(),
            created_at: now,
            last_sync_at: now,
            confirmed_tail: DATA_START,
            confirmed_count: 0,
            queue_depth: DEFAULT_QUEUE_DEPTH,
            direct,
            ring: engine::new_ring(direct, DEFAULT_QUEUE_DEPTH),
            pool: Vec::new(),
            pool_buf_len: 0,
            pool_dirty: false,
        };
        // Zero the first block of the log region so any stale log terminates here.
        let block = AlignedBuf::zeroed(BLOCK_SIZE as usize)?;
        store.file.write_all_at(&block[..], DATA_START)?;
        store.write_superblocks(true)?;
        Ok(store)
    }

    /// Open an existing device: validate both superblocks (magic, version,
    /// CRC), pick the valid copy with the highest `sync_seq`, then scan the
    /// extent log from `DATA_START`, rebuilding the in-memory index.
    ///
    /// The scan stops at the first record with a bad magic, bad header CRC, or
    /// invalid `disk_len`; the tail is salvaged to that record's start and
    /// everything past it is treated as an unconfirmed write and dropped.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let (file, direct) = engine::open_device(path.as_ref(), false)?;
        let device_size = file.metadata()?.len() & !(BLOCK_SIZE - 1);
        let mut block = AlignedBuf::zeroed(BLOCK_SIZE as usize)?;
        let mut best: Option<Superblock> = None;
        for offset in [SB_A_OFFSET, SB_B_OFFSET] {
            let candidate = file
                .read_exact_at(&mut block[..], offset)
                .ok()
                .and_then(|()| Superblock::from_bytes(&block[..]).ok())
                .and_then(|sb| sb.validate().map(|()| sb).ok());
            match (candidate, &best) {
                (Some(sb), None) => best = Some(sb),
                (Some(sb), Some(prev)) if sb.sync_seq > prev.sync_seq => best = Some(sb),
                _ => {}
            }
        }
        let sb = best.ok_or(StoreError::NoValidSuperblock)?;
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
            queue_depth: DEFAULT_QUEUE_DEPTH,
            direct,
            ring: engine::new_ring(direct, DEFAULT_QUEUE_DEPTH),
            pool: Vec::new(),
            pool_buf_len: 0,
            pool_dirty: false,
        };
        store.scan()?;
        // Everything found by the scan is as confirmed as the device knows.
        store.confirmed_tail = store.tail;
        store.confirmed_count = store.extent_count;
        // Mark the filesystem as mounted: clear the clean-unmount flag on disk.
        store.write_superblocks(false)?;
        Ok(store)
    }

    /// Confirm all outstanding writes: fdatasync the data region, then write
    /// superblock copy B followed by copy A (authoritative), per format.md §2.
    /// `sync_seq` is bumped once per call.
    pub fn sync(&mut self) -> Result<(), StoreError> {
        // Extent data must reach durable storage before the superblock tail may
        // cover it (no inverted superblock -> unflushed extent pointers).
        self.file.sync_data()?;
        self.confirmed_tail = self.tail;
        self.confirmed_count = self.extent_count;
        self.sync_seq = self
            .sync_seq
            .checked_add(1)
            .ok_or(StoreError::Internal("sync_seq overflow"))?;
        self.last_sync_at = unix_now();
        self.write_superblocks(false)
    }

    /// Number of live extents (tombstones excluded).
    pub fn extent_count(&self) -> u64 {
        self.extent_count
    }

    /// Sum of `data_len` over all live extents.
    pub fn live_bytes(&self) -> u64 {
        self.live_bytes
    }

    /// Next append offset (includes not-yet-synced appends).
    pub fn tail(&self) -> u64 {
        self.tail
    }

    /// Device size in bytes (rounded down to a 4KiB multiple).
    pub fn device_size(&self) -> u64 {
        self.device_size
    }

    /// True if the device was opened with `O_DIRECT`; false on the buffered fallback.
    pub fn is_direct(&self) -> bool {
        self.direct
    }

    /// Current superblock sync counter.
    pub fn sync_seq(&self) -> u64 {
        self.sync_seq
    }

    /// Filesystem UUID.
    pub fn uuid(&self) -> [u8; 16] {
        self.uuid
    }

    /// Filesystem creation time, unix seconds.
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    /// Last sync time, unix seconds.
    pub fn last_sync_at(&self) -> u64 {
        self.last_sync_at
    }

    /// Configured io_uring queue depth.
    pub fn queue_depth(&self) -> u32 {
        self.queue_depth
    }

    /// Id the next append will receive.
    pub fn next_extent_id(&self) -> u64 {
        self.next_extent_id
    }

    /// Change the queue depth used by batch operations (recreates the ring,
    /// marks the buffer pool for rebuild on the next batch).
    pub fn set_queue_depth(&mut self, queue_depth: u32) {
        self.queue_depth = queue_depth.max(1);
        self.ring = engine::new_ring(self.direct, self.queue_depth);
        self.pool_dirty = true;
    }

    /// Serialize the current confirmed state and write copy B then copy A,
    /// fdatasyncing after each, so at least one valid copy always survives.
    pub(crate) fn write_superblocks(&self, clean: bool) -> Result<(), StoreError> {
        let mut sb = Superblock {
            magic: *SB_MAGIC,
            format_version: FORMAT_VERSION,
            flags: if clean { SB_FLAG_CLEAN } else { 0 },
            device_size: self.device_size,
            data_start: DATA_START,
            tail: self.confirmed_tail,
            extent_count: self.confirmed_count,
            sync_seq: self.sync_seq,
            uuid: self.uuid,
            created_at: self.created_at,
            last_sync_at: self.last_sync_at,
            reserved: [0; 4004],
            sb_crc32c: 0,
        };
        sb.seal();
        let mut buf = AlignedBuf::zeroed(BLOCK_SIZE as usize)?;
        buf[..Superblock::LEN].copy_from_slice(sb.as_bytes());
        self.file.write_all_at(&buf[..], SB_B_OFFSET)?;
        self.file.sync_data()?;
        self.file.write_all_at(&buf[..], SB_A_OFFSET)?;
        self.file.sync_data()?;
        Ok(())
    }
}

impl Drop for ExtentStore {
    /// Best-effort clean-unmount marker (format.md §2). Errors are ignored:
    /// destructors cannot fail, and v0 recovery never depends on the flag.
    fn drop(&mut self) {
        let _ = self.write_superblocks(true);
    }
}

/// Current unix time in seconds (0 if the clock is before the epoch).
fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}
