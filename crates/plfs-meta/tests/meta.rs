//! S7 unit tests: metadata state machine semantics.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use plfs_meta::{ChunkRef, Kind, MetaError, MetaOp, MetaState, OpResult, ROOT_INO};

static NEXT: AtomicU64 = AtomicU64::new(0);

const NOW: (i64, u32) = (1_750_000_000, 0);

fn fresh() -> (PathBuf, MetaState) {
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("plfs-meta-{}-{n}.redb", std::process::id()));
    let state = MetaState::create(&path, NOW).expect("create");
    (path, state)
}

fn cid(n: u8) -> [u8; 16] {
    [n; 16]
}

fn chunk(n: u8, version: u64, len: u64) -> ChunkRef {
    ChunkRef {
        chunk_id: cid(n),
        version,
        len,
        replicas: vec![],
    }
}

fn create_file(state: &mut MetaState, parent: u64, name: &str) -> u64 {
    match state
        .apply_at(
            &MetaOp::CreateFile {
                parent,
                name: name.into(),
                mode: 0o644,
                uid: 0,
                gid: 0,
            },
            NOW,
        )
        .expect("create file")
    {
        OpResult::Ino(ino) => ino,
        other => panic!("expected Ino, got {other:?}"),
    }
}

fn mkdir(state: &mut MetaState, parent: u64, name: &str) -> u64 {
    match state
        .apply_at(
            &MetaOp::Mkdir {
                parent,
                name: name.into(),
                mode: 0o755,
                uid: 0,
                gid: 0,
            },
            NOW,
        )
        .expect("mkdir")
    {
        OpResult::Ino(ino) => ino,
        other => panic!("expected Ino, got {other:?}"),
    }
}

#[test]
fn create_initializes_root() {
    let (path, state) = fresh();
    let root = state.getattr(ROOT_INO).expect("getattr").expect("root");
    assert_eq!(root.kind, Kind::Dir);
    assert_eq!(root.nlink, 2);
    assert!(state.listdir(ROOT_INO).expect("listdir").is_empty());
    let _ = std::fs::remove_file(path);
}

#[test]
fn mkdir_create_lookup_listdir() {
    let (path, mut state) = fresh();
    let a = mkdir(&mut state, ROOT_INO, "a");
    let f = create_file(&mut state, a, "f");
    assert_eq!(state.lookup(a, "f").expect("lookup"), Some(f));
    assert_eq!(state.lookup(a, "nope").expect("lookup"), None);
    assert_eq!(
        state.listdir(a).expect("listdir"),
        vec![("f".to_string(), f)]
    );
    create_file(&mut state, a, "b");
    create_file(&mut state, a, "z");
    let names: Vec<String> = state
        .listdir(a)
        .expect("listdir")
        .into_iter()
        .map(|e| e.0)
        .collect();
    assert_eq!(names, vec!["b", "f", "z"], "byte-ordered listing");
    let root_inode = state.getattr(ROOT_INO).expect("getattr").expect("root");
    assert_eq!(root_inode.nlink, 3, "subdir bumps parent nlink");
    let _ = std::fs::remove_file(path);
}

#[test]
fn duplicate_create_is_exists() {
    let (path, mut state) = fresh();
    create_file(&mut state, ROOT_INO, "f");
    let err = state
        .apply_at(
            &MetaOp::CreateFile {
                parent: ROOT_INO,
                name: "f".into(),
                mode: 0o644,
                uid: 0,
                gid: 0,
            },
            NOW,
        )
        .expect_err("duplicate");
    assert!(matches!(err, MetaError::Exists));
    // Failed op must not consume an inode number.
    let g = create_file(&mut state, ROOT_INO, "g");
    let f = state.lookup(ROOT_INO, "f").expect("lookup").expect("f");
    assert_eq!(g, f + 1);
    let _ = std::fs::remove_file(path);
}

