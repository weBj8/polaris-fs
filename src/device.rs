//! 块设备抽象：用文件模拟 SCM（低延迟、小容量）和 QLC（大容量）两种块设备。
//!
//! `BlockDevice` 是 v0.1 的统一接口；后续要接 SPDK/NVMe-oF 只需新增实现。
//! 这里刻意不引入异步运行时：单节点 + 同步 IO 已足够覆盖 v0.1 的吞吐量目标，
//! 真要榨网卡是 v0.2 的事（按设计文档第 14 项用 Seastar 式 per-core reactor）。

use std::fs::{File, OpenOptions};
use std::os::unix::fs::FileExt;
use std::path::Path;

pub const BLOCK_SIZE: usize = 4096;

/// 块设备统一接口。所有 offset / count 都以 [`BLOCK_SIZE`] 为单位。
pub trait BlockDevice {
    fn num_blocks(&self) -> std::io::Result<u64>;

    /// 从 `block` 开始读 `buf.len() / BLOCK_SIZE` 块。`buf.len()` 必须是 BLOCK_SIZE 的整数倍。
    fn read_blocks(&self, block: u64, buf: &mut [u8]) -> std::io::Result<()>;

    /// 从 `block` 开始写 `buf.len() / BLOCK_SIZE` 块。`buf.len()` 必须是 BLOCK_SIZE 的整数倍。
    fn write_blocks(&mut self, block: u64, buf: &[u8]) -> std::io::Result<()>;

    /// 把脏数据刷到底层存储（fsync）。
    fn sync(&mut self) -> std::io::Result<()>;
}

/// 用一个普通文件做后端的块设备。文件大小固定，创建时按 `size_blocks * BLOCK_SIZE` 预分配。
pub struct FileBlockDevice {
    file: File,
    num_blocks: u64,
}

impl FileBlockDevice {
    /// 打开已存在的设备文件；大小由文件长度推断。
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = OpenOptions::new().read(true).write(true).open(path)?;
        let len = file.metadata()?.len();
        if len == 0 || len % BLOCK_SIZE as u64 != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("device file size {len} not aligned to {BLOCK_SIZE}"),
            ));
        }
        let num_blocks = len / BLOCK_SIZE as u64;
        Ok(Self { file, num_blocks })
    }

    /// 创建新的设备文件，预分配 `size_blocks` 个块。已存在则截断。
    pub fn create<P: AsRef<Path>>(path: P, size_blocks: u64) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let total = size_blocks
            .checked_mul(BLOCK_SIZE as u64)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "size overflow"))?;
        file.set_len(total)?;
        // 预分配空间：把稀疏文件按页落盘，避免后续写时的延迟派生和元数据抖动。
        // 这是块设备语义（mkfs 期望设备有稳定容量）的必要保证，跟 RAM 上的 Vec::with_capacity 同理。
        let zero = vec![0u8; BLOCK_SIZE * 256];
        let mut written: u64 = 0;
        while written < total {
            let chunk = (total - written).min(zero.len() as u64) as usize;
            let n = file.write_at(&zero[..chunk], written)?;
            if n == 0 {
                return Err(std::io::Error::new(std::io::ErrorKind::WriteZero, "zero-byte write"));
            }
            written += n as u64;
        }
        file.sync_all()?;
        Ok(Self {
            file,
            num_blocks: size_blocks,
        })
    }
}

impl BlockDevice for FileBlockDevice {
    fn num_blocks(&self) -> std::io::Result<u64> {
        Ok(self.num_blocks)
    }

    fn read_blocks(&self, block: u64, buf: &mut [u8]) -> std::io::Result<()> {
        if buf.is_empty() {
            return Ok(());
        }
        check_aligned(buf.len(), block, (buf.len() / BLOCK_SIZE) as u64, self.num_blocks)?;
        let offset = block * BLOCK_SIZE as u64;
        let mut read = 0usize;
        while read < buf.len() {
            let n = self.file.read_at(&mut buf[read..], offset + read as u64)?;
            if n == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "device file shorter than num_blocks",
                ));
            }
            read += n;
        }
        Ok(())
    }

    fn write_blocks(&mut self, block: u64, buf: &[u8]) -> std::io::Result<()> {
        if buf.is_empty() {
            return Ok(());
        }
        check_aligned(buf.len(), block, (buf.len() / BLOCK_SIZE) as u64, self.num_blocks)?;
        let offset = block * BLOCK_SIZE as u64;
        let mut written = 0usize;
        while written < buf.len() {
            let n = self.file.write_at(&buf[written..], offset + written as u64)?;
            if n == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "zero-byte write",
                ));
            }
            written += n;
        }
        Ok(())
    }

    fn sync(&mut self) -> std::io::Result<()> {
        self.file.sync_all()
    }
}

fn check_aligned(buf_len: usize, block: u64, n_blocks: u64, total: u64) -> std::io::Result<()> {
    if buf_len % BLOCK_SIZE != 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "buf length not block-aligned",
        ));
    }
    if block.saturating_add(n_blocks) > total {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("io out of range: block={block}, n={n_blocks}, total={total}"),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn roundtrip_blocks() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("dev.img");
        let mut dev = FileBlockDevice::create(&path, 16).unwrap();
        assert_eq!(dev.num_blocks().unwrap(), 16);

        let mut buf = vec![0u8; BLOCK_SIZE * 4];
        for (i, b) in buf.iter_mut().enumerate() {
            *b = (i % 251) as u8;
        }
        dev.write_blocks(2, &buf).unwrap();
        dev.sync().unwrap();

        let mut got = vec![0u8; BLOCK_SIZE * 4];
        dev.read_blocks(2, &mut got).unwrap();
        assert_eq!(buf, got);
    }

    #[test]
    fn out_of_range_rejected() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("dev.img");
        let mut dev = FileBlockDevice::create(&path, 4).unwrap();
        let buf = vec![0u8; BLOCK_SIZE];
        assert!(dev.write_blocks(4, &buf).is_err());
        assert!(dev.write_blocks(3, &buf).is_ok());
        let two = vec![0u8; BLOCK_SIZE * 2];
        assert!(dev.write_blocks(3, &two).is_err());
    }
}
