//! Log scanning with salvage (format.md §4): rebuild the in-memory index by
//! walking the extent log from `DATA_START`, stopping at the first bad record.

use porfs_format::{BLOCK_SIZE, DATA_START, EXTENT_DATA_MAX, ExtentHeader};

use crate::StoreError;
use crate::aligned::AlignedBuf;
use crate::store::{ExtentMeta, ExtentStore};
use std::os::unix::fs::FileExt;

impl ExtentStore {
    /// Rebuild the in-memory index by scanning the log; sets `tail` to the end
    /// of the last valid record (salvage truncation at the first bad record).
    pub(crate) fn scan(&mut self) -> Result<(), StoreError> {
        let mut block = AlignedBuf::zeroed(BLOCK_SIZE as usize)?;
        let mut offset = DATA_START;
        while offset + BLOCK_SIZE <= self.device_size {
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
                || offset + disk_len > self.device_size
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
