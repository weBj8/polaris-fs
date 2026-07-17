//! Format v2 tests: checkpoint fast-mount, group-commit policy, durability
//! horizon, automatic checkpoints, slot corruption fallback, and v1 rejection.

use std::fs::OpenOptions;
use std::os::unix::fs::FileExt;
use std::path::Path;

use porfs_format::{
    CKPT_SLOT_A_OFF, CKPT_SLOT_B_OFF, DATA_START, FORMAT_VERSION, FormatError, SB_FLAG_CLEAN,
    SB_MAGIC, Superblock,
};
use porfs_store::{ExtentStore, StoreError};

const MIB: u64 = 1 << 20;

/// Device files live on the build filesystem (real disk, O_DIRECT-capable).
fn test_dir() -> tempfile::TempDir {
    match std::env::var("CARGO_TARGET_TMPDIR") {
        Ok(dir) => tempfile::tempdir_in(dir).unwrap(),
        Err(_) => tempfile::tempdir().unwrap(),
    }
}

/// Overwrite device bytes at `offset` through the page cache and flush them,
/// so a subsequent O_DIRECT read observes the corruption.
fn corrupt(path: &Path, offset: u64, bytes: &[u8]) {
    let file = OpenOptions::new().write(true).open(path).unwrap();
    file.write_all_at(bytes, offset).unwrap();
    file.sync_all().unwrap();
}

fn patterned(len: usize, seed: u8) -> Vec<u8> {
    (0..len)
        .map(|i| seed.wrapping_mul(31).wrapping_add(i as u8))
        .collect()
}

#[test]
fn checkpoint_roundtrip() {
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let (ids, payloads, killed, tail_at_ckpt) = {
        let mut store = ExtentStore::create(&dev, 96 * MIB).unwrap();
        let payloads = [
            patterned(1000, 1),
            patterned(5000, 2),
            patterned(60_000, 3),
            patterned(200, 4),
        ];
        let mut ids = Vec::new();
        for (i, p) in payloads.iter().enumerate() {
            ids.push(store.append(7, (i * 4096) as u64, p).unwrap());
        }
        // A synced tombstone: the checkpoint must drop this entry.
        let killed = store.append(7, 99_000, b"short lived").unwrap();
        store.discard(killed).unwrap();
        store.sync().unwrap();
        assert!(store.checkpoint().unwrap(), "4 entries must fit the slot");
        assert_eq!(store.checkpoint_slot_gen(), 1);
        assert_eq!(store.checkpoint_entry_count(), 4);
        (ids, payloads, killed, store.tail())
    };
    let mut store = ExtentStore::open(&dev).unwrap();
    assert!(store.mount_used_checkpoint());
    assert_eq!(store.checkpoint_slot_gen(), 1);
    assert_eq!(store.checkpoint_covered_tail(), tail_at_ckpt);
    assert_eq!(store.checkpoint_entry_count(), 4);
    assert_eq!(store.extent_count(), 4);
    for (i, &id) in ids.iter().enumerate() {
        assert_eq!(
            store.read(id).unwrap(),
            payloads[i],
            "extent {id} byte-equal"
        );
    }
    // Checkpoints drop synced tombstones (format.md §4): still deleted, but
    // the id is gone from the rebuilt index entirely.
    assert!(matches!(
        store.read(killed),
        Err(StoreError::UnknownExtent(_))
    ));
    // Ids and tail continue correctly; appending after the reopen works.
    let id = store.append(8, 0, b"after checkpoint").unwrap();
    assert_eq!(id, 5);
    assert_eq!(store.read(id).unwrap(), b"after checkpoint");
    assert_eq!(
        store.tail(),
        tail_at_ckpt + porfs_format::extent_disk_len(16)
    );
}

