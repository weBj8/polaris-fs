//! The MDS data plane: where file payload extents physically live.
//!
//! [`DataPlane`] hides the two backends behind one method surface mirroring
//! the store calls the MDS makes:
//!
//! - `Local(ExtentStore)`: the single-node v0 layout — extents live in one
//!   local extent device;
//! - `Remote(RemoteStore)`: the P12.5 cluster layout — extents live on
//!   chunkservers (wire protocol v3), placed by rendezvous hashing
//!   (P10) and optionally chain-replicated two ways (P9).
//!
//! [`RemoteStore`] is a synchronous facade over an internal tokio runtime:
//! the MDS public API stays blocking and `Mds` stays `!Send` (the FUSE
//! worker-thread model is unchanged). Every network call is wrapped in
//! `tokio::time::timeout`, so a partitioned-but-not-killed chunkserver
//! cannot hang the single MDS worker thread: reads fail over to the
//! replica, writes and syncs fail loudly (`EIO` upward).

use std::net::SocketAddr;
use std::sync::Mutex as StdMutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use futures_util::future::join_all;
use porfs_cluster::{FailureDomain, Member, Membership, StripeMap};
use porfs_format::EXTENT_DATA_MAX;
use porfs_rpc::{ChunkClient, ErrorCode, Request, Response, RpcError};
use porfs_store::{ExtentStore, StoreError};

use crate::error::{MdsError, Result};
use crate::keys::{ExtentRef, MemberSpec};

/// Per-attempt read timeout (mirrors `PRIMARY_READ_TIMEOUT` in
/// porfs-cluster): a primary that does not answer within this window is
/// treated as failed and the read fails over to the replica.
const READ_TIMEOUT: Duration = Duration::from_secs(2);
/// Write timeout: chain writes are allowed to be slow (a fsync-less append
/// still crosses two servers); a timeout fails the op (`EIO`), it never
/// hangs the worker thread.
const WRITE_TIMEOUT: Duration = Duration::from_secs(30);
/// Sync timeout: fsync is a durability barrier and may legitimately take
/// real time on a loaded server.
const SYNC_TIMEOUT: Duration = Duration::from_secs(30);
/// Timeout bounding the forced first connect per server at format/open
/// (the client's own connect backoff is unbounded by design).
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// Pipelining window for batched wire reads (the server's MAX_INFLIGHT).
const PIPELINE_WINDOW: usize = 32;
const BREAKER_MIN: Duration = Duration::from_secs(2);
const BREAKER_MAX: Duration = Duration::from_secs(60);

/// Network timeouts of the cluster data plane. Production code uses the
/// [`Default`] (the constants above); loopback tests construct shorter
/// values to keep failure-path tests fast.
#[derive(Debug, Clone, Copy)]
pub struct ClusterTimeouts {
    /// Per-attempt read timeout (failover trigger).
    pub read: Duration,
    /// Per-write timeout.
    pub write: Duration,
    /// Per-server sync (fsync broadcast) timeout.
    pub sync: Duration,
    /// Forced initial-connect timeout per server at format/open.
    pub connect: Duration,
}

impl Default for ClusterTimeouts {
    fn default() -> Self {
        Self {
            read: READ_TIMEOUT,
            write: WRITE_TIMEOUT,
            sync: SYNC_TIMEOUT,
            connect: CONNECT_TIMEOUT,
        }
    }
}

/// One subrange read against one extent: `len` bytes at `off` within the
/// extent `eref` refers to.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ReadReq {
    pub(crate) eref: ExtentRef,
    pub(crate) off: u32,
    pub(crate) len: u32,
}

/// Liveness of one extent copy, as probed at mount time / self-check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Probe {
    /// The copy read back fine.
    Live,
    /// A REACHABLE server answered `NotFound` (unknown or tombstoned).
    NotFound,
    /// The server did not answer, errored, or answered something else —
    /// the copy's state is unknown; reconciling against it is forbidden.
    Unreachable,
}

/// The MDS data plane: local extent device or remote chunkserver cluster.
pub(crate) enum DataPlane {
    /// Single-node layout: extents in one local extent device (boxed: the
    /// store is half a kilobyte of io_uring state, so the enum stays small
    /// for the remote variant's sake).
    Local(Box<ExtentStore>),
    /// Cluster layout: extents on chunkservers (wire v3); boxed as well,
    /// so the enum itself stays pointer-sized.
    Remote(Box<RemoteStore>),
}

