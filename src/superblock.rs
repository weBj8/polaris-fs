//! 磁盘超级块。固定写入 SCM 设备 block 0。
//!
//! 描述整个文件系统的静态布局参数。SCM / QLC 两个设备的分区边界都记在这里。
//! 单节点 v0.1 不做冗余；v0.2 可以把超级块也镜像到 QLC 起始块。

use bytemuck::{Pod, Zeroable};

pub const MAGIC: u64 = u64::from_le_bytes(*b"PORFS\0\0\0");
pub const VERSION: u32 = 2;

/// 超级块大小 = 1 块 = 4096 字节，但有效字段只占前 ~256 字节，剩余字节预留给后续版本扩展。
pub const SUPERBLOCK_BLOCKS: u64 = 1;

// 显式字段字节数：magic(8) + version(4) + block_size(4) + 18×u64(144) + label(64) = 224
const FIXED_FIELDS_BYTES: usize = 8 + 4 + 4 + 8 * 18 + 64;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Superblock {
    pub magic: u64,
    pub version: u32,
    pub block_size: u32,
    pub scm_blocks: u64,
    pub qlc_blocks: u64,

    pub inode_bitmap_start: u64,
    pub inode_bitmap_blocks: u64,
    pub inode_count_max: u64,

    pub inode_table_start: u64,
    pub inode_table_blocks: u64,

    pub scm_bitmap_start: u64,
    pub scm_bitmap_blocks: u64,
    pub scm_data_start: u64,
    pub scm_data_blocks: u64,

    pub qlc_bitmap_start: u64,
    pub qlc_bitmap_blocks: u64,
    pub qlc_data_start: u64,
    pub qlc_data_blocks: u64,

    pub root_ino: u64,
    pub last_ino: u64,
    pub created_at: u64,
    pub label: [u8; 64],
    pub reserved: [u8; 4096 - FIXED_FIELDS_BYTES],
}

impl Superblock {
    /// 校验魔数与版本。
    pub fn validate(&self) -> Result<(), String> {
        if self.magic != MAGIC {
            return Err(format!("bad magic: 0x{:x}", self.magic));
        }
        if self.version != VERSION {
            return Err(format!("unsupported version: {}", self.version));
        }
        if self.block_size != crate::device::BLOCK_SIZE as u32 {
            return Err(format!("unexpected block_size: {}", self.block_size));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn superblock_fits_in_one_block() {
        assert_eq!(size_of::<Superblock>(), 4096);
    }
}
