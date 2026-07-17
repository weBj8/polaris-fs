//! Namespace operations: format, create/mkdir, lookup, readdir, link/unlink,
//! rmdir, the full rename matrix, and name validation.

mod common;

use porfs_mds::{InodeAttr, MdsError, NodeKind, ROOT_INO};

use common::{format_mds, open_mds, test_dir};

fn names(entries: &[(String, u64, NodeKind)]) -> Vec<&str> {
    entries.iter().map(|e| e.0.as_str()).collect()
}

/// readdir order is hash order by design (big-directory index); tests
/// compare SETS of names, not sequences.
fn sorted_names(entries: &[(String, u64, NodeKind)]) -> Vec<&str> {
    let mut names = names(entries);
    names.sort_unstable();
    names
}

#[test]
fn format_creates_empty_root() {
    let dir = test_dir();
    let mut mds = format_mds(dir.path());
    let root = mds.getattr(ROOT_INO).unwrap();
    assert_eq!(root.kind, NodeKind::Dir);
    assert_eq!(root.mode, 0o755);
    assert_eq!(root.nlink, 1);
    assert_eq!(root.size, 0);
    assert!(mds.readdir(ROOT_INO).unwrap().is_empty());
    let report = mds.self_check().unwrap();
    assert_eq!(report.inodes, 1);
}

#[test]
fn create_lookup_getattr() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "file", 0o644, 0, 0).unwrap();
    assert_eq!(mds.lookup(ROOT_INO, "file").unwrap(), f);
    let attr: InodeAttr = mds.getattr(f).unwrap();
    assert_eq!(attr.kind, NodeKind::File);
    assert_eq!(attr.mode, 0o644);
    assert_eq!(attr.nlink, 1);
    assert_eq!(attr.size, 0);
}

#[test]
fn create_existing_name_fails() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    mds.create(ROOT_INO, "dup", 0o644, 0, 0).unwrap();
    let err = mds.create(ROOT_INO, "dup", 0o644, 0, 0).unwrap_err();
    assert!(matches!(err, MdsError::Exists(_)), "{err:?}");
    assert!(matches!(
        mds.mkdir(ROOT_INO, "dup", 0o755, 0, 0).unwrap_err(),
        MdsError::Exists(_)
    ));
}

#[test]
fn mkdir_nested_and_readdir_order() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let a = mds.mkdir(ROOT_INO, "a", 0o755, 0, 0).unwrap();
    let b = mds.mkdir(a, "b", 0o700, 0, 0).unwrap();
    let f = mds.create(b, "f", 0o600, 0, 0).unwrap();
    mds.create(ROOT_INO, "z", 0o644, 0, 0).unwrap();
    assert_eq!(mds.lookup(a, "b").unwrap(), b);
    assert_eq!(mds.lookup(b, "f").unwrap(), f);
    assert_eq!(mds.getattr(b).unwrap().mode, 0o700);
    // Hash-ordered listing: compare the name set, and resolve kinds per name.
    let root_entries = mds.readdir(ROOT_INO).unwrap();
    assert_eq!(sorted_names(&root_entries), ["a", "z"]);
    for (name, ino, kind) in &root_entries {
        match name.as_str() {
            "a" => {
                assert_eq!(*ino, a);
                assert_eq!(*kind, NodeKind::Dir);
            }
            "z" => assert_eq!(*kind, NodeKind::File),
            other => panic!("unexpected entry {other:?}"),
        }
    }
    assert_eq!(sorted_names(&mds.readdir(b).unwrap()), ["f"]);
}

#[test]
fn dir_ops_on_files_fail() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    assert!(matches!(mds.readdir(f).unwrap_err(), MdsError::NotDir(_)));
    assert!(matches!(
        mds.lookup(f, "x").unwrap_err(),
        MdsError::NotDir(_)
    ));
    assert!(matches!(
        mds.create(f, "x", 0o644, 0, 0).unwrap_err(),
        MdsError::NotDir(_)
    ));
}

#[test]
fn unlink_removes_file() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    mds.unlink(ROOT_INO, "f").unwrap();
    assert!(matches!(
        mds.lookup(ROOT_INO, "f").unwrap_err(),
        MdsError::NotFound(_)
    ));
    assert!(matches!(mds.getattr(f).unwrap_err(), MdsError::NotFound(_)));
    assert!(matches!(
        mds.unlink(ROOT_INO, "f").unwrap_err(),
        MdsError::NotFound(_)
    ));
    assert!(mds.readdir(ROOT_INO).unwrap().is_empty());
}

