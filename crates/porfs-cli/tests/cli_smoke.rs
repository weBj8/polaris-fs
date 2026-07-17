//! Smoke test: drive the real `porfs` binary through mkfs, info, and a small
//! bench on freshly created device files (via std::process).

use std::path::Path;
use std::process::{Command, Output};

/// Device files live on the build filesystem (real disk, O_DIRECT-capable).
fn test_dir() -> tempfile::TempDir {
    match std::env::var("CARGO_TARGET_TMPDIR") {
        Ok(dir) => tempfile::tempdir_in(dir).unwrap(),
        Err(_) => tempfile::tempdir().unwrap(),
    }
}

fn porfs(args: &[&str]) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_porfs"));
    cmd.args(args);
    cmd
}

fn assert_success(output: &Output, needles: &[&str]) {
    assert!(
        output.status.success(),
        "porfs failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for needle in needles {
        assert!(
            stdout.contains(needle),
            "stdout missing {needle:?}:\n{stdout}"
        );
    }
}

#[test]
fn mkfs_info_bench_smoke() {
    if !porfs_store::io_uring_available() {
        eprintln!("SKIP: io_uring unavailable in this environment");
        return;
    }
    let dir = test_dir();
    let dev: &Path = &dir.path().join("cli.img");

    let out = porfs(&["mkfs", "--device"])
        .arg(dev)
        .args(["--size", "16MiB"])
        .output()
        .unwrap();
    assert_success(&out, &["created PolarisFS format v1 device", "uuid:"]);

    let out = porfs(&["info", "--device"]).arg(dev).output().unwrap();
    assert_success(
        &out,
        &[
            "format_version: 1",
            "extent_count:   0",
            "data_start:     1048576",
            "io_mode:",
        ],
    );

    let bench_dev = dir.path().join("bench.img");
    let out = porfs(&["bench", "--device"])
        .arg(&bench_dev)
        .args([
            "--size",
            "16MiB",
            "--extent-size",
            "64KiB",
            "--queue-depth",
            "8",
        ])
        .output()
        .unwrap();
    assert_success(
        &out,
        &[
            "seq-write",
            "seq-read",
            "random-read",
            "MB/s",
            "IOPS",
            "summary:",
        ],
    );

    // The bench device is a valid filesystem holding all written extents.
    let out = porfs(&["info", "--device"])
        .arg(&bench_dev)
        .output()
        .unwrap();
    assert_success(&out, &["extent_count:   256"]);
}

#[test]
fn info_on_missing_device_fails() {
    let dir = test_dir();
    let out = porfs(&["info", "--device"])
        .arg(dir.path().join("does-not-exist.img"))
        .output()
        .unwrap();
    assert!(!out.status.success());
}
