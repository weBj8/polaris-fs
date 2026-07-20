//! S5 gate proptest: random op sequences against a real loopback gRPC data
//! node, checked against a model. Properties (design doc §5):
//!   - Put idempotency: same (chunk_id, version, crc) ⇒ success no-op;
//!     same id with any different (version, crc) ⇒ AlreadyExists.
//!   - Delete is exact-version and idempotent: wrong version ⇒ silent
//!     success and the chunk survives; exact ⇒ gone; re-delete ⇒ success.
//!
//! Case count: PROPTEST_CASES env (default 8 — CI runs every test < 5 s).

use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use plfs_arena::{Arena, MkfsConfig};
use plfs_common::data::v1 as pb;
use plfs_common::data::v1::chunk_store_client::ChunkStoreClient;
use plfs_data::ChunkStoreSvc;
use proptest::prelude::*;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::Code;

const ID_POOL: usize = 6;

static NEXT: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone)]
enum Op {
    /// Fresh content for the id: Ok when the id is unknown, AlreadyExists
    /// unless identical to the live version+payload.
    Put {
        id: usize,
        version: u64,
        payload: Vec<u8>,
    },
    /// Exact re-put of the model's live (version, payload): must succeed.
    PutAgain {
        id: usize,
    },
    Get {
        id: usize,
    },
    Delete {
        id: usize,
        exact: bool,
        version: u64,
    },
    List,
}

fn op_strategy() -> impl Strategy<Value = Op> {
    let payload = prop::collection::vec(any::<u8>(), 0..=2048);
    prop_oneof![
        5 => (any::<proptest::sample::Index>(), any::<u64>(), payload).prop_map(
            |(i, v, p)| Op::Put { id: i.index(ID_POOL), version: v % 3, payload: p }
        ),
        2 => any::<proptest::sample::Index>().prop_map(|i| Op::PutAgain { id: i.index(ID_POOL) }),
        3 => any::<proptest::sample::Index>().prop_map(|i| Op::Get { id: i.index(ID_POOL) }),
        2 => (any::<proptest::sample::Index>(), any::<bool>(), any::<u64>()).prop_map(
            |(i, exact, v)| Op::Delete { id: i.index(ID_POOL), exact, version: v % 3 }
        ),
        1 => Just(Op::List),
    ]
}

struct Server {
    addr: SocketAddr,
    shutdown: tokio::sync::oneshot::Sender<()>,
    handle: tokio::task::JoinHandle<()>,
}

async fn boot(path: &std::path::Path) -> Server {
    Arena::mkfs(
        path,
        &MkfsConfig {
            total_bytes: 48 << 20,
            l_fraction: 0.90,
            l_slot_size: 1 << 20,
            s_slot_size: 64 << 10,
        },
    )
    .expect("mkfs");
    let svc = ChunkStoreSvc::open(path).expect("open service");
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local addr");
    let (sd_tx, sd_rx) = tokio::sync::oneshot::channel::<()>();
    let handle = tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(svc.into_server())
            .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
                sd_rx.await.ok();
            })
            .await
            .expect("server");
    });
    Server {
        addr,
        shutdown: sd_tx,
        handle,
    }
}

fn ids() -> Vec<[u8; 16]> {
    (0..ID_POOL).map(|i| [(i + 1) as u8; 16]).collect()
}

fn put_req(id: [u8; 16], version: u64, payload: &[u8]) -> pb::PutRequest {
    pb::PutRequest {
        chunk_id: id.to_vec(),
        version,
        crc32c: crc32fast::hash(payload),
        payload: payload.to_vec(),
        seal: true,
    }
}

async fn run_case(ops: Vec<Op>, case: u64) {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(&dir).expect("tmpdir");
    let path = dir.join(format!("plfs-data-gate-{}-{case}", std::process::id()));
    let server = boot(&path).await;
    let mut c = ChunkStoreClient::connect(format!("http://{}", server.addr))
        .await
        .expect("connect");
    let ids = ids();
    let mut model: HashMap<usize, (u64, Vec<u8>)> = HashMap::new();
    for op in ops {
        match op {
            Op::Put {
                id,
                version,
                payload,
            } => {
                let res = c.put(put_req(ids[id], version, &payload)).await;
                match (res, model.get(&id)) {
                    (Ok(_), None) => {
                        model.insert(id, (version, payload));
                    }
                    (Ok(_), Some((v, p))) => {
                        assert_eq!((*v, p), (version, &payload), "Ok put must be identical");
                    }
                    (Err(e), Some(_)) => assert_eq!(e.code(), Code::AlreadyExists),
                    (Err(e), None) => panic!("put failed on fresh id: {e}"),
                }
            }
            Op::PutAgain { id } => {
                let Some((version, payload)) = model.get(&id).cloned() else {
                    continue;
                };
                c.put(put_req(ids[id], version, &payload))
                    .await
                    .expect("idempotent re-put must succeed");
            }
            Op::Get { id } => {
                let res = c
                    .get(pb::GetRequest {
                        chunk_id: ids[id].to_vec(),
                        if_version: 0,
                    })
                    .await;
                match (res, model.get(&id)) {
                    (Ok(stream), Some((v, payload))) => {
                        let mut stream = stream.into_inner();
                        let mut first: Option<pb::GetReply> = None;
                        let mut data = Vec::new();
                        while let Some(f) = stream.message().await.expect("frame") {
                            first.get_or_insert_with(|| f.clone());
                            data.extend_from_slice(&f.data);
                        }
                        let first = first.expect("one frame");
                        assert_eq!(first.version, *v);
                        assert_eq!(&data, payload, "get payload mismatch");
                    }
                    (Err(e), None) => assert_eq!(e.code(), Code::NotFound),
                    (res, want) => panic!(
                        "get mismatch: {:?} vs model {:?}",
                        res.map(|_| ()),
                        want.map(|m| m.0)
                    ),
                }
            }
            Op::Delete { id, exact, version } => {
                let req_version = if exact {
                    model.get(&id).map_or(version, |m| m.0)
                } else {
                    version
                };
                c.delete(pb::DeleteRequest {
                    chunk_id: ids[id].to_vec(),
                    version: req_version,
                })
                .await
                .expect("delete is always an idempotent success");
                let still_there = c
                    .stat(pb::StatRequest {
                        chunk_id: ids[id].to_vec(),
                    })
                    .await
                    .is_ok();
                match model.get(&id) {
                    Some((v, _)) if *v == req_version => {
                        assert!(!still_there, "exact-version delete must remove");
                        model.remove(&id);
                    }
                    _ => {
                        assert_eq!(
                            still_there,
                            model.contains_key(&id),
                            "delete must not touch other versions"
                        );
                    }
                }
            }
            Op::List => {
                let mut stream = c.list(pb::ListRequest {}).await.expect("list").into_inner();
                let mut got = HashSet::new();
                while let Some(item) = stream.message().await.expect("list frame") {
                    got.insert(item.chunk_id);
                }
                let want: HashSet<Vec<u8>> = model.keys().map(|&i| ids[i].to_vec()).collect();
                assert_eq!(got, want, "list keys != model keys");
            }
        }
    }
    server.shutdown.send(()).ok();
    server.handle.await.ok();
    let _ = std::fs::remove_file(&path);
}

fn proptest_cases() -> u32 {
    std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(proptest_cases()))]
    #[test]
    fn gate_idempotent_put_version_checked_delete(ops in prop::collection::vec(op_strategy(), 10..40)) {
        let case = NEXT.fetch_add(1, Ordering::Relaxed);
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("runtime")
            .block_on(run_case(ops, case));
    }
}
