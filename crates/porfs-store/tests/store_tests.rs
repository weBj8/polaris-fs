//! End-to-end tests of the extent store against real device files, including
//! on-disk corruption injection (via plain pwrite) and the buffered fallback.

use std::fs::OpenOptions;
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};

use porfs_format::{DATA_START, EXTENT_DATA_MAX, extent_disk_len};
use porfs_store::{ExtentStore, StoreError};

const MIB: u64 = 1 << 20;

/// Device files live on the build filesystem (real disk, O_DIRECT-capable)
/// rather than on the possibly-tmpfs system temp dir.
fn test_dir() -> tempfile::TempDir {
    match std::env::var("CARGO_TARGET_TMPDIR") {
        Ok(dir) => tempfile::tempdir_in(dir).unwrap(),
        Err(_) => tempfile::tempdir().unwrap(),
    }
}

/// True when the io_uring path cannot be exercised in this environment;
/// affected tests print a note and skip (the syscall fallback keeps working).
fn skip_without_uring() -> bool {
    if porfs_store::io_uring_available() {
        false
    } else {
        eprintln!("SKIP: io_uring unavailable in this environment");
        true
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
fn single_append_read_roundtrip() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let mut store = ExtentStore::create(&dev, 80 * MIB).unwrap();
    assert_eq!(store.extent_count(), 0);
    assert_eq!(store.tail(), DATA_START);

    let payload = patterned(100_000, 7);
    let id = store.append(42, 0, &payload).unwrap();
    assert_eq!(id, 0);
    assert_eq!(store.extent_count(), 1);
    assert_eq!(store.live_bytes(), 100_000);
    assert_eq!(store.read(id).unwrap(), payload);
}

#[test]
fn batch_append_read_roundtrip() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let mut store = ExtentStore::create(&dev, 128 * MIB).unwrap();

    let sizes = [
        1usize,
        63,
        64,
        65,
        1000,
        4096,
        MIB as usize,
        EXTENT_DATA_MAX as usize,
    ];
    let items: Vec<(u64, u64, Vec<u8>)> = sizes
        .iter()
        .enumerate()
        .map(|(i, &len)| (i as u64 + 1, (i * 4096) as u64, patterned(len, i as u8)))
        .collect();
    let refs: Vec<(u64, u64, &[u8])> = items
        .iter()
        .map(|(ino, off, data)| (*ino, *off, data.as_slice()))
        .collect();
    let ids = store.append_batch(&refs).unwrap();
    assert_eq!(ids, (0..sizes.len() as u64).collect::<Vec<_>>());
    assert_eq!(store.extent_count(), sizes.len() as u64);

    let all = store.read_batch(&ids).unwrap();
    for ((_, _, want), got) in items.iter().zip(all.iter()) {
        assert_eq!(got, want);
    }
    // Single reads agree, including the 4MiB maximum extent.
    for (i, &id) in ids.iter().enumerate() {
        assert_eq!(store.read(id).unwrap(), items[i].2);
    }
}

#[test]
fn oversize_append_rejected() {
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let mut store = ExtentStore::create(&dev, 80 * MIB).unwrap();
    let big = vec![0u8; EXTENT_DATA_MAX as usize + 1];
    assert!(matches!(
        store.append(1, 0, &big),
        Err(StoreError::DataTooLarge { .. })
    ));
    assert_eq!(store.extent_count(), 0);
}

#[test]
fn data_corruption_detected() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let id = {
        let mut store = ExtentStore::create(&dev, 80 * MIB).unwrap();
        let id = store.append(1, 0, &patterned(5000, 3)).unwrap();
        store.sync().unwrap();
        id
    };
    // Flip one data byte on disk: record starts at DATA_START, data at +64.
    corrupt(&dev, DATA_START + 64 + 123, &[0xFF]);
    let mut store = ExtentStore::open(&dev).unwrap();
    assert!(matches!(store.read(id), Err(StoreError::CorruptData(_))));
}

