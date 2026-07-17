# PolarisFS ROADMAP v6 — full GPFS feature parity (40 phases)

> 2026-07-17 · supersedes v5 (old versions live in git history); the v2.0 DASE design
> draft is archived at `docs/design-v2-dase-archive.md` for reference only.
> Team: 1 human developer + AI pair.
> One line: **an open-source, modern GPFS. Copy the principles and the full feature
> set; modernize the entire technology stack; lean on open-source libraries.**

## 0. Positioning & technology decisions

**Goal**: a shared POSIX parallel filesystem with GPFS feature parity, starting at
10–50 node clusters. GPFS is old (kmod client, 1990s kernel assumptions, dedicated
ops teams, closed source, million-line codebase). We rewrite — full feature parity,
modern architecture, no feature cuts.

### 0.1 GPFS feature-parity checklist (all of it is in the plan; no cuts)

| GPFS feature | Our phase | GPFS feature | Our phase |
|---|---|---|---|
| Distributed byte-range tokens (metanode) | P14–P15 | Snapshots (global/fileset) | P21–P22 |
| Data-shipping fallback | P14 | Writable clones | P22 |
| Striping | P8 | Filesets/junctions | P22 |
| 2/3-way replication + metadata replicas | P9 | user/group/fileset quotas | P23 |
| Failure-group placement | P10 | NFSv4 ACLs | P24 |
| mmap/O_APPEND/fcntl strong semantics | P17 | Storage pools + placement policy | P25 |
| Online disk add/restripe | P28 | ILM policy engine (mmapplypolicy) | P26 |
| Online disk remove/drain | P29 | QoS (mmchqos) | P27 |
| Declustered rebuild | P30 | At-rest encryption | P36 |
| fsck (mmfsck) | P31 | Compression | P37 |
| Rolling upgrades | P32 | WORM/immutability (mmchattr -i) | P38 |
| Backup support (mmbackup) | P33 | Multi-cluster remote mount + AFM | P39 |
| NFS/S3 export (CES; SMB dropped) | P40 | Performance monitoring (mmpmon) | P12 |

### 0.2 What we have that GPFS doesn't (differentiators)

- **CRC32C end-to-end checksums + built-in scrub** (GPFS has no disk-layer E2E checksums)
- **Modern client**: FUSE + io_uring + passthrough, no kmod, no kernel-version lock-in
- **Rust memory safety** (data + control plane) with Zig hot-path components (manual rings, no GC)
- Log-structured on-disk layout → zero-copy snapshots, no journal replay on recovery
- Open source on commodity hardware, no subscriptions

### 0.3 What we copy vs. don't copy

**Copied from GPFS (principles)**:
- Byte-range tokens/leases: clients cache writes only while holding a lease;
  conflicts trigger revocation callbacks (P14)
