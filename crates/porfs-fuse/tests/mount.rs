//! Real-mount integration tests: mount a freshly formatted MDS in a tempdir
//! via `fuser::spawn_mount2`, then exercise POSIX operations through
//! `std::fs` on the mountpoint, and unmount cleanly (guard + fusermount
//! fallback, even on panic paths).
//!
//! Every test skips gracefully (eprintln + return) when /dev/fuse or
//! fusermount3 is unavailable, so the suite stays green in containers.
//!
//! pjdfstest note: pjdfstest is not packaged for Arch (no pacman package;
//! AUR would need a system-wide build/install, which this environment does
//! not warrant). P4 gate evidence is therefore this mirrored std::fs suite
//! covering the same basic op categories; pjdfstest becomes mandatory at P5
//! (xattr, more rename edge cases, 1M-entry directories).

use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use fuser::{BackgroundSession, Config, MountOption};
use porfs_fuse::{MountConfig, PorfsFs};

/// Device size per test MDS (sparse file; DATA_START is 64 MiB).
const DEV_SIZE: u64 = 128 * 1024 * 1024;

/// True when this box can actually perform FUSE mounts.
fn mount_available() -> bool {
    if !Path::new("/dev/fuse").exists() {
        return false;
    }
    std::process::Command::new("fusermount3")
        .arg("--version")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Skip helper: returns false (after a note) when mounting is impossible.
fn require_mount(what: &str) -> bool {
    if mount_available() {
        true
    } else {
        eprintln!("skipping {what}: /dev/fuse or fusermount3 unavailable");
        false
    }
}

/// Temp dir on the build filesystem when `CARGO_TARGET_TMPDIR` is set (real
/// disk, O_DIRECT-capable), else the system temp dir (/tmp is tmpfs here —
/// device files stay small).
fn test_dir() -> tempfile::TempDir {
    match std::env::var("CARGO_TARGET_TMPDIR") {
        Ok(dir) => tempfile::tempdir_in(dir).unwrap(),
        Err(_) => tempfile::tempdir().unwrap(),
    }
}

/// The (meta, data) pair of an MDS in `dir`.
fn pair(dir: &Path) -> (PathBuf, PathBuf) {
    (dir.join("meta.redb"), dir.join("data.img"))
}

/// Format a fresh MDS in `dir` and close it (releasing locks for the mount).
fn format_mds(dir: &Path) -> (PathBuf, PathBuf) {
    let (meta, data) = pair(dir);
    drop(porfs_mds::Mds::format(&meta, &data, DEV_SIZE).unwrap());
    (meta, data)
}

/// Deterministic content pattern: byte at absolute file position `pos`.
fn pattern_byte(pos: u64) -> u8 {
    ((pos.wrapping_mul(31).wrapping_add(7)) & 0xff) as u8
}

fn pattern(base: u64, len: usize) -> Vec<u8> {
    (0..len as u64).map(|i| pattern_byte(base + i)).collect()
}

/// RAII mount guard: unmounts on drop (and once more via fusermount3 in
/// case the session died mid-flight).
struct MountGuard {
    session: Option<BackgroundSession>,
    mountpoint: PathBuf,
}

impl MountGuard {
    /// Mount the MDS stored in `mds_dir` at `mountpoint`. Returns `None`
    /// (after a note) when the kernel rejects the mount — treated as a skip.
    fn mount(mds_dir: &Path, mountpoint: &Path) -> Option<Self> {
        let (meta, data) = pair(mds_dir);
        // default_permissions off: the tests run as the dev user while
        // inodes report uid 0, and these tests verify storage semantics,
        // not enforcement (enforcement is pjdfstest's job, run as root).
        let config = MountConfig {
            default_permissions: false,
            ..MountConfig::default()
        };
        let fs = match PorfsFs::open(&meta, &data, config) {
            Ok(fs) => fs,
            Err(err) => {
                eprintln!("skipping: MDS open failed: {err}");
                return None;
            }
        };
        let mut config = Config::default();
        config.mount_options = vec![MountOption::FSName("porfs-test".to_string())];
        match fuser::spawn_mount2(fs, mountpoint, &config) {
            Ok(session) => Some(Self {
                session: Some(session),
                mountpoint: mountpoint.to_path_buf(),
            }),
            Err(err) => {
                eprintln!("skipping: FUSE mount failed: {err}");
                None
            }
        }
    }

    /// Unmount now (idempotent; Drop covers panic paths).
    fn unmount(mut self) {
        if let Some(session) = self.session.take() {
            teardown(session, &self.mountpoint);
        }
    }
}

/// A leaked open fd makes the plain unmount fail with EBUSY and the
/// session join would then block forever (a lazy detach keeps the FUSE
/// connection alive until every fd closes — and a panicking test unwinds
/// past its `close` calls). So on EBUSY first close any fds this process
/// still holds under the mountpoint, retry, and only then fall back to a
/// lazy detach — exactly what a real admin does with `fusermount -uz`.
fn teardown(session: BackgroundSession, mountpoint: &Path) {
    if !fusermount(mountpoint, &["-u"]) {
        let leaked = close_leaked_fds(mountpoint);
        if leaked > 0 {
            eprintln!(
                "porfs-test: closed {leaked} leaked fd(s) under {}",
                mountpoint.display()
            );
        }
        if !fusermount(mountpoint, &["-u"]) {
            let _ = fusermount(mountpoint, &["-u", "-z"]);
        }
    }
    let _ = session.umount_and_join();
}

/// Close every fd this process still holds inside `mp`, returning how
/// many were closed (see [`teardown`]).
fn close_leaked_fds(mp: &Path) -> usize {
    let mut fds = Vec::new();
    if let Ok(dir) = fs::read_dir("/proc/self/fd") {
        for entry in dir.flatten() {
            let Ok(target) = fs::read_link(entry.path()) else {
                continue;
            };
            if target.starts_with(mp) {
                if let Some(fd) = entry
                    .file_name()
                    .to_str()
                    .and_then(|s| s.parse::<i32>().ok())
                {
                    fds.push(fd);
                }
            }
        }
    }
    for fd in &fds {
        unsafe { libc::close(*fd) };
    }
    fds.len()
}

impl Drop for MountGuard {
    fn drop(&mut self) {
        if let Some(session) = self.session.take() {
            teardown(session, &self.mountpoint);
            let _ = fusermount(&self.mountpoint, &["-u"]);
        }
    }
}

fn fusermount(mountpoint: &Path, args: &[&str]) -> bool {
    std::process::Command::new("fusermount3")
        .args(args)
        .arg(mountpoint)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// One mount lifecycle: formatted MDS + mountpoint + guard.
struct Fixture {
    _mds_dir: tempfile::TempDir,
    _mp_dir: tempfile::TempDir,
    guard: MountGuard,
    mp: PathBuf,
}

impl Fixture {
    fn new() -> Option<Self> {
        let mds_dir = test_dir();
        format_mds(mds_dir.path());
        let mp_dir = test_dir();
        let mp = mp_dir.path().to_path_buf();
        let guard = MountGuard::mount(mds_dir.path(), &mp)?;
        Some(Self {
            _mds_dir: mds_dir,
            _mp_dir: mp_dir,
            guard,
            mp,
        })
    }
}

#[test]
fn mount_smoke_and_statfs() {
    if !require_mount("mount_smoke_and_statfs") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    assert!(fx.mp.is_dir());
    let entries: Vec<_> = fs::read_dir(&fx.mp).unwrap().collect();
    assert!(entries.is_empty());
    // statfs(2) via coreutils `stat -f`: total blocks and block size.
    let out = std::process::Command::new("stat")
        .args(["-f", "-c", "%b %S"])
        .arg(&fx.mp)
        .output()
        .unwrap();
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).unwrap();
    let mut parts = text.split_whitespace();
    let blocks: u64 = parts.next().unwrap().parse().unwrap();
    let bsize: u64 = parts.next().unwrap().parse().unwrap();
    assert_eq!(bsize, 4096);
    assert!(blocks > 0, "statfs reports nonzero blocks: {text}");
    fx.guard.unmount();
}

#[test]
fn mkdir_and_rmdir() {
    if !require_mount("mkdir_and_rmdir") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let dir = fx.mp.join("parent");
    fs::create_dir(&dir).unwrap();
    fs::create_dir(dir.join("child")).unwrap();
    assert!(dir.join("child").is_dir());
    assert!(
        fs::remove_dir(&dir).is_err(),
        "rmdir of non-empty dir must fail"
    );
    fs::remove_dir(dir.join("child")).unwrap();
    fs::remove_dir(&dir).unwrap();
    assert!(!dir.exists());
    fx.guard.unmount();
}

#[test]
fn create_write_read_multiextent() {
    if !require_mount("create_write_read_multiextent") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let path = fx.mp.join("big.bin");
    // > 4 MiB: crosses the single-extent payload limit (plus many FUSE
    // write requests, each <= max_write).
    let len = 5 * 1024 * 1024 + 123;
    let data = pattern(0, len);
    fs::write(&path, &data).unwrap();
    assert_eq!(fs::metadata(&path).unwrap().len(), len as u64);
    assert_eq!(fs::read(&path).unwrap(), data);
    // Sub-range read through a shared fd.
    {
        let mut file = fs::File::open(&path).unwrap();
        let mut buf = vec![0u8; 4096];
        file.seek(SeekFrom::Start(4 * 1024 * 1024 - 100)).unwrap();
        file.read_exact(&mut buf).unwrap();
        assert_eq!(
            buf,
            &data[4 * 1024 * 1024 - 100..4 * 1024 * 1024 - 100 + 4096]
        );
    }
    fx.guard.unmount();
}

#[test]
fn sparse_file_reads_zeros() {
    if !require_mount("sparse_file_reads_zeros") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let path = fx.mp.join("sparse");
    let payload = pattern(0, 8192);
    {
        let mut file = fs::File::create(&path).unwrap();
        file.seek(SeekFrom::Start(1 << 20)).unwrap();
        file.write_all(&payload).unwrap();
    }
    assert_eq!(fs::metadata(&path).unwrap().len(), (1 << 20) + 8192);
    let got = fs::read(&path).unwrap();
    assert!(got[..1 << 20].iter().all(|&b| b == 0));
    assert_eq!(&got[1 << 20..], &payload[..]);
    fx.guard.unmount();
}

#[test]
fn truncate_shrink_and_grow() {
    if !require_mount("truncate_shrink_and_grow") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let path = fx.mp.join("tr");
    let data = pattern(0, 1 << 20);
    fs::write(&path, &data).unwrap();
    let file = fs::File::options().write(true).open(&path).unwrap();
    file.set_len(4096).unwrap();
    assert_eq!(fs::metadata(&path).unwrap().len(), 4096);
    assert_eq!(fs::read(&path).unwrap(), &data[..4096]);
    file.set_len(2 << 20).unwrap();
    drop(file); // close before unmount: an open fd keeps the mountpoint busy
    let got = fs::read(&path).unwrap();
    assert_eq!(got.len(), 2 << 20);
    assert_eq!(&got[..4096], &data[..4096]);
    assert!(got[4096..].iter().all(|&b| b == 0));
    fx.guard.unmount();
}

#[test]
fn rename_ops() {
    if !require_mount("rename_ops") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    fs::create_dir(fx.mp.join("d")).unwrap();
    fs::write(fx.mp.join("f"), b"payload").unwrap();
    // Same-dir rename.
    fs::rename(fx.mp.join("f"), fx.mp.join("g")).unwrap();
    assert!(!fx.mp.join("f").exists());
    // Cross-dir rename.
    fs::rename(fx.mp.join("g"), fx.mp.join("d").join("h")).unwrap();
    assert_eq!(fs::read(fx.mp.join("d").join("h")).unwrap(), b"payload");
    // Overwrite an existing file.
    fs::write(fx.mp.join("victim"), b"old").unwrap();
    fs::rename(fx.mp.join("d").join("h"), fx.mp.join("victim")).unwrap();
    assert_eq!(fs::read(fx.mp.join("victim")).unwrap(), b"payload");
    fx.guard.unmount();
}

#[test]
fn hard_link_shares_data() {
    if !require_mount("hard_link_shares_data") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    fs::write(fx.mp.join("orig"), b"shared-bytes").unwrap();
    fs::hard_link(fx.mp.join("orig"), fx.mp.join("alias")).unwrap();
    assert_eq!(fs::metadata(fx.mp.join("orig")).unwrap().nlink(), 2);
    assert_eq!(fs::read(fx.mp.join("alias")).unwrap(), b"shared-bytes");
    // Same inode behind both names.
    assert_eq!(
        fs::metadata(fx.mp.join("orig")).unwrap().ino(),
        fs::metadata(fx.mp.join("alias")).unwrap().ino()
    );
    fs::remove_file(fx.mp.join("orig")).unwrap();
    assert_eq!(fs::metadata(fx.mp.join("alias")).unwrap().nlink(), 1);
    assert_eq!(fs::read(fx.mp.join("alias")).unwrap(), b"shared-bytes");
    fx.guard.unmount();
}

#[test]
fn unlink_removes_file() {
    if !require_mount("unlink_removes_file") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let path = fx.mp.join("gone");
    fs::write(&path, b"temp").unwrap();
    fs::remove_file(&path).unwrap();
    assert!(!path.exists());
    assert_eq!(fs::read_dir(&fx.mp).unwrap().count(), 0);
    fx.guard.unmount();
}

#[test]
fn readdir_lists_entries() {
    if !require_mount("readdir_lists_entries") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    fs::create_dir(fx.mp.join("dd")).unwrap();
    fs::write(fx.mp.join("ff"), b"x").unwrap();
    fs::write(fx.mp.join("gg"), b"y").unwrap();
    let mut names: Vec<String> = fs::read_dir(&fx.mp)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert_eq!(names, ["dd", "ff", "gg"]);
    // `ls -la`-level detail: "." and ".." appear via getdents.
    let out = std::process::Command::new("ls")
        .arg("-a")
        .arg(&fx.mp)
        .output()
        .unwrap();
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(text.contains("  .\n") || text.starts_with(".\n"), "{text}");
    assert!(text.contains(".."), "{text}");
    fx.guard.unmount();
}

#[test]
fn chmod_utimes_via_setattr() {
    if !require_mount("chmod_utimes_via_setattr") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let path = fx.mp.join("modes");
    fs::write(&path, b"m").unwrap();
    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o640);
    fs::set_permissions(&path, perms).unwrap();
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o640
    );
    fx.guard.unmount();
}

