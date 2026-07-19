# PolarisFS (plfs) ROADMAP v8 — GameFS design pivot (20 steps)

> 2026-07-19 · supersedes v7 (the GPFS/IBM Storage Scale compatibility roadmap; old
> versions live in git history). The project adopts the GameFS KISS design; the binding
> design contract is [`docs/design.md`](docs/design.md) (GameFS Design Document v0.2).
> Team: 1 human developer + AI pair.
> One line: **a KISS distributed file system for game server clusters — Raft-replicated
> metadata per volume, ChunkArena storage over raw block devices or sparse images,
> one binary, three roles, standalone mode out of the box.**

## 0. Positioning & technology decisions

**Naming (binding)**: project **PolarisFS**; binary **`plfs`**; library crates
`plfs-*`; gRPC proto package `plfs.data.v1`. Wherever the design doc says `gamefs`,
these names apply. On-disk magics follow the design doc (§6) unchanged.

**Goal** (design doc §1.2): aggregate the capacity of all storage machines (HDD) as
the durable tier; client-local SSD as persistent read cache + WAL; Raft-replicated
metadata (no metadata loss on any single-machine failure); instant O(1) volume
snapshots with rollback; POSIX semantics via FUSE; one machine (standalone) to tens of
machines (cluster); no host filesystem for data storage (raw block / sparse `.img`
with hole-punch space reclamation); library-first, one binary three roles, ~10k LoC
of our own Rust.

**KISS boundary (non-goals, design doc §1.3 — never re-litigate without owner
decision)**: no multi-writer cache coherence / byte-range locks (single-writer
assumption; Registry lease as escape hatch); no erasure coding (RF 2/3 suffices); no
striping logic (parallel chunk fetch gives aggregate bandwidth); no strongly
consistent cross-client namespace (pub/sub merge, ~1 s); no RDMA / kernel bypass; no
directory-level snapshots.

**Workload model** (design doc §1.1): game workloads are naturally single-writer —
no distributed write locks. This is the foundation of the design.

### 0.1 Library selection (borrow, don't build — design doc §2 is normative)

| Subsystem | Choice | Why not build it |
|---|---|---|
| Async runtime (control plane / RPC) | `tokio` 1.x (`full`) | ecosystem default |
| Storage-engine I/O (data path) | `io-uring` crate, O_DIRECT + buffered fallback | **owner amendment 2026-07-19**: porfs P1 lineage — SQ pipeline, aligned buffer pool; same engine pattern as the old extent store |
| RPC | `tonic` + `prost` 0.12+ | gRPC streaming, typed contracts, backpressure |
| FUSE | `fuser` 0.14+ (FUSE3) | maintained Rust FUSE bindings |
| Metadata state machine | `redb` 2.x | pure-Rust embedded ACID KV, savepoint API → checkpoints |
| Consensus | `openraft` 0.9+ | mature async Raft, multi-raft (one group per volume) |
| Serialization (meta) | `serde` + `bincode` | compact, versioned via enum discipline |
| CRC | `crc32fast` 1.x | SSE4.2 hardware CRC32C |
| IDs | `uuid` v7 | time-ordered chunk ids → HDD-friendly allocation locality |
| WAL | hand-rolled (~400 LoC) | no crate fits exactly (design doc §2 note) |
| Read cache index | `lru` + `redb` | RAM hot index + persistent catalog |
| Byte buffers | `bytes` | zero-copy chunk payload plumbing |
| Concurrency | `parking_lot`, `tokio-util`, `async-trait` | — |
| Sparse file / discard | `nix` (`fallocate`, `FALLOC_FL_PUNCH_HOLE`), `libc::ioctl(BLKDISCARD)` | §6.4 space reclamation |
| Registry store | `openraft` + `redb` | same stack as metadata groups |
| CLI | `clap` 4 (derive) | `plfs mkfs / data / meta / mount / registry / standalone ...` |
| Config | `serde` + `toml` | — |
| Observability | `tracing` + `metrics` + `metrics-exporter-prometheus` | three-graph dashboard |
| Errors | `thiserror` (lib) / `anyhow` (bin) | — |
| Fault-injection tests | `turmoil` 0.6+ | deterministic simulated cluster |
| Property tests | `proptest` 1.x | allocator, WAL replay, layout math |
| Benchmarks | `criterion` + external `fio` | — |

