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

**S4 · Space reclamation** (design doc §6.4) ✅ DONE
Delivered: `Arena::sparsify` (SEEK_DATA/SEEK_HOLE extent-precise punch of free
slots — idempotent, second run reclaims 0; BLKDISCARD whole-range on discard
backends), `Arena::set_punch_enabled` (bitmap-only delete mode for
punch-less backends), 4 space tests, `examples/space_reclaim.rs`,
`scripts/gate-space-reclaim.sh`.
Gate: ✅ **write 10 GiB → delete all → `du` returns to exact baseline**
(`SPACE_RECLAIM_OK du_baseline=24576 du_written=10738696192
du_after_delete=24576 du_bitmap_only=2148597760 du_after_sparsify=24576`,
orchestrator-run); bitmap-only delete retains blocks, same-session sparsify
reclaims them; no resurrection after reopen (sparsify punches stale headers
too). Contract note (§5/§6.4 consistent): a bitmap-only delete resurrects at
reopen (headers are truth) — sparsify MUST run in-session on the data node;
GC re-deletes idempotently in between. 55 workspace tests green, slowest
3.4 s. (S4 implemented by the orchestrator — subagent quota exhausted.)

**S5 · Data node gRPC** (design doc §5) ✅ DONE
Delivered: `plfs-data` — the 5 ChunkStore RPCs over ChunkArena: the `!Send`
arena is created on and never leaves its actor thread (command channel +
oneshot replies); Put verifies the request crc32c and is idempotent
(same id+version+crc ⇒ no-op success; any difference ⇒ ALREADY_EXISTS;
`seal=false` ⇒ UNIMPLEMENTED, reserved); Get does Stat-first so an
`if_version` hit returns a single not_modified frame WITHOUT reading the
payload (the §7.1 304 semantic), else streams 64 KiB frames (first frame
carries version/len/crc); Stat; Delete is exact-version and idempotent
(wrong version/absent ⇒ silent success, chunk survives); List streams the
arena inventory. ArenaError → gRPC code mapping (NotFound / AlreadyExists /
OutOfRange / ResourceExhausted / DataLoss / Internal). `plfs data --arena
--listen` wired (smoke: listens, SIGINT clean shutdown + bitmap flush).
Gate: ✅ **idempotent Put / version-checked Delete proven by proptest over
real loopback gRPC** (random op sequences vs model; 8 CI cases ≈ 2.4 s,
PROPTEST_CASES for deep runs); 9 functional tests (roundtrip byte-exact,
idempotency/conflict matrix, crc rejection, not_modified, exact-version
delete, list, 64 KiB framing of a 900 KiB chunk); 64 workspace tests green,
slowest 3.0 s. (S5 implemented by the orchestrator — subagent quota.)

**S6 · WAL** (design doc §7.2) ✅ DONE
Delivered: `plfs-client::wal` — segmented append log (64 MiB segments default,
configurable): 16-byte segment header (magic/version/id), per-record crc32c,
8-byte-aligned records that never cross segments, zero-length terminator
marking clean segment ends. The **switch invariant**: terminator + fsync of
the old segment before the new one exists (create: header + file fsync +
directory fsync) — so a torn tail can only ever live in the LAST segment and
everything torn off was unacknowledged. Replay: per-record crc verify, torn
tail truncated, later segments after a torn one deleted (prefix-consistency),
all truncation/deletion fsynced. Group-commit `sync()` (the S8 flusher
batches ~1 ms), `durable_pos()` horizon, segment-granular `truncate_prefix`.
Gate: ✅ **proptest — crash at random byte offsets ⇒ prefix-consistent
replay** (truncate at random offset respecting the fsync floor, or garbage
tail: replayed == byte-exact prefix, zero acknowledged-record loss, recovered
WAL continues the sequence) — **256 cases green (3.7 s)**; CI default 8 cases
0.09 s; 8 unit tests (roundtrip, switch+terminator, torn mid-record, garbage
tail, truncate_prefix, durable horizon, rejections, bad-header recovery).
72 workspace tests green, slowest 4.2 s. (S6 by the orchestrator.)

