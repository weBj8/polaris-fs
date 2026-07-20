//! S8 kill -9 fuzz (scripts/kill9-client.sh): a writer process applies a
//! deterministic op stream through ClientCore (create/write/fsync/unlink),
//! logging each completed op index to an fsynced acklog; `verify` rebuilds
//! the model from the op stream and checks the recovered volume — every op
//! that returned is reflected after kill -9 (WAL replay + raft recovery),
//! with a one-op gap tolerated at the kill point.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;

use plfs_client::core::ClientCore;
use plfs_meta::{MetaOp, ROOT_INO};

const FILES: u64 = 8;
const ARENA_BYTES: u64 = 1 << 30;

/// Deterministic per-op RNG (splitmix on seed+index).
fn rng(seed: u64, n: u64) -> u64 {
    let mut x = seed.wrapping_add(n.wrapping_mul(0x9E37_79B9_7F4A_7C15));
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

fn payload(tag: u64, len: usize) -> Vec<u8> {
    let mut x = tag.max(1);
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

#[derive(Debug, Clone)]
enum Op {
    Create(u64),
    Write(u64, u64, u64),
    Fsync,
    Unlink(u64),
}

fn op_at(seed: u64, n: u64, live: &[bool]) -> Op {
    let r = rng(seed, n);
    let file = r % FILES;
    let choice = (r >> 32) % 100;
    if !live[file as usize] {
        Op::Create(file)
    } else if choice < 75 {
        Op::Write(file, (r >> 8) % 65536, 1 + (r >> 20) % 8192)
    } else if choice < 85 {
        Op::Fsync
    } else {
        Op::Unlink(file)
    }
}

/// Rebuild the live-file map at op n (for op generation and the model).
fn live_at(seed: u64, n: u64) -> Vec<bool> {
    let mut live = vec![false; FILES as usize];
    for i in 1..=n {
        match op_at(seed, i, &live) {
            Op::Create(f) => live[f as usize] = true,
            Op::Unlink(f) => live[f as usize] = false,
            _ => {}
        }
    }
    live
}

async fn apply(core: &mut ClientCore, op: &Op) {
    match op {
        Op::Create(file) => {
            core.meta_op(MetaOp::CreateFile {
                parent: ROOT_INO,
                name: format!("f{file}"),
                mode: 0o644,
                uid: 0,
                gid: 0,
            })
            .await
            .expect("create")
            .expect("create ok");
        }
        Op::Write(file, off, tag) => {
            let ino = lookup(core, *file).await;
            let data = payload(*tag, 1 + (*tag % 8192) as usize);
            core.write(ino, *off, &data).await.expect("write");
        }
        Op::Fsync => core.fsync().await.expect("fsync"),
        Op::Unlink(file) => {
            core.meta_op(MetaOp::Unlink {
                parent: ROOT_INO,
                name: format!("f{file}"),
            })
            .await
            .expect("unlink")
            .expect("unlink ok");
        }
    }
}

async fn lookup(core: &ClientCore, file: u64) -> u64 {
    core.state()
        .lookup(ROOT_INO, &format!("f{file}"))
        .expect("lookup")
        .expect("file must exist")
}

/// The expected volume state after ops 1..=n: file name → sparse content.
fn model_at(seed: u64, n: u64) -> BTreeMap<u64, BTreeMap<u64, u8>> {
    let mut live = vec![false; FILES as usize];
    let mut files: BTreeMap<u64, BTreeMap<u64, u8>> = BTreeMap::new();
    for i in 1..=n {
        match op_at(seed, i, &live) {
            Op::Create(f) => {
                live[f as usize] = true;
                files.insert(f, BTreeMap::new());
            }
            Op::Write(f, off, tag) => {
                let data = payload(tag, 1 + (tag % 8192) as usize);
                let file = files.get_mut(&f).expect("live file");
                for (j, b) in data.iter().enumerate() {
                    file.insert(off + j as u64, *b);
                }
            }
            Op::Unlink(f) => {
                live[f as usize] = false;
                files.remove(&f);
            }
            Op::Fsync => {}
        }
    }
    files
}

async fn check(dir: &std::path::Path, seed: u64, n: u64) -> bool {
    let mut core = ClientCore::open(dir).await.expect("open");
    let model = model_at(seed, n);
    let mut ok = true;
    for (file, want) in &model {
        let ino = core
            .state()
            .lookup(ROOT_INO, &format!("f{file}"))
            .expect("lookup");
        let Some(ino) = ino else {
            eprintln!("f{file} missing after {n} ops");
            ok = false;
            break;
        };
        let size = core
            .state()
            .getattr(ino)
            .expect("getattr")
            .expect("inode")
            .size;
        let got = core.read(ino, 0, size).await.expect("read");
        if got.len() as u64 != size {
            eprintln!("f{file} size mismatch");
            ok = false;
            break;
        }
        if want.iter().any(|(off, b)| got[*off as usize] != *b) {
            eprintln!("f{file} content diverges after {n} ops");
            ok = false;
            break;
        }
    }
    core.shutdown().await.expect("shutdown");
    ok
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap_or_default();
    let dir = PathBuf::from(args.next().expect("dir"));
    let acklog_path = args.next().expect("acklog");
    let seed: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(42);
    match mode.as_str() {
        "run" => {
            let fresh = !dir.join("meta.redb").exists();
            let mut core = if fresh {
                ClientCore::create(&dir, ARENA_BYTES).await.expect("create")
            } else {
                ClientCore::open(&dir).await.expect("open")
            };
            let mut acklog = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&acklog_path)
                .expect("acklog");
            let mut n = 0u64;
            loop {
                n += 1;
                let live = live_at(seed, n - 1);
                let op = op_at(seed, n, &live);
                apply(&mut core, &op).await;
                acklog
                    .write_all(format!("{n}\n").as_bytes())
                    .expect("acklog");
                acklog.sync_data().expect("acklog fsync");
                // Deterministic 0–3 ms pause to widen the kill window.
                let pause = rng(seed, n) % 4;
                tokio::time::sleep(std::time::Duration::from_millis(pause)).await;
            }
        }
        "verify" => {
            let logged: u64 = std::fs::read_to_string(&acklog_path)
                .unwrap_or_default()
                .lines()
                .filter_map(|l| l.parse().ok())
                .max()
                .unwrap_or(0);
            // Kill during volume creation: nothing was ever acknowledged,
            // a zero-byte/absent meta store is a valid end state.
            if logged == 0 {
                let meta = dir.join("meta.redb");
                let incomplete =
                    !meta.exists() || std::fs::metadata(&meta).map(|m| m.len()).unwrap_or(0) == 0;
                if incomplete {
                    println!("VERIFY_OK ops=0 creation_incomplete=true");
                    return;
                }
            }
            // Every logged op returned ⇒ durable. One trailing op (logged or
            // in-flight at the kill) may be present or absent.
            if check(&dir, seed, logged).await {
                println!("VERIFY_OK ops={logged}");
                return;
            }
            if check(&dir, seed, logged + 1).await {
                println!("VERIFY_OK ops={} gap_tail=true", logged + 1);
                return;
            }
            eprintln!("VERIFY_FAIL logged={logged}");
            std::process::exit(1);
        }
        other => {
            eprintln!("unknown mode {other:?}: want run|verify");
            std::process::exit(2);
        }
    }
}
