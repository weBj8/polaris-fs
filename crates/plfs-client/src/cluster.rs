//! RF replication data plane (design doc §7.2, §7.4): placement (rendezvous
//! over live data nodes), quorum Put with degraded marking, failover Get,
//! and a background repair worker that re-replicates chunks off dead nodes.
//!
//! Quorum semantics (§7.2): RF=2 → both or 1/2 + degraded mark; RF=3 → 2/3.
//! Fewer ⇒ the flush fails loudly (never silently acks one copy as more).

use std::collections::VecDeque;

use plfs_common::data::v1 as pb;
use plfs_common::data::v1::chunk_store_client::ChunkStoreClient;
use plfs_common::registry::v1 as rpb;
use plfs_meta::ChunkRef;
use plfs_registry::client::RegistryClient;
use tonic::transport::Channel;

use crate::core::ClientError;

/// A chunk replica that failed to write (quorum shortfall) or sits on a
/// dead data node — queued for re-replication.
#[derive(Debug, Clone)]
pub struct DegradedChunk {
    /// The chunk.
    pub chunk_id: [u8; 16],
    /// Its version.
    pub version: u64,
    /// The failed/dead replica address (to replace).
    pub dead_addr: String,
}

fn grpc_err(e: impl std::fmt::Display) -> ClientError {
    ClientError::Codec(format!("grpc: {e}"))
}

async fn connect(addr: &str) -> Result<ChunkStoreClient<Channel>, ClientError> {
    ChunkStoreClient::connect(format!("http://{addr}"))
        .await
        .map_err(grpc_err)
}

pub(crate) async fn put_one(
    addr: &str,
    chunk_id: [u8; 16],
    version: u64,
    payload: Vec<u8>,
) -> Result<(), ClientError> {
    let mut client = connect(addr).await?;
    let req = pb::PutRequest {
        chunk_id: chunk_id.to_vec(),
        version,
        crc32c: crc32fast::hash(&payload),
        payload,
        seal: true,
    };
    client.put(req).await.map_err(grpc_err)?;
    Ok(())
}

async fn get_one(addr: &str, chunk_id: [u8; 16]) -> Result<(u64, Vec<u8>), ClientError> {
    let mut client = connect(addr).await?;
    let mut stream = client
        .get(pb::GetRequest {
            chunk_id: chunk_id.to_vec(),
            if_version: 0,
        })
        .await
        .map_err(grpc_err)?
        .into_inner();
    let mut version = 0;
    let mut data = Vec::new();
    while let Some(frame) = stream.message().await.map_err(grpc_err)? {
        version = frame.version;
        data.extend_from_slice(&frame.data);
    }
    Ok((version, data))
}

pub(crate) async fn delete_one(
    addr: &str,
    chunk_id: [u8; 16],
    version: u64,
) -> Result<(), ClientError> {
    let mut client = connect(addr).await?;
    client
        .delete(pb::DeleteRequest {
            chunk_id: chunk_id.to_vec(),
            version,
        })
        .await
        .map_err(grpc_err)?;
    Ok(())
}

/// Single-replica Stat (scrub needs per-replica answers, not failover).
pub(crate) async fn stat_one(
    addr: &str,
    chunk_id: [u8; 16],
) -> Result<Option<(u64, u64, u32)>, ClientError> {
    let mut client = connect(addr).await?;
    match client
        .stat(pb::StatRequest {
            chunk_id: chunk_id.to_vec(),
        })
        .await
    {
        Ok(reply) => {
            let r = reply.into_inner();
            Ok(Some((r.version, r.len, r.crc32c)))
        }
        Err(s) if s.code() == tonic::Code::NotFound => Ok(None),
        Err(s) => Err(grpc_err(s)),
    }
}

/// Arena inventory of one node (scrub's orphan sweep, §10.3).
pub(crate) async fn list_chunks(addr: &str) -> Result<Vec<([u8; 16], u64)>, ClientError> {
    let mut client = connect(addr).await?;
    let mut stream = client
        .list(pb::ListRequest {})
        .await
        .map_err(grpc_err)?
        .into_inner();
    let mut out = Vec::new();
    while let Some(item) = stream.message().await.map_err(grpc_err)? {
        let id: [u8; 16] = item
            .chunk_id
            .try_into()
            .map_err(|_| ClientError::Codec("bad chunk id in List".into()))?;
        out.push((id, item.version));
    }
    Ok(out)
}

/// Repair-worker pacing (§7.4): 30 MB/s per disk by default.
pub(crate) const REPAIR_RATE: u64 = 30 * 1024 * 1024;

pub(crate) async fn throttle(len: u64, rate: u64) {
    if rate > 0 && len > 0 {
        tokio::time::sleep(std::time::Duration::from_secs_f64(len as f64 / rate as f64)).await;
    }
}

/// The replication data plane.
pub struct ClusterSink {
    registry: RegistryClient,
    rf: usize,
    live: Vec<String>,
    degraded: std::sync::Arc<std::sync::Mutex<VecDeque<DegradedChunk>>>,
}

