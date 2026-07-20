//! S6 gate proptest (design doc §7.2): crash at random byte offsets ⇒
//! prefix-consistent replay. Model: run random Append/Sync sequences, then
//! simulate the crash by (a) truncating the last segment at a random byte
//! offset that respects fsync durability (bytes covered by a completed
//! sync/segment-switch survive), or (b) appending random garbage to the
//! last segment (torn tail). Properties after recovery:
//!   - replayed records are exactly a PREFIX of the appended sequence
//!     (no gaps, no torn records, payloads byte-exact);
//!   - every acknowledged record (covered by a completed sync) survives;
//!   - the recovered WAL accepts new appends and keeps the prefix stable.
//!
//! Case count: PROPTEST_CASES env (default 8 — CI runs every test < 5 s).

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use plfs_client::wal::{RecordPos, Wal};
use proptest::prelude::*;

static NEXT: AtomicU64 = AtomicU64::new(0);

const SEGMENT_LEN: u64 = 2048;

#[derive(Debug, Clone)]
enum Op {
    Append(Vec<u8>),
    Sync,
}

fn op_strategy() -> impl Strategy<Value = Op> {
    prop_oneof![
        6 => prop::collection::vec(any::<u8>(), 1..512usize).prop_map(Op::Append),
        1 => Just(Op::Sync),
    ]
}

fn crash_dir(case: u64) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("plfs-wal-gate-{}-{case}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("tmpdir");
    dir
}

fn run_case(ops: Vec<Op>, truncate_at: u64, garbage_tail: bool, case: u64) {
    let dir = crash_dir(case);
    let (mut wal, _) = Wal::open(&dir, SEGMENT_LEN).expect("open");
    let mut appended: Vec<(Vec<u8>, RecordPos)> = Vec::new();
    let mut last_sync: Option<RecordPos> = None;
    let mut ack_count = 0usize;
    for op in &ops {
        match op {
            Op::Append(p) => {
                let pos = wal.append(p).expect("append");
                appended.push((p.clone(), pos));
            }
            Op::Sync => {
                last_sync = Some(wal.sync().expect("sync"));
                ack_count = appended.len();
            }
        }
    }
    let last_segment = wal.append_pos().segment;
    let file_len = appended
        .last()
        .filter(|(_, p)| p.segment == last_segment)
        .map_or(16, |(_, p)| p.offset);
    // fsync durability floor in the last segment: the create fsync covers
    // the header; a completed sync covers everything up to its position.
    let floor = match last_sync {
        Some(p) if p.segment == last_segment => p.offset,
        _ => 16,
    };
    drop(wal);

    let seg_path = dir.join(format!("seg_{last_segment:016}.wal"));
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&seg_path)
        .expect("open segment");
    if garbage_tail {
        use std::os::unix::fs::FileExt;
        let garbage: Vec<u8> = (0..(1 + truncate_at % 64) as u8)
            .map(|b| b ^ 0x5A)
            .collect();
        file.write_all_at(&garbage, file_len).expect("garbage");
    } else {
        let at = floor + truncate_at % (file_len - floor + 1);
        file.set_len(at).expect("truncate");
    }
    drop(file);

    let (mut wal, records) = Wal::open(&dir, SEGMENT_LEN).expect("recover");
    // Property 1: replayed == appended[..k] (prefix, byte-exact).
    assert!(
        records.len() <= appended.len(),
        "replay longer than history"
    );
    for (i, rec) in records.iter().enumerate() {
        assert_eq!(
            rec.payload, appended[i].0,
            "record {i} diverges from history"
        );
    }
    // Property 2: every acknowledged record survives.
    assert!(
        records.len() >= ack_count,
        "lost acknowledged records: replayed {} < acked {}",
        records.len(),
        ack_count
    );
    if garbage_tail {
        assert_eq!(
            records.len(),
            appended.len(),
            "garbage tail must not eat real records"
        );
    }
    // Property 3: the recovered WAL continues the prefix.
    let extra: Vec<Vec<u8>> = (0..3u8).map(|i| vec![0xE0 | i; 17 + i as usize]).collect();
    for p in &extra {
        wal.append(p).expect("append after recovery");
    }
    wal.sync().expect("sync after recovery");
    drop(wal);
    let (_wal, records2) = Wal::open(&dir, SEGMENT_LEN).expect("reopen");
    let want: Vec<&Vec<u8>> = appended[..records.len()]
        .iter()
        .map(|(p, _)| p)
        .chain(extra.iter())
        .collect();
    let got: Vec<&Vec<u8>> = records2.iter().map(|r| &r.payload).collect();
    assert_eq!(got, want, "history + post-recovery appends must replay");

    let _ = std::fs::remove_dir_all(&dir);
}

fn proptest_cases() -> u32 {
    std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(proptest_cases()))]
    #[test]
    fn crash_at_random_byte_offset_prefix_consistent(
        ops in prop::collection::vec(op_strategy(), 1..60),
        truncate_at in any::<u64>(),
        garbage_tail in any::<bool>(),
    ) {
        let case = NEXT.fetch_add(1, Ordering::Relaxed);
        run_case(ops, truncate_at, garbage_tail, case);
    }
}
