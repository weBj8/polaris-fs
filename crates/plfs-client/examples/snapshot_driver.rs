//! S15 gate driver (design doc §9): snapshot over 8 files → delete half,
//! rewrite the other half → the snapshot view stays byte-exact for all 8
//! originals, and the live tree carries the new state.

use plfs_client::core::ClientCore;
use plfs_meta::{MetaOp, OpResult, ROOT_INO};

const PAYLOAD_LEN: usize = 300_000;

fn payload(file: usize, variant: u32) -> Vec<u8> {
    (0..PAYLOAD_LEN as u32)
        .map(|j| ((j as usize + file * 13 + variant as usize * 7) % 251) as u8)
        .collect()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let workdir = std::path::PathBuf::from(std::env::args().nth(1).expect("workdir"));
    let keep = std::env::args().any(|a| a == "--keep");
    let mut core = ClientCore::create(&workdir, 1 << 30)
        .await
        .expect("create core");

    let mut inos = Vec::new();
    for i in 0..8 {
        let ino = match core
            .meta_op(MetaOp::CreateFile {
                parent: ROOT_INO,
                name: format!("file-{i}.bin"),
                mode: 0o644,
                uid: 0,
                gid: 0,
            })
            .await
            .expect("create")
        {
            Ok(OpResult::Ino(ino)) => ino,
            other => panic!("create: {other:?}"),
        };
        core.write(ino, 0, &payload(i, 0)).await.expect("write");
        inos.push(ino);
    }
    let sentinel = match core
        .meta_op(MetaOp::CreateFile {
            parent: ROOT_INO,
            name: "sentinel.txt".into(),
            mode: 0o644,
            uid: 0,
            gid: 0,
        })
        .await
        .expect("create")
    {
        Ok(OpResult::Ino(ino)) => ino,
        other => panic!("create: {other:?}"),
    };
    core.write(sentinel, 0, b"snapshot-gate-sentinel-v1\n")
        .await
        .expect("write");
    core.fsync().await.expect("fsync");

    let snap = core
        .snapshot_create("gate-s1")
        .await
        .expect("snapshot_create");
    eprintln!("snapshot {snap} created over 8 files");

    for (i, ino) in inos.iter().enumerate() {
        if i % 2 == 0 {
            match core
                .meta_op(MetaOp::Unlink {
                    parent: ROOT_INO,
                    name: format!("file-{i}.bin"),
                })
                .await
                .expect("unlink")
            {
                Ok(_) => {}
                other => panic!("unlink: {other:?}"),
            }
        } else {
            core.write(*ino, 0, &payload(i, 1)).await.expect("rewrite");
        }
    }
    // This flush runs gc_drain: snapshot-pinned chunks must survive it.
    core.write(sentinel, 0, b"snapshot-gate-sentinel-v2\n")
        .await
        .expect("rewrite");
    core.fsync().await.expect("fsync");

    for (i, ino) in inos.iter().enumerate() {
        if i % 2 == 1 {
            let got = core
                .read(*ino, 0, PAYLOAD_LEN as u64)
                .await
                .expect("live read");
            assert_eq!(got, payload(i, 1), "live file-{i} diverges");
        }
    }
    eprintln!("live tree verified (4 rewritten, 4 unlinked)");

    assert!(
        core.snapshot_list()
            .expect("list")
            .iter()
            .any(|s| s.id == snap && s.name == "gate-s1")
    );
    for (i, _) in inos.iter().enumerate() {
        let ino = core
            .snapshot_lookup(snap, ROOT_INO, &format!("file-{i}.bin"))
            .expect("snap lookup")
            .expect("present in snapshot");
        let got = core
            .snapshot_read(snap, ino, 0, PAYLOAD_LEN as u64)
            .await
            .expect("snap read");
        assert_eq!(got, payload(i, 0), "snapshot file-{i} diverges");
    }
    eprintln!("snapshot view byte-exact for all 8 originals");
    let sent_ino = core
        .snapshot_lookup(snap, ROOT_INO, "sentinel.txt")
        .expect("snap lookup")
        .expect("sentinel in snapshot");
    let sent = core
        .snapshot_read(snap, sent_ino, 0, 64)
        .await
        .expect("snap read");
    assert_eq!(sent, b"snapshot-gate-sentinel-v1\n");

    if !keep {
        core.snapshot_delete(snap).await.expect("snapshot_delete");
        assert!(core.snapshot_list().expect("list").is_empty());
    }
    println!("SNAPSHOT_OK id={snap} files=8 (4 deleted + 4 rewritten post-snap)");
    core.shutdown().await.expect("shutdown");
}
