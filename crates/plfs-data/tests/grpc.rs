//! S5 functional tests: the five ChunkStore RPCs over real loopback gRPC.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use plfs_arena::{Arena, MkfsConfig};
use plfs_common::data::v1 as pb;
use plfs_common::data::v1::chunk_store_client::ChunkStoreClient;
use plfs_data::ChunkStoreSvc;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::Code;

static NEXT: AtomicU64 = AtomicU64::new(0);

struct CaseFile(PathBuf);

impl CaseFile {
    fn new() -> Self {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
        std::fs::create_dir_all(&dir).expect("tmpdir");
        Self(dir.join(format!("plfs-data-grpc-{}-{n}", std::process::id())))
    }
}

impl Drop for CaseFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

struct Node {
    addr: SocketAddr,
    shutdown: Option<oneshot::Sender<()>>,
    handle: JoinHandle<()>,
    _file: CaseFile,
}

async fn boot() -> Node {
    let file = CaseFile::new();
    Arena::mkfs(
        &file.0,
        &MkfsConfig {
            total_bytes: 64 << 20,
            l_fraction: 0.90,
            l_slot_size: 1 << 20,
            s_slot_size: 64 << 10,
        },
    )
    .expect("mkfs");
    let svc = ChunkStoreSvc::open(&file.0).expect("open service");
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local addr");
    let (sd_tx, sd_rx) = oneshot::channel::<()>();
    let handle = tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(svc.into_server())
            .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
                sd_rx.await.ok();
            })
            .await
            .expect("server");
    });
    Node {
        addr,
        shutdown: Some(sd_tx),
        handle,
        _file: file,
    }
}

impl Node {
    async fn client(&self) -> ChunkStoreClient<tonic::transport::Channel> {
        ChunkStoreClient::connect(format!("http://{}", self.addr))
            .await
            .expect("connect")
    }