#[test]
fn checkpoint_then_unsynced_tail_salvaged() {
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let synced_id;
    {
        let mut store = ExtentStore::create(&dev, 96 * MIB).unwrap();
        let base = store.append(1, 0, &patterned(3000, 5)).unwrap();
        store.sync().unwrap();
        assert!(store.checkpoint().unwrap());
        // Beyond the checkpoint: synced (must survive) + unsynced (salvaged).
        synced_id = store.append(1, 3000, &patterned(4000, 6)).unwrap();
        store.sync().unwrap();
        let unsynced = store.append(1, 7000, b"not durable").unwrap();
        assert_eq!(unsynced, synced_id + 1);
        assert_eq!(base, 0);
        // Simulate a crash: no clean-unmount superblock write on drop.
        std::mem::forget(store);
    }
    let mut store = ExtentStore::open(&dev).unwrap();
    assert!(store.mount_used_checkpoint());
    assert_eq!(store.read(0).unwrap(), patterned(3000, 5));
    assert_eq!(store.read(synced_id).unwrap(), patterned(4000, 6));
    assert!(matches!(
        store.read(synced_id + 1),
        Err(StoreError::UnknownExtent(_))
    ));
    // The salvaged horizon confirms exactly the synced state.
    assert_eq!(store.confirmed_id(), synced_id);
}

#[test]
fn newer_slot_corrupted_falls_back_to_older() {
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let first = patterned(1000, 7);
    let second = patterned(2000, 8);
    {
        let mut store = ExtentStore::create(&dev, 96 * MIB).unwrap();
        store.append(1, 0, &first).unwrap();
        store.sync().unwrap();
        assert!(store.checkpoint().unwrap()); // gen 1 -> slot A
        store.append(1, 1000, &second).unwrap();
        store.sync().unwrap();
        assert!(store.checkpoint().unwrap()); // gen 2 -> slot B
        assert_eq!(store.checkpoint_slot_gen(), 2);
    }
    // Wreck the newer slot's payload: mount must fall back to gen 1 (slot A)
    // and recover the second extent by scanning the uncovered tail.
    corrupt(&dev, CKPT_SLOT_B_OFF + 64 + 3, &[0xFF]);
    let mut store = ExtentStore::open(&dev).unwrap();
    assert!(store.mount_used_checkpoint());
    assert_eq!(store.checkpoint_slot_gen(), 1);
    assert_eq!(store.read(0).unwrap(), first);
    assert_eq!(store.read(1).unwrap(), second);
}

#[test]
fn both_slots_corrupted_full_scan() {
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let data = patterned(1500, 9);
    {
        let mut store = ExtentStore::create(&dev, 96 * MIB).unwrap();
        store.append(1, 0, &data).unwrap();
        store.sync().unwrap();
        assert!(store.checkpoint().unwrap());
    }
    corrupt(&dev, CKPT_SLOT_A_OFF + 64, &[0xAA]);
    corrupt(&dev, CKPT_SLOT_B_OFF + 64, &[0xBB]);
    let mut store = ExtentStore::open(&dev).unwrap();
    assert!(!store.mount_used_checkpoint());
    assert_eq!(store.read(0).unwrap(), data);
    assert_eq!(store.extent_count(), 1);
}

#[test]
fn checkpoint_capacity_fallback() {
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let data = patterned(900, 10);
    {
        let mut store = ExtentStore::create(&dev, 96 * MIB).unwrap();
        for _ in 0..3 {
            store.append(1, 0, &data).unwrap();
        }
        store.sync().unwrap();
        // 3 entries * 40B + 64B header = 184B > 100B cap: nothing is written.
        store.set_ckpt_slot_cap(100);
        assert!(!store.checkpoint().unwrap());
        assert_eq!(store.checkpoint_slot_gen(), 0);
    }
    let mut store = ExtentStore::open(&dev).unwrap();
    assert!(!store.mount_used_checkpoint());
    for id in 0..3 {
        assert_eq!(store.read(id).unwrap(), data);
    }
}

#[test]
fn commit_policy_auto_syncs() {
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    // Default is disabled (P1 bench semantics): no auto-sync without a policy.
    {
        let mut store = ExtentStore::create(&dev, 96 * MIB).unwrap();
        store.append(1, 0, b"x").unwrap();
        assert_eq!(store.confirmed_id(), 0);
    }
    let mut store = ExtentStore::create(&dev, 96 * MIB).unwrap();
    store.set_commit_policy(3);
    for i in 0..7u64 {
        store.append(1, i * 4096, b"payload").unwrap();
        if i == 2 {
            // Auto-sync after the 3rd append: horizon covers ids 0..=2.
            assert_eq!(store.confirmed_id(), 2);
        }
    }
    // Auto-syncs fired after appends 3 and 6; the last covered ids 0..=5.
    assert_eq!(store.confirmed_id(), 5);
    store.sync().unwrap();
    assert_eq!(store.confirmed_id(), 6);
}

