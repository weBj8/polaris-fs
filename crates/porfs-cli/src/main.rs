//! `porfs` — PolarisFS command line tool (mkfs / info / bench / mds-check).

mod bench;

use std::path::Path;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use porfs_cli::{human_bytes, init_tracing, install_prometheus_exporter};
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
    /// future "mkfs one command" shape of P6). With --meta and --chunks,
    /// format a cluster MDS whose file data lives on chunkservers (no
    /// local device).
    Mkfs {
        /// Device (regular file) to initialize (local mode; not used
        /// with --chunks).
        #[arg(long)]
        device: Option<std::path::PathBuf>,
        /// Device size, e.g. 4GiB, 512MiB, or plain bytes (local mode;
        /// rejected with --chunks).
        #[arg(long)]
        size: Option<String>,
        /// Optional redb metadata file: format a full MDS pair (or, with
        /// --chunks, a cluster MDS).
        #[arg(long)]
        meta: Option<std::path::PathBuf>,
        /// Chunkserver members of a cluster filesystem: comma-separated
        /// addr[@rack][/chassis] entries (default rack = the addr itself,
        /// default chassis = "default" — truthful topology is the
        /// operator's job, docs/protocol.md §5). Formats a cluster MDS
        /// with file data on these chunkservers.
        #[arg(long)]
        chunks: Option<String>,
        /// Copies of every extent (1 or 2; default 2). Only meaningful
        /// with --chunks; replicas=2 requires members in at least two
        /// racks.
        #[arg(long)]
        replicas: Option<usize>,
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
    /// Open an MDS instance (redb metadata + extent-store device, or a
    /// cluster MDS when --data is omitted), run the mount-time reconcile
    /// and the consistency self-check, print a summary.
    MdsCheck {
        /// redb metadata file of the MDS.
        #[arg(long)]
        meta: std::path::PathBuf,
        /// Extent-store device of the MDS. Omit for a cluster filesystem
        /// (chunkserver membership is read from the meta file).
        #[arg(long)]
        data: Option<std::path::PathBuf>,
    },
    /// Mount an MDS instance via FUSE. Runs in the foreground until
    /// unmounted (fusermount3 -u) or interrupted. Local mode takes --data;
    /// cluster mode takes --chunks on first mount (membership is
    /// persisted) and neither on a remount.
    Mount {
        /// redb metadata file of the MDS.
        #[arg(long)]
        meta: std::path::PathBuf,
        /// Extent-store device of the MDS (local mode). Mutually
        /// exclusive with --chunks; omit both for a cluster remount.
        #[arg(long)]
        data: Option<std::path::PathBuf>,
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
        /// existing pair is mounted as-is. With --chunks, pass --format
        /// without a size (capacity comes from the chunkserver devices).
        #[arg(long, num_args = 0..=1)]
        format: Option<Option<String>>,
        /// Chunkserver members of a cluster filesystem: comma-separated
        /// addr[@rack][/chassis] entries (defaults as in `mkfs`). Formats
        /// a cluster MDS when the meta file does not exist yet; an
        /// existing MDS is mounted as-is.
        #[arg(long)]
        chunks: Option<String>,
        /// Copies of every extent (1 or 2; default 2). Only meaningful
        /// with --chunks; replicas=2 requires members in at least two
        /// racks.
        #[arg(long)]
        replicas: Option<usize>,
    },
    /// Run a chunkserver: serve an extent-store device over wire
    /// protocol v1 (docs/protocol.md) until interrupted.
    Chunkserver {
        /// Extent-store device file.
        #[arg(long)]
        device: std::path::PathBuf,
        /// Create the device with this size first when it does not exist
        /// (e.g. 16GiB).
        #[arg(long)]
        size: Option<String>,
        /// Listen address.
        #[arg(long, default_value = "127.0.0.1:9100")]
        listen: std::net::SocketAddr,
        /// Optional Prometheus HTTP listener (for example 127.0.0.1:9900).
        #[arg(long)]
        metrics_listen: Option<std::net::SocketAddr>,
    },
}

