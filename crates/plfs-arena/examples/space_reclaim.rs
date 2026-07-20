//! Space-reclamation gate helper (scripts/gate-space-reclaim.sh): fill an
//! arena with chunks, delete them (optionally bitmap-only), sparsify, and
//! report real allocated blocks (`st_blocks`) at each step.

use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use plfs_arena::{Arena, ChunkId, MkfsConfig, SlotClass};

fn blocks(path: &std::path::Path) -> u64 {
    std::fs::metadata(path)
        .map(|m| m.blocks() * 512)
        .unwrap_or(0)
}

/// Xorshift64 stream — not compressible, so block counts stay honest.
fn payload(seed: u64, len: usize) -> Vec<u8> {
    let mut x = seed.max(1);
    let mut out = Vec::with_capacity(len);
    while out.len() < len {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        out.extend_from_slice(&x.to_le_bytes());
    }
    out.truncate(len);
    out
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap_or_default();
    let arena = PathBuf::from(args.next().expect("arena path"));
    match cmd.as_str() {
        "mkfs" => {
            let gib: u64 = args.next().expect("gib").parse()?;
            Arena::mkfs(
                &arena,
                &MkfsConfig {
                    total_bytes: gib << 30,
                    l_fraction: 0.90,
                    l_slot_size: 1 << 20,
                    s_slot_size: 64 << 10,
                },
            )?;
            println!("blocks={}", blocks(&arena));
        }
        "fill" => {
            let gib: u64 = args.next().expect("gib").parse()?;
            let target = gib << 30;
            let mut a = Arena::open(&arena)?;
            let cap = a.geometry().capacity(SlotClass::L) as usize;
            let mut written = 0u64;
            let mut chunks = 0u64;
            while written < target {
                let data = payload(written + 1, cap);
                a.put(ChunkId::new_v7(), written, &data)?;
                written += data.len() as u64;
                chunks += 1;
            }
            println!(
                "written={written} chunks={chunks} blocks={}",
                blocks(&arena)
            );
        }
        "delete-all" => {
            let nopunch = args.next().is_some_and(|a| a == "nopunch");
            let mut a = Arena::open(&arena)?;
            if nopunch {
                a.set_punch_enabled(false);
            }
            let metas = a.list();
            let n = metas.len();
            for m in metas {
                if !a.delete(&m.chunk_id, m.version)? {
                    return Err(format!("delete returned false for {}", m.chunk_id).into());
                }
            }
            println!("deleted={n} blocks={}", blocks(&arena));
        }
        // Bitmap-only delete leaves headers on disk, so a reopen would
        // resurrect the chunks (headers are truth, contract §5): the
        // sparsify pass must run in the SAME session while the in-memory
        // bitmap still marks the slots free.
        "reclaim-all" => {
            let mut a = Arena::open(&arena)?;
            a.set_punch_enabled(false);
            let metas = a.list();
            let n = metas.len();
            for m in metas {
                if !a.delete(&m.chunk_id, m.version)? {
                    return Err(format!("delete returned false for {}", m.chunk_id).into());
                }
            }
            let retained = blocks(&arena);
            a.set_punch_enabled(true);
            let r = a.sparsify()?;
            println!(
                "deleted={n} retained={retained} reclaimed={} blocks={}",
                r.bytes_reclaimed,
                blocks(&arena)
            );
        }
        "sparsify" => {
            let mut a = Arena::open(&arena)?;
            let r = a.sparsify()?;
            println!(
                "punched={} ranges={} reclaimed={} blocks={}",
                r.punched,
                r.free_ranges_punched,
                r.bytes_reclaimed,
                blocks(&arena)
            );
        }
        "blocks" => {
            println!("{}", blocks(&arena));
        }
        other => {
            return Err(format!(
                "unknown command {other:?}; want mkfs|fill|delete-all|sparsify|blocks"
            )
            .into());
        }
    }
    Ok(())
}
