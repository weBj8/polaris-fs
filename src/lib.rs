//! PolarisFS v2.0 - single-node basic implementation.
//!
//! 这版实现聚焦设计文档里的"单节点可用"路径，跳过所有分布式机制：
//! RDMA / SPDK / NVMe-oF / EC / token / V-Tree / 异步聚合 / similarity 缩减 / QoS / scrub。
//! 保留的核心架构：
//!   - 两层文件模拟块设备：SCM（小、快、放元数据+小文件+写缓冲）和 QLC（大、慢、放大条带）
//!   - DoM：<=1MB 文件内联在 SCM 元数据区
//!   - PFL：<32MB 单条带；>=32MB 全宽条带跨多块 QLC 设备（v0.1 简化为单块 QLC 内的连续条带）
//!   - Inode 表 + 目录块都在 SCM
//!   - SCM 写缓冲：大文件先落 SCM，整条带凑满后同步刷到 QLC，再用 QLC 偏移更新 inode
//!
//! 跳过项以 `// TODO(v0.2)` 标注，方便后续接入 EC / token / 多节点。

pub mod alloc;
pub mod cli;
pub mod device;
pub mod dir;
pub mod fs;
pub mod fuse;
pub mod inode;
pub mod layout;
pub mod superblock;

pub use fs::PolarisFs;