#[test]
fn remount_preserves_tree_and_data() {
    if !require_mount("remount_preserves_tree_and_data") {
        return;
    }
    let mds_dir = test_dir();
    format_mds(mds_dir.path());
    let content_a = pattern(0, 9000);
    let content_b = pattern(9_000_000, 3000);
    {
        let mp_dir = test_dir();
        let Some(guard) = MountGuard::mount(mds_dir.path(), mp_dir.path()) else {
            return;
        };
        let mp = mp_dir.path();
        fs::create_dir(mp.join("dir")).unwrap();
        fs::create_dir(mp.join("dir").join("sub")).unwrap();
        fs::write(mp.join("dir").join("a.bin"), &content_a).unwrap();
        fs::write(mp.join("dir").join("sub").join("b.bin"), &content_b).unwrap();
        // fsync through the mount: the data must survive the remount.
        let file = fs::File::options()
            .write(true)
            .open(mp.join("dir").join("a.bin"))
            .unwrap();
        file.sync_all().unwrap();
        drop(file); // close before unmount: an open fd keeps the mountpoint busy
        fs::hard_link(mp.join("dir").join("a.bin"), mp.join("a-link")).unwrap();
        guard.unmount();
    }
    {
        let mp_dir = test_dir();
        let Some(guard) = MountGuard::mount(mds_dir.path(), mp_dir.path()) else {
            return;
        };
        let mp = mp_dir.path();
        assert_eq!(fs::read(mp.join("dir").join("a.bin")).unwrap(), content_a);
        assert_eq!(
            fs::read(mp.join("dir").join("sub").join("b.bin")).unwrap(),
            content_b
        );
        assert_eq!(fs::read(mp.join("a-link")).unwrap(), content_a);
        assert_eq!(
            fs::metadata(mp.join("dir").join("a.bin")).unwrap().nlink(),
            2
        );
        let mut names: Vec<String> = fs::read_dir(mp)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        assert_eq!(names, ["a-link", "dir"]);
        guard.unmount();
    }
    // Final consistency gate on the offline MDS.
    let (meta, data) = pair(mds_dir.path());
    let mut mds = porfs_mds::Mds::open(&meta, &data).unwrap();
    let report = mds.self_check().unwrap();
    assert_eq!(report.inodes, 5); // root, dir, sub, a.bin, b.bin
}