impl DataPlane {
    /// Append one extent per `(ino, off, bytes)` item, in order.
    pub(crate) fn append_batch(&mut self, items: &[(u64, u64, &[u8])]) -> Result<Vec<ExtentRef>> {
        match self {
            DataPlane::Local(store) => Ok(store
                .append_batch(items)?
                .into_iter()
                .map(ExtentRef::Local)
                .collect()),
            DataPlane::Remote(remote) => remote.append_batch(items),
        }
    }

    /// Read `reqs` in order: per request, `min(len, data_len - off)` bytes
    /// of the extent's payload. Local reads go through the store and slice
    /// in memory; remote reads issue `ReadExtentRange` on the wire with
    /// replica failover.
    pub(crate) fn read_ranges(&mut self, reqs: &[ReadReq]) -> Result<Vec<Vec<u8>>> {
        match self {
            DataPlane::Local(store) => {
                let ids: Vec<u64> = reqs
                    .iter()
                    .map(|req| match req.eref {
                        ExtentRef::Local(id) => Ok(id),
                        ExtentRef::Remote { .. } => Err(MdsError::Corrupt(
                            "cluster extent ref in a local-mode database".to_string(),
                        )),
                    })
                    .collect::<Result<_>>()?;
                let bufs = store.read_batch(&ids)?;
                reqs.iter()
                    .zip(bufs)
                    .map(|(req, data)| slice_range(data, req.off, req.len))
                    .collect()
            }
            DataPlane::Remote(remote) => remote.read_ranges(reqs),
        }
    }

    /// Durability barrier: confirm every extent appended so far. Remote
    /// mode broadcasts one timed `sync` per server and requires ALL to
    /// succeed (any failure is `EIO` — nothing is silently acknowledged).
    pub(crate) fn sync(&mut self) -> Result<()> {
        match self {
            DataPlane::Local(store) => Ok(store.sync()?),
            DataPlane::Remote(remote) => remote.sync(),
        }
    }

    /// Tombstone every copy of each ref after its map rows committed out.
    /// Tolerates "already gone" (a crash may have salvaged the extent
    /// first); propagates every other failure.
    pub(crate) fn discard_tolerant(&mut self, refs: &[ExtentRef]) -> Result<()> {
        match self {
            DataPlane::Local(store) => {
                for eref in refs {
                    let ExtentRef::Local(id) = *eref else {
                        return Err(MdsError::Corrupt(
                            "cluster extent ref in a local-mode database".to_string(),
                        ));
                    };
                    match store.discard(id) {
                        Ok(())
                        | Err(StoreError::UnknownExtent(_))
                        | Err(StoreError::Tombstoned(_)) => {}
                        Err(err) => return Err(MdsError::Store(err)),
                    }
                }
                Ok(())
            }
            DataPlane::Remote(remote) => remote.discard_tolerant(refs),
        }
    }

    /// Probe every copy of every ref, flat in ref order (a replicated ref
    /// yields two results: primary then replica).
    pub(crate) fn probe(&mut self, refs: &[ExtentRef]) -> Result<Vec<Probe>> {
        match self {
            DataPlane::Local(store) => refs
                .iter()
                .map(|eref| match *eref {
                    ExtentRef::Local(id) => match store.read(id) {
                        Ok(_) => Ok(Probe::Live),
                        Err(StoreError::UnknownExtent(_)) | Err(StoreError::Tombstoned(_)) => {
                            Ok(Probe::NotFound)
                        }
                        Err(err) => Err(MdsError::Store(err)),
                    },
                    ExtentRef::Remote { .. } => Err(MdsError::Corrupt(
                        "cluster extent ref in a local-mode database".to_string(),
                    )),
                })
                .collect(),
            DataPlane::Remote(remote) => remote.probe(refs),
        }
    }

    /// Address of cluster member `server` (for error messages; `None` in
    /// local mode).
    pub(crate) fn member_addr(&self, server: u32) -> Option<SocketAddr> {
        match self {
            DataPlane::Local(_) => None,
            DataPlane::Remote(remote) => remote
                .members
                .get(server as usize)
                .map(|member| member.addr),
        }
    }

    /// Total device bytes (remote: summed over all servers).
    pub(crate) fn device_size(&self) -> Result<u64> {
        match self {
            DataPlane::Local(store) => Ok(store.device_size()),
            DataPlane::Remote(remote) => Ok(remote.aggregate_stats()?.device_size),
        }
    }

    /// Next append offset (remote: summed over all servers).
    pub(crate) fn tail(&self) -> Result<u64> {
        match self {
            DataPlane::Local(store) => Ok(store.tail()),
            DataPlane::Remote(remote) => Ok(remote.aggregate_stats()?.tail),
        }
    }

