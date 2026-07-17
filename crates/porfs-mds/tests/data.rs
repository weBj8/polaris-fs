//! The file data path: write/read roundtrips, sparse holes, exact and
//! partial overwrites (the read-modify-write merge), multi-extent files,
//! truncate shrink/grow, and EOF clamping.

mod common;

use porfs_format::EXTENT_DATA_MAX;
use porfs_mds::{MdsError, NodeKind, ROOT_INO, SetAttr};

use common::{format_mds, pattern, test_dir};

const KIB: u64 = 1024;

#[test]
fn write_read_roundtrip() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let data = pattern(0, 4096);
    assert_eq!(mds.write(f, 0, &data).unwrap(), data.len());
    assert_eq!(mds.getattr(f).unwrap().size, 4096);
    assert_eq!(mds.read(f, 0, 1 << 20).unwrap(), data);
    // Sub-range read.
    assert_eq!(mds.read(f, 100, 200).unwrap(), &data[100..300]);
    mds.self_check().unwrap();
}

#[test]
fn sparse_holes_read_as_zeros() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let data = pattern(0, 2048);
    mds.write(f, KIB, &data).unwrap();
    assert_eq!(mds.getattr(f).unwrap().size, KIB + 2048);
    let got = mds.read(f, 0, KIB + 2048).unwrap();
    assert_eq!(&got[..KIB as usize], &vec![0u8; KIB as usize][..]);
    assert_eq!(&got[KIB as usize..], &data[..]);
    // A read entirely inside the hole is all zeros.
    assert_eq!(mds.read(f, 0, 512).unwrap(), vec![0u8; 512]);
}

#[test]
fn exact_overwrite_replaces_extent() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let a = pattern(0, 8192);
    let b = pattern(1_000_000, 8192);
    mds.write(f, 0, &a).unwrap();
    mds.write(f, 0, &b).unwrap();
    assert_eq!(mds.read(f, 0, 8192).unwrap(), b);
    mds.self_check().unwrap();
}

#[test]
fn partial_head_overwrite_merges() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let a = pattern(0, 8192);
    mds.write(f, 0, &a).unwrap();
    // Overwrite the second half: the kept first half is RMW-merged.
    let b = pattern(2_000_000, 4096);
    mds.write(f, 4096, &b).unwrap();
    let mut want = a.clone();
    want[4096..].copy_from_slice(&b);
    assert_eq!(mds.read(f, 0, 8192).unwrap(), want);
}

#[test]
fn partial_tail_overwrite_merges() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let a = pattern(0, 8192);
    mds.write(f, 0, &a).unwrap();
    // Overwrite the first half: the kept second half is RMW-merged.
    let b = pattern(3_000_000, 4096);
    mds.write(f, 0, &b).unwrap();
    let mut want = a.clone();
    want[..4096].copy_from_slice(&b);
    assert_eq!(mds.read(f, 0, 8192).unwrap(), want);
}

#[test]
fn partial_middle_overwrite_merges_both_sides() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let a = pattern(0, 12288);
    mds.write(f, 0, &a).unwrap();
    // Overwrite the middle third of one extent: both sides are kept via RMW.
    let b = pattern(4_000_000, 4096);
    mds.write(f, 4096, &b).unwrap();
    let mut want = a.clone();
    want[4096..8192].copy_from_slice(&b);
    assert_eq!(mds.read(f, 0, 12288).unwrap(), want);
    mds.self_check().unwrap();
}

#[test]
fn append_growth_with_gap() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let a = pattern(0, 4096);
    let b = pattern(5_000_000, 4096);
    mds.write(f, 0, &a).unwrap();
    mds.write(f, 8192, &b).unwrap(); // leaves a 4 KiB hole
    assert_eq!(mds.getattr(f).unwrap().size, 12288);
    let got = mds.read(f, 0, 12288).unwrap();
    assert_eq!(&got[..4096], &a[..]);
    assert_eq!(&got[4096..8192], &[0u8; 4096][..]);
    assert_eq!(&got[8192..], &b[..]);
}

#[test]
fn multi_extent_file_roundtrip() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "big", 0o644).unwrap();
    // 9 MiB in one write: 4 MiB + 4 MiB + 1 MiB = three extents.
    let len = (2 * EXTENT_DATA_MAX + (1 << 20)) as usize;
    let data = pattern(0, len);
    assert_eq!(mds.write(f, 0, &data).unwrap(), len);
    assert_eq!(mds.getattr(f).unwrap().size, len as u64);
    assert_eq!(mds.read(f, 0, len as u64).unwrap(), data);
    // A read straddling the 4 MiB extent boundary.
    let off = EXTENT_DATA_MAX - 1000;
    assert_eq!(
        mds.read(f, off, 2000).unwrap(),
        &data[off as usize..off as usize + 2000]
    );
    mds.self_check().unwrap();
}