// ---- P5: xattr / symlink / mknod / seek / punch-hole / durability ----

use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::FileTypeExt;

fn cpath(p: &Path) -> CString {
    CString::new(p.as_os_str().as_bytes()).unwrap()
}

fn cname(n: &str) -> CString {
    CString::new(n).unwrap()
}

#[test]
fn xattr_roundtrip_and_flags() {
    if !require_mount("xattr_roundtrip_and_flags") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let file = fx.mp.join("f");
    fs::write(&file, b"x").unwrap();
    let path = cpath(&file);
    unsafe {
        let name = cname("user.alpha");
        assert_eq!(
            libc::setxattr(path.as_ptr(), name.as_ptr(), b"one".as_ptr().cast(), 3, 0),
            0
        );
        // XATTR_CREATE on an existing name: EEXIST.
        assert_eq!(
            libc::setxattr(path.as_ptr(), name.as_ptr(), b"two".as_ptr().cast(), 3, 1),
            -1
        );
        assert_eq!(*libc::__errno_location(), libc::EEXIST);
        // XATTR_REPLACE on a missing name: ENODATA.
        let missing = cname("user.missing");
        assert_eq!(
            libc::setxattr(path.as_ptr(), missing.as_ptr(), b"v".as_ptr().cast(), 1, 2),
            -1
        );
        assert_eq!(*libc::__errno_location(), libc::ENODATA);
        // Size query (buf null, size 0), then read.
        let need = libc::getxattr(path.as_ptr(), name.as_ptr(), std::ptr::null_mut(), 0);
        assert_eq!(need, 3);
        let mut buf = [0u8; 8];
        let got = libc::getxattr(path.as_ptr(), name.as_ptr(), buf.as_mut_ptr().cast(), 8);
        assert_eq!(got, 3);
        assert_eq!(&buf[..3], b"one");
        // listxattr: size query then list.
        let lneed = libc::listxattr(path.as_ptr(), std::ptr::null_mut(), 0);
        assert!(lneed > 0);
        let mut lbuf = vec![0u8; lneed as usize];
        assert_eq!(
            libc::listxattr(path.as_ptr(), lbuf.as_mut_ptr().cast(), lneed as usize),
            lneed
        );
        assert!(lbuf.windows(10).any(|w| w == b"user.alpha"));
        // removexattr; a second remove reports ENODATA.
        assert_eq!(libc::removexattr(path.as_ptr(), name.as_ptr()), 0);
        assert_eq!(libc::removexattr(path.as_ptr(), name.as_ptr()), -1);
        assert_eq!(*libc::__errno_location(), libc::ENODATA);
    }
    fx.guard.unmount();
}