    /// Live payload bytes (remote: summed over all servers).
    pub(crate) fn live_bytes(&self) -> Result<u64> {
        match self {
            DataPlane::Local(store) => Ok(store.live_bytes()),
            DataPlane::Remote(remote) => Ok(remote.aggregate_stats()?.live_bytes),
        }
    }

    /// Live extent count (remote: summed over all servers).
    pub(crate) fn extent_count(&self) -> Result<u64> {
        match self {
            DataPlane::Local(store) => Ok(store.extent_count()),
            DataPlane::Remote(remote) => Ok(remote.aggregate_stats()?.extent_count),
        }
    }
}

/// Slice `data` to `[off, off + len)` with the wire protocol's
/// `ReadExtentRange` semantics; beyond-end offsets are metadata corruption
/// (the recorded row length can never exceed the stored payload).
fn slice_range(data: Vec<u8>, off: u32, len: u32) -> Result<Vec<u8>> {
    let data_len = data.len() as u64;
    let off = u64::from(off);
    if off > data_len {
        return Err(MdsError::Corrupt(format!(
            "extent payload is {data_len} bytes but a row references offset {off}"
        )));
    }
    let start = off as usize;
    let end = (off + u64::from(len)).min(data_len) as usize;
    Ok(data[start..end].to_vec())
}

/// One copy address of a batched wire op: request index in the caller's
/// order plus the server-local extent id.
#[derive(Debug, Clone, Copy)]
struct CopyReq {
    idx: usize,
    extent: u64,
    off: u32,
    len: u32,
}

/// Aggregate of the per-server `Stats` counters (cluster statfs).
#[derive(Debug, Default)]
struct AggregateStats {
    device_size: u64,
    tail: u64,
    live_bytes: u64,
    extent_count: u64,
}

/// Synchronous facade over the chunkserver cluster: an internal tokio
/// runtime, one UUID-pinned client per member, rendezvous placement, and
/// per-call timeouts. All methods block the calling (MDS worker) thread
/// via `block_on` — the single-writer model of `Mds` is preserved.
pub(crate) struct RemoteStore {
    rt: tokio::runtime::Runtime,
    clients: Vec<ChunkClient>,
    members: Vec<MemberSpec>,
    map: StripeMap,
    replicas: usize,
    write_seq: AtomicU64,
    mount_seed: u64,
    timeouts: ClusterTimeouts,
    breaker: StdMutex<Vec<BreakerSlot>>,
}

/// Per-server circuit-breaker slot: a failed server (transport error or
/// timeout — never a protocol answer, which proves reachability) is
/// skipped until `down_until`; reads fail straight over to the replica.
/// Paths that cannot skip (writes, sync, statfs) give a marked server a
/// single read-timeout probe instead of their full window — the probe
/// doubles as the recovery check, so a restarted server rejoins without
/// waiting out the window. An expired window allows one probe attempt:
/// success clears the slot, failure re-arms it with a doubled window.
#[derive(Debug, Default, Clone, Copy)]
struct BreakerSlot {
    down_until: Option<Instant>,
    strikes: u32,
}