#[test]
fn salvage_on_corrupt_header() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let ids = {
        let mut store = ExtentStore::create(&dev, 80 * MIB).unwrap();
        let ids = store
            .append_batch(&[
                (1, 0, &patterned(1000, 1)[..]),
                (1, 1000, &patterned(2000, 2)[..]),
                (1, 3000, &patterned(500, 3)[..]),
            ])
            .unwrap();
        store.sync().unwrap();
        ids
    };
    // Destroy the magic of the second record; everything past it must vanish.
    let second_offset = DATA_START + extent_disk_len(1000);
    corrupt(&dev, second_offset, &[0xDE, 0xAD, 0xBE, 0xEF]);

    let mut store = ExtentStore::open(&dev).unwrap();
    assert_eq!(store.tail(), second_offset, "tail salvaged to bad record");
    assert_eq!(store.extent_count(), 1);
    assert_eq!(store.read(ids[0]).unwrap(), patterned(1000, 1));
    assert!(matches!(
        store.read(ids[1]),
        Err(StoreError::UnknownExtent(_))
    ));
    assert!(matches!(
        store.read(ids[2]),
        Err(StoreError::UnknownExtent(_))
    ));
    // The store keeps appending from the salvaged tail, reusing the space.
    // Only extent id 0 was seen before the bad record, so ids restart at 1.
    let new_id = store.append(9, 0, b"after salvage").unwrap();
    assert_eq!(new_id, 1);
    assert_eq!(store.read(new_id).unwrap(), b"after salvage");
}

#[test]
fn persistence_across_reopen() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let (ids, uuid) = {
        let mut store = ExtentStore::create(&dev, 80 * MIB).unwrap();
        let ids = store
            .append_batch(&[
                (7, 0, &patterned(3000, 11)[..]),
                (7, 3000, &patterned(6000, 12)[..]),
            ])
            .unwrap();
        store.sync().unwrap();
        (ids, store.uuid())
    };
    let mut store = ExtentStore::open(&dev).unwrap();
    assert_eq!(store.uuid(), uuid);
    assert_eq!(store.extent_count(), 2);
    assert_eq!(store.read(ids[0]).unwrap(), patterned(3000, 11));
    assert_eq!(store.read(ids[1]).unwrap(), patterned(6000, 12));
    // Ids continue monotonically after the reopen.
    let id = store.append(8, 0, b"third").unwrap();
    assert_eq!(id, 2);
    assert_eq!(store.read(id).unwrap(), b"third");
}

#[test]
fn discard_semantics() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let (keep, kill) = {
        let mut store = ExtentStore::create(&dev, 80 * MIB).unwrap();
        let keep = store.append(1, 0, b"i stay").unwrap();
        let kill = store.append(1, 6, b"i die").unwrap();
        store.discard(kill).unwrap();
        // Immediate effect: read errors, counts drop.
        assert!(matches!(store.read(kill), Err(StoreError::Tombstoned(_))));
        assert_eq!(store.extent_count(), 1);
        assert_eq!(store.live_bytes(), 6);
        assert!(matches!(
            store.discard(kill),
            Err(StoreError::Tombstoned(_))
        ));
        assert!(matches!(
            store.discard(999),
            Err(StoreError::UnknownExtent(999))
        ));
        store.sync().unwrap();
        (keep, kill)
    };
    // After reopen the tombstone is still in effect.
    let mut store = ExtentStore::open(&dev).unwrap();
    assert_eq!(store.extent_count(), 1);
    assert_eq!(store.live_bytes(), 6);
    assert_eq!(store.read(keep).unwrap(), b"i stay");
    assert!(matches!(store.read(kill), Err(StoreError::Tombstoned(_))));
}

