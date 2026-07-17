//! Shared helpers for the porfs-mds integration tests. Compiled into each
//! test binary; not every binary uses every helper, hence the crate-local
//! `dead_code` allowance (standard for shared test utilities).
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use porfs_mds::Mds;

/// Device size for tests: 128 MiB (sparse; the format's DATA_START is 64 MiB).
pub const DEV_SIZE: u64 = 128 * 1024 * 1024;

/// Temp dir on the build filesystem when `CARGO_TARGET_TMPDIR` is set (real
/// disk, O_DIRECT-capable), else the system temp dir (small files only —
/// /tmp is tmpfs on the dev box).
pub fn test_dir() -> tempfile::TempDir {
    match std::env::var("CARGO_TARGET_TMPDIR") {
        Ok(dir) => tempfile::tempdir_in(dir).unwrap(),
        Err(_) => tempfile::tempdir().unwrap(),
    }
}

/// The (meta, data) path pair of an MDS living in `dir`.
pub fn pair(dir: &Path) -> (PathBuf, PathBuf) {
    (dir.join("meta.redb"), dir.join("data.img"))
}

/// Format a fresh MDS in `dir`.
pub fn format_mds(dir: &Path) -> Mds {
    let (meta, data) = pair(dir);
    Mds::format(&meta, &data, DEV_SIZE).unwrap()
}

/// Reopen the MDS of `dir` (runs the mount-time reconcile).
pub fn open_mds(dir: &Path) -> Mds {
    let (meta, data) = pair(dir);
    Mds::open(&meta, &data).unwrap()
}

/// Deterministic content pattern: the byte at absolute file position `pos`.
pub fn pattern_byte(pos: u64) -> u8 {
    ((pos.wrapping_mul(31).wrapping_add(7)) & 0xff) as u8
}

/// `len` bytes of the deterministic pattern starting at `base`.
pub fn pattern(base: u64, len: usize) -> Vec<u8> {
    (0..len as u64).map(|i| pattern_byte(base + i)).collect()
}