impl RemoteStore {
    /// Connect to every member with UUID-pinned clients, verifying reach-
    /// ability and identity up front: per server a timed `stats` forces the
    /// connect + Hello, and a pinned UUID mismatch surfaces here as an
    /// error. Fails unless EVERY member is reachable and correctly
    /// identified — a reconcile run against the wrong server would
    /// probe-wipe the namespace.
    pub(crate) fn connect(
        members: &[MemberSpec],
        replicas: usize,
        timeouts: ClusterTimeouts,
    ) -> Result<Self> {
        if members.is_empty() {
            return Err(MdsError::InvalidOp(
                "cluster membership must not be empty".to_string(),
            ));
        }
        if !(1..=2).contains(&replicas) {
            return Err(MdsError::InvalidOp(format!(
                "replicas must be 1 or 2, got {replicas}"
            )));
        }
        let mut racks: Vec<&str> = members.iter().map(|m| m.rack.as_str()).collect();
        racks.sort_unstable();
        racks.dedup();
        if replicas == 2 && racks.len() < 2 {
            return Err(MdsError::InvalidOp(format!(
                "replicas=2 requires members in at least two racks; found {} unique rack(s)",
                racks.len()
            )));
        }
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .map_err(|err| MdsError::Cluster(format!("tokio runtime: {err}")))?;
        let membership = Membership::with_topology(
            members
                .iter()
                .map(|m| Member {
                    addr: m.addr,
                    failure_domain: FailureDomain::new(hash_tag(&m.rack), hash_tag(&m.chassis)),
                })
                .collect(),
        );
        let map = StripeMap::from_membership(&membership);
        let clients: Vec<ChunkClient> = members
            .iter()
            .map(|m| ChunkClient::new_pinned(m.addr, m.uuid))
            .collect();
        rt.block_on(async {
            for (client, member) in clients.iter().zip(members) {
                match tokio::time::timeout(timeouts.connect, client.stats()).await {
                    Ok(Ok(_)) => {}
                    Ok(Err(err)) => {
                        return Err(MdsError::Cluster(format!(
                            "chunkserver {} failed identity verification: {err}",
                            member.addr
                        )));
                    }
                    Err(_) => {
                        return Err(MdsError::Cluster(format!(
                            "chunkserver {} unreachable within {:?}",
                            member.addr, timeouts.connect
                        )));
                    }
                }
            }
            Ok(())
        })?;
        Ok(Self {
            rt,
            clients: clients.clone(),
            members: members.to_vec(),
            map,
            replicas,
            write_seq: AtomicU64::new(0),
            mount_seed: mount_seed(),
            timeouts,
            breaker: StdMutex::new(vec![BreakerSlot::default(); clients.len()]),
        })
    }

    /// True while `server`'s breaker window is open (skip it: reads fail
    /// straight over, writes/syncs fail fast). An expired window returns
    /// false, letting exactly one probe attempt through.
    fn breaker_down(&self, server: usize) -> bool {
        let slots = self.breaker.lock().unwrap_or_else(|p| p.into_inner());
        slots[server]
            .down_until
            .is_some_and(|until| Instant::now() < until)
    }

    /// A server answered successfully: clear its breaker.
    fn breaker_success(&self, server: usize) {
        let mut slots = self.breaker.lock().unwrap_or_else(|p| p.into_inner());
        slots[server] = BreakerSlot::default();
    }

    /// A server failed on the wire (timeout/transport): arm or grow its
    /// skip window.
    fn breaker_failure(&self, server: usize) {
        let mut slots = self.breaker.lock().unwrap_or_else(|p| p.into_inner());
        let slot = &mut slots[server];
        slot.strikes = slot.strikes.saturating_add(1);
        let shift = slot.strikes.saturating_sub(1).min(5);
        let window = (BREAKER_MIN * 2u32.pow(shift)).min(BREAKER_MAX);
        slot.down_until = Some(Instant::now() + window);
    }

    /// Append every item concurrently: placement is rendezvous per
    /// `(ino, off / EXTENT_DATA_MAX)`; with `replicas == 2` the primary
    /// chain-writes to the rack-separated secondary. Any failure fails the
    /// whole batch (partial appends leak harmlessly — GC material, P27).
    pub(crate) fn append_batch(&self, items: &[(u64, u64, &[u8])]) -> Result<Vec<ExtentRef>> {
        let results: Vec<Result<ExtentRef>> = self.rt.block_on(async {
            let futures: Vec<_> = items
                .iter()
                .map(|&(ino, off, bytes)| {
                    let chunk_key = off / EXTENT_DATA_MAX;
                    let write_id = ((self.mount_seed as u128) << 64)
                        | self.write_seq.fetch_add(1, Ordering::Relaxed) as u128;
                    async move {
                        if self.replicas == 2 {
                            let (primary, secondary) = self.map.place_replicas(ino, chunk_key);
                            let bound = self.write_bound(&[primary, secondary]);
                            let secondary_addr = self.members[secondary].addr;
                            let (extent, replica_extent) = self
                                .timed_write(
                                    self.clients[primary].write_extent_replicated(
                                        write_id,
                                        ino,
                                        off,
                                        bytes.to_vec(),
                                        Some(secondary_addr),
                                    ),
                                    primary,
                                    bound,
                                )
                                .await?;
                            let replica_extent = replica_extent.ok_or_else(|| {
                                MdsError::Cluster(format!(
                                    "chunkserver {} acked a replicated write without a replica id",
                                    self.members[primary].addr
                                ))
                            })?;
                            Ok(ExtentRef::Remote {
                                server: primary as u32,
                                extent,
                                replica: Some((secondary as u32, replica_extent)),
                            })
                        } else {
                            let primary = self.map.place(ino, chunk_key);
                            let bound = self.write_bound(&[primary]);
                            let extent = self
                                .timed_write(
                                    self.clients[primary].write_extent(
                                        write_id,
                                        ino,
                                        off,
                                        bytes.to_vec(),
                                    ),
                                    primary,
                                    bound,
                                )
                                .await?;
                            Ok(ExtentRef::Remote {
                                server: primary as u32,
                                extent,
                                replica: None,
                            })
                        }
                    }
                })
                .collect();
            join_all(futures).await
        });
        results.into_iter().collect()
    }

