//! Loopback integration tests for the chunkserver RPC (protocol v1):
//! real TCP between two tasks, a real extent store on a temp device.

use std::net::SocketAddr;
use std::time::{Duration, Instant};

use porfs_rpc::server::{Server, serve};
use porfs_rpc::{ChunkClient, ErrorCode, RpcError};
use porfs_store::ExtentStore;
use tempfile::TempDir;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::watch;

const DEV_SIZE: u64 = 128 * 1024 * 1024;

/// One running server + client pair on a fresh device.
struct Fixture {
    _dir: TempDir,
    device: std::path::PathBuf,
    server: Server,
    client: ChunkClient,
    shutdown: watch::Sender<bool>,
}

async fn start() -> Fixture {
    let dir = tempfile::tempdir().unwrap();
    let device = dir.path().join("dev.img");
    let (listener, server, shutdown) = spawn_on(&device, false).await;
    let _ = listener;
    let client = ChunkClient::new(server.addr());
    Fixture {
        _dir: dir,
        device,
        server,
        client,
        shutdown,
    }
}

/// Bind an ephemeral loopback port and serve the device on it. `open`
/// selects open-existing vs create-fresh.
async fn spawn_on(
    device: &std::path::Path,
    open: bool,
) -> (SocketAddr, Server, watch::Sender<bool>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (server, shutdown) = serve_on(listener, device, open).await;
    (addr, server, shutdown)
}

/// Serve `device` on an already-bound listener (used for same-address
/// restarts: the client reconnects to the address it knows).
async fn spawn_on_addr(
    device: &std::path::Path,
    open: bool,
    addr: SocketAddr,
) -> (Server, watch::Sender<bool>) {
    let listener = TcpListener::bind(addr).await.unwrap();
    serve_on(listener, device, open).await
}

async fn serve_on(
    listener: TcpListener,
    device: &std::path::Path,
    open: bool,
) -> (Server, watch::Sender<bool>) {
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let device = device.to_path_buf();
    let server = serve(
        listener,
        move || {
            if open {
                ExtentStore::open(&device)
            } else {
                ExtentStore::create(&device, DEV_SIZE)
            }
        },
        shutdown_rx,
    )
    .await
    .unwrap();
    (server, shutdown_tx)
}

/// Stop a fixture: drop the client connection, signal shutdown, and give
/// the store thread a moment to exit (clean device handoff). Returns the
/// tempdir so the device file stays alive for a restart.
async fn stop(fx: Fixture) -> TempDir {
    let Fixture {
        _dir,
        device: _,
        server,
        client,
        shutdown,
    } = fx;
    drop(client);
    let _ = shutdown.send(true);
    drop(server);
    tokio::time::sleep(Duration::from_millis(300)).await;
    _dir
}

fn pattern(seed: u64, len: usize) -> Vec<u8> {
    (0..len as u64)
        .map(|i| ((seed.wrapping_mul(31).wrapping_add(i) * 7) & 0xff) as u8)
        .collect()
}

#[tokio::test]
async fn write_read_roundtrip_all_sizes() {
    let fx = start().await;
    let sizes = [1usize, 4096, 1 << 20, 4 << 20];
    for (i, size) in sizes.iter().enumerate() {
        let data = pattern(i as u64 + 1, *size);
        let id = fx
            .client
            .write_extent(0xA000 + i as u128, 7, (i as u64) << 20, data.clone())
            .await
            .unwrap();
        let back = fx.client.read_extent(id).await.unwrap();
        assert_eq!(back, data, "size {size}");
    }
    let (_, _, _, extent_count, _) = fx.client.stats().await.unwrap();
    assert_eq!(extent_count, 4);
    stop(fx).await;
}

#[tokio::test]
async fn write_id_is_idempotent() {
    let fx = start().await;
    let data = pattern(1, 100);
    let first = fx
        .client
        .write_extent(42, 7, 0, data.clone())
        .await
        .unwrap();
    let second = fx
        .client
        .write_extent(42, 7, 0, data.clone())
        .await
        .unwrap();
    assert_eq!(first, second);
    let (_, _, _, extent_count, _) = fx.client.stats().await.unwrap();
    assert_eq!(extent_count, 1, "retry must not append a duplicate");
    stop(fx).await;
}

#[tokio::test]
async fn tombstone_semantics() {
    let fx = start().await;
    let id = fx
        .client
        .write_extent(1, 7, 0, pattern(3, 64))
        .await
        .unwrap();
    fx.client.tombstone(id).await.unwrap();
    let err = fx.client.read_extent(id).await.unwrap_err();
    assert!(
        matches!(
            err,
            RpcError::Remote {
                code: ErrorCode::NotFound,
                ..
            }
        ),
        "tombstoned read: {err}"
    );
    // Idempotent: tombstoning again is fine; an unknown id is NotFound.
    fx.client.tombstone(id).await.unwrap();
    let err = fx.client.tombstone(u64::MAX).await.unwrap_err();
    assert!(matches!(
        err,
        RpcError::Remote {
            code: ErrorCode::NotFound,
            ..
        }
    ));
    stop(fx).await;
}

#[tokio::test]
async fn oversize_payload_rejected() {
    let fx = start().await;
    let big = vec![0u8; (5 << 20) as usize];
    let err = fx.client.write_extent(9, 7, 0, big).await.unwrap_err();
    assert!(
        matches!(
            err,
            RpcError::Remote {
                code: ErrorCode::BadRequest,
                ..
            }
        ),
        "oversize: {err}"
    );
    // And an empty payload is a BadRequest too.
    let err = fx
        .client
        .write_extent(10, 7, 0, Vec::new())
        .await
        .unwrap_err();
    assert!(matches!(
        err,
        RpcError::Remote {
            code: ErrorCode::BadRequest,
            ..
        }
    ));
    stop(fx).await;
}

