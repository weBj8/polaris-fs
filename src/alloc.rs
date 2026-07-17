//! 位图块分配器。每个块设备 (SCM / QLC) 都挂一个独立的位图，管理自己 `[data_start, data_start + total)` 范围内的数据块。
//!
//! v0.1 设计选择：单进程、同步分配、整段连续分配（`allocate_contiguous`）。
//! PFL "宽条带" 在单节点 + 单 QLC 文件场景下退化为"在 QLC 内连续分配一块整条带区域"，
//! 跨物理盘的扇出留给 v0.2 接 NVMe-oF 后再做。
//!
//! 位图本身的存储位置由 [`BitmapAllocator::format`] / [`BitmapAllocator::load`] 调用方指定，
//! 跟 superblock / inode table 的布局解耦——上层 (superblock.rs) 负责算好各区域偏移。

use crate::device::{BlockDevice, BLOCK_SIZE};

pub struct BitmapAllocator {
    /// 内存中的位图副本，长度为 `bitmap_blocks * BLOCK_SIZE`。
    bitmap: Vec<u8>,
    /// 位图在设备上的起始块号。
    bitmap_start_block: u64,
    /// 位图占用的块数。
    #[allow(dead_code)]
    bitmap_blocks: u64,
    /// 管理的数据区起始块号（绝对块号）。
    data_start_block: u64,
    /// 管理的数据块总数。
    total_data_blocks: u64,
}

impl BitmapAllocator {
    /// 位图本身需要多少块才能覆盖 `total_data_blocks` 个数据块。
    pub fn bitmap_blocks_for(total_data_blocks: u64) -> u64 {
        let bits_per_block = (BLOCK_SIZE * 8) as u64;
        total_data_blocks.div_ceil(bits_per_block)
    }

    /// 在设备上初始化空白位图。已存在的位图会被清零（所有数据块标记为空闲），
    /// 然后把"超出 `total_data_blocks` 的尾部零碎 bit"标记为已用，避免误分配。
    pub fn format(
        dev: &mut dyn BlockDevice,
        bitmap_start_block: u64,
        total_data_blocks: u64,
        data_start_block: u64,
    ) -> std::io::Result<Self> {
        let bitmap_blocks = Self::bitmap_blocks_for(total_data_blocks);
        let mut bitmap = vec![0u8; (bitmap_blocks as usize) * BLOCK_SIZE];
        // 标记尾部不足一个完整位图的 bit 为已用。
        let total_bits = (bitmap.len() * 8) as u64;
        for i in total_data_blocks..total_bits {
            let byte = (i / 8) as usize;
            let bit = (i % 8) as u8;
            bitmap[byte] |= 1 << bit;
        }
        dev.write_blocks(
            bitmap_start_block,
            &bitmap,
        )?;
        dev.sync()?;
        Ok(Self {
            bitmap,
            bitmap_start_block,
            bitmap_blocks,
            data_start_block,
            total_data_blocks,
        })
    }

    /// 从设备读回已有位图。
    pub fn load(
        dev: &dyn BlockDevice,
        bitmap_start_block: u64,
        bitmap_blocks: u64,
        total_data_blocks: u64,
        data_start_block: u64,
    ) -> std::io::Result<Self> {
        let mut bitmap = vec![0u8; (bitmap_blocks as usize) * BLOCK_SIZE];
        dev.read_blocks(bitmap_start_block, &mut bitmap)?;
        Ok(Self {
            bitmap,
            bitmap_start_block,
            bitmap_blocks,
            data_start_block,
            total_data_blocks,
        })
    }

    /// 分配 `n` 个**连续**的块，返回起始块的**绝对块号**。找不到则返回 `None`。
    /// 调用方负责在写完数据后调用 [`flush`] 把位图落盘。
    pub fn allocate_contiguous(&mut self, n: u64) -> Option<u64> {
        if n == 0 {
            return Some(self.data_start_block);
        }
        if n > self.free_blocks() {
            return None;
        }
        // First-fit：找一段长度 >= n 的连续 0 bit。
        let mut run = 0u64;
        let mut start = 0u64;
        for i in 0..self.total_data_blocks {
            if !self.bit_get(i) {
                if run == 0 {
                    start = i;
                }
                run += 1;
                if run == n {
                    for k in start..start + n {
                        self.bit_set(k);
                    }
                    return Some(self.data_start_block + start);
                }
            } else {
                run = 0;
            }
        }
        None
    }

    /// 释放从 `start_abs`（绝对块号）开始的 `n` 个块。
    pub fn free(&mut self, start_abs: u64, n: u64) {
        if start_abs < self.data_start_block {
            return;
        }
        let start = start_abs - self.data_start_block;
        if start + n > self.total_data_blocks {
            return;
        }
        for k in start..start + n {
            self.bit_clear(k);
        }
    }

    /// 把内存中的位图刷回设备。
    pub fn flush(&mut self, dev: &mut dyn BlockDevice) -> std::io::Result<()> {
        dev.write_blocks(self.bitmap_start_block, &self.bitmap)?;
        dev.sync()
    }

    /// 当前可用块数。
    pub fn free_blocks(&self) -> u64 {
        let mut count = 0u64;
        for i in 0..self.total_data_blocks {
            if !self.bit_get(i) {
                count += 1;
            }
        }
        count
    }

    pub fn total_blocks(&self) -> u64 {
        self.total_data_blocks
    }

    fn bit_get(&self, idx: u64) -> bool {
        let byte = (idx / 8) as usize;
        let bit = (idx % 8) as u8;
        (self.bitmap[byte] >> bit) & 1 == 1
    }

    fn bit_set(&mut self, idx: u64) {
        let byte = (idx / 8) as usize;
        let bit = (idx % 8) as u8;
        self.bitmap[byte] |= 1 << bit;
    }

    fn bit_clear(&mut self, idx: u64) {
        let byte = (idx / 8) as usize;
        let bit = (idx % 8) as u8;
        self.bitmap[byte] &= !(1 << bit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::FileBlockDevice;
    use tempfile::tempdir;

    #[test]
    fn format_allocate_free_persists() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("dev.img");
        let mut dev = FileBlockDevice::create(&path, 256).unwrap();
        // 数据块从 block 10 开始，共 100 块，位图放在 block 1。
        let mut alloc = BitmapAllocator::format(&mut dev, 1, 100, 10).unwrap();
        assert_eq!(alloc.free_blocks(), 100);

        let a = alloc.allocate_contiguous(4).unwrap();
        assert_eq!(a, 10);
        assert_eq!(alloc.free_blocks(), 96);

        let b = alloc.allocate_contiguous(4).unwrap();
        assert_eq!(b, 14);
        assert_eq!(alloc.free_blocks(), 92);

        alloc.free(a, 4);
        assert_eq!(alloc.free_blocks(), 96);

        // 释放后 first-fit 应回到 10。
        let c = alloc.allocate_contiguous(4).unwrap();
        assert_eq!(c, 10);

        alloc.flush(&mut dev).unwrap();

        // 重新加载验证持久化。
        let mut reloaded = BitmapAllocator::load(&dev, 1, alloc.bitmap_blocks, 100, 10).unwrap();
        assert_eq!(reloaded.free_blocks(), 92);
        assert!(reloaded.allocate_contiguous(200).is_none());
    }
}