#[test]
fn unlink_dir_fails() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    mds.mkdir(ROOT_INO, "d", 0o755, 0, 0).unwrap();
    assert!(matches!(
        mds.unlink(ROOT_INO, "d").unwrap_err(),
        MdsError::IsDir(_)
    ));
}

#[test]
fn rmdir_rules() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let d = mds.mkdir(ROOT_INO, "d", 0o755, 0, 0).unwrap();
    mds.create(d, "child", 0o644, 0, 0).unwrap();
    assert!(matches!(
        mds.rmdir(ROOT_INO, "d").unwrap_err(),
        MdsError::NotEmpty(_)
    ));
    mds.unlink(d, "child").unwrap();
    mds.rmdir(ROOT_INO, "d").unwrap();
    assert!(matches!(
        mds.lookup(ROOT_INO, "d").unwrap_err(),
        MdsError::NotFound(_)
    ));
    // rmdir on a file is ENOTDIR.
    mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    assert!(matches!(
        mds.rmdir(ROOT_INO, "f").unwrap_err(),
        MdsError::NotDir(_)
    ));
}

#[test]
fn hard_links_share_inode_and_data() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "orig", 0o644, 0, 0).unwrap();
    mds.write(f, 0, b"shared").unwrap();
    mds.link(f, ROOT_INO, "alias").unwrap();
    assert_eq!(mds.getattr(f).unwrap().nlink, 2);
    assert_eq!(mds.lookup(ROOT_INO, "alias").unwrap(), f);
    mds.unlink(ROOT_INO, "orig").unwrap();
    assert_eq!(mds.getattr(f).unwrap().nlink, 1);
    // The surviving link still reads the data.
    assert_eq!(mds.read(f, 0, 100).unwrap(), b"shared");
    // Directories cannot be hard-linked.
    let d = mds.mkdir(ROOT_INO, "d", 0o755, 0, 0).unwrap();
    assert!(matches!(
        mds.link(d, ROOT_INO, "d2").unwrap_err(),
        MdsError::IsDir(_)
    ));
    assert!(matches!(
        mds.link(999, ROOT_INO, "ghost").unwrap_err(),
        MdsError::NotFound(_)
    ));
}

#[test]
fn rename_same_dir_keeps_inode() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "before", 0o644, 0, 0).unwrap();
    mds.rename(ROOT_INO, "before", ROOT_INO, "after").unwrap();
    assert!(matches!(
        mds.lookup(ROOT_INO, "before").unwrap_err(),
        MdsError::NotFound(_)
    ));
    assert_eq!(mds.lookup(ROOT_INO, "after").unwrap(), f);
}

#[test]
fn rename_cross_dir_keeps_data() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let d = mds.mkdir(ROOT_INO, "d", 0o755, 0, 0).unwrap();
    let f = mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    mds.write(f, 0, b"payload").unwrap();
    mds.rename(ROOT_INO, "f", d, "g").unwrap();
    assert_eq!(mds.lookup(d, "g").unwrap(), f);
    assert_eq!(mds.read(f, 0, 100).unwrap(), b"payload");
    assert!(matches!(
        mds.lookup(ROOT_INO, "f").unwrap_err(),
        MdsError::NotFound(_)
    ));
}

#[test]
fn rename_overwrites_existing_file() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let a = mds.create(ROOT_INO, "a", 0o644, 0, 0).unwrap();
    mds.write(a, 0, b"winner").unwrap();
    let b = mds.create(ROOT_INO, "b", 0o644, 0, 0).unwrap();
    mds.write(b, 0, b"loser").unwrap();
    mds.rename(ROOT_INO, "a", ROOT_INO, "b").unwrap();
    assert_eq!(mds.lookup(ROOT_INO, "b").unwrap(), a);
    assert_eq!(mds.read(a, 0, 100).unwrap(), b"winner");
    assert!(matches!(mds.getattr(b).unwrap_err(), MdsError::NotFound(_)));
    let report = mds.self_check().unwrap();
    assert_eq!(report.inodes, 2); // root + a
}

#[test]
fn rename_dir_over_empty_dir() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let a = mds.mkdir(ROOT_INO, "a", 0o755, 0, 0).unwrap();
    mds.create(a, "child", 0o644, 0, 0).unwrap();
    let b = mds.mkdir(ROOT_INO, "b", 0o755, 0, 0).unwrap();
    mds.rename(ROOT_INO, "a", ROOT_INO, "b").unwrap();
    assert_eq!(mds.lookup(ROOT_INO, "b").unwrap(), a);
    assert!(matches!(mds.getattr(b).unwrap_err(), MdsError::NotFound(_)));
    assert_eq!(sorted_names(&mds.readdir(a).unwrap()), ["child"]);
    mds.self_check().unwrap();
}

