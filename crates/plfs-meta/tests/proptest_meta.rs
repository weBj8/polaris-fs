//! S7 gate proptest: random MetaOp sequences against the redb state machine,
//! checked op-by-op against a model. Covers the layout math (chunk_at,
//! supersede/truncate refcount accounting) and persistence (reopen == model).
//!
//! Case count: PROPTEST_CASES env (default 8 — CI runs every test < 5 s).

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use plfs_meta::{ChunkRef, Kind, MetaError, MetaOp, MetaState, OpResult, ROOT_INO};
use proptest::prelude::*;

static NEXT: AtomicU64 = AtomicU64::new(0);

const NOW: (i64, u32) = (1_750_000_000, 0);
const NAMES: [&str; 4] = ["a", "b", "c", "d"];
const SEEDS: u8 = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind2 {
    Dir,
    File,
}

#[derive(Debug, Clone)]
struct MInode {
    kind: Kind2,
    nlink: i64,
    size: u64,
}

#[derive(Debug, Default)]
struct Model {
    dentries: BTreeMap<(u64, String), u64>,
    inodes: BTreeMap<u64, MInode>,
    layouts: BTreeMap<u64, Vec<(u8, u64, u64)>>,
    refs: BTreeMap<u8, i64>,
    gc: BTreeSet<(u8, u64)>,
    next_ino: u64,
    last_seq: u64,
}

impl Model {
    fn new() -> Self {
        let mut m = Model {
            next_ino: 2,
            ..Default::default()
        };
        m.inodes.insert(
            ROOT_INO,
            MInode {
                kind: Kind2::Dir,
                nlink: 2,
                size: 0,
            },
        );
        m
    }

    fn free_inode(&mut self, ino: u64) {
        if let Some(chunks) = self.layouts.remove(&ino) {
            for (seed, version, _) in chunks {
                self.dec_ref(seed, version);
            }
        }
        self.inodes.remove(&ino);
    }

    fn dec_ref(&mut self, seed: u8, version: u64) {
        let r = self.refs.entry(seed).or_insert(0);
        *r -= 1;
        if *r <= 0 {
            self.refs.remove(&seed);
            self.gc.insert((seed, version));
        }
    }

    fn live_dirs(&self) -> Vec<u64> {
        self.inodes
            .iter()
            .filter(|(_, i)| i.kind == Kind2::Dir)
            .map(|(ino, _)| *ino)
            .collect()
    }

    fn live_files(&self) -> Vec<u64> {
        self.inodes
            .iter()
            .filter(|(_, i)| i.kind == Kind2::File)
            .map(|(ino, _)| *ino)
            .collect()
    }

    fn mkdir(&mut self, parent: u64, name: &str) -> Result<u64, MetaError> {
        if self
            .inodes
            .get(&parent)
            .is_none_or(|i| i.kind != Kind2::Dir)
        {
            return Err(MetaError::NotDir);
        }
        if self.dentries.contains_key(&(parent, name.to_string())) {
            return Err(MetaError::Exists);
        }
        let ino = self.next_ino;
        self.next_ino += 1;
        self.dentries.insert((parent, name.to_string()), ino);
        self.inodes.insert(
            ino,
            MInode {
                kind: Kind2::Dir,
                nlink: 2,
                size: 0,
            },
        );
        self.inodes.get_mut(&parent).expect("parent").nlink += 1;
        Ok(ino)
    }

    fn create(&mut self, parent: u64, name: &str) -> Result<u64, MetaError> {
        if self
            .inodes
            .get(&parent)
            .is_none_or(|i| i.kind != Kind2::Dir)
        {
            return Err(MetaError::NotDir);
        }
        if self.dentries.contains_key(&(parent, name.to_string())) {
            return Err(MetaError::Exists);
        }
        let ino = self.next_ino;
        self.next_ino += 1;
        self.dentries.insert((parent, name.to_string()), ino);
        self.inodes.insert(
            ino,
            MInode {
                kind: Kind2::File,
                nlink: 1,
                size: 0,
            },
        );
        Ok(ino)
    }

    fn unlink(&mut self, parent: u64, name: &str) -> Result<(), MetaError> {
        let Some(ino) = self.dentries.get(&(parent, name.to_string())).copied() else {
            return Err(MetaError::NotFound);
        };
        if self.inodes[&ino].kind == Kind2::Dir {
            return Err(MetaError::IsDir);
        }
        self.dentries.remove(&(parent, name.to_string()));
        let inode = self.inodes.get_mut(&ino).expect("inode");
        inode.nlink -= 1;
        if inode.nlink == 0 {
            self.free_inode(ino);
        }
        Ok(())
    }

