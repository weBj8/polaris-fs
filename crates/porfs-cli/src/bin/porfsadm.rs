use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use porfs_cli::{human_bytes, init_tracing};
use porfs_rpc::ChunkClient;

#[derive(Parser)]
#[command(name = "porfsadm", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Query one chunkserver's current extent-store status via Stats RPC.
    Status {
        /// Chunkserver RPC address.
        #[arg(long, default_value = "127.0.0.1:9100")]
        addr: std::net::SocketAddr,
    },
}

fn main() -> Result<()> {
    init_tracing()?;
    match Cli::parse().command {
        Commands::Status { addr } => cmd_status(addr),
    }
}

fn cmd_status(addr: std::net::SocketAddr) -> Result<()> {
    tracing::info!(command = "status", %addr, "querying chunkserver status");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("build tokio runtime")?;
    let stats = runtime.block_on(async move {
        let client = ChunkClient::new(addr);
        client.stats().await
    })?;
    println!("chunkserver:    {addr}");
    println!(
        "device_size:    {} ({})",
        stats.device_size,
        human_bytes(stats.device_size)
    );
    println!(
        "tail:           {} ({})",
        stats.tail,
        human_bytes(stats.tail)
    );
    println!(
        "free_bytes:     {} ({})",
        stats.free_bytes(),
        human_bytes(stats.free_bytes())
    );
    println!(
        "live_bytes:     {} ({})",
        stats.live_bytes,
        human_bytes(stats.live_bytes)
    );
    println!("extent_count:   {}", stats.extent_count);
    println!("confirmed_id:   {}", stats.confirmed_id);
    Ok(())
}