    async fn stop(mut self) {
        self.shutdown.take().expect("shutdown").send(()).ok();
        self.handle.await.ok();
    }
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

/// Collect a Get stream: (first-frame meta, reassembled payload).
async fn collect(stream: tonic::Streaming<pb::GetReply>) -> (pb::GetReply, Vec<u8>) {
    let mut stream = stream;
    let mut first: Option<pb::GetReply> = None;
    let mut data = Vec::new();
    while let Some(frame) = stream.message().await.expect("stream frame") {
        if first.is_none() {
            first = Some(pb::GetReply {
                data: Vec::new(),
                ..frame.clone()
            });
        }
        data.extend_from_slice(&frame.data);
    }
    (first.expect("at least one frame"), data)
}

#[tokio::test]
async fn put_get_roundtrip_byte_exact() {
    let node = boot().await;
    let mut c = node.client().await;
    let id = [0x11; 16];
    let payload: Vec<u8> = (0..100_000u32).map(|i| (i % 251) as u8).collect();
    c.put(put_req(id, 7, &payload)).await.expect("put");
    let (meta, got) = collect(
        c.get(pb::GetRequest {
            chunk_id: id.to_vec(),
            if_version: 0,
        })
        .await
        .expect("get")
        .into_inner(),
    )
    .await;
    assert_eq!(meta.version, 7);
    assert_eq!(meta.total_len, payload.len() as u64);
    assert_eq!(meta.crc32c, crc32fast::hash(&payload));
    assert!(!meta.not_modified);
    assert_eq!(got, payload);
    node.stop().await;
}

#[tokio::test]
async fn put_is_idempotent_and_conflict_checked() {
    let node = boot().await;
    let mut c = node.client().await;
    let id = [0x22; 16];
    c.put(put_req(id, 3, b"alpha")).await.expect("put 1");
    c.put(put_req(id, 3, b"alpha"))
        .await
        .expect("identical retry must succeed (no-op)");
    let err = c
        .put(put_req(id, 3, b"different"))
        .await
        .expect_err("same id, different crc must conflict");
    assert_eq!(err.code(), Code::AlreadyExists);
    let err = c
        .put(put_req(id, 4, b"alpha"))
        .await
        .expect_err("same id, different version must conflict");
    assert_eq!(err.code(), Code::AlreadyExists);
    node.stop().await;
}

#[tokio::test]
async fn put_rejects_bad_crc_and_unsealed() {
    let node = boot().await;
    let mut c = node.client().await;
    let id = [0x33; 16];
    let mut req = put_req(id, 1, b"payload");
    req.crc32c ^= 1;
    let err = c.put(req).await.expect_err("crc mismatch");
    assert_eq!(err.code(), Code::InvalidArgument);
    let mut req = put_req(id, 1, b"payload");
    req.seal = false;
    let err = c.put(req).await.expect_err("unsealed put");
    assert_eq!(err.code(), Code::Unimplemented);
    node.stop().await;
}

#[tokio::test]
async fn get_if_version_is_not_modified() {
    let node = boot().await;
    let mut c = node.client().await;
    let id = [0x44; 16];
    c.put(put_req(id, 7, b"cached-bytes")).await.expect("put");
    let (meta, data) = collect(
        c.get(pb::GetRequest {
            chunk_id: id.to_vec(),
            if_version: 7,
        })
        .await
        .expect("get hit")
        .into_inner(),
    )
    .await;
    assert!(meta.not_modified);
    assert_eq!(meta.version, 7);
    assert!(data.is_empty());
    let (meta, data) = collect(
        c.get(pb::GetRequest {
            chunk_id: id.to_vec(),
            if_version: 3,
        })
        .await
        .expect("get miss")
        .into_inner(),
    )
    .await;
    assert!(!meta.not_modified);
    assert_eq!(data, b"cached-bytes");
    node.stop().await;
}

#[tokio::test]
async fn get_and_stat_missing_are_not_found() {
    let node = boot().await;
    let mut c = node.client().await;
    let id = [0x55; 16];
    let err = c
        .get(pb::GetRequest {
            chunk_id: id.to_vec(),
            if_version: 0,
        })
        .await
        .expect_err("get missing");
    assert_eq!(err.code(), Code::NotFound);
    let err = c
        .stat(pb::StatRequest {
            chunk_id: id.to_vec(),
        })
        .await
        .expect_err("stat missing");
    assert_eq!(err.code(), Code::NotFound);
    node.stop().await;
}

#[tokio::test]
async fn stat_reports_version_len_crc() {
    let node = boot().await;
    let mut c = node.client().await;
    let id = [0x66; 16];
    let payload = b"stat-me".to_vec();
    c.put(put_req(id, 42, &payload)).await.expect("put");
    let reply = c
        .stat(pb::StatRequest {
            chunk_id: id.to_vec(),
        })
        .await
        .expect("stat")
        .into_inner();
    assert_eq!(reply.version, 42);
    assert_eq!(reply.len, payload.len() as u64);
    assert_eq!(reply.crc32c, crc32fast::hash(&payload));
    assert!(reply.sealed);
    node.stop().await;
}

#[tokio::test]
async fn delete_is_exact_version_and_idempotent() {
    let node = boot().await;
    let mut c = node.client().await;
    let id = [0x77; 16];
    c.put(put_req(id, 7, b"victim")).await.expect("put");
    c.delete(pb::DeleteRequest {
        chunk_id: id.to_vec(),
        version: 8,
    })
    .await
    .expect("wrong-version delete is idempotent success");
    c.stat(pb::StatRequest {
        chunk_id: id.to_vec(),
    })
    .await
    .expect("chunk must survive a wrong-version delete");
    c.delete(pb::DeleteRequest {
        chunk_id: id.to_vec(),
        version: 7,
    })
    .await
    .expect("exact-version delete");
    let err = c
        .stat(pb::StatRequest {
            chunk_id: id.to_vec(),
        })
        .await
        .expect_err("deleted chunk");
    assert_eq!(err.code(), Code::NotFound);
    c.delete(pb::DeleteRequest {
        chunk_id: id.to_vec(),
        version: 7,
    })
    .await
    .expect("re-delete is idempotent success");
    node.stop().await;
}

#[tokio::test]
async fn list_streams_all_chunks() {
    let node = boot().await;
    let mut c = node.client().await;
    let ids: Vec<[u8; 16]> = (1u8..=3).map(|i| [i; 16]).collect();
    for (i, id) in ids.iter().enumerate() {
        c.put(put_req(*id, i as u64 + 1, b"x")).await.expect("put");
    }
    let mut stream = c.list(pb::ListRequest {}).await.expect("list").into_inner();
    let mut got = Vec::new();
    while let Some(item) = stream.message().await.expect("list frame") {
        got.push((item.chunk_id, item.version));
    }
    got.sort();
    let mut want: Vec<(Vec<u8>, u64)> = ids
        .iter()
        .enumerate()
        .map(|(i, id)| (id.to_vec(), i as u64 + 1))
        .collect();
    want.sort();
    assert_eq!(got, want);
    node.stop().await;
}

#[tokio::test]
async fn large_payload_streams_in_64kib_frames() {
    let node = boot().await;
    let mut c = node.client().await;
    let id = [0x88; 16];
    // 900 KiB > 64 KiB frame size, within the 1 MiB L-slot capacity.
    let payload: Vec<u8> = (0..900 * 1024u32).map(|i| (i % 253) as u8).collect();
    c.put(put_req(id, 1, &payload)).await.expect("put");
    let mut stream = c
        .get(pb::GetRequest {
            chunk_id: id.to_vec(),
            if_version: 0,
        })
        .await
        .expect("get")
        .into_inner();
    let mut frames = 0usize;
    let mut data = Vec::new();
    while let Some(frame) = stream.message().await.expect("frame") {
        assert!(frame.data.len() <= 64 * 1024, "frame exceeds 64 KiB");
        frames += 1;
        data.extend_from_slice(&frame.data);
    }
    assert!(frames >= 14, "expected many frames, got {frames}");
    assert_eq!(data, payload);
    node.stop().await;
}