#[test]
fn superblock_copy_a_corrupted_opens_from_b() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let id = {
        let mut store = ExtentStore::create(&dev, 80 * MIB).unwrap();
        let id = store.append(1, 0, b"durable").unwrap();
        store.sync().unwrap();
        id
    };
    corrupt(&dev, 0, b"GARBAGE!"); // kill copy A's magic
    let mut store = ExtentStore::open(&dev).unwrap();
    assert_eq!(store.read(id).unwrap(), b"durable");
    assert_eq!(store.extent_count(), 1);
}

#[test]
fn both_superblocks_corrupted_error() {
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    {
        let _store = ExtentStore::create(&dev, 80 * MIB).unwrap();
    }
    corrupt(&dev, 0, b"GARBAGE!");
    corrupt(&dev, 4096, b"GARBAGE!");
    assert!(matches!(
        ExtentStore::open(&dev),
        Err(StoreError::NoValidSuperblock)
    ));
}

#[test]
fn device_full_error() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let size = DATA_START + 2 * 4096;
    let mut store = ExtentStore::create(&dev, size).unwrap();
    store.append(1, 0, &[0u8; 100]).unwrap(); // occupies block 1
    store.append(1, 0, &[0u8; 100]).unwrap(); // occupies block 2
    assert!(matches!(
        store.append(1, 0, &[0u8; 100]),
        Err(StoreError::DeviceFull { .. })
    ));
}

#[test]
fn pool_grows_across_batches() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let mut store = ExtentStore::create(&dev, 96 * MIB).unwrap();
    // First batch is small (pool sized to one block)...
    let small = store.append(1, 0, &patterned(500, 1)).unwrap();
    // ...then a batch with mixed sizes forces the pool to grow mid-stream.
    let ids = store
        .append_batch(&[
            (1, 0, &patterned(100, 2)[..]),
            (1, 0, &patterned(2 * MIB as usize, 3)[..]),
            (1, 0, &patterned(1000, 4)[..]),
        ])
        .unwrap();
    // ...and later small appends reuse the grown pool.
    let tail_id = store.append(1, 0, b"pool reuse").unwrap();
    assert_eq!(store.read(small).unwrap(), patterned(500, 1));
    let all = store.read_batch(&ids).unwrap();
    assert_eq!(all[0], patterned(100, 2));
    assert_eq!(all[1], patterned(2 * MIB as usize, 3));
    assert_eq!(all[2], patterned(1000, 4));
    assert_eq!(store.read(tail_id).unwrap(), b"pool reuse");
    // Reopen and read everything back through a freshly built pool.
    store.sync().unwrap();
    drop(store);
    let mut store = ExtentStore::open(&dev).unwrap();
    let all = store
        .read_batch(&[small, ids[0], ids[1], ids[2], tail_id])
        .unwrap();
    assert_eq!(all[0], patterned(500, 1));
    assert_eq!(all[2], patterned(2 * MIB as usize, 3));
    assert_eq!(all[4], b"pool reuse");
}

#[test]
fn read_batch_into_roundtrip() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let mut store = ExtentStore::create(&dev, 80 * MIB).unwrap();
    let payloads = [
        patterned(100, 1),
        patterned(5000, 2),
        patterned(1_000_000, 3),
    ];
    let mut ids = Vec::new();
    for (i, p) in payloads.iter().enumerate() {
        ids.push(store.append(1, i as u64, p).unwrap());
    }
    let mut dsts: Vec<Vec<u8>> = payloads
        .iter()
        .map(|p| Vec::with_capacity(p.len()))
        .collect();
    store.read_batch_into(&ids, &mut dsts).unwrap();
    for (i, p) in payloads.iter().enumerate() {
        assert_eq!(&dsts[i], p);
        assert_eq!(dsts[i].len(), p.len());
    }
}

#[test]
fn read_batch_into_detects_corruption() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let id = {
        let mut store = ExtentStore::create(&dev, 80 * MIB).unwrap();
        let id = store.append(1, 0, &patterned(5000, 9)).unwrap();
        store.sync().unwrap();
        id
    };
    corrupt(&dev, DATA_START + 64 + 42, &[0xFF]);
    let mut store = ExtentStore::open(&dev).unwrap();
    let mut dsts = vec![Vec::with_capacity(5000)];
    assert!(matches!(
        store.read_batch_into(&[id], &mut dsts),
        Err(StoreError::CorruptData(_))
    ));
}

