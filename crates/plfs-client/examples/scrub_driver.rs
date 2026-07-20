//! S17 gate driver (design doc §10.3): after the shell flips payload bytes
//! inside one replica's slot, the scrub detects the crc mismatch and
//! repairs the replica from a healthy copy; a chunk written straight to a
//! data node (no metadata root) is swept as an orphan.

use plfs_client::core::{ClientCore, SinkConfig};
use plfs_common::data::v1::PutRequest;
use plfs_common::data::v1::chunk_store_client::ChunkStoreClient;
use plfs_meta::{MetaOp, OpResult, ROOT_INO};

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
    let registry = args.next().expect("registry");
    let workdir = std::path::PathBuf::from(args.next().expect("workdir"));
    let phase = args.next().expect("phase");

    match phase.as_str() {
        "write" => {
            let mut core = ClientCore::create_with_sink(
                &workdir,
                1 << 30,
                SinkConfig::Cluster { registry, rf: 3 },
            )
            .await
            .expect("create core");
            let big = create_file(&mut core, "rot-target.bin").await;
            core.write(big, 0, &vec![0xABu8; 1 << 20])
                .await
                .expect("write");
            let small = create_file(&mut core, "small.bin").await;
            core.write(small, 0, b"small-payload").await.expect("write");
            core.fsync().await.expect("fsync");
            println!("WROTE_OK");
            core.shutdown().await.expect("shutdown");
        }
        "scrub" => {
            let inject_addr = args.next().expect("inject addr");
            let mut core =
                ClientCore::open_with_sink(&workdir, SinkConfig::Cluster { registry, rf: 3 })
                    .await
                    .expect("open core");
            // Orphan injection: a chunk no metadata root has ever seen,
            // written straight to a data node.
            let mut client = ChunkStoreClient::connect(format!("http://{inject_addr}"))
                .await
                .expect("connect");
            let payload = vec![0xCDu8; 32 * 1024];
            client
                .put(PutRequest {
                    chunk_id: vec![0xEE; 16],
                    version: 1,
                    crc32c: crc32fast::hash(&payload),
                    payload,
                    seal: true,
                })
                .await
                .expect("orphan put");

            let first = core.scrub(0).await.expect("scrub");
            assert!(first.repaired >= 1, "rot not repaired: {first:?}");
            assert!(first.orphans_deleted >= 1, "orphan not swept: {first:?}");

            let ino = core
                .state()
                .lookup(ROOT_INO, "rot-target.bin")
                .expect("lookup")
                .expect("file");
            let got = core.read(ino, 0, 1 << 20).await.expect("read");
            assert!(
                got.iter().all(|&b| b == 0xAB),
                "rot-target diverges after scrub"
            );

            let second = core.scrub(0).await.expect("scrub");
            assert_eq!(
                second.repaired, 0,
                "second pass still repairing: {second:?}"
            );
            assert_eq!(
                second.orphans_deleted, 0,
                "second pass still sweeping: {second:?}"
            );
            println!(
                "SCRUB_OK repaired={} orphans={}",
                first.repaired, first.orphans_deleted
            );
            core.shutdown().await.expect("shutdown");
        }
        other => panic!("unknown phase: {other}"),
    }
}
