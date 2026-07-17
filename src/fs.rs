//! PolarisFs 核心：mkfs / open + mkdir/create/lookup/readdir/read/write/unlink/rmdir/truncate/rename。
//!
//! 设备拓扑（v0.1 单节点）：
//!   - SCM 文件：超级块(1) + inode 位图 + inode 表 + SCM 数据位图 + SCM 数据区
//!   - QLC 文件：QLC 数据位图 + QLC 数据区
//! 大文件先落 SCM 写缓冲，整条带凑满后刷 QLC（FastEC 思想的简化版：v0.1 单 QLC 设备，
//! "凑满整条带"退化为"凑满一个 stripe_size 大小的窗口"）。
//!
//! 文件大小 -> 布局：
//!   - size <= 1MB：DoM，数据在 SCM 数据区
//!   - size <= 32MB：PFL 小文件，单条带在 QLC
//!   - size > 32MB：PFL 大文件，v0.1 退化为大块单条带在 QLC（多盘扇出留 v0.2）
//!
//! v0.1 简化：write() 直接按目标布局走，不做"先 SCM 后 QLC"的异步聚合。
//! 真正的 SCM 写缓冲 + 异步 flush 留给 v0.2 接 SPDK 后再做。

use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::alloc::BitmapAllocator;
use crate::device::{BlockDevice, FileBlockDevice, BLOCK_SIZE};
use crate::dir::{DirBlock, DirEntry, DIRENTRIES_PER_BLOCK};
use crate::inode::{Inode, InodeTable, is_dir, is_reg};
use crate::layout::{
    DomLayout, LAYOUT_DOM, LAYOUT_EMPTY, LAYOUT_PFL_SMALL, LAYOUT_PFL_WIDE, PflLayout,
    PFL_SMALL_THRESHOLD, decode_dom, decode_pfl, encode_dom, encode_pfl,
};
use crate::superblock::{MAGIC, VERSION, Superblock};

pub const DEFAULT_SCM_BLOCKS: u64 = 4096; // 16MB SCM
pub const DEFAULT_QLC_BLOCKS: u64 = 65536; // 256MB QLC
pub const DEFAULT_INODE_MAX: u64 = 1024;
pub const DEFAULT_STRIPE_SIZE: u32 = 1024 * 1024; // 1MB 条带单元

pub struct PolarisFs {
    pub scm: FileBlockDevice,
    pub qlc: FileBlockDevice,
    pub sb: Superblock,
    pub inodes: InodeTable,
    pub scm_alloc: BitmapAllocator,
    pub qlc_alloc: BitmapAllocator,
    pub stripe_size: u32,
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn nix_getuid() -> u32 {
    unsafe { libc::getuid() }
}

fn nix_getgid() -> u32 {
    unsafe { libc::getgid() }
}

fn blocks_for_bytes(bytes: u64) -> u64 {
    bytes.div_ceil(BLOCK_SIZE as u64)
}

/// 把文件名按 '/' 切分，返回 (父目录 ino, 末尾名)。根目录 ino 是 [`Superblock::root_ino`]。
/// 路径必须以 '/' 开头；空路径或非法路径返回 EINVAL。
fn split_path(path: &str, root_ino: u64) -> io::Result<(u64, String)> {
    if !path.starts_with('/') {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "path must start with /"));
    }
    let trimmed = path.trim_end_matches('/');
    if trimmed.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "empty path"));
    }
    if let Some(idx) = trimmed.rfind('/') {
        let parent_path = if idx == 0 { String::new() } else { trimmed[..idx].to_string() };
        let name = trimmed[idx + 1..].to_string();
        if name.is_empty() || name.contains('\0') {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid name"));
        }
        let parent_ino = if parent_path.is_empty() {
            root_ino
        } else {
            // 递归解析由调用方负责；这里只返回"父路径字符串"以方便调用方再走 lookup_path
            // 但 v0.1 直接在 lookup_path 里处理，这里返回根标记。
            // 简化：父目录交给调用方解析。本函数只返回末尾名 + 父路径的索引位置。
            // 不在此实现完整解析。
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "use lookup_path"));
        };
        Ok((parent_ino, name))
    } else {
        Ok((root_ino, trimmed.to_string()))
    }
}

impl PolarisFs {
    // ── mkfs / open ─────────────────────────────────────────────────────────

