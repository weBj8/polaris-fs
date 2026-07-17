//! Inode 与 inode 表 / 位图管理。
//!
//! On-disk inode 固定 128 字节，4KB 块容纳 32 个 inode。inode 表布局在
//! superblock 里固定，启动时直接按偏移读写。
//! v0.1 用位图管理 inode 分配，跟块分配器思路一致；可以回收 inode 号避免长期累积耗尽。

use bytemuck::{Pod, Zeroable};
use std::io;

use crate::device::{BlockDevice, BLOCK_SIZE};

pub const S_IFMT: u32 = 0o170000;
pub const S_IFDIR: u32 = 0o040000;
pub const S_IFREG: u32 = 0o100000;
pub const S_IFLNK: u32 = 0o120000;

pub fn is_dir(mode: u32) -> bool {
    (mode & S_IFMT) == S_IFDIR
}
pub fn is_reg(mode: u32) -> bool {
    (mode & S_IFMT) == S_IFREG
}

pub const INODE_SIZE: usize = 256;
pub const INODES_PER_BLOCK: usize = BLOCK_SIZE / INODE_SIZE;

/// 256 字节 inode。前 72B 是基础元数据，56B 是 `layout_data`（描述 DoM/PFL 当前布局），
/// 后 128B 是 `pending_flush`（仅在文件处于"SCM 缓冲待聚合"状态时非零，由 flusher 线程更新）。
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Inode {
    pub ino: u64,
    pub mode: u32,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub blocks: u64,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
    pub layout_type: u8,
    pub _pad: [u8; 7],
    pub layout_data: [u8; 56],
    pub pending_flush: [u8; 128],
}

impl Inode {
    pub fn new_dir(ino: u64, mode: u32, uid: u32, gid: u32, now: u64) -> Self {
        Self {
            ino,
            mode: S_IFDIR | (mode & 0o7777),
            nlink: 2,
            uid,
            gid,
            size: 0,
            blocks: 0,
            atime: now,
            mtime: now,
            ctime: now,
            layout_type: crate::layout::LAYOUT_EMPTY,
            _pad: [0; 7],
            layout_data: [0; 56],
            pending_flush: [0; 128],
        }
    }

    pub fn new_reg(ino: u64, mode: u32, uid: u32, gid: u32, now: u64) -> Self {
        Self {
            ino,
            mode: S_IFREG | (mode & 0o7777),
            nlink: 1,
            uid,
            gid,
            size: 0,
            blocks: 0,
            atime: now,
            mtime: now,
            ctime: now,
            layout_type: crate::layout::LAYOUT_EMPTY,
            _pad: [0; 7],
            layout_data: [0; 56],
            pending_flush: [0; 128],
        }
    }
}

/// 读写 inode 的入口；inode 号从 1 开始（0 保留表示"无效 inode"）。
pub struct InodeTable {
    /// inode 表在 SCM 设备上的起始块。
    start_block: u64,
    /// 最大 inode 数。
    count_max: u64,
    /// inode 位图在 SCM 上的起始块。
    bitmap_start: u64,
    /// 位图占用块数。
    #[allow(dead_code)]
    bitmap_blocks: u64,
    /// 内存中的 inode 位图副本。
    bitmap: Vec<u8>,
}

impl InodeTable {
    pub fn new(
        start_block: u64,
        count_max: u64,
        bitmap_start: u64,
        bitmap_blocks: u64,
        bitmap: Vec<u8>,
    ) -> Self {
        Self {
            start_block,
            count_max,
            bitmap_start,
            bitmap_blocks,
            bitmap,
        }
    }

    pub fn bitmap_blocks_for(count_max: u64) -> u64 {
        let bits_per_block = (BLOCK_SIZE * 8) as u64;
        count_max.div_ceil(bits_per_block)
    }

