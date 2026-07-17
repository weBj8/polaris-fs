//! Crash survival: unclean shutdowns, mount-time reconcile, planted
//! corruption, and the 20-iteration random unclean-reopen loop (the P3
//! "metadata survives crashes" gate evidence).
//!
//! A note on simulating "unclean" in-process: the phase contract asks for
//! `std::mem::forget` (kill -9 semantics), but redb holds an `flock` on the
//! metadata file — a forgotten (leaked) `Database` keeps the lock and any
//! same-process reopen fails with `DatabaseAlreadyOpen`. `drop(mds)` is
//! recovery-equivalent here: redb commits fdatasync per transaction, and
//! `ExtentStore::drop` only persists *already confirmed* superblock state —
//! un-synced appends are salvaged on remount exactly as after a kill -9.
//! True-kill coverage of the store layer itself is the P2
//! `scripts/kill9-soak.sh` harness (re-run for this phase).

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::os::unix::fs::FileExt;

use porfs_format::{DATA_START, extent_disk_len};
use porfs_mds::{MdsError, NodeKind, ROOT_INO, SetAttr};
use redb::{Database, ReadableTable, TableDefinition};

use common::{format_mds, open_mds, pair, pattern, test_dir};

/// Mirror of the crate-private InodeRec for the planted-corruption tests
/// (bincode is field-order based, so the layouts match).
#[derive(serde::Serialize, serde::Deserialize)]
struct RecMirror {
    kind: KindMirror,
    mode: u32,
    uid: u32,
    gid: u32,
    size: u64,
    atime: (i64, u32),
    mtime: (i64, u32),
    ctime: (i64, u32),
    nlink: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
enum KindMirror {
    File,
    Dir,
}

const INODES_T: TableDefinition<&[u8; 8], &[u8]> = TableDefinition::new("inodes");
const DIR_ENTRIES_T: TableDefinition<&[u8], &[u8; 8]> = TableDefinition::new("dir_entries");
const FILE_EXTENTS_T: TableDefinition<&[u8], &[u8; 16]> = TableDefinition::new("file_extents");

#[test]
fn unclean_reopen_preserves_namespace_and_data() {
    let dir = test_dir();
    let mut mds = format_mds(dir.path());
    let d = mds.mkdir(ROOT_INO, "dir", 0o755).unwrap();
    let sub = mds.mkdir(d, "sub", 0o700).unwrap();
    let f1 = mds.create(d, "f1", 0o644).unwrap();
    let f2 = mds.create(sub, "f2", 0o600).unwrap();
    let d1 = pattern(0, 6000);
    let d2 = pattern(7_000_000, 3000);
    mds.write(f1, 0, &d1).unwrap();
    mds.write(f2, 1024, &d2).unwrap();
    mds.link(f1, ROOT_INO, "f1_alias").unwrap();
    mds.rename(d, "f1", sub, "f1_moved").unwrap();
    mds.fsync(f1).unwrap();
    mds.fsync(f2).unwrap();
    drop(mds); // unclean: no shutdown beyond destructors (see header note)

    let mut mds = open_mds(dir.path());
    assert_eq!(mds.last_reconcile_repairs(), 0);
    assert_eq!(mds.lookup(sub, "f1_moved").unwrap(), f1);
    assert_eq!(mds.lookup(ROOT_INO, "f1_alias").unwrap(), f1);
    assert_eq!(mds.getattr(f1).unwrap().nlink, 2);
    assert_eq!(mds.read(f1, 0, 1 << 20).unwrap(), d1);
    let mut want_f2 = vec![0u8; 1024];
    want_f2.extend_from_slice(&d2);
    assert_eq!(mds.read(f2, 0, 1 << 20).unwrap(), want_f2);
    let root_names: Vec<String> = mds
        .readdir(ROOT_INO)
        .unwrap()
        .into_iter()
        .map(|e| e.0)
        .collect();
    assert_eq!(root_names, ["dir", "f1_alias"]);
    let report = mds.self_check().unwrap();
    assert_eq!(report.inodes, 5); // root, dir, sub, f1, f2
}

#[test]
fn reconcile_repairs_unsynced_write() {
    let dir = test_dir();
    let mut mds = format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let data = pattern(0, 8192);
    mds.write(f, 0, &data).unwrap(); // committed metadata, UNconfirmed data
    drop(mds);

    let mut mds = open_mds(dir.path());
    assert!(mds.last_reconcile_repairs() >= 1);
    // Metadata survived (size), but the unconfirmed extent was salvaged and
    // the map row repaired into a hole.
    assert_eq!(mds.getattr(f).unwrap().size, 8192);
    assert_eq!(mds.read(f, 0, 8192).unwrap(), vec![0u8; 8192]);
    mds.self_check().unwrap();
    // The file stays writable afterwards.
    mds.write(f, 0, b"after").unwrap();
    mds.fsync(f).unwrap();
    assert_eq!(mds.read(f, 0, 5).unwrap(), b"after");
}

#[test]
fn reconcile_after_torn_tail_record() {
    let dir = test_dir();
    let mut mds = format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let a = pattern(0, 4096);
    let b = pattern(8_000_000, 4096);
    mds.write(f, 0, &a).unwrap();
    mds.fsync(f).unwrap();
    mds.write(f, 4096, &b).unwrap();
    mds.fsync(f).unwrap();
    drop(mds);

    // Corrupt the second record's header on disk (torn-write simulation):
    // the mount scan must salvage-truncate the log there, and the MDS
    // reconcile must drop the now-dangling map row.
    let (_meta, data) = pair(dir.path());
    let second_record = DATA_START + extent_disk_len(4096);
    {
        let dev = OpenOptions::new().write(true).open(&data).unwrap();
        dev.write_all_at(&[0xFF; 64], second_record).unwrap();
        dev.sync_all().unwrap();
    }
    let mut mds = open_mds(dir.path());
    assert!(mds.last_reconcile_repairs() >= 1);
    let got = mds.read(f, 0, 8192).unwrap();
    assert_eq!(&got[..4096], &a[..]); // first extent intact
    assert_eq!(&got[4096..], &[0u8; 4096][..]); // torn tail reads as a hole
    mds.self_check().unwrap();
}

#[test]
fn fsync_makes_data_durable() {
    let dir = test_dir();
    let mut mds = format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let data = pattern(0, 12288);
    mds.write(f, 0, &data).unwrap();
    mds.fsync(f).unwrap();
    drop(mds);

    let mut mds = open_mds(dir.path());
    assert_eq!(mds.last_reconcile_repairs(), 0);
    assert_eq!(mds.read(f, 0, 1 << 20).unwrap(), data);
    mds.self_check().unwrap();
}

#[test]
fn self_check_detects_planted_dangling_entry() {
    let dir = test_dir();
    let mut mds = format_mds(dir.path());
    mds.create(ROOT_INO, "f", 0o644).unwrap();
    drop(mds);

    // Plant a directory entry pointing at a nonexistent inode, via raw redb.
    let (meta, _data) = pair(dir.path());
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

    let mut mds = open_mds(dir.path());
    let err = mds.self_check().unwrap_err();
    assert!(matches!(err, MdsError::Corrupt(_)), "{err:?}");
}

#[test]
fn self_check_detects_planted_nlink_mismatch() {
    let dir = test_dir();
    let mut mds = format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    drop(mds);

    // Plant an nlink bump (2 links recorded, only 1 entry exists).
    let (meta, _data) = pair(dir.path());
    let db = Database::open(&meta).unwrap();
    let txn = db.begin_write().unwrap();
    {
        let mut inodes = txn.open_table(INODES_T).unwrap();
        let key = f.to_be_bytes();
        let raw = inodes.get(&key).unwrap().unwrap().value().to_vec();
        let mut rec: RecMirror = bincode::deserialize(&raw).unwrap();
        rec.nlink = 2;
        inodes
            .insert(&key, bincode::serialize(&rec).unwrap().as_slice())
            .unwrap();
    }
    txn.commit().unwrap();
    drop(db);

    let mut mds = open_mds(dir.path());
    let err = mds.self_check().unwrap_err();
    assert!(matches!(err, MdsError::Corrupt(_)), "{err:?}");
}

#[test]
fn reconcile_repairs_planted_dangling_extent_row() {
    let dir = test_dir();
    let mut mds = format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    mds.write(f, 0, &pattern(0, 4096)).unwrap();
    mds.fsync(f).unwrap();
    drop(mds);

    // Plant a map row pointing at an extent id that never existed.
    let (meta, _data) = pair(dir.path());
    let db = Database::open(&meta).unwrap();
    let txn = db.begin_write().unwrap();
    {
        let mut fext = txn.open_table(FILE_EXTENTS_T).unwrap();
        let mut key = f.to_be_bytes().to_vec();
        key.extend_from_slice(&1_000_000u64.to_be_bytes());
        let mut value = [0u8; 16];
        value[..8].copy_from_slice(&999_999u64.to_be_bytes());
        value[8..].copy_from_slice(&100u64.to_be_bytes());
        fext.insert(key.as_slice(), &value).unwrap();
    }
    txn.commit().unwrap();
    drop(db);

    let mut mds = open_mds(dir.path());
    assert!(mds.last_reconcile_repairs() >= 1);
    let got = mds.read(f, 0, 1_000_100).unwrap();
    assert_eq!(&got[..4096], &pattern(0, 4096)[..]);
    assert!(got[4096..].iter().all(|&b| b == 0));
    mds.self_check().unwrap();
}

/// xorshift64* deterministic randomness (same construction as the P2 crash
/// harness): reproducible runs, no dependencies.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// The P3 gate loop: 20 rounds of random namespace+data ops, an fsync, one
/// deliberately un-fsynced scratch write, an unclean drop, then a reopen
/// with reconcile + self_check + full model verification.
#[test]
fn random_unclean_reopen_loop() {
    const ITERS: u64 = 20;
    const OPS: u64 = 25;
    let dir = test_dir();
    let mut rng = Rng(0x5EED_5EED_5EED_5EED);
    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    let mut dirs: BTreeSet<String> = BTreeSet::new();
    let mut counter = 0u64;

    for iter in 0..ITERS {
        let mut mds = if iter == 0 {
            format_mds(dir.path())
        } else {
            open_mds(dir.path())
        };

        for _ in 0..OPS {
            counter += 1;
            match rng.below(100) {
                0..=29 => {
                    let name = format!("f{counter}");
                    mds.create(ROOT_INO, &name, 0o644).unwrap();
                    files.insert(name, Vec::new());
                }
                30..=59 => {
                    if files.is_empty() {
                        continue;
                    }
                    let idx = rng.below(files.len() as u64) as usize;
                    let name = files.keys().nth(idx).unwrap().clone();
                    let ino = mds.lookup(ROOT_INO, &name).unwrap();
                    let offset = rng.below(16 * 1024);
                    let len = 1 + rng.below(8 * 1024) as usize;
                    let data = pattern(rng.next(), len);
                    mds.write(ino, offset, &data).unwrap();
                    let model = files.get_mut(&name).unwrap();
                    let end = offset as usize + data.len();
                    if model.len() < end {
                        model.resize(end, 0);
                    }
                    model[offset as usize..end].copy_from_slice(&data);
                }
                60..=69 => {
                    let name = format!("d{counter}");
                    mds.mkdir(ROOT_INO, &name, 0o755).unwrap();
                    dirs.insert(name);
                }
                70..=79 => {
                    if files.is_empty() {
                        continue;
                    }
                    let idx = rng.below(files.len() as u64) as usize;
                    let name = files.keys().nth(idx).unwrap().clone();
                    mds.unlink(ROOT_INO, &name).unwrap();
                    files.remove(&name);
                }
                80..=89 => {
                    if files.is_empty() {
                        continue;
                    }
                    let idx = rng.below(files.len() as u64) as usize;
                    let name = files.keys().nth(idx).unwrap().clone();
                    let new_name = format!("r{counter}");
                    mds.rename(ROOT_INO, &name, ROOT_INO, &new_name).unwrap();
                    let content = files.remove(&name).unwrap();
                    files.insert(new_name, content);
                }
                90..=94 => {
                    if dirs.is_empty() {
                        continue;
                    }
                    let idx = rng.below(dirs.len() as u64) as usize;
                    let name = dirs.iter().nth(idx).unwrap().clone();
                    // The model's directories are always empty (flat layout).
                    mds.rmdir(ROOT_INO, &name).unwrap();
                    dirs.remove(&name);
                }
                _ => {
                    if files.is_empty() {
                        continue;
                    }
                    let idx = rng.below(files.len() as u64) as usize;
                    let name = files.keys().nth(idx).unwrap().clone();
                    let model = files.get_mut(&name).unwrap();
                    if model.is_empty() {
                        continue;
                    }
                    let new_size = rng.below(model.len() as u64 + 1) as usize;
                    let ino = mds.lookup(ROOT_INO, &name).unwrap();
                    mds.setattr(
                        ino,
                        SetAttr {
                            size: Some(new_size as u64),
                            ..SetAttr::default()
                        },
                    )
                    .unwrap();
                    model.truncate(new_size);
                }
            }
        }

        // Make everything durable, then deliberately leave one more write
        // un-fsynced so the mount-time reconcile has something to repair.
        mds.fsync(ROOT_INO).unwrap();
        let scratch = format!("s{iter}");
        let scratch_ino = mds.create(ROOT_INO, &scratch, 0o644).unwrap();
        mds.write(scratch_ino, 0, &pattern(iter, 4096)).unwrap();
        drop(mds); // unclean (see header note)

        let mut mds = open_mds(dir.path());
        assert!(
            mds.last_reconcile_repairs() >= 1,
            "iter {iter}: reconcile should repair the un-fsynced scratch write"
        );
        // The scratch file's metadata survived; its unconfirmed data reads
        // back as a hole (the store never confirmed it).
        assert_eq!(mds.getattr(scratch_ino).unwrap().size, 4096);
        assert_eq!(mds.read(scratch_ino, 0, 4096).unwrap(), vec![0u8; 4096]);
        mds.unlink(ROOT_INO, &scratch).unwrap();
        mds.self_check().unwrap();

        // Full model verification.
        for (name, content) in &files {
            let ino = mds.lookup(ROOT_INO, name).unwrap();
            let attr = mds.getattr(ino).unwrap();
            assert_eq!(attr.kind, NodeKind::File);
            assert_eq!(
                attr.size,
                content.len() as u64,
                "iter {iter} size of {name}"
            );
            assert_eq!(
                mds.read(ino, 0, content.len() as u64 + 1).unwrap(),
                *content,
                "iter {iter} content of {name}"
            );
        }
        let mut want_names: Vec<String> = files.keys().cloned().collect();
        want_names.extend(dirs.iter().cloned());
        want_names.sort();
        let got_names: Vec<String> = mds
            .readdir(ROOT_INO)
            .unwrap()
            .into_iter()
            .map(|e| e.0)
            .collect();
        assert_eq!(got_names, want_names, "iter {iter} readdir");
        drop(mds);
    }
}