    /// Read the requested subranges in order, primary first with per-server
    /// pipelining (`call_many`); per-request failures re-issue against the
    /// replica copy. A request whose every copy failed is an error (`EIO`).
    pub(crate) fn read_ranges(&self, reqs: &[ReadReq]) -> Result<Vec<Vec<u8>>> {
        let mut out: Vec<Option<Vec<u8>>> = reqs.iter().map(|_| None).collect();
        let mut primary: Vec<Vec<CopyReq>> = self.clients.iter().map(|_| Vec::new()).collect();
        for (idx, req) in reqs.iter().enumerate() {
            match req.eref {
                ExtentRef::Remote { server, extent, .. } => {
                    let slot = primary.get_mut(server as usize).ok_or_else(|| {
                        MdsError::Corrupt(format!(
                            "extent ref points at server {server}, membership has {} servers",
                            self.clients.len()
                        ))
                    })?;
                    slot.push(CopyReq {
                        idx,
                        extent,
                        off: req.off,
                        len: req.len,
                    });
                }
                ExtentRef::Local(_) => {
                    return Err(MdsError::Corrupt(
                        "local extent ref in a cluster-mode database".to_string(),
                    ));
                }
            }
        }
        self.rt.block_on(async {
            for (server, group) in primary.iter().enumerate() {
                if !group.is_empty() && !self.breaker_down(server) {
                    self.read_group(server, group, &mut out).await;
                }
            }
            // Failover pass: every request the primary did not answer
            // re-issues against its replica copy, again batched per server.
            let mut replica: Vec<Vec<CopyReq>> = self.clients.iter().map(|_| Vec::new()).collect();
            for (idx, req) in reqs.iter().enumerate() {
                if out[idx].is_some() {
                    continue;
                }
                if let ExtentRef::Remote {
                    replica: Some((server, extent)),
                    ..
                } = req.eref
                {
                    replica[server as usize].push(CopyReq {
                        idx,
                        extent,
                        off: req.off,
                        len: req.len,
                    });
                }
            }
            for (server, group) in replica.iter().enumerate() {
                if !group.is_empty() && !self.breaker_down(server) {
                    self.read_group(server, group, &mut out).await;
                }
            }
            // Desperation pass: a request still unanswered has every
            // copy either failed or breaker-skipped — retry ALL copies
            // ignoring breaker marks (a recovered server clears its mark
            // on this success; a genuinely all-down request keeps its
            // EIO, now bounded by one timeout per copy).
            for (idx, req) in reqs.iter().enumerate() {
                if out[idx].is_some() {
                    continue;
                }
                if let ExtentRef::Remote {
                    server,
                    extent,
                    replica,
                    ..
                } = req.eref
                {
                    let mut copies = vec![(server, extent)];
                    if let Some(pair) = replica {
                        copies.push(pair);
                    }
                    for (server, extent) in copies {
                        match tokio::time::timeout(
                            self.timeouts.read,
                            self.clients[server as usize].read_extent_range(
                                extent,
                                u64::from(req.off),
                                req.len,
                            ),
                        )
                        .await
                        {
                            Ok(Ok(data)) => {
                                self.breaker_success(server as usize);
                                out[idx] = Some(data);
                                break;
                            }
                            Ok(Err(_)) => {}
                            Err(_) => {
                                self.clients[server as usize].disconnect().await;
                            }
                        }
                    }
                }
            }
        });
        let mut result = Vec::with_capacity(reqs.len());
        for (idx, slot) in out.into_iter().enumerate() {
            result.push(slot.ok_or_else(|| {
                let req = &reqs[idx];
                MdsError::Cluster(format!(
                    "extent {:?} unreadable: every copy failed ({} bytes at offset {})",
                    req.eref, req.len, req.off
                ))
            })?);
        }
        Ok(result)
    }

