//! S9 integration tests: real FUSE mounts driven through std::fs.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use plfs_client::fuse::MountGuard;

static NEXT: AtomicU64 = AtomicU64::new(0);

/// Unprivileged FUSE mounts need a user+mount namespace on this box
/// (fusermount3 is EPERM-rejected by the kernel): re-exec the test binary
/// inside `unshare -rm` once (porfs rootless-mode lineage).
fn ensure_userns() {
    if std::env::var("PLFS_FUSE_USERNS").is_ok() {
        return;
    }
    let exe = std::env::current_exe().expect("exe");
    let status = std::process::Command::new("unshare")
        .arg("-rm")
        .arg(&exe)
        .args(std::env::args().skip(1))
        .env("PLFS_FUSE_USERNS", "1")
        .status()
        .expect("spawn unshare");
    std::process::exit(status.code().unwrap_or(1));
}

fn fresh_dirs(name: &str) -> (PathBuf, PathBuf) {
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let base = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("plfs-fuse-{name}-{}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let vol = base.join("vol");
    let mnt = base.join("mnt");
    std::fs::create_dir_all(&vol).expect("vol");
    std::fs::create_dir_all(&mnt).expect("mnt");
    (vol, mnt)
}

#[test]
fn mount_rw_suite_and_remount_persistence() {
    ensure_userns();
    let (vol, mnt) = fresh_dirs("suite");
    {
        let _guard = MountGuard::mount_fresh(&vol, &mnt, 1 << 30).expect("mount");
        std::fs::create_dir(mnt.join("saves")).expect("mkdir");
        let data: Vec<u8> = (0..300_000u32).map(|i| (i % 251) as u8).collect();
        std::fs::write(mnt.join("saves/player1.sav"), &data).expect("write");
        let got = std::fs::read(mnt.join("saves/player1.sav")).expect("read");
        assert_eq!(got, data, "byte-exact through the mount");
        // Append + rewrite across chunk boundaries.
        let mut f = std::fs::OpenOptions::new()
            .append(true)
            .open(mnt.join("saves/player1.sav"))
            .expect("append open");
        use std::io::Write;
        f.write_all(&[7u8; 1000]).expect("append");
        drop(f);
        assert_eq!(
            std::fs::metadata(mnt.join("saves/player1.sav"))
                .expect("meta")
                .len(),
            data.len() as u64 + 1000
        );
        std::fs::rename(mnt.join("saves/player1.sav"), mnt.join("saves/player2.sav"))
            .expect("rename");
        std::os::unix::fs::symlink("player2.sav", mnt.join("saves/current")).expect("symlink");
        let target = std::fs::read_link(mnt.join("saves/current")).expect("readlink");
        assert_eq!(target.to_str().expect("utf8"), "player2.sav");
        std::fs::hard_link(mnt.join("saves/player2.sav"), mnt.join("saves/backup.sav"))
            .expect("link");
        let names: Vec<String> = std::fs::read_dir(mnt.join("saves"))
            .expect("readdir")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .collect();
        for want in ["player2.sav", "current", "backup.sav"] {
            assert!(
                names.iter().any(|n| n == want),
                "missing {want} in {names:?}"
            );
        }
        let f = std::fs::File::options()
            .write(true)
            .open(mnt.join("saves/player2.sav"))
            .expect("open");
        f.sync_all().expect("fsync");
        std::fs::remove_file(mnt.join("saves/backup.sav")).expect("unlink");
    }
    // Remount: flushed state persists byte-exact.
    {
        let _guard = MountGuard::mount(&vol, &mnt).expect("remount");
        let got = std::fs::read(mnt.join("saves/player2.sav")).expect("read");
        let mut want: Vec<u8> = (0..300_000u32).map(|i| (i % 251) as u8).collect();
        want.extend_from_slice(&[7u8; 1000]);
        assert_eq!(got, want, "remount read-back byte-exact");
    }
    let _ = std::fs::remove_dir_all(vol.parent().expect("base"));
}

#[test]
fn standalone_grpc_datapath_roundtrip() {
    ensure_userns();
    // The S10 standalone path: format a volume, serve it via loopback
    // ChunkStore gRPC, mount with the gRPC sink.
    let (vol, mnt) = fresh_dirs("standalone");
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("runtime");
    rt.block_on(async {
        plfs_client::core::ClientCore::format_volume(&vol, 1 << 30).expect("format");
        let svc = plfs_data::ChunkStoreSvc::open(vol.join("arena.img")).expect("svc");
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind");
        let addr = listener.local_addr().expect("addr");
        tokio::spawn(async move {
            tonic::transport::Server::builder()
                .add_service(svc.into_server())
                .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
                .await
        });
        let sink = plfs_client::core::SinkConfig::Grpc(format!("http://{addr}"));
        {
            let _guard = MountGuard::mount_with_sink(&vol, &mnt, sink.clone()).expect("mount grpc");
            let data: Vec<u8> = (0..200_000u32).map(|i| (i % 241) as u8).collect();
            std::fs::write(mnt.join("demo.bin"), &data).expect("write");
            let got = std::fs::read(mnt.join("demo.bin")).expect("read");
            assert_eq!(got, data, "byte-exact over loopback gRPC");
        }
        {
            let _guard = MountGuard::mount_with_sink(&vol, &mnt, sink).expect("remount grpc");
            let got = std::fs::read(mnt.join("demo.bin")).expect("read");
            let want: Vec<u8> = (0..200_000u32).map(|i| (i % 241) as u8).collect();
            assert_eq!(got, want, "remount byte-exact over loopback gRPC");
        }
    });
    let _ = std::fs::remove_dir_all(vol.parent().expect("base"));
}

#[test]
fn sparse_write_and_truncate() {
    ensure_userns();
    let (vol, mnt) = fresh_dirs("sparse");
    {
        let _guard = MountGuard::mount_fresh(&vol, &mnt, 1 << 30).expect("mount");
        use std::io::{Seek, SeekFrom, Write};
        let mut f = std::fs::File::create(mnt.join("sparse.bin")).expect("create");
        f.seek(SeekFrom::Start(1 << 20)).expect("seek");
        f.write_all(&[0xAB; 4096]).expect("sparse write");
        f.sync_all().expect("fsync");
        drop(f);
        let got = std::fs::read(mnt.join("sparse.bin")).expect("read");
        assert_eq!(got.len(), (1 << 20) + 4096);
        assert!(got[..1 << 20].iter().all(|b| *b == 0), "hole reads zeros");
        assert_eq!(&got[(1 << 20)..], &[0xAB; 4096]);
        f_truncate(&mnt, "sparse.bin", 1024);
        assert_eq!(
            std::fs::metadata(mnt.join("sparse.bin"))
                .expect("meta")
                .len(),
            1024
        );
        assert!(
            std::fs::read(mnt.join("sparse.bin"))
                .expect("read")
                .iter()
                .all(|b| *b == 0)
        );
    }
    let _ = std::fs::remove_dir_all(vol.parent().expect("base"));
}

fn f_truncate(mnt: &std::path::Path, name: &str, len: u64) {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .open(mnt.join(name))
        .expect("open");
    f.set_len(len).expect("set_len");
}