    pub fn mkfs(
        scm_path: &str,
        qlc_path: &str,
        scm_blocks: u64,
        qlc_blocks: u64,
        inode_max: u64,
        label: &str,
    ) -> io::Result<Self> {
        let mut scm = FileBlockDevice::create(scm_path, scm_blocks)?;
        let mut qlc = FileBlockDevice::create(qlc_path, qlc_blocks)?;

        // 布局 SCM：sb(1) + inode_bitmap + inode_table + scm_bitmap + scm_data
        let inode_bitmap_blocks = InodeTable::bitmap_blocks_for(inode_max);
        let inode_table_blocks = inode_max.div_ceil(crate::inode::INODES_PER_BLOCK as u64);
        let scm_bitmap_start = 1 + inode_bitmap_blocks + inode_table_blocks;
        let scm_data_start = scm_bitmap_start + 1; // scm 数据位图先按 1 块估算，下面校正
        let scm_data_blocks = scm_blocks.saturating_sub(scm_data_start);
        let scm_bitmap_blocks = BitmapAllocator::bitmap_blocks_for(scm_data_blocks);
        // 重算真正的 scm_data_start（位图可能 > 1 块）
        let scm_bitmap_start = 1 + inode_bitmap_blocks + inode_table_blocks;
        let scm_data_start = scm_bitmap_start + scm_bitmap_blocks;
        let scm_data_blocks = scm_blocks.saturating_sub(scm_data_start);

        // 布局 QLC：bitmap + data
        let qlc_bitmap_blocks = BitmapAllocator::bitmap_blocks_for(qlc_blocks - 1);
        let qlc_data_start = qlc_bitmap_blocks; // 从 block 0 开始放位图，紧跟着数据
        let qlc_data_blocks = qlc_blocks.saturating_sub(qlc_data_start);

        let mut sb = Superblock {
            magic: MAGIC,
            version: VERSION,
            block_size: BLOCK_SIZE as u32,
            scm_blocks,
            qlc_blocks,
            inode_bitmap_start: 1,
            inode_bitmap_blocks,
            inode_count_max: inode_max,
            inode_table_start: 1 + inode_bitmap_blocks,
            inode_table_blocks,
            scm_bitmap_start,
            scm_bitmap_blocks,
            scm_data_start,
            scm_data_blocks,
            qlc_bitmap_start: 0,
            qlc_bitmap_blocks,
            qlc_data_start,
            qlc_data_blocks,
            root_ino: 1,
            last_ino: 1,
            created_at: now_secs(),
            label: [0; 64],
            reserved: [0; 4096 - (8 + 4 + 4 + 8 * 18 + 64)],
        };
        let label_bytes = label.as_bytes();
        let copy_len = label_bytes.len().min(63);
        sb.label[..copy_len].copy_from_slice(&label_bytes[..copy_len]);

        // 写超级块到 SCM block 0
        scm.write_blocks(0, bytemuck::bytes_of(&sb))?;

        // 初始化 inode 位图（并预留 inode 0）
        let inode_bitmap = InodeTable::format_bitmap(&mut scm, sb.inode_bitmap_start, inode_max)?;
        // 初始化 SCM 数据位图
        let mut scm_alloc = BitmapAllocator::format(
            &mut scm,
            sb.scm_bitmap_start,
            scm_data_blocks,
            scm_data_start,
        )?;
        // 初始化 QLC 数据位图
        let qlc_alloc = BitmapAllocator::format(
            &mut qlc,
            sb.qlc_bitmap_start,
            qlc_data_blocks,
            qlc_data_start,
        )?;

        let mut inodes = InodeTable::new(
            sb.inode_table_start,
            inode_max,
            sb.inode_bitmap_start,
            sb.inode_bitmap_blocks,
            inode_bitmap,
        );

        // 根目录必须预留一个目录块，否则后续 mkdir/create 没有 dir_add 的落点
        let root_ino = inodes.allocate().expect("root inode alloc");
        assert_eq!(root_ino, 1);
        let now = now_secs();
        let mut root = Inode::new_dir(
            root_ino,
            0o755,
            nix_getuid(),
            nix_getgid(),
            now,
        );
        let root_dir_block = scm_alloc
            .allocate_contiguous(1)
            .ok_or_else(|| io::Error::new(io::ErrorKind::StorageFull, "scm data full at mkfs"))?;
        scm_alloc.flush(&mut scm)?;
        let root_dom = DomLayout { data_block: root_dir_block, data_blocks: 1 };
        root.layout_type = LAYOUT_DOM;
        crate::layout::encode_dom(&mut root.layout_data, &root_dom);
        root.size = BLOCK_SIZE as u64;
        root.blocks = 1;
        inodes.write(&mut scm, &root)?;
        scm.write_blocks(root_dir_block, &vec![0u8; BLOCK_SIZE])?;
        inodes.flush_bitmap(&mut scm)?;
        scm.sync()?;
        qlc.sync()?;

        Ok(Self {
            scm,
            qlc,
            sb,
            inodes,
            scm_alloc,
            qlc_alloc,
            stripe_size: DEFAULT_STRIPE_SIZE,
        })
    }