**Explicitly rejected** (design doc §2): rocksdb (C++ build chain; redb suffices at
our metadata scale), raft-rs (openraft's multi-group ergonomics win for per-volume
groups), HTTP data plane (gRPC gives streaming + typed contracts), hand-rolled FUSE
protocol layer (fuser covers FUSE3).

**Code-size discipline ("small and sharp")**: if a mature open-source library solves
it, don't write it yourself; in-house code covers only the differentiating parts
(ChunkArena, WAL, cache, FUSE semantics). Line counts are references, not ceilings.

### 0.2 Format-contract rule

On-disk and wire formats are contracts, edited **before** code:

- Arena on-disk format: `docs/format-arena.md` (written at S2, versioned;
  design doc §6 is the source of truth for v1).
- Data-plane wire protocol: the proto file under `crates/plfs-common/proto/`
  (design doc §5 is the source of truth for v1).
- A contradiction between contract and design doc = stop and report, never
  silently redesign.

## 1. Target architecture

One binary, three roles + standalone (design doc §3):

```
plfs mkfs   --device /dev/sdb            # or: --img /var/lib/plfs/disk0.img --size 14T
plfs data   --arena /dev/sdb             # chunk server role (deliberately dumb)
plfs meta   --volume save --peers ...    # metadata raft group member
plfs mount  --volume save /mnt/save      # FUSE client role (the smart component)
plfs registry --peers ...                # cluster control (3 nodes)
plfs standalone --dir /var/lib/plfs --mount /mnt/plfs   # all-in-one, RF=1
```

- **Client** (smart): FUSE daemon, metadata Raft leader for owned volumes (redb
  state machine), persistent SSD read cache (chunk-granular, LRU,
  version-validated), WAL with batch fsync, background workers (flusher,
  re-replication, GC, snapshot scheduler, scrubber client, cache eviction).
- **Data node** (dumb): ChunkArena instances, 5 RPCs, knows nothing about files,
  replicas, consistency, snapshots.
- **Registry**: 3-node openraft group (single-node in standalone); node inventory +
  liveness leases, cluster config, checkpoint pointers, pub/sub, metadata lease
  issuer. Never on the I/O path.
- Standalone uses the exact same code paths as cluster mode (single-node Raft,
  loopback gRPC). No special-case forks.

## 2. The twenty steps

Each step is a mergeable, testable increment with a gate. Steps 1–6 single-machine;
standalone usable at S10; cluster features complete at S16; hardening through S20.

**S1 · Repo & skeleton** ✅ DONE
Cargo workspace: `plfs-common` (proto/types), `plfs-arena`, `plfs-data`,
`plfs-meta`, `plfs-client`, `plfs` (bin). CI: `scripts/ci.sh` = fmt/clippy/test.
Delivered: wire contract v1 (`crates/plfs-common/proto/plfs/data/v1/chunkstore.proto`,
tonic 0.14.6 + tonic-prost codegen, package `plfs.data.v1`); clap CLI shell with all
six subcommands (mkfs/data/meta/mount/registry/standalone) + `debug_assert` self-check.
Gate: ✅ `cargo fmt --check` clean, ✅ `cargo clippy --workspace --all-targets` zero
warnings, ✅ `cargo test --workspace` green (6 tests); ci.sh exit 0 (verified by
orchestrator).

**S2 · ChunkArena core** (design doc §6.1–6.3) ✅ DONE
`docs/format-arena.md` v1 written first (binding contract). Delivered: `plfs-arena`
— mkfs on sparse img/block device (geometry fixpoint, real punch/BLKDISCARD
capability probes, PUNCH_OK/DISCARD_OK in superblock), dual superblocks + dual
bitmaps + slot R/W per contract, L/S slot classes (defaults 1 MiB/64 KiB, 90/10),
SEALED single-write publish, in-memory chunk_id→slot index, io_uring SQ-pipeline
engine (O_DIRECT + buffered fallback, porfs P1 lineage), boot header scan + bitmap
reconcile, exact-version idempotent Delete with hole punch.
Gate: ✅ proptest random Put/Get/Delete/List/Reopen then reopen ⇒ state consistent
(payloads byte-exact, live set == model, deleted ⇒ NotFound) — **256 cases green
(108 s, `scripts/gate-arena-proptest.sh`; CI default 8 cases ≈ 3.7 s)**; 38 unit +
5 integration tests green (boot reconcile both directions, duplicate resolution,
class boundaries, OutOfSpace, idempotency matrix); clippy zero; O_DIRECT+ring path
verified live on xfs (PUNCH_OK). Owner ruling: every `cargo test` test < 5 s —
deep gates moved behind `PROPTEST_CASES`/scripts.