#[test]
fn auto_checkpoint_every_8_syncs() {
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let covered_after_16;
    {
        let mut store = ExtentStore::create(&dev, 96 * MIB).unwrap();
        for i in 0..8u64 {
            store.append(1, i * 4096, b"n").unwrap();
            store.sync().unwrap();
        }
        // The 8th sync wrote checkpoint gen 1 covering all 8 extents.
        assert_eq!(store.checkpoint_slot_gen(), 1);
        assert_eq!(store.checkpoint_entry_count(), 8);
        assert_eq!(store.checkpoint_covered_tail(), store.tail());
        for i in 8..16u64 {
            store.append(1, i * 4096, b"n").unwrap();
            store.sync().unwrap();
        }
        assert_eq!(store.checkpoint_slot_gen(), 2);
        assert_eq!(store.checkpoint_entry_count(), 16);
        covered_after_16 = store.checkpoint_covered_tail();
        assert_eq!(covered_after_16, store.tail());
    }
    let store = ExtentStore::open(&dev).unwrap();
    assert!(store.mount_used_checkpoint());
    assert_eq!(store.checkpoint_slot_gen(), 2);
    assert_eq!(store.checkpoint_covered_tail(), covered_after_16);
    assert_eq!(store.extent_count(), 16);
    assert_eq!(store.confirmed_id(), 15);
}

#[test]
fn v1_device_rejected() {
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    // Hand-build a format v1 device: both superblocks valid except version=1.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&dev)
        .unwrap();
    file.set_len(DATA_START + 4096).unwrap();
    let mut sb = Superblock {
        magic: *SB_MAGIC,
        format_version: 1,
        flags: SB_FLAG_CLEAN,
        device_size: DATA_START + 4096,
        data_start: 1 << 20, // v1 layout
        tail: 1 << 20,
        extent_count: 0,
        sync_seq: 1,
        uuid: [0xCD; 16],
        created_at: 1_700_000_000,
        last_sync_at: 1_700_000_000,
        reserved: [0; 4004],
        sb_crc32c: 0,
    };
    sb.seal();
    file.write_all_at(sb.as_bytes(), 0).unwrap();
    file.write_all_at(sb.as_bytes(), 4096).unwrap();
    file.sync_all().unwrap();
    drop(file);
    assert!(matches!(
        ExtentStore::open(&dev),
        Err(StoreError::Format(FormatError::UnsupportedVersion(1)))
    ));
    // Sanity: the current version constant really moved on.
    assert_eq!(FORMAT_VERSION, 2);
}

/// Manual perf probe (release mode): full-scan mount vs checkpoint mount.
/// Run: `cargo test --release -p porfs-store mount_timing -- --ignored --nocapture`
#[test]
#[ignore = "manual perf probe"]
fn mount_timing_probe() {
    use std::time::Instant;
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    const N: u64 = 50_000;
    {
        let mut store = ExtentStore::create(&dev, 272 * MIB).unwrap();
        let payload = patterned(100, 1);
        let items: Vec<(u64, u64, &[u8])> = (0..1000).map(|_| (1, 0, &payload[..])).collect();
        for _ in 0..(N / 1000) {
            store.append_batch(&items).unwrap();
        }
        store.sync().unwrap();
        assert_eq!(store.extent_count(), N);
    }
    let t = Instant::now();
    let mut store = ExtentStore::open(&dev).unwrap();
    let full_scan_ms = t.elapsed().as_secs_f64() * 1e3;
    assert!(!store.mount_used_checkpoint());
    assert!(store.checkpoint().unwrap());
    drop(store);
    let t = Instant::now();
    let mut store = ExtentStore::open(&dev).unwrap();
    let ckpt_ms = t.elapsed().as_secs_f64() * 1e3;
    assert!(store.mount_used_checkpoint());
    assert_eq!(store.checkpoint_entry_count(), N as u32);
    assert_eq!(store.read(N - 1).unwrap(), patterned(100, 1));
    eprintln!(
        "mount timing ({N} extents): full-scan {full_scan_ms:.1} ms, checkpoint {ckpt_ms:.1} ms"
    );
}