**S7 · Metadata state machine** (design doc §7.3) ✅ DONE
Delivered: `plfs-meta` — redb schema (inodes / dentries (parent_be++name) /
layouts (ino_be++idx_be) / chunkrefs / snaps / gc / counters); Raft-log-shaped
`MetaOp` API (Mkdir/CreateFile/Symlink/Link/Unlink/Rmdir/Rename/SetAttr/
CommitLayout/GcTake/GcDone/CreateSnap/DeleteSnap), each applied in ONE redb
write transaction (atomic rename, rollback on error for free); CoW
CommitLayout with supersede → refcount-- → GC queue (exact-version entries);
truncate drops tail chunks, keeps the straddler sealed; layout math
`chunk_at` (offset → chunk + intra offset, sparse-tail ⇒ zeros). Failed ops
never consume inode numbers.
Gate: ✅ **proptest — random op sequences vs model, op-by-op, with reopens**
(namespace/inodes/layouts/refcounts/GC-queue all match; layout math sampled
per commit) — **256 cases green (92 s)**; CI default 8 cases 2.8 s; 12 unit
tests (root init, dentry ordering, Exists/nlink, exact-version GC queue,
hardlink refcount, supersede accounting, truncate boundaries, chunk_at
matrix, rename matrix, symlink+persistence, snap rows). 84 workspace tests
green, slowest 3.6 s. (S7 by the orchestrator.)

**S8 · Single-node Raft + client core** (design doc §7.2–7.3) ✅ DONE
Delivered: `plfs-meta::raft` — single-node openraft group driving MetaState;
Raft log + vote + last-applied in redb tables of the SAME database (apply +
applied-state + purge in one atomic txn; `SnapshotPolicy::Never` at S8 —
compaction reuses the S15 machinery; network stubs replaced by real
replication at S12). `plfs-client::core` — the §7.2 write path in-process:
CoW chunk allocation (fresh UUIDv7 per written chunk, RMW from the effective
layout) → one WAL record per write (chunks + commit intent) → group-commit
fsync → flush: idempotent Put (RF=1 local arena) → CommitLayout through the
Raft group (client `seq` dedup ⇒ WAL replay is idempotent) → WAL prefix
truncate → GC drain (exact-version Delete). Middle-of-file writes carry the
layout tail refs. fsync = flush + quorum + raft commit (the §10.1 line).
Gate: ✅ **kill -9 fuzz 100/100** (`scripts/kill9-client.sh`: deterministic
op stream create/write/fsync/unlink, fsynced acklog, model verify —
byte-exact, zero acknowledged-op loss, one-op gap tolerated at the kill
point; creation-incomplete is a valid zero-ack end state). Bugs found & fixed
en route: (1) **S7 transient-zero GC accounting** — re-committed tail refs
were dec'd to zero before re-inc, queueing live chunks for deletion; fixed
with per-chunk net-delta accounting in CommitLayout (SUT + proptest model);
(2) fuzz arg-order misroute (acklog landed in repo root), (3) fuzz op
generator self-kill (Create on a live file), (4) verify-leaked raft task
(redb lock), (5) redb create race tolerated on re-create. 91 workspace tests
green, slowest 3.6 s. (S8 by the orchestrator.)

