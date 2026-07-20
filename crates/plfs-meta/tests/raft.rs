//! S8 raft tests: single-node group driving the metadata state machine.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use plfs_meta::raft::MetaRaft;
use plfs_meta::{ChunkRef, MetaOp, MetaState, OpResult, ROOT_INO};

static NEXT: AtomicU64 = AtomicU64::new(0);

const NOW: (i64, u32) = (1_750_000_000, 0);

fn fresh_path() -> PathBuf {
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(format!("plfs-raft-{}-{n}.redb", std::process::id()))
}

#[tokio::test]
async fn bootstrap_and_write_through_group() {
    let path = fresh_path();
    let state = MetaState::create(&path, NOW).expect("create");
    let raft = MetaRaft::bootstrap(&state, 1).await.expect("bootstrap");
    let r = raft
        .write(MetaOp::CreateFile {
            parent: ROOT_INO,
            name: "f".into(),
            mode: 0o644,
            uid: 0,
            gid: 0,
        })
        .await
        .expect("raft write");
    let ino = match r {
        Ok(OpResult::Ino(ino)) => ino,
        other => panic!("expected Ino, got {other:?}"),
    };
    assert_eq!(state.lookup(ROOT_INO, "f").expect("lookup"), Some(ino));
    // Business errors come back as the response payload, not raft errors.
    let r = raft
        .write(MetaOp::CreateFile {
            parent: ROOT_INO,
            name: "f".into(),
            mode: 0o644,
            uid: 0,
            gid: 0,
        })
        .await
        .expect("raft write");
    assert!(matches!(r, Err(plfs_meta::MetaError::Exists)));
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn restart_recovers_applied_state_without_reapply() {
    let path = fresh_path();
    let ino;
    let last_seq;
    {
        let state = MetaState::create(&path, NOW).expect("create");
        let raft = MetaRaft::bootstrap(&state, 1).await.expect("bootstrap");
        match raft
            .write(MetaOp::CreateFile {
                parent: ROOT_INO,
                name: "f".into(),
                mode: 0o644,
                uid: 0,
                gid: 0,
            })
            .await
            .expect("write")
        {
            Ok(OpResult::Ino(i)) => ino = i,
            other => panic!("expected Ino, got {other:?}"),
        }
        raft.write(MetaOp::CommitLayout {
            ino,
            first_idx: 0,
            chunks: vec![ChunkRef {
                chunk_id: [7; 16],
                version: 1,
                len: 42,
                replicas: vec![],
            }],
            new_size: 42,
            seq: 1,
        })
        .await
        .expect("commit")
        .expect("commit ok");
        last_seq = state.commit_seq().expect("commit_seq");
        raft.shutdown().await.expect("shutdown");
    }
    // Reopen: the applied log id persists, so the group continues without
    // re-applying history (a re-applied CreateFile would fail with Exists
    // and a re-applied CommitLayout would double the refcount).
    let state = MetaState::open(&path).expect("open");
    let raft = MetaRaft::bootstrap(&state, 1).await.expect("bootstrap");
    assert_eq!(state.commit_seq().expect("commit_seq"), last_seq);
    assert_eq!(state.chunk_refcount(&[7; 16]).expect("refcount"), 1);
    let r = raft
        .write(MetaOp::CreateFile {
            parent: ROOT_INO,
            name: "g".into(),
            mode: 0o644,
            uid: 0,
            gid: 0,
        })
        .await
        .expect("write g");
    match r {
        Ok(OpResult::Ino(i)) => assert_eq!(i, ino + 1, "ino sequence continues"),
        other => panic!("expected Ino, got {other:?}"),
    }
    assert_eq!(state.lookup(ROOT_INO, "f").expect("lookup"), Some(ino));
    let _ = std::fs::remove_file(&path);
}
