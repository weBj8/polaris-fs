//! Cluster-mode MDS over loopback chunkservers (wire protocol v3):
//! byte-exact roundtrips with replicas, read failover, fsync durability
//! across chunkserver restarts, the mount-time reconcile taxonomy, and the
//! server-identity guard.
//!
//! Test topology note: every chunkserver runs on its OWN tokio runtime.
//! Dropping that runtime aborts the server's connection-handler tasks,
//! closing the MDS's pinned client connections — the only way to make a
//! "stopped" server actually unreachable for an MDS that already holds
//! connections to it (dropping just the listener would leave established
//! connections serving). The MDS itself is sync (`RemoteStore` owns an
//! internal runtime), so these are plain `#[test]`s: no ambient runtime on
//! the test thread means the data plane's `block_on` never nests.

mod common;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use porfs_mds::{ClusterTimeouts, Mds, MdsError, ROOT_INO};
use porfs_rpc::server::{Server, serve};
use porfs_store::ExtentStore;
use tempfile::TempDir;
use tokio::net::TcpListener;
use tokio::sync::watch;

use common::pattern;

const DEV_SIZE: u64 = 128 * 1024 * 1024;
/// Grace period for a dropped runtime to tear down connection handlers and
/// let the store worker exit (clean superblock for the restart).
const SETTLE: Duration = Duration::from_millis(300);

/// Loopback-friendly timeouts: failover and unreachable-server tests must
/// not sit out the production windows.
fn test_timeouts() -> ClusterTimeouts {
    ClusterTimeouts {
        read: Duration::from_millis(500),
        write: Duration::from_secs(5),
        sync: Duration::from_secs(5),
        connect: Duration::from_secs(1),
    }
}

/// Serve `device` on `addr` (creating it when `create` is set) on a fresh
/// runtime; the runtime drives all server tasks in the background. The
/// shutdown sender must stay alive as long as the server: dropping it
/// fires the server's shutdown forwarder and closes the listener.
fn spawn_server(
    device: PathBuf,
    addr: SocketAddr,
    create: bool,
) -> (tokio::runtime::Runtime, Server, watch::Sender<bool>) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let server = runtime.block_on(async move {
        let listener = TcpListener::bind(addr).await.unwrap();
        serve(
            listener,
            move || {
                if create {
                    ExtentStore::create(&device, DEV_SIZE)
                } else {
                    ExtentStore::open(&device)
                }
            },
            shutdown_rx,
        )
        .await
        .unwrap()
    });
    assert_eq!(server.addr(), addr);
    (runtime, server, shutdown_tx)
}

/// One chunkserver: device dir, address, and the runtime driving it
/// (`None` while stopped).
struct TestServer {
    _dir: TempDir,
    device: PathBuf,
    addr: SocketAddr,
    runtime: Option<tokio::runtime::Runtime>,
    server: Option<Server>,
    shutdown: Option<watch::Sender<bool>>,
}

impl TestServer {
    /// Fresh device on an ephemeral loopback port.
    fn create() -> TestServer {
        let dir = common::test_dir();
        let device = dir.path().join("dev.img");
        let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = probe.local_addr().unwrap();
        drop(probe);
        let (runtime, server, shutdown) = spawn_server(device.clone(), addr, true);
        TestServer {
            _dir: dir,
            device,
            addr,
            runtime: Some(runtime),
            server: Some(server),
            shutdown: Some(shutdown),
        }
    }

    /// Kill the server: dropping the runtime aborts its connection
    /// handlers, so every client connection closes (see the module note).
    fn stop(&mut self) {
        drop(self.server.take());
        drop(self.shutdown.take());
        drop(self.runtime.take());
        std::thread::sleep(SETTLE);
    }

    /// Restart on the same address and device (`ExtentStore::open`).
    fn restart(&mut self) {
        let (runtime, server, shutdown) = spawn_server(self.device.clone(), self.addr, false);
        self.runtime = Some(runtime);
        self.server = Some(server);
        self.shutdown = Some(shutdown);
    }

    /// Restart on the same address with a FRESH device (new store UUID).
    fn reformat(&mut self) {
        let (runtime, server, shutdown) = spawn_server(self.device.clone(), self.addr, true);
        self.runtime = Some(runtime);
        self.server = Some(server);
        self.shutdown = Some(shutdown);
    }
}

/// A loopback chunkserver cluster plus the MDS meta file location.
struct Cluster {
    servers: Vec<TestServer>,
    racks: Vec<&'static str>,
    dir: TempDir,
}