#[test]
fn read_batch_into_rejects_bad_dsts() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let mut store = ExtentStore::create(&dev, 80 * MIB).unwrap();
    let id = store.append(1, 0, &patterned(5000, 4)).unwrap();
    // dsts length mismatch.
    let mut none: Vec<Vec<u8>> = Vec::new();
    assert!(matches!(
        store.read_batch_into(&[id], &mut none),
        Err(StoreError::LengthMismatch { ids: 1, dsts: 0 })
    ));
    // capacity too small (need 5000).
    let mut small = vec![Vec::with_capacity(100)];
    assert!(matches!(
        store.read_batch_into(&[id], &mut small),
        Err(StoreError::CapacityTooSmall {
            id: _,
            need: 5000,
            have: 100
        })
    ));
    // len() of dst is irrelevant; capacity() is what counts.
    let mut cap_ok = vec![{
        let mut v = Vec::with_capacity(5000);
        v.resize(10, 0xAA);
        v
    }];
    store.read_batch_into(&[id], &mut cap_ok).unwrap();
    assert_eq!(cap_ok[0], patterned(5000, 4));
}

#[test]
fn read_batch_into_mixed_sizes_and_reuse() {
    if skip_without_uring() {
        return;
    }
    let dir = test_dir();
    let dev = dir.path().join("dev.img");
    let mut store = ExtentStore::create(&dev, 96 * MIB).unwrap();
    let ids = store
        .append_batch(&[
            (1, 0, &patterned(100, 5)[..]),
            (1, 0, &patterned(2 * MIB as usize, 6)[..]),
            (1, 0, &patterned(1000, 7)[..]),
        ])
        .unwrap();
    let mut dsts: Vec<Vec<u8>> = vec![
        Vec::with_capacity(2 * MIB as usize),
        Vec::with_capacity(2 * MIB as usize),
        Vec::with_capacity(2 * MIB as usize),
    ];
    for round in 0..2 {
        store.read_batch_into(&ids, &mut dsts).unwrap();
        assert_eq!(dsts[0], patterned(100, 5), "round {round}");
        assert_eq!(dsts[1], patterned(2 * MIB as usize, 6), "round {round}");
        assert_eq!(dsts[2], patterned(1000, 7), "round {round}");
    }
    // Reuse with a subset: tombstoned id must still fail the batch.
    store.discard(ids[0]).unwrap();
    assert!(matches!(
        store.read_batch_into(&ids, &mut dsts),
        Err(StoreError::Tombstoned(_))
    ));
}

#[test]
fn buffered_fallback_on_dev_shm() {
    let shm = Path::new("/dev/shm");
    if !shm.is_dir() {
        eprintln!("SKIP: /dev/shm not available");
        return;
    }
    let dev: PathBuf = shm.join(format!("porfs-test-{}", std::process::id()));
    let _ = std::fs::remove_file(&dev);
    let id = {
        let mut store = ExtentStore::create(&dev, 72 * MIB).unwrap();
        assert!(!store.is_direct(), "tmpfs must not accept O_DIRECT");
        let id = store.append(1, 0, &patterned(10_000, 5)).unwrap();
        let gone = store.append(1, 0, b"gone").unwrap();
        store.discard(gone).unwrap();
        store.sync().unwrap();
        assert_eq!(store.read(id).unwrap(), patterned(10_000, 5));
        id
    };
    let mut store = ExtentStore::open(&dev).unwrap();
    assert!(!store.is_direct());
    assert_eq!(store.read(id).unwrap(), patterned(10_000, 5));
    assert_eq!(store.extent_count(), 1);
    drop(store);
    std::fs::remove_file(&dev).unwrap();
}
