//! S16 gate driver (design doc §9): rollback restores byte-exact state and
//! is itself reversible; snapshot delete re-enqueues exclusively-referenced
//! chunks; the retention scheduler creates and prunes on schedule.

use plfs_client::core::ClientCore;
use plfs_meta::{MetaOp, OpResult, ROOT_INO};

const PAYLOAD_LEN: usize = 300_000;

fn payload(file: usize, variant: u32) -> Vec<u8> {
    (0..PAYLOAD_LEN as u32)
        .map(|j| ((j as usize + file * 13 + variant as usize * 7) % 251) as u8)
        .collect()
}

async fn create_file(core: &mut ClientCore, name: &str) -> u64 {
    match core
        .meta_op(MetaOp::CreateFile {
            parent: ROOT_INO,
            name: name.into(),
            mode: 0o644,
            uid: 0,
            gid: 0,
        })
        .await
        .expect("create")
    {
        Ok(OpResult::Ino(ino)) => ino,
        other => panic!("create: {other:?}"),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args().skip(1);
    let workdir = std::path::PathBuf::from(args.next().expect("workdir"));
    let phase = args.next().expect("phase");

    match phase.as_str() {
        "rollback" => {
            let mut core = ClientCore::create(&workdir, 1 << 30)
                .await
                .expect("create core");
            let mut inos = Vec::new();
            for i in 0..4 {
                let ino = create_file(&mut core, &format!("file-{i}.bin")).await;
                core.write(ino, 0, &payload(i, 0)).await.expect("write");
                inos.push(ino);
            }
            core.fsync().await.expect("fsync");
            let a = core.snapshot_create("A").await.expect("snap A");
            for (i, ino) in inos.iter().enumerate() {
                core.write(*ino, 0, &payload(i, 1)).await.expect("rewrite");
            }
            core.fsync().await.expect("fsync");
            let _b = core.snapshot_create("B").await.expect("snap B");
            for (i, ino) in inos.iter().enumerate() {
                core.write(*ino, 0, &payload(i, 2)).await.expect("rewrite");
            }
            core.fsync().await.expect("fsync");

            let pre = core.snapshot_rollback(a).await.expect("rollback to A");
            for (i, ino) in inos.iter().enumerate() {
                let got = core.read(*ino, 0, PAYLOAD_LEN as u64).await.expect("read");
                assert_eq!(got, payload(i, 0), "post-rollback file-{i} diverges");
            }
            eprintln!("rollback to A byte-exact");

            core.snapshot_rollback(pre).await.expect("rollback to pre");
            for (i, ino) in inos.iter().enumerate() {
                let got = core.read(*ino, 0, PAYLOAD_LEN as u64).await.expect("read");
                assert_eq!(got, payload(i, 2), "reversible file-{i} diverges");
            }
            eprintln!("rollback to pre-rollback byte-exact (reversible)");

            let big = create_file(&mut core, "big.bin").await;
            let data = vec![7u8; 32 << 20];
            for off in (0..data.len()).step_by(1 << 20) {
                let end = (off + (1 << 20)).min(data.len());
                core.write(big, off as u64, &data[off..end])
                    .await
                    .expect("write big");
            }
            core.fsync().await.expect("fsync");
            let c = core.snapshot_create("C").await.expect("snap C");
            match core
                .meta_op(MetaOp::Unlink {
                    parent: ROOT_INO,
                    name: "big.bin".into(),
                })
                .await
                .expect("unlink")
            {
                Ok(_) => {}
                other => panic!("unlink: {other:?}"),
            }
            core.fsync().await.expect("fsync");
            println!("ROLLBACK_OK c={c}");
            core.shutdown().await.expect("shutdown");
        }
        "reclaim" => {
            let c: u64 = args.next().expect("snap id").parse().expect("num");
            let mut core = ClientCore::open(&workdir).await.expect("open core");
            core.snapshot_delete(c).await.expect("snapshot_delete");
            println!("RECLAIM_OK deleted={c}");
            core.shutdown().await.expect("shutdown");
        }
        "schedule" => {
            std::fs::create_dir_all(&workdir).expect("mkdir");
            std::fs::write(
                workdir.join("snapshots.toml"),
                "interval_secs = 1\nkeep = 3\nname_prefix = \"auto\"\n",
            )
            .expect("toml");
            let core = ClientCore::create(&workdir, 1 << 30)
                .await
                .expect("create core");
            tokio::time::sleep(std::time::Duration::from_millis(4500)).await;
            let snaps = core.snapshot_list().expect("list");
            let auto: Vec<_> = snaps
                .iter()
                .filter(|s| s.name.starts_with("auto-"))
                .collect();
            assert_eq!(auto.len(), 3, "expected 3 retained snapshots: {snaps:?}");
            println!("SCHEDULER_OK kept=3");
            core.shutdown().await.expect("shutdown");
        }
        other => panic!("unknown phase: {other}"),
    }
}