- **Distributed lock management (the metanode idea — GPFS's soul, kept intact)**:
  each file's lock arbitrator is hashed by inode across cluster nodes; lock tables
  are memory-only; failures are rebuilt via lease timeouts + epoch fencing +
  grace-period re-registration (P14–P15)
- Data-shipping fallback; large buffer pools + striped parallel reads from many
  storage nodes
- The full data-management concept set: failure groups, storage pools, ILM, filesets

**Not copied**: no quorum/group-services cluster state machine — membership and
epochs are issued by the single MDS (namespace stays serialized on one MDS; that is
a **control-plane** simplification only. **The lock plane is distributed**; the two
are decoupled). No shared-SAN-disk assumption: clients talk directly to chunkservers
(a modernization of the GPFS-NSD server model).

| GPFS technology | Our technology | Why |
|---|---|---|
| Kernel-module client (mmfs) | **FUSE**, no kmod | No kernel-version lock-in; a crash doesn't take the machine down |
| Per-request syscalls through /dev/fuse | **FUSE over io_uring** (kernel ≥6.15) + **FUSE passthrough** (≥6.9 hot-data shortcut) | Free performance from the modern kernel |
| Multi-threaded epoll daemons | **io_uring everywhere** (extent store/RPC/bench) | Per-core batched submission; saturate NVMe |
| C + proprietary daemons | **Rust as the main language**, **Zig hot-path components** (P20 fuse-io_uring transport, P40 data plane) | Memory safety and zero-overhead where each matters |
| quorum/group services | MDS-issued membership & epochs | Drops the heaviest cluster state machine |

**Kernel baseline**: ≥6.9 (passthrough); sweet spot ≥6.15 (fuse io_uring). Dev box runs 7.1.

### 0.4 Library selection (borrow, don't build)

| Subsystem | Choice | Why not build it |
|---|---|---|
| FUSE client | `fuser` (plus our own io_uring transport in Zig from P20) | Mature Rust implementation of the libfuse protocol |
| io_uring | `io-uring` crate (no runtime dependency on the data plane) | Thin bindings; we only write ring submission policy |
| Control-plane async | `tokio` + `tokio-util` codec | Ecosystem default; RPC/timers included |
| Data-plane runtime (optional) | `glommio` (thread-per-core on io_uring; decision at P7) | Saves a hand-rolled reactor |
| Metadata persistence | `redb` (pure-Rust ACID KV; P3 decision: redb vs extent-store-backed, leaning redb) | Embedded B-tree+WAL already exists |
| MDS standby replication | `raft-rs` (TiKV's production Raft, P16 wraps a tiny state machine) | **Not writing Raft is a rule of this roadmap** |
| Serialization | `serde`+`bincode` (RPC/metadata); `bytemuck` (on-disk Pod) | — |
| Checksums | `crc32c` crate (SSE4.2) + in-house PCLMULQDQ folding path (P1, property-tested against the crate) | Data-path necessity; the fast path is already done |
| Concurrency | `dashmap` / `parking_lot` / `crossbeam` | — |
| Logs/metrics | `tracing`; `metrics` + `metrics-exporter-prometheus` | P12 plugs straight in |
| EC (P35) | `reed-solomon-simd` (or `reed-solomon-erasure`) | Production-grade codecs |
| Compression (P37) | `zstd` | — |
| Encryption (P36) | RustCrypto: `aes-xts` / `aes-gcm` | — |
| NFS gateway (P40) | `nfsserve` crate | NFS server skeleton exists |
| S3 gateway (P40) | `s3s` crate | S3 protocol layer exists |
| CLI / errors | `clap` derive; `thiserror` (libs) / `anyhow` (bins) | — |

**Code-size discipline ("small and sharp")**: one rule only — **if a mature
open-source library solves it, don't write it yourself**; in-house code covers the
differentiating parts (on-disk format, extent policy, lease protocol, striping
maps). Line counts are references, not ceilings: never shuffle code to fit a budget,
no per-file/per-phase LOC caps.
**Difficulty discipline: the feature list is never cut (all 40 phases ship); the
only thing we ever cut is the urge to build what a library already solves.**

## 1. Target architecture

```
        ┌──── polaris-mds (single active + hot standby) ────┐
        │ dir tree/inodes + file→chunk map (CRUSH pure fn)   │
        │ cluster membership + epoch issuing (only center)   │
        └──┬──────────────┬──────────────┬──────────────────┘
     metadata RPC      data/lease RPC, parallel (io_uring)
   ┌───────▼──┐   ┌──────▼─────┐  ┌─────▼──────┐
   │ FUSE client│  │chunkserver1│  │chunkserver2│ ×N
   │ passthrough│─▶│ io_uring   │  │ io_uring   │
   │ writeback │   │ extent+WAL │  │ CRC32C     │
   └─────┬────┘   │ +lock arb. │  │ +lock arb. │
         └────────┴────────────┴───┴────────────┘
   Distributed lease plane: hash(inode) → per-file arbitrator (first alive),
   in-memory lock tables + revocation callbacks + data-shipping + epoch/grace recovery
```

- v1 consistency promise: **close-to-open**; P14–P17 builds up to GPFS-grade strong semantics.
- v1 redundancy: 2-way chain replication; EC lands in P35.

## 2. The forty phases

### Act 1: single-node MVP (P1–P6) — prove it can be a filesystem at all

**P1 · On-disk format v0 + io_uring extent store + bench** (~2 wks) ✅ DONE
Delivered: `docs/format.md`, `porfs-format`, `porfs-store` (append-only extent log +
CRC32C + dual superblocks + salvage scan + zero-copy `read_batch_into`),
`porfs mkfs/info/bench`.
Gate (semantics calibrated): **seq write ≥85% of same-device raw fio (CRC included)
— ✅ measured 94%**; **seq read, two layers**: pure io_uring pipeline ≥ raw fio read
(engine has zero loss) — ✅ measured 111%; verified read vs a **fio+verify baseline**
(raw fio does no checksums; comparing a verified read against unverified fio on a
low-memory-bandwidth box is apples-to-oranges: verified+copied read moves ~4 bytes
of bus traffic per logical byte vs fio's 1, and this dev APU has 6.9GB/s single-core
DRAM < 2× device speed — physically unreachable here; dual-channel servers have no
such wall); CRC corruption detection 100% ✅; `cargo test` green (34) ✅; clippy zero ✅.

**P2 · WAL + group commit + crash recovery** (1–2 wks) ✅ DONE
Delivered: the extent log IS the WAL; group-commit policy + `confirmed_id` durability
horizon; checkpoint slots (ping-pong, generational) for scan-free mount; superblock
generation switching; `scripts/kill9-soak.sh`.
Gate: **1000 kill -9 iterations, zero corruption, zero confirmed-write loss — ✅
(63s, all 1000 mounts via checkpoint)**.

**P3 · MDS v0 (library form, single node)** (2 wks) ✅ DONE
Delivered: `porfs-mds` — directory tree/inode table/file→extent interval map on **redb**
(ACID, one write txn per namespace mutation → atomic rename; persistence decision: redb
over extent-store-backed, matching the future MDS/chunkserver device split), log-structured
COW write path (append extents → commit map → tombstone replaced), truncate/sparse holes,
reconcile-on-mount for un-fsynced writes, `porfs mds-check`.
Gate: metadata ops survive crashes — ✅ (20-round unclean-reopen loop with full model
verification, torn-tail reconcile, fsync durability; 92 workspace tests green, clippy 0);
self-check passes ✅ (also detects planted corruption).

**P4 · FUSE client v0** (2 wks) ✅ DONE
Delivered: `porfs-fuse` — fuser::Filesystem over the MDS via a dedicated worker thread
(the `!Send` MDS never crosses threads; the channel seam is where P6 RPC plugs in),
`porfs mount` subcommand, errno/attr mapping, attribute-cache TTLs, statfs.
Gate: POSIX basic ops through a real mount — ✅ 11 real-mount integration tests
(std::fs suite mirroring pjdfstest basic categories: multi-extent I/O, sparse,
truncate, rename/link/unlink/readdir/setattr/fsync, remount persistence);
pjdfstest itself is not packaged on this box — it becomes mandatory at P5.
Found & fixed en route: test-suite unmount wedge (leaked open fds → EBUSY →
session-join deadlock; lazy-detach fallback added).

**P5 · POSIX completion I** (2 wks)
xattr, sparse files, sharded big-directory index (1M entries listed fast), rename
edge cases, fsync/fdatasync semantics.
Gate: full pjdfstest pass; 1M-file directory ls/find hits performance targets.

**P6 · MVP freeze (gate phase)** (1 wk)
`porfs mkfs + mount` one command; real-workload smoke: git clone, kernel build, sqlite stress.
Gate: all three run clean with zero data errors. **First "usable" milestone.**

### Act 2: multi-node parallelism (P7–P13) — become "distributed"

**P7 · RPC + chunkserver service** (2 wks)
Length-prefixed binary frames over TCP (tokio-util codec + bincode; evaluate glommio
for the data plane), static membership, extent read/write as a service.
Gate: cross-machine extent I/O correct; reconnect semantics documented.

**P8 · Striped parallel read** (2 wks)
File→chunk→chunkserver mapping as a pure function (CRUSH-style, no central lookup);
clients read in parallel.
Gate: 4-client aggregate read ≥ 70% of pool bandwidth.

**P9 · Parallel write + 2-way chain replication** (2 wks)
Primary forwards to secondary; ACK only after both; replica consistency checks;
metadata dual-replicated.
Gate: killing any chunkserver loses zero confirmed writes; reads fail over to the survivor.

**P10 · Failure groups + placement policy** (1–2 wks)
Nodes/disks tagged with failure domains (rack/chassis); replicas forced across
failure groups; CRUSH input carries topology.
Gate: simulated full-rack power loss loses nothing and stays readable.

**P11 · Close-to-open consistency** (1–2 wks)
Client attribute/page-cache timeout model documented + implemented; open forces revalidation.
Gate: multi-machine open/close cross-read/write verification shows zero errors.

**P12 · Observability** (1 wk)
prometheus metrics (latency histograms/bandwidth/replica watermarks), `porfsadm` CLI,
`tracing` structured-log conventions.
Gate: dashboards answer "which layer is slow".

**P13 · Production v0.1 rollout (gate phase)**
Run "rebuildable data" (dataset replicas/distribution files) on our own cluster,
4 weeks, zero incidents. **If we won't run it ourselves → downgrade to a learning project.**

### Act 3: distributed consistency (P14–P18) — GPFS's soul

**P14 · Distributed lease manager (DLM) core** (6–8 wks) — hardest phase of the roadmap
Per-file arbitrator = first alive node of an ordered candidate list from
hash(inode, epoch) (chunkservers double as arbitrators — the metanode idea);
in-memory byte-range lock tables; 30s lease timeout + renewal piggybacked on I/O;
conflict → revocation callback → bounded-time flush + release, overdue = client dead;
conflicting writes degrade to data-shipping during revocation; multi-range ops
acquire in (inode, offset) global order (no deadlocks); fcntl blocking locks map to
arbitrator wait queues + timeout fallback.
**Schedule contingency (features not cut): if not stable in 8 weeks → write-lease
moves out of v1 scope and ships later; v1 launches on close-to-open; the DLM stays
on the roadmap until done.**
Gate: zero errors in shared-write cross-verification with a live arbitrator;
revocation callback p99 < 2× lease timeout.

**P15 · Arbitrator failover + lease recovery** (3–4 wks)
MDS issues epochs; arbitrator death → next candidate takes over with a new epoch →
clients re-register held leases inside a grace period to rebuild the lock table,
expired ones are voided; epoch fencing kills split-brain (stale-epoch messages are
rejected outright).
Gate: lease service recovers <10s after killing an arbitrator; zero dirty data in
the recovery window; stale-epoch messages have zero effect.

**P16 · MDS hot standby + failover** (3–4 wks)
`raft-rs` wraps a tiny state machine for WAL replication and leader election (we do
not write Raft); membership/epoch state rebuilt with the state machine; semi-automatic switchover.
Gate: killing the MDS → standby takes over <30s with acceptable business blip.

**P17 · POSIX strong semantics II** (3–4 wks)
Cross-machine mmap coherence (bound to leases, dirty-page recall); multi-writer
O_APPEND atomicity; cross-directory atomic rename; cross-machine fcntl deadlock
detection (wait-for graph at the arbitrator).
Gate: concurrent mmap/O_APPEND/rename cross-verification zero errors; injected
deadlocks are detected and broken.

**P18 · 72h fault-injection soak (gate phase)**
Random kills of MDS/arbitrator/chunkserver/clients + network partitions + resident
shared-write verifier.
Gate: 72h, zero data errors, service recoverable.

### Act 4: client performance (P19–P20) — modern kernel features fully on

**P19 · Client performance I** (2 wks)
FUSE writeback cache + max_readahead/max_write tuning; stripe-aligned prefetch;
large-I/O coalescing; concurrency tuning.
Gate: single-client seq read ≥ 40% of raw device inside the fuser framework.

**P20 · Client performance II** (4 wks)
Local chunk read cache + **FUSE passthrough** (hot data shortcut, ≥6.9) → **FUSE
over io_uring transport** (≥6.15; first Zig candidate: no GC, hand-managed rings,
C ABI into Rust).
Gate: single-client seq read ≥ 70% of raw device (typical FUSE baseline <30%);
small-I/O latency ≤ raw device +50µs.

### Act 5: data services (P21–P27) — the GPFS data-management set

**P21 · COW global snapshots** (2–3 wks)
Log-structured write-in-free-space → instant zero-copy snapshots; retention policy
(last N / time window).
Gate: 1000 snapshots, no leaks; snapshot content byte-identical to snapshot time.

**P22 · Filesets + writable clones** (3 wks)
Directory subtrees with independent inode space + junction mountpoints; fileset-level
snapshots; snapshot → writable clone.
Gate: fileset quotas/snapshots act independently; clone writes never touch the source snapshot.

**P23 · Three-level quotas** (2 wks)
user/group/fileset quotas; soft/hard limits + grace time; crash-safe quota accounting.
Gate: over-limit writes rejected without hurting under-limit users; expired grace
turns soft into hard.

**P24 · NFSv4 ACLs** (2–3 wks)
NFSv4 ACL model + bidirectional mode-bit mapping; inheritance rules; getfacl/setfacl compatible.
Gate: ACL semantic test suite passes; mixed ACL/mode behavior is predictable.

**P25 · Storage pools + placement policy** (2–3 wks)
Chunkservers grouped into pools (e.g. nvme/sata); per-file/directory pool assignment;
new writes land by pool.
Gate: cross-pool striping of one file is correct; a full pool only rejects its own writes.

**P26 · ILM policy engine** (3–4 wks)
mmapplypolicy-style: rule language (size/age/pool/path patterns) → scan → inter-pool
migration/prefetch/eviction; files stay readable/writable during migration
(extent-by-extent moves + lease protection).
Gate: 1M-file scan+migrate zero errors; business degradation during migration <20%.

**P27 · Scrub + GC + QoS** (3 wks)
Background CRC scrub (silent-corruption detection → replica repair); extent GC to
reclaim holes; mClock-style four-class queues (client/recovery/scrub/migration) with throttling.
Gate: business degradation during scrub/GC <25%; injected bit-flips 100% repaired by scrub.

### Act 6: elasticity & operations (P28–P33) — GPFS's operational soul

**P28 · Online expansion + restripe** (3 wks)
Add chunkservers/disks online; incremental rebalance (new files stripe at the new
width, mmrestripefs-style); throttled so it never hurts the business.
Gate: doubling nodes ~linearly grows aggregate bandwidth; business stays online throughout.

**P29 · Online shrink + drain & retire** (2–3 wks)
Node/disk drain: data migrated out → marked read-only → removed; mmdeldisk equivalent.
Gate: zero data-unavailability windows during a drain.

**P30 · Declustered replica rebuild** (2–3 wks)
Detect under-replication after disk/node failure → pool-wide parallel rebuild
(declustered) → auto-restore target replica count; rebuild throttling.
Gate: replicas auto-heal after killing a disk; business degradation during rebuild <30%.

**P31 · porfs-fsck** (3 wks)
Online read-only consistency inspection + offline repair (directory tree/inodes/
extent maps/quota ledgers); mmfsck equivalent.
Gate: 20 injected corruption classes (orphan inodes, broken directory links, wrong
quotas, ...) all detected and repairable.

**P32 · Rolling upgrades** (2 wks)
Protocol version negotiation (min-common-version); N-1→N online upgrade drills;
on-disk format version gate (format.md bumps first).
Gate: mixed-version cluster at full load, zero errors; node-by-node upgrades invisible to the business.

**P33 · Backup & restore** (2–3 wks)
Snapshot-consistent export + incremental manifests (mmbackup-style, integrates
restic/rsync/tar); full+incremental restore drills.
Gate: backup→disaster-restore RTO/RPO documented in the ops manual and met.

### Act 7: v1.0 freeze (P34)

**P34 · v1.0 freeze (gate phase)** (2 wks)
Ops manual (install/upgrade/rollback/inspection/incident response); 30-day
long-haul; security baseline audit.
Gate: a non-author deploys once and handles one incident, manual only.

### Act 8: enterprise & ecosystem (P35–P40) — GPFS's advanced base, all committed

**P35 · EC 8+2 cold pool + DoM small files** (4–6 wks)
Erasure coding for capacity pools (`reed-solomon-simd`, 8+2 first, interfaces ready
for LDC wide stripes); ≤1MB small files stored with metadata, skipping the data path (DoM).
Gate: EC pool read bandwidth ≥ 6× single disk; killing any 2 disks loses nothing.

**P36 · Encryption** (3 wks)
At rest: per-fileset AES-256-XTS (RustCrypto), keys external (KMS integration);
in transit: TLS on all RPC.
Gate: encrypted-pool performance hit <15%; a pulled disk is unreadable.

**P37 · Async compression/dedup** (3–4 wks)
Background zstd compression + fingerprint dedup on capacity pools (never on the
write latency path); per-pool switches.
Gate: ≥1.8× capacity saving on compressible datasets; zero write-path regression.

**P38 · WORM/immutable files** (2 wks)
Immutability + retention period (mmchattr --immutable style); compliant delete audit log.
Gate: no path (including root) can modify/delete inside retention; deletable after expiry.

**P39 · Multi-cluster + AFM** (5–7 wks)
Remote-mount another cluster's namespace (GPFS multi-cluster parity) + AFM-style
site read/write caching (async prefetch, disconnected write-back).
Gate: cross-cluster reads run at local bandwidth after cache hits; post-outage write
recovery verifies zero errors; semantics documented.

**P40 · Ecosystem & hardware finale** (parallelizable)
NFSv4/S3 gateways (`nfsserve`/`s3s` crates as the base, CES parity; SMB dropped — no
mature Rust library and not our market) + RDMA/zero-copy data plane (second Zig
candidate). MDS sharding is the roadmap's only "contingency" item: it starts only
past 50 nodes and is not part of GPFS-parity basics.

## 3. Capacity & discipline

- P1–P34 total ≈ 14–20 months (10–15h/week); ≈ 6–8 months full-time. P35–P40 are all
  committed, ordered by real demand.
- No phase starts before the previous gate passes; P6/P13/P18/P34 are the
  "do we continue" decision points.
- With more hands (AI agent quota), acts 5–6 may parallelize internally
  (P21–P33 are mostly orthogonal subsystems).
- Format changes edit `docs/format.md` and bump the version first — the format is
  the contract, code is its servant.

## 4. Current actions

1. ✅ Repo cleanup, ROADMAP v6, P1 (format v0 + extent store + bench + fio baseline)
2. ✅ P2 (format v2: checkpoints + group commit + crash harness; 1000× kill -9 clean)
3. ✅ P3 (porfs-mds on redb; namespace txns + COW data path + reconcile; 92 tests green)
4. ✅ P4 (porfs-fuse + porfs mount; 11 real-mount tests green; 103 workspace tests)
5. Next: **P5 · POSIX completion I** (xattr, sparse, big-dir index, rename edges)