    /// One batched read pass against `server`: a pipelined `call_many`;
    /// a per-request (`Remote`) error falls back to single timed calls to
    /// isolate the failures; a transport error or timeout leaves every
    /// request of the group unanswered (the caller fails those over).
    async fn read_group(&self, server: usize, group: &[CopyReq], out: &mut [Option<Vec<u8>>]) {
        let client = &self.clients[server];
        let requests: Vec<Request> = group
            .iter()
            .map(|copy| Request::ReadExtentRange {
                extent_id: copy.extent,
                offset: u64::from(copy.off),
                len: copy.len,
            })
            .collect();
        match tokio::time::timeout(
            self.timeouts.read,
            client.call_many(&requests, PIPELINE_WINDOW),
        )
        .await
        {
            Ok(Ok(responses)) => {
                self.breaker_success(server);
                for (copy, response) in group.iter().zip(responses) {
                    if let Response::ReadAck { data } = response {
                        out[copy.idx] = Some(data);
                    }
                }
            }
            Ok(Err(RpcError::Remote { .. })) => {
                // The server is alive but at least one request errored;
                // isolate per request (the batch aborts at the first).
                self.breaker_success(server);
                for copy in group {
                    match tokio::time::timeout(
                        self.timeouts.read,
                        client.read_extent_range(copy.extent, u64::from(copy.off), copy.len),
                    )
                    .await
                    {
                        Ok(Ok(data)) => out[copy.idx] = Some(data),
                        Ok(Err(_)) => {} // per-request error: the failover pass takes it
                        Err(_) => {
                            client.disconnect().await;
                        }
                    }
                }
            }
            _ => {
                client.disconnect().await;
                self.breaker_failure(server);
            }
        }
    }

    /// Fsync the cluster: one timed `sync` per server, all concurrently;
    /// EVERY server must confirm (a partial durability ack is a lie). A
    /// breaker-down server is not skipped — the barrier is strict — but
    /// it gets only a read-timeout probe: a still-dead server fails the
    /// fsync in ~READ_TIMEOUT instead of the full sync timeout, a
    /// restarted one clears its mark on the probe success.
    pub(crate) fn sync(&self) -> Result<()> {
        self.rt.block_on(async {
            let futures: Vec<_> = self
                .clients
                .iter()
                .enumerate()
                .map(|(server, client)| {
                    let addr = self.members[server].addr;
                    async move {
                        let bound = if self.breaker_down(server) {
                            self.timeouts.read
                        } else {
                            self.timeouts.sync
                        };
                        match tokio::time::timeout(bound, client.sync()).await {
                            Ok(Ok(_)) => {
                                self.breaker_success(server);
                                Ok(())
                            }
                            Ok(Err(err)) => Err(MdsError::Cluster(format!(
                                "sync on chunkserver {addr} failed: {err}"
                            ))),
                            Err(_) => {
                                client.disconnect().await;
                                self.breaker_failure(server);
                                Err(MdsError::Cluster(format!(
                                    "sync on chunkserver {addr} timed out after {bound:?}"
                                )))
                            }
                        }
                    }
                })
                .collect();
            join_all(futures).await.into_iter().collect::<Result<()>>()
        })
    }

    /// Tombstone every copy of every ref, tolerating `NotFound` (the copy
    /// is already gone — salvaged after a crash, or tombstoned earlier).
    pub(crate) fn discard_tolerant(&self, refs: &[ExtentRef]) -> Result<()> {
        let mut copies: Vec<(usize, u64)> = Vec::new();
        for eref in refs {
            match *eref {
                ExtentRef::Remote {
                    server,
                    extent,
                    replica,
                } => {
                    self.check_server_index(server)?;
                    copies.push((server as usize, extent));
                    if let Some((server, extent)) = replica {
                        self.check_server_index(server)?;
                        copies.push((server as usize, extent));
                    }
                }
                ExtentRef::Local(_) => {
                    return Err(MdsError::Corrupt(
                        "local extent ref in a cluster-mode database".to_string(),
                    ));
                }
            }
        }
        self.rt.block_on(async {
            let futures: Vec<_> = copies
                .iter()
                .map(|&(server, extent)| {
                    let client = &self.clients[server];
                    let addr = self.members[server].addr;
                    async move {
                        if self.breaker_down(server) {
                            // Leak-tolerant: the tombstone is GC material
                            // (P27); blocking the write path on a dead
                            // server would be worse.
                            return Ok(());
                        }
                        match tokio::time::timeout(self.timeouts.write, client.tombstone(extent))
                            .await
                        {
                            Ok(Ok(())) => {
                                self.breaker_success(server);
                                Ok(())
                            }
                            Ok(Err(RpcError::Remote {
                                code: ErrorCode::NotFound,
                                ..
                            })) => Ok(()),
                            Ok(Err(err)) => Err(MdsError::Cluster(format!(
                                "tombstone of extent {extent} on chunkserver {addr} failed: {err}"
                            ))),
                            Err(_) => {
                                client.disconnect().await;
                                self.breaker_failure(server);
                                Err(MdsError::Cluster(format!(
                                    "tombstone of extent {extent} on chunkserver {addr} timed out"
                                )))
                            }
                        }
                    }
                })
                .collect();
            join_all(futures).await.into_iter().collect::<Result<()>>()
        })
    }