impl ClusterSink {
    /// New cluster sink over data nodes discovered from the registry.
    pub fn new(registry_addr: &str, rf: usize) -> Self {
        Self {
            registry: RegistryClient::new(vec![registry_addr.to_string()]),
            rf,
            live: Vec::new(),
            degraded: std::sync::Arc::new(std::sync::Mutex::new(VecDeque::new())),
        }
    }

    /// Refresh the live data-node set (addresses of nodes with a live
    /// liveness lease).
    pub async fn refresh_live(&mut self) -> Result<(), ClientError> {
        let nodes = self
            .registry
            .list_nodes(rpb::NodeKind::Data)
            .await
            .map_err(grpc_err)?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs() as i64);
        self.live = nodes
            .into_iter()
            .filter(|n| n.lease_until > now)
            .map(|n| n.addr)
            .collect();
        if self.live.is_empty() {
            return Err(ClientError::Codec("no live data nodes".into()));
        }
        Ok(())
    }

    /// Rendezvous placement: argmax(xxhash(chunk_id ‖ node)) over live
    /// nodes, RF distinct addresses (design doc §7.2, §14 failure-domain:
    /// distinct data nodes are the failure domain here).
    pub fn place(&mut self, chunk_id: &[u8; 16]) -> Vec<String> {
        let mut scored: Vec<(u64, &String)> = self
            .live
            .iter()
            .map(|addr| (score(chunk_id, addr), addr))
            .collect();
        scored.sort_by_key(|(s, _)| std::cmp::Reverse(*s));
        scored
            .into_iter()
            .take(self.rf.min(self.live.len()))
            .map(|(_, a)| a.clone())
            .collect()
    }

    /// Quorum Put (§7.2): fire at all replicas concurrently; RF=2 → both or
    /// 1/2 + degraded; RF=3 → 2/3. Failed replicas are queued for repair.
    pub async fn put(
        &mut self,
        chunk_id: [u8; 16],
        version: u64,
        payload: &[u8],
        replicas: &[String],
    ) -> Result<(), ClientError> {
        let mut failures = Vec::new();
        let futures: Vec<_> = replicas
            .iter()
            .map(|addr| put_one(addr, chunk_id, version, payload.to_vec()))
            .collect();
        let results = futures::future::join_all(futures).await;
        let mut ok = 0usize;
        for (addr, res) in replicas.iter().zip(results) {
            match res {
                Ok(()) => ok += 1,
                Err(_) => failures.push(addr.clone()),
            }
        }
        for addr in failures {
            self.degraded
                .lock()
                .expect("degraded lock")
                .push_back(DegradedChunk {
                    chunk_id,
                    version,
                    dead_addr: addr,
                });
        }
        let quorum = match replicas.len() {
            1 => 1,
            2 => 1, // RF=2: 2/2 or 1/2 + degraded mark (§7.2)
            _ => 2, // RF=3: 2/3
        };
        if ok >= quorum.min(replicas.len()) {
            Ok(())
        } else {
            Err(ClientError::Codec(format!(
                "quorum shortfall: {ok}/{} replicas wrote",
                replicas.len()
            )))
        }
    }

    /// Failover Get (§7.4): try replicas in order; first healthy answer wins.
    pub async fn get(
        &mut self,
        chunk_id: [u8; 16],
        replicas: &[String],
    ) -> Result<(u64, Vec<u8>), ClientError> {
        let mut last = ClientError::Codec("no replicas".into());
        for addr in replicas {
            match get_one(addr, chunk_id).await {
                Ok(res) => return Ok(res),
                Err(e) => last = e,
            }
        }
        Err(last)
    }

    /// Delete from all replicas (exact-version, idempotent; tolerant of
    /// missing replicas — a dead node's copy is re-deleted on repair).
    pub async fn delete(
        &mut self,
        chunk_id: [u8; 16],
        version: u64,
        replicas: &[String],
    ) -> Result<(), ClientError> {
        let futures: Vec<_> = replicas
            .iter()
            .map(|addr| delete_one(addr, chunk_id, version))
            .collect();
        let _ = futures::future::join_all(futures).await;
        Ok(())
    }

    /// Stat from the first answering replica.
    pub async fn stat(
        &mut self,
        chunk_id: [u8; 16],
        replicas: &[String],
    ) -> Result<Option<(u64, u64, u32)>, ClientError> {
        for addr in replicas {
            let mut client = connect(addr).await?;
            match client
                .stat(pb::StatRequest {
                    chunk_id: chunk_id.to_vec(),
                })
                .await
            {
                Ok(reply) => {
                    let r = reply.into_inner();
                    return Ok(Some((r.version, r.len, r.crc32c)));
                }
                Err(s) if s.code() == tonic::Code::NotFound => return Ok(None),
                Err(_) => continue,
            }
        }
        Ok(None)
    }

    /// Aggregated capacity stats over live data nodes (statfs backend).
    pub async fn cluster_stats(&self) -> Result<(u64, u64), ClientError> {
        let stats = self.registry.stats().await.map_err(grpc_err)?;
        Ok((stats.capacity_bytes, stats.used_bytes))
    }

    /// The current live data-node set (after the last `refresh_live`).
    pub fn live_set(&self) -> Vec<String> {
        self.live.clone()
    }

    /// The shared degraded queue (the repair loop drains it).
    pub fn degraded_handle(&self) -> std::sync::Arc<std::sync::Mutex<VecDeque<DegradedChunk>>> {
        self.degraded.clone()
    }

    /// Chunks whose replica set lost a member because `dead_addr` went away:
    /// degrade every recorded chunk replica on it (node-down source, §7.4).
    pub fn degrade_node(&mut self, dead_addr: &str, chunks: &[ChunkRef]) {
        for chunk in chunks {
            self.degraded
                .lock()
                .expect("degraded lock")
                .push_back(DegradedChunk {
                    chunk_id: chunk.chunk_id,
                    version: chunk.version,
                    dead_addr: dead_addr.to_string(),
                });
        }
    }

    /// Re-replicate one degraded chunk: read a healthy replica, put to a
    /// fresh live node, return the new replica set (dead addr swapped) and
    /// the payload length (repair pacing).
    pub async fn repair_one(
        &mut self,
        entry: &DegradedChunk,
        replicas: &[String],
    ) -> Result<(Vec<String>, u64), ClientError> {
        let healthy: Vec<String> = replicas
            .iter()
            .filter(|a| **a != entry.dead_addr)
            .cloned()
            .collect();
        let (_, payload) = self.get(entry.chunk_id, &healthy).await?;
        let len = payload.len() as u64;
        let fresh = self
            .live
            .iter()
            .find(|a| !replicas.contains(a))
            .cloned()
            .ok_or_else(|| ClientError::Codec("no fresh node for repair".into()))?;
        put_one(&fresh, entry.chunk_id, entry.version, payload).await?;
        Ok((
            healthy.into_iter().chain(std::iter::once(fresh)).collect(),
            len,
        ))
    }
}