    fn rmdir(&mut self, parent: u64, name: &str) -> Result<(), MetaError> {
        let Some(ino) = self.dentries.get(&(parent, name.to_string())).copied() else {
            return Err(MetaError::NotFound);
        };
        if self.inodes[&ino].kind != Kind2::Dir {
            return Err(MetaError::NotDir);
        }
        if self.dentries.keys().any(|(p, _)| *p == ino) {
            return Err(MetaError::NotEmpty);
        }
        self.dentries.remove(&(parent, name.to_string()));
        self.inodes.remove(&ino);
        self.inodes.get_mut(&parent).expect("parent").nlink -= 1;
        Ok(())
    }

    fn rename(&mut self, sp: u64, sn: &str, dp: u64, dn: &str) -> Result<(), MetaError> {
        let Some(src_ino) = self.dentries.get(&(sp, sn.to_string())).copied() else {
            return Err(MetaError::NotFound);
        };
        let src_dir = self.inodes[&src_ino].kind == Kind2::Dir;
        if let Some(dst_ino) = self.dentries.get(&(dp, dn.to_string())).copied() {
            if dst_ino == src_ino {
                return Ok(());
            }
            let dst_dir = self.inodes[&dst_ino].kind == Kind2::Dir;
            match (src_dir, dst_dir) {
                (true, false) => return Err(MetaError::NotDir),
                (false, true) => return Err(MetaError::IsDir),
                (true, true) => {
                    if self.dentries.keys().any(|(p, _)| *p == dst_ino) {
                        return Err(MetaError::NotEmpty);
                    }
                    self.dentries.remove(&(dp, dn.to_string()));
                    self.inodes.remove(&dst_ino);
                    self.inodes.get_mut(&dp).expect("dp").nlink -= 1;
                }
                (false, false) => {
                    self.dentries.remove(&(dp, dn.to_string()));
                    let inode = self.inodes.get_mut(&dst_ino).expect("dst");
                    inode.nlink -= 1;
                    if inode.nlink == 0 {
                        self.free_inode(dst_ino);
                    }
                }
            }
        }
        self.dentries.remove(&(sp, sn.to_string()));
        self.dentries.insert((dp, dn.to_string()), src_ino);
        if src_dir && sp != dp {
            self.inodes.get_mut(&sp).expect("sp").nlink -= 1;
            self.inodes.get_mut(&dp).expect("dp").nlink += 1;
        }
        Ok(())
    }

    fn commit_layout(
        &mut self,
        ino: u64,
        first_idx: u64,
        chunks: &[(u8, u64, u64)],
        new_size: u64,
        seq: u64,
    ) -> Result<(), MetaError> {
        if seq <= self.last_seq {
            return Ok(()); // replayed duplicate: skip
        }
        self.last_seq = seq;
        let Some(inode) = self.inodes.get(&ino) else {
            return Err(MetaError::NotFound);
        };
        if inode.kind != Kind2::File {
            return Err(MetaError::IsDir);
        }
        let layout = self.layouts.entry(ino).or_default();
        if first_idx > layout.len() as u64 {
            return Err(MetaError::Invalid("layout commit leaves a gap".to_string()));
        }
        // Net refcount accounting (matches the SUT): re-committed tail refs
        // cancel against superseded rows; only negative NET deltas GC.
        let dropped: Vec<(u8, u64, u64)> = layout.drain(first_idx as usize..).collect();
        let mut net: BTreeMap<u8, (i64, u64)> = BTreeMap::new();
        for (seed, version, _) in dropped {
            net.entry(seed).or_insert((0, version)).0 -= 1;
        }
        for &(seed, version, len) in chunks {
            self.layouts
                .get_mut(&ino)
                .expect("layout")
                .push((seed, version, len));
            net.entry(seed).or_insert((0, version)).0 += 1;
        }
        for (seed, (delta, version)) in net {
            if delta < 0 {
                for _ in 0..-delta {
                    self.dec_ref(seed, version);
                }
            } else {
                for _ in 0..delta {
                    *self.refs.entry(seed).or_insert(0) += 1;
                }
            }
        }
        self.inodes.get_mut(&ino).expect("inode").size = new_size;
        Ok(())
    }

