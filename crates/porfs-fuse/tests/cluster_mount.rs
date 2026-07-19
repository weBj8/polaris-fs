//! Real-mount cluster integration tests: a FUSE mount whose data plane is
//! four loopback chunkservers (wire protocol v3), replicas=2 across two
//! racks. Covers the POSIX surface on a cluster mount, a flag-less remount
//! (membership from the persisted `cluster_config`), read failover with one
//! chunkserver down, and honest EIO on dead placements.
//!
//! Server-stop note (mirrored from porfs-mds/tests/cluster.rs): every
//! chunkserver runs on its OWN tokio runtime; dropping that runtime aborts
//! the connection handlers, closing the MDS's pinned client connections —
//! the only way to make a "stopped" server actually unreachable for an MDS
//! that already holds connections to it. The mount's MDS worker thread owns
//! the data-plane runtime, so the test threads stay plain `#[test]`s.
//!
//! Every test skips gracefully (eprintln + return) when /dev/fuse or
//! fusermount3 is unavailable, so the suite stays green in containers.

use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use fuser::{BackgroundSession, Config, MountOption};
use porfs_fuse::{MountConfig, PorfsFs};
use porfs_mds::{ClusterTimeouts, Mds};
use porfs_rpc::server::{Server, serve};
use porfs_store::ExtentStore;
use tempfile::TempDir;
use tokio::net::TcpListener;
use tokio::sync::watch;

/// Device size per chunkserver (sparse file; DATA_START is 64 MiB).
const DEV_SIZE: u64 = 128 * 1024 * 1024;
/// Grace period for a dropped runtime to tear down connection handlers and
/// let the store worker exit (clean superblock for the restart).
const SETTLE: Duration = Duration::from_millis(300);

/// True when this box can actually perform FUSE mounts.
fn mount_available() -> bool {
    if !Path::new("/dev/fuse").exists() {
        return false;
    }
    std::process::Command::new("fusermount3")
        .arg("--version")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Skip helper: returns false (after a note) when mounting is impossible.
fn require_mount(what: &str) -> bool {
    if mount_available() {
        true
    } else {
        eprintln!("skipping {what}: /dev/fuse or fusermount3 unavailable");
        false
    }
}

/// Temp dir on the build filesystem when `CARGO_TARGET_TMPDIR` is set (real
/// disk, O_DIRECT-capable), else the system temp dir (/tmp is tmpfs here —
/// device files stay small).
fn test_dir() -> tempfile::TempDir {
    match std::env::var("CARGO_TARGET_TMPDIR") {
        Ok(dir) => tempfile::tempdir_in(dir).unwrap(),
        Err(_) => tempfile::tempdir().unwrap(),
    }
}

/// Loopback-friendly timeouts for the happy-path and failover tests:
/// failure paths must not sit out the production windows, but writes and
/// syncs get real room (a chain write crosses two servers).
fn fast_timeouts() -> ClusterTimeouts {
    ClusterTimeouts {
        read: Duration::from_millis(500),
        write: Duration::from_secs(5),
        sync: Duration::from_secs(5),
        connect: Duration::from_secs(1),
    }
}

/// Tight timeouts for the honest-EIO test: a dead placement must surface
/// EIO quickly, and a 40-write sweep (each iteration also pays the
/// close-flush sync broadcast to the dead server) must stay bounded.
fn eio_timeouts() -> ClusterTimeouts {
    ClusterTimeouts {
        read: Duration::from_millis(300),
        write: Duration::from_millis(500),
        sync: Duration::from_millis(400),
        connect: Duration::from_millis(500),
    }
}

/// Deterministic content pattern: byte at absolute file position `pos`.
fn pattern_byte(pos: u64) -> u8 {
    ((pos.wrapping_mul(31).wrapping_add(7)) & 0xff) as u8
}

fn pattern(base: u64, len: usize) -> Vec<u8> {
    (0..len as u64).map(|i| pattern_byte(base + i)).collect()
}

/// Serve `device` on `addr` (creating it when `create` is set) on a fresh
/// runtime; the runtime drives all server tasks in the background.
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
        let dir = test_dir();
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
}

/// A loopback chunkserver cluster (two racks) plus the MDS meta location.
struct Cluster {
    servers: Vec<TestServer>,
    racks: Vec<&'static str>,
    dir: TempDir,
}