impl Cluster {
    fn start(racks: &[&'static str]) -> Cluster {
        Cluster {
            servers: racks.iter().map(|_| TestServer::create()).collect(),
            racks: racks.to_vec(),
            dir: common::test_dir(),
        }
    }

    /// The `format_cluster` member list `(addr, rack, chassis)`.
    fn members(&self) -> Vec<(SocketAddr, String, String)> {
        self.servers
            .iter()
            .enumerate()
            .map(|(i, server)| (server.addr, self.racks[i].to_string(), format!("c{i}")))
            .collect()
    }

    fn meta(&self) -> PathBuf {
        self.dir.path().join("meta.redb")
    }

    fn format(&self, replicas: usize) -> Mds {
        Mds::format_cluster_with_timeouts(self.meta(), self.members(), replicas, test_timeouts())
            .unwrap()
    }

    fn open(&self) -> Mds {
        Mds::open_cluster_with_timeouts(self.meta(), test_timeouts()).unwrap()
    }

    fn stop_all(&mut self) {
        for server in &mut self.servers {
            server.stop();
        }
    }

    fn restart_all(&mut self) {
        for server in &mut self.servers {
            server.restart();
        }
    }
}

/// Drive a file through the op matrix (multi-chunk writes, RMW, sparse,
/// punch, truncate shrink/grow), mirroring every step in `model`.
fn apply_ops(mds: &mut Mds, ino: u64, model: &mut Vec<u8>) {
    // Multi-chunk initial write (> 4 MiB: several extents).
    let base = pattern(0, (9 << 20) + 777);
    mds.write(ino, 0, &base).unwrap();
    *model = base;

    // RMW overwrite straddling extent boundaries on both ends.
    let rmw_off = (1 << 20) + 123;
    let rmw = pattern(5_000_000, 300_000);
    mds.write(ino, rmw_off as u64, &rmw).unwrap();
    model[rmw_off..rmw_off + rmw.len()].copy_from_slice(&rmw);

    // Sparse region: a hole, then data far out.
    let sparse_off = (16 << 20) as u64;
    let sparse = pattern(9_000_000, 100_000);
    mds.write(ino, sparse_off, &sparse).unwrap();
    model.resize(sparse_off as usize + sparse.len(), 0);
    model[sparse_off as usize..].copy_from_slice(&sparse);

    // Punch a hole across an extent boundary inside the first region.
    let punch_off = (2 << 20) + 999;
    let punch_len = 500_000u64;
    mds.punch_hole(ino, punch_off, punch_len).unwrap();
    model[punch_off as usize..(punch_off + punch_len) as usize].fill(0);

    // Truncate: shrink past the sparse write, then grow (zeros).
    let shrunk = (14 << 20) as u64;
    mds.setattr(
        ino,
        porfs_mds::SetAttr {
            size: Some(shrunk),
            ..Default::default()
        },
    )
    .unwrap();
    model.truncate(shrunk as usize);
    let grown = shrunk + 4096;
    mds.setattr(
        ino,
        porfs_mds::SetAttr {
            size: Some(grown),
            ..Default::default()
        },
    )
    .unwrap();
    model.resize(grown as usize, 0);
}

#[test]
fn cluster_roundtrip_byte_exact() {
    let cluster = Cluster::start(&["r1", "r2", "r3"]);
    let mut mds = cluster.format(2);
    let f = mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    let mut model = Vec::new();
    apply_ops(&mut mds, f, &mut model);
    assert_eq!(mds.read(f, 0, model.len() as u64 + 1).unwrap(), model);
    // The sparse gap between the regions reads back as zeros.
    let gap = mds.read(f, 12 << 20, 8192).unwrap();
    assert_eq!(gap, vec![0u8; 8192]);
    mds.fsync(f).unwrap();
    mds.self_check().unwrap();

    // statfs aggregates across the three servers.
    let statfs = mds.statfs().unwrap();
    assert_eq!(statfs.total_bytes, 3 * DEV_SIZE);
    assert!(statfs.live_bytes > 0);

    // Remount: the persisted membership needs no flags.
    drop(mds);
    let mut mds = cluster.open();
    assert_eq!(mds.last_reconcile_repairs(), 0);
    assert_eq!(mds.read(f, 0, model.len() as u64 + 1).unwrap(), model);
}

#[test]
fn cluster_read_failover_both_directions() {
    let mut cluster = Cluster::start(&["r1", "r2"]);
    let mut mds = cluster.format(2);
    let f = mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    let data = pattern(42, (9 << 20) + 123);
    mds.write(f, 0, &data).unwrap();
    mds.fsync(f).unwrap();

    // replicas=2 with two servers puts a copy of every extent on each
    // server, so killing either one leaves the file fully readable:
    // primaries on the dead server fail over to their replicas.
    cluster.servers[0].stop();
    assert_eq!(mds.read(f, 0, data.len() as u64 + 1).unwrap(), data);

    // Back up (the pinned client reconnects, UUID verified), then kill
    // the other one: failover must work in both directions.
    cluster.servers[0].restart();
    assert_eq!(mds.read(f, 0, data.len() as u64 + 1).unwrap(), data);
    cluster.servers[1].stop();
    assert_eq!(mds.read(f, 0, data.len() as u64 + 1).unwrap(), data);
}

#[test]
fn cluster_fsync_survives_chunkserver_restart() {
    let mut cluster = Cluster::start(&["r1", "r2"]);
    let mut mds = cluster.format(2);
    let f = mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    let data = pattern(77, (9 << 20) + 55);
    mds.write(f, 0, &data).unwrap();
    mds.fsync(f).unwrap();
    drop(mds);

    // Kill and restart BOTH servers on the same devices, then remount.
    cluster.stop_all();
    cluster.restart_all();
    let mut mds = cluster.open();
    assert_eq!(mds.last_reconcile_repairs(), 0);
    assert_eq!(mds.read(f, 0, data.len() as u64 + 1).unwrap(), data);
    mds.self_check().unwrap();
}

#[test]
fn cluster_reconcile_heals_unsynced_writes() {
    let mut cluster = Cluster::start(&["r1", "r2"]);
    let mut mds = cluster.format(2);
    let f = mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    let data = pattern(88, (5 << 20) + 7);
    mds.write(f, 0, &data).unwrap(); // metadata committed, data UNconfirmed
    drop(mds);

    // Both servers restart without ever having synced: their stores
    // salvage the unconfirmed tails, so every copy answers NotFound and
    // the reconcile turns the rows into holes.
    cluster.stop_all();
    cluster.restart_all();
    let mut mds = cluster.open();
    assert!(
        mds.last_reconcile_repairs() >= 1,
        "reconcile should drop the unsynced rows"
    );
    assert_eq!(mds.getattr(f).unwrap().size, data.len() as u64);
    assert_eq!(
        mds.read(f, 0, data.len() as u64).unwrap(),
        vec![0u8; data.len()]
    );
    mds.self_check().unwrap();
}

#[test]
fn cluster_open_fails_when_a_server_is_unreachable() {
    let mut cluster = Cluster::start(&["r1", "r2"]);
    let mut mds = cluster.format(2);
    let f = mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    let data = pattern(99, (3 << 20) + 1);
    mds.write(f, 0, &data).unwrap();
    mds.fsync(f).unwrap();
    drop(mds);

    cluster.servers[1].stop();
    let err = match Mds::open_cluster_with_timeouts(cluster.meta(), test_timeouts()) {
        Ok(_) => panic!("an unreachable server must fail the mount"),
        Err(err) => err,
    };
    assert!(
        matches!(err, MdsError::Cluster(_)),
        "unreachable server must fail the mount: {err:?}"
    );

    // Nothing was destroyed: with the server back, everything reads.
    cluster.servers[1].restart();
    let mut mds = cluster.open();
    assert_eq!(mds.last_reconcile_repairs(), 0);
    assert_eq!(mds.read(f, 0, data.len() as u64 + 1).unwrap(), data);
}

#[test]
fn cluster_identity_guard_refuses_reformatted_server() {
    let mut cluster = Cluster::start(&["r1", "r2"]);
    let mut mds = cluster.format(2);
    let f = mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    let data = pattern(111, (3 << 20) + 9);
    mds.write(f, 0, &data).unwrap();
    mds.fsync(f).unwrap();
    drop(mds);

    // Restart server 1 with a FRESH device (new store UUID) at the same
    // address: the pinned client must refuse it before any row is deleted.
    cluster.servers[1].stop();
    let original = cluster.servers[1].device.with_file_name("dev.orig");
    std::fs::rename(&cluster.servers[1].device, &original).unwrap();
    cluster.servers[1].reformat();
    let started = std::time::Instant::now();
    let err = match Mds::open_cluster_with_timeouts(cluster.meta(), test_timeouts()) {
        Ok(_) => panic!("an identity-mismatched server must fail the mount"),
        Err(err) => err,
    };
    assert!(
        matches!(err, MdsError::Cluster(_)),
        "identity mismatch must fail the mount: {err:?}"
    );
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "identity guard took {:?} (a pinned mismatch must fail fast)",
        started.elapsed()
    );