    fn truncate(&mut self, ino: u64, size: u64) {
        let old = self.inodes[&ino].size;
        if size < old && self.inodes[&ino].kind == Kind2::File {
            let layout = self.layouts.entry(ino).or_default();
            let mut base = 0u64;
            let mut keep = layout.len();
            for (i, &(_, _, len)) in layout.iter().enumerate() {
                if base >= size {
                    keep = i;
                    break;
                }
                base += len;
            }
            let dropped: Vec<(u8, u64, u64)> = layout.split_off(keep.min(layout.len()));
            for (seed, version, _) in dropped {
                self.dec_ref(seed, version);
            }
        }
        self.inodes.get_mut(&ino).expect("inode").size = size;
    }

    fn chunk_at(&self, ino: u64, offset: u64) -> Option<(u64, (u8, u64, u64), u64)> {
        let mut base = 0u64;
        for (i, &(seed, version, len)) in self.layouts.get(&ino)?.iter().enumerate() {
            if offset < base + len {
                return Some((i as u64, (seed, version, len), offset - base));
            }
            base += len;
        }
        None
    }
}

fn err_kind(e: &MetaError) -> &'static str {
    match e {
        MetaError::NotFound => "NotFound",
        MetaError::Exists => "Exists",
        MetaError::NotEmpty => "NotEmpty",
        MetaError::NotDir => "NotDir",
        MetaError::IsDir => "IsDir",
        MetaError::Invalid(_) => "Invalid",
        _ => "Other",
    }
}

#[derive(Debug, Clone)]
enum Op {
    Mkdir {
        name: usize,
    },
    Create {
        name: usize,
    },
    Unlink {
        parent: usize,
        name: usize,
    },
    Rmdir {
        name: usize,
    },
    Rename {
        sn: usize,
        dp: usize,
        dn: usize,
    },
    CommitLayout {
        ino: usize,
        first_idx: u64,
        chunks: Vec<(u8, u64, u64)>,
        new_size: u64,
    },
    Truncate {
        ino: usize,
        size: u64,
    },
    GcDrain,
    Reopen,
}

fn op_strategy() -> impl Strategy<Value = Op> {
    let name = any::<proptest::sample::Index>().prop_map(|i| i.index(NAMES.len()));
    let chunk = (0..SEEDS, 1..4u64, 1..512u64);
    prop_oneof![
        2 => name.clone().prop_map(|name| Op::Mkdir { name }),
        3 => name.clone().prop_map(|name| Op::Create { name }),
        2 => (any::<proptest::sample::Index>(), name.clone()).prop_map(|(p, name)| Op::Unlink {
            parent: p.index(8),
            name,
        }),
        1 => name.clone().prop_map(|name| Op::Rmdir { name }),
        2 => (name.clone(), any::<proptest::sample::Index>(), name).prop_map(|(sn, dp, dn)| Op::Rename {
            sn,
            dp: dp.index(8),
            dn,
        }),
        3 => (any::<proptest::sample::Index>(), 0..5u64, prop::collection::vec(chunk, 1..4), 0..4096u64).prop_map(
            |(ino, first_idx, chunks, new_size)| Op::CommitLayout {
                ino: ino.index(16),
                first_idx,
                chunks,
                new_size,
            }
        ),
        2 => (any::<proptest::sample::Index>(), 0..4096u64).prop_map(|(ino, size)| Op::Truncate {
            ino: ino.index(16),
            size,
        }),
        1 => Just(Op::GcDrain),
        1 => Just(Op::Reopen),
    ]
}

fn cid(seed: u8) -> [u8; 16] {
    [seed; 16]
}

