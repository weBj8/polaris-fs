//! S4: space reclamation (design doc §6.4) — delete returns blocks to the
//! host via hole punching, and `sparsify` reclaims after bitmap-only
//! deletes. Assertions use real allocated blocks (`st_blocks`), never
//! apparent size.

use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use plfs_arena::{Arena, ChunkId, MkfsConfig, SlotClass};

const MIB: u64 = 1 << 20;
/// Slack for superblock + bitmap copies + fs metadata.
const EPSILON: u64 = 8 * MIB;

static NEXT: AtomicU64 = AtomicU64::new(0);

struct CaseFile(PathBuf);

impl CaseFile {
    fn new() -> Self {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
        std::fs::create_dir_all(&dir).expect("tmpdir");
        Self(dir.join(format!("plfs-arena-space-{}-{n}", std::process::id())))
    }
}

impl Drop for CaseFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn blocks(path: &std::path::Path) -> u64 {
    std::fs::metadata(path)
        .map(|m| m.blocks() * 512)
        .unwrap_or(0)
}

/// Xorshift64 stream — not compressible, so block counts stay honest.
fn payload(len: usize) -> Vec<u8> {
    let mut x = 0x9E37_79B9_7F4A_7C15u64;
    let mut out = Vec::with_capacity(len);
    while out.len() < len {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        out.extend_from_slice(&x.to_le_bytes());
    }
    out.truncate(len);
    out
}

fn mk_arena(file: &CaseFile) -> Arena {
    Arena::mkfs(
        &file.0,
        &MkfsConfig {
            total_bytes: 256 * MIB,
            l_fraction: 0.90,
            l_slot_size: 1 << 20,
            s_slot_size: 64 << 10,
        },
    )
    .expect("mkfs");
    Arena::open(&file.0).expect("open")
}

/// Fill with `mib` 1-MiB L-chunks; return (id, version) for exact deletes.
fn fill(arena: &mut Arena, mib: u64) -> Vec<(ChunkId, u64)> {
    let cap = arena.geometry().capacity(SlotClass::L) as usize;
    let data = payload(cap);
    let mut out = Vec::new();
    for i in 0..mib {
        let id = ChunkId::new_v7();
        arena.put(id, i, &data).expect("put");
        out.push((id, i));
    }
    out
}

#[test]
fn delete_returns_blocks_to_host() {
    let file = CaseFile::new();
    let mut arena = mk_arena(&file);
    if !arena.punch_ok() {
        eprintln!("skip: backend cannot punch");
        return;
    }
    let baseline = blocks(&file.0);
    let chunks = fill(&mut arena, 96);
    let grown = blocks(&file.0);
    assert!(
        grown >= baseline + 90 * MIB,
        "expected >=90 MiB growth, baseline={baseline} grown={grown}"
    );
    for (id, v) in chunks {
        assert!(arena.delete(&id, v).expect("delete"));
    }
    let after = blocks(&file.0);
    assert!(
        after <= baseline + EPSILON,
        "delete must return blocks: baseline={baseline} after={after}"
    );
}

#[test]
fn sparsify_reclaims_bitmap_only_deletes() {
    let file = CaseFile::new();
    let mut arena = mk_arena(&file);
    if !arena.punch_ok() {
        eprintln!("skip: backend cannot punch");
        return;
    }
    let baseline = blocks(&file.0);
    let chunks = fill(&mut arena, 64);
    arena.set_punch_enabled(false);
    for (id, v) in chunks {
        assert!(arena.delete(&id, v).expect("delete"));
    }
    let retained = blocks(&file.0);
    assert!(
        retained >= baseline + 60 * MIB,
        "bitmap-only delete must retain blocks: baseline={baseline} retained={retained}"
    );
    arena.set_punch_enabled(true);
    let report = arena.sparsify().expect("sparsify");
    assert!(report.punched);
    assert!(
        report.bytes_reclaimed >= 60 * MIB,
        "reclaimed {} bytes",
        report.bytes_reclaimed
    );
    let after = blocks(&file.0);
    assert!(
        after <= baseline + EPSILON,
        "sparsify must return blocks: baseline={baseline} after={after}"
    );
    let second = arena.sparsify().expect("sparsify again");
    assert_eq!(second.bytes_reclaimed, 0, "sparsify must be idempotent");
    arena.close().expect("close");
    let mut arena = Arena::open(&file.0).expect("reopen");
    assert!(
        arena.is_empty(),
        "sparsify must punch stale headers too — no resurrection"
    );
    let id = ChunkId::new_v7();
    arena.put(id, 1, &payload(128)).expect("put after reopen");
}

#[test]
fn sparsify_noop_when_punch_disabled() {
    let file = CaseFile::new();
    let mut arena = mk_arena(&file);
    arena.set_punch_enabled(false);
    let report = arena.sparsify().expect("sparsify");
    assert!(!report.punched);
    assert_eq!(report.free_ranges_punched, 0);
    assert_eq!(report.bytes_reclaimed, 0);
}

#[test]
fn put_reuses_slots_after_sparsify() {
    let file = CaseFile::new();
    let mut arena = mk_arena(&file);
    if !arena.punch_ok() {
        eprintln!("skip: backend cannot punch");
        return;
    }
    let chunks = fill(&mut arena, 32);
    for (id, v) in chunks {
        assert!(arena.delete(&id, v).expect("delete"));
    }
    arena.sparsify().expect("sparsify");
    let id = ChunkId::new_v7();
    let data = payload(1000);
    arena.put(id, 7, &data).expect("put after sparsify");
    let (version, got) = arena.get(&id).expect("get");
    assert_eq!(version, 7);
    assert_eq!(got, data);
}