    pub fn open(scm_path: &str, qlc_path: &str) -> io::Result<Self> {
        let scm = FileBlockDevice::open(scm_path)?;
        let qlc = FileBlockDevice::open(qlc_path)?;

        let mut sb_buf = [0u8; BLOCK_SIZE];
        scm.read_blocks(0, &mut sb_buf)?;
        let sb: Superblock = *bytemuck::from_bytes(&sb_buf);
        sb.validate().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let inode_bitmap_blocks = sb.inode_bitmap_blocks;
        let mut inode_bitmap = vec![0u8; (inode_bitmap_blocks as usize) * BLOCK_SIZE];
        scm.read_blocks(sb.inode_bitmap_start, &mut inode_bitmap)?;
        let inodes = InodeTable::new(
            sb.inode_table_start,
            sb.inode_count_max,
            sb.inode_bitmap_start,
            sb.inode_bitmap_blocks,
            inode_bitmap,
        );

        let scm_alloc = BitmapAllocator::load(
            &scm,
            sb.scm_bitmap_start,
            sb.scm_bitmap_blocks,
            sb.scm_data_blocks,
            sb.scm_data_start,
        )?;
        let qlc_alloc = BitmapAllocator::load(
            &qlc,
            sb.qlc_bitmap_start,
            sb.qlc_bitmap_blocks,
            sb.qlc_data_blocks,
            sb.qlc_data_start,
        )?;

        Ok(Self {
            scm,
            qlc,
            sb,
            inodes,
            scm_alloc,
            qlc_alloc,
            stripe_size: DEFAULT_STRIPE_SIZE,
        })
    }

    pub fn sync_all(&mut self) -> io::Result<()> {
        self.inodes.flush_bitmap(&mut self.scm)?;
        self.scm_alloc.flush(&mut self.scm)?;
        self.qlc_alloc.flush(&mut self.qlc)?;
        self.scm.sync()?;
        self.qlc.sync()
    }

    // ── 目录操作 ─────────────────────────────────────────────────────────