**S3 · Arena crash consistency** (design doc §6.2, §6.5) ✅ DONE
Delivered: `Arena::abandon` (in-process crash simulation, no bitmap flush);
`tests/crash.rs` (seeded sequences with abandons at random + per-ack-boundary
points); `examples/arena_fuzz.rs` write/verify pair (deterministic payload
expander, fsynced acklog at exact ack points); `scripts/kill9-arena.sh` (real
kill -9 soak, ITERS env); criterion baseline.
Gate: ✅ kill -9 mid-Put fuzz — **100/100 iterations, zero false-allocated slots,
zero lost acked-live chunks, every payload byte-exact** (16.2 s, orchestrator-run);
✅ criterion baseline: put 16 KiB 936 µs, put 1 MiB 1.163 ms (~900 MiB/s incl.
fdatasync), get 1 MiB 512 µs (~2 GiB/s + crc), boot-scan 256 MiB half-full 71.2 ms
(~3.6 GiB/s); workspace 51 tests green, slowest 3.2 s. Contract observation (legal
per §5/§7, noted for S5/S7): address chunks by (chunk_id, version) — after a
power-loss a resurrected stale version can share a chunk_id with the newer live
one; GC re-deletes stale versions, never rely on bare-id Get across crashes.

**S4 · Space reclamation** (design doc §6.4)
Hole punch (sparse img: `FALLOC_FL_PUNCH_HOLE|FALLOC_FL_KEEP_SIZE`) / BLKDISCARD
(raw device) on Delete; capability probed at mkfs, recorded in superblock
(`PUNCH_OK`, `DISCARD_OK`); bitmap-only fallback when unsupported.
Gate: write 10 GiB → delete all → `du` returns to baseline.

**S5 · Data node gRPC** (design doc §5)
5 RPCs over ChunkArena: Put (idempotent on chunk_id+version+crc), Get (stream,
64 KiB frames, version+crc first frame, `if_version` ⇒ NOT_MODIFIED semantics),
Stat, Delete (exact-version, idempotent), List (stream, arena inventory).
Gate: idempotent Put / version-checked Delete proven by proptest over real
loopback gRPC.

**S6 · WAL** (design doc §7.2)
Segmented append log (64 MiB segments), crc per record, group-commit fsync (~1 ms
batching), prefix truncate on flush, replay at startup.
Gate: proptest — crash at random byte offsets ⇒ prefix-consistent replay
(no acknowledged-record loss, torn tail discarded).

**S7 · Metadata state machine** (design doc §7.3)
redb schema (ino/dentry/layout/chunkref/snap/gc); apply-op API (idempotent,
versioned enums); layout math (chunk slicing of file offsets).
Gate: proptest on layout math and op application vs a model.

**S8 · Single-node Raft + client core** (design doc §7.2–7.3)
openraft group (1 member) driving the S7 state machine; write path end-to-end
in-process: CoW chunk allocation → WAL → flush → quorum Put (RF=1 local) → Raft
metadata commit → WAL truncate → superseded chunks to GC queue.
Gate: write/read/fsync/unlink workload survives kill -9 at random points —
replayed state equals model (in-process harness).

**S9 · FUSE mount** (design doc §8)
fuser wiring: create/read/write/unlink/mkdir/rename/fsync/statfs +
documented degradations (EXDEV cross-volume, mmap(WRITE) ENODEV, relatime,
O_DIRECT → WAL).
Gate: `fio` randrw passes on a real mount; real game-asset-style directory tree
loads (bulk read) correctly.