    // Restore the original device: the namespace is intact and reads back.
    cluster.servers[1].stop();
    std::fs::remove_file(&cluster.servers[1].device).unwrap();
    std::fs::rename(&original, &cluster.servers[1].device).unwrap();
    cluster.servers[1].restart();
    let mut mds = cluster.open();
    assert_eq!(mds.last_reconcile_repairs(), 0);
    assert_eq!(mds.read(f, 0, data.len() as u64 + 1).unwrap(), data);
    mds.self_check().unwrap();
}

#[test]
fn cluster_circuit_breaker_skips_dead_server_after_first_strike() {
    // Three servers so some replica pairs avoid the dead one.
    let mut cluster = Cluster::start(&["r1", "r2", "r3"]);
    let mut mds = cluster.format(2);
    let mut files = Vec::new();
    for i in 0..12u64 {
        let f = mds.create(ROOT_INO, &format!("f{i}"), 0o644, 0, 0).unwrap();
        let data = pattern(1000 + i, 100_000);
        mds.write(f, 0, &data).unwrap();
        files.push((f, data));
    }
    mds.fsync(files[0].0).unwrap();

    cluster.servers[0].stop();
    let slow = Duration::from_millis(400); // just under the 500ms test read timeout

    // Priming pass: the first read hitting the dead server's placements
    // pays the timeout and arms the breaker; every read still returns
    // byte-exact data via failover.
    let mut timeout_reads = 0usize;
    for (f, data) in &files {
        let started = std::time::Instant::now();
        assert_eq!(mds.read(*f, 0, data.len() as u64 + 1).unwrap(), *data);
        if started.elapsed() >= slow {
            timeout_reads += 1;
        }
    }
    assert!(
        timeout_reads >= 1,
        "expected at least one failover read to pay the read timeout"
    );

    // Armed: reads to the same dead placements now skip the dead server
    // outright — no per-read timeout.
    for (f, data) in &files {
        let started = std::time::Instant::now();
        assert_eq!(mds.read(*f, 0, data.len() as u64 + 1).unwrap(), *data);
        assert!(
            started.elapsed() < slow,
            "breaker-armed read took {:?} (should fail over without a timeout)",
            started.elapsed()
        );
    }

    // fsync still fails (strict barrier), via one read-timeout probe of
    // the dead server — it is never given the full sync timeout.
    let started = std::time::Instant::now();
    assert!(matches!(mds.fsync(files[0].0), Err(MdsError::Cluster(_))));
    assert!(
        started.elapsed() < Duration::from_millis(1500),
        "breaker-armed fsync took {:?} (should probe once at read timeout)",
        started.elapsed()
    );

    // Writes: pairs including the dead server fail EIO after ONE
    // read-timeout probe (never the full write window); clean pairs
    // keep accepting writes.
    let mut fast_eio = 0usize;
    let mut succeeded = 0usize;
    for i in 0..12u64 {
        let f = mds.create(ROOT_INO, &format!("w{i}"), 0o644, 0, 0).unwrap();
        let started = std::time::Instant::now();
        match mds.write(f, 0, &pattern(2000 + i, 100_000)) {
            Ok(_) => succeeded += 1,
            Err(MdsError::Cluster(_)) => {
                assert!(
                    started.elapsed() < Duration::from_millis(1500),
                    "dead-placement write took {:?} (one probe at read timeout)",
                    started.elapsed()
                );
                fast_eio += 1;
            }
            Err(err) => panic!("unexpected write error: {err:?}"),
        }
    }
    assert!(
        fast_eio >= 1,
        "expected fast-EIO writes on dead-inclusive placements"
    );
    assert!(
        succeeded >= 1,
        "expected live placements to keep accepting writes"
    );

    // After the server returns, the first write probing it clears the
    // breaker — reads and writes succeed with no wait-out-the-window.
    cluster.servers[0].restart();
    for (f, data) in &files {
        assert_eq!(mds.read(*f, 0, data.len() as u64 + 1).unwrap(), *data);
    }
    let f = mds.create(ROOT_INO, "back", 0o644, 0, 0).unwrap();
    mds.write(f, 0, &pattern(3000, 100_000)).unwrap();
    mds.fsync(f).unwrap();
}