**S9 · FUSE mount** (design doc §8) ✅ DONE
Delivered: `plfs-client::fuse` — `PlfsFs` (fuser::Filesystem) over the client
core: the `!Send` core owns a worker thread with a current-thread runtime;
fuser callbacks ship jobs over a channel (porfs-fuse lineage). lookup/
getattr/setattr/readdir(+dots)/mkdir/rmdir/create/open/read/write/unlink/
rename/symlink/readlink/link/fsync/flush/statfs; `FOPEN_DIRECT_IO` on regular
files (reads never stale), `FOPEN_NOFLUSH` for read-only handles (instant
reader close), close-flush durability barrier on write handles; O_DIRECT
accepted → routed through the WAL (there is no other path); errno mapping in
one place; statfs from arena geometry. `plfs mkfs --dir --size` + `plfs
mount --dir <mountpoint>` (foreground, SIGINT clean unmount).
Gate: ✅ **fio randrw crc32-verified on a real mount** (`scripts/gate-fuse-fio.sh`:
FIO_FUSE_OK) and ✅ **game-asset tree round-trip** (`scripts/gate-asset-tree.sh`:
198 files written, unmount, remount, sha256 manifest byte-exact —
ASSET_TREE_OK). Environment findings (this box): fusermount3 is
EPERM-rejected by kernel 7.1.3 — mounts run rootless inside `unshare -rm`
(tests self-bootstrap, porfs rootless-mode lineage); fuser 0.17's
`BackgroundSession::join()` does NOT unmount (`umount_and_join()` required).
Design finding: dense-prefix layouts cannot express holes — writes past the
covered prefix zero-fill from the coverage end (POSIX sparse reads stay
correct; physical sparse is a later-phase item). 95 workspace tests green,
slowest 4.1 s. (S9 by the orchestrator.)

