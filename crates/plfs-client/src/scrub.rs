//! Scrubber (design doc §10.3): walk the reachable chunk set, verify each
//! replica's payload crc against its slot header, repair rot from a healthy
//! replica, and sweep orphans (on-disk chunks no metadata root or GC entry
//! references).

use plfs_meta::MetaState;

use crate::cluster::{
    ClusterSink, REPAIR_RATE, delete_one, list_chunks, put_one, stat_one, throttle,
};
use crate::core::ClientError;

/// One scrub pass over the reachable set and every live node's inventory.
#[derive(Debug, Default)]
pub struct ScrubStats {
    /// Chunks verified.
    pub checked: u64,
    /// Replicas re-written from a healthy copy.
    pub repaired: u64,
    /// Orphaned chunks deleted.
    pub orphans_deleted: u64,
}

/// Run one scrub pass; `rate` paces I/O in bytes/sec (0 = unthrottled).
pub async fn scrub_once(
    sink: &mut ClusterSink,
    state: &MetaState,
    rate: u64,
) -> Result<ScrubStats, ClientError> {
    let mut stats = ScrubStats::default();
    sink.refresh_live().await?;
    // Reachable set: live layouts + every snapshot checkpoint.
    let mut reachable = state.chunk_map()?;
    for snap in state.list_snaps()? {
        for (id, vr) in state.open_checkpoint(snap.id)?.chunk_map()? {
            reachable.entry(id).or_insert(vr);
        }
    }
    let queued = state.gc_chunk_ids()?;
    for (chunk_id, (version, replicas)) in &reachable {
        stats.checked += 1;
        let mut good: Option<Vec<u8>> = None;
        let mut rotten: Vec<String> = Vec::new();
        for addr in replicas {
            match stat_one(addr, *chunk_id).await {
                Ok(Some((v, len, crc))) if v == *version => {
                    match sink.get(*chunk_id, std::slice::from_ref(addr)).await {
                        Ok((_, data))
                            if data.len() as u64 == len && crc32fast::hash(&data) == crc =>
                        {
                            if good.is_none() {
                                good = Some(data);
                            }
                        }
                        Ok(_) => rotten.push(addr.clone()), // payload diverges from its header
                        Err(_) => {} // unreachable — the repair loop owns node loss
                    }
                    throttle(len, rate).await;
                }
                Ok(_) => rotten.push(addr.clone()), // missing or stale version
                Err(_) => {}                        // node down — the repair loop owns it
            }
        }
        if rotten.is_empty() {
            continue;
        }
        // Never propagate rot: with no healthy copy the pass fails loudly.
        let data = match &good {
            Some(d) => d.clone(),
            None => sink
                .get(*chunk_id, replicas)
                .await
                .map(|(_, d)| d)
                .map_err(|_| {
                    ClientError::Codec("scrub: every replica failed crc — unrecoverable".into())
                })?,
        };
        for addr in rotten {
            let _ = delete_one(&addr, *chunk_id, *version).await;
            put_one(&addr, *chunk_id, *version, data.clone()).await?;
            stats.repaired += 1;
            throttle(data.len() as u64, rate).await;
        }
    }
    // Orphan sweep: on-disk chunks no metadata root references and no GC
    // entry is already tracking.
    for addr in sink.live_set() {
        let Ok(inventory) = list_chunks(&addr).await else {
            continue; // node down — the repair loop owns it
        };
        for (id, version) in inventory {
            if !reachable.contains_key(&id) && !queued.contains(&id) {
                delete_one(&addr, id, version).await?;
                stats.orphans_deleted += 1;
                throttle(1 << 20, rate).await;
            }
        }
    }
    Ok(stats)
}

/// The periodic scrub worker (§10.3): one pass per interval.
pub async fn scrub_loop(registry: String, state: MetaState, interval_secs: u64) {
    let mut sink = ClusterSink::new(&registry, 1);
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(interval_secs)).await;
        if let Err(e) = scrub_once(&mut sink, &state, REPAIR_RATE).await {
            tracing::warn!("scrub: pass failed: {e}");
        }
    }
}
