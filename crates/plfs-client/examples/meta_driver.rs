//! MetaOps driver: applies a deterministic metadata workload through a
//! 3-node group (leader hints followed automatically) and verifies state.
//! Used by scripts/gate-meta-failover.sh.

use std::collections::BTreeMap;

use plfs_common::meta::v1 as pb;
use plfs_common::meta::v1::meta_ops_client::MetaOpsClient;
use plfs_meta::service::{MetaRead, MetaReadReply};
use plfs_meta::{ChunkRef, MetaOp, OpResult, ROOT_INO};

fn enc<T: serde::Serialize>(v: &T) -> Vec<u8> {
    bincode::serialize(v).expect("encode")
}

fn dec<T: serde::de::DeserializeOwned>(b: &[u8]) -> T {
    bincode::deserialize(b).expect("decode")
}

/// Leader-tracking MetaOps client (design doc §7.3: followers proxy to the
/// leader). On a hint or transport error it rotates endpoints.
struct ClusterClient {
    endpoints: Vec<String>,
    leader: Option<String>,
}

impl ClusterClient {
    fn new(endpoints: Vec<String>) -> Self {
        Self {
            endpoints,
            leader: None,
        }
    }

    async fn apply(
        &mut self,
        op: &MetaOp,
    ) -> Result<Result<OpResult, plfs_meta::MetaError>, String> {
        let payload = enc(op);
        for attempt in 0..(self.endpoints.len() * 4) {
            let addr = self.pick(attempt);
            let Ok(mut client) = MetaOpsClient::connect(format!("http://{addr}")).await else {
                continue;
            };
            let resp = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                client.apply(pb::RpcEnvelope {
                    payload: payload.clone(),
                }),
            )
            .await;
            let Ok(Ok(resp)) = resp else {
                continue;
            };
            let reply = resp.into_inner();
            if !reply.leader_hint.is_empty() {
                self.leader = reply
                    .leader_hint
                    .split_once('@')
                    .map(|(_, addr)| addr.to_string());
                continue;
            }
            if reply.payload.is_empty() {
                continue;
            }
            return Ok(dec(&reply.payload));
        }
        Err("no reachable leader".into())
    }

    async fn read(&mut self, q: &MetaRead) -> Result<MetaReadReply, String> {
        let payload = enc(q);
        for attempt in 0..(self.endpoints.len() * 4) {
            let addr = self.pick(attempt);
            let Ok(mut client) = MetaOpsClient::connect(format!("http://{addr}")).await else {
                continue;
            };
            let resp = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                client.read(pb::RpcEnvelope {
                    payload: payload.clone(),
                }),
            )
            .await;
            let Ok(Ok(resp)) = resp else {
                continue;
            };
            let reply = resp.into_inner();
            if !reply.leader_hint.is_empty() {
                self.leader = reply
                    .leader_hint
                    .split_once('@')
                    .map(|(_, addr)| addr.to_string());
                continue;
            }
            if reply.payload.is_empty() {
                continue;
            }
            let inner: Result<MetaReadReply, plfs_meta::MetaError> = dec(&reply.payload);
            return inner.map_err(|e| e.to_string());
        }
        Err("no reachable leader".into())
    }

    fn pick(&self, attempt: usize) -> String {
        if attempt == 0
            && let Some(l) = &self.leader
        {
            return l.clone();
        }
        self.endpoints[attempt % self.endpoints.len()].clone()
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap_or_default();
    let endpoints: Vec<String> = args
        .next()
        .expect("endpoints csv")
        .split(',')
        .map(str::to_string)
        .collect();
    match mode.as_str() {
        "workload" => {
            let count: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(30);
            let mut client = ClusterClient::new(endpoints);
            let mut inos = BTreeMap::new();
            for i in 0..count {
                let name = format!("file-{i:03}");
                let r = client
                    .apply(&MetaOp::CreateFile {
                        parent: ROOT_INO,
                        name: name.clone(),
                        mode: 0o644,
                        uid: 0,
                        gid: 0,
                    })
                    .await
                    .expect("apply");
                let ino = match r {
                    Ok(OpResult::Ino(ino)) => ino,
                    other => panic!("create {name}: {other:?}"),
                };
                let chunks = vec![ChunkRef {
                    chunk_id: [(i % 251) as u8; 16],
                    version: i + 1,
                    len: 4096 + i,
                    replicas: vec![],
                }];
                client
                    .apply(&MetaOp::CommitLayout {
                        ino,
                        first_idx: 0,
                        chunks,
                        new_size: 4096 + i,
                        seq: i + 1,
                    })
                    .await
                    .expect("commit")
                    .expect("commit ok");
                inos.insert(name, (ino, 4096 + i));
                if (i + 1) % 10 == 0 {
                    eprintln!("applied {}/{}", i + 1, count);
                }
            }
            // Verify through reads: every file resolves with its layout.
            for (name, (ino, len)) in &inos {
                let MetaReadReply::Lookup(Some(found)) = client
                    .read(&MetaRead::Lookup(ROOT_INO, name.clone()))
                    .await
                    .expect("read")
                else {
                    panic!("lookup {name} failed");
                };
                assert_eq!(found, *ino, "lookup {name}");
                let MetaReadReply::Layout(rows) =
                    client.read(&MetaRead::Layout(*ino)).await.expect("layout")
                else {
                    panic!("layout {name} failed");
                };
                assert_eq!(rows.len(), 1);
                assert_eq!(rows[0].1.len, *len);
            }
            println!("WORKLOAD_OK files={}", inos.len());
        }
        "flood" => {
            // Continuous writes with an fsynced ack log: line per applied op.
            let start: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let acklog_path = args.next().expect("acklog");
            let mut client = ClusterClient::new(endpoints);
            let mut acklog = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&acklog_path)
                .expect("acklog");
            use std::io::Write;
            let mut i = start;
            loop {
                let name = format!("flood-{i:05}");
                let r = client
                    .apply(&MetaOp::CreateFile {
                        parent: ROOT_INO,
                        name: name.clone(),
                        mode: 0o644,
                        uid: 0,
                        gid: 0,
                    })
                    .await;
                match r {
                    Ok(Ok(OpResult::Ino(ino))) => {
                        acklog
                            .write_all(format!("{i} {ino} {name}\n").as_bytes())
                            .expect("acklog");
                        acklog.sync_data().expect("acklog fsync");
                        i += 1;
                    }
                    Ok(Ok(_)) => panic!("unexpected OpResult"),
                    Ok(Err(plfs_meta::MetaError::Exists)) => {
                        // Already written (retry after failover): skip.
                        i += 1;
                    }
                    Ok(Err(e)) => panic!("apply failed: {e}"),
                    Err(_) => {
                        // No reachable leader right now (election window):
                        // back off briefly and retry.
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    }
                }
            }
        }
        "flood_verify" => {
            // Every op acknowledged in the acklog must be present.
            let acklog_path = args.next().expect("acklog");
            let mut client = ClusterClient::new(endpoints);
            let mut checked = 0u64;
            for line in std::fs::read_to_string(&acklog_path)
                .expect("acklog")
                .lines()
            {
                let name = line.split_whitespace().nth(2).expect("name");
                match client
                    .read(&MetaRead::Lookup(ROOT_INO, name.to_string()))
                    .await
                {
                    Ok(MetaReadReply::Lookup(Some(_))) => checked += 1,
                    other => {
                        eprintln!("DIVERGENCE: {name} -> {other:?}");
                        std::process::exit(1);
                    }
                }
            }
            println!("FLOOD_VERIFY_OK acked={checked}");
        }
        "verify" => {
            let count: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(30);
            let mut client = ClusterClient::new(endpoints);
            for i in 0..count {
                let name = format!("file-{i:03}");
                let MetaReadReply::Lookup(Some(_)) = client
                    .read(&MetaRead::Lookup(ROOT_INO, name.clone()))
                    .await
                    .expect("read")
                else {
                    panic!("VERIFY_FAIL: {name} missing");
                };
            }
            println!("VERIFY_OK files={count}");
        }
        other => {
            eprintln!("unknown mode {other:?}: want workload|verify");
            std::process::exit(2);
        }
    }
}
