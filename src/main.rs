//! PolarisFS v0.1 入口：解析命令行并分发到 mkfs / mount。

use std::process::ExitCode;

use clap::Parser;
use fuser::{Config, MountOption, SessionACL};
use porfs::cli::{Cli, Cmd};
use porfs::fs::PolarisFs;
use porfs::fuse::PolarisFuse;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("porfs: {e}");
            ExitCode::from(1)
        }
    }
}

fn run(cli: Cli) -> std::io::Result<()> {
    match cli.cmd {
        Cmd::Mkfs {
            dir,
            scm_blocks,
            qlc_blocks,
            inode_max,
            label,
        } => {
            std::fs::create_dir_all(&dir)?;
            let scm = porfs::cli::scm_path(&dir);
            let qlc = porfs::cli::qlc_path(&dir);
            let scm_s = scm.to_string_lossy().into_owned();
            let qlc_s = qlc.to_string_lossy().into_owned();
            let mut fs = PolarisFs::mkfs(&scm_s, &qlc_s, scm_blocks, qlc_blocks, inode_max, &label)?;
            fs.sync_all()?;
            eprintln!(
                "mkfs done: scm={} blocks, qlc={} blocks, inodes={}, label={:?}",
                fs.sb.scm_blocks, fs.sb.qlc_blocks, fs.sb.inode_count_max, label
            );
            Ok(())
        }
        Cmd::Mount {
            dir,
            mountpoint,
            allow_other,
            writeback,
            foreground,
        } => {
            let _ = foreground; // Session::new 默认前台；后台化留给 v0.2
            let scm = porfs::cli::scm_path(&dir);
            let qlc = porfs::cli::qlc_path(&dir);
            let scm_s = scm.to_string_lossy().into_owned();
            let qlc_s = qlc.to_string_lossy().into_owned();
            let fs = PolarisFs::open(&scm_s, &qlc_s)?;
            eprintln!(
                "opened: scm={} blocks, qlc={} blocks, root_ino={}",
                fs.sb.scm_blocks, fs.sb.qlc_blocks, fs.sb.root_ino
            );
            drop(fs);

            let fs = PolarisFs::open(&scm_s, &qlc_s)?;
            let fuse_impl = PolarisFuse::new(fs);

            let mut config = Config::default();
            config.mount_options.push(MountOption::FSName("porfs".into()));
            config.mount_options.push(MountOption::DefaultPermissions);
            if allow_other {
                config.acl = SessionACL::All;
            }
            let _ = writeback;

            eprintln!("mounting {} -> Ctrl-C to exit", mountpoint.display());
            eprintln!("卸载用: fusermount3 -u {}", mountpoint.display());
            // mount2 阻塞到会话结束。退出后由 PolarisFuse::drop 做 sync_all 保证持久化。
            fuser::mount2(fuse_impl, &mountpoint, &config)?;
            Ok(())
        }
    }
}
