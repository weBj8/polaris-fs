//! Persistent SSD read cache (design doc §7.1): chunk-granular, version-
//! keyed, LRU-evicted at a capacity watermark. The cache is a **pure
//! derivative** — any entry can vanish at any time; correctness never
//! depends on it.
//!
//! - Entries live as files in `<vol>/cache/` (`<hex(chunk_id)>-<version>`),
//!   the catalog (chunk_id → version/len/last_used) in `catalog.redb`, so a
//!   warm start serves straight from SSD (~100 µs) with no warm-up pass.
//! - Sealed chunks are immutable, so a `(chunk_id, version)` hit can never
//!   be stale in content; the only staleness is a chunk the data node has
//!   since GC-deleted. Validation is one cheap `Stat` (the design doc's
//!   304), suppressed for 5 s per entry by an advisory lease.
//! - Eviction deletes cache files; the RAM `lru` is the hot index.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use lru::LruCache;
use redb::{Database, ReadableTable, TableDefinition};

/// Advisory validation lease (design doc §7.1: suppress the Stat for hot
/// chunks).
const LEASE: Duration = Duration::from_secs(5);

const CATALOG: TableDefinition<&[u8], &[u8]> = TableDefinition::new("cache_catalog");

/// Cache capacity watermark (evict above 85% of `capacity_bytes`).
#[derive(Debug, Clone, Copy)]
pub struct CacheConfig {
    /// Hard capacity in bytes; eviction starts at 85%.
    pub capacity_bytes: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            capacity_bytes: 8 << 30,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Entry {
    version: u64,
    len: u64,
    last_used: u64,
}

fn enc_entry(e: &Entry) -> Vec<u8> {
    let mut v = Vec::with_capacity(24);
    v.extend_from_slice(&e.version.to_le_bytes());
    v.extend_from_slice(&e.len.to_le_bytes());
    v.extend_from_slice(&e.last_used.to_le_bytes());
    v
}

fn dec_entry(b: &[u8]) -> Option<Entry> {
    if b.len() != 24 {
        return None;
    }
    Some(Entry {
        version: u64::from_le_bytes(b[0..8].try_into().ok()?),
        len: u64::from_le_bytes(b[8..16].try_into().ok()?),
        last_used: u64::from_le_bytes(b[16..24].try_into().ok()?),
    })
}

fn entry_path(dir: &Path, chunk_id: &[u8; 16], version: u64) -> PathBuf {
    dir.join(format!("{}-{version}.bin", hex(chunk_id)))
}

fn hex(b: &[u8; 16]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Cache statistics.
#[derive(Debug, Default, Clone, Copy)]
pub struct CacheStats {
    /// Served from the SSD cache (validated or leased).
    pub hits: u64,
    /// Fetched from the data plane.
    pub misses: u64,
    /// Currently cached payload bytes.
    pub used_bytes: u64,
}

struct Inner {
    db: Database,
    lru: LruCache<[u8; 16], Entry>,
    lease: HashMap<[u8; 16], Instant>,
    used: u64,
}

/// The persistent read cache.
pub struct ReadCache {
    dir: PathBuf,
    inner: Mutex<Inner>,
    capacity: u64,
    hits: AtomicU64,
    misses: AtomicU64,
}

impl ReadCache {
    /// Open (or create) the cache at `<vol>/cache`. The RAM index is rebuilt
    /// from the catalog; files missing on disk are dropped from the catalog.
    pub fn open(vol_dir: &Path, cfg: CacheConfig) -> Result<Self, String> {
        let dir = vol_dir.join("cache");
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let db = Database::create(dir.join("catalog.redb")).map_err(|e| e.to_string())?;

        let mut lru = LruCache::unbounded();
        let mut used = 0u64;
        {
            let wtxn = db.begin_write().map_err(|e| e.to_string())?;
            wtxn.open_table(CATALOG).map_err(|e| e.to_string())?;
            wtxn.commit().map_err(|e| e.to_string())?;
            let txn = db.begin_read().map_err(|e| e.to_string())?;
            let table = txn.open_table(CATALOG).map_err(|e| e.to_string())?;
            let mut stale = Vec::new();
            for row in table.iter().map_err(|e| e.to_string())? {
                let (k, v) = row.map_err(|e| e.to_string())?;
                let key: [u8; 16] = k.value().try_into().map_err(|_| "bad key".to_string())?;
                let Some(entry) = dec_entry(v.value()) else {
                    stale.push(key);
                    continue;
                };
                if entry_path(&dir, &key, entry.version).exists() {
                    used += entry.len;
                    lru.put(key, entry);
                } else {
                    stale.push(key);
                }
            }
            drop(table);
            drop(txn);
            if !stale.is_empty() {
                let wtxn = db.begin_write().map_err(|e| e.to_string())?;
                let mut t = wtxn.open_table(CATALOG).map_err(|e| e.to_string())?;
                for key in stale {
                    t.remove(key.as_slice()).map_err(|e| e.to_string())?;
                }
                drop(t);
                wtxn.commit().map_err(|e| e.to_string())?;
            }
        }
        Ok(Self {
            dir,
            inner: Mutex::new(Inner {
                db,
                lru,
                lease: HashMap::new(),
                used,
            }),
            capacity: cfg.capacity_bytes,
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        })
    }

    /// Serve a cached payload: `Some(bytes)` on a hit, `None` when the entry
    /// is absent. Validation (`validate`) decides whether the hit counts.
    pub fn probe(&self, chunk_id: &[u8; 16], version: u64) -> Option<Vec<u8>> {
        let path = {
            let mut inner = self.inner.lock().ok()?;
            let entry = inner.lru.get(chunk_id)?;
            if entry.version != version {
                return None;
            }
            entry_path(&self.dir, chunk_id, version)
        };
        let bytes = std::fs::read(&path).ok()?;
        // last_used updates stay in memory: persisting them costs a write
        // transaction per read, and exact LRU order across restarts is not
        // worth it (the catalog is a pure derivative — insert-time order is
        // a fine approximation after reopen).
        let mut inner = self.inner.lock().ok()?;
        if let Some(entry) = inner.lru.get_mut(chunk_id) {
            entry.last_used = unix_now();
        }
        Some(bytes)
    }

    /// Insert a fetched payload and evict down to the watermark.
    pub fn insert(&self, chunk_id: &[u8; 16], version: u64, payload: &[u8]) {
        let tmp = self.dir.join(format!(".tmp-{}", std::process::id()));
        if std::fs::write(&tmp, payload).is_err() {
            return;
        }
        let path = entry_path(&self.dir, chunk_id, version);
        if std::fs::rename(&tmp, &path).is_err() {
            let _ = std::fs::remove_file(&tmp);
            return;
        }
        let mut inner = match self.inner.lock() {
            Ok(i) => i,
            Err(_) => return,
        };
        let entry = Entry {
            version,
            len: payload.len() as u64,
            last_used: unix_now(),
        };
        inner.lru.put(*chunk_id, entry);
        inner.used += payload.len() as u64;
        Self::catalog_put(&inner.db, chunk_id, &entry);
        self.evict_locked(&mut inner);
    }

    /// Drop an entry (e.g. the data node no longer has the chunk).
    pub fn evict(&self, chunk_id: &[u8; 16]) {
        if let Ok(mut inner) = self.inner.lock() {
            self.evict_one(&mut inner, chunk_id);
        }
    }

    /// True when the entry was validated within the advisory lease.
    pub fn lease_valid(&self, chunk_id: &[u8; 16]) -> bool {
        self.inner
            .lock()
            .ok()
            .and_then(|inner| inner.lease.get(chunk_id).copied())
            .is_some_and(|t| t.elapsed() < LEASE)
    }

    /// Mark the entry validated now (starts/refreshes the lease).
    pub fn validate(&self, chunk_id: &[u8; 16]) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.lease.insert(*chunk_id, Instant::now());
        }
    }