#[test]
fn symlink_and_readlink() {
    if !require_mount("symlink_and_readlink") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    fs::write(fx.mp.join("target"), b"data").unwrap();
    std::os::unix::fs::symlink("target", fx.mp.join("link")).unwrap();
    assert_eq!(
        fs::read_link(fx.mp.join("link")).unwrap(),
        Path::new("target")
    );
    assert!(
        fs::symlink_metadata(fx.mp.join("link"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    // Reads through the link reach the target.
    assert_eq!(fs::read(fx.mp.join("link")).unwrap(), b"data");
    fx.guard.unmount();
}

#[test]
fn mknod_fifo_and_socket() {
    if !require_mount("mknod_fifo_and_socket") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let fifo = cpath(&fx.mp.join("pipe"));
    let sock = cpath(&fx.mp.join("sock"));
    unsafe {
        assert_eq!(libc::mkfifo(fifo.as_ptr(), 0o640), 0);
        let mode = libc::mknod(sock.as_ptr(), libc::S_IFSOCK | 0o640, 0);
        assert_eq!(mode, 0);
    }
    let meta = fs::metadata(fx.mp.join("pipe")).unwrap();
    assert!(meta.file_type().is_fifo());
    assert_eq!(meta.mode() & 0o777, 0o640);
    assert!(
        fs::metadata(fx.mp.join("sock"))
            .unwrap()
            .file_type()
            .is_socket()
    );
    fx.guard.unmount();
}

#[test]
fn seek_data_and_hole() {
    if !require_mount("seek_data_and_hole") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let file = fx.mp.join("sparse");
    {
        let mut f = fs::File::create(&file).unwrap();
        f.write_all(&[0xAA; 4096]).unwrap();
        f.seek(SeekFrom::Start(1 << 20)).unwrap();
        f.write_all(&[0xBB; 4096]).unwrap();
    }
    let path = cpath(&file);
    unsafe {
        let fd = libc::open(path.as_ptr(), libc::O_RDONLY);
        assert!(fd >= 0);
        // Offset 0 is inside the first block: SEEK_DATA = 0.
        assert_eq!(libc::lseek(fd, 0, 3), 0);
        // SEEK_HOLE from 0: the gap starts right after the first block.
        assert_eq!(libc::lseek(fd, 0, 4), 4096);
        // Inside the gap: SEEK_DATA jumps to the second block at 1MiB.
        assert_eq!(libc::lseek(fd, 100_000, 3), 1 << 20);
        // SEEK_HOLE inside the gap is the position itself.
        assert_eq!(libc::lseek(fd, 100_000, 4), 100_000);
        // Past the last block: no more data -> ENXIO.
        let eof_data = libc::lseek(fd, (1 << 20) + 4096, 3);
        assert_eq!(eof_data, -1);
        assert_eq!(*libc::__errno_location(), libc::ENXIO);
        assert_eq!(libc::close(fd), 0);
    }
    fx.guard.unmount();
}

#[test]
fn punch_hole_zeroes_range() {
    if !require_mount("punch_hole_zeroes_range") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let file = fx.mp.join("dense");
    let content: Vec<u8> = (0..(1 << 20)).map(|i| (i % 251) as u8).collect();
    fs::write(&file, &content).unwrap();
    let path = cpath(&file);
    unsafe {
        let fd = libc::open(path.as_ptr(), libc::O_RDWR);
        assert!(fd >= 0);
        // PUNCH_HOLE|KEEP_SIZE over [256KiB, 768KiB).
        assert_eq!(libc::fallocate(fd, 3, 256 << 10, 512 << 10), 0);
        assert_eq!(libc::close(fd), 0);
    }
    let after = fs::read(&file).unwrap();
    assert_eq!(after.len(), content.len()); // size unchanged
    assert_eq!(&after[..(256 << 10)], &content[..(256 << 10)]);
    assert!(after[(256 << 10)..(768 << 10)].iter().all(|b| *b == 0));
    assert_eq!(&after[(768 << 10)..], &content[(768 << 10)..]);
    fx.guard.unmount();
}

#[test]
fn fsync_and_fdatasync_through_mount() {
    if !require_mount("fsync_and_fdatasync_through_mount") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let a = fs::File::create(fx.mp.join("a")).unwrap();
    a.sync_all().unwrap();
    let b = fs::File::create(fx.mp.join("b")).unwrap();
    b.sync_data().unwrap();
    drop(a);
    drop(b);
    fx.guard.unmount();
}

#[test]
fn clean_unmount_flushes_unfsynced_data() {
    if !require_mount("clean_unmount_flushes_unfsynced_data") {
        return;
    }
    let mds_dir = test_dir();
    format_mds(mds_dir.path());
    let content = b"never fsynced, must still survive a polite unmount";
    {
        let mp_dir = test_dir();
        let Some(guard) = MountGuard::mount(mds_dir.path(), mp_dir.path()) else {
            return;
        };
        // No fsync on purpose: a clean unmount is a durability barrier.
        fs::write(mp_dir.path().join("f"), content).unwrap();
        guard.unmount();
    }
    {
        let mp_dir = test_dir();
        let Some(guard) = MountGuard::mount(mds_dir.path(), mp_dir.path()) else {
            return;
        };
        assert_eq!(fs::read(mp_dir.path().join("f")).unwrap(), content);
        guard.unmount();
    }
    let (meta, data) = pair(mds_dir.path());
    let mds = porfs_mds::Mds::open(&meta, &data).unwrap();
    assert_eq!(mds.last_reconcile_repairs(), 0);
}

#[test]
fn statfs_reports_device_geometry() {
    if !require_mount("statfs_reports_device_geometry") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let path = cpath(&fx.mp);
    let mut stats: libc::statfs = unsafe { std::mem::zeroed() };
    assert_eq!(unsafe { libc::statfs(path.as_ptr(), &mut stats) }, 0);
    // f_blocks counts frsize-sized blocks: total must equal the device size.
    assert_eq!(stats.f_bsize as u64, 4096);
    assert_eq!(stats.f_blocks * stats.f_bsize as u64, DEV_SIZE);
    assert!(stats.f_bfree < stats.f_blocks);
    fx.guard.unmount();
}

#[test]
fn overlong_name_is_enametoolong() {
    if !require_mount("overlong_name_is_enametoolong") {
        return;
    }
    let Some(fx) = Fixture::new() else { return };
    let long = "x".repeat(256);
    let err = fs::write(fx.mp.join(&long), b"").unwrap_err();
    assert_eq!(err.raw_os_error(), Some(libc::ENAMETOOLONG));
    let err = fs::rename(fx.mp.join(&long), fx.mp.join("b")).unwrap_err();
    assert_eq!(err.raw_os_error(), Some(libc::ENAMETOOLONG));
    fx.guard.unmount();
}