#[test]
fn unlink_frees_inode_and_queues_chunks_for_gc() {
    let (path, mut state) = fresh();
    let f = create_file(&mut state, ROOT_INO, "f");
    state
        .apply_at(
            &MetaOp::CommitLayout {
                ino: f,
                first_idx: 0,
                chunks: vec![chunk(1, 7, 100), chunk(2, 7, 200)],
                new_size: 300,
                seq: 1,
            },
            NOW,
        )
        .expect("commit");
    assert_eq!(state.chunk_refcount(&cid(1)).expect("ref"), 1);
    state
        .apply_at(
            &MetaOp::Unlink {
                parent: ROOT_INO,
                name: "f".into(),
            },
            NOW,
        )
        .expect("unlink");
    assert!(state.getattr(f).expect("getattr").is_none());
    assert_eq!(state.chunk_refcount(&cid(1)).expect("ref"), 0);
    let entries = match state
        .apply_at(&MetaOp::GcTake { max: 10 }, NOW)
        .expect("gc take")
    {
        OpResult::GcBatch(v) => v,
        other => panic!("expected GcBatch, got {other:?}"),
    };
    let got: Vec<([u8; 16], u64)> = entries
        .iter()
        .map(|(_, e)| (e.chunk_id, e.version))
        .collect();
    assert_eq!(
        got,
        vec![(cid(1), 7), (cid(2), 7)],
        "exact ids + versions queued"
    );
    let seqs: Vec<u64> = entries.iter().map(|(s, _)| *s).collect();
    state
        .apply_at(&MetaOp::GcDone { seqs }, NOW)
        .expect("gc done");
    assert_eq!(state.gc_len().expect("gc len"), 0);
    let _ = std::fs::remove_file(path);
}

#[test]
fn hardlink_keeps_chunks_until_last_link() {
    let (path, mut state) = fresh();
    let f = create_file(&mut state, ROOT_INO, "f");
    state
        .apply_at(
            &MetaOp::CommitLayout {
                ino: f,
                first_idx: 0,
                chunks: vec![chunk(1, 1, 10)],
                new_size: 10,
                seq: 1,
            },
            NOW,
        )
        .expect("commit");
    state
        .apply_at(
            &MetaOp::Link {
                parent: ROOT_INO,
                name: "g".into(),
                ino: f,
            },
            NOW,
        )
        .expect("link");
    state
        .apply_at(
            &MetaOp::Unlink {
                parent: ROOT_INO,
                name: "f".into(),
            },
            NOW,
        )
        .expect("unlink f");
    assert_eq!(
        state.chunk_refcount(&cid(1)).expect("ref"),
        1,
        "still referenced"
    );
    assert_eq!(state.gc_len().expect("gc len"), 0);
    state
        .apply_at(
            &MetaOp::Unlink {
                parent: ROOT_INO,
                name: "g".into(),
            },
            NOW,
        )
        .expect("unlink g");
    assert_eq!(state.gc_len().expect("gc len"), 1);
    let _ = std::fs::remove_file(path);
}

#[test]
fn commit_layout_supersedes_tail_and_accounts_refs() {
    let (path, mut state) = fresh();
    let f = create_file(&mut state, ROOT_INO, "f");
    state
        .apply_at(
            &MetaOp::CommitLayout {
                ino: f,
                first_idx: 0,
                chunks: vec![chunk(1, 1, 100), chunk(2, 1, 100), chunk(3, 1, 100)],
                new_size: 300,
                seq: 1,
            },
            NOW,
        )
        .expect("commit 1");
    state
        .apply_at(
            &MetaOp::CommitLayout {
                ino: f,
                first_idx: 1,
                chunks: vec![chunk(9, 2, 50)],
                new_size: 150,
                seq: 2,
            },
            NOW,
        )
        .expect("commit 2");
    let layout = state.layout(f).expect("layout");
    let ids: Vec<[u8; 16]> = layout.iter().map(|(_, c)| c.chunk_id).collect();
    assert_eq!(ids, vec![cid(1), cid(9)]);
    assert_eq!(state.chunk_refcount(&cid(2)).expect("ref"), 0, "superseded");
    assert_eq!(state.chunk_refcount(&cid(3)).expect("ref"), 0, "superseded");
    assert_eq!(state.chunk_refcount(&cid(9)).expect("ref"), 1);
    assert_eq!(state.gc_len().expect("gc len"), 2);
    let gap = state
        .apply_at(
            &MetaOp::CommitLayout {
                ino: f,
                first_idx: 5,
                chunks: vec![chunk(4, 1, 1)],
                new_size: 151,
                seq: 3,
            },
            NOW,
        )
        .expect_err("gap commit");
    assert!(matches!(gap, MetaError::Invalid(_)));
    let _ = std::fs::remove_file(path);
}

