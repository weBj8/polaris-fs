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
    /// Raw block device to format.
    #[arg(long, conflicts_with = "img")]
    device: Option<PathBuf>,
    /// Sparse image file to format.
    #[arg(long, requires = "size")]
    img: Option<PathBuf>,
    /// Image size in bytes (required with --img).
    #[arg(long)]
    size: Option<u64>,
}

#[derive(Debug, Args)]
struct DataArgs {
    /// Path to the arena (raw device or image file).
    #[arg(long)]
    arena: PathBuf,
}

#[derive(Debug, Args)]
struct MetaArgs {
    /// Volume name.
    #[arg(long)]
    volume: String,
    /// Comma-separated peer list.
    #[arg(long, value_delimiter = ',')]
    peers: Vec<String>,
}

#[derive(Debug, Args)]
struct MountArgs {
    /// Volume name.
    #[arg(long)]
    volume: String,
    /// Mount point.
    mountpoint: PathBuf,
}

#[derive(Debug, Args)]
struct RegistryArgs {
    /// Comma-separated peer list.
    #[arg(long, value_delimiter = ',')]
    peers: Vec<String>,
}

#[derive(Debug, Args)]
struct StandaloneArgs {
    /// Data directory.
    #[arg(long)]
    dir: PathBuf,
    /// Mount point.
    #[arg(long)]
    mount: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match &cli.command {
        Command::Mkfs(args) => tracing::info!(?args, "mkfs"),
        Command::Data(args) => tracing::info!(?args, "data"),
        Command::Meta(args) => tracing::info!(?args, "meta"),
        Command::Mount(args) => tracing::info!(?args, "mount"),
        Command::Registry(args) => tracing::info!(?args, "registry"),
        Command::Standalone(args) => tracing::info!(?args, "standalone"),
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
