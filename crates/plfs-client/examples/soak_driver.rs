//! S19 soak driver: a deterministic-pattern workload with an on-disk
//! acked ledger (kill -9 safe). Every VERIFY_EVERY acks it scrubs and
//! re-reads the whole acked set — any acknowledged write lost or diverged
//! panics the soak. A standalone `verify` mode runs the same check after
//! the workload stops.
//!
//!   work   <registry> <workdir> — soak-<seq>.bin forever (seq 0 is the
//!          0xAB canary the corruption leg flips)
//!   verify <registry> <workdir> — one full check, then exit

use plfs_client::core::{ClientCore, SinkConfig};
use plfs_meta::{MetaOp, OpResult, ROOT_INO};

const VERIFY_EVERY: u64 = 20;

fn pattern(seq: u64) -> Vec<u8> {
    (0..100_000u32)
        .map(|i| ((seq.wrapping_mul(31) + u64::from(i)) % 251) as u8)
        .collect()
}

fn acked_path(dir: &std::path::Path) -> std::path::PathBuf {
    dir.join("acked.txt")
}

fn last_acked(dir: &std::path::Path) -> u64 {
    std::fs::read_to_string(acked_path(dir))
        .ok()
        .and_then(|s| s.lines().last().and_then(|l| l.parse().ok()))
        .unwrap_or(0)
}

fn has_ledger(dir: &std::path::Path) -> bool {
    acked_path(dir).exists()
}

fn record_acked(dir: &std::path::Path, seq: u64) {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(acked_path(dir))
        .expect("acked open");
    writeln!(f, "{seq}").expect("acked write");
    f.sync_all().expect("acked sync");
}

async fn file_ino(core: &mut ClientCore, name: &str) -> u64 {
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
        Ok(_) => panic!("create: unexpected"),
        Err(_) => core
            .state()
            .lookup(ROOT_INO, name)
            .expect("lookup")
            .expect("exists but not found"),
    }
}

async fn verify_all(core: &mut ClientCore, last: u64) {
    let stats = core.scrub(0).await.expect("scrub");
    for seq in 0..=last {
        let name = format!("soak-{seq}.bin");
        let ino = core
            .state()
            .lookup(ROOT_INO, &name)
            .expect("lookup")
            .unwrap_or_else(|| panic!("ACKED WRITE LOST: {name}"));
        if seq == 0 {
            let got = core.read(ino, 0, 1 << 20).await.expect("read");
            assert!(got.iter().all(|&b| b == 0xAB), "canary diverges");
        } else {
            let got = core.read(ino, 0, 100_000).await.expect("read");
            assert!(got == pattern(seq), "ACKED WRITE DIVERGES: {name}");
        }
    }
    println!("VERIFY_OK files={} scrub={stats:?}", last + 1);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args().skip(1);
    let registry = args.next().expect("registry");
    let workdir = std::path::PathBuf::from(args.next().expect("workdir"));
    let mode = args.next().expect("mode");
    let sink = || SinkConfig::Cluster {
        registry: registry.clone(),
        rf: 3,
    };

    match mode.as_str() {
        "work" => {
            let mut core = if workdir.join("meta.redb").exists() {
                ClientCore::open_with_sink(&workdir, sink())
                    .await
                    .expect("open")
            } else {
                ClientCore::create_with_sink(&workdir, 1 << 30, sink())
                    .await
                    .expect("create")
            };
            if !has_ledger(&workdir) {
                let ino = file_ino(&mut core, "soak-0.bin").await;
                core.write(ino, 0, &vec![0xABu8; 1 << 20])
                    .await
                    .expect("canary");
                core.fsync().await.expect("fsync");
                record_acked(&workdir, 0);
            }
            let mut seq = last_acked(&workdir).max(1);
            eprintln!("work: resuming at seq {seq}");
            loop {
                let name = format!("soak-{seq}.bin");
                let ino = file_ino(&mut core, &name).await;
                let data = pattern(seq);
                if let Err(e) = core.write(ino, 0, &data).await {
                    eprintln!("write seq {seq}: {e} — retrying");
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    continue;
                }
                if let Err(e) = core.fsync().await {
                    eprintln!("fsync seq {seq}: {e} — retrying");
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    continue;
                }
                record_acked(&workdir, seq);
                if seq.is_multiple_of(VERIFY_EVERY) {
                    verify_all(&mut core, seq).await;
                }
                seq += 1;
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
        }
        "verify" => {
            let mut core = ClientCore::open_with_sink(&workdir, sink())
                .await
                .expect("open");
            verify_all(&mut core, last_acked(&workdir)).await;
            core.shutdown().await.expect("shutdown");
        }
        other => panic!("unknown mode: {other}"),
    }
}
