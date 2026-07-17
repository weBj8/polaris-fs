//! 命令行入口：`porfs mkfs` / `porfs mount`。
//!
//! v0.1 用最朴素的两个子命令：
//!   - `mkfs --dir <dir>`：在 `<dir>/scm.img` 和 `<dir>/qlc.img` 上建文件系统。
//!   - `mount --dir <dir> --mountpoint <mnt>`：打开已有文件系统并挂载到 `<mnt>`。
//!
//! 设计上把两个"块设备文件"放在同一个 `--dir` 下，方便单节点演示。
//! 真要分盘就在 v0.2 加 `--scm <path> --qlc <path>` 参数。

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::fs::{
    DEFAULT_INODE_MAX, DEFAULT_QLC_BLOCKS, DEFAULT_SCM_BLOCKS,
};

#[derive(Parser, Debug)]
#[command(name = "porfs", version, about = "PolarisFS v0.1 single-node FUSE mount")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// 创建新文件系统（覆盖已有）
    Mkfs {
        #[arg(long)]
        dir: PathBuf,
        #[arg(long, default_value_t = DEFAULT_SCM_BLOCKS)]
        scm_blocks: u64,
        #[arg(long, default_value_t = DEFAULT_QLC_BLOCKS)]
        qlc_blocks: u64,
        #[arg(long, default_value_t = DEFAULT_INODE_MAX)]
        inode_max: u64,
        #[arg(long, default_value = "porfs-v0.1")]
        label: String,
    },
    /// 挂载文件系统
    Mount {
        #[arg(long)]
        dir: PathBuf,
        #[arg(long)]
        mountpoint: PathBuf,
        /// 允许非属主用户访问（需 /etc/fuse.conf 开启 user_allow_other）
        #[arg(long, default_value_t = false)]
        allow_other: bool,
        /// 是否开启内核写回缓存（v0.1 默认关，避免数据不一致风险）
        #[arg(long, default_value_t = false)]
        writeback: bool,
        /// 是否前台运行（默认是）
        #[arg(long, default_value_t = true)]
        foreground: bool,
    },
}

pub fn scm_path(dir: &std::path::Path) -> PathBuf {
    dir.join("scm.img")
}

pub fn qlc_path(dir: &std::path::Path) -> PathBuf {
    dir.join("qlc.img")
}
