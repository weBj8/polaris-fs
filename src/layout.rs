//! 文件布局描述符。
//!
//! 设计文档 v2.0 三种布局：
//!   - DoM (Data-on-MDT)：≤1MB 文件随元数据放 SCM，免数据路径
//!   - PFL 小文件 (single-stripe)：<32MB 单条带
//!   - PFL 大文件 (wide-stripe)：≥32MB 全宽条带跨多盘 QLC
//!
//! v0.2 简化：
//!   - DoM：数据落在 SCM 数据区连续块
//!   - PFL 小文件：单条带直接放 QLC
//!   - PFL 大文件：单 QLC 设备下退化为大块单条带；多盘扇出留 v0.3

use bytemuck::{Pod, Zeroable};

pub const LAYOUT_EMPTY: u8 = 0;
pub const LAYOUT_DOM: u8 = 1;
pub const LAYOUT_PFL_SMALL: u8 = 2;
pub const LAYOUT_PFL_WIDE: u8 = 3;

/// `PendingFlush::state` 取值（v0.2 引入）。
pub const PF_NONE: u8 = 0;
pub const PF_BUFFERING: u8 = 1;
pub const PF_FLUSHED: u8 = 2;

pub const DOM_THRESHOLD: u64 = 1 * 1024 * 1024;
pub const PFL_SMALL_THRESHOLD: u32 = 32 * 1024 * 1024;

/// DoM 布局数据，存于 inode `layout_data` 前 16 字节。
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct DomLayout {
    pub data_block: u64,
    pub data_blocks: u64,
}

/// PFL 单条带布局，存于 inode `layout_data` 前 24 字节。
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct PflLayout {
    pub stripe_block: u64,
    pub stripe_blocks: u64,
    pub stripe_size: u32,
    pub stripe_count: u32,
}

/// SCM 写缓冲待聚合元数据（v0.2），存于 inode `pending_flush` 字段（128B）。
///
/// 字段语义随 `state` 变化：
/// - `PF_NONE`：所有字段无意义
/// - `PF_BUFFERING`：`scm_buf_*` 是 SCM 缓冲区位置；`target_total_blocks` 是文件最终需要的块数；
///   `flushed_qlc_block` 是 QLC 上目标 stripe 起始块（聚合时一次性连续分配）；
///   `flushed_blocks` 是已聚合到 QLC 的块数
/// - `PF_FLUSHED`：数据全部在 QLC，缓冲区可释放；`flushed_qlc_block` / `flushed_blocks`
///   描述 QLC 上最终布局
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct PendingFlush {
    pub scm_buf_block: u64,
    pub scm_buf_blocks: u64,
    pub flushed_qlc_block: u64,
    pub flushed_blocks: u64,
    pub target_total_blocks: u64,
    pub state: u8,
    pub _pad: [u8; 7],
    pub _reserved: [u8; 80],
}

pub fn decode_dom(layout_data: &[u8]) -> DomLayout {
    let mut buf = [0u8; 16];
    buf.copy_from_slice(&layout_data[..16]);
    bytemuck::cast(buf)
}

pub fn decode_pfl(layout_data: &[u8]) -> PflLayout {
    let mut buf = [0u8; 24];
    buf.copy_from_slice(&layout_data[..24]);
    bytemuck::cast(buf)
}

pub fn decode_pending_flush(pf: &[u8]) -> PendingFlush {
    let mut buf = [0u8; 128];
    buf.copy_from_slice(&pf[..128]);
    bytemuck::cast(buf)
}

pub fn encode_dom(layout_data: &mut [u8], dom: &DomLayout) {
    layout_data[..16].copy_from_slice(bytemuck::bytes_of(dom));
}

pub fn encode_pfl(layout_data: &mut [u8], pfl: &PflLayout) {
    layout_data[..24].copy_from_slice(bytemuck::bytes_of(pfl));
}

pub fn encode_pending_flush(pf: &mut [u8], pending: &PendingFlush) {
    pf[..128].copy_from_slice(bytemuck::bytes_of(pending));
}

pub fn choose_layout(size: u64) -> u8 {
    if size <= DOM_THRESHOLD {
        LAYOUT_DOM
    } else if size <= PFL_SMALL_THRESHOLD as u64 {
        LAYOUT_PFL_SMALL
    } else {
        LAYOUT_PFL_WIDE
    }
}