    /// 在设备上格式化空白 inode 位图。
    pub fn format_bitmap(
        dev: &mut dyn BlockDevice,
        bitmap_start: u64,
        count_max: u64,
    ) -> io::Result<Vec<u8>> {
        let bitmap_blocks = Self::bitmap_blocks_for(count_max);
        let mut bitmap = vec![0u8; (bitmap_blocks as usize) * BLOCK_SIZE];
        // 尾部不存在的 inode bit 标记为已用。
        let total_bits = (bitmap.len() * 8) as u64;
        for i in count_max..total_bits {
            let byte = (i / 8) as usize;
            let bit = (i % 8) as u8;
            bitmap[byte] |= 1 << bit;
        }
        dev.write_blocks(bitmap_start, &bitmap)?;
        dev.sync()?;
        Ok(bitmap)
    }

    pub fn allocate(&mut self) -> Option<u64> {
        for i in 1..self.count_max {
            let byte = (i / 8) as usize;
            let bit = (i % 8) as u8;
            if self.bitmap[byte] >> bit & 1 == 0 {
                self.bitmap[byte] |= 1 << bit;
                return Some(i);
            }
        }
        None
    }

    pub fn free(&mut self, ino: u64) {
        if ino == 0 || ino >= self.count_max {
            return;
        }
        let byte = (ino / 8) as usize;
        let bit = (ino % 8) as u8;
        self.bitmap[byte] &= !(1 << bit);
    }

    pub fn is_allocated(&self, ino: u64) -> bool {
        if ino == 0 || ino >= self.count_max {
            return false;
        }
        let byte = (ino / 8) as usize;
        let bit = (ino % 8) as u8;
        self.bitmap[byte] >> bit & 1 == 1
    }

    /// 读取 inode `ino`。未分配的 inode 返回错误。
    pub fn read(&self, dev: &dyn BlockDevice, ino: u64) -> io::Result<Inode> {
        if ino == 0 || ino >= self.count_max {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "ino out of range"));
        }
        let idx = ino - 1;
        let block = self.start_block + idx / INODES_PER_BLOCK as u64;
        let off_in_block = (idx as usize % INODES_PER_BLOCK) * INODE_SIZE;
        let mut buf = [0u8; BLOCK_SIZE];
        dev.read_blocks(block, &mut buf)?;
        let inode: Inode = bytemuck::pod_read_unaligned(&buf[off_in_block..off_in_block + INODE_SIZE]);
        Ok(inode)
    }

    /// 写回 inode `ino`。
    pub fn write(&self, dev: &mut dyn BlockDevice, inode: &Inode) -> io::Result<()> {
        let ino = inode.ino;
        if ino == 0 || ino >= self.count_max {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "ino out of range"));
        }
        let idx = ino - 1;
        let block = self.start_block + idx / INODES_PER_BLOCK as u64;
        let off_in_block = (idx as usize % INODES_PER_BLOCK) * INODE_SIZE;
        let mut buf = [0u8; BLOCK_SIZE];
        dev.read_blocks(block, &mut buf)?;
        buf[off_in_block..off_in_block + INODE_SIZE].copy_from_slice(bytemuck::bytes_of(inode));
        dev.write_blocks(block, &buf)?;
        Ok(())
    }

    pub fn flush_bitmap(&mut self, dev: &mut dyn BlockDevice) -> io::Result<()> {
        dev.write_blocks(self.bitmap_start, &self.bitmap)?;
        dev.sync()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::FileBlockDevice;
    use tempfile::tempdir;

    #[test]
    fn inode_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("dev.img");
        let mut dev = FileBlockDevice::create(&path, 64).unwrap();
        let bm = InodeTable::format_bitmap(&mut dev, 1, 64).unwrap();
        let mut table = InodeTable::new(10, 64, 1, 1, bm);
        let ino = table.allocate().unwrap();
        assert!(ino >= 1);
        let now = 12345u64;
        let mut inode = Inode::new_dir(ino, 0o755, 0, 0, now);
        inode.size = 4096;
        table.write(&mut dev, &inode).unwrap();

        let got = table.read(&dev, ino).unwrap();
        assert_eq!(got.ino, ino);
        assert!(is_dir(got.mode));
        assert_eq!(got.size, 4096);

        table.free(ino);
    }
}