#[test]
fn truncate_shrink_straddler_keeps_prefix() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let a = pattern(0, 8192);
    mds.write(f, 0, &a).unwrap();
    // 8192 -> 4096 cuts the single extent in half (straddler rewrite).
    let attr = mds
        .setattr(
            f,
            SetAttr {
                size: Some(4096),
                ..SetAttr::default()
            },
        )
        .unwrap();
    assert_eq!(attr.size, 4096);
    assert_eq!(mds.read(f, 0, 8192).unwrap(), &a[..4096]);
    assert!(mds.read(f, 4096, 10).unwrap().is_empty());
    mds.self_check().unwrap();
}

#[test]
fn truncate_shrink_drops_trailing_extents() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let a = pattern(0, 8192);
    let b = pattern(6_000_000, 8192);
    mds.write(f, 0, &a).unwrap();
    mds.write(f, 8192, &b).unwrap();
    mds.setattr(
        f,
        SetAttr {
            size: Some(4096),
            ..SetAttr::default()
        },
    )
    .unwrap();
    assert_eq!(mds.read(f, 0, 1 << 20).unwrap(), &a[..4096]);
    mds.self_check().unwrap();
}

#[test]
fn truncate_grow_reads_back_zeros() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let a = pattern(0, 4096);
    mds.write(f, 0, &a).unwrap();
    let attr = mds
        .setattr(
            f,
            SetAttr {
                size: Some(1 << 20),
                ..SetAttr::default()
            },
        )
        .unwrap();
    assert_eq!(attr.size, 1 << 20);
    let got = mds.read(f, 0, 1 << 20).unwrap();
    assert_eq!(&got[..4096], &a[..]);
    assert_eq!(&got[4096..], &vec![0u8; (1 << 20) - 4096][..]);
}

#[test]
fn read_past_eof_is_clamped() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let a = pattern(0, 4096);
    mds.write(f, 0, &a).unwrap();
    assert_eq!(mds.read(f, 0, 1 << 20).unwrap(), a);
    assert!(mds.read(f, 4096, 100).unwrap().is_empty());
    assert!(mds.read(f, 1 << 30, 100).unwrap().is_empty());
    assert!(mds.read(f, 0, 0).unwrap().is_empty());
}

#[test]
fn setattr_updates_fields() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    let before = mds.getattr(f).unwrap();
    let attr = mds
        .setattr(
            f,
            SetAttr {
                mode: Some(0o600),
                uid: Some(1000),
                gid: Some(1001),
                mtime: Some((1_700_000_000, 42)),
                ..SetAttr::default()
            },
        )
        .unwrap();
    assert_eq!(attr.mode, 0o600);
    assert_eq!(attr.uid, 1000);
    assert_eq!(attr.gid, 1001);
    assert_eq!(attr.mtime, (1_700_000_000, 42));
    assert_eq!(attr.atime, before.atime); // untouched fields stay
    assert!(attr.ctime >= before.ctime); // ctime always refreshes
}

#[test]
fn data_ops_on_dirs_fail() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let d = mds.mkdir(ROOT_INO, "d", 0o755).unwrap();
    assert!(matches!(mds.write(d, 0, b"x"), Err(MdsError::IsDir(_))));
    assert!(matches!(mds.read(d, 0, 1), Err(MdsError::IsDir(_))));
    assert!(matches!(
        mds.setattr(
            d,
            SetAttr {
                size: Some(10),
                ..SetAttr::default()
            }
        ),
        Err(MdsError::IsDir(_))
    ));
    // Mode-only setattr on a directory is fine.
    let attr = mds
        .setattr(
            d,
            SetAttr {
                mode: Some(0o700),
                ..SetAttr::default()
            },
        )
        .unwrap();
    assert_eq!(attr.mode, 0o700);
    assert_eq!(attr.kind, NodeKind::Dir);
}

#[test]
fn unlinked_file_data_is_gone() {
    let dir = test_dir();
    let mds = &mut format_mds(dir.path());
    let f = mds.create(ROOT_INO, "f", 0o644).unwrap();
    mds.write(f, 0, &pattern(0, 8192)).unwrap();
    mds.unlink(ROOT_INO, "f").unwrap();
    let report = mds.self_check().unwrap();
    assert_eq!(report.extents, 0);
    // Recreating reuses no stale data.
    let g = mds.create(ROOT_INO, "g", 0o644).unwrap();
    assert!(mds.read(g, 0, 100).unwrap().is_empty());
}
