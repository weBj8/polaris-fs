//! Public-API integration tests: end-to-end mkfs/open/put/get/stat/delete/
//! list over real device files, including the default 1 MiB / 64 KiB
//! geometry, hole-punch space reclamation, and the buffered syscall fallback
//! (forced by placing an arena on tmpfs).

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use plfs_arena::{Arena, ArenaError, ChunkId, MkfsConfig, PutOutcome, SlotClass};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Cleanup(PathBuf);

impl Drop for Cleanup {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn temp_path(tag: &str) -> (PathBuf, Cleanup) {
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(&dir).expect("tmpdir");
    let p = dir.join(format!("plfs-arena-it-{tag}-{}-{n}", std::process::id()));
    (p.clone(), Cleanup(p))
}

fn fill(seed: u8, len: usize) -> Vec<u8> {
    (0..len)
        .map(|i| seed.wrapping_mul(31).wrapping_add(i as u8))
        .collect()
}

#[test]
fn default_geometry_end_to_end() {
    let (path, _g) = temp_path("default");
    let report = Arena::mkfs(&path, &MkfsConfig::new(64 << 20)).expect("mkfs");
    assert_eq!(report.geometry.l_slot_size, 1 << 20);
    assert_eq!(report.geometry.s_slot_size, 1 << 16);
    let mut arena = Arena::open(&path).expect("open");
    assert_eq!(arena.geometry(), &report.geometry);
    let l_cap = report.geometry.capacity(SlotClass::L) as usize;
    let mut expected = Vec::new();
    for (i, len) in [0usize, 100, 65_472, 65_473, 500_000, l_cap]
        .into_iter()
        .enumerate()
    {
        let id = ChunkId::new_v7();
        let payload = fill(i as u8, len);
        assert_eq!(
            arena.put(id, i as u64, &payload).expect("put"),
            PutOutcome::Stored
        );
        expected.push((id, i as u64, payload));
    }
    arena.close().expect("close");
    let mut arena = Arena::open(&path).expect("reopen");
    assert_eq!(arena.len(), expected.len());
    for (id, version, payload) in &expected {
        let (v, got) = arena.get(id).expect("get");
        assert_eq!(&v, version);
        assert_eq!(&got, payload);
    }
}

#[test]
fn delete_punch_returns_space() {
    let (path, _g) = temp_path("punch");
    let cfg = MkfsConfig {
        total_bytes: 32 << 20,
        l_fraction: 0.9,
        l_slot_size: 256 << 10,
        s_slot_size: 16 << 10,
    };
    Arena::mkfs(&path, &cfg).expect("mkfs");
    let mut arena = Arena::open(&path).expect("open");
    if !arena.punch_ok() {
        return; // backend cannot punch; nothing to assert
    }
    use std::os::unix::fs::MetadataExt;
    let blocks = || std::fs::metadata(&path).expect("meta").blocks();
    let baseline = blocks();
    let mut ids = Vec::new();
    for i in 0..16u8 {
        let id = ChunkId::new_v7();
        arena.put(id, 1, &fill(i, 200_000)).expect("put");
        ids.push(id);
    }
    arena.sync_bitmap().expect("sync");
    let after_put = blocks();
    assert!(
        after_put > baseline,
        "data blocks allocated: {baseline} -> {after_put}"
    );
    for id in &ids {
        assert!(arena.delete(id, 1).expect("delete"));
    }
    arena.sync_bitmap().expect("sync");
    let after_delete = blocks();
    assert!(
        after_delete < after_put,
        "hole punch returns blocks: {after_put} -> {after_delete}"
    );
}

#[test]
fn random_payloads_reopen_roundtrip() {
    let (path, _g) = temp_path("rand");
    let cfg = MkfsConfig {
        total_bytes: 16 << 20,
        l_fraction: 0.9,
        l_slot_size: 256 << 10,
        s_slot_size: 16 << 10,
    };
    Arena::mkfs(&path, &cfg).expect("mkfs");
    let mut arena = Arena::open(&path).expect("open");
    let mut rng = StdRng::seed_from_u64(0xC0FFEE);
    let l_cap = arena.geometry().capacity(SlotClass::L) as usize;
    let mut expected = Vec::new();
    for i in 0..24u64 {
        let len = (rng.next_u64() as usize) % (l_cap + 1);
        let mut payload = vec![0u8; len];
        rng.fill_bytes(&mut payload);
        let id = ChunkId::new_v7();
        arena.put(id, i, &payload).expect("put");
        expected.push((id, i, payload));
    }
    arena.close().expect("close");
    let mut arena = Arena::open(&path).expect("reopen");
    for (id, version, payload) in &expected {
        let (v, got) = arena.get(id).expect("get");
        assert_eq!(&v, version);
        assert_eq!(&got, payload);
    }
}

#[test]
fn stat_list_consistency() {
    let (path, _g) = temp_path("stat");
    let cfg = MkfsConfig {
        total_bytes: 8 << 20,
        l_fraction: 0.9,
        l_slot_size: 256 << 10,
        s_slot_size: 16 << 10,
    };
    Arena::mkfs(&path, &cfg).expect("mkfs");
    let mut arena = Arena::open(&path).expect("open");
    let small = fill(1, 100);
    let large = fill(2, 100_000);
    let id_s = ChunkId::new_v7();
    let id_l = ChunkId::new_v7();
    arena.put(id_s, 7, &small).expect("put");
    arena.put(id_l, 9, &large).expect("put");
    let ms = arena.stat(&id_s).expect("stat");
    assert_eq!(ms.class, SlotClass::S);
    assert_eq!(ms.version, 7);
    assert_eq!(ms.payload_len, 100);
    assert_eq!(ms.payload_crc32c, crc32fast::hash(&small));
    let ml = arena.stat(&id_l).expect("stat");
    assert_eq!(ml.class, SlotClass::L);
    let mut listed = arena.list();
    listed.sort_by_key(|m| m.chunk_id);
    let mut want = [ms, ml];
    want.sort_by_key(|m| m.chunk_id);
    assert_eq!(listed, want);
    // Oversize payload: nothing stored, index untouched.
    let l_cap = arena.geometry().capacity(SlotClass::L) as usize;
    assert!(matches!(
        arena.put(ChunkId::new_v7(), 0, &fill(3, l_cap + 1)),
        Err(ArenaError::Oversize { .. })
    ));
    assert_eq!(arena.len(), 2);
}

/// The buffered syscall fallback (literal pwrite path, sequential driver) is
/// only reachable when O_DIRECT is unavailable — force it with tmpfs.
#[test]
fn buffered_fallback_on_tmpfs() {
    let shm = PathBuf::from("/dev/shm");
    if !shm.is_dir() {
        return;
    }
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let path = shm.join(format!("plfs-arena-it-shm-{}-{n}", std::process::id()));
    let _guard = Cleanup(path.clone());
    let cfg = MkfsConfig {
        total_bytes: 8 << 20,
        l_fraction: 0.9,
        l_slot_size: 256 << 10,
        s_slot_size: 16 << 10,
    };
    Arena::mkfs(&path, &cfg).expect("mkfs");
    let mut arena = Arena::open(&path).expect("open");
    assert!(!arena.is_direct(), "tmpfs must use the buffered fallback");
    let mut expected = Vec::new();
    for (i, len) in [0usize, 1, 448, 449, 16_000, 100_000]
        .into_iter()
        .enumerate()
    {
        let id = ChunkId::new_v7();
        let payload = fill(i as u8, len);
        arena.put(id, i as u64, &payload).expect("put");
        expected.push((id, i as u64, payload));
    }
    let gone = expected.pop().expect("one");
    assert!(arena.delete(&gone.0, gone.1).expect("delete"));
    arena.close().expect("close");
    let mut arena = Arena::open(&path).expect("reopen");
    for (id, version, payload) in &expected {
        let (v, got) = arena.get(id).expect("get");
        assert_eq!(&v, version);
        assert_eq!(&got, payload);
    }
    assert!(matches!(arena.get(&gone.0), Err(ArenaError::NotFound(_))));
}
