//! `porfs` — PolarisFS command line tool (mkfs / info / bench / mds-check).

mod bench;

use std::path::Path;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use porfs_format::DATA_START;
use porfs_store::{DEFAULT_QUEUE_DEPTH, ExtentStore};

/// PolarisFS tool: format v2 device management and extent-store benchmark.
#[derive(Parser)]
#[command(name = "porfs", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a device with the PolarisFS v2 on-disk format. With
    /// --meta, also format the redb metadata half of an MDS pair (the
    /// future "mkfs one command" shape of P6).
    Mkfs {
        /// Device (regular file) to initialize.
        #[arg(long)]
        device: std::path::PathBuf,
        /// Device size, e.g. 4GiB, 512MiB, or plain bytes.
        #[arg(long)]
        size: String,
        /// Optional redb metadata file: format a full MDS pair.
        #[arg(long)]
        meta: Option<std::path::PathBuf>,
    },
    /// Dump superblock fields and store state of a device.
    Info {
        /// Device to inspect.
        #[arg(long)]
        device: std::path::PathBuf,
    },
    /// Benchmark: sequential write, sequential read, random read.
    Bench {
        /// Device to (re)create for the benchmark.
        #[arg(long)]
        device: std::path::PathBuf,
        /// Total bytes to write, e.g. 1GiB.
        #[arg(long, default_value = "1GiB")]
        size: String,
        /// Bytes per extent, e.g. 1MiB (max 4MiB).
        #[arg(long, default_value = "1MiB")]
        extent_size: String,
        /// io_uring queue depth for batch operations.
        #[arg(long, default_value_t = DEFAULT_QUEUE_DEPTH)]
        queue_depth: u32,
    },
    /// Open an MDS pair (redb metadata + extent-store device), run the
    /// mount-time reconcile and the consistency self-check, print a summary.
    MdsCheck {
        /// redb metadata file of the MDS.
        #[arg(long)]
        meta: std::path::PathBuf,
        /// Extent-store device of the MDS.
        #[arg(long)]
        data: std::path::PathBuf,
    },
    /// Mount an MDS instance via FUSE. Runs in the foreground until
    /// unmounted (fusermount3 -u) or interrupted.
    Mount {
        /// redb metadata file of the MDS.
        #[arg(long)]
        meta: std::path::PathBuf,
        /// Extent-store device of the MDS.
        #[arg(long)]
        data: std::path::PathBuf,
        /// Directory to mount on (must exist).
        #[arg(long)]
        mountpoint: std::path::PathBuf,
        /// Attribute-cache TTL in seconds.
        #[arg(long, default_value_t = 1.0)]
        attr_ttl: f64,
        /// Lookup entry-cache TTL in seconds.
        #[arg(long, default_value_t = 1.0)]
        entry_ttl: f64,
        /// Disable kernel-side permission enforcement (default_permissions
        /// is on unless this flag is passed).
        #[arg(long, default_value_t = false)]
        no_default_permissions: bool,
        /// Allow access by users other than the mount owner (FUSE
        /// allow_other). Required for multi-user semantics and pjdfstest.
        #[arg(long, default_value_t = false)]
        allow_other: bool,
        /// One-command form: format the MDS pair with this size first when
        /// it does not exist yet (e.g. --format 4GiB), then mount. An
        /// existing pair is mounted as-is.
        #[arg(long)]
        format: Option<String>,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Mkfs { device, size, meta } => cmd_mkfs(&device, &size, meta.as_deref()),
        Commands::Info { device } => cmd_info(&device),
        Commands::Bench {
            device,
            size,
            extent_size,
            queue_depth,
        } => bench::cmd_bench(&device, &size, &extent_size, queue_depth),
        Commands::MdsCheck { meta, data } => cmd_mds_check(&meta, &data),
        Commands::Mount {
            meta,
            data,
            mountpoint,
            attr_ttl,
            entry_ttl,
            no_default_permissions,
            allow_other,
            format,
        } => cmd_mount(MountOpts {
            meta: &meta,
            data: &data,
            mountpoint: &mountpoint,
            attr_ttl,
            entry_ttl,
            no_default_permissions,
            allow_other,
            format: format.as_deref(),
        }),
    }
}