#[test]
fn rename_dir_over_nonempty_dir_fails() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    mds.mkdir(ROOT_INO, "a", 0o755, 0, 0).unwrap();
    let b = mds.mkdir(ROOT_INO, "b", 0o755, 0, 0).unwrap();
    mds.create(b, "occupied", 0o644, 0, 0).unwrap();
    assert!(matches!(
        mds.rename(ROOT_INO, "a", ROOT_INO, "b").unwrap_err(),
        MdsError::NotEmpty(_)
    ));
    // Nothing changed.
    assert_eq!(sorted_names(&mds.readdir(ROOT_INO).unwrap()), ["a", "b"]);
}

#[test]
fn rename_kind_mismatch_fails() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    mds.mkdir(ROOT_INO, "d", 0o755, 0, 0).unwrap();
    assert!(matches!(
        mds.rename(ROOT_INO, "f", ROOT_INO, "d").unwrap_err(),
        MdsError::IsDir(_)
    ));
    assert!(matches!(
        mds.rename(ROOT_INO, "d", ROOT_INO, "f").unwrap_err(),
        MdsError::NotDir(_)
    ));
    assert!(matches!(
        mds.rename(ROOT_INO, "missing", ROOT_INO, "x").unwrap_err(),
        MdsError::NotFound(_)
    ));
}

#[test]
fn rename_onto_same_name_is_noop() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    mds.rename(ROOT_INO, "f", ROOT_INO, "f").unwrap();
    assert_eq!(mds.lookup(ROOT_INO, "f").unwrap(), f);
    // Renaming a hard link onto another name of the same inode is a no-op.
    mds.link(f, ROOT_INO, "g").unwrap();
    mds.rename(ROOT_INO, "f", ROOT_INO, "g").unwrap();
    assert_eq!(mds.getattr(f).unwrap().nlink, 2);
    assert_eq!(sorted_names(&mds.readdir(ROOT_INO).unwrap()), ["f", "g"]);
}

#[test]
fn invalid_names_rejected() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644, 0, 0).unwrap();
    let long = "x".repeat(256);
    fn expect_reject<T>(bad: &str, r: porfs_mds::Result<T>) -> bool {
        if bad.len() > porfs_mds::MAX_NAME_LEN {
            matches!(r, Err(MdsError::NameTooLong(_)))
        } else {
            matches!(r, Err(MdsError::InvalidName(_)))
        }
    }
    for bad in ["", "a/b", ".", "..", long.as_str()] {
        assert!(
            expect_reject(bad, mds.create(ROOT_INO, bad, 0o644, 0, 0)),
            "create accepted {bad:?}"
        );
        assert!(expect_reject(bad, mds.mkdir(ROOT_INO, bad, 0o755, 0, 0)));
        assert!(expect_reject(bad, mds.link(f, ROOT_INO, bad)));
        assert!(expect_reject(bad, mds.lookup(ROOT_INO, bad)));
        assert!(expect_reject(bad, mds.rename(ROOT_INO, "f", ROOT_INO, bad)));
        assert!(expect_reject(bad, mds.unlink(ROOT_INO, bad)));
    }
    // A 255-byte name is legal.
    let max = "y".repeat(255);
    mds.create(ROOT_INO, &max, 0o644, 0, 0).unwrap();
    assert!(mds.lookup(ROOT_INO, &max).is_ok());
}

#[test]
fn reopen_preserves_namespace() {
    let dir = test_dir();
    let mut mds = format_mds(dir.path());
    let d = mds.mkdir(ROOT_INO, "d", 0o755, 0, 0).unwrap();
    let f = mds.create(d, "f", 0o640, 0, 0).unwrap();
    mds.link(f, ROOT_INO, "alias").unwrap();
    drop(mds);

    let mds = &mut open_mds(dir.path());
    assert_eq!(mds.last_reconcile_repairs(), 0);
    assert_eq!(mds.lookup(d, "f").unwrap(), f);
    assert_eq!(mds.lookup(ROOT_INO, "alias").unwrap(), f);
    let attr = mds.getattr(f).unwrap();
    assert_eq!(attr.mode, 0o640);
    assert_eq!(attr.nlink, 2);
    assert_eq!(
        sorted_names(&mds.readdir(ROOT_INO).unwrap()),
        ["alias", "d"]
    );
    mds.self_check().unwrap();
}

#[test]
fn format_refuses_reformat() {
    let dir = test_dir();
    let mds = format_mds(dir.path());
    drop(mds);
    let (meta, data) = common::pair(dir.path());
    let result = porfs_mds::Mds::format(&meta, &data, common::DEV_SIZE);
    assert!(matches!(result, Err(MdsError::Exists(_))));
}
