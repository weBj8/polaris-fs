//! 目录项磁盘格式与单块解析。
//!
//! v0.1：固定 256 字节的 DirEntry，4KB 块容纳 16 项。`ino == 0` 表示空槽，可被复用。
//! 大目录走 DoM 布局在 SCM 上连续块。不做 hash / B-tree——单节点 v0.1 直线性扫描已够用；
//! v0.2 上 V-Tree 时再换宽扇出结构。

use bytemuck::{Pod, Zeroable};

pub const DIRENTRY_SIZE: usize = 256;
pub const DIRENTRIES_PER_BLOCK: usize = 4096 / DIRENTRY_SIZE; // 16
pub const MAX_NAME_LEN: usize = 244;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct DirEntry {
    pub ino: u64,
    pub name_len: u16,
    pub _pad: u16,
    pub name: [u8; MAX_NAME_LEN],
}

impl DirEntry {
    pub fn empty() -> Self {
        Self {
            ino: 0,
            name_len: 0,
            _pad: 0,
            name: [0; MAX_NAME_LEN],
        }
    }

    pub fn name_str(&self) -> Option<&str> {
        if self.name_len == 0 {
            return None;
        }
        let len = self.name_len as usize;
        std::str::from_utf8(&self.name[..len]).ok()
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), String> {
        let bytes = name.as_bytes();
        if bytes.len() > MAX_NAME_LEN {
            return Err(format!("name too long: {} > {}", bytes.len(), MAX_NAME_LEN));
        }
        if bytes.iter().any(|b| *b == 0 || *b == b'/') {
            return Err("name contains NUL or '/'".into());
        }
        self.name = [0; MAX_NAME_LEN];
        self.name[..bytes.len()].copy_from_slice(bytes);
        self.name_len = bytes.len() as u16;
        Ok(())
    }
}

/// 单块目录视图：把 4KB 缓冲区当作 16 个 DirEntry 的数组。
pub struct DirBlock<'a> {
    buf: &'a mut [u8],
}

impl<'a> DirBlock<'a> {
    pub fn from_buf(buf: &'a mut [u8]) -> Self {
        assert_eq!(buf.len(), 4096);
        Self { buf }
    }

    pub fn entries(&self) -> impl Iterator<Item = (usize, DirEntry)> {
        (0..DIRENTRIES_PER_BLOCK).map(|i| {
            let off = i * DIRENTRY_SIZE;
            let entry: DirEntry = bytemuck::pod_read_unaligned(&self.buf[off..off + DIRENTRY_SIZE]);
            (i, entry)
        })
    }

    pub fn get(&self, slot: usize) -> DirEntry {
        let off = slot * DIRENTRY_SIZE;
        bytemuck::pod_read_unaligned(&self.buf[off..off + DIRENTRY_SIZE])
    }

    pub fn put(&mut self, slot: usize, entry: &DirEntry) {
        let off = slot * DIRENTRY_SIZE;
        self.buf[off..off + DIRENTRY_SIZE].copy_from_slice(bytemuck::bytes_of(entry));
    }

    /// 找空槽（ino==0），返回 Some(slot)。
    pub fn find_free(&self) -> Option<usize> {
        for i in 0..DIRENTRIES_PER_BLOCK {
            let entry = self.get(i);
            if entry.ino == 0 {
                return Some(i);
            }
        }
        None
    }

    pub fn find_by_name(&self, name: &str) -> Option<usize> {
        for i in 0..DIRENTRIES_PER_BLOCK {
            let entry = self.get(i);
            if entry.ino != 0 && entry.name_str() == Some(name) {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_roundtrip() {
        let mut e = DirEntry::empty();
        e.set_name("hello.txt").unwrap();
        assert_eq!(e.name_str(), Some("hello.txt"));
        assert!(e.set_name(&"x".repeat(245)).is_err());
        assert!(e.set_name("with\0null").is_err());
        assert!(e.set_name("with/slash").is_err());
    }

    #[test]
    fn block_find() {
        let mut buf = [0u8; 4096];
        let mut block = DirBlock::from_buf(&mut buf);
        let mut e = DirEntry::empty();
        e.ino = 42;
        e.set_name("foo").unwrap();
        block.put(3, &e);
        assert_eq!(block.find_by_name("foo"), Some(3));
        assert!(block.find_free().is_some());
    }
}