/// Everything the `mount` subcommand takes, grouped so `cmd_mount` stays
/// readable (and under the argument-count ceiling).
struct MountOpts<'a> {
    meta: &'a Path,
    data: &'a Path,
    mountpoint: &'a Path,
    attr_ttl: f64,
    entry_ttl: f64,
    no_default_permissions: bool,
    allow_other: bool,
    format: Option<&'a str>,
}

fn cmd_mount(opts: MountOpts<'_>) -> Result<()> {
    use fuser::{Config, MountOption};

    let MountOpts {
        meta,
        data,
        mountpoint,
        attr_ttl,
        entry_ttl,
        no_default_permissions,
        allow_other,
        format,
    } = opts;

    if let Some(size) = format {
        match (meta.exists(), data.exists()) {
            (true, true) => {
                println!("porfs: MDS pair exists, mounting as-is (--format skipped)");
            }
            (false, false) => {
                let size = parse_size(size)?;
                porfs_mds::Mds::format(meta, data, size).with_context(|| {
                    format!("format MDS pair ({}, {})", meta.display(), data.display())
                })?;
                println!(
                    "porfs: formatted MDS pair ({}, {})",
                    meta.display(),
                    data.display()
                );
            }
            _ => bail!(
                "half an MDS pair exists (meta: {}, data: {}); remove the leftover",
                meta.display(),
                data.display()
            ),
        }
    }

    let config = porfs_fuse::MountConfig {
        attr_ttl: std::time::Duration::from_secs_f64(attr_ttl),
        entry_ttl: std::time::Duration::from_secs_f64(entry_ttl),
        default_permissions: !no_default_permissions,
        ..Default::default()
    };
    let fs = porfs_fuse::PorfsFs::open(meta, data, config)
        .with_context(|| format!("open MDS ({}, {})", meta.display(), data.display()))?;
    // Owner-only access (no allow_other); kernel permission enforcement
    // (default_permissions) is on unless explicitly disabled.
    let mut fuse_config = Config::default();
    fuse_config.mount_options = vec![MountOption::FSName("porfs".to_string())];
    if config.default_permissions {
        fuse_config
            .mount_options
            .push(MountOption::DefaultPermissions);
    }
    if allow_other {
        fuse_config
            .mount_options
            .push(MountOption::CUSTOM("allow_other".to_string()));
    }
    println!(
        "porfs: serving {} on {} — unmount with `fusermount3 -u {}` (Ctrl-C detaches; run fusermount3 -u afterwards if needed)",
        data.display(),
        mountpoint.display(),
        mountpoint.display()
    );
    fuser::mount2(fs, mountpoint, &fuse_config).context("fuse session error")?;
    println!("porfs: unmounted {}", mountpoint.display());
    Ok(())
}

fn cmd_mds_check(meta: &Path, data: &Path) -> Result<()> {
    let mut mds = porfs_mds::Mds::open(meta, data)
        .with_context(|| format!("open MDS ({}, {})", meta.display(), data.display()))?;
    let report = mds.self_check().context("mds self-check failed")?;
    println!("mds-check: OK");
    println!("meta:             {}", meta.display());
    println!("data:             {}", data.display());
    println!("inodes:           {}", report.inodes);
    println!("dir_entries:      {}", report.entries);
    println!("extent_map_rows:  {}", report.extents);
    println!("orphans_repaired: {}", report.orphans_repaired);
    Ok(())
}

fn cmd_mkfs(device: &Path, size: &str, meta: Option<&Path>) -> Result<()> {
    let size = parse_size(size)?;
    match meta {
        None => {
            let store = ExtentStore::create(device, size)
                .with_context(|| format!("mkfs on {}", device.display()))?;
            println!(
                "created PolarisFS format v{} device",
                porfs_format::FORMAT_VERSION
            );
            println!("  device:       {}", device.display());
            println!(
                "  device_size:  {} ({})",
                store.device_size(),
                human(store.device_size())
            );
            println!("  uuid:         {}", uuid::Uuid::from_bytes(store.uuid()));
            println!("  data_start:   {DATA_START}");
            println!("  io_mode:      {}", io_mode(store.is_direct()));
        }
        Some(meta) => {
            let mds = porfs_mds::Mds::format(meta, device, size).with_context(|| {
                format!("mkfs MDS pair ({}, {})", meta.display(), device.display())
            })?;
            let stats = mds.statfs()?;
            println!(
                "created PolarisFS MDS pair (extent format v{})",
                porfs_format::FORMAT_VERSION
            );
            println!("  meta:         {}", meta.display());
            println!("  device:       {}", device.display());
            println!(
                "  device_size:  {} ({})",
                stats.total_bytes,
                human(stats.total_bytes)
            );
        }
    }
    Ok(())
}