**S10 · Standalone GA** (design doc §3.1)
`plfs standalone` one command: registry (1-node Raft) + meta group (1-node Raft) +
arena (sparse img) + WAL + cache + FUSE in one process.
Gate: full demo on one machine — mount, write saves, kill daemon, remount,
data intact.

**S11 · SSD read cache** (design doc §7.1)
Persistent catalog (redb) + RAM LRU hot index + Stat-validation + advisory read
lease (5 s); eviction at 85% watermark; readahead on open.
Gate: warm-start asset load within 1.2× of local SSD; hit-rate metric exported.

**S12 · 3-node metadata Raft** (design doc §7.3)
Multi-member groups; follower reads proxy to leader; failover < 1 s.
Gate: kill leader mid-workload — volume continues on follower, zero metadata
divergence.

**S13 · Registry** (design doc §4.3)
Membership, liveness leases, node stats, pub/sub; cluster mode boots.
Gate: 3 clients × 3 data nodes boot and serve a workload.

**S14 · Replication** (design doc §7.2, §7.4)
RF=2/3 placement (random + failure-domain aware), quorum flush, degraded marking,
client-local repair queue.
Gate: kill one data node — reads unaffected, self-heal visible in metrics.

**S15 · Snapshots** (design doc §9)
Volume-level read-only crash-consistent snapshots: O(1) create (WAL flush →
write-barrier → redb savepoint → row insert), `.snapshots` view sharing the SSD
cache, delete.
Gate: snapshot → delete/modify half the tree → original fully readable,
byte-exact.

**S16 · Rollback + GC + scheduler** (design doc §9)
Reversible rollback (implicit pre-rollback snapshot → pointer swap → orphaned
chunks to GC); refcount GC with snapshot awareness (version-checked Delete →
hole punch); retention scheduler (toml).
Gate: rollback restores byte-exact state and is itself reversible; GC reclaims
space (du evidence); retention policies run on schedule.

**S17 · Re-replication & scrubber** (design doc §7.4, §10.3)
Background repair workers (rate-limited 30 MB/s/disk); weekly List-driven scrub
with crc verify.
Gate: inject bit rot (flip bytes in a slot) → detected → repaired from healthy
replica.

**S18 · Cross-client sync** (design doc §4.3, §8.2)
Pub/sub merge of foreign volumes; metadata lease escape hatch for cross-client
writes.
Gate: two clients — file written on A visible on B ≤ 1 s.

**S19 · turmoil fault-injection soak**
Deterministic partitions/crashes/delays across all paths.
Gate: 72 h simulated soak — zero acknowledged-write loss, zero metadata
divergence.

**S20 · Production hardening**
Prometheus dashboard (cache hit rate / WAL backlog / per-disk queue depth),
panic-safe FUSE loop, upgrade/format-version policy, runbook, chaos drill vs.
design doc §10.2 table.
Gate: fio + game-workload benchmark report; chaos drill passes the failure table.

## 3. Capacity & discipline

- Estimated velocity (1 dev + AI, design doc §12): S1–S10 ≈ 5–6 weeks;
  S11–S16 ≈ 5 weeks; S17–S20 ≈ 3–4 weeks.
- No step starts before the previous gate passes; S10/S16/S20 are the
  "do we continue" decision points.
- Format changes edit the contract doc and bump the version first — the format is
  the contract, code is its servant.
- Definition of done (design doc §13): destroy one storage machine and
  fat-finger-delete a directory — full recovery in minutes via replicas and
  snapshots, zero data loss.

## 4. Current actions

1. ✅ S1 (repo & skeleton: 6 crates, wire contract v1, ci.sh green, 6 tests)
2. ✅ S2 (ChunkArena core: format-arena.md v1, mkfs/open/put/get/stat/delete/list,
   io_uring engine O_DIRECT+fallback, 256-case proptest green, 43 tests, all <5 s)
3. ✅ S3 (crash consistency: kill -9 100/100 zero false-alloc/loss; criterion
   baseline put 1 MiB 1.16 ms / get 512 µs / scan 3.6 GiB/s; 51 tests green)
4. 🏃 S4 (space reclamation) — in progress
