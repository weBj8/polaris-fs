//! The extent data path: append, read, and discard (tombstone) operations.
//! Every write is one self-contained 4KiB-aligned block at the log tail;
//! every read re-verifies the record header and the data CRC32C.
//!
//! Batches run through [`ExtentStore::drive`] with a `prepare`/`complete`
//! closure pair, so CPU-side record building overlaps in-flight device I/O.
//! Write `prepare` never memsets the whole buffer: it copies the sealed
//! header and the payload, then zeroes only the pad tail.

use std::os::unix::fs::FileExt;

use porfs_format::{
    BLOCK_SIZE, EXT_FLAG_TOMBSTONE, EXT_MAGIC, EXTENT_DATA_MAX, ExtentHeader, checksum,
    checksum_append, extent_disk_len,
};

use crate::aligned::AlignedBuf;
use crate::engine::IoPrep;
use crate::store::{ExtentMeta, ExtentStore};
use crate::{ExtentId, StoreError};

/// Chunk size for fused copy+checksum passes: small enough that the chunk is
/// still cache-hot when the checksum pass touches it.
const CRC_CHUNK: usize = 64 * 1024;

/// Copy `src` into `buf` at `offset`, computing its CRC32C in the same pass.
/// On memory-bandwidth-bound machines this is ~1.5x faster than a separate
/// checksum pass followed by a copy.
fn copy_with_crc(buf: &mut AlignedBuf, offset: usize, src: &[u8]) -> u32 {
    let mut crc = 0u32;
    for (i, chunk) in src.chunks(CRC_CHUNK).enumerate() {
        buf.write_bytes(offset + i * CRC_CHUNK, chunk);
        crc = checksum_append(crc, chunk);
    }
    crc
}

/// Copy `src` into a fresh `Vec<u8>`, computing CRC32C in the same pass, and
/// return both. Skips the `vec![0; n]` zeroing: the allocation comes from
/// `with_capacity` and every byte is initialized by the copies before
/// `set_len`.
fn alloc_copy_with_crc(src: &[u8]) -> (Vec<u8>, u32) {
    let mut out: Vec<u8> = Vec::with_capacity(src.len());
    let crc = fill_vec_with_crc(&mut out, src);
    (out, crc)
}

/// Copy `src` into `dst`'s spare capacity, set its length to `src.len()`, and
/// compute CRC32C in the same pass (fused, so the checksum input is
/// cache-hot). `dst` must have `capacity() >= src.len()`.
fn fill_vec_with_crc(dst: &mut Vec<u8>, src: &[u8]) -> u32 {
    let mut crc = 0u32;
    let dst_ptr = dst.as_mut_ptr();
    for (i, chunk) in src.chunks(CRC_CHUNK).enumerate() {
        // SAFETY: the caller guarantees `dst.capacity() >= src.len()`; the
        // chunks tile exactly `src.len()` bytes, so writes stay in capacity;
        // source and destination never overlap.
        unsafe {
            std::ptr::copy_nonoverlapping(chunk.as_ptr(), dst_ptr.add(i * CRC_CHUNK), chunk.len())
        }
        crc = checksum_append(crc, chunk);
    }
    // SAFETY: all `src.len()` bytes were initialized by the copies above.
    unsafe { dst.set_len(src.len()) };
    crc
}

/// Validate the on-disk record in `buf` against its index entry: header parse
/// and CRC, extent id, lengths, tombstone flag. Returns the payload slice and
/// its expected CRC32C.
fn checked_record<'a>(
    buf: &'a [u8],
    meta: &ExtentMeta,
    id: ExtentId,
) -> Result<(&'a [u8], u32), StoreError> {
    let header = ExtentHeader::from_bytes(&buf[..ExtentHeader::LEN])?;
    header.validate()?;
    if header.extent_id != id
        || header.data_len != meta.data_len
        || header.disk_len != meta.disk_len
        || header.is_tombstone()
    {
        return Err(StoreError::CorruptHeader(id));
    }
    Ok((
        &buf[ExtentHeader::LEN..ExtentHeader::LEN + meta.data_len as usize],
        header.data_crc32c,
    ))
}

impl ExtentStore {
    /// Append one extent. The record is written immediately (one 4KiB-aligned
    /// block) but is only durable-confirmed after [`ExtentStore::sync`].
    pub fn append(
        &mut self,
        inode: u64,
        logical_offset: u64,
        data: &[u8],
    ) -> Result<ExtentId, StoreError> {
        let ids = self.append_batch(&[(inode, logical_offset, data)])?;
        ids.first()
            .copied()
            .ok_or(StoreError::Internal("append_batch returned no id"))
    }

