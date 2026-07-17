//! `porfs bench`: sequential write, sequential read, and random-read phases
//! over a freshly created device, with a summary table.

use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result, ensure};
use porfs_format::{BLOCK_SIZE, DATA_START, EXTENT_DATA_MAX, extent_disk_len};
use porfs_store::ExtentStore;

use crate::{human, parse_size};

pub(crate) fn cmd_bench(
    device: &Path,
    size: &str,
    extent_size: &str,
    queue_depth: u32,
) -> Result<()> {
    let size = parse_size(size)?;
    let extent_size = parse_size(extent_size)?;
    ensure!(
        extent_size > 0 && extent_size <= EXTENT_DATA_MAX,
        "extent-size must be in 1..={EXTENT_DATA_MAX} bytes"
    );
    let count = size / extent_size;
    ensure!(
        count > 0,
        "size {size} is smaller than one extent {extent_size}"
    );
    let device_size = DATA_START + count * extent_disk_len(extent_size) + BLOCK_SIZE;

    let mut store = ExtentStore::create(device, device_size)
        .with_context(|| format!("mkfs bench device {}", device.display()))?;
    store.set_queue_depth(queue_depth);
    let qd = usize::try_from(queue_depth).context("queue depth too large")?;
    let bytes = count * extent_size;
    println!(
        "bench: {count} extents x {} = {} on {} ({}, qd {queue_depth})",
        human(extent_size),
        human(bytes),
        device.display(),
        crate::io_mode(store.is_direct())
    );

    // Phase 1: sequential write of `count` extents (deterministic xorshift
    // content, one pre-filled buffer reused across batches), batched.
    let esz = extent_size as usize;
    let batch_cap = qd.min(count as usize);
    let mut batch_buf = vec![0u8; batch_cap * esz];
    for (j, chunk) in batch_buf.chunks_exact_mut(esz).enumerate() {
        fill_pattern(chunk, j as u64);
    }
    let mut ids = Vec::with_capacity(count as usize);
    let t0 = Instant::now();
    let mut done = 0u64;
    while done < count {
        let batch_n = qd.min((count - done) as usize);
        let items: Vec<(u64, u64, &[u8])> = batch_buf[..batch_n * esz]
            .chunks_exact(esz)
            .enumerate()
            .map(|(j, chunk)| (0u64, (done + j as u64) * extent_size, chunk))
            .collect();
        let batch_ids = store.append_batch(&items)?;
        ids.extend_from_slice(&batch_ids);
        done += batch_n as u64;
    }
    let write_secs = t0.elapsed().as_secs_f64();
    let t0 = Instant::now();
    store.sync()?;
    let sync_secs = t0.elapsed().as_secs_f64();
    let w = report("seq-write", bytes, count, write_secs);
    println!("  sync: {sync_secs:.3} s");

    // Phase 2: pipelined sequential read of all extents via read_batch_into
    // (zero-copy API). The store verifies each record's header (extent id
    // included) and data CRC32C; the dst set is allocated once and reused.
    let mut dsts: Vec<Vec<u8>> = (0..qd).map(|_| Vec::with_capacity(esz)).collect();
    let t0 = Instant::now();
    for chunk in ids.chunks(qd) {
        store.read_batch_into(chunk, &mut dsts[..chunk.len()])?;
        std::hint::black_box(&dsts);
    }
    let r = report("seq-read", bytes, count, t0.elapsed().as_secs_f64());

    // Phase 3: same extents in deterministic pseudo-random order.
    let mut order = ids.clone();
    shuffle(&mut order, 0x9E37_79B9_7F4A_7C15);
    let t0 = Instant::now();
    for chunk in order.chunks(qd) {
        store.read_batch_into(chunk, &mut dsts[..chunk.len()])?;
        std::hint::black_box(&dsts);
    }
    let rr = report("random-read", bytes, count, t0.elapsed().as_secs_f64());

    println!();
    println!("summary:");
    println!("  {:<12} {:>12} {:>12}", "phase", "MB/s", "IOPS");
    for (name, (mb, iops)) in [("seq-write", w), ("seq-read", r), ("random-read", rr)] {
        println!("  {name:<12} {mb:>12.1} {iops:>12.0}");
    }
    Ok(())
}

/// One bench result line; returns (MB/s, IOPS) for the summary table.
fn report(name: &str, bytes: u64, ios: u64, secs: f64) -> (f64, f64) {
    let mb_s = bytes as f64 / (1024.0 * 1024.0) / secs;
    let iops = ios as f64 / secs;
    println!("  {name}: {mb_s:.1} MB/s, {iops:.0} IOPS ({secs:.3} s)");
    (mb_s, iops)
}

/// Fill `buf` with a deterministic xorshift64 pattern derived from `seed`.
fn fill_pattern(buf: &mut [u8], seed: u64) {
    let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut chunks = buf.chunks_exact_mut(8);
    for chunk in &mut chunks {
        chunk.copy_from_slice(&next().to_le_bytes());
    }
    let rem = chunks.into_remainder();
    if !rem.is_empty() {
        rem.copy_from_slice(&next().to_le_bytes()[..rem.len()]);
    }
}

/// In-place Fisher-Yates shuffle driven by a deterministic xorshift64 stream.
fn shuffle(ids: &mut [u64], seed: u64) {
    let mut state = seed | 1;
    for i in (1..ids.len()).rev() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let j = (state % (i as u64 + 1)) as usize;
        ids.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_is_deterministic() {
        let mut a = vec![0u8; 1000];
        let mut b = vec![0u8; 1000];
        fill_pattern(&mut a, 7);
        fill_pattern(&mut b, 7);
        assert_eq!(a, b);
        fill_pattern(&mut b, 8);
        assert_ne!(a, b);
    }

    #[test]
    fn shuffle_is_a_permutation() {
        let mut v: Vec<u64> = (0..100).collect();
        let orig = v.clone();
        shuffle(&mut v, 42);
        let mut sorted = v.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, orig);
        assert_ne!(v, orig);
    }
}