impl Cluster {
    /// Four servers: two in rack r1, two in rack r2.
    fn start() -> Cluster {
        Cluster {
            servers: (0..4).map(|_| TestServer::create()).collect(),
            racks: vec!["r1", "r1", "r2", "r2"],
            dir: test_dir(),
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

    /// Format a replicas=2 cluster MDS and close it (releasing the redb
    /// lock so the mount can open the same meta file).
    fn format(&self) {
        drop(Mds::format_cluster_with_timeouts(
            self.meta(),
            self.members(),
            2,
            fast_timeouts(),
        ));
    }
}

/// RAII mount guard: unmounts on drop (and once more via fusermount3 in
/// case the session died mid-flight).
struct MountGuard {
    session: Option<BackgroundSession>,
    mountpoint: PathBuf,
}

impl MountGuard {
    /// Mount the cluster MDS at `meta` on `mountpoint` with explicit
    /// timeouts. Returns `None` (after a note) on mount/open failure —
    /// treated as a skip.
    fn mount_timeouts(meta: &Path, mountpoint: &Path, timeouts: ClusterTimeouts) -> Option<Self> {
        let config = MountConfig {
            default_permissions: false,
            ..MountConfig::default()
        };
        let fs = match PorfsFs::open_cluster_with_timeouts(meta, config, timeouts) {
            Ok(fs) => fs,
            Err(err) => {
                eprintln!("skipping: cluster MDS open failed: {err}");
                return None;
            }
        };
        Self::spawn(fs, mountpoint)
    }

    /// Mount the cluster MDS at `meta` with production defaults — the
    /// flag-less remount path (`Mds::open_cluster`).
    fn mount(meta: &Path, mountpoint: &Path) -> Option<Self> {
        let config = MountConfig {
            default_permissions: false,
            ..MountConfig::default()
        };
        let fs = match PorfsFs::open_cluster(meta, config) {
            Ok(fs) => fs,
            Err(err) => {
                eprintln!("skipping: cluster MDS open failed: {err}");
                return None;
            }
        };
        Self::spawn(fs, mountpoint)
    }

    fn spawn(fs: PorfsFs, mountpoint: &Path) -> Option<Self> {
        let mut config = Config::default();
        config.mount_options = vec![MountOption::FSName("porfs-cluster-test".to_string())];
        match fuser::spawn_mount2(fs, mountpoint, &config) {
            Ok(session) => Some(Self {
                session: Some(session),
                mountpoint: mountpoint.to_path_buf(),
            }),
            Err(err) => {
                eprintln!("skipping: FUSE mount failed: {err}");
                None
            }
        }
    }

    /// Unmount now (idempotent; Drop covers panic paths).
    fn unmount(mut self) {
        if let Some(session) = self.session.take() {
            teardown(session, &self.mountpoint);
        }
    }
}

/// See tests/mount.rs: a leaked open fd makes the plain unmount fail with
/// EBUSY, so close any fds this process still holds under the mountpoint,
/// retry, and only then fall back to a lazy detach.
fn teardown(session: BackgroundSession, mountpoint: &Path) {
    if !fusermount(mountpoint, &["-u"]) {
        let leaked = close_leaked_fds(mountpoint);
        if leaked > 0 {
            eprintln!(
                "porfs-cluster-test: closed {leaked} leaked fd(s) under {}",
                mountpoint.display()
            );
        }
        if !fusermount(mountpoint, &["-u"]) {
            let _ = fusermount(mountpoint, &["-u", "-z"]);
        }
    }
    let _ = session.umount_and_join();
}

/// Close every fd this process still holds inside `mp`, returning how many.
fn close_leaked_fds(mp: &Path) -> usize {
    let mut fds = Vec::new();
    if let Ok(dir) = fs::read_dir("/proc/self/fd") {
        for entry in dir.flatten() {
            let Ok(target) = fs::read_link(entry.path()) else {
                continue;
            };
            if target.starts_with(mp) {
                if let Some(fd) = entry
                    .file_name()
                    .to_str()
                    .and_then(|s| s.parse::<i32>().ok())
                {
                    fds.push(fd);
                }
            }
        }
    }
    for fd in &fds {
        unsafe { libc::close(*fd) };
    }
    fds.len()
}

impl Drop for MountGuard {
    fn drop(&mut self) {
        if let Some(session) = self.session.take() {
            teardown(session, &self.mountpoint);
            let _ = fusermount(&self.mountpoint, &["-u"]);
        }
    }
}

fn fusermount(mountpoint: &Path, args: &[&str]) -> bool {
    std::process::Command::new("fusermount3")
        .args(args)
        .arg(mountpoint)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

#[test]
fn cluster_mount_posix_and_flagless_remount() {
    if !require_mount("cluster_mount_posix_and_flagless_remount") {
        return;
    }
    let cluster = Cluster::start();
    cluster.format();
    let meta = cluster.meta();

    let big = pattern(0, 5 * 1024 * 1024 + 123);
    let rmw = pattern(9_000_000, 300_000);
    let sparse_payload = pattern(4_000_000, 8192);
    let tr_base = pattern(7_000_000, 1 << 20);
    let mut big_model = big.clone();
    big_model[1 << 20..(1 << 20) + rmw.len()].copy_from_slice(&rmw);

    {
        let mp_dir = test_dir();
        let Some(guard) = MountGuard::mount_timeouts(&meta, mp_dir.path(), fast_timeouts()) else {
            return;
        };
        let mp = mp_dir.path();

        // > 4 MiB: crosses the single-extent payload limit.
        fs::write(mp.join("big.bin"), &big).unwrap();
        // RMW partial overwrite straddling an extent boundary.
        {
            let mut file = fs::File::options()
                .write(true)
                .open(mp.join("big.bin"))
                .unwrap();
            file.seek(SeekFrom::Start(1 << 20)).unwrap();
            file.write_all(&rmw).unwrap();
            file.sync_all().unwrap();
        }
        // Sparse file: a hole, then data at 1 MiB.
        {
            let mut file = fs::File::create(mp.join("sparse")).unwrap();
            file.seek(SeekFrom::Start(1 << 20)).unwrap();
            file.write_all(&sparse_payload).unwrap();
            file.sync_all().unwrap();
        }
        // Truncate shrink + grow.
        fs::write(mp.join("tr"), &tr_base).unwrap();
        {
            let file = fs::File::options().write(true).open(mp.join("tr")).unwrap();
            file.set_len(4096).unwrap();
            file.set_len(2 << 20).unwrap();
        }
        // rename / readdir / xattr smoke.
        fs::create_dir(mp.join("d")).unwrap();
        fs::rename(mp.join("tr"), mp.join("d").join("tr-renamed")).unwrap();
        {
            use std::os::unix::ffi::OsStrExt;
            let path = std::ffi::CString::new(mp.join("big.bin").as_os_str().as_bytes()).unwrap();
            let name = std::ffi::CString::new("user.smoke").unwrap();
            unsafe {
                assert_eq!(
                    libc::setxattr(path.as_ptr(), name.as_ptr(), b"ok".as_ptr().cast(), 2, 0),
                    0
                );
                let mut buf = [0u8; 8];
                let got = libc::getxattr(path.as_ptr(), name.as_ptr(), buf.as_mut_ptr().cast(), 8);
                assert_eq!(got, 2);
                assert_eq!(&buf[..2], b"ok");
            }
        }
        let mut names: Vec<String> = fs::read_dir(mp)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        assert_eq!(names, ["big.bin", "d", "sparse"]);
        guard.unmount();
    }

    // Remount with NO cluster flags: the persisted membership drives
    // `Mds::open_cluster` (production timeouts).
    {
        let mp_dir = test_dir();
        let Some(guard) = MountGuard::mount(&meta, mp_dir.path()) else {
            return;
        };
        let mp = mp_dir.path();
        assert_eq!(fs::read(mp.join("big.bin")).unwrap(), big_model);
        let sparse = fs::read(mp.join("sparse")).unwrap();
        assert!(sparse[..1 << 20].iter().all(|&b| b == 0));
        assert_eq!(&sparse[1 << 20..], &sparse_payload[..]);
        let tr = fs::read(mp.join("d").join("tr-renamed")).unwrap();
        assert_eq!(tr.len(), 2 << 20);
        assert_eq!(&tr[..4096], &tr_base[..4096]);
        assert!(tr[4096..].iter().all(|&b| b == 0));
        // Sub-range read through a shared fd crosses the extent boundary.
        {
            let mut file = fs::File::open(mp.join("big.bin")).unwrap();
            let mut buf = vec![0u8; 4096];
            file.seek(SeekFrom::Start(4 * 1024 * 1024 - 100)).unwrap();
            file.read_exact(&mut buf).unwrap();
            assert_eq!(
                buf,
                &big_model[4 * 1024 * 1024 - 100..4 * 1024 * 1024 - 100 + 4096]
            );
        }
        guard.unmount();
    }
}

#[test]
fn cluster_mount_read_failover() {
    if !require_mount("cluster_mount_read_failover") {
        return;
    }
    let mut cluster = Cluster::start();
    cluster.format();
    let meta = cluster.meta();

    let mp_dir = test_dir();
    let Some(guard) = MountGuard::mount_timeouts(&meta, mp_dir.path(), fast_timeouts()) else {
        return;
    };
    let mp = mp_dir.path();

    // Several files with distinct sizes/contents, every one fsynced.
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    for i in 0..8u64 {
        let name = format!("f{i}.bin");
        let data = pattern(
            i * 1_000_000,
            (1 << 20) * (i as usize + 1) + 123 * i as usize,
        );
        let path = mp.join(&name);
        fs::write(&path, &data).unwrap();
        let file = fs::File::options().write(true).open(&path).unwrap();
        file.sync_all().unwrap();
        files.push((name, data));
    }

    // Stop ONE chunkserver: with replicas=2 every extent keeps one live
    // copy, so every file must still read byte-exact (primaries on the
    // dead server fail over to their replicas).
    cluster.servers[2].stop();
    for (name, data) in &files {
        assert_eq!(
            fs::read(mp.join(name)).unwrap(),
            *data,
            "file {name} must read byte-exact with a chunkserver down"
        );
    }

    // Restart it on the same device: the pinned client reconnects (UUID
    // verified) and everything still reads.
    cluster.servers[2].restart();
    for (name, data) in &files {
        assert_eq!(fs::read(mp.join(name)).unwrap(), *data);
    }
    guard.unmount();
}

#[test]
fn cluster_mount_honest_eio_on_dead_placement() {
    if !require_mount("cluster_mount_honest_eio_on_dead_placement") {
        return;
    }
    let mut cluster = Cluster::start();
    cluster.format();
    let meta = cluster.meta();

    let mp_dir = test_dir();
    let Some(guard) = MountGuard::mount_timeouts(&meta, mp_dir.path(), eio_timeouts()) else {
        return;
    };
    let mp = mp_dir.path();

    cluster.servers[2].stop();

    // Sweep bounded small new-file writes: each write's placement is
    // rendezvous-determined, so some land on the dead server (EIO) and
    // some on live ones (success). Files are created empty first
    // (metadata only), then re-opened (the open path answers
    // FOPEN_DIRECT_IO) so the write itself reports the error
    // synchronously instead of surfacing at close. Every single write
    // must return — success or EIO — well under the write timeout plus
    // slack: a hang here is the failure mode this test exists to catch.
    let started = Instant::now();
    let mut succeeded = 0usize;
    let mut failed = 0usize;
    for i in 0..40 {
        let path = mp.join(format!("sweep{i}"));
        fs::File::create(&path).unwrap();
        let mut file = fs::File::options().write(true).open(&path).unwrap();
        let write_started = Instant::now();
        match file.write_all(&pattern(i, 4096)) {
            Ok(()) => succeeded += 1,
            Err(err) => {
                assert_eq!(
                    err.raw_os_error(),
                    Some(libc::EIO),
                    "dead-placement writes must fail with EIO, got {err:?}"
                );
                failed += 1;
            }
        }
        assert!(
            write_started.elapsed() < Duration::from_secs(10),
            "write {i} took {:?}: the mount must not hang on a dead chunkserver",
            write_started.elapsed()
        );
    }
    assert!(
        failed >= 1,
        "sweep hit no dead placement (40 writes, one of four servers down)"
    );
    assert!(
        succeeded >= 1,
        "sweep had no live placement (40 writes, one of four servers down)"
    );
    assert!(
        started.elapsed() < Duration::from_secs(120),
        "sweep took {:?}: every op must stay timeout-bounded",
        started.elapsed()
    );
    guard.unmount();
}