    /// Probe every copy of every ref (flat, in ref order): one byte at
    /// offset 0 decides `Live`; `NotFound` only ever comes from a REACHABLE
    /// server; everything else is `Unreachable`. Probes batch per server;
    /// a server that fails its liveness check classifies all of its copies
    /// `Unreachable` in one timeout instead of one timeout per extent.
    pub(crate) fn probe(&self, refs: &[ExtentRef]) -> Result<Vec<Probe>> {
        let mut copies: Vec<(usize, u64)> = Vec::new();
        for eref in refs {
            match *eref {
                ExtentRef::Remote {
                    server,
                    extent,
                    replica,
                } => {
                    self.check_server_index(server)?;
                    copies.push((server as usize, extent));
                    if let Some((server, extent)) = replica {
                        self.check_server_index(server)?;
                        copies.push((server as usize, extent));
                    }
                }
                ExtentRef::Local(_) => {
                    return Err(MdsError::Corrupt(
                        "local extent ref in a cluster-mode database".to_string(),
                    ));
                }
            }
        }
        let mut out = vec![Probe::Unreachable; copies.len()];
        let mut by_server: Vec<Vec<usize>> = self.clients.iter().map(|_| Vec::new()).collect();
        for (copy_idx, &(server, _)) in copies.iter().enumerate() {
            by_server[server].push(copy_idx);
        }
        self.rt.block_on(async {
            for (server, group) in by_server.iter().enumerate() {
                if group.is_empty() {
                    continue;
                }
                let client = &self.clients[server];
                // Liveness gate: one round trip classifies a dead server for
                // the whole group (a partition answers with silence, and one
                // timeout per extent would make mounts glacial).
                match tokio::time::timeout(self.timeouts.read, client.stats()).await {
                    Ok(Ok(_)) => {}
                    Ok(Err(_)) => continue, // every copy of the group stays Unreachable
                    Err(_) => {
                        client.disconnect().await;
                        continue;
                    }
                }
                let requests: Vec<Request> = group
                    .iter()
                    .map(|&copy_idx| Request::ReadExtentRange {
                        extent_id: copies[copy_idx].1,
                        offset: 0,
                        len: 1,
                    })
                    .collect();
                match tokio::time::timeout(
                    self.timeouts.read,
                    client.call_many(&requests, PIPELINE_WINDOW),
                )
                .await
                {
                    Ok(Ok(responses)) => {
                        for (&copy_idx, response) in group.iter().zip(responses) {
                            out[copy_idx] = match response {
                                Response::ReadAck { .. } => Probe::Live,
                                _ => Probe::Unreachable,
                            };
                        }
                    }
                    Ok(Err(RpcError::Remote { .. })) => {
                        // Server reachable; isolate the per-extent answers.
                        for &copy_idx in group {
                            let extent = copies[copy_idx].1;
                            out[copy_idx] = match tokio::time::timeout(
                                self.timeouts.read,
                                client.read_extent_range(extent, 0, 1),
                            )
                            .await
                            {
                                Ok(Ok(_)) => Probe::Live,
                                Ok(Err(RpcError::Remote {
                                    code: ErrorCode::NotFound,
                                    ..
                                })) => Probe::NotFound,
                                Ok(Err(_)) => Probe::Unreachable,
                                Err(_) => {
                                    client.disconnect().await;
                                    Probe::Unreachable
                                }
                            };
                        }
                    }
                    _ => {
                        client.disconnect().await;
                    } // died mid-probe: group stays Unreachable
                }
            }
        });
        Ok(out)
    }