#[tokio::test]
async fn durability_survives_restart() {
    let fx = start().await;
    let data = pattern(9, 1 << 20);
    let id = fx.client.write_extent(7, 3, 0, data.clone()).await.unwrap();
    let confirmed = fx.client.sync().await.unwrap();
    assert!(id <= confirmed);
    let device = fx.device.clone();
    let _dir = stop(fx).await;

    // Fresh server on the same device: the confirmed extent must read back.
    let (_, server, shutdown) = spawn_on(&device, true).await;
    let client = ChunkClient::new(server.addr());
    let back = client.read_extent(id).await.unwrap();
    assert_eq!(back, data);
    drop(client);
    let _ = shutdown.send(true);
    drop(server);
    tokio::time::sleep(Duration::from_millis(300)).await;
}

#[tokio::test]
async fn reconnect_semantics_match_the_contract() {
    let fx = start().await;
    let addr = fx.server.addr();
    let id = fx
        .client
        .write_extent(5, 3, 0, pattern(5, 32))
        .await
        .unwrap();
    fx.client.sync().await.unwrap();
    let device = fx.device.clone();
    let _dir = stop(fx).await;

    // With the server down, an op stays pending (backoff, no spurious
    // failure) — it must NOT complete within a short window.
    let client = ChunkClient::new(addr);
    let pending = tokio::spawn({
        let client = client.clone();
        async move { client.read_extent(id).await }
    });
    tokio::time::sleep(Duration::from_millis(400)).await;
    assert!(!pending.is_finished(), "op must wait out the server outage");

    // Server returns ON THE SAME ADDRESS: the pending op completes via a
    // transparent reconnect.
    let (server, shutdown) = spawn_on_addr(&device, true, addr).await;
    let back = tokio::time::timeout(Duration::from_secs(10), pending)
        .await
        .expect("op must complete after the server returns")
        .unwrap()
        .unwrap();
    assert_eq!(back, pattern(5, 32));
    drop(client);
    let _ = shutdown.send(true);
    drop(server);
    tokio::time::sleep(Duration::from_millis(300)).await;
}

#[tokio::test]
async fn concurrent_clients_isolated_writes() {
    let fx = start().await;
    let addr = fx.server.addr();
    let mut tasks = Vec::new();
    for c in 0..8u64 {
        tasks.push(tokio::spawn(async move {
            let client = ChunkClient::new(addr);
            let mut ids = Vec::new();
            for w in 0..32u64 {
                let data = pattern(c * 100 + w, 1024);
                let write_id = ((c as u128) << 64) | w as u128;
                let id = client
                    .write_extent(write_id, c, w << 10, data)
                    .await
                    .unwrap();
                ids.push((id, c * 100 + w));
            }
            for (id, seed) in ids {
                assert_eq!(client.read_extent(id).await.unwrap(), pattern(seed, 1024));
            }
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }
    let (_, _, _, extent_count, _) = fx.client.stats().await.unwrap();
    assert_eq!(extent_count, 8 * 32);
    stop(fx).await;
}

#[tokio::test]
async fn oversized_frame_drops_connection() {
    let fx = start().await;
    let mut raw = tokio::net::TcpStream::connect(fx.server.addr())
        .await
        .unwrap();
    let len: u32 = porfs_rpc::MAX_FRAME + 1;
    raw.write_all(&len.to_le_bytes()).await.unwrap();
    raw.write_all(&vec![0u8; 1024]).await.unwrap(); // start of the junk frame
    let mut buf = [0u8; 16];
    let reading = raw.read(&mut buf);
    let outcome = tokio::time::timeout(Duration::from_secs(5), reading).await;
    match outcome {
        Ok(Ok(0)) => {}  // closed cleanly
        Ok(Err(_)) => {} // reset
        other => panic!("server kept the oversize frame connection: {other:?}"),
    }
    drop(raw);
    stop(fx).await;
}

#[tokio::test]
async fn ops_before_hello_and_bad_version_rejected() {
    let fx = start().await;
    // Hello with a wrong version: VersionMismatch, then the server closes.
    let mut conn = tokio::net::TcpStream::connect(fx.server.addr())
        .await
        .unwrap();
    let hello = porfs_rpc::proto::encode(&porfs_rpc::proto::Request::Hello {
        protocol_version: 999,
        client_nonce: 1,
    })
    .unwrap();
    conn.write_all(&(hello.len() as u32).to_le_bytes())
        .await
        .unwrap();
    conn.write_all(&hello).await.unwrap();
    let mut len_buf = [0u8; 4];
    conn.read_exact(&mut len_buf).await.unwrap();
    let frame_len = u32::from_le_bytes(len_buf) as usize;
    let mut buf = vec![0u8; frame_len];
    conn.read_exact(&mut buf).await.unwrap();
    let response: porfs_rpc::proto::Response = porfs_rpc::proto::decode(&buf).unwrap();
    assert!(matches!(
        response,
        porfs_rpc::proto::Response::Error {
            code: ErrorCode::VersionMismatch,
            ..
        }
    ));
    drop(conn);
    stop(fx).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn many_small_ops_stay_fast() {
    let fx = start().await;
    let started = Instant::now();
    for i in 0..200u64 {
        fx.client
            .write_extent(i as u128, 1, i * 64, pattern(i, 64))
            .await
            .unwrap();
    }
    let elapsed = started.elapsed();
    assert!(
        elapsed < Duration::from_secs(10),
        "200 serialized ops took {elapsed:?}"
    );
    stop(fx).await;
}