    /// Record a served hit / a data-plane fetch.
    pub fn record(&self, hit: bool) {
        if hit {
            self.hits.fetch_add(1, Ordering::Relaxed);
            metrics::counter!("plfs_cache_hits_total").increment(1);
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            metrics::counter!("plfs_cache_misses_total").increment(1);
        }
    }

    /// Current statistics.
    pub fn stats(&self) -> CacheStats {
        let used = self.inner.lock().map(|i| i.used).unwrap_or(0);
        CacheStats {
            hits: self.hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
            used_bytes: used,
        }
    }

    fn catalog_put(db: &Database, chunk_id: &[u8; 16], entry: &Entry) {
        let Ok(mut wtxn) = db.begin_write() else {
            return;
        };
        // The catalog is a pure derivative — no fsync on its commits.
        wtxn.set_durability(redb::Durability::None);
        let ok = wtxn.open_table(CATALOG).map(|mut t| {
            let _ = t.insert(chunk_id.as_slice(), enc_entry(entry).as_slice());
        });
        if ok.is_ok() {
            let _ = wtxn.commit();
        }
    }

    fn evict_locked(&self, inner: &mut Inner) {
        let watermark = self.capacity * 85 / 100;
        while inner.used > watermark {
            let Some((key, entry)) = inner.lru.pop_lru() else {
                break;
            };
            self.evict_entry(inner, &key, entry);
        }
    }

