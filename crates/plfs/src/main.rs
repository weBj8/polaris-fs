//! plfs: PolarisFS control binary — one binary, three roles + standalone.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// PolarisFS: KISS distributed file system for game server clusters.
#[derive(Debug, Parser)]
#[command(name = "plfs", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Format an arena on a raw block device or a sparse image.
    Mkfs(MkfsArgs),
    /// Run a data node hosting a ChunkArena.
    Data(DataArgs),
    /// Run a metadata node for a volume.
    Meta(MetaArgs),
    /// Mount a volume via FUSE.
    Mount(MountArgs),
    /// Run the registry (cluster membership).
    Registry(RegistryArgs),
    /// Single-process dev mode: data + meta + mount in one.
    Standalone(StandaloneArgs),
}

#[derive(Debug, Args)]
struct MkfsArgs {
    /// Volume directory to create (arena + metadata store).
    #[arg(long)]
    dir: PathBuf,
    /// Arena capacity in bytes.
    #[arg(long)]
    size: u64,
}

#[derive(Debug, Args)]
struct DataArgs {
    /// Path to the arena (raw device or image file).
    #[arg(long)]
    arena: PathBuf,
    /// gRPC listen address for the ChunkStore service.
    #[arg(long, default_value = "0.0.0.0:9100")]
    listen: std::net::SocketAddr,
    /// Registry endpoint (host:port); when set, this node registers its
    /// capacity and heartbeats its liveness lease there.
    #[arg(long)]
    registry: Option<String>,
    /// Node key for registry registration.
    #[arg(long, default_value = "data-0")]
    node_key: String,
}

#[derive(Debug, Args)]
struct MetaArgs {
    /// Metadata directory (holds meta.redb for this member).
    #[arg(long)]
    dir: std::path::PathBuf,
    /// This node's raft id.
    #[arg(long)]
    node_id: u64,
    /// gRPC listen address (raft transport + MetaOps).
    #[arg(long)]
    listen: std::net::SocketAddr,
    /// Group members as id@host:port (comma-separated, must include self).
    #[arg(long, value_delimiter = ',')]
    peers: Vec<String>,
}

#[derive(Debug, Args)]
struct MountArgs {
    /// Volume directory (created by `plfs mkfs --dir`).
    #[arg(long)]
    dir: PathBuf,
    /// Mount point.
    mountpoint: PathBuf,
}

#[derive(Debug, Args)]
struct RegistryArgs {
    /// Registry directory (holds registry.redb for this member).
    #[arg(long)]
    dir: std::path::PathBuf,
    /// This node's raft id.
    #[arg(long)]
    node_id: u64,
    /// gRPC listen address (raft transport + Registry service).
    #[arg(long)]
    listen: std::net::SocketAddr,
    /// Group members as id@host:port (comma-separated, must include self).
    #[arg(long, value_delimiter = ',')]
    peers: Vec<String>,
}