fn main() -> Result<()> {
    init_tracing()?;
    match Cli::parse().command {
        Commands::Mkfs {
            device,
            size,
            meta,
            chunks,
            replicas,
        } => cmd_mkfs(
            device.as_deref(),
            size.as_deref(),
            meta.as_deref(),
            chunks.as_deref(),
            replicas,
        ),
        Commands::Info { device } => cmd_info(&device),
        Commands::Bench {
            device,
            size,
            extent_size,
            queue_depth,
        } => bench::cmd_bench(&device, &size, &extent_size, queue_depth),
        Commands::MdsCheck { meta, data } => cmd_mds_check(&meta, data.as_deref()),
        Commands::Chunkserver {
            device,
            size,
            listen,
            metrics_listen,
        } => cmd_chunkserver(&device, size.as_deref(), listen, metrics_listen),
        Commands::Mount {
            meta,
            data,
            mountpoint,
            attr_ttl,
            entry_ttl,
            no_default_permissions,
            allow_other,
            format,
            chunks,
            replicas,
        } => cmd_mount(MountOpts {
            meta: &meta,
            data: data.as_deref(),
            mountpoint: &mountpoint,
            attr_ttl,
            entry_ttl,
            no_default_permissions,
            allow_other,
            format: format.as_ref().map(|inner| inner.as_deref()),
            chunks: chunks.as_deref(),
            replicas,
        }),
    }
}

/// Everything the `mount` subcommand takes, grouped so `cmd_mount` stays
/// readable (and under the argument-count ceiling).
struct MountOpts<'a> {
    meta: &'a Path,
    data: Option<&'a Path>,
    mountpoint: &'a Path,
    attr_ttl: f64,
    entry_ttl: f64,
    no_default_permissions: bool,
    allow_other: bool,
    format: Option<Option<&'a str>>,
    chunks: Option<&'a str>,
    replicas: Option<usize>,
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
        chunks,
        replicas,
    } = opts;

    if chunks.is_some() && data.is_some() {
        bail!("--chunks and --data are mutually exclusive (cluster data plane vs local device)");
    }
    if chunks.is_none() && replicas.is_some() {
        bail!("--replicas is only meaningful with --chunks");
    }

    let config = porfs_fuse::MountConfig {
        attr_ttl: std::time::Duration::from_secs_f64(attr_ttl),
        entry_ttl: std::time::Duration::from_secs_f64(entry_ttl),
        default_permissions: !no_default_permissions,
        ..Default::default()
    };

    let serving_label: String;
    let fs = match (chunks, data) {
        (Some(member_spec), None) => {
            let members = parse_members(member_spec)?;
            let replicas = check_replicas(replicas)?;
            if let Some(Some(_)) = format {
                bail!(
                    "--format takes no size with --chunks (capacity comes from the chunkserver devices)"
                );
            }
            if meta.exists() {
                println!("porfs: cluster MDS exists, mounting as-is (--chunks ignored)");
            } else {
                porfs_mds::Mds::format_cluster(meta, members.clone(), replicas)
                    .map(|_| ())
                    .with_context(|| format!("format cluster MDS ({})", meta.display()))?;
                println!(
                    "porfs: formatted cluster MDS {} ({} chunkservers, replicas={replicas})",
                    meta.display(),
                    members.len()
                );
            }
            serving_label = format!("{} chunkservers", members.len());
            open_cluster_fs(meta, &config)?
        }
        (None, Some(data)) => {
            if let Some(size) = format {
                let size = size.ok_or_else(|| {
                    anyhow::anyhow!(
                        "--format requires a size for a local MDS pair (e.g. --format 4GiB)"
                    )
                })?;
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
            serving_label = data.display().to_string();
            let fs = porfs_fuse::PorfsFs::open(meta, data, config)
                .with_context(|| format!("open MDS ({}, {})", meta.display(), data.display()))?;
            tracing::info!(
                command = "mount",
                meta = %meta.display(),
                data = %data.display(),
                mountpoint = %mountpoint.display(),
                "starting FUSE mount"
            );
            fs
        }
        (None, None) => {
            serving_label = "cluster".to_string();
            open_cluster_fs(meta, &config)?
        }
        (Some(_), Some(_)) => unreachable!("--chunks and --data rejected above"),
    };
    if data.is_none() {
        tracing::info!(
            command = "mount",
            meta = %meta.display(),
            mountpoint = %mountpoint.display(),
            "starting FUSE mount"
        );
    }
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
        serving_label,
        mountpoint.display(),
        mountpoint.display()
    );
    fuser::mount2(fs, mountpoint, &fuse_config).context("fuse session error")?;
    println!("porfs: unmounted {}", mountpoint.display());
    tracing::info!(command = "mount", mountpoint = %mountpoint.display(), "FUSE mount exited");
    Ok(())
}

/// Open a cluster-mode MDS for mounting, translating the local-mode
/// mismatch into the operator-facing guidance.
fn open_cluster_fs(meta: &Path, config: &porfs_fuse::MountConfig) -> Result<porfs_fuse::PorfsFs> {
    match porfs_fuse::PorfsFs::open_cluster(meta, *config) {
        Ok(fs) => Ok(fs),
        Err(porfs_mds::MdsError::InvalidOp(msg)) if msg.contains("local-mode") => bail!(
            "{} is not a cluster filesystem; pass --data for a local MDS or --chunks to format one",
            meta.display()
        ),
        Err(err) => Err(err).with_context(|| format!("open cluster MDS ({})", meta.display())),
    }
}

