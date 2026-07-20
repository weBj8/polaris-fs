//! Retention scheduler (design doc §9): interval snapshots with keep-N
//! pruning, driven by `<volume>/snapshots.toml`.

use plfs_meta::raft::MetaRaftConfig;
use plfs_meta::{MetaOp, MetaState};

/// `snapshots.toml` schema.
#[derive(Debug, serde::Deserialize)]
pub struct RetentionConfig {
    /// Seconds between scheduled snapshots.
    pub interval_secs: u64,
    /// Keep the newest N scheduled snapshots; older ones are deleted.
    pub keep: usize,
    /// Scheduled snapshot name prefix (only these are pruned).
    #[serde(default = "default_prefix")]
    pub name_prefix: String,
}

fn default_prefix() -> String {
    "auto".into()
}

/// Create a snapshot every `interval_secs`, then prune the prefix group to
/// the newest `keep` (crash-consistent snapshots: whatever the metadata has
/// committed, per §9's crash-consistent contract).
pub async fn retention_loop(
    cfg: RetentionConfig,
    raft: openraft::Raft<MetaRaftConfig>,
    state: MetaState,
) {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(cfg.interval_secs)).await;
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let name = format!("{}-{ts}", cfg.name_prefix);
        if let Err(e) = raft.client_write(MetaOp::CreateSnap { name }).await {
            tracing::warn!("retention: CreateSnap failed: {e}");
            continue;
        }
        let mut auto = match state.list_snaps() {
            Ok(snaps) => snaps
                .into_iter()
                .filter(|s| s.name.starts_with(&cfg.name_prefix))
                .collect::<Vec<_>>(),
            Err(e) => {
                tracing::warn!("retention: list_snaps failed: {e}");
                continue;
            }
        };
        auto.sort_by_key(|s| s.id);
        let stale = auto.len().saturating_sub(cfg.keep);
        for snap in auto.into_iter().take(stale) {
            if let Err(e) = crate::core::delete_snap_with_reclaim(&state, &raft, snap.id).await {
                tracing::warn!("retention: delete {} failed: {e}", snap.id);
            }
        }
    }
}
