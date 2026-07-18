//! Transport ceiling micro-bench: one client, one chunkserver, windowed
//! pipelined reads — isolates the RPC transport from the cluster layer.

use std::time::{Duration, Instant};

use porfs_rpc::server::{Server, serve};
use porfs_rpc::{ChunkClient, Request, Response};
use porfs_store::ExtentStore;
use tempfile::TempDir;
use tokio::net::TcpListener;
use tokio::sync::watch;

const DEV_SIZE: u64 = 1 << 30;
const CHUNK: usize = 4 << 20;

async fn start() -> (TempDir, Server, watch::Sender<bool>, ChunkClient) {
    let dir = tempfile::tempdir().unwrap();
    let device = dir.path().join("dev.img");
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let server = serve(
        listener,
        move || ExtentStore::create(&device, DEV_SIZE),
        shutdown_rx,
    )
    .await
    .unwrap();
    let client = ChunkClient::new(server.addr());
    (dir, server, shutdown_tx, client)
}

fn mib(bytes: f64) -> f64 {
    bytes / (1 << 20) as f64
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn transport_ceiling() {
    let (_dir, server, shutdown, client) = start().await;

    // Seed 64 extents of 4 MiB each (256 MiB).
    let payload: Vec<u8> = (0..CHUNK as u64).map(|i| (i % 251) as u8).collect();
    let mut ids = Vec::new();
    for i in 0..64u64 {
        ids.push(
            client
                .write_extent(i as u128, 1, i * CHUNK as u64, payload.clone())
                .await
                .unwrap(),
        );
    }

    // Single-roundtrip latency over 4 MiB.
    let started = Instant::now();
    for &id in ids.iter().take(8) {
        client.read_extent(id).await.unwrap();
    }
    let rtt = started.elapsed() / 8;
    println!(
        "bench: 4MiB single roundtrip: {rtt:?} ({:.0} MiB/s)",
        mib(CHUNK as f64 / rtt.as_secs_f64())
    );

    // Windowed pipelined reads at several window sizes.
    let requests: Vec<Request> = ids
        .iter()
        .map(|&extent_id| Request::ReadExtent { extent_id })
        .collect();
    for window in [1usize, 8, 32] {
        let started = Instant::now();
        let responses = client.call_many(&requests, window).await.unwrap();
        let elapsed = started.elapsed();
        assert_eq!(responses.len(), ids.len());
        for response in responses {
            let Response::ReadAck { data } = response else {
                panic!("unexpected response")
            };
            assert_eq!(data.len(), CHUNK);
        }
        let total = (ids.len() * CHUNK) as f64;
        println!(
            "bench: window {window:>2}: {:.0} MiB/s ({elapsed:?} for {} MiB)",
            mib(total / elapsed.as_secs_f64()),
            (total as u64) >> 20
        );
    }

    drop(client);
    let _ = shutdown.send(true);
    drop(server);
    tokio::time::sleep(Duration::from_millis(200)).await;
}

/// Contention picture: 4 servers × 4 clients each (16 connections), each
/// client windowed-reads 32 MiB — aggregate over all of them.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn transport_16_conn_contention() {
    // Four servers on their own devices.
    let mut dirs = Vec::new();
    let mut addrs = Vec::new();
    let mut servers = Vec::new();
    let mut shutdowns = Vec::new();
    for _ in 0..4 {
        let dir = tempfile::tempdir().unwrap();
        let device = dir.path().join("dev.img");
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let server = serve(
            listener,
            move || ExtentStore::create(&device, DEV_SIZE),
            shutdown_rx,
        )
        .await
        .unwrap();
        addrs.push(server.addr());
        servers.push(server);
        shutdowns.push(shutdown_tx);
        dirs.push(dir);
    }

    // Seed 8 extents of 4 MiB per server (one client seeds each).
    let payload: Vec<u8> = (0..CHUNK as u64).map(|i| (i % 251) as u8).collect();
    let mut per_server_ids = Vec::new();
    for (s, addr) in addrs.iter().enumerate() {
        let client = ChunkClient::new(*addr);
        let mut ids = Vec::new();
        for i in 0..8u64 {
            ids.push(
                client
                    .write_extent(
                        ((s as u128) << 64) | i as u128,
                        1,
                        i * CHUNK as u64,
                        payload.clone(),
                    )
                    .await
                    .unwrap(),
            );
        }
        per_server_ids.push(ids);
    }

    // 16 clients (4 per server) read their server's 32 MiB concurrently.
    let started = Instant::now();
    let mut tasks = Vec::new();
    for (s, addr) in addrs.iter().enumerate() {
        for _ in 0..4 {
            let addr = *addr;
            let ids = per_server_ids[s].clone();
            tasks.push(tokio::spawn(async move {
                let client = ChunkClient::new(addr);
                let requests: Vec<Request> = ids
                    .iter()
                    .map(|&extent_id| Request::ReadExtent { extent_id })
                    .collect();
                let responses = client.call_many(&requests, 8).await.unwrap();
                responses.len() * CHUNK
            }));
        }
    }
    let mut total = 0usize;
    for task in tasks {
        total += task.await.unwrap();
    }
    let elapsed = started.elapsed();
    println!(
        "bench: 16-conn contention: {:.0} MiB/s ({elapsed:?} for {} MiB, 4 servers)",
        mib(total as f64 / elapsed.as_secs_f64()),
        (total as u64) >> 20
    );

    for shutdown in &shutdowns {
        let _ = shutdown.send(true);
    }
    drop(servers);
}