#[test]
fn truncate_drops_tail_chunks_keeps_straddler() {
    let (path, mut state) = fresh();
    let f = create_file(&mut state, ROOT_INO, "f");
    state
        .apply_at(
            &MetaOp::CommitLayout {
                ino: f,
                first_idx: 0,
                chunks: vec![chunk(1, 1, 100), chunk(2, 1, 100), chunk(3, 1, 100)],
                new_size: 300,
                seq: 1,
            },
            NOW,
        )
        .expect("commit");
    let setattr = |size: u64| MetaOp::SetAttr {
        ino: f,
        size: Some(size),
        mode: None,
        uid: None,
        gid: None,
        atime: None,
        mtime: None,
    };
    state.apply_at(&setattr(150), NOW).expect("truncate 150");
    assert_eq!(state.layout(f).expect("layout").len(), 2, "straddler kept");
    assert_eq!(state.chunk_refcount(&cid(3)).expect("ref"), 0);
    state.apply_at(&setattr(100), NOW).expect("truncate 100");
    assert_eq!(state.layout(f).expect("layout").len(), 1);
    assert_eq!(state.chunk_refcount(&cid(2)).expect("ref"), 0);
    state.apply_at(&setattr(10_000), NOW).expect("extend");
    assert_eq!(
        state.layout(f).expect("layout").len(),
        1,
        "extend is sparse"
    );
    let _ = std::fs::remove_file(path);
}

#[test]
fn chunk_at_maps_offsets() {
    let (path, mut state) = fresh();
    let f = create_file(&mut state, ROOT_INO, "f");
    state
        .apply_at(
            &MetaOp::CommitLayout {
                ino: f,
                first_idx: 0,
                chunks: vec![chunk(1, 1, 100), chunk(2, 1, 200), chunk(3, 1, 50)],
                new_size: 350,
                seq: 1,
            },
            NOW,
        )
        .expect("commit");
    let at = |off: u64| {
        state
            .chunk_at(f, off)
            .expect("chunk_at")
            .map(|(i, c, intra)| (i, c.chunk_id, intra))
    };
    assert_eq!(at(0), Some((0, cid(1), 0)));
    assert_eq!(at(99), Some((0, cid(1), 99)));
    assert_eq!(at(100), Some((1, cid(2), 0)));
    assert_eq!(at(299), Some((1, cid(2), 199)));
    assert_eq!(at(300), Some((2, cid(3), 0)));
    assert_eq!(at(349), Some((2, cid(3), 49)));
    assert_eq!(at(350), None, "past covered prefix");
    let _ = std::fs::remove_file(path);
}

fn rename(
    state: &mut MetaState,
    sp: u64,
    sn: &str,
    dp: u64,
    dn: &str,
) -> Result<OpResult, MetaError> {
    state.apply_at(
        &MetaOp::Rename {
            src_parent: sp,
            src_name: sn.into(),
            dst_parent: dp,
            dst_name: dn.into(),
        },
        NOW,
    )
}

#[test]
fn rename_matrix() {
    let (path, mut state) = fresh();
    let d = mkdir(&mut state, ROOT_INO, "d");
    let f = create_file(&mut state, ROOT_INO, "f");
    let g = create_file(&mut state, ROOT_INO, "g");
    // file over file: dst replaced.
    rename(&mut state, ROOT_INO, "f", ROOT_INO, "g").expect("f -> g");
    assert_eq!(state.lookup(ROOT_INO, "g").expect("lookup"), Some(f));
    assert!(
        state.getattr(g).expect("getattr").is_none(),
        "replaced file freed"
    );
    // file over dir: IsDir.
    let err = rename(&mut state, ROOT_INO, "g", ROOT_INO, "d").expect_err("file over dir");
    assert!(matches!(err, MetaError::IsDir));
    // dir over file: NotDir.
    let err = rename(&mut state, ROOT_INO, "d", ROOT_INO, "g").expect_err("dir over file");
    assert!(matches!(err, MetaError::NotDir));
    // dir over non-empty dir: NotEmpty.
    mkdir(&mut state, ROOT_INO, "e");
    let e = state.lookup(ROOT_INO, "e").expect("lookup").expect("e");
    create_file(&mut state, e, "x");
    let err = rename(&mut state, ROOT_INO, "d", ROOT_INO, "e").expect_err("dir over non-empty dir");
    assert!(matches!(err, MetaError::NotEmpty));
    // dir over empty dir: ok.
    state
        .apply_at(
            &MetaOp::Unlink {
                parent: e,
                name: "x".into(),
            },
            NOW,
        )
        .expect("unlink x");
    rename(&mut state, ROOT_INO, "d", ROOT_INO, "e").expect("d -> e");
    assert_eq!(state.lookup(ROOT_INO, "e").expect("lookup"), Some(d));
    // rename onto itself: ok, no-op.
    rename(&mut state, ROOT_INO, "e", ROOT_INO, "e").expect("self rename");
    let _ = std::fs::remove_file(path);
}