fn run_case(ops: Vec<Op>, case: u64) {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(&dir).expect("tmpdir");
    let path = dir.join(format!("plfs-meta-gate-{}-{case}.redb", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let mut state = MetaState::create(&path, NOW).expect("create");
    let mut model = Model::new();
    let mut commit_seq = 0u64;
    for (op_no, op) in ops.iter().enumerate() {
        match op {
            Op::Mkdir { name } => {
                let parent = *model.live_dirs().first().unwrap_or(&ROOT_INO);
                let parent = model
                    .live_dirs()
                    .get(case as usize % model.live_dirs().len().max(1))
                    .copied()
                    .unwrap_or(parent);
                let got = state.apply_at(
                    &MetaOp::Mkdir {
                        parent,
                        name: NAMES[*name].into(),
                        mode: 0o755,
                        uid: 0,
                        gid: 0,
                    },
                    NOW,
                );
                let want = model.mkdir(parent, NAMES[*name]);
                match (got, want) {
                    (Ok(OpResult::Ino(a)), Ok(b)) => assert_eq!(a, b),
                    (Err(e), Err(w)) => assert_eq!(err_kind(&e), err_kind(&w)),
                    (g, w) => panic!("mkdir mismatch: {g:?} vs {w:?}"),
                }
            }
            Op::Create { name } => {
                let dirs = model.live_dirs();
                let parent = dirs[case as usize % dirs.len()];
                let got = state.apply_at(
                    &MetaOp::CreateFile {
                        parent,
                        name: NAMES[*name].into(),
                        mode: 0o644,
                        uid: 0,
                        gid: 0,
                    },
                    NOW,
                );
                let want = model.create(parent, NAMES[*name]);
                match (got, want) {
                    (Ok(OpResult::Ino(a)), Ok(b)) => assert_eq!(a, b),
                    (Err(e), Err(w)) => assert_eq!(err_kind(&e), err_kind(&w)),
                    (g, w) => panic!("create mismatch: {g:?} vs {w:?}"),
                }
            }
            Op::Unlink { parent, name } => {
                let dirs = model.live_dirs();
                let parent = dirs[*parent % dirs.len()];
                let got = state.apply_at(
                    &MetaOp::Unlink {
                        parent,
                        name: NAMES[*name].into(),
                    },
                    NOW,
                );
                let want = model.unlink(parent, NAMES[*name]);
                match (got, want) {
                    (Ok(OpResult::None), Ok(())) => {}
                    (Err(e), Err(w)) => assert_eq!(err_kind(&e), err_kind(&w)),
                    (g, w) => panic!("unlink mismatch: {g:?} vs {w:?}"),
                }
            }
            Op::Rmdir { name } => {
                let got = state.apply_at(
                    &MetaOp::Rmdir {
                        parent: ROOT_INO,
                        name: NAMES[*name].into(),
                    },
                    NOW,
                );
                let want = model.rmdir(ROOT_INO, NAMES[*name]);
                match (got, want) {
                    (Ok(OpResult::None), Ok(())) => {}
                    (Err(e), Err(w)) => assert_eq!(err_kind(&e), err_kind(&w)),
                    (g, w) => panic!("rmdir mismatch: {g:?} vs {w:?}"),
                }
            }
            Op::Rename { sn, dp, dn } => {
                let dirs = model.live_dirs();
                let dp = dirs[*dp % dirs.len()];
                let got = state.apply_at(
                    &MetaOp::Rename {
                        src_parent: ROOT_INO,
                        src_name: NAMES[*sn].into(),
                        dst_parent: dp,
                        dst_name: NAMES[*dn].into(),
                    },
                    NOW,
                );
                let want = model.rename(ROOT_INO, NAMES[*sn], dp, NAMES[*dn]);
                match (got, want) {
                    (Ok(OpResult::None), Ok(())) => {}
                    (Err(e), Err(w)) => assert_eq!(err_kind(&e), err_kind(&w)),
                    (g, w) => panic!("rename mismatch: {g:?} vs {w:?}"),
                }
            }
            Op::CommitLayout {
                ino,
                first_idx,
                chunks,
                new_size,
            } => {
                let files = model.live_files();
                if files.is_empty() {
                    continue;
                }
                let ino = files[*ino % files.len()];
                commit_seq += 1;
                let seq = commit_seq;
                let refs: Vec<ChunkRef> = chunks
                    .iter()
                    .map(|&(seed, version, len)| ChunkRef {
                        chunk_id: cid(seed),
                        version,
                        len,
                        replicas: vec![],
                    })
                    .collect();
                let got = state.apply_at(
                    &MetaOp::CommitLayout {
                        ino,
                        first_idx: *first_idx,
                        chunks: refs,
                        new_size: *new_size,
                        seq,
                    },
                    NOW,
                );
                let want = model.commit_layout(ino, *first_idx, chunks, *new_size, seq);
                match (got, want) {
                    (Ok(OpResult::None), Ok(())) => {}
                    (Err(e), Err(w)) => assert_eq!(err_kind(&e), err_kind(&w)),
                    (g, w) => panic!("commit_layout mismatch: {g:?} vs {w:?}"),
                }
                // Layout math: sampled offsets must resolve identically.
                let size = model.inodes[&ino].size;
                for off in [
                    0u64,
                    size / 3,
                    size / 2,
                    size.saturating_sub(1),
                    size,
                    size + 100,
                ] {
                    let got = state.chunk_at(ino, off).expect("chunk_at");
                    let want = model.chunk_at(ino, off);
                    match (got, want) {
                        (None, None) => {}
                        (Some((gi, gc, gio)), Some((wi, wc, wio))) => {
                            assert_eq!(
                                (gi, gc.chunk_id, gc.version, gc.len, gio),
                                (wi, cid(wc.0), wc.1, wc.2, wio),
                                "chunk_at({off})"
                            );
                        }
                        (g, w) => panic!("chunk_at({off}) mismatch: {g:?} vs {w:?}"),
                    }
                }
            }
            Op::Truncate { ino, size } => {
                let files = model.live_files();
                if files.is_empty() {
                    continue;
                }
                let ino = files[*ino % files.len()];
                state
                    .apply_at(
                        &MetaOp::SetAttr {
                            ino,
                            size: Some(*size),
                            mode: None,
                            uid: None,
                            gid: None,
                            atime: None,
                            mtime: None,
                        },
                        NOW,
                    )
                    .expect("truncate");
                model.truncate(ino, *size);
            }
            Op::GcDrain => {
                let batch = match state
                    .apply_at(&MetaOp::GcTake { max: 10_000 }, NOW)
                    .expect("gc take")
                {
                    OpResult::GcBatch(v) => v,
                    other => panic!("expected GcBatch, got {other:?}"),
                };
                let seqs: Vec<u64> = batch.iter().map(|(s, _)| *s).collect();
                state
                    .apply_at(&MetaOp::GcDone { seqs }, NOW)
                    .expect("gc done");
                model.gc.clear();
            }
            Op::Reopen => {
                drop(state);
                state = MetaState::open(&path).expect("reopen");
            }
        }
        if op_no % 2 == 0 {
            compare_full_at(&mut state, &model, &path);
        }
    }
    drop(state);
    let mut state = MetaState::open(&path).expect("final reopen");
    compare_full_at(&mut state, &model, &path);
    let _ = std::fs::remove_file(&path);
}

fn compare_full_at(state: &mut MetaState, model: &Model, _path: &std::path::Path) {
    for (ino, mi) in &model.inodes {
        let got = state
            .getattr(*ino)
            .expect("getattr")
            .unwrap_or_else(|| panic!("inode {ino} missing"));
        let want_kind = match mi.kind {
            Kind2::Dir => Kind::Dir,
            Kind2::File => Kind::File,
        };
        assert_eq!(got.kind, want_kind, "kind for ino {ino}");
        assert_eq!(got.nlink as i64, mi.nlink, "nlink for ino {ino}");
        assert_eq!(got.size, mi.size, "size for ino {ino}");
        if mi.kind == Kind2::Dir {
            let want: BTreeSet<(String, u64)> = model
                .dentries
                .iter()
                .filter(|((p, _), _)| p == ino)
                .map(|((_, n), c)| (n.clone(), *c))
                .collect();
            let got: BTreeSet<(String, u64)> =
                state.listdir(*ino).expect("listdir").into_iter().collect();
            assert_eq!(got, want, "dentries of ino {ino}");
        } else {
            let want: Vec<(u64, ChunkRef)> = model
                .layouts
                .get(ino)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .enumerate()
                .map(|(i, (seed, version, len))| {
                    (
                        i as u64,
                        ChunkRef {
                            chunk_id: cid(seed),
                            version,
                            len,
                            replicas: vec![],
                        },
                    )
                })
                .collect();
            assert_eq!(
                state.layout(*ino).expect("layout"),
                want,
                "layout of ino {ino}"
            );
        }
    }
    for seed in 0..SEEDS {
        let want = model.refs.get(&seed).copied().unwrap_or(0).max(0) as u64;
        assert_eq!(
            state.chunk_refcount(&cid(seed)).expect("refcount"),
            want,
            "refcount of seed {seed}"
        );
    }
    let gc_got: BTreeSet<(u8, u64)> = match state.apply_at(&MetaOp::GcTake { max: 10_000 }, NOW) {
        Ok(OpResult::GcBatch(v)) => v
            .into_iter()
            .map(|(_, e)| (e.chunk_id[0], e.version))
            .collect(),
        other => panic!("gc take failed: {other:?}"),
    };
    assert_eq!(gc_got, model.gc, "gc queue contents");
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
    fn apply_ops_match_model(ops in prop::collection::vec(op_strategy(), 20..50)) {
        let case = NEXT.fetch_add(1, Ordering::Relaxed);
        run_case(ops, case);
    }
}
