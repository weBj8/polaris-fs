//! Simulated-crash integration tests (contract §5/§7): model-driven random
//! Put/PutAgain/Delete sequences, `Arena::abandon` (drop without the lazy
//! bitmap flush) at random op boundaries, then reopen and assert:
//!   (i)   every acked, never-deleted chunk is present byte-exact;
//!   (ii)  no false-allocated slot — every allocated chunk traces to an acked
//!         put with identical payload;
//!   (iii) `list()` is consistent with the reconciled index;
//!   (iv)  the arena is fully usable after reopen.
//! Resurrected-after-delete chunks are legal (delete has no fsync) but must
//! also read back byte-exact.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use plfs_arena::{Arena, ArenaError, ChunkId, MkfsConfig, PutOutcome};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

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
    let p = dir.join(format!("plfs-arena-crash-{tag}-{}-{n}", std::process::id()));
    (p.clone(), Cleanup(p))
}

const ID_POOL: usize = 24;

/// What the model knows about one acked chunk version.
struct Model {
    /// Live set: id -> (version, payload).
    live: HashMap<ChunkId, (u64, Vec<u8>)>,
    /// Payload of every acked (id, version), including since-deleted ones.
    acked_payloads: HashMap<(ChunkId, u64), Vec<u8>>,
}

impl Model {
    fn new() -> Self {
        Self {
            live: HashMap::new(),
            acked_payloads: HashMap::new(),
        }
    }
}

fn assert_reopened_consistent(arena: &mut Arena, model: &Model) {
    // (i) every acked, never-deleted chunk present byte-exact.
    for (id, (version, payload)) in &model.live {
        let (v, got) = arena
            .get(id)
            .unwrap_or_else(|e| panic!("live chunk {id} missing after reopen: {e}"));
        assert_eq!(&v, version, "version mismatch for {id}");
        assert_eq!(&got, payload, "payload mismatch for {id}");
    }
    // (ii) every allocated chunk traces to an acked put, payload identical.
    let listed = arena.list();
    for meta in &listed {
        let key = (meta.chunk_id, meta.version);
        let want = model
            .acked_payloads
            .get(&key)
            .unwrap_or_else(|| panic!("false-allocated slot: {key:?} was never put-acked"));
        let (_, got) = arena.get(&meta.chunk_id).expect("listed chunk readable");
        assert_eq!(
            &got, want,
            "allocated chunk {key:?} payload != acked payload"
        );
    }
    // (iii) list() keys == live set + resurrected set, and the live model is
    // exactly the allocated set minus resurrections: after in-process
    // abandon the punch already hit the fs, so deleted chunks stay gone.
    let present: HashSet<ChunkId> = listed.iter().map(|m| m.chunk_id).collect();
    let live: HashSet<ChunkId> = model.live.keys().copied().collect();
    assert_eq!(
        present, live,
        "allocated set != model live set after reopen"
    );
    // stat agrees with list.
    for meta in &listed {
        assert_eq!(arena.stat(&meta.chunk_id).as_ref(), Some(meta));
    }
    // Never-written ids are NotFound.
    let unknown = ChunkId::new_v7();
    assert!(matches!(arena.get(&unknown), Err(ArenaError::NotFound(_))));
}

