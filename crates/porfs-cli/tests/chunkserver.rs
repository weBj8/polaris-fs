//! End-to-end test of the `porfs chunkserver` binary as a subprocess:
//! kill -9 after a confirmed write must lose nothing (durability horizon
//! over the wire, protocol v1 §3/§4).

use std::process::{Child, Command};
use std::time::{Duration, Instant};

use porfs_rpc::{ChunkClient, RpcError};
use tempfile::TempDir;

const LISTEN: &str = "127.0.0.1:19411";

fn spawn_chunkserver(device: &std::path::Path, create: bool) -> Child {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_porfs"));
    cmd.arg("chunkserver")
        .arg("--device")
        .arg(device)
        .arg("--listen")
        .arg(LISTEN);
    if create {
        cmd.arg("--size").arg("128MiB");
    }
    cmd.spawn().expect("spawn porfs chunkserver")
}

async fn wait_ready(addr: std::net::SocketAddr) -> ChunkClient {
    let client = ChunkClient::new(addr);
    let started = Instant::now();
    loop {
        match tokio::time::timeout(Duration::from_secs(2), client.stats()).await {
            Ok(Ok(_)) => return client,
            Ok(Err(RpcError::Remote { .. })) => panic!("server answered with remote error"),
            _ if started.elapsed() > Duration::from_secs(15) => {
                panic!("chunkserver did not come up on {addr}")
            }
            _ => tokio::time::sleep(Duration::from_millis(100)).await,
        }
    }
}

#[test]
fn chunkserver_kill9_keeps_confirmed_extents() {
    let dir = TempDir::new().unwrap();
    let device = dir.path().join("dev.img");
    let addr: std::net::SocketAddr = LISTEN.parse().unwrap();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async move {
        let mut child = spawn_chunkserver(&device, true);
        let client = wait_ready(addr).await;
        let data: Vec<u8> = (0..(1 << 20) as u64).map(|i| (i % 251) as u8).collect();
        let id = client
            .write_extent(0xC0FFEE, 1, 0, data.clone())
            .await
            .unwrap();
        let confirmed = client.sync().await.unwrap();
        assert!(id <= confirmed);

        // kill -9 the server mid-flight: unconfirmed tail may be lost,
        // everything at or below confirmed_id must survive.
        child.kill().unwrap();
        child.wait().unwrap();

        let mut child = spawn_chunkserver(&device, false);
        let client = wait_ready(addr).await;
        let back = client.read_extent(id).await.unwrap();
        assert_eq!(back, data, "confirmed extent must survive kill -9");

        child.kill().unwrap();
        child.wait().unwrap();
    });
}
