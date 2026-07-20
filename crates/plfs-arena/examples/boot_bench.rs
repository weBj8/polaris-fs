//! v2 bounded-boot bench: fill an arena, clean-close it, then time the
//! reopen via the checkpoint vs the full slot-header scan.

use plfs_arena::{Arena, BootMode, ChunkId, MkfsConfig};

fn main() {
    let mut args = std::env::args().skip(1);
    let dir = std::path::PathBuf::from(args.next().expect("dir"));
    let chunks: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(2000);
    let total_gib: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(4);
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("arena.img");
    Arena::mkfs(
        &path,
        &MkfsConfig {
            total_bytes: total_gib << 30,
            l_fraction: 0.9,
            l_slot_size: 1 << 20,
            s_slot_size: 64 << 10,
        },
    )
    .expect("mkfs");
    let mut arena = Arena::open(&path).expect("open");
    let payload = vec![0x5Au8; 100_000];
    for _ in 0..chunks {
        arena.put(ChunkId::new_v7(), 1, &payload).expect("put");
    }
    arena.close().expect("close");

    let t = std::time::Instant::now();
    let arena = Arena::open(&path).expect("reopen (checkpoint)");
    let fast = t.elapsed();
    assert_eq!(arena.boot_mode(), BootMode::Checkpoint);
    drop(arena);

    let t = std::time::Instant::now();
    let arena = Arena::open(&path).expect("reopen (scan)");
    let scan = t.elapsed();
    assert_eq!(arena.boot_mode(), BootMode::Scan);
    drop(arena);

    println!(
        "BOOT_BENCH chunks={chunks} checkpoint_ms={} scan_ms={}",
        fast.as_millis(),
        scan.as_millis()
    );
}
