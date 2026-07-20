//! S18 gate driver (design doc §4.3, §8.2): client A owns a volume and
//! serves its metadata; foreign client B discovers A through the registry
//! and sees A's committed writes within one second.

use plfs_client::core::{ClientCore, SinkConfig};
use plfs_client::foreign::ForeignClient;
use plfs_common::registry::v1 as rpb;
use plfs_meta::{MetaOp, OpResult, ROOT_INO};
use plfs_registry::client::RegistryClient;

const CONTENT1: &[u8] = b"cross-client sentinel ONE\n";
const CONTENT2: &[u8] = b"cross-client sentinel TWO\n";

fn unix_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis())
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
    let registry = args.next().expect("registry");
    let phase = args.next().expect("phase");
    match phase.as_str() {
        "write" => {
            let workdir = std::path::PathBuf::from(args.next().expect("workdir"));
            let volume = args.next().expect("volume");
            let meta_addr = args.next().expect("meta addr");
            let mut core = ClientCore::create_with_sink(
                &workdir,
                1 << 30,
                SinkConfig::Cluster {
                    registry: registry.clone(),
                    rf: 2,
                },
            )
            .await
            .expect("create core");
            core.serve_meta(&meta_addr).expect("serve_meta");
            let epoch = core.claim_writer("driver-A").await.expect("claim");
            let reg = RegistryClient::new(vec![registry.clone()]);
            let node_key = format!("writer-{volume}");
            reg.register(rpb::NodeInfo {
                node_key: node_key.clone(),
                kind: rpb::NodeKind::Client as i32,
                addr: meta_addr,
                volume: volume.clone(),
                capacity_bytes: 0,
                used_bytes: 0,
                last_seen: 0,
                lease_until: 0,
            })
            .await
            .expect("register");
            let hb = RegistryClient::new(vec![registry]);
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    let _ = hb.heartbeat(&node_key, 0).await;
                }
            });

            let ino1 = create_file(&mut core, "file-1.txt").await;
            core.write(ino1, 0, CONTENT1).await.expect("write");
            core.fsync().await.expect("fsync");
            println!("FILE1_READY epoch={epoch}");

            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            let ino2 = create_file(&mut core, "file-2.txt").await;
            core.write(ino2, 0, CONTENT2).await.expect("write");
            core.fsync().await.expect("fsync");
            println!("FILE2_FSYNCED {}", unix_ms());

            tokio::time::sleep(std::time::Duration::from_secs(4)).await;
            core.shutdown().await.expect("shutdown");
        }
        "read" => {
            let volume = args.next().expect("volume");
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
            let mut b = loop {
                match ForeignClient::connect(&registry, &volume, 2).await {
                    Ok(b) => break b,
                    Err(e) if std::time::Instant::now() < deadline => {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        let _ = e;
                    }
                    Err(e) => panic!("connect: {e}"),
                }
            };
            let ino1 = {
                let mut found = None;
                for _ in 0..60 {
                    if let Ok(Some(ino)) = b.lookup(ROOT_INO, "file-1.txt").await {
                        found = Some(ino);
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }
                found.expect("file-1 not visible")
            };
            let got = b.read(ino1, 0, 4096).await.expect("read");
            assert_eq!(&got[..CONTENT1.len()], CONTENT1, "file-1 diverges");
            println!("FILE1_SEEN");

            let ino2 = {
                let mut found = None;
                for _ in 0..120 {
                    if let Ok(Some(ino)) = b.lookup(ROOT_INO, "file-2.txt").await {
                        found = Some(ino);
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }
                found.expect("file-2 not visible")
            };
            println!("FILE2_SEEN {}", unix_ms());
            let got = b.read(ino2, 0, 4096).await.expect("read");
            assert_eq!(&got[..CONTENT2.len()], CONTENT2, "file-2 diverges");
            println!("SYNC_OK");
        }
        other => panic!("unknown phase: {other}"),
    }
}