fn cmd_info(device: &Path) -> Result<()> {
    let store = ExtentStore::open(device).with_context(|| format!("open {}", device.display()))?;
    println!("device:         {}", device.display());
    println!("format_version: {}", porfs_format::FORMAT_VERSION);
    println!("uuid:           {}", uuid::Uuid::from_bytes(store.uuid()));
    println!(
        "device_size:    {} ({})",
        store.device_size(),
        human(store.device_size())
    );
    println!("data_start:     {DATA_START}");
    println!("tail:           {}", store.tail());
    println!("extent_count:   {}", store.extent_count());
    println!(
        "live_bytes:     {} ({})",
        store.live_bytes(),
        human(store.live_bytes())
    );
    println!("sync_seq:       {}", store.sync_seq());
    println!("confirmed_id:   {}", store.confirmed_id());
    println!("checkpoint:");
    println!("  slot_gen:     {}", store.checkpoint_slot_gen());
    println!("  covered_tail: {}", store.checkpoint_covered_tail());
    println!("  entry_count:  {}", store.checkpoint_entry_count());
    println!(
        "  mount_used:   {}",
        if store.mount_used_checkpoint() {
            "yes"
        } else {
            "no"
        }
    );
    println!("created_at:     {}", store.created_at());
    println!("last_sync_at:   {}", store.last_sync_at());
    println!("io_mode:        {}", io_mode(store.is_direct()));
    Ok(())
}

fn io_mode(direct: bool) -> &'static str {
    if direct {
        "O_DIRECT"
    } else {
        "buffered (O_DIRECT unsupported)"
    }
}

/// Human-readable byte size (base 1024).
fn human(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;
    let b = bytes as f64;
    if b >= GIB {
        format!("{:.1} GiB", b / GIB)
    } else if b >= MIB {
        format!("{:.1} MiB", b / MIB)
    } else if b >= KIB {
        format!("{:.1} KiB", b / KIB)
    } else {
        format!("{bytes} B")
    }
}

/// Parse a human size: bare bytes, or K/KB (1000), KiB (1024), M/MB, MiB,
/// G/GB, GiB, T/TB, TiB (case-insensitive). Integers only.
fn parse_size(s: &str) -> Result<u64> {
    let s = s.trim();
    let split = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    let (num, unit) = s.split_at(split);
    if num.is_empty() {
        bail!("invalid size {s:?}: missing number");
    }
    let value: u64 = num
        .parse()
        .with_context(|| format!("invalid size number in {s:?}"))?;
    let mult: u64 = match unit.to_ascii_lowercase().as_str() {
        "" | "b" => 1,
        "k" | "kb" => 1_000,
        "kib" => 1 << 10,
        "m" | "mb" => 1_000_000,
        "mib" => 1 << 20,
        "g" | "gb" => 1_000_000_000,
        "gib" => 1 << 30,
        "t" | "tb" => 1_000_000_000_000,
        "tib" => 1 << 40,
        _ => bail!("invalid size {s:?}: unknown unit {unit:?}"),
    };
    value
        .checked_mul(mult)
        .with_context(|| format!("size overflow: {s:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sizes() {
        assert_eq!(parse_size("4096").unwrap(), 4096);
        assert_eq!(parse_size("1KiB").unwrap(), 1024);
        assert_eq!(parse_size("1KB").unwrap(), 1000);
        assert_eq!(parse_size("512MiB").unwrap(), 512 << 20);
        assert_eq!(parse_size("4GiB").unwrap(), 4 << 30);
        assert_eq!(parse_size("1gib").unwrap(), 1 << 30);
        assert_eq!(parse_size("2T").unwrap(), 2_000_000_000_000);
    }

    #[test]
    fn parse_size_rejects_garbage() {
        assert!(parse_size("").is_err());
        assert!(parse_size("GiB").is_err());
        assert!(parse_size("1ZiB").is_err());
        assert!(parse_size("1.5GiB").is_err());
        assert!(parse_size("abc").is_err());
    }
}