fn cmd_chunkserver(
    device: &Path,
    size: Option<&str>,
    listen: std::net::SocketAddr,
    metrics_listen: Option<std::net::SocketAddr>,
) -> Result<()> {
    let device = device.to_path_buf();
    let size = size.map(parse_size).transpose()?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("build tokio runtime")?;
    runtime.block_on(async move {
        tracing::info!(
            command = "chunkserver",
            device = %device.display(),
            %listen,
            metrics_listen = ?metrics_listen,
            "starting chunkserver"
        );
        if let Some(metrics_listen) = metrics_listen {
            install_prometheus_exporter(metrics_listen)?;
            tracing::info!(%metrics_listen, "Prometheus exporter listening");
        }
        let listener = tokio::net::TcpListener::bind(listen)
            .await
            .with_context(|| format!("bind {listen}"))?;
        let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
        let open_device = device.clone();
        let open = move || match size {
            Some(size) => porfs_store::ExtentStore::create(&open_device, size),
            None => porfs_store::ExtentStore::open(&open_device),
        };
        let server = porfs_rpc::server::serve(listener, open, shutdown_rx)
            .await
            .map_err(|err| anyhow::anyhow!(err))
            .context("serve chunkserver")?;
        println!(
            "porfs-chunkserver: serving {} on {}",
            device.display(),
            server.addr()
        );
        tracing::info!(device = %device.display(), addr = %server.addr(), "chunkserver ready");
        tokio::signal::ctrl_c().await?;
        println!("porfs-chunkserver: shutting down");
        tracing::info!("chunkserver shutting down");
        let _ = shutdown_tx.send(true);
        drop(server);
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(())
}

fn cmd_mds_check(meta: &Path, data: Option<&Path>) -> Result<()> {
    tracing::info!(
        command = "mds-check",
        meta = %meta.display(),
        data = ?data.map(|path| path.display().to_string()),
        "opening metadata service"
    );
    let (mut mds, data_label): (porfs_mds::Mds, String) = match data {
        Some(data) => (
            porfs_mds::Mds::open(meta, data)
                .with_context(|| format!("open MDS ({}, {})", meta.display(), data.display()))?,
            data.display().to_string(),
        ),
        None => (
            porfs_mds::Mds::open_cluster(meta)
                .with_context(|| format!("open cluster MDS ({})", meta.display()))?,
            "cluster (chunkserver data plane)".to_string(),
        ),
    };
    let report = mds.self_check().context("mds self-check failed")?;
    println!("mds-check: OK");
    println!("meta:             {}", meta.display());
    println!("data:             {data_label}");
    println!("inodes:           {}", report.inodes);
    println!("dir_entries:      {}", report.entries);
    println!("extent_map_rows:  {}", report.extents);
    println!("orphans_repaired: {}", report.orphans_repaired);
    Ok(())
}

fn cmd_mkfs(
    device: Option<&Path>,
    size: Option<&str>,
    meta: Option<&Path>,
    chunks: Option<&str>,
    replicas: Option<usize>,
) -> Result<()> {
    if let Some(member_spec) = chunks {
        let meta = meta.ok_or_else(|| anyhow::anyhow!("--chunks requires --meta"))?;
        if device.is_some() || size.is_some() {
            bail!(
                "--device/--size are not used with --chunks (capacity comes from the chunkserver devices)"
            );
        }
        let members = parse_members(member_spec)?;
        let replicas = check_replicas(replicas)?;
        tracing::info!(
            command = "mkfs",
            meta = %meta.display(),
            chunkservers = members.len(),
            replicas,
            "formatting cluster MDS"
        );
        let mds = porfs_mds::Mds::format_cluster(meta, members.clone(), replicas)
            .with_context(|| format!("mkfs cluster MDS ({})", meta.display()))?;
        let stats = mds.statfs()?;
        println!(
            "created PolarisFS cluster MDS (extent format v{})",
            porfs_format::FORMAT_VERSION
        );
        println!("  meta:         {}", meta.display());
        println!("  chunkservers: {}", members.len());
        println!("  replicas:     {replicas}");
        println!(
            "  device_size:  {} ({})",
            stats.total_bytes,
            human_bytes(stats.total_bytes)
        );
        return Ok(());
    }
    if replicas.is_some() {
        bail!("--replicas is only meaningful with --chunks");
    }
    let (device, size) = match (device, size) {
        (Some(device), Some(size)) => (device, size),
        _ => bail!(
            "mkfs requires --device and --size (or --meta with --chunks for a cluster filesystem)"
        ),
    };
    let size = parse_size(size)?;
    tracing::info!(
        command = "mkfs",
        device = %device.display(),
        meta = meta.map(|path| path.display().to_string()),
        size_bytes = size,
        "formatting device"
    );
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
                human_bytes(store.device_size())
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
                human_bytes(stats.total_bytes)
            );
        }
    }
    Ok(())
}

