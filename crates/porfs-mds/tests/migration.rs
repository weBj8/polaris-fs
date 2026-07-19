//! Schema 1 -> 2 migration: a database written by the P5 layout (legacy
//! `file_extents` table with fixed 16-byte values, `schema_version = 1`)
//! opens via the migration path — rows land in `file_extents_v2`
//! byte-identical, the legacy table is dropped, and file data reads back
//! byte-exact.

mod common;

use porfs_mds::{Mds, MdsError, ROOT_INO};
use porfs_store::ExtentStore;
use redb::{Database, ReadableDatabase, ReadableTableMetadata, TableDefinition};
use twox_hash::XxHash64;

use common::{DEV_SIZE, pair, pattern, test_dir};

/// Mirror of the crate-private InodeRec for the hand-built fixture
/// (bincode is field-order based, so the layouts match — the same trick
/// crash.rs uses for its planted-corruption fixtures).
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
    link_target: Option<Vec<u8>>,
    rdev: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
enum KindMirror {
    File,
    Dir,
}

const INODES_T: TableDefinition<&[u8; 8], &[u8]> = TableDefinition::new("inodes");
const DIR_ENTRIES_T: TableDefinition<&[u8], &[u8; 8]> = TableDefinition::new("dir_entries");
/// The legacy schema-1 table: fixed 16-byte values (`id BE ++ len BE`).
const LEGACY_EXTENTS_T: TableDefinition<&[u8], &[u8; 16]> = TableDefinition::new("file_extents");
const CURRENT_EXTENTS_T: TableDefinition<&[u8], &[u8]> = TableDefinition::new("file_extents_v2");
const XATTRS_T: TableDefinition<&[u8], &[u8]> = TableDefinition::new("xattrs");
const META_T: TableDefinition<&str, u64> = TableDefinition::new("meta");

const FILE_INO: u64 = 2;

fn rec(kind: KindMirror, size: u64) -> Vec<u8> {
    bincode::serialize(&RecMirror {
        kind,
        mode: 0o644,
        uid: 0,
        gid: 0,
        size,
        atime: (1_700_000_000, 0),
        mtime: (1_700_000_000, 0),
        ctime: (1_700_000_000, 0),
        nlink: 1,
        link_target: None,
        rdev: 0,
    })
    .unwrap()
}

/// Hand-build a schema-1 database: legacy tables, `schema_version = 1`,
/// a root directory, and one file with two extent rows.
fn build_schema_1_fixture(meta: &std::path::Path, extent_ids: [u64; 2], lens: [u64; 2]) {
    let db = Database::create(meta).unwrap();
    let txn = db.begin_write().unwrap();
    {
        let mut inodes = txn.open_table(INODES_T).unwrap();
        txn.open_table(DIR_ENTRIES_T).unwrap();
        let mut entries = txn.open_table(DIR_ENTRIES_T).unwrap();
        let mut fext = txn.open_table(LEGACY_EXTENTS_T).unwrap();
        txn.open_table(XATTRS_T).unwrap();
        let mut meta_t = txn.open_table(META_T).unwrap();

        inodes
            .insert(&ROOT_INO.to_be_bytes(), rec(KindMirror::Dir, 0).as_slice())
            .unwrap();
        inodes
            .insert(
                &FILE_INO.to_be_bytes(),
                rec(KindMirror::File, lens.iter().sum()).as_slice(),
            )
            .unwrap();

        // The file's directory entry under root (schema-1 hash-ordered key).
        let name = "f";
        let mut key = ROOT_INO.to_be_bytes().to_vec();
        key.extend_from_slice(&XxHash64::oneshot(0, name.as_bytes()).to_be_bytes());
        key.extend_from_slice(name.as_bytes());
        entries
            .insert(key.as_slice(), &FILE_INO.to_be_bytes())
            .unwrap();

        for (i, (&id, &len)) in extent_ids.iter().zip(&lens).enumerate() {
            let mut key = FILE_INO.to_be_bytes().to_vec();
            key.extend_from_slice(&((i as u64) * 4096).to_be_bytes());
            let mut value = [0u8; 16];
            value[..8].copy_from_slice(&id.to_be_bytes());
            value[8..].copy_from_slice(&len.to_be_bytes());
            fext.insert(key.as_slice(), &value).unwrap();
        }

        meta_t.insert("next_ino", FILE_INO + 1).unwrap();
        meta_t.insert("schema_version", 1u64).unwrap();
    }
    txn.commit().unwrap();
    drop(db);
}

