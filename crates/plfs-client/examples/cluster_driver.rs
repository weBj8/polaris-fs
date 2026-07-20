//! S13 gate driver: discover data nodes through the Registry, then run 3
//! client volumes (each with its own local metadata raft + a gRPC sink to a
//! discovered data node) — write + fsync + read-back byte-exact.

use plfs_client::core::{ClientCore, SinkConfig};
use plfs_common::registry::v1 as pb;
use plfs_meta::{MetaOp, OpResult, ROOT_INO};
use plfs_registry::client::RegistryClient;

const PAYLOAD_LEN: usize = 200_000;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args().skip(1);
    let registry_addr = args.next().expect("registry addr");
    let workdir = std::path::PathBuf::from(args.next().expect("workdir"));

    let registry = RegistryClient::new(vec![registry_addr]);
    let data_nodes = registry
        .list_nodes(pb::NodeKind::Data)
        .await
        .expect("list data nodes");
    assert!(
        data_nodes.len() >= 3,
        "need >=3 data nodes, got {}",
        data_nodes.len()
    );
    let stats = registry.stats().await.expect("stats");
    assert!(stats.capacity_bytes > 0, "capacity must be positive");
    assert!(
        stats.live_data_nodes >= 3,
        "need >=3 LIVE data nodes (leases), got {}",
        stats.live_data_nodes
    );

    for i in 0..3 {
        let addr = &data_nodes[i % data_nodes.len()].addr;
        let dir = workdir.join(format!("client-{i}"));
        let mut core = ClientCore::create_with_sink(
            &dir,
            512 << 20,
            SinkConfig::Grpc(format!("http://{addr}")),
        )
        .await
        .expect("create client core");
        let ino = match core
            .meta_op(MetaOp::CreateFile {
                parent: ROOT_INO,
                name: format!("payload-{i}.bin"),
                mode: 0o644,
                uid: 0,
                gid: 0,
            })
            .await
            .expect("create file")
        {
            Ok(OpResult::Ino(ino)) => ino,
            other => panic!("create: {other:?}"),
        };
        let payload: Vec<u8> = (0..PAYLOAD_LEN as u32)
            .map(|j| ((j as usize * 31 + i * 7) % 251) as u8)
            .collect();
        core.write(ino, 0, &payload).await.expect("write");
        core.fsync().await.expect("fsync");
        let got = core.read(ino, 0, PAYLOAD_LEN as u64).await.expect("read");
        assert_eq!(got, payload, "client {i} read-back diverges");
        core.shutdown().await.expect("shutdown");
    }
    println!(
        "CLUSTER_WORKLOAD_OK clients=3 data_nodes={} capacity={}",
        data_nodes.len(),
        stats.capacity_bytes
    );
}
