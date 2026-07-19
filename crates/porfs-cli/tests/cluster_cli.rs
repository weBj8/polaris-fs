//! Cluster CLI integration: `porfs mkfs --chunks` + `porfs mds-check`
//! (no --data) over real loopback chunkserver subprocesses, plus the
//! cluster argument-validation matrix (no servers needed for the
//! pre-network rejections).

use std::process::{Child, Command, Output};
use std::time::{Duration, Instant};

use porfs_rpc::ChunkClient;
use tempfile::TempDir;

/// A chunkserver subprocess, killed on drop (panic paths included).
struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn spawn_chunkserver(device: &std::path::Path, listen: std::net::SocketAddr) -> ChildGuard {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_porfs"));
    cmd.arg("chunkserver")
        .arg("--device")
        .arg(device)
        .arg("--listen")
        .arg(listen.to_string())
        .arg("--size")
        .arg("128MiB");
    ChildGuard(cmd.spawn().expect("spawn porfs chunkserver"))
}

fn reserve_addr() -> std::net::SocketAddr {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);
    addr
}

/// Temp dir on the build filesystem when `CARGO_TARGET_TMPDIR` is set
/// (real disk; /tmp is tmpfs on the dev box — device files stay off it).
fn test_dir() -> TempDir {
    match std::env::var("CARGO_TARGET_TMPDIR") {
        Ok(dir) => tempfile::tempdir_in(dir).unwrap(),
        Err(_) => tempfile::tempdir().unwrap(),
    }
}

async fn wait_ready(addr: std::net::SocketAddr) {
    let client = ChunkClient::new(addr);
    let started = Instant::now();
    loop {
        match tokio::time::timeout(Duration::from_secs(2), client.stats()).await {
            Ok(Ok(_)) => return,
            _ if started.elapsed() > Duration::from_secs(15) => {
                panic!("chunkserver did not come up on {addr}")
            }
            _ => tokio::time::sleep(Duration::from_millis(100)).await,
        }
    }
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_porfs"))
        .args(args)
        .output()
        .expect("run porfs")
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn cluster_mkfs_and_mds_check_happy_path() {
    let dir = test_dir();
    let addrs: Vec<_> = (0..4).map(|_| reserve_addr()).collect();
    let _servers: Vec<ChildGuard> = addrs
        .iter()
        .enumerate()
        .map(|(i, addr)| spawn_chunkserver(&dir.path().join(format!("dev{i}.img")), *addr))
        .collect();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        for addr in &addrs {
            wait_ready(*addr).await;
        }
    });

    let meta = dir.path().join("meta.redb");
    let chunks = format!(
        "{}@r1,{}@r1,{}@r2,{}@r2",
        addrs[0], addrs[1], addrs[2], addrs[3]
    );

    // Format: replicas=2 across two racks.
    let output = run(&[
        "mkfs",
        "--meta",
        meta.to_str().unwrap(),
        "--chunks",
        &chunks,
        "--replicas",
        "2",
    ]);
    assert!(output.status.success(), "mkfs: {}", stderr(&output));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("created PolarisFS cluster MDS"), "{stdout}");
    assert!(stdout.contains("chunkservers: 4"), "{stdout}");
    assert!(stdout.contains("replicas:     2"), "{stdout}");

    // Re-format is refused (the meta file exists).
    let output = run(&[
        "mkfs",
        "--meta",
        meta.to_str().unwrap(),
        "--chunks",
        &chunks,
    ]);
    assert!(!output.status.success(), "re-mkfs must fail");

    // Self-check over the cluster data plane (no --data).
    let output = run(&["mds-check", "--meta", meta.to_str().unwrap()]);
    assert!(output.status.success(), "mds-check: {}", stderr(&output));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("mds-check: OK"), "{stdout}");
    assert!(stdout.contains("inodes:           1"), "{stdout}"); // root only
    assert!(stdout.contains("extent_map_rows:  0"), "{stdout}");
    assert!(stdout.contains("orphans_repaired: 0"), "{stdout}");

    // replicas=2 with every member in one rack is refused by the
    // placement invariant (surfaced from Mds::format_cluster).
    let one_rack = format!(
        "{}@r1,{}@r1,{}@r1,{}@r1",
        addrs[0], addrs[1], addrs[2], addrs[3]
    );
    let output = run(&[
        "mkfs",
        "--meta",
        dir.path().join("one-rack.redb").to_str().unwrap(),
        "--chunks",
        &one_rack,
        "--replicas",
        "2",
    ]);
    assert!(!output.status.success(), "single-rack replicas=2 must fail");
    assert!(
        stderr(&output).contains("at least two racks"),
        "stderr must name the rack violation: {}",
        stderr(&output)
    );
}

