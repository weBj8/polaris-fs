//! kill -9 crash-recovery harness (P2 gate): a parent test spawns this same
//! test binary as a child process (gated by `PORFS_CRASH_CHILD=1`), lets it
//! append and sync for a random 10-80ms, then SIGKILLs it mid-flight. The
//! parent then mounts the device and verifies the durability horizon: every
//! extent the child reported as `committed` is readable, CRC-valid, and
//! byte-equal to the deterministic pattern; anything beyond the horizon may
//! exist (then it must also be CRC-valid — never garbage) or be salvaged away.
//!
//! Iteration count: `PORFS_CRASH_ITERS` (default 30; the soak script sets
//! 1000). Run in release mode: `cargo test --release -p porfs-store crash`.
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use porfs_store::{ExtentStore, StoreError};

const MIB: u64 = 1 << 20;
/// Device size per iteration: the child can never fill this inside 80ms.
const DEV_SIZE: u64 = 192 * MIB;
const ENV_CHILD: &str = "PORFS_CRASH_CHILD";
const ENV_DEV: &str = "PORFS_CRASH_DEV";
const ENV_SEED: &str = "PORFS_CRASH_SEED";
const ENV_ITERS: &str = "PORFS_CRASH_ITERS";

/// xorshift64* deterministic randomness: reproducible runs, no dependencies.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

/// Deterministic extent content: xorshift fill seeded by the extent id, so
/// the verifier regenerates expectations without any child-side state.
fn pattern(id: u64) -> Vec<u8> {
    let len = 512 + (id % 8) as usize * 512;
    let mut rng = Rng(id.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let mut out = vec![0u8; len];
    let mut chunks = out.chunks_exact_mut(8);
    for chunk in &mut chunks {
        chunk.copy_from_slice(&rng.next().to_le_bytes());
    }
    let rem = chunks.into_remainder();
    if !rem.is_empty() {
        rem.copy_from_slice(&rng.next().to_le_bytes()[..rem.len()]);
    }
    out
}

/// Device files live on the build filesystem (real disk, O_DIRECT-capable).
fn test_dir() -> tempfile::TempDir {
    match std::env::var("CARGO_TARGET_TMPDIR") {
        Ok(dir) => tempfile::tempdir_in(dir).unwrap(),
        Err(_) => tempfile::tempdir().unwrap(),
    }
}

/// Child mode: append deterministic extents, syncing every 3-9 appends, and
/// report each durability horizon to the parent as `committed <id>` on
/// stdout. Runs until killed, the device fills up, or an I/O fails.
#[test]
fn crash_child() {
    if std::env::var(ENV_CHILD).is_err() {
        return; // not spawned as a crash child: nothing to do
    }
    let dev = PathBuf::from(std::env::var(ENV_DEV).expect("child needs PORFS_CRASH_DEV"));
    let seed: u64 = std::env::var(ENV_SEED)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let mut rng = Rng(seed | 1);
    let mut store = ExtentStore::open(&dev).expect("child opens device");
    let mut pending = 0u64;
    let mut cadence = 3 + rng.next() % 7;
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    loop {
        let id = store.next_extent_id();
        let data = pattern(id);
        if store.append(1, 0, &data).is_err() {
            break; // device full or I/O error: exit quietly, work is done
        }
        pending += 1;
        if pending >= cadence {
            if store.sync().is_err() {
                break;
            }
            // The sync completed: id is on the durable side of the horizon.
            let _ = writeln!(out, "committed {id}");
            let _ = out.flush();
            pending = 0;
            cadence = 3 + rng.next() % 7;
        }
    }
}

#[test]
fn crash_kill9_loop() {
    if std::env::var(ENV_CHILD).is_ok() {
        return; // we are the child: do not recurse
    }
    let iters: u32 = std::env::var(ENV_ITERS)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);
    let dir = test_dir();
    let mut rng = Rng(0xC0FF_EE11);
    let mut used_ckpt = 0u32;
    for i in 0..iters {
        let dev = dir.path().join(format!("crash-{i}.img"));
        {
            let _store = ExtentStore::create(&dev, DEV_SIZE).unwrap();
        }
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args(["crash_child", "--exact", "--nocapture", "--test-threads=1"])
            .env(ENV_CHILD, "1")
            .env(ENV_DEV, &dev)
            .env(ENV_SEED, (i as u64 + 1).to_string())
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn crash child");
        std::thread::sleep(Duration::from_millis(10 + rng.next() % 71));
        let _ = child.kill(); // SIGKILL; may already have exited (fine)
        let output = child.wait_with_output().expect("reap crash child");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let last_committed: Option<u64> = stdout
            .lines()
            .filter_map(|line| line.strip_prefix("committed "))
            .filter_map(|id| id.trim().parse().ok())
            .next_back();
        verify_device(&dev, last_committed, i);
        let store = ExtentStore::open(&dev).unwrap();
        if store.mount_used_checkpoint() {
            used_ckpt += 1;
        }
        drop(store);
        // Per-iteration cleanup keeps peak disk usage at one device image:
        // leftover images accumulate ~40MiB each and fill tmpfs-backed tempdirs.
        std::fs::remove_file(&dev)
            .unwrap_or_else(|e| panic!("iter {i}: remove {}: {e}", dev.display()));
        if iters >= 100 && (i + 1) % 100 == 0 {
            eprintln!("[crash_kill9_loop] {}/{} iterations ok", i + 1, iters);
        }
    }
    eprintln!("[crash_kill9_loop] {iters} iterations ok (checkpoint mounts: {used_ckpt})");
}

/// Post-kill verification of one device (format.md §6 durability horizon).
fn verify_device(dev: &Path, last_committed: Option<u64>, iter: u32) {
    let mut store = ExtentStore::open(dev)
        .unwrap_or_else(|e| panic!("iter {iter}: device at {} must mount: {e}", dev.display()));
    let Some(last) = last_committed else {
        return; // killed before the first sync: nothing was ever confirmed
    };
    // Every id <= last committed must be readable and byte-exact.
    for chunk_start in (0..=last).step_by(64) {
        let chunk_end = (chunk_start + 63).min(last);
        let ids: Vec<u64> = (chunk_start..=chunk_end).collect();
        let got = store.read_batch(&ids).unwrap_or_else(|e| {
            panic!("iter {iter}: confirmed ids {chunk_start}..={chunk_end} must be readable: {e}")
        });
        for (id, data) in ids.iter().zip(got.iter()) {
            assert_eq!(
                *data,
                pattern(*id),
                "iter {iter}: confirmed extent {id} content mismatch"
            );
        }
    }
    // Ids beyond the horizon were never confirmed: they may be salvaged away
    // (UnknownExtent) or survive. A survivor must be CRC-clean and byte-exact;
    // a CRC failure is also a legal outcome (a torn, unconfirmed record being
    // *detected* — that is the checksum working, not garbage being returned).
    for id in (last + 1)..store.next_extent_id() {
        match store.read(id) {
            Ok(data) => assert_eq!(
                data,
                pattern(id),
                "iter {iter}: surviving extent {id} content mismatch"
            ),
            Err(StoreError::UnknownExtent(_)) | Err(StoreError::CorruptData(_)) => {}
            Err(e) => panic!("iter {iter}: unexpected error reading extent {id}: {e}"),
        }
    }
}