fn run_crash_sequence(seed: u64, ops: usize, crash_every: std::ops::Range<usize>) {
    let (path, _guard) = temp_path("seq");
    let cfg = MkfsConfig {
        total_bytes: 16 << 20,
        l_fraction: 0.90,
        l_slot_size: 256 << 10,
        s_slot_size: 16 << 10,
    };
    Arena::mkfs(&path, &cfg).expect("mkfs");
    let mut arena = Arena::open(&path).expect("open");
    let mut rng = StdRng::seed_from_u64(seed);
    let ids: Vec<ChunkId> = (0..ID_POOL).map(|_| ChunkId::new_v7()).collect();
    let mut model = Model::new();
    let mut version = 0u64;
    let mut since_crash = rng.random_range(crash_every.clone());
    let l_cap = arena.geometry().capacity(plfs_arena::SlotClass::L) as usize;

    for _ in 0..ops {
        since_crash -= 1;
        if since_crash == 0 {
            arena.abandon();
            arena = Arena::open(&path).expect("reopen after abandon");
            assert_reopened_consistent(&mut arena, &model);
            since_crash = rng.random_range(crash_every.clone());
        }
        match rng.random_range(0..10u8) {
            // Put fresh content (new id, or re-put of a deleted id).
            0..=4 => {
                let id = ids[rng.random_range(0..ID_POOL)];
                let len = rng.random_range(0..=l_cap.min(40_000));
                let mut payload = vec![0u8; len];
                rng.fill(&mut payload[..]);
                version += 1;
                match (arena.put(id, version, &payload), model.live.get(&id)) {
                    (Ok(PutOutcome::Stored), None) => {
                        model.live.insert(id, (version, payload.clone()));
                        model.acked_payloads.insert((id, version), payload);
                    }
                    (Ok(PutOutcome::Stored), Some(_)) => panic!("Stored over live id"),
                    (Ok(PutOutcome::NoOp), _) => panic!("NoOp for fresh version"),
                    (Err(ArenaError::AlreadyExists(_)), Some(_)) => {}
                    (res, want) => panic!("unexpected put result: {res:?} vs {want:?}"),
                }
            }
            // PutAgain: idempotent re-put must be a NoOp.
            5..=6 => {
                let id = ids[rng.random_range(0..ID_POOL)];
                let Some((v, payload)) = model.live.get(&id).cloned() else {
                    continue;
                };
                assert_eq!(
                    arena.put(id, v, &payload).expect("putagain"),
                    PutOutcome::NoOp
                );
            }
            // Delete, mostly exact-version.
            7..=8 => {
                let id = ids[rng.random_range(0..ID_POOL)];
                let exact = rng.random_bool(0.8);
                let req = match (model.live.get(&id), exact) {
                    (Some((v, _)), true) => *v,
                    (Some((v, _)), false) => v.wrapping_add(1),
                    (None, _) => rng.random::<u64>(),
                };
                let deleted = arena.delete(&id, req).expect("delete");
                match model.live.get(&id) {
                    Some((v, _)) if *v == req => {
                        assert!(deleted, "exact delete must succeed");
                        model.live.remove(&id);
                    }
                    _ => assert!(!deleted),
                }
            }
            // Get verification on a live or random id.
            _ => {
                let id = ids[rng.random_range(0..ID_POOL)];
                match (arena.get(&id), model.live.get(&id)) {
                    (Ok((v, got)), Some((mv, mp))) => assert_eq!((v, &got), (*mv, mp)),
                    (Err(ArenaError::NotFound(_)), None) => {}
                    (res, want) => panic!("get mismatch: {res:?} vs {want:?}"),
                }
            }
        }
    }
    arena.abandon();
    let mut arena = Arena::open(&path).expect("final reopen");
    assert_reopened_consistent(&mut arena, &model);
    // (iv) fully usable after reopen.
    let id = ChunkId::new_v7();
    let payload = b"post-crash write".to_vec();
    arena.put(id, 1, &payload).expect("put after reopen");
    assert_eq!(arena.get(&id).expect("get after reopen").1, payload);
}

#[test]
fn crash_sequences_multiple_seeds() {
    for seed in 1..=6u64 {
        run_crash_sequence(seed, 120, 7..25);
    }
}

#[test]
fn crash_immediately_after_each_ack() {
    // Abandon right after every acked op: the tightest boundary — only the
    // fdatasynced slot bytes may survive, never the bitmap.
    let (path, _guard) = temp_path("ack");
    let cfg = MkfsConfig {
        total_bytes: 8 << 20,
        l_fraction: 0.90,
        l_slot_size: 256 << 10,
        s_slot_size: 16 << 10,
    };
    Arena::mkfs(&path, &cfg).expect("mkfs");
    let mut model = Model::new();
    let mut keep = Vec::new();
    for i in 0..6u64 {
        let id = ChunkId::new_v7();
        let payload = vec![(i as u8) + 1; 1000 * (i as usize + 1)];
        {
            let mut arena = Arena::open(&path).expect("open");
            assert_eq!(arena.put(id, i, &payload).expect("put"), PutOutcome::Stored);
            arena.abandon();
        }
        model.live.insert(id, (i, payload.clone()));
        model.acked_payloads.insert((id, i), payload);
        keep.push(id);
    }
    // Delete two chunks across separate crash boundaries.
    for (v, id) in keep[..2].iter().enumerate() {
        let mut arena = Arena::open(&path).expect("open");
        assert!(arena.delete(id, v as u64).expect("delete"));
        arena.abandon();
        model.live.remove(id);
    }
    let mut arena = Arena::open(&path).expect("reopen");
    assert_reopened_consistent(&mut arena, &model);
}