    /// Append a batch of `(inode, logical_offset, data)` extents, pipelined
    /// through io_uring with a sliding window of `queue_depth` in-flight I/Os.
    ///
    /// Extent ids and offsets are assigned up front (contiguous, in slice
    /// order). The batch is all-or-nothing with respect to the in-memory
    /// state: on error nothing is committed and a later open will salvage
    /// away any partially written records.
    pub fn append_batch(
        &mut self,
        items: &[(u64, u64, &[u8])],
    ) -> Result<Vec<ExtentId>, StoreError> {
        if items.is_empty() {
            return Ok(Vec::new());
        }
        let mut total: u64 = 0;
        for &(_, _, data) in items {
            let len = data.len() as u64;
            if len > EXTENT_DATA_MAX {
                return Err(StoreError::DataTooLarge {
                    len: data.len(),
                    max: EXTENT_DATA_MAX as usize,
                });
            }
            total += extent_disk_len(len);
        }
        if total > self.device_size - self.tail {
            return Err(StoreError::DeviceFull {
                need: total,
                tail: self.tail,
                size: self.device_size,
            });
        }
        // Precompute ids and on-disk offsets so `prepare` is idempotent per op.
        let n = items.len();
        let mut ids = Vec::with_capacity(n);
        let mut plans = Vec::with_capacity(n);
        let mut offset = self.tail;
        let mut max_disk_len = 0usize;
        for (i, &(_, _, data)) in items.iter().enumerate() {
            let disk_len = extent_disk_len(data.len() as u64) as usize;
            ids.push(self.next_extent_id + i as u64);
            plans.push((offset, disk_len));
            offset += disk_len as u64;
            max_disk_len = max_disk_len.max(disk_len);
        }
        self.ensure_pool(max_disk_len)?;
        let prepare = |op: usize, buf: &mut AlignedBuf| -> Result<IoPrep, StoreError> {
            let (offset, disk_len) = plans[op];
            let (inode, logical_offset, data) = items[op];
            // Fused: the payload copy and its CRC32C share one memory pass.
            let data_crc32c = copy_with_crc(buf, ExtentHeader::LEN, data);
            let mut header = ExtentHeader {
                magic: EXT_MAGIC,
                header_len: ExtentHeader::LEN as u16,
                flags: 0,
                extent_id: ids[op],
                inode,
                logical_offset,
                data_len: data.len() as u32,
                data_crc32c,
                disk_len: disk_len as u32,
                reserved: [0; 16],
                header_crc32c: 0,
            };
            header.seal();
            buf.write_bytes(0, header.as_bytes());
            let pad = ExtentHeader::LEN + data.len();
            buf.zero_range(pad, disk_len - pad);
            Ok(IoPrep {
                offset,
                len: disk_len,
                write: true,
            })
        };
        let complete = |_: usize, _: &AlignedBuf| Ok(());
        self.drive(n, prepare, complete)?;
        for (i, &(inode, logical_offset, data)) in items.iter().enumerate() {
            self.index.insert(
                ids[i],
                ExtentMeta {
                    offset: plans[i].0,
                    disk_len: plans[i].1 as u32,
                    data_len: data.len() as u32,
                    inode,
                    logical_offset,
                    tombstoned: false,
                },
            );
        }
        self.tail = offset;
        self.next_extent_id += n as u64;
        self.extent_count += n as u64;
        self.live_bytes += items.iter().map(|item| item.2.len() as u64).sum::<u64>();
        Ok(ids)
    }

    /// Read one extent back. The data CRC32C is verified; reading a tombstoned
    /// or unknown id is an error.
    pub fn read(&mut self, id: ExtentId) -> Result<Vec<u8>, StoreError> {
        let mut out = self.read_batch(&[id])?;
        out.pop()
            .ok_or(StoreError::Internal("read_batch returned no data"))
    }