#[test]
fn cluster_arg_validation() {
    let dir = test_dir();
    let meta = dir.path().join("meta.redb");
    let meta = meta.to_str().unwrap();
    let device = dir.path().join("data.img");
    let device = device.to_str().unwrap();

    // mkfs: --size/--device with --chunks.
    let output = run(&[
        "mkfs",
        "--meta",
        meta,
        "--device",
        device,
        "--size",
        "1GiB",
        "--chunks",
        "127.0.0.1:9101",
    ]);
    assert!(!output.status.success());
    assert!(
        stderr(&output).contains("not used with --chunks"),
        "{}",
        stderr(&output)
    );

    // mkfs: --chunks without --meta.
    let output = run(&["mkfs", "--chunks", "127.0.0.1:9101"]);
    assert!(!output.status.success());
    assert!(stderr(&output).contains("--chunks requires --meta"));

    // mkfs: --replicas without --chunks.
    let output = run(&[
        "mkfs",
        "--device",
        device,
        "--size",
        "1GiB",
        "--replicas",
        "2",
    ]);
    assert!(!output.status.success());
    assert!(stderr(&output).contains("only meaningful with --chunks"));

    // mkfs: unparseable member names the bad token.
    let output = run(&["mkfs", "--meta", meta, "--chunks", "bogus-member"]);
    assert!(!output.status.success());
    assert!(
        stderr(&output).contains("bogus-member"),
        "{}",
        stderr(&output)
    );

    // mkfs: replicas outside 1|2.
    let output = run(&[
        "mkfs",
        "--meta",
        meta,
        "--chunks",
        "127.0.0.1:9101",
        "--replicas",
        "3",
    ]);
    assert!(!output.status.success());
    assert!(stderr(&output).contains("must be 1 or 2"));

    // mkfs: neither device+size nor --chunks.
    let output = run(&["mkfs", "--meta", meta]);
    assert!(!output.status.success());
    assert!(stderr(&output).contains("requires --device and --size"));

    // mount: --chunks + --data are mutually exclusive.
    let output = run(&[
        "mount",
        "--meta",
        meta,
        "--data",
        device,
        "--chunks",
        "127.0.0.1:9101",
        "--mountpoint",
        dir.path().to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert!(stderr(&output).contains("mutually exclusive"));

    // mount: --replicas without --chunks.
    let output = run(&[
        "mount",
        "--meta",
        meta,
        "--data",
        device,
        "--mountpoint",
        dir.path().to_str().unwrap(),
        "--replicas",
        "2",
    ]);
    assert!(!output.status.success());
    assert!(stderr(&output).contains("only meaningful with --chunks"));

    // mount: --format takes no size with --chunks.
    let output = run(&[
        "mount",
        "--meta",
        meta,
        "--mountpoint",
        dir.path().to_str().unwrap(),
        "--format",
        "4GiB",
        "--chunks",
        "127.0.0.1:9101",
    ]);
    assert!(!output.status.success());
    assert!(stderr(&output).contains("takes no size with --chunks"));

    // mount: --format requires a size in local mode.
    let output = run(&[
        "mount",
        "--meta",
        meta,
        "--data",
        device,
        "--mountpoint",
        dir.path().to_str().unwrap(),
        "--format",
    ]);
    assert!(!output.status.success());
    assert!(stderr(&output).contains("--format requires a size"));
}
