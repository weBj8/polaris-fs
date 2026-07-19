//! S3 baseline benches (criterion): put for both slot classes, get, and the
//! boot-scan reopen. Device files live in `CARGO_TARGET_TMPDIR` (real disk)
//! and are removed per bench. Run with `cargo bench -p plfs-arena`.

use std::path::{Path, PathBuf};

use criterion::{Criterion, criterion_group, criterion_main};
use plfs_arena::{Arena, ChunkId, MkfsConfig, SlotClass};

fn tmpdir() -> PathBuf {
    let dir = std::env::var("CARGO_TARGET_TMPDIR")
        .unwrap_or_else(|_| concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/tmp").to_string());
    let dir = PathBuf::from(dir);
    std::fs::create_dir_all(&dir).expect("tmpdir");
    dir
}

struct DevFile(PathBuf);

impl DevFile {
    fn new(tag: &str) -> Self {
        Self(tmpdir().join(format!("plfs-arena-bench-{tag}-{}", std::process::id())))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for DevFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn bench_put(c: &mut Criterion) {
    let dev = DevFile::new("put");
    // 16 GiB sparse: enough free slots for the whole measurement.
    Arena::mkfs(dev.path(), &MkfsConfig::new(16 << 30)).expect("mkfs");
    let mut arena = Arena::open(dev.path()).expect("open");
    let small = vec![0xABu8; 16 * 1024];
    let l_cap = arena.geometry().capacity(SlotClass::L) as usize;
    let large = vec![0xCDu8; l_cap];
    let mut group = c.benchmark_group("arena_put");
    group.bench_function("put_16KiB_S", |b| {
        b.iter(|| {
            arena.put(ChunkId::new_v7(), 1, &small).expect("put");
        })
    });
    group.bench_function("put_1MiB_L", |b| {
        b.iter(|| {
            arena.put(ChunkId::new_v7(), 1, &large).expect("put");
        })
    });
    group.finish();
}

fn bench_get(c: &mut Criterion) {
    let dev = DevFile::new("get");
    Arena::mkfs(dev.path(), &MkfsConfig::new(1 << 30)).expect("mkfs");
    let mut arena = Arena::open(dev.path()).expect("open");
    let l_cap = arena.geometry().capacity(SlotClass::L) as usize;
    let large = vec![0x5Au8; l_cap];
    let id = ChunkId::new_v7();
    arena.put(id, 1, &large).expect("put");
    c.bench_function("arena_get_1MiB", |b| {
        b.iter(|| {
            let (_, payload) = arena.get(&id).expect("get");
            std::hint::black_box(payload);
        })
    });
}

fn bench_boot_scan(c: &mut Criterion) {
    let dev = DevFile::new("scan");
    Arena::mkfs(dev.path(), &MkfsConfig::new(256 << 20)).expect("mkfs");
    {
        let mut arena = Arena::open(dev.path()).expect("open");
        let l_cap = arena.geometry().capacity(SlotClass::L) as usize;
        let large = vec![0x77u8; l_cap];
        let fill = arena.geometry().l_slot_count / 2;
        for i in 0..fill {
            arena.put(ChunkId::new_v7(), i, &large).expect("fill");
        }
        arena.close().expect("close");
    }
    c.bench_function("arena_boot_scan_256MiB_half_full", |b| {
        b.iter(|| {
            let arena = Arena::open(dev.path()).expect("open");
            std::hint::black_box(&arena);
            // Measure open only: Drop would flush the bitmaps a second time.
            arena.abandon();
        })
    });
}

criterion_group!(benches, bench_put, bench_get, bench_boot_scan);
criterion_main!(benches);