#[test]
fn rmdir_requires_empty() {
    let (path, mut state) = fresh();
    let d = mkdir(&mut state, ROOT_INO, "d");
    create_file(&mut state, d, "x");
    let err = state
        .apply_at(
            &MetaOp::Rmdir {
                parent: ROOT_INO,
                name: "d".into(),
            },
            NOW,
        )
        .expect_err("non-empty");
    assert!(matches!(err, MetaError::NotEmpty));
    // The failed rmdir must not have removed the dentry.
    assert_eq!(state.lookup(ROOT_INO, "d").expect("lookup"), Some(d));
    let _ = std::fs::remove_file(path);
}

#[test]
fn symlink_and_persistence() {
    let (path, mut state) = fresh();
    let s = match state
        .apply_at(
            &MetaOp::Symlink {
                parent: ROOT_INO,
                name: "s".into(),
                target: "/some/target".into(),
                uid: 0,
                gid: 0,
            },
            NOW,
        )
        .expect("symlink")
    {
        OpResult::Ino(ino) => ino,
        other => panic!("expected Ino, got {other:?}"),
    };
    let f = create_file(&mut state, ROOT_INO, "f");
    state
        .apply_at(
            &MetaOp::CommitLayout {
                ino: f,
                first_idx: 0,
                chunks: vec![chunk(5, 3, 42)],
                new_size: 42,
                seq: 1,
            },
            NOW,
        )
        .expect("commit");
    drop(state);
    let state = MetaState::open(&path).expect("reopen");
    let inode = state.getattr(s).expect("getattr").expect("symlink");
    assert_eq!(inode.kind, Kind::Symlink);
    assert_eq!(inode.symlink_target.as_deref(), Some("/some/target"));
    let layout = state.layout(f).expect("layout");
    assert_eq!(layout, vec![(0, chunk(5, 3, 42))]);
    let _ = std::fs::remove_file(path);
}

#[test]
fn commit_layout_seq_dedup() {
    let (path, mut state) = fresh();
    let f = create_file(&mut state, ROOT_INO, "f");
    let commit = |seq: u64, chunks: Vec<ChunkRef>, new_size: u64| MetaOp::CommitLayout {
        ino: f,
        first_idx: 0,
        chunks,
        new_size,
        seq,
    };
    state
        .apply_at(&commit(5, vec![chunk(1, 5, 10)], 10), NOW)
        .expect("seq 5");
    state
        .apply_at(&commit(5, vec![chunk(2, 5, 99)], 99), NOW)
        .expect("replayed seq 5");
    assert_eq!(
        state.layout(f).expect("layout"),
        vec![(0, chunk(1, 5, 10))],
        "replayed seq must be skipped"
    );
    state
        .apply_at(&commit(4, vec![chunk(3, 4, 7)], 7), NOW)
        .expect("older seq 4");
    assert_eq!(
        state.layout(f).expect("layout"),
        vec![(0, chunk(1, 5, 10))],
        "older seq must be skipped"
    );
    state
        .apply_at(&commit(6, vec![chunk(9, 6, 20)], 20), NOW)
        .expect("seq 6");
    assert_eq!(state.layout(f).expect("layout"), vec![(0, chunk(9, 6, 20))]);
    assert_eq!(state.commit_seq().expect("commit_seq"), 6);
    let _ = std::fs::remove_file(path);
}

#[test]
fn snap_rows() {
    let (path, mut state) = fresh();
    let id = match state
        .apply_at(
            &MetaOp::CreateSnap {
                name: "hourly-1".into(),
            },
            NOW,
        )
        .expect("snap")
    {
        OpResult::SnapId(id) => id,
        other => panic!("expected SnapId, got {other:?}"),
    };
    let snaps = state.list_snaps().expect("list");
    assert_eq!(snaps.len(), 1);
    assert_eq!(snaps[0].name, "hourly-1");
    state
        .apply_at(&MetaOp::DeleteSnap { id }, NOW)
        .expect("delete");
    assert!(state.list_snaps().expect("list").is_empty());
    let _ = std::fs::remove_file(path);
}
