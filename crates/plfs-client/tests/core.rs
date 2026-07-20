//! S8 integration tests: client-core write path end to end in-process.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use plfs_client::core::{CHUNK_SIZE, ClientCore};
use plfs_meta::{MetaOp, OpResult, ROOT_INO};

static NEXT: AtomicU64 = AtomicU64::new(0);

fn fresh_dir(name: &str) -> PathBuf {
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("plfs-client-{name}-{}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("mkdir");
    dir
}

async fn create_file(core: &ClientCore, name: &str) -> u64 {
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
        other => panic!("expected Ino, got {other:?}"),
    }
}

#[tokio::test(flavor = "current_thread")]
async fn write_read_roundtrip_across_chunks() {
    let dir = fresh_dir("roundtrip");
    let mut core = ClientCore::create(&dir, 512 << 20).await.expect("create");
    let ino = create_file(&core, "f").await;
    // 2.5 MiB spanning three chunks, not chunk-aligned.
    let buf: Vec<u8> = (0..(2 * CHUNK_SIZE + CHUNK_SIZE / 2))
        .map(|i| (i % 251) as u8)
        .collect();
    core.write(ino, 123, &buf).await.expect("write");
    let got = core.read(ino, 0, 1 << 22).await.expect("read");
    assert_eq!(got.len() as u64, 123 + buf.len() as u64);
    assert_eq!(&got[123..], &buf[..], "read-your-writes (pending overlay)");
    core.fsync().await.expect("fsync");
    let got = core.read(ino, 123, buf.len() as u64).await.expect("read");
    assert_eq!(got, buf, "read-after-flush from the arena");
    // Partial-overwrite RMW: rewrite the middle of chunk 1.
    let patch: Vec<u8> = (0..1000u32).map(|i| (i % 13) as u8).collect();
    core.write(ino, CHUNK_SIZE + 500, &patch)
        .await
        .expect("patch");
    let mut want = buf.clone();
    want[CHUNK_SIZE as usize + 500 - 123..][..1000].copy_from_slice(&patch);
    core.fsync().await.expect("fsync");
    let got = core.read(ino, 123, want.len() as u64).await.expect("read");
    assert_eq!(got, want, "read-modify-write preserved untouched bytes");
    core.shutdown().await.expect("shutdown");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test(flavor = "current_thread")]
async fn reopen_persists_flushed_state() {
    let dir = fresh_dir("persist");
    let ino;
    let buf: Vec<u8> = (0..100_000u32).map(|i| (i % 7) as u8).collect();
    {
        let mut core = ClientCore::create(&dir, 512 << 20).await.expect("create");
        ino = create_file(&core, "f").await;
        core.write(ino, 0, &buf).await.expect("write");
        core.fsync().await.expect("fsync");
        core.shutdown().await.expect("shutdown");
    }
    let mut core = ClientCore::open(&dir).await.expect("open");
    let got = core.read(ino, 0, buf.len() as u64).await.expect("read");
    assert_eq!(got, buf);
    core.shutdown().await.expect("shutdown");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test(flavor = "current_thread")]
async fn crash_replay_recovers_unflushed_writes() {
    let dir = fresh_dir("crash");
    let ino;
    let buf: Vec<u8> = (0..300_000u32).map(|i| (i % 11) as u8).collect();
    {
        let mut core = ClientCore::create(&dir, 512 << 20).await.expect("create");
        ino = create_file(&core, "f").await;
        core.write(ino, 0, &buf).await.expect("write");
        // NO fsync: the write is WAL-durable only. shutdown() stops the raft
        // core so the redb handle frees (in a real kill -9 the OS releases
        // it); the WAL replay on reopen is the recovery under test. True
        // kill -9 coverage: scripts/kill9-client.sh.
        core.shutdown().await.expect("shutdown");
    }
    let mut core = ClientCore::open(&dir).await.expect("open");
    let got = core.read(ino, 0, buf.len() as u64).await.expect("read");
    assert_eq!(got, buf, "WAL replay flushed the unflushed write");
    // And the layout commit landed exactly once (no double refcount).
    let layout = core.state().layout(ino).expect("layout");
    let chunk_id = layout[0].1.chunk_id;
    assert_eq!(core.state().chunk_refcount(&chunk_id).expect("refcount"), 1);
    core.shutdown().await.expect("shutdown");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test(flavor = "current_thread")]
async fn cache_populates_and_hits_across_reopen() {
    let dir = fresh_dir("cache");
    let ino;
    let buf: Vec<u8> = (0..100_000u32).map(|i| (i % 17) as u8).collect();
    {
        let mut core = ClientCore::create(&dir, 512 << 20).await.expect("create");
        ino = create_file(&core, "f").await;
        core.write(ino, 0, &buf).await.expect("write");
        core.fsync().await.expect("fsync");
        core.shutdown().await.expect("shutdown");
    }
    let mut core = ClientCore::open(&dir).await.expect("open");
    let got = core.read(ino, 0, buf.len() as u64).await.expect("read 1");
    assert_eq!(got, buf);
    let stats = core.cache_stats().expect("cache present");
    assert_eq!(stats.misses, 1, "first read is a data-plane fetch");
    let got = core.read(ino, 0, buf.len() as u64).await.expect("read 2");
    assert_eq!(got, buf);
    let stats = core.cache_stats().expect("cache present");
    assert_eq!(stats.hits, 1, "second read is a cache hit");
    core.shutdown().await.expect("shutdown");
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test(flavor = "current_thread")]
async fn unlink_queues_chunks_and_gc_deletes_them() {
    let dir = fresh_dir("gc");
    let mut core = ClientCore::create(&dir, 512 << 20).await.expect("create");
    let ino = create_file(&core, "f").await;
    core.write(ino, 0, &[1u8; 10_000]).await.expect("write");
    core.fsync().await.expect("fsync");
    let layout = core.state().layout(ino).expect("layout");
    let chunk_id = layout[0].1.chunk_id;
    core.meta_op(MetaOp::Unlink {
        parent: ROOT_INO,
        name: "f".into(),
    })
    .await
    .expect("unlink")
    .expect("unlink ok");
    core.fsync().await.expect("flush drains gc");
    assert_eq!(core.state().gc_len().expect("gc len"), 0);
    assert!(
        matches!(core.state().chunk_refcount(&chunk_id), Ok(0)),
        "chunk refcount dropped"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