#[test]
fn schema_1_database_migrates_on_open() {
    let dir = test_dir();
    let (meta, data) = pair(dir.path());

    // The extent device the schema-1 rows point at.
    let first = pattern(0, 4096);
    let second = pattern(1_000_000, 4096);
    let extent_ids = {
        let mut store = ExtentStore::create(&data, DEV_SIZE).unwrap();
        let ids = store
            .append_batch(&[
                (FILE_INO, 0, first.as_slice()),
                (FILE_INO, 4096, second.as_slice()),
            ])
            .unwrap();
        store.sync().unwrap();
        drop(store);
        [ids[0], ids[1]]
    };
    build_schema_1_fixture(&meta, extent_ids, [4096, 4096]);

    // Open: the migration runs, then the reconcile (all extents live).
    let mut mds = Mds::open(&meta, &data).unwrap();
    assert_eq!(mds.last_reconcile_repairs(), 0);

    // File data reads back byte-exact.
    let mut want = first.clone();
    want.extend_from_slice(&second);
    assert_eq!(mds.read(FILE_INO, 0, 8192).unwrap(), want);
    mds.self_check().unwrap();
    // redb holds an flock: drop the MDS before opening the raw handle.
    drop(mds);

    // The schema is stamped 2 and the legacy table is gone.
    {
        let db = Database::open(&meta).unwrap();
        let txn = db.begin_read().unwrap();
        let meta_t = txn.open_table(META_T).unwrap();
        assert_eq!(meta_t.get("schema_version").unwrap().unwrap().value(), 2);
        let legacy = txn.open_table(LEGACY_EXTENTS_T);
        assert!(
            matches!(legacy, Err(redb::TableError::TableDoesNotExist(_))),
            "legacy file_extents must be dropped, got {legacy:?}"
        );
        let current = txn.open_table(CURRENT_EXTENTS_T).unwrap();
        assert_eq!(current.len().unwrap(), 2);
        drop(txn);
        drop(db);
    }

    // The migrated MDS keeps working: a post-migration write round-trips
    // through a close/reopen.
    let mut mds = Mds::open(&meta, &data).unwrap();
    let third = pattern(2_000_000, 2048);
    mds.write(FILE_INO, 8192, &third).unwrap();
    mds.fsync(FILE_INO).unwrap();
    drop(mds);
    let mut mds = Mds::open(&meta, &data).unwrap();
    assert_eq!(mds.last_reconcile_repairs(), 0);
    assert_eq!(mds.read(FILE_INO, 8192, 2048).unwrap(), third);
    mds.self_check().unwrap();
}

#[test]
fn schema_1_database_without_root_is_rejected() {
    let dir = test_dir();
    let (meta, data) = pair(dir.path());
    drop(ExtentStore::create(&data, DEV_SIZE).unwrap());
    build_schema_1_fixture(&meta, [0, 1], [4096, 4096]);

    // Damage the fixture: a migrated database without a root inode is
    // reported as corruption, never a panic or a silent half-open.
    {
        let db = Database::open(&meta).unwrap();
        let txn = db.begin_write().unwrap();
        {
            let mut inodes = txn.open_table(INODES_T).unwrap();
            inodes.remove(&ROOT_INO.to_be_bytes()).unwrap();
        }
        txn.commit().unwrap();
        drop(db);
    }
    let err = match Mds::open(&meta, &data) {
        Ok(_) => panic!("a rootless database must not open"),
        Err(err) => err,
    };
    assert!(matches!(err, MdsError::Corrupt(_)), "{err:?}");
}
