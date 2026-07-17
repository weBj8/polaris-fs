//! `porfs mds-check` smoke tests: success on a healthy MDS, failure on a
//! corrupted one. Uses the workspace-built `porfs` binary (run via
//! `cargo test --workspace` so the binary target is built).

mod common;

use std::process::Output;

use porfs_mds::ROOT_INO;
use redb::{Database, TableDefinition};

use common::{format_mds, pair, test_dir};

const DIR_ENTRIES_T: TableDefinition<&[u8], &[u8; 8]> = TableDefinition::new("dir_entries");

fn run_mds_check(meta: &std::path::Path, data: &std::path::Path) -> Output {
    assert_cmd::Command::cargo_bin("porfs")
        .unwrap()
        .arg("mds-check")
        .arg("--meta")
        .arg(meta)
        .arg("--data")
        .arg(data)
        .output()
        .unwrap()
}

#[test]
fn mds_check_healthy_mds() {
    let dir = test_dir();
    let mut mds = format_mds(dir.path());
    let d = mds.mkdir(ROOT_INO, "d", 0o755).unwrap();
    let f = mds.create(d, "hello", 0o644).unwrap();
    mds.write(f, 0, b"world").unwrap();
    mds.fsync(f).unwrap();
    drop(mds);

    let (meta, data) = pair(dir.path());
    let output = run_mds_check(&meta, &data);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("mds-check: OK"), "{stdout}");
    assert!(stdout.contains("inodes:           3"), "{stdout}"); // root, d, hello
    assert!(stdout.contains("dir_entries:      2"), "{stdout}");
    assert!(stdout.contains("extent_map_rows:  1"), "{stdout}");
    assert!(stdout.contains("orphans_repaired: 0"), "{stdout}");
}

#[test]
fn mds_check_corrupted_mds_fails() {
    let dir = test_dir();
    let mds = format_mds(dir.path());
    drop(mds);

    // Plant a dangling directory entry directly in the redb file.
    let (meta, data) = pair(dir.path());
    let db = Database::open(&meta).unwrap();
    let txn = db.begin_write().unwrap();
    {
        let mut entries = txn.open_table(DIR_ENTRIES_T).unwrap();
        let mut key = ROOT_INO.to_be_bytes().to_vec();
        key.extend_from_slice(b"ghost");
        entries
            .insert(key.as_slice(), &999_999u64.to_be_bytes())
            .unwrap();
    }
    txn.commit().unwrap();
    drop(db);

    let output = run_mds_check(&meta, &data);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("corrupt"), "{stderr}");
}