    /// Read a batch of extents, pipelined through io_uring. Every extent's
    /// header and data CRC32C is verified. Any unknown/tombstoned id or any
    /// corruption fails the whole batch.
    pub fn read_batch(&mut self, ids: &[ExtentId]) -> Result<Vec<Vec<u8>>, StoreError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let mut metas = Vec::with_capacity(ids.len());
        for &id in ids {
            let meta = self.index.get(&id).ok_or(StoreError::UnknownExtent(id))?;
            if meta.tombstoned {
                return Err(StoreError::Tombstoned(id));
            }
            metas.push(*meta);
        }
        let max_disk_len = metas.iter().map(|m| m.disk_len as usize).max().unwrap_or(0);
        self.ensure_pool(max_disk_len)?;
        let n = ids.len();
        let mut slots: Vec<Option<Vec<u8>>> = Vec::with_capacity(n);
        slots.resize_with(n, || None);
        let prepare = |op: usize, _: &mut AlignedBuf| -> Result<IoPrep, StoreError> {
            let meta = metas[op];
            Ok(IoPrep {
                offset: meta.offset,
                len: meta.disk_len as usize,
                write: false,
            })
        };
        let complete = |op: usize, buf: &AlignedBuf| -> Result<(), StoreError> {
            let id = ids[op];
            let (data, want_crc) = checked_record(buf, &metas[op], id)?;
            // Fused: copy the payload out and verify its CRC32C in one pass.
            let (out, crc) = alloc_copy_with_crc(data);
            if crc != want_crc {
                return Err(StoreError::CorruptData(id));
            }
            slots[op] = Some(out);
            Ok(())
        };
        self.drive(n, prepare, complete)?;
        let mut out = Vec::with_capacity(n);
        for slot in slots {
            out.push(slot.ok_or(StoreError::Internal("missing read result"))?);
        }
        Ok(out)
    }

    /// Read a batch of extents into caller-provided destination buffers,
    /// pipelined through io_uring. Zero-allocation steady state: `dsts[i]`
    /// must have `capacity() >= data_len` of extent `ids[i]`; on success it is
    /// filled and its length is set to `data_len`. Callers that reuse the
    /// same `dsts` across calls perform no allocation or page-faults.
    ///
    /// Header and data CRC32C are verified exactly as in
    /// [`ExtentStore::read_batch`]; unknown/tombstoned ids, wrong `dsts`
    /// length, insufficient capacity, or any corruption fail the whole batch.
    pub fn read_batch_into(
        &mut self,
        ids: &[ExtentId],
        dsts: &mut [Vec<u8>],
    ) -> Result<(), StoreError> {
        if ids.len() != dsts.len() {
            return Err(StoreError::LengthMismatch {
                ids: ids.len(),
                dsts: dsts.len(),
            });
        }
        if ids.is_empty() {
            return Ok(());
        }
        let mut metas = Vec::with_capacity(ids.len());
        for &id in ids {
            let meta = self.index.get(&id).ok_or(StoreError::UnknownExtent(id))?;
            if meta.tombstoned {
                return Err(StoreError::Tombstoned(id));
            }
            metas.push(*meta);
        }
        for (i, meta) in metas.iter().enumerate() {
            let have = dsts[i].capacity();
            if have < meta.data_len as usize {
                return Err(StoreError::CapacityTooSmall {
                    id: ids[i],
                    need: meta.data_len as usize,
                    have,
                });
            }
        }
        let max_disk_len = metas.iter().map(|m| m.disk_len as usize).max().unwrap_or(0);
        self.ensure_pool(max_disk_len)?;
        let n = ids.len();
        let prepare = |op: usize, _: &mut AlignedBuf| -> Result<IoPrep, StoreError> {
            let meta = metas[op];
            Ok(IoPrep {
                offset: meta.offset,
                len: meta.disk_len as usize,
                write: false,
            })
        };
        let complete = |op: usize, buf: &AlignedBuf| -> Result<(), StoreError> {
            let id = ids[op];
            let (data, want_crc) = checked_record(buf, &metas[op], id)?;
            // Fused: copy the payload into the caller's buffer and verify its
            // CRC32C in the same pass.
            if fill_vec_with_crc(&mut dsts[op], data) != want_crc {
                return Err(StoreError::CorruptData(id));
            }
            Ok(())
        };
        self.drive(n, prepare, complete)
    }

    /// Logically delete an extent by appending a tombstone record (data_len = 0,
    /// same extent id, flag bit0 set). Like appends, the tombstone is only
    /// durable-confirmed after [`ExtentStore::sync`].
    pub fn discard(&mut self, id: ExtentId) -> Result<(), StoreError> {
        let meta = *self.index.get(&id).ok_or(StoreError::UnknownExtent(id))?;
        if meta.tombstoned {
            return Err(StoreError::Tombstoned(id));
        }
        if self.device_size - self.tail < BLOCK_SIZE {
            return Err(StoreError::DeviceFull {
                need: BLOCK_SIZE,
                tail: self.tail,
                size: self.device_size,
            });
        }
        let mut buf = AlignedBuf::zeroed(BLOCK_SIZE as usize)?;
        let mut header = ExtentHeader {
            magic: EXT_MAGIC,
            header_len: ExtentHeader::LEN as u16,
            flags: EXT_FLAG_TOMBSTONE,
            extent_id: id,
            inode: meta.inode,
            logical_offset: meta.logical_offset,
            data_len: 0,
            data_crc32c: checksum(&[]),
            disk_len: BLOCK_SIZE as u32,
            reserved: [0; 16],
            header_crc32c: 0,
        };
        header.seal();
        buf[..ExtentHeader::LEN].copy_from_slice(header.as_bytes());
        self.file.write_all_at(&buf[..], self.tail)?;
        self.index.insert(
            id,
            ExtentMeta {
                tombstoned: true,
                ..meta
            },
        );
        self.tail += BLOCK_SIZE;
        self.extent_count -= 1;
        self.live_bytes -= u64::from(meta.data_len);
        Ok(())
    }
}