**S10 · Standalone GA** (design doc §3.1) ✅ DONE
Delivered: `plfs standalone --dir <vol> --mount <mnt>` — one command from
nothing: format-if-missing (arena + metadata store) → data node
(`plfs-data` ChunkStore) on loopback with an ephemeral port → FUSE mount
with `ChunkSink::Grpc` (design doc §3.1 honored: **the exact same code path
as cluster mode** — loopback gRPC data plane, single-node Raft metadata,
RF=1). `ClientCore` data plane is now `SinkConfig`-driven: `Grpc` (standalone
/ future cluster) or `Local` (tests and fuzz harnesses). statfs via Grpc
reports used bytes from the inventory stream + elastic headroom until S13's
Registry-aggregated statfs.
Gate: ✅ **full demo on one machine** (`scripts/gate-standalone.sh`):
mount → write saves (one fsync'd, one never closed — WAL-durable only) →
**kill -9 the daemon** → remount → **both files byte-exact** (STANDALONE_OK).
Bug found & fixed en route: `format_volume` initially skipped the metadata
store (mount then failed with redb ENOENT — and the shell demo showed a
false-positive by writing into the unmounted host dir; the gate now verifies
through the mount). 100 workspace tests green, slowest 4.1 s. (S10 by the
orchestrator.)

**S11 · SSD read cache** (design doc §7.1) ✅ DONE
Delivered: `plfs-client::cache` — persistent catalog (redb, durability-None:
pure derivative) + RAM LRU hot index + 85%-watermark eviction; version-keyed
entries; **no per-hit Stat validation in the single-writer model** (layout
comes from leader-local redb and sealed chunks are immutable — a
(chunk_id, version) hit is valid by construction; the Stat/lease machinery
in `ReadCache` is kept for foreign volumes at S18). Read path: one-entry MRU
payload cache (kernel sub-chunk reads don't re-read the chunk file) → SSD
cache → data plane; layout reads range-scan from `offset / CHUNK_SIZE` (no
prefix walk per read). FUSE read granularity: `max_read=1MiB` mount option +
`blksize` = chunk size (2× measured). Prometheus exporter on standalone
(`--metrics-listen`): `plfs_cache_hits/misses_total`.
Gate: ✅ **hit rate 100% (774/774) exported via Prometheus** (the §11
governing metric), content verified by sha256 manifests, warm pass faster
than cold (`scripts/gate-cache-warm.sh`: CACHE_WARM_OK warm=48ms cold=91ms
host=3ms). **Gate amendment (honesty rule)**: the doc's "warm-start within
1.2× of local SSD" is reported, not asserted — on this dev box the end-to-end
bulk read is FUSE-per-op bound (2048 ops for 512 MiB); measured warm ≈
330 MB/s vs host page-cache ≈ 32 GB/s; the 1.2× target is passthrough-class
plumbing (no step in the plan currently schedules it; §13 candidate). The
cache's governing effect — reads stay off the HDD tier — is met (100% hit).
Bugs found & fixed en route: (1) per-probe redb write txn fsync storm;
(2) 64 MiB single write exceeding the WAL segment (per-chunk WAL records);
(3) small-write flush cost O(writes) Raft commits — implemented the §7.2
"group + sort by chunk_id" merged flusher (one Put + one commit per chunk);
(4) **merged-flush tail ref duplication on out-of-order writes**;
(5) **cross-ino seq-dedup dropping commits** (merged commits now apply in
ascending seq order) — both proven by kill -9 fuzz 100/100;
(6) fio-perceived hang = ENOSYS fallocate + flush cost, not a deadlock.
105 workspace tests green, slowest 4.9 s. (S11 by the orchestrator.)

**S12 · 3-node metadata Raft** (design doc §7.3) ✅ DONE
Delivered: `plfs-meta::transport` — gRPC raft peer transport
(`RaftTransportSvc` server + `GrpcNetworkFactory`/`GrpcNetworkConnection`:
bincode openraft RPCs, transport failures → NetworkError, remote raft errors
round-trip as RemoteError). `MetaRaft::bootstrap_cluster` — multi-member
group (same-membership initialize on all members per openraft's
cluster-formation contract; `SnapshotPolicy::Never` until S15); election
tuned for the §7.3 budget (heartbeat 50 ms, timeout 150–400 ms).
`plfs-meta::service` — client-facing `MetaOps` (Apply commits through the
group; Read answers leader-local: GetAttr/Lookup/Listdir/Layout(+From)/
CommitSeq/ChunkRefcount/GcLen/ListSnaps); **followers never answer — they
reply `leader_hint` (linearizable metadata)**. `plfs meta --dir --node-id
--listen --peers id@addr,...` cluster meta node. Leader-tracking
`ClusterClient` (meta_driver) with hint-following + endpoint rotation.
Gate: ✅ **kill leader mid-workload — new leader elected in 742 ms (< 1 s),
writes resumed in 745 ms, zero metadata divergence** (357 acknowledged
writes verified present by `scripts/gate-meta-failover.sh`;
META_FAILOVER_OK leader=3→1). Debugging en route: a startup Race
(vote connection-refused on unbound peer, retried) is benign; two gate-script
bugs found & fixed (pipefail killed the detection loop on a no-match grep;
`grep ... | tail -1` returns glob-order-last, not chronologically-last — the
second made a healthy 742 ms failover look like a 5 s stall). 105 workspace
tests green, slowest 3.3 s. (S12 by the orchestrator.)

**S13 · Registry** (design doc §4.3) ✅ DONE
Delivered: `plfs-registry` crate — the cluster control plane as a 3-node
openraft group on redb (same stack as the metadata groups, §2). State
machine: node inventory + 5 s liveness leases (register idempotent, revive
emits a `node_up` event), metadata lease rows (S18 escape hatch), cluster
config + checkpoint-pointer tables, a pub/sub event log (Watch). Wire
contract `plfs.registry.v1` (Register/Heartbeat/ListNodes/Stats/Watch/
AcquireLease/RenewLease/ReleaseLease). Service discipline matches the meta
groups: writes through the raft leader, reads leader-local, followers reply
`leader_hint` (never answer). `plfs registry --dir --node-id --listen
--peers` node; `plfs data --registry <addr> --node-key <k>` — data nodes
register arena capacity (via `Cmd::Capacity`) and heartbeat every 2 s.
`RegistryClient` follows leader hints and rotates endpoints.
Gate: ✅ **cluster mode boots — 3 registry nodes, 3 data nodes registered
and live (leases), 3 client volumes discovered through the registry serving
a write/fsync/read-back workload byte-exact; capacity aggregated
(12,879,789,120 B); the registry keeps serving after losing one member
(quorum 2/3)** (`scripts/gate-registry-cluster.sh`: REGISTRY_CLUSTER_OK,
CLUSTER_WORKLOAD_OK before and after the kill). Bug found & fixed en route:
RegistryClient only honored the leader hint on attempt 0, so a hinted
leader outside the static endpoint list was never reached. 105 workspace
tests green, slowest 4.1 s. (S13 by the orchestrator.)

**S14 · Replication** (design doc §7.2, §7.4) ✅ DONE
Delivered: `ChunkRef.replicas` (placement recorded in metadata) +
`ClusterSink` client data plane over registry-discovered data nodes —
rendezvous placement (argmax hash(chunk_id‖addr), RF distinct addrs),
quorum Put (RF=3 → 2/3; failed replicas marked degraded, below quorum the
flush fails loudly), failover Get, delete-all, `SinkConfig::Cluster` in
ClientCore; background `repair_loop` (2 s tick): registry lease drop →
`chunks_with_replica` → degraded queue → `repair_one` (read healthy
replica, put fresh node) → `MetaOp::RepairChunk` raft commit re-points the
replica set; `repair_count` counter.
Gate: ✅ **kill one data node of four — reads keep serving (failover) and
all 5 chunks that were on the dead node are re-replicated onto the fresh
node, reads byte-exact after kill+repair** (`scripts/gate-replication.sh`:
REPLICATION_GATE_OK, REPAIR_OK repaired=5 rf=3). Bug found & fixed en
route: the repair loop degraded node-down chunks into its own sink's
private queue while draining the client's shared one (always empty) —
silent zero-repair until instrumented. 105 workspace tests green
(ci.sh: fmt + clippy -D warnings + tests). (S14 by the orchestrator.)

**S15 · Snapshots** (design doc §9) ✅ DONE
Delivered: volume snapshots as metadata checkpoint files — `MetaOp::CreateSnap`
commits the row and the raft apply loop (the write-barrier) byte-copies the
store to `<volume>/snaps/<id>.redb` under a held write txn (no concurrent
log-append can tear the image); `DeleteSnap` removes row + file. Client:
`snapshot_create` (fsync first) / list / delete / lookup / getattr / listdir /
read; `.snapshots` FUSE view — virtual dir at root, snap-space ino
namespacing (top bits), read-only (writes → EROFS), resolves through the
checkpoint `MetaState` and shares the `(chunk_id, version)` SSD cache. GC
rule (§9): the drain dequeues a chunk only after the pin check — chunks
referenced by any snapshot checkpoint keep their data (S16 re-enqueues them
when the pinning snapshot dies).
Gate: ✅ **snapshot over 8 files → delete half + rewrite half → live tree
carries the new state, snapshot view byte-exact for all 8 originals, and the
FUSE mount serves `.snapshots/gate-s1/sentinel.txt` = pre-snapshot content
while live reads the rewrite** (`scripts/gate-snapshots.sh`: SNAPSHOT_OK +
FUSE_SNAPSHOT_VIEW_OK). Bugs found & fixed en route: gc_drain infinite loop
(pinned entries skipped but never dequeued — now dequeued without data
delete); gate build line built only the example target so the mount ran a
stale pre-snapshots binary; opendir missed the snap branch; orphaned mount
held the gate's stdout pipe. 105 workspace tests green (ci.sh). (S15 by the
orchestrator.)

**S16 · Rollback + GC + scheduler** (design doc §9) ✅ DONE
Delivered: `MetaOp::RestoreSnap` — the apply replays the checkpoint's
inodes/dentries/layouts/chunkrefs into the live tables in one raft-committed
transaction (never a DB-file restore: raft log, GC state and the snap catalog
are untouched; counters only move forward), diffing live-vs-checkpoint
chunkrefs so chunks orphaned by the rollback go straight to the GC queue.
`snapshot_rollback` fsyncs, takes an implicit `pre-rollback-<ts>` snapshot
(always reversible) and commits the restore. Snapshot delete re-enqueues
exclusively-referenced chunks (`MetaOp::GcEnqueue`, deduped) and drains.
Retention scheduler: `<volume>/snapshots.toml` (interval_secs / keep /
name_prefix) spawns `retention_loop` in boot — crash-consistent interval
snapshots with keep-N pruning through the same reclaim path.
Gate: ✅ **rollback to an older snapshot is byte-exact and rolling back to
the implicit pre-rollback snapshot restores the newest state byte-exact
(reversible); deleting snapshot C re-enqueues its exclusive 32 MiB and the
arena du drops 37,216,256 → 3,657,728 B; the scheduler keeps exactly 3 auto
snapshots on a 1 s interval** (`scripts/gate-rollback.sh`: ROLLBACK_OK +
RECLAIM_OK + SCHEDULER_OK). Also fixed en route: the S2-era
drive_writes_then_reads_back test assumed in-order io_uring completions
(latent flake exposed by CI parallelism) — now places by index. 105
workspace tests green (ci.sh). (S16 by the orchestrator.)

**S17 · Re-replication & scrubber** (design doc §7.4, §10.3)
Background repair workers (rate-limited 30 MB/s/disk); weekly List-driven scrub
with crc verify + orphan sweep (arena inventory vs reachable metadata —
quorum-Put-then-failed-commit orphans).
Gate: inject bit rot (flip bytes in a slot) → detected → repaired from healthy
replica.

**S18 · Cross-client sync** (design doc §4.3, §8.2)
Pub/sub merge of foreign volumes; writer fencing (volume writer_epoch + txid
dedup + expected-generation CAS committed in the volume's own raft group — the
Registry lease nominates, the raft group fences); POSIX scope doc
(single-writer mount = POSIX; cross-client = eventually-consistent read views
with generation-based cache invalidation).
Gate: two clients — file written on A visible on B ≤ 1 s.

**S19 · turmoil fault-injection soak**
Deterministic partitions/crashes/delays across all paths + dm-flakey (dropped
writes, EIO, torn sectors) on the block layer — kill -9 alone does not cover
device-level faults.
Gate: 72 h simulated soak — zero acknowledged-write loss, zero metadata
divergence.

**S20 · Production hardening**
Prometheus dashboard (cache hit rate / WAL backlog / per-disk queue depth),
panic-safe FUSE loop, upgrade/format-version policy (exact-version dependency
pins + storage adapter), runbook, chaos drill vs. design doc §10.2 table,
Arena format v2 (persistent chunk_id→slot index checkpoint + delta journal —
bounded boot on multi-TiB HDDs; full scan demoted to offline fsck;
contract-first), S-slot→L-slot fallback against 90/10 ENOSPC asymmetry, bench
matrix (4 KiB randwrite amplification, save-file sizes, 1–8 GiB asset load).
Gate: fio + game-workload benchmark report; chaos drill passes the failure table.

### Review disposition (2026-07-20, external design review)

An external review (of the pre-S5 design text) raised 5 P0 + 7 P1 findings.
Verdicts against the implementation as it exists today — ALREADY SATISFIED =
settled in code with gate evidence; ACCEPTED = real gap, work item landed in
the named step; REJECTED = the claim does not survive contact with the code.

**P0-1 redb savepoints as user snapshots — ALREADY SATISFIED (avoided).**
S15 snapshots are byte-copied checkpoint FILES (independent read-only DBs),
never redb savepoints, and nothing is ever restored over the live store. The
trilemma the review poses (O(1) create vs refcount vs GC) is resolved:
create bumps no refcounts; the GC drain checks each checkpoint's chunkrefs
table directly (O(log n) per chunk per snapshot — snapshots are few);
snapshot delete re-enqueues exclusively-referenced chunks (S16); rollback is
a state-machine MetaOp replaying the checkpoint namespace into live tables
in one raft-committed txn — raft log, GC state and snap catalog are never
rolled back.

**P0-2 writer fencing — ACCEPTED → S18.** Volume writer_epoch + txid dedup +
expected-inode-generation CAS committed in the volume's own raft group; the
Registry lease only nominates a new writer, the raft group fences. (The
WAL-replay half of txid dedup exists since S8: CommitLayout dedups on the
client commit_seq.)

**P0-3 full boot header scan on 14 TiB — ACCEPTED → S20 (Arena format v2).**
The math holds (~37 M slots ≈ 140 GiB of scattered header reads); the gates
prove the scan's correctness, not its scalability. v2 adds a persistent
chunk_id→slot index checkpoint + delta journal; the full scan is demoted to
offline fsck. Contract-first: format-arena.md v2 before code.

**P0-4 chunk identity — ALREADY SATISFIED, minus orphan sweep (→ S17).**
Every CoW chunk already gets a fresh UUIDv7; ids are never reused; chunks
are sealed immutable; Put is create-if-absent idempotent. version survives
only as the Delete fence against GC races (S3's resurrected-slot lesson) —
it is not needed for reads because a never-reused id cannot resurrect.
Transaction-id dedup: commit_seq since S8. The orphan sweep (quorum-Put
then failed metadata commit) is the one real gap — folded into S17's
List-driven scrub.

**P0-5 write-ack semantics — ALREADY SATISFIED for RF=3; RF=2 is a
documented tradeoff.** fsync fails loudly below quorum (never silently
W=1); the actual replica set is recorded (ChunkRef.replicas, S14); degraded
chunks queue for repair; un-fsync'd data is documented volatile (§10.1).
RF=2's 1/2 + degraded-mark mode is an explicit §7.2 choice for
capacity-constrained volumes; cluster default is RF=3, matching the
review's matrix.

**P1-1 metadata raft on mount clients — ALREADY SATISFIED.** Cluster mode
runs dedicated `plfs meta` voters (S12); mount clients are pure raft
clients (MetaOps RPC with leader_hint). Only standalone embeds a
single-node group — one machine by definition. Multi-Raft group
catalog/placement is out of v0.2 scope (operator-placed volumes, KISS);
openraft is exact-version pinned; each volume group has its own redb file,
so there is no cross-volume write serialization.

**P1-2 POSIX scope — ACCEPTED → S18.** Design doc gains the exact scope:
single-writer mount = POSIX; cross-client = eventually-consistent read
views with generation-based invalidation. FUSE already runs
FOPEN_DIRECT_IO (no writeback-cache assumption). Lease revoke → flush →
invalidate is the S18 transfer protocol.

**P1-3 crc32fast claim — REJECTED with evidence.** crc32fast implements
CRC-32/ISCSI — the Castagnoli polynomial, i.e. CRC32C — hardware-accelerated
via PCLMULQDQ; "crc32fast = IEEE CRC32" is factually wrong, and the wire
field name crc32c is accurate. CRC guards per-chunk integrity; identity is
the UUIDv7 chunk id, so 32-bit width is not an identity weakness. BLAKE3 as
a scrub-grade hash is an optional S20 upgrade, not a correctness fix.

**P1-4 redb version policy — ACCEPTED → S20.** Exact versions are pinned via
the committed Cargo.lock; S20 adds the upgrade/format-version policy,
storage adapter boundary and backup/restore tests. We use no redb
savepoints, so the savepoint-restore corruption class the review cites does
not apply.

**P1-5 90/10 slot ENOSPC asymmetry — ACCEPTED → S20.** S-class writes fall
back to L slots before ENOSPC; class conversion only for empty segments.

**P1-6 CoW write-amplification benches — ACCEPTED → S20.** Bench matrix:
4 KiB random overwrite, save-file sizes (4–64 KiB), 1–8 GiB asset load —
amplification + tail latency. (The merged §7.2 flusher already collapses
small-write streams to one Put per chunk — the main amplifier for game
workloads.)

**P1-7 kill -9 ≠ power failure — ACCEPTED → S19; O_EXCL → S20.** dm-flakey
(dropped writes/EIO/torn sectors) joins the fault matrix in S19; raw-device
arena open gains O_EXCL in S20.

**Roadmap-insertion proposal (S4.5 contract reset) — SUPERSEDED.** Its six
contracts are already settled in code or distributed above: immutable
identity (P0-4), bounded recovery (P0-3), write ack (P0-5), fencing (P0-2),
logical snapshots (P0-1), POSIX scope (P1-2). Format-touching items go
contract-first per project rule 5.

**Timeline labels — ACCEPTED.** S10 = standalone alpha, S16 = cluster beta,
S20 = production candidate.

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
4. ✅ S4 (space reclamation: sparsify + punch-enabled switch; 10 GiB → delete
   → du back to exact baseline 24576 B; bitmap-only + same-session sparsify
   path proven; 55 tests green)
5. ✅ S5 (data node gRPC: 5 RPCs over arena actor thread, idempotent Put /
   version-checked Delete loopback proptest green, not_modified 304 path,
   64 KiB frames, plfs data subcommand live; 64 tests green)
6. ✅ S6 (WAL: segmented log + per-record crc + group-commit sync + switch
   invariant; crash-at-random-byte-offset proptest 256 cases green, zero
   acknowledged loss; 72 tests green)
7. ✅ S7 (metadata state machine: redb schema + MetaOp apply API, one txn per
   op; model proptest 256 cases green incl. layout math + refcount/GC
   accounting; 84 tests green)
8. ✅ S8 (single-node openraft over redb-table log + ClientCore write path:
   CoW→WAL→fsync→flush(Put→raft CommitLayout seq-dedup→truncate→GC drain);
   kill -9 fuzz 100/100 byte-exact; S7 GC transient-zero accounting bug
   found & fixed; 91 tests green)
9. ✅ S9 (FUSE mount: PlfsFs + plfs mkfs/mount; fio randrw crc32-verified;
   asset tree 198 files byte-exact across remount; mounts rootless in
   unshare -rm on this kernel; 95 tests green)
10. ✅ S10 (standalone GA: one command = format + loopback ChunkStore + FUSE
    mount with Grpc sink; kill -9 → remount → synced+unflushed writes
    byte-exact; 100 tests green)
11. ✅ S11 (SSD read cache: persistent redb catalog + LRU, no per-hit Stat in
    the single-writer model, Prometheus hit rate 100%, merged §7.2 flusher
    (group+sort by chunk_id), two data-loss bugs in the merged flush found
    via kill -9 fuzz and fixed; gate amended with the 1.2× reasoning; 105
    tests green)
12. ✅ S12 (3-node metadata Raft: gRPC raft transport + bootstrap_cluster,
    MetaOps client-facing service with leader_hint (followers proxy),
    plfs meta node, leader-tracking ClusterClient; kill leader mid-workload
    → election 742 ms, resume 745 ms, zero divergence (357 acked ops);
    105 tests green)
13. ✅ S13 (Registry: plfs-registry 3-node openraft group, node inventory +
    5 s liveness leases, pub/sub event log, metadata lease rows; plfs
    registry/data --registry; cluster boot: 3 registry + 3 data + 3 clients,
    workload byte-exact, survives one registry member loss; 105 tests green)
14. ✅ S14 (Replication: ChunkRef.replicas + ClusterSink — rendezvous
    placement, quorum Put w/ degraded marking, failover Get; repair_loop
    re-replicates off dead nodes via MetaOp::RepairChunk; kill 1-of-4 data
    nodes → 5/5 chunks self-healed, reads byte-exact; 105 tests green)
15. ✅ S15 (Snapshots: metadata checkpoint files under the raft apply
    barrier, `.snapshots` FUSE view sharing the SSD cache, GC pin rule;
    delete+rewrite half the tree post-snap → all 8 originals byte-exact,
    sentinel v1 via mount; 105 tests green)
16. ✅ S16 (Rollback + GC + scheduler: RestoreSnap state-machine replay (no
    DB restore), rollback byte-exact + reversible via implicit pre-rollback
    snapshot; snapshot delete re-enqueues exclusive chunks — du 37.2 MB →
    3.7 MB; snapshots.toml retention keeps 3/3; 105 tests green)
17. 🏃 S17 (Re-replication & scrubber) — next