#[derive(Debug, Args)]
struct StandaloneArgs {
    /// Volume directory (arena + metadata + WAL).
    #[arg(long)]
    dir: PathBuf,
    /// Mount point.
    #[arg(long)]
    mount: PathBuf,
    /// Arena capacity in bytes (first format only).
    #[arg(long, default_value_t = 16 << 30)]
    size: u64,
    /// Prometheus metrics listen address (cache hit rate etc.).
    #[arg(long, default_value = "127.0.0.1:0")]
    metrics_listen: std::net::SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match &cli.command {
        Command::Mkfs(args) => {
            let dir = args.dir.clone();
            let size = args.size;
            tokio::task::spawn_blocking(move || plfs_client::fuse::PlfsFs::create(&dir, size))
                .await??;
            tracing::info!(dir = ?args.dir, size, "volume formatted");
        }
        Command::Data(args) => {
            let svc = plfs_data::ChunkStoreSvc::open(&args.arena)?;
            if let Some(registry_addr) = &args.registry {
                let (capacity, used) = svc.capacity_used().await?;
                let client =
                    plfs_registry::client::RegistryClient::new(vec![registry_addr.clone()]);
                let node = plfs_common::registry::v1::NodeInfo {
                    node_key: args.node_key.clone(),
                    kind: plfs_common::registry::v1::NodeKind::Data.into(),
                    addr: args.listen.to_string(),
                    volume: String::new(),
                    capacity_bytes: capacity,
                    used_bytes: used,
                    last_seen: 0,
                    lease_until: 0,
                };
                client
                    .register(node)
                    .await
                    .map_err(|e| anyhow::anyhow!("registry register: {e}"))?;
                tracing::info!(%registry_addr, key = %args.node_key, "registered to registry");
                let hb_svc = svc.clone();
                let hb_key = args.node_key.clone();
                let hb_client =
                    plfs_registry::client::RegistryClient::new(vec![registry_addr.clone()]);
                tokio::spawn(async move {
                    loop {
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        let used = hb_svc.capacity_used().await.map(|(_, u)| u).unwrap_or(0);
                        if let Err(e) = hb_client.heartbeat(&hb_key, used).await {
                            tracing::warn!(%e, "registry heartbeat failed");
                        }
                    }
                });
            }
            tonic::transport::Server::builder()
                .add_service(svc.into_server())
                .serve_with_shutdown(args.listen, async {
                    let _ = tokio::signal::ctrl_c().await;
                })
                .await?;
        }
        Command::Meta(args) => {
            let dir = args.dir.clone();
            std::fs::create_dir_all(&dir)?;
            let meta_path = dir.join("meta.redb");
            if !meta_path.exists() {
                plfs_meta::MetaState::create(&meta_path, (0, 0))?;
                tracing::info!(dir = ?dir, "metadata store created");
            }
            let state = plfs_meta::MetaState::open(&meta_path)?;
            let members: std::collections::BTreeMap<u64, String> = args
                .peers
                .iter()
                .map(|p| {
                    let (id, addr) = p.split_once('@').expect("peer as id@host:port");
                    (id.parse().expect("peer id"), addr.to_string())
                })
                .collect();
            if !members.contains_key(&args.node_id) {
                anyhow::bail!("--peers must include this node (id {})", args.node_id);
            }
            let group =
                plfs_meta::raft::MetaRaft::bootstrap_cluster(&state, args.node_id, &members)
                    .await?;
            let raft_handle = group.raft().clone();
            tracing::info!(id = args.node_id, listen = %args.listen, "meta node serving");
            tonic::transport::Server::builder()
                .add_service(
                    plfs_meta::transport::RaftTransportSvc::new(raft_handle.clone()).into_server(),
                )
                .add_service(
                    plfs_meta::service::MetaOpsSvc::new(raft_handle, state, args.node_id, members)
                        .into_server(),
                )
                .serve(args.listen)
                .await?;
        }
        Command::Mount(args) => {
            let dir = args.dir.clone();
            let mountpoint = args.mountpoint.clone();
            let guard = tokio::task::spawn_blocking(move || {
                plfs_client::fuse::MountGuard::mount(&dir, &mountpoint)
            })
            .await??;
            tracing::info!(mountpoint = ?args.mountpoint, "mounted; ctrl-c to unmount");
            tokio::signal::ctrl_c().await?;
            drop(guard);
            tracing::info!("unmounted");
        }
        Command::Registry(args) => {
            let dir = args.dir.clone();
            std::fs::create_dir_all(&dir)?;
            let path = dir.join("registry.redb");
            let state = if path.exists() {
                plfs_registry::state::RegistryState::open(&path)?
            } else {
                plfs_registry::state::RegistryState::create(&path)?
            };
            let members: std::collections::BTreeMap<u64, String> = args
                .peers
                .iter()
                .map(|p| {
                    let (id, addr) = p.split_once('@').expect("peer as id@host:port");
                    (id.parse().expect("peer id"), addr.to_string())
                })
                .collect();
            if !members.contains_key(&args.node_id) {
                anyhow::bail!("--peers must include this node (id {})", args.node_id);
            }
            let group = plfs_registry::raft::RegistryRaft::bootstrap_cluster(
                &state,
                args.node_id,
                &members,
            )
            .await?;
            let raft_handle = group.raft().clone();
            tracing::info!(id = args.node_id, listen = %args.listen, "registry node serving");
            tonic::transport::Server::builder()
                .add_service(
                    plfs_registry::transport::RaftTransportSvc::new(raft_handle).into_server(),
                )
                .add_service(
                    plfs_registry::service::RegistrySvc::new(group, state, args.node_id, members)
                        .into_server(),
                )
                .serve(args.listen)
                .await?;
        }
        Command::Standalone(args) => {
            let dir = args.dir.clone();
            let mountpoint = args.mount.clone();
            std::fs::create_dir_all(&dir)?;
            // 1. Format on first run (arena image + metadata store).
            if !dir.join("meta.redb").exists() {
                plfs_client::core::ClientCore::format_volume(&dir, args.size)?;
                tracing::info!(dir = ?dir, size = args.size, "volume formatted");
            }
            // 2. Data node on loopback (design doc §3.1: same code path as
            // cluster mode, RF=1).
            let arena_path = dir.join("arena.img");
            let svc = plfs_data::ChunkStoreSvc::open(&arena_path)?;
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
            let addr = listener.local_addr()?;
            tokio::spawn(async move {
                tonic::transport::Server::builder()
                    .add_service(svc.into_server())
                    .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
                    .await
            });
            tracing::info!(%addr, "data node serving (loopback)");
            let handle = metrics_exporter_prometheus::PrometheusBuilder::new()
                .with_http_listener(args.metrics_listen)
                .install()
                .ok();
            let _ = handle;
            tracing::info!(addr = %args.metrics_listen, "metrics exporter listening");
            // 3. Mount with the gRPC sink.
            let guard = {
                let dir = dir.clone();
                tokio::task::spawn_blocking(move || {
                    plfs_client::fuse::MountGuard::mount_with_sink(
                        &dir,
                        &mountpoint,
                        plfs_client::core::SinkConfig::Grpc(format!("http://{addr}")),
                    )
                })
                .await??
            };
            tracing::info!(mountpoint = ?args.mount, "standalone mounted; ctrl-c to stop");
            tokio::signal::ctrl_c().await?;
            drop(guard);
            tracing::info!("unmounted");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::Cli;

    #[test]
    fn cli_debug_assert() {
        Cli::command().debug_assert();
    }
}
