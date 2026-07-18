//! End-to-end test of the `porfs chunkserver` binary as a subprocess:
//! kill -9 after a confirmed write must lose nothing (durability horizon
//! over the wire, protocol v1 §3/§4).

use std::process::{Child, Command};
use std::time::{Duration, Instant};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use porfs_rpc::{ChunkClient, ErrorCode, RpcError};
use tempfile::TempDir;

const LISTEN: &str = "127.0.0.1:19411";

fn spawn_chunkserver(device: &std::path::Path, create: bool) -> Child {
    spawn_chunkserver_with_metrics(device, create, None)
}

fn spawn_chunkserver_with_metrics(
    device: &std::path::Path,
    create: bool,
    metrics_listen: Option<SocketAddr>,
) -> Child {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_porfs"));
    cmd.arg("chunkserver")
        .arg("--device")
        .arg(device)
        .arg("--listen")
        .arg(LISTEN);
    if let Some(metrics_listen) = metrics_listen {
        cmd.arg("--metrics-listen").arg(metrics_listen.to_string());
    }
    if create {
        cmd.arg("--size").arg("128MiB");
    }
    cmd.spawn().expect("spawn porfs chunkserver")
}

fn spawn_chunkserver_on(
    device: &std::path::Path,
    listen: SocketAddr,
    metrics_listen: Option<SocketAddr>,
    create: bool,
) -> Child {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_porfs"));
    cmd.arg("chunkserver")
        .arg("--device")
        .arg(device)
        .arg("--listen")
        .arg(listen.to_string());
    if let Some(metrics_listen) = metrics_listen {
        cmd.arg("--metrics-listen").arg(metrics_listen.to_string());
    }
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

fn reserve_addr() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);
    addr
}

fn fetch_metrics(addr: SocketAddr) -> String {
    let started = Instant::now();
    loop {
        let mut stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(err) if started.elapsed() <= Duration::from_secs(10) => {
                let _ = err;
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(err) => panic!("connect metrics endpoint {addr}: {err}"),
        };
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        write!(
            stream,
            "GET /metrics HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n"
        )
        .unwrap();
        let mut raw = String::new();
        stream.read_to_string(&mut raw).unwrap();
        if raw.contains("200 OK") {
            return raw;
        }
        assert!(
            started.elapsed() <= Duration::from_secs(10),
            "metrics endpoint on {addr} never became ready: {raw}"
        );
        std::thread::sleep(Duration::from_millis(100));
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

#[test]
fn chunkserver_metrics_and_porfsadm_status_work() {
    let dir = TempDir::new().unwrap();
    let device = dir.path().join("metrics-dev.img");
    let addr = reserve_addr();
    let metrics_addr = reserve_addr();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async move {
        let mut child = spawn_chunkserver_on(&device, addr, Some(metrics_addr), true);
        let client = wait_ready(addr).await;
        let payload = vec![0x5a; 4096];
        let id = client
            .write_extent(0xDEADBEEF, 9, 0, payload.clone())
            .await
            .unwrap();
        let back = client.read_extent(id).await.unwrap();
        assert_eq!(back, payload);
        let confirmed = client.sync().await.unwrap();
        assert!(confirmed >= id);
        let stats = client.stats().await.unwrap();
        assert_eq!(stats.extent_count, 1);
        let err = client.read_extent(u64::MAX).await.unwrap_err();
        assert!(matches!(
            err,
            RpcError::Remote {
                code: ErrorCode::NotFound,
                ..
            }
        ));

        let mut adm = Command::new(env!("CARGO_BIN_EXE_porfsadm"));
        adm.arg("status").arg("--addr").arg(addr.to_string());
        let output = adm.output().expect("run porfsadm status");
        assert!(output.status.success(), "porfsadm failed: {output:?}");
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains(&format!("chunkserver:    {addr}")));
        assert!(stdout.contains("extent_count:   1"));
        assert!(stdout.contains(&format!("confirmed_id:   {confirmed}")));

        let started = Instant::now();
        loop {
            let metrics = fetch_metrics(metrics_addr);
            if metrics.contains("porfs_rpc_requests_total{")
                && metrics.contains("porfs_rpc_request_errors_total{")
                && metrics.contains("porfs_rpc_write_bytes_total")
                && metrics.contains("porfs_rpc_read_bytes_total")
                && metrics.contains("porfs_chunkserver_store_capacity_bytes")
                && metrics.contains("porfs_chunkserver_store_live_bytes")
                && metrics.contains("porfs_chunkserver_store_confirmed_watermark")
                && metrics.contains("porfs_rpc_request_duration_seconds")
                && metrics.contains("service=\"porfs-chunkserver\"")
                && metrics.contains("op=\"write_extent\"")
                && metrics.contains("op=\"read_extent\"")
                && metrics.contains("op=\"stats\"")
                && metrics.contains("code=\"not_found\"")
            {
                break;
            }
            assert!(
                started.elapsed() <= Duration::from_secs(10),
                "metrics did not contain expected series yet"
            );
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        child.kill().unwrap();
        child.wait().unwrap();
    });
}