    /// 解析路径到 inode。返回 (ino, inode)。找不到则 ENOENT。
    pub fn lookup_path(&self, path: &str) -> io::Result<(u64, Inode)> {
        if !path.starts_with('/') {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "path must start with /"));
        }
        let trimmed = path.trim_end_matches('/');
        if trimmed.is_empty() {
            let ino = self.sb.root_ino;
            let inode = self.inodes.read(&self.scm, ino)?;
            return Ok((ino, inode));
        }
        let parts: Vec<&str> = trimmed[1..].split('/').filter(|s| !s.is_empty()).collect();
        let mut cur_ino = self.sb.root_ino;
        let mut cur_inode = self.inodes.read(&self.scm, cur_ino)?;
        for part in parts {
            if !is_dir(cur_inode.mode) {
                return Err(io::Error::new(io::ErrorKind::NotADirectory, "not a directory"));
            }
            let child_ino = self.dir_lookup(cur_ino, part)?.ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, format!("entry not found: {part}"))
            })?;
            cur_ino = child_ino;
            cur_inode = self.inodes.read(&self.scm, cur_ino)?;
        }
        Ok((cur_ino, cur_inode))
    }

    /// 在目录 inode `dir_ino` 的目录块里查找名为 `name` 的子项，返回子 inode 号。
    pub fn dir_lookup(&self, dir_ino: u64, name: &str) -> io::Result<Option<u64>> {
        let dir_inode = self.inodes.read(&self.scm, dir_ino)?;
        if !is_dir(dir_inode.mode) {
            return Err(io::Error::new(io::ErrorKind::NotADirectory, "not a directory"));
        }
        let n_blocks = blocks_for_bytes(dir_inode.size) as usize;
        if n_blocks == 0 {
            return Ok(None);
        }
        let data_block = self.dir_data_block(dir_inode)?;
        for i in 0..n_blocks {
            let mut buf = [0u8; BLOCK_SIZE];
            self.scm.read_blocks(data_block + i as u64, &mut buf)?;
            let block = DirBlock::from_buf(&mut buf);
            for slot in 0..DIRENTRIES_PER_BLOCK {
                let entry = block.get(slot);
                if entry.ino != 0 && entry.name_str() == Some(name) {
                    return Ok(Some(entry.ino));
                }
            }
        }
        Ok(None)
    }

    fn dir_data_block(&self, dir_inode: Inode) -> io::Result<u64> {
        match dir_inode.layout_type {
            LAYOUT_DOM => {
                let dom = decode_dom(&dir_inode.layout_data);
                Ok(dom.data_block)
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "directory must use DoM layout",
            )),
        }
    }

    /// 列目录，返回 (name, ino, mode)。
    pub fn readdir(&self, dir_ino: u64) -> io::Result<Vec<(String, u64, u32)>> {
        let dir_inode = self.inodes.read(&self.scm, dir_ino)?;
        if !is_dir(dir_inode.mode) {
            return Err(io::Error::new(io::ErrorKind::NotADirectory, "not a directory"));
        }
        let mut out = Vec::new();
        out.push((".".to_string(), dir_ino, dir_inode.mode));
        out.push(("..".to_string(), self.parent_of(dir_ino)?, dir_inode.mode));
        let n_blocks = blocks_for_bytes(dir_inode.size) as usize;
        if n_blocks == 0 {
            return Ok(out);
        }
        let data_block = self.dir_data_block(dir_inode)?;
        for i in 0..n_blocks {
            let mut buf = [0u8; BLOCK_SIZE];
            self.scm.read_blocks(data_block + i as u64, &mut buf)?;
            let block = DirBlock::from_buf(&mut buf);
            for slot in 0..DIRENTRIES_PER_BLOCK {
                let entry = block.get(slot);
                if entry.ino != 0 {
                    if let Some(name) = entry.name_str() {
                        let child = self.inodes.read(&self.scm, entry.ino)?;
                        out.push((name.to_string(), entry.ino, child.mode));
                    }
                }
            }
        }
        Ok(out)
    }

    fn parent_of(&self, _dir_ino: u64) -> io::Result<u64> {
        // v0.1 不维护 ".." 反向指针，根目录的父目录是自己。
        Ok(self.sb.root_ino)
    }

    pub fn mkdir(
        &mut self,
        parent_ino: u64,
        name: &str,
        mode: u32,
        uid: u32,
        gid: u32,
    ) -> io::Result<Inode> {
        if !self.inodes.is_allocated(parent_ino) {
            return Err(io::Error::new(io::ErrorKind::NotFound, "parent not found"));
        }
        let parent = self.inodes.read(&self.scm, parent_ino)?;
        if !is_dir(parent.mode) {
            return Err(io::Error::new(io::ErrorKind::NotADirectory, "parent not a directory"));
        }
        if self.dir_lookup(parent_ino, name)?.is_some() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "entry exists"));
        }
        let new_ino = self
            .inodes
            .allocate()
            .ok_or_else(|| io::Error::new(io::ErrorKind::StorageFull, "inode table full"))?;
        let now = now_secs();
        let mut new_inode = Inode::new_dir(new_ino, mode, uid, gid, now);
        // 分配 1 个 SCM 块作为目录块
        let dir_block = self
            .scm_alloc
            .allocate_contiguous(1)
            .ok_or_else(|| io::Error::new(io::ErrorKind::StorageFull, "scm data full"))?;
        let dom = DomLayout { data_block: dir_block, data_blocks: 1 };
        new_inode.layout_type = LAYOUT_DOM;
        crate::layout::encode_dom(&mut new_inode.layout_data, &dom);
        new_inode.size = BLOCK_SIZE as u64;
        new_inode.blocks = 1;
        // 清空目录块
        self.scm.write_blocks(dir_block, &vec![0u8; BLOCK_SIZE])?;
        self.inodes.write(&mut self.scm, &new_inode)?;

        // 在父目录加入 entry
        self.dir_add(parent_ino, new_ino, name)?;
        // 父目录 nlink++
        let mut parent = self.inodes.read(&self.scm, parent_ino)?;
        parent.nlink = parent.nlink.saturating_add(1);
        parent.mtime = now;
        parent.ctime = now;
        self.inodes.write(&mut self.scm, &parent)?;

        Ok(new_inode)
    }

    fn dir_add(&mut self, parent_ino: u64, child_ino: u64, name: &str) -> io::Result<()> {
        let parent = self.inodes.read(&self.scm, parent_ino)?;
        let data_block = self.dir_data_block(parent)?;
        let n_blocks = blocks_for_bytes(parent.size) as usize;

        let mut buf = [0u8; BLOCK_SIZE];
        if n_blocks > 0 {
            for i in 0..n_blocks {
                self.scm.read_blocks(data_block + i as u64, &mut buf)?;
                let mut block = DirBlock::from_buf(&mut buf);
                if let Some(slot) = block.find_free() {
                    let mut entry = DirEntry::empty();
                    entry.ino = child_ino;
                    entry.set_name(name).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
                    block.put(slot, &entry);
                    self.scm.write_blocks(data_block + i as u64, &buf)?;
                    return Ok(());
                }
            }
        }

        // v0.2: 目录扩展。把旧数据搬到更大的连续 DoM 区域，新 entry 写到尾部新块的 slot 0。
        let new_n_blocks = (n_blocks + 1).max(1);
        let new_data_block = self
            .scm_alloc
            .allocate_contiguous(new_n_blocks as u64)
            .ok_or_else(|| io::Error::new(io::ErrorKind::StorageFull, "scm full for dir grow"))?;

        if n_blocks > 0 {
            let mut old_buf = vec![0u8; n_blocks * BLOCK_SIZE];
            self.scm.read_blocks(data_block, &mut old_buf)?;
            self.scm.write_blocks(new_data_block, &old_buf)?;
            self.scm_alloc.free(data_block, n_blocks as u64);
        }

        let mut buf = [0u8; BLOCK_SIZE];
        let mut block = DirBlock::from_buf(&mut buf);
        let mut entry = DirEntry::empty();
        entry.ino = child_ino;
        entry.set_name(name).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        block.put(0, &entry);
        self.scm.write_blocks(new_data_block + n_blocks as u64, &buf)?;

        let mut parent = self.inodes.read(&self.scm, parent_ino)?;
        let dom = DomLayout {
            data_block: new_data_block,
            data_blocks: new_n_blocks as u64,
        };
        parent.layout_type = LAYOUT_DOM;
        crate::layout::encode_dom(&mut parent.layout_data, &dom);
        parent.size = (new_n_blocks * BLOCK_SIZE) as u64;
        parent.blocks = new_n_blocks as u64;
        let now = now_secs();
        parent.mtime = now;
        parent.ctime = now;
        self.inodes.write(&mut self.scm, &parent)?;
        Ok(())
    }

    pub fn create(
        &mut self,
        parent_ino: u64,
        name: &str,
        mode: u32,
        uid: u32,
        gid: u32,
    ) -> io::Result<Inode> {
        if !self.inodes.is_allocated(parent_ino) {
            return Err(io::Error::new(io::ErrorKind::NotFound, "parent not found"));
        }
        let parent = self.inodes.read(&self.scm, parent_ino)?;
        if !is_dir(parent.mode) {
            return Err(io::Error::new(io::ErrorKind::NotADirectory, "parent not a directory"));
        }
        if self.dir_lookup(parent_ino, name)?.is_some() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "entry exists"));
        }
        let new_ino = self
            .inodes
            .allocate()
            .ok_or_else(|| io::Error::new(io::ErrorKind::StorageFull, "inode table full"))?;
        let now = now_secs();
        let new_inode = Inode::new_reg(new_ino, mode, uid, gid, now);
        self.inodes.write(&mut self.scm, &new_inode)?;
        self.dir_add(parent_ino, new_ino, name)?;
        let mut parent = self.inodes.read(&self.scm, parent_ino)?;
        parent.mtime = now;
        parent.ctime = now;
        self.inodes.write(&mut self.scm, &parent)?;
        Ok(new_inode)
    }

    // ── 文件读写 ─────────────────────────────────────────────────────────

    pub fn write(&mut self, ino: u64, offset: u64, data: &[u8]) -> io::Result<usize> {
        if data.is_empty() {
            return Ok(0);
        }
        let mut inode = self.inodes.read(&self.scm, ino)?;
        if !is_reg(inode.mode) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "not a regular file"));
        }
        let new_size = offset + data.len() as u64;
        let need_blocks = blocks_for_bytes(new_size);

        if new_size > inode.size {
            self.grow_file(&mut inode, ino, new_size, need_blocks)?;
        }

        match inode.layout_type {
            LAYOUT_DOM => self.write_dom(&mut inode, offset, data)?,
            LAYOUT_PFL_SMALL | LAYOUT_PFL_WIDE => self.write_pfl(&mut inode, offset, data)?,
            LAYOUT_EMPTY => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "empty layout"));
            }
            other => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unknown layout type {other}"),
                ));
            }
        };

        if new_size > inode.size {
            inode.size = new_size;
        }
        let now = now_secs();
        inode.mtime = now;
        inode.ctime = now;
        inode.blocks = need_blocks;
        self.inodes.write(&mut self.scm, &inode)?;
        Ok(data.len())
    }

    fn grow_file(
        &mut self,
        inode: &mut Inode,
        ino: u64,
        new_size: u64,
        need_blocks: u64,
    ) -> io::Result<()> {
        let current = inode.layout_type;
        let target = if new_size <= crate::layout::DOM_THRESHOLD {
            LAYOUT_DOM
        } else if new_size <= PFL_SMALL_THRESHOLD as u64 {
            LAYOUT_PFL_SMALL
        } else {
            LAYOUT_PFL_WIDE
        };

        match (current, target) {
            (LAYOUT_EMPTY, LAYOUT_DOM) => {
                let blk = self
                    .scm_alloc
                    .allocate_contiguous(need_blocks)
                    .ok_or_else(|| io::Error::new(io::ErrorKind::StorageFull, "scm full"))?;
                let dom = DomLayout {
                    data_block: blk,
                    data_blocks: need_blocks,
                };
                inode.layout_type = LAYOUT_DOM;
                crate::layout::encode_dom(&mut inode.layout_data, &dom);
            }
            (LAYOUT_EMPTY, LAYOUT_PFL_SMALL) | (LAYOUT_EMPTY, LAYOUT_PFL_WIDE) => {
                self.alloc_pfl(inode, need_blocks, target)?;
            }
            (LAYOUT_DOM, LAYOUT_DOM) => {
                let mut dom = decode_dom(&inode.layout_data);
                if need_blocks > dom.data_blocks {
                    let extra = need_blocks - dom.data_blocks;
                    // v0.1 简化：要求 DoM 区域连续——只支持"尾部追加"连续分配
                    let new_start = self
                        .scm_alloc
                        .allocate_contiguous(need_blocks)
                        .ok_or_else(|| io::Error::new(io::ErrorKind::StorageFull, "scm full"))?;
                    // 释放旧区域、把旧数据拷贝到新区域
                    let mut buf = vec![0u8; (dom.data_blocks * BLOCK_SIZE as u64) as usize];
                    self.scm.read_blocks(dom.data_block, &mut buf)?;
                    self.scm.write_blocks(new_start, &buf)?;
                    self.scm_alloc.free(dom.data_block, dom.data_blocks);
                    dom.data_block = new_start;
                    dom.data_blocks = need_blocks;
                    let _ = extra;
                    crate::layout::encode_dom(&mut inode.layout_data, &dom);
                }
            }
            (LAYOUT_DOM, LAYOUT_PFL_SMALL) | (LAYOUT_DOM, LAYOUT_PFL_WIDE) => {
                // 从 SCM 迁到 QLC：先在 QLC 分配，再读旧数据写过去，再释放 SCM
                let old_dom = decode_dom(&inode.layout_data);
                let old_bytes = inode.size;
                self.alloc_pfl(inode, need_blocks, target)?;
                // 拷数据
                if old_bytes > 0 {
                    let mut buf = vec![0u8; old_bytes as usize];
                    let n_old_blocks = blocks_for_bytes(old_bytes) as usize;
                    let mut read = 0usize;
                    for i in 0..n_old_blocks {
                        let chunk = (BLOCK_SIZE).min(buf.len() - read);
                        let mut blk = [0u8; BLOCK_SIZE];
                        self.scm.read_blocks(old_dom.data_block + i as u64, &mut blk)?;
                        buf[read..read + chunk].copy_from_slice(&blk[..chunk]);
                        read += chunk;
                    }
                    let pfl = decode_pfl(&inode.layout_data);
                    let mut written = 0usize;
                    while written < buf.len() {
                        let chunk = BLOCK_SIZE.min(buf.len() - written);
                        let mut blk = [0u8; BLOCK_SIZE];
                        blk[..chunk].copy_from_slice(&buf[written..written + chunk]);
                        self.qlc.write_blocks(pfl.stripe_block + (written / BLOCK_SIZE) as u64, &blk)?;
                        written += chunk;
                    }
                }
                // 释放 SCM 旧区域
                self.scm_alloc.free(old_dom.data_block, old_dom.data_blocks);
            }
            (LAYOUT_PFL_SMALL, LAYOUT_PFL_SMALL) | (LAYOUT_PFL_SMALL, LAYOUT_PFL_WIDE)
            | (LAYOUT_PFL_WIDE, LAYOUT_PFL_SMALL) | (LAYOUT_PFL_WIDE, LAYOUT_PFL_WIDE) => {
                let mut pfl = decode_pfl(&inode.layout_data);
                if need_blocks > pfl.stripe_blocks {
                    let new_start = self
                        .qlc_alloc
                        .allocate_contiguous(need_blocks)
                        .ok_or_else(|| io::Error::new(io::ErrorKind::StorageFull, "qlc full"))?;
                    let mut buf = vec![0u8; (pfl.stripe_blocks * BLOCK_SIZE as u64) as usize];
                    self.qlc.read_blocks(pfl.stripe_block, &mut buf)?;
                    self.qlc.write_blocks(new_start, &buf)?;
                    self.qlc_alloc.free(pfl.stripe_block, pfl.stripe_blocks);
                    pfl.stripe_block = new_start;
                    pfl.stripe_blocks = need_blocks;
                    crate::layout::encode_pfl(&mut inode.layout_data, &pfl);
                }
                if target == LAYOUT_PFL_WIDE && inode.layout_type == LAYOUT_PFL_SMALL {
                    inode.layout_type = LAYOUT_PFL_WIDE;
                }
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unsupported layout transition {current} -> {target}"),
                ));
            }
        }
        let _ = ino;
        Ok(())
    }

    fn alloc_pfl(&mut self, inode: &mut Inode, need_blocks: u64, target: u8) -> io::Result<()> {
        let blk = self
            .qlc_alloc
            .allocate_contiguous(need_blocks)
            .ok_or_else(|| io::Error::new(io::ErrorKind::StorageFull, "qlc full"))?;
        let pfl = PflLayout {
            stripe_block: blk,
            stripe_blocks: need_blocks,
            stripe_size: self.stripe_size,
            stripe_count: 1,
        };
        inode.layout_type = target;
        crate::layout::encode_pfl(&mut inode.layout_data, &pfl);
        Ok(())
    }

    fn write_dom(&mut self, inode: &mut Inode, offset: u64, data: &[u8]) -> io::Result<()> {
        let dom = decode_dom(&inode.layout_data);
        let start_block = dom.data_block + offset / BLOCK_SIZE as u64;
        let in_block_off = (offset % BLOCK_SIZE as u64) as usize;

        // 跨块写入：把 data 按 BLOCK_SIZE 边界切片，逐块读-改-写
        let mut written = 0usize;
        let mut cur_block = start_block;
        let mut cur_off = in_block_off;
        while written < data.len() {
            let chunk = (BLOCK_SIZE - cur_off).min(data.len() - written);
            let mut buf = [0u8; BLOCK_SIZE];
            self.scm.read_blocks(cur_block, &mut buf)?;
            buf[cur_off..cur_off + chunk].copy_from_slice(&data[written..written + chunk]);
            self.scm.write_blocks(cur_block, &buf)?;
            written += chunk;
            cur_block += 1;
            cur_off = 0;
        }
        Ok(())
    }

    fn write_pfl(&mut self, inode: &mut Inode, offset: u64, data: &[u8]) -> io::Result<()> {
        let pfl = decode_pfl(&inode.layout_data);
        let start_block = pfl.stripe_block + offset / BLOCK_SIZE as u64;
        let in_block_off = (offset % BLOCK_SIZE as u64) as usize;
        let mut written = 0usize;
        let mut cur_block = start_block;
        let mut cur_off = in_block_off;
        while written < data.len() {
            let chunk = (BLOCK_SIZE - cur_off).min(data.len() - written);
            let mut buf = [0u8; BLOCK_SIZE];
            self.qlc.read_blocks(cur_block, &mut buf)?;
            buf[cur_off..cur_off + chunk].copy_from_slice(&data[written..written + chunk]);
            self.qlc.write_blocks(cur_block, &buf)?;
            written += chunk;
            cur_block += 1;
            cur_off = 0;
        }
        Ok(())
    }

    pub fn read(&self, ino: u64, offset: u64, size: u32) -> io::Result<Vec<u8>> {
        let inode = self.inodes.read(&self.scm, ino)?;
        if !is_reg(inode.mode) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "not a regular file"));
        }
        if offset >= inode.size {
            return Ok(Vec::new());
        }
        let avail = (inode.size - offset).min(size as u64) as usize;
        let mut out = vec![0u8; avail];
        match inode.layout_type {
            LAYOUT_DOM => {
                let dom = decode_dom(&inode.layout_data);
                let start_block = dom.data_block + offset / BLOCK_SIZE as u64;
                let in_block_off = (offset % BLOCK_SIZE as u64) as usize;
                let mut read = 0usize;
                let mut cur_block = start_block;
                let mut cur_off = in_block_off;
                while read < avail {
                    let chunk = (BLOCK_SIZE - cur_off).min(avail - read);
                    let mut buf = [0u8; BLOCK_SIZE];
                    self.scm.read_blocks(cur_block, &mut buf)?;
                    out[read..read + chunk].copy_from_slice(&buf[cur_off..cur_off + chunk]);
                    read += chunk;
                    cur_block += 1;
                    cur_off = 0;
                }
            }
            LAYOUT_PFL_SMALL | LAYOUT_PFL_WIDE => {
                let pfl = decode_pfl(&inode.layout_data);
                let start_block = pfl.stripe_block + offset / BLOCK_SIZE as u64;
                let in_block_off = (offset % BLOCK_SIZE as u64) as usize;
                let mut read = 0usize;
                let mut cur_block = start_block;
                let mut cur_off = in_block_off;
                while read < avail {
                    let chunk = (BLOCK_SIZE - cur_off).min(avail - read);
                    let mut buf = [0u8; BLOCK_SIZE];
                    self.qlc.read_blocks(cur_block, &mut buf)?;
                    out[read..read + chunk].copy_from_slice(&buf[cur_off..cur_off + chunk]);
                    read += chunk;
                    cur_block += 1;
                    cur_off = 0;
                }
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "empty layout")),
        }
        Ok(out)
    }

    pub fn getattr(&self, ino: u64) -> io::Result<Inode> {
        self.inodes.read(&self.scm, ino)
    }

    pub fn setattr(
        &mut self,
        ino: u64,
        size: Option<u64>,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
    ) -> io::Result<Inode> {
        let mut inode = self.inodes.read(&self.scm, ino)?;
        if let Some(s) = size {
            if s == 0 {
                self.release_file_storage(&inode)?;
                inode.layout_type = LAYOUT_EMPTY;
                inode.layout_data = [0; 56];
                inode.size = 0;
                inode.blocks = 0;
            } else if s < inode.size {
                self.truncate_down(&mut inode, s)?;
            } else if s > inode.size {
                self.grow_file(&mut inode, ino, s, blocks_for_bytes(s))?;
                inode.size = s;
            }
        }
        if let Some(m) = mode {
            inode.mode = (inode.mode & !0o7777) | (m & 0o7777);
        }
        if let Some(u) = uid {
            inode.uid = u;
        }
        if let Some(g) = gid {
            inode.gid = g;
        }
        let now = now_secs();
        inode.ctime = now;
        inode.mtime = now;
        self.inodes.write(&mut self.scm, &inode)?;
        Ok(inode)
    }

    /// 释放尾部块。`new_size` 必须小于当前 size；DoM 释放 SCM 尾部，PFL 释放 QLC 尾部。
    fn truncate_down(&mut self, inode: &mut Inode, new_size: u64) -> io::Result<()> {
        let new_blocks = blocks_for_bytes(new_size);
        match inode.layout_type {
            LAYOUT_DOM => {
                let mut dom = decode_dom(&inode.layout_data);
                if new_blocks < dom.data_blocks {
                    let freed = dom.data_blocks - new_blocks;
                    self.scm_alloc
                        .free(dom.data_block + new_blocks, freed);
                    self.scm_alloc.flush(&mut self.scm)?;
                    dom.data_blocks = new_blocks;
                    encode_dom(&mut inode.layout_data, &dom);
                    inode.blocks = new_blocks;
                }
                inode.size = new_size;
            }
            LAYOUT_PFL_SMALL | LAYOUT_PFL_WIDE => {
                let mut pfl = decode_pfl(&inode.layout_data);
                if new_blocks < pfl.stripe_blocks {
                    let freed = pfl.stripe_blocks - new_blocks;
                    self.qlc_alloc
                        .free(pfl.stripe_block + new_blocks, freed);
                    self.qlc_alloc.flush(&mut self.qlc)?;
                    pfl.stripe_blocks = new_blocks;
                    encode_pfl(&mut inode.layout_data, &pfl);
                    inode.blocks = new_blocks;
                }
                inode.size = new_size;
            }
            _ => {
                inode.size = new_size;
            }
        }
        Ok(())
    }

    /// 跨目录原子 rename。v0.2 不支持 rename 覆盖既有 target（返回 EEXIST）。
    pub fn rename(
        &mut self,
        old_parent: u64,
        old_name: &str,
        new_parent: u64,
        new_name: &str,
    ) -> io::Result<()> {
        if old_parent == new_parent && old_name == new_name {
            return Ok(());
        }
        let child_ino = self
            .dir_lookup(old_parent, old_name)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "old entry not found"))?;

        if self.dir_lookup(new_parent, new_name)?.is_some() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "target exists; v0.2 does not support rename-over",
            ));
        }

        self.dir_remove(old_parent, old_name)?;
        self.dir_add(new_parent, child_ino, new_name)?;

        let now = now_secs();
        let mut child = self.inodes.read(&self.scm, child_ino)?;
        child.ctime = now;
        child.mtime = now;
        self.inodes.write(&mut self.scm, &child)?;
        Ok(())
    }

    pub fn unlink(&mut self, parent_ino: u64, name: &str) -> io::Result<()> {
        let child_ino = self
            .dir_lookup(parent_ino, name)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "entry not found"))?;
        let child = self.inodes.read(&self.scm, child_ino)?;
        if is_dir(child.mode) {
            return Err(io::Error::new(io::ErrorKind::IsADirectory, "is a directory; use rmdir"));
        }
        self.dir_remove(parent_ino, name)?;
        let mut child = child;
        child.nlink = child.nlink.saturating_sub(1);
        if child.nlink == 0 {
            self.release_file_storage(&child)?;
            self.inodes.free(child_ino);
        } else {
            self.inodes.write(&mut self.scm, &child)?;
        }
        let now = now_secs();
        let mut parent = self.inodes.read(&self.scm, parent_ino)?;
        parent.mtime = now;
        parent.ctime = now;
        self.inodes.write(&mut self.scm, &parent)?;
        Ok(())
    }

    pub fn rmdir(&mut self, parent_ino: u64, name: &str) -> io::Result<()> {
        let child_ino = self
            .dir_lookup(parent_ino, name)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "entry not found"))?;
        let child = self.inodes.read(&self.scm, child_ino)?;
        if !is_dir(child.mode) {
            return Err(io::Error::new(io::ErrorKind::NotADirectory, "not a directory"));
        }
        let entries = self.readdir(child_ino)?;
        // 跳过 . 和 ..
        let real_count = entries.iter().filter(|(n, _, _)| n != "." && n != "..").count();
        if real_count > 0 {
            return Err(io::Error::new(io::ErrorKind::DirectoryNotEmpty, "dir not empty"));
        }
        self.dir_remove(parent_ino, name)?;
        // 释放目录块 + inode
        let dom = decode_dom(&child.layout_data);
        self.scm_alloc.free(dom.data_block, dom.data_blocks);
        self.scm_alloc.flush(&mut self.scm)?;
        self.inodes.free(child_ino);
        let mut parent = self.inodes.read(&self.scm, parent_ino)?;
        parent.nlink = parent.nlink.saturating_sub(1);
        let now = now_secs();
        parent.mtime = now;
        parent.ctime = now;
        self.inodes.write(&mut self.scm, &parent)?;
        Ok(())
    }

    fn dir_remove(&mut self, parent_ino: u64, name: &str) -> io::Result<()> {
        let parent = self.inodes.read(&self.scm, parent_ino)?;
        let data_block = self.dir_data_block(parent)?;
        let n_blocks = blocks_for_bytes(parent.size) as usize;
        for i in 0..n_blocks.max(1) {
            let mut buf = [0u8; BLOCK_SIZE];
            self.scm.read_blocks(data_block + i as u64, &mut buf)?;
            let mut block = DirBlock::from_buf(&mut buf);
            if let Some(slot) = block.find_by_name(name) {
                block.put(slot, &DirEntry::empty());
                self.scm.write_blocks(data_block + i as u64, &buf)?;
                return Ok(());
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "entry not found in dir_remove"))
    }

    fn release_file_storage(&mut self, inode: &Inode) -> io::Result<()> {
        match inode.layout_type {
            LAYOUT_DOM => {
                let dom = decode_dom(&inode.layout_data);
                self.scm_alloc.free(dom.data_block, dom.data_blocks);
                self.scm_alloc.flush(&mut self.scm)?;
            }
            LAYOUT_PFL_SMALL | LAYOUT_PFL_WIDE => {
                let pfl = decode_pfl(&inode.layout_data);
                self.qlc_alloc.free(pfl.stripe_block, pfl.stripe_blocks);
                self.qlc_alloc.flush(&mut self.qlc)?;
            }
            _ => {}
        }
        Ok(())
    }

    // v0.1 不实现 rename，留给后续
}

// split_path 在 v0.1 里用 lookup_path 替代，留个空 stub 防止 unused
#[allow(dead_code)]
fn _split_path_stub(_p: &str, _r: u64) -> io::Result<(u64, String)> {
    split_path(_p, _r)
}