fn cmd_info(device: &Path) -> Result<()> {
    tracing::info!(command = "info", device = %device.display(), "reading device info");
    let store = ExtentStore::open(device).with_context(|| format!("open {}", device.display()))?;
    println!("device:         {}", device.display());
    println!("format_version: {}", porfs_format::FORMAT_VERSION);
    println!("uuid:           {}", uuid::Uuid::from_bytes(store.uuid()));
    println!(
        "device_size:    {} ({})",
        store.device_size(),
        human_bytes(store.device_size())
    );
    println!("data_start:     {DATA_START}");
    println!("tail:           {}", store.tail());
    println!("extent_count:   {}", store.extent_count());
    println!(
        "live_bytes:     {} ({})",
        store.live_bytes(),
        human_bytes(store.live_bytes())
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

/// Parse the `--chunks` member list: comma-separated `addr[@rack][/chassis]`
/// entries. The default rack is the address string itself, the default
/// chassis "default" (truthful topology is the operator's job,
/// docs/protocol.md §5). Every bad token is a usage error naming it.
fn parse_members(spec: &str) -> Result<Vec<(std::net::SocketAddr, String, String)>> {
    let mut members = Vec::new();
    for token in spec.split(',') {
        let token = token.trim();
        if token.is_empty() {
            bail!("invalid --chunks member: empty entry in {spec:?}");
        }
        let (addr_str, topo) = token
            .split_once('@')
            .map_or((token, None), |(a, t)| (a, Some(t)));
        let addr: std::net::SocketAddr = addr_str.parse().with_context(|| {
            format!("invalid --chunks member {token:?}: bad address {addr_str:?}")
        })?;
        let (rack, chassis) = match topo {
            None => (addr_str.to_string(), "default".to_string()),
            Some("") => bail!("invalid --chunks member {token:?}: empty rack tag"),
            Some(tags) => match tags.split_once('/') {
                None => (tags.to_string(), "default".to_string()),
                Some((rack, chassis))
                    if !rack.is_empty() && !chassis.is_empty() && !chassis.contains('/') =>
                {
                    (rack.to_string(), chassis.to_string())
                }
                _ => bail!("invalid --chunks member {token:?}: bad rack/chassis tags"),
            },
        };
        members.push((addr, rack, chassis));
    }
    if members.is_empty() {
        bail!("--chunks requires at least one member");
    }
    Ok(members)
}

/// The replica count: 1 or 2 (default 2). Rack separation for replicas=2
/// is enforced by `Mds::format_cluster` against the live membership.
fn check_replicas(replicas: Option<usize>) -> Result<usize> {
    let replicas = replicas.unwrap_or(2);
    if !(1..=2).contains(&replicas) {
        bail!("--replicas must be 1 or 2, got {replicas}");
    }
    Ok(replicas)
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

    #[test]
    fn parse_members_defaults_and_topology() {
        let members =
            parse_members("127.0.0.1:9101,127.0.0.1:9102@r2,127.0.0.1:9103@r2/c7").unwrap();
        assert_eq!(members.len(), 3);
        assert_eq!(members[0].0.to_string(), "127.0.0.1:9101");
        assert_eq!(members[0].1, "127.0.0.1:9101"); // default rack = the addr
        assert_eq!(members[0].2, "default");
        assert_eq!(members[1].1, "r2");
        assert_eq!(members[1].2, "default");
        assert_eq!(members[2].1, "r2");
        assert_eq!(members[2].2, "c7");
    }

    #[test]
    fn parse_members_rejects_bad_tokens() {
        for bad in [
            "",
            "not-an-addr",
            "127.0.0.1:9101@",
            "127.0.0.1:9101@/c1",
            "127.0.0.1:9101@r1/",
            "127.0.0.1:9101@r1/c1/extra",
            "127.0.0.1:9101,,127.0.0.1:9102",
            "@r1",
        ] {
            let err = parse_members(bad).unwrap_err();
            assert!(!err.to_string().is_empty(), "{bad:?}");
        }
    }

    #[test]
    fn replicas_validation() {
        assert_eq!(check_replicas(None).unwrap(), 2);
        assert_eq!(check_replicas(Some(1)).unwrap(), 1);
        assert_eq!(check_replicas(Some(2)).unwrap(), 2);
        assert!(check_replicas(Some(0)).is_err());
        assert!(check_replicas(Some(3)).is_err());
    }
}