    fn evict_one(&self, inner: &mut Inner, chunk_id: &[u8; 16]) {
        if let Some(entry) = inner.lru.pop(chunk_id) {
            self.evict_entry(inner, chunk_id, entry);
        }
        inner.lease.remove(chunk_id);
    }

    fn evict_entry(&self, inner: &mut Inner, chunk_id: &[u8; 16], entry: Entry) {
        let _ = std::fs::remove_file(entry_path(&self.dir, chunk_id, entry.version));
        inner.used = inner.used.saturating_sub(entry.len);
        if let Ok(mut wtxn) = inner.db.begin_write() {
            wtxn.set_durability(redb::Durability::None);
            let ok = wtxn.open_table(CATALOG).map(|mut t| {
                let _ = t.remove(chunk_id.as_slice());
            });
            if ok.is_ok() {
                let _ = wtxn.commit();
            }
        }
        inner.lease.remove(chunk_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmpdir(name: &str) -> PathBuf {
        let dir = std::env::var("CARGO_TARGET_TMPDIR").unwrap_or_else(|_| "target/tmp".into());
        let dir = PathBuf::from(dir).join(format!("plfs-cache-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn id(n: u8) -> [u8; 16] {
        [n; 16]
    }

    #[test]
    fn insert_probe_persist_across_reopen() {
        let dir = tmpdir("persist");
        {
            let cache = ReadCache::open(
                &dir,
                CacheConfig {
                    capacity_bytes: 1 << 20,
                },
            )
            .unwrap();
            cache.insert(&id(1), 7, b"payload-1");
            assert_eq!(cache.probe(&id(1), 7).as_deref(), Some(&b"payload-1"[..]));
            assert_eq!(cache.probe(&id(1), 8), None, "version-keyed");
        }
        let cache = ReadCache::open(
            &dir,
            CacheConfig {
                capacity_bytes: 1 << 20,
            },
        )
        .unwrap();
        assert_eq!(
            cache.probe(&id(1), 7).as_deref(),
            Some(&b"payload-1"[..]),
            "warm start serves from SSD"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn lru_eviction_at_watermark() {
        let dir = tmpdir("evict");
        let cache = ReadCache::open(
            &dir,
            CacheConfig {
                capacity_bytes: 1000,
            },
        )
        .unwrap();
        for n in 0..10u8 {
            cache.insert(&id(n), 1, &[n; 100]);
        }
        let stats = cache.stats();
        assert!(
            stats.used_bytes <= 850,
            "evicted to watermark: {}",
            stats.used_bytes
        );
        assert_eq!(cache.probe(&id(0), 1), None, "oldest evicted");
        assert_eq!(cache.probe(&id(9), 1).as_deref(), Some(&[9u8; 100][..]));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn metrics_register_with_prometheus_recorder() {
        let handle = metrics_exporter_prometheus::PrometheusBuilder::new()
            .install_recorder()
            .expect("recorder");
        let dir = tmpdir("metrics");
        let cache = ReadCache::open(
            &dir,
            CacheConfig {
                capacity_bytes: 1 << 20,
            },
        )
        .unwrap();
        cache.record(true);
        cache.record(false);
        let body = handle.render();
        assert!(body.contains("plfs_cache_hits_total"), "body: {body}");
        assert!(body.contains("plfs_cache_misses_total"), "body: {body}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn lease_lifecycle() {
        let dir = tmpdir("lease");
        let cache = ReadCache::open(
            &dir,
            CacheConfig {
                capacity_bytes: 1 << 20,
            },
        )
        .unwrap();
        assert!(!cache.lease_valid(&id(1)));
        cache.validate(&id(1));
        assert!(cache.lease_valid(&id(1)));
        cache.evict(&id(1));
        assert!(!cache.lease_valid(&id(1)));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
