//! S2 gate proptest: random Put/Get/Delete/List/Reopen sequences against a
//! small arena, checked against a model map. After every reopen (and at the
//! end) the three gate properties must hold:
//!   (a) every live chunk in the model reads back byte-exact,
//!   (b) `list()` keys == model keys,
//!   (c) deleted/never-written ids are `NotFound`.
//!
//! Case count: `PROPTEST_CASES` env (default 8 — CI runs every test in <5 s).
//! Deep gate runs use `scripts/gate-arena-proptest.sh` (256 cases).

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use plfs_arena::{Arena, ArenaError, ChunkId, MkfsConfig, PutOutcome};
use proptest::prelude::*;

const TOTAL: u64 = 12 << 20;
const L_SLOT: u32 = 256 << 10;
const S_SLOT: u32 = 16 << 10;
const L_CAP: usize = L_SLOT as usize - 64;
const S_CAP: usize = S_SLOT as usize - 64;
const ID_POOL: usize = 10;

static NEXT: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone)]
enum Op {
    Put {
        id_idx: usize,
        version: u64,
        payload: Vec<u8>,
    },
    /// Exact duplicate of the stored (version, payload) — must be a NoOp.
    PutAgain {
        id_idx: usize,
    },
    Get {
        id_idx: usize,
    },
    Delete {
        id_idx: usize,
        /// Use the model's exact version when the chunk is live.
        exact: bool,
        version: u64,
    },
    List,
    Reopen,
}

fn op_strategy() -> impl Strategy<Value = Op> {
    let payload = prop_oneof![
        7 => prop::collection::vec(any::<u8>(), 0..=S_CAP),
        2 => prop::collection::vec(any::<u8>(), (S_CAP + 1)..=L_CAP),
        1 => prop::collection::vec(any::<u8>(), (L_CAP + 1)..=(L_CAP + 2048)),
    ];
    prop_oneof![
        5 => (any::<proptest::sample::Index>(), any::<u64>(), payload).prop_map(
            |(i, v, payload)| Op::Put {
                id_idx: i.index(ID_POOL),
                version: v % 4,
                payload,
            }
        ),
        2 => any::<proptest::sample::Index>()
            .prop_map(|i| Op::PutAgain { id_idx: i.index(ID_POOL) }),
        3 => any::<proptest::sample::Index>()
            .prop_map(|i| Op::Get { id_idx: i.index(ID_POOL) }),
        2 => (any::<proptest::sample::Index>(), any::<bool>(), any::<u64>()).prop_map(
            |(i, exact, v)| Op::Delete {
                id_idx: i.index(ID_POOL),
                exact,
                version: v % 4,
            }
        ),
        1 => Just(Op::List),
        1 => Just(Op::Reopen),
    ]
}

struct CaseFile(PathBuf);

impl CaseFile {
    fn new() -> Self {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
        std::fs::create_dir_all(&dir).expect("tmpdir");
        Self(dir.join(format!("plfs-arena-gate-{}-{n}", std::process::id())))
    }
}

impl Drop for CaseFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// The three gate properties, checked after every reopen.
fn gate_assert(arena: &mut Arena, ids: &[ChunkId], model: &HashMap<usize, (u64, Vec<u8>)>) {
    // (a) every live chunk in the model reads back byte-exact
    for (idx, (version, payload)) in model {
        let (v, got) = arena
            .get(&ids[*idx])
            .expect("live chunk must be readable after reopen");
        assert_eq!(&v, version, "version mismatch for id {}", ids[*idx]);
        assert_eq!(&got, payload, "payload mismatch for id {}", ids[*idx]);
    }
    // (b) list() keys == model keys
    let listed: HashSet<ChunkId> = arena.list().into_iter().map(|m| m.chunk_id).collect();
    let want: HashSet<ChunkId> = model.keys().map(|&i| ids[i]).collect();
    assert_eq!(listed, want, "list() keys != model keys");
    // (c) deleted/never-written ids are NotFound
    for (idx, id) in ids.iter().enumerate() {
        if !model.contains_key(&idx) {
            assert!(
                matches!(arena.get(id), Err(ArenaError::NotFound(_))),
                "id {id} must be NotFound"
            );
        }
    }
}

