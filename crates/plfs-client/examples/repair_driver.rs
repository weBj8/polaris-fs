//! S14 gate driver: one replicated client (RF=3) over registry-discovered
//! data nodes. Writes chunks, then waits for the background repair worker
//! to re-replicate the chunks that were on a killed data node.

use plfs_client::core::{ClientCore, SinkConfig};
use plfs_meta::{MetaOp, OpResult, ROOT_INO};

const PAYLOAD_LEN: usize = 300_000;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args().skip(1);
    let registry_addr = args.next().expect("registry addr");
    let workdir = std::path::PathBuf::from(args.next().expect("workdir"));
    let expect_repair: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(1);

    let mut core = ClientCore::create_with_sink(
        &workdir,
        1 << 30,
        SinkConfig::Cluster {
            registry: registry_addr,
            rf: 3,
        },
    )
    .await
    .expect("create core");

    // Write 8 files (8 chunks) so every live node ends up with a replica of
    // at least one of them (RF=3 over 4 nodes ⇒ each node in ~75% of sets).
    let mut files = Vec::new();
    for i in 0..8 {
        let ino = match core
            .meta_op(MetaOp::CreateFile {
                parent: ROOT_INO,
                name: format!("replicated-{i}.bin"),
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
        let payload: Vec<u8> = (0..PAYLOAD_LEN as u32)
            .map(|j| ((j as usize + i * 13) % 251) as u8)
            .collect();
        core.write(ino, 0, &payload).await.expect("write");
        files.push((ino, payload));
    }
    core.fsync().await.expect("fsync");

    // Reads work (and keep working while a replica is down — failover).
    for (ino, payload) in &files {
        let got = core.read(*ino, 0, PAYLOAD_LEN as u64).await.expect("read");
        assert_eq!(&got, payload, "initial read-back diverges");
    }
    eprintln!(
        "wrote and read back {} files, waiting for repair...",
        files.len()
    );

    // The script kills a data node; the repair worker must re-replicate the
    // chunks that were on it (lease expiry ~5 s + 2 s poll tick).
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while core.repair_count() < expect_repair {
        if std::time::Instant::now() > deadline {
            eprintln!("REPAIR_TIMEOUT: repair_count={}", core.repair_count());
            std::process::exit(1);
        }
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }

    // Reads still byte-exact after the node loss + repair.
    for (ino, payload) in &files {
        let got = core.read(*ino, 0, PAYLOAD_LEN as u64).await.expect("read");
        assert_eq!(&got, payload, "read-back after kill+repair diverges");
    }
    println!("REPAIR_OK repaired={} rf=3", core.repair_count());
    core.shutdown().await.expect("shutdown");
}