fn score(chunk_id: &[u8; 16], addr: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2235;
    for b in chunk_id.iter().chain(addr.as_bytes()) {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// Background repair worker (design doc §7.4): polls the live data-node
/// set, degrades chunks on nodes that dropped out (via the volume's
/// `chunks_with_replica`), and re-replicates the degraded queue onto fresh
/// live nodes, re-pointing each chunk's replica set through the raft.
pub async fn repair_loop(
    queue: std::sync::Arc<std::sync::Mutex<VecDeque<DegradedChunk>>>,
    registry_addr: String,
    state: plfs_meta::MetaState,
    raft: openraft::Raft<plfs_meta::raft::MetaRaftConfig>,
    counter: std::sync::Arc<std::sync::atomic::AtomicU64>,
) {
    // The repair sink shares the client's degraded queue: node-down degrades
    // land in the same queue the drain loop below pops from.
    let mut sink = ClusterSink::new(&registry_addr, 1);
    sink.degraded = queue.clone();
    let mut prev_live: Vec<String> = Vec::new();
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        if let Err(e) = sink.refresh_live().await {
            tracing::warn!("repair: refresh_live failed: {e}");
            continue;
        }
        let live = sink.live_set();
        for dropped in prev_live.iter().filter(|a| !live.contains(a)) {
            match state.chunks_with_replica(dropped) {
                Ok(chunks) => {
                    tracing::info!(%dropped, count = chunks.len(), "repair: node down, degrading chunks");
                    sink.degrade_node(dropped, &chunks);
                }
                Err(e) => tracing::warn!("repair: chunks_with_replica failed: {e}"),
            }
        }
        prev_live = live;
        // Drain the degraded queue.
        loop {
            let entry = queue.lock().expect("degraded lock").pop_front();
            let Some(entry) = entry else { break };
            let replicas: Vec<String> = state
                .chunks_with_replica(&entry.dead_addr)
                .unwrap_or_default()
                .into_iter()
                .find(|c| c.chunk_id == entry.chunk_id)
                .map(|c| c.replicas)
                .unwrap_or_default();
            match sink.repair_one(&entry, &replicas).await {
                Ok((new_replicas, len)) => {
                    let op = plfs_meta::MetaOp::RepairChunk {
                        chunk_id: entry.chunk_id,
                        version: entry.version,
                        new_replicas,
                    };
                    match raft.client_write(op).await {
                        Ok(_) => {
                            tracing::info!("repair: re-replicated chunk onto fresh node");
                            counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            throttle(len, REPAIR_RATE).await;
                        }
                        Err(e) => {
                            tracing::warn!("repair: RepairChunk commit failed: {e}");
                            queue.lock().expect("degraded lock").push_back(entry);
                            break;
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("repair: repair_one failed: {e}");
                    queue.lock().expect("degraded lock").push_back(entry);
                    break;
                }
            }
        }
    }
}