    /// Per-server `stats`, summed. An unreachable server is an error (a
    /// statfs that silently skips servers would lie about capacity); a
    /// breaker-down server gets the same read-timeout probe as `sync`.
    fn aggregate_stats(&self) -> Result<AggregateStats> {
        self.rt.block_on(async {
            let futures: Vec<_> = self
                .clients
                .iter()
                .enumerate()
                .map(|(server, client)| {
                    let addr = self.members[server].addr;
                    async move {
                        match tokio::time::timeout(self.timeouts.read, client.stats()).await {
                            Ok(Ok(stats)) => {
                                self.breaker_success(server);
                                Ok(stats)
                            }
                            Ok(Err(err)) => Err(MdsError::Cluster(format!(
                                "stats on chunkserver {addr} failed: {err}"
                            ))),
                            Err(_) => {
                                client.disconnect().await;
                                self.breaker_failure(server);
                                Err(MdsError::Cluster(format!(
                                    "stats on chunkserver {addr} timed out"
                                )))
                            }
                        }
                    }
                })
                .collect();
            let mut aggregate = AggregateStats::default();
            for stats in join_all(futures).await {
                let stats = stats?;
                aggregate.device_size += stats.device_size;
                aggregate.tail += stats.tail;
                aggregate.live_bytes += stats.live_bytes;
                aggregate.extent_count += stats.extent_count;
            }
            Ok(aggregate)
        })
    }

    /// The timeout for one write attempt: the full write window when the
    /// placement's servers are all healthy; a single read-timeout probe
    /// when any of them is breaker-down (the idempotent write doubles as
    /// the recovery probe — a restarted server rejoins on the first try).
    fn write_bound(&self, placement: &[usize]) -> Duration {
        if placement.iter().any(|&server| self.breaker_down(server)) {
            self.timeouts.read
        } else {
            self.timeouts.write
        }
    }

    /// One timed write-op call against `server`; outcomes feed the
    /// circuit breaker (a `Remote` error proves reachability and marks
    /// neither success nor failure).
    async fn timed_write<T>(
        &self,
        call: impl std::future::Future<Output = std::result::Result<T, RpcError>>,
        server: usize,
        bound: Duration,
    ) -> Result<T> {
        let addr = self.members[server].addr;
        match tokio::time::timeout(bound, call).await {
            Ok(Ok(value)) => {
                self.breaker_success(server);
                Ok(value)
            }
            Ok(Err(err)) => Err(MdsError::Cluster(format!(
                "write on chunkserver {addr} failed: {err}"
            ))),
            Err(_) => {
                self.clients[server].disconnect().await;
                self.breaker_failure(server);
                Err(MdsError::Cluster(format!(
                    "write on chunkserver {addr} timed out after {bound:?}"
                )))
            }
        }
    }

    /// Guard against a persisted ref pointing outside the membership.
    fn check_server_index(&self, server: u32) -> Result<()> {
        if (server as usize) < self.clients.len() {
            Ok(())
        } else {
            Err(MdsError::Corrupt(format!(
                "extent ref points at server {server}, membership has {} servers",
                self.clients.len()
            )))
        }
    }
}

/// Learn every member's store UUID over unpinned connections (the identity
/// that `cluster_config` then pins): per endpoint a timed `stats` forces
/// connect + Hello, after which the client reports the served UUID.
pub(crate) fn learn_membership(
    members: &[(SocketAddr, String, String)],
    timeouts: ClusterTimeouts,
) -> Result<Vec<MemberSpec>> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .map_err(|err| MdsError::Cluster(format!("tokio runtime: {err}")))?;
    rt.block_on(async {
        let mut specs = Vec::with_capacity(members.len());
        for (addr, rack, chassis) in members {
            let client = ChunkClient::new(*addr);
            match tokio::time::timeout(timeouts.connect, client.stats()).await {
                Ok(Ok(_)) => {}
                Ok(Err(err)) => {
                    return Err(MdsError::Cluster(format!(
                        "chunkserver {addr} answered the format probe with an error: {err}"
                    )));
                }
                Err(_) => {
                    return Err(MdsError::Cluster(format!(
                        "chunkserver {addr} unreachable within {:?}",
                        timeouts.connect
                    )));
                }
            }
            let uuid = client.store_uuid().ok_or_else(|| {
                MdsError::Cluster(format!(
                    "chunkserver {addr} connected without revealing its store uuid"
                ))
            })?;
            specs.push(MemberSpec {
                addr: *addr,
                rack: rack.clone(),
                chassis: chassis.clone(),
                uuid,
            });
        }
        Ok(specs)
    })
}

/// Hash a topology tag (rack/chassis) into the `u64` domain porfs-cluster
/// places on; stable across mounts (pure function of the string).
fn hash_tag(tag: &str) -> u64 {
    twox_hash::XxHash64::oneshot(0, tag.as_bytes())
}

/// Per-mount random seed for write ids: time ^ pid, same construction as
/// the RPC client's connection nonce.
fn mount_seed() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    nanos ^ (std::process::id() as u64) << 32
}