fn run_case(ops: Vec<Op>) {
    let file = CaseFile::new();
    let cfg = MkfsConfig {
        total_bytes: TOTAL,
        l_fraction: 0.90,
        l_slot_size: L_SLOT,
        s_slot_size: S_SLOT,
    };
    Arena::mkfs(&file.0, &cfg).expect("mkfs");
    let mut arena = Arena::open(&file.0).expect("open");
    let ids: Vec<ChunkId> = (0..ID_POOL).map(|_| ChunkId::new_v7()).collect();
    let mut model: HashMap<usize, (u64, Vec<u8>)> = HashMap::new();
    for op in ops {
        match op {
            Op::Put {
                id_idx,
                version,
                payload,
            } => {
                let result = arena.put(ids[id_idx], version, &payload);
                match (result, model.get(&id_idx)) {
                    (Ok(PutOutcome::Stored), None) => {
                        model.insert(id_idx, (version, payload));
                    }
                    (Ok(PutOutcome::Stored), Some(_)) => {
                        panic!("Stored over a live id (must be NoOp/AlreadyExists)")
                    }
                    (Ok(PutOutcome::NoOp), Some((v, p))) => {
                        assert_eq!((*v, p), (version, &payload), "NoOp must be identical");
                    }
                    (Ok(PutOutcome::NoOp), None) => panic!("NoOp for unknown id"),
                    (Err(ArenaError::AlreadyExists(_)), Some(_)) => {}
                    (Err(ArenaError::AlreadyExists(_)), None) => {
                        panic!("AlreadyExists for unknown id")
                    }
                    (Err(ArenaError::Oversize { .. }), None) => {
                        assert!(payload.len() > L_CAP, "Oversize within capacity");
                    }
                    (Err(ArenaError::Oversize { .. }), Some(_)) => {
                        panic!("Oversize on live id (must be AlreadyExists)")
                    }
                    (Err(ArenaError::OutOfSpace { .. }), None) => {}
                    (Err(ArenaError::OutOfSpace { .. }), Some(_)) => {
                        panic!("OutOfSpace on live id (must be AlreadyExists)")
                    }
                    (Err(err), _) => panic!("unexpected put error: {err:?}"),
                }
            }
            Op::PutAgain { id_idx } => {
                let Some((version, payload)) = model.get(&id_idx).cloned() else {
                    continue;
                };
                assert_eq!(
                    arena.put(ids[id_idx], version, &payload).expect("put"),
                    PutOutcome::NoOp
                );
            }
            Op::Get { id_idx } => match (arena.get(&ids[id_idx]), model.get(&id_idx)) {
                (Ok((v, got)), Some((mv, mp))) => {
                    assert_eq!((v, &got), (*mv, mp));
                }
                (Err(ArenaError::NotFound(_)), None) => {}
                (res, want) => panic!("get mismatch: {res:?} vs model {want:?}"),
            },
            Op::Delete {
                id_idx,
                exact,
                version,
            } => {
                let req = if exact {
                    model.get(&id_idx).map_or(version, |m| m.0)
                } else {
                    version
                };
                let deleted = arena.delete(&ids[id_idx], req).expect("delete");
                match model.get(&id_idx) {
                    Some((v, _)) if *v == req => {
                        assert!(deleted, "exact-version delete must succeed");
                        model.remove(&id_idx);
                    }
                    _ => assert!(!deleted, "delete must be false (absent/version mismatch)"),
                }
            }
            Op::List => {
                let listed: HashSet<ChunkId> =
                    arena.list().into_iter().map(|m| m.chunk_id).collect();
                let want: HashSet<ChunkId> = model.keys().map(|&i| ids[i]).collect();
                assert_eq!(listed, want);
            }
            Op::Reopen => {
                arena.close().expect("close");
                arena = Arena::open(&file.0).expect("reopen");
                gate_assert(&mut arena, &ids, &model);
            }
        }
    }
    arena.close().expect("close");
    let mut arena = Arena::open(&file.0).expect("final reopen");
    gate_assert(&mut arena, &ids, &model);
}

fn proptest_cases() -> u32 {
    std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(proptest_cases()))]
    #[test]
    fn gate_random_ops_reopen_consistent(ops in prop::collection::vec(op_strategy(), 20..60)) {
        run_case(ops);
    }
}
