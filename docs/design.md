# GameFS — Design Document v0.2

**A KISS distributed file system for game server clusters**
2026-07-19 · Implementation language: Rust · License: TBD

Changelog v0.1 → v0.2:
- Metadata is now **Raft-replicated per volume** (3-node groups); local DB is the leader's state machine, not a single point of failure.
- Data nodes no longer use ext4: new **ChunkArena** storage layer over raw block devices or **sparse `.img` files**, with hole-punching so deleted chunks return space to the host filesystem.
- Added **standalone mode**: single process running client+server on one machine works out of the box (RF=1).
- Full crate survey — we prefer mature libraries over hand-rolling.
- Implementation plan broken into **20 steps**.

---

## 1. Goals and Non-Goals

### 1.1 Workload model

GameFS serves a cluster of game servers:

| Workload | Pattern | Requirement |
|---|---|---|
| Game assets / configs | Bulk reads at server start, long-lived hot set | High random-read IOPS, aggressive local caching |
| Player saves | Small files (KBs), random writes, **single writer** | Durable writes, instant snapshots/rollback |
| Logs | Append-only | Sequential throughput |
| Cross-node sharing | Rare | Eventual consistency acceptable |

**Key insight:** game workloads are naturally *single-writer*. No distributed write locks are needed — the foundation of the KISS design.

### 1.2 Goals

- Aggregate capacity of **all storage machines (HDD)** as the durable tier.
- Client-local **SSD** as persistent read cache + WAL.
- **Raft-replicated metadata**: no metadata loss on any single-machine failure.
- **Instant O(1) volume snapshots** with rollback.
- **POSIX semantics** via FUSE for game-server workloads (§8).
- Runs anywhere from **one machine (standalone)** to tens of machines (cluster).
- No dependence on a host filesystem for data storage: raw block device or sparse `.img` back-ends, with **automatic space reclamation** (hole punching).
- Library-first implementation; one binary, three roles. Target: **~10k lines of our own Rust**.

### 1.3 Non-goals (KISS boundary)

- ❌ Multi-writer cache coherence / byte-range locks (single-writer assumption; Registry lease as escape hatch).
- ❌ Erasure coding (RF 2/3 on cheap HDD is enough).
- ❌ Striping logic (parallel chunk fetch gives aggregate bandwidth for free).
- ❌ Strongly consistent cross-client namespace (pub/sub merge, ~1 s).
- ❌ RDMA / kernel bypass. TCP is not the bottleneck at this scale.
- ❌ Directory-level snapshots.

---

## 2. Crate Survey (build vs. reuse)

**Policy: reuse mature crates everywhere; hand-roll only what is our core value** (ChunkArena, WAL, cache, FUSE semantics).

| Concern | Crate | Version note | Why |
|---|---|---|---|
| Async runtime | `tokio` | 1.x, features `full` | ecosystem default, `tokio::fs`, process, signal |
| RPC | `tonic` + `prost` | 0.12+ | gRPC streaming for `Get`/`List`, codegen, backpressure, TLS via `tonic::transport` later |
| FUSE | `fuser` | 0.14+ (FUSE3) | actively maintained Rust FUSE bindings |
| Metadata state machine | `redb` | 2.x | pure-Rust embedded ACID KV, single file, savepoint API → trivial checkpoints |
| Consensus | `openraft` | 0.9+ | mature async Raft, multi-raft (one group per volume), pluggable storage/network |
| Serialization (meta) | `serde` + `bincode` | 1.x / 1.3 | compact, versioned via enum discipline |
| CRC | `crc32fast` | 1.x | SSE4.2 hardware CRC32C |
| IDs | `uuid` v7 | 1.x, feature `v7` | time-ordered chunk ids → HDD-friendly allocation locality |
| WAL | hand-rolled | ~400 LoC | append-only segmented log + batch fsync; no crate fits exactly (see note) |
| Read cache index | `lru` + `redb` | — | LRU in RAM for hot index, redb for persistent cache catalog |
| Byte buffers | `bytes` | 1.x | zero-copy chunk payload plumbing |
| Concurrency utils | `parking_lot`, `tokio-util` (CancellationToken), `async-trait` | — | |
| Sparse file / discard | `nix` (`fallocate`, `FALLOC_FL_PUNCH_HOLE`), `libc::ioctl(BLKDISCARD)` | — | §6.4 space reclamation |
| Registry store | `openraft` + `redb` backend | — | same stack as metadata groups |
| CLI | `clap` 4 (derive) | — | `gamefs mkfs / mount / snapshot ...` |
| Config | `serde` + `toml` | — | |
| Observability | `tracing` + `tracing-subscriber`, `metrics` + `metrics-exporter-prometheus` | — | three-graph dashboard: cache hit, WAL depth, HDD queue |
| Error handling | `thiserror` (lib) / `anyhow` (bin) | — | |
| Fault-injection tests | `turmoil` | 0.6+ | deterministic simulated cluster: partitions, crashes, clock control |
| Property tests | `proptest` | 1.x | chunk allocator, WAL replay, layout math |
| Benchmarks | `criterion` + `fio` (external) | — | |

**Explicitly rejected:**
- `rocksdb` — C++ build chain, operational weight; redb suffices at our metadata scale (≤ GBs).
- `raft-rs` (tikv) — more battle-tested but harder to embed; `openraft`'s multi-group ergonomics win for per-volume groups.
- `seaweedfs`-style HTTP data plane — gRPC gives us streaming + typed contracts for free.
- Writing our own FUSE protocol layer — `fuser` covers FUSE3.

---

## 3. Deployment Modes

One binary, three roles, composable:

```
gamefs mkfs   --device /dev/sdb            # or: --img /var/lib/gamefs/disk0.img --size 14T
gamefs data   --arena /dev/sdb             # chunk server role
gamefs meta   --volume save --peers ...    # metadata raft group member
gamefs mount  --volume save /mnt/save      # FUSE client role
gamefs registry --peers ...                # cluster control (3 nodes)
```

### 3.1 Standalone mode (single client + single server)

```
gamefs standalone --dir /var/lib/gamefs --mount /mnt/gamefs
```

- Runs registry (single-node Raft), one metadata group (single-node Raft), one arena (sparse img), WAL, cache, FUSE — all in one process.
- **RF = 1** (documented: durability = local disk only; snapshots still work).
- Uses the **exact same code paths** as cluster mode: single-node Raft groups, loopback gRPC. No special-case forks — this is our dev/test/prod-unified topology, and the CI default.
- Later scale-out: `gamefs registry join`, add arenas, raise volume RF — metadata migrates via Raft membership change, no downtime design (membership change is native to openraft).

### 3.2 Cluster mode

```
┌─────────────┐  ┌─────────────┐
│  Client A   │  │  Client B   │    game servers (FUSE + cache + WAL + meta leader for owned volumes)
└──────┬──────┘  └──────┬──────┘
       └────────┬───────┘
          TCP gRPC (10/25GbE)
   ┌────────┬───┴────┬────────┐
┌──▼───┐ ┌──▼───┐ ┌──▼───┐   …    data nodes: ChunkArena on raw HDD
└──────┘ └──────┘ └──────┘
        ┌──────────────┐
        │ Registry ×3  │         openraft, KBs of data, never on I/O path
        └──────────────┘
Metadata Raft groups (3 members each) are colocated: group members run on any 3 machines (data nodes may double as meta followers; meta leaders preferably on the owning game server for locality).
```

---

## 4. Architecture & Components

### 4.1 Client (the smart component)

On every game server:

- **FUSE daemon** (`fuser`) — POSIX surface (§8).
- **Metadata Raft leader** for owned volumes — state machine in `redb`.
- **SSD read cache** — persistent, chunk-granular, LRU, version-validated (§7.1).
- **WAL** — write buffering with batch fsync (§7.2).
- Background workers: flusher, re-replication, GC, snapshot scheduler, scrubber client, cache eviction.

### 4.2 Data node (deliberately dumb)

- Hosts **ChunkArena** instances (one per disk/device).
- 5 RPCs (§5). Knows nothing about files, replicas, consistency, snapshots.
- May also host metadata Raft followers (cheap, KB-scale traffic).

### 4.3 Registry

- 3-node `openraft` group (single-node in standalone).
- Node inventory + liveness leases; cluster config; latest metadata-checkpoint pointers; pub/sub for cross-client metadata merge; metadata lease issuer (escape hatch for cross-client writes).
- **No file metadata.** Registry loss pauses membership changes only.

---

## 5. Data Plane Protocol (tonic/gRPC)

```proto
syntax = "proto3";
package gamefs.data.v1;

service ChunkStore {
  rpc Put(PutRequest)        returns (PutResponse);    // idempotent on (chunk_id, version, crc)
  rpc Get(GetRequest)        returns (stream GetReply);// 64KiB frames; version + crc in first frame
  rpc Stat(StatRequest)      returns (StatReply);      // version/len/crc only; cache validation
  rpc Delete(DeleteRequest)  returns (DeleteReply);    // exact-version, idempotent; triggers hole punch
  rpc List(ListRequest)      returns (stream ListReply); // arena inventory for scrub/repair
}

message PutRequest  { bytes chunk_id = 1; uint64 version = 2; uint32 crc32c = 3;
                      bytes payload = 4; bool seal = 5; }
message GetRequest  { bytes chunk_id = 1; uint64 if_version = 2; } // 0 = any
message DeleteRequest { bytes chunk_id = 1; uint64 version = 2; }
```

Design notes:
- `Put` idempotency (same id+version+crc ⇒ no-op success) makes client retry trivial.
- `Delete` is version-checked: protects against GC racing a re-Put.
- `Get` with `if_version` == current returns `NOT_MODIFIED`-style reply — the 304 semantic behind cache validation (§7.1).
- Max message 1 MiB + header; 64 KiB stream frames keep memory flat.
- Auth (v1): shared-token metadata on an isolated LAN; mTLS via `tonic::transport::ServerTlsConfig` in v2.

---

## 6. ChunkArena — Storage Layer over Raw Devices / Sparse Images

Replaces any host filesystem on data nodes. Core value component, ~1,200 LoC.

### 6.1 Layout

```
┌──────────────────┬──────────────────┬─────────────────────────────┬─────────────┐
│ Superblock A(4K) │ Superblock B(4K) │ Bitmap region               │ Slot region │
│   (primary)      │   (mirror)       │ (2 copies + crc, fixed pos) │ (N × 1 MiB) │
└──────────────────┴──────────────────┴─────────────────────────────┴─────────────┘
Superblock: magic "GFARENA1", format_version, slot_size, small_slot_size,
            slot_counts, bitmap_offsets, arena_uuid, created_at, crc32c
```

Two slot classes, partitioned at `mkfs` (default 90/10):
- **L-slots**: 1 MiB — normal chunks.
- **S-slots**: 64 KiB — inline chunks for files < 64 KiB (saves, configs).

### 6.2 Slot header (first 64 B of each slot)

```
magic u32 "GFS1" | header_version u16 | flags u16 (SEALED=1)
chunk_id [16]B (UUIDv7) | version u64 | payload_len u64
payload_crc32c u32 | header_crc32c u32
```

**The slot header is the source of truth.** The bitmap is a hint, rebuilt/reconciled at startup:
- Slot with valid header + SEALED ⇒ allocated.
- Slot with valid header, not SEALED ⇒ interrupted Put ⇒ treat as free (idempotent re-Put will rewrite).
- Bitmap says used, header invalid ⇒ bitmap bit cleared (crash between pwrite and bitmap update).

### 6.3 Operations

| Op | Implementation |
|---|---|
| `Put` | find free slot (bitmap scan, random offset start) → `pwrite(payload)` → `pwrite(header with SEALED)` → `fdatasync` → set bitmap bit (async flush ok; reconciled at boot) |
| `Get` | in-memory index `chunk_id → slot_no` → single `pread` |
| `Delete` | clear in-memory index → clear bitmap bit → **punch hole / discard** (§6.4) |
| startup | read superblock (fallback mirror) → scan slot headers (sequential read, ~seconds per TB on HDD) → rebuild index + reconcile bitmap |

Multi-disk machine: one arena + one gRPC service instance per device, announced to Registry as separate capacity units.

### 6.4 Sparse images & space reclamation

**Sparse `.img`:** `mkfs --img disk.img --size 14T` creates with `fallocate(len=0)` semantics (sparse; actual allocation only on write). Superblock+bitmap occupy a few MB; the rest is holes.

**Automatic release on delete** — when a slot is freed:
- Sparse img backend: `fallocate(fd, FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE, offset, slot_size)` — the host fs (ext4/xfs/btrfs) immediately returns those blocks; `du` shrinks.
- Raw block device backend: `BLKDISCARD` ioctl — returns the range to thin-provisioned LVM/SSD firmware; harmless no-op on thick HDDs.
- Backend capability is probed at `mkfs` and recorded in the superblock (`PUNCH_OK`, `DISCARD_OK` flags); if hole punching is unsupported (e.g. img on a fs without punch support), Delete falls back to bitmap-only and a monthly `sparsify` maintenance pass re-punches free ranges.

Guarantee: **file unlink → chunk refcount → 0 → GC Delete RPC → hole punched**. Space returns to the host within GC latency (default ≤ 60 s, rate-limited).

### 6.5 Crash consistency

- Interrupted Put ⇒ un-SEALED slot ⇒ freed at boot; the client's idempotent re-Put repairs.
- Bitmap corruption ⇒ rebuilt from headers (headers are truth).
- Torn header write ⇒ crc mismatch ⇒ slot free. Payload is never trusted without a valid SEALED header.
- No journal needed: allocation is single-slot atomic publish (header last), mirroring the tmp+rename trick — but without a filesystem.

---

## 7. Client Subsystems

### 7.1 Read path & persistent SSD cache

```
read(ino, off, len):
  layout from local meta (µs)
  per chunk:
    RAM hit → return
    SSD cache hit:
        Stat(chunk_id, if_version=cached_version)     // skipped if advisory read-lease (5s) valid
        NOT_MODIFIED → serve from SSD (~100µs)
        else → fall through
    miss → Get from lowest-RTT replica → verify crc → insert SSD cache → serve
```

- Cache entry: `(chunk_id, version) → bytes` on SSD; catalog in redb; RAM `lru` for the hot index.
- Eviction: LRU at 85% capacity watermark; eviction = delete cache file. Cache is a **pure derivative** — any entry can be dropped at any time, zero consistency machinery.
- **Version validation instead of invalidation push**: correctness reduces to one cheap `Stat`. Read leases (5 s, advisory, Data-node-granted) suppress even that for hot chunks.
- Readahead: `open()` prefetches subsequent chunks asynchronously; sequential game-asset loading becomes near-local after first pass.

### 7.2 Write path & WAL

```
write(ino, off, buf):
  CoW: sealed chunks are never modified in place;
       allocate new chunk (or fill open chunk), apply buf
  append to WAL segment; fsync (batched, group-commit ~1ms) → return

flusher (every 1s or 8MiB):
  group + sort by chunk_id (UUIDv7 ≈ creation order → HDD-friendly locality)
  Put to RF replicas concurrently; await quorum (RF=2: 2/2 or 1/2+degraded mark; RF=3: 2/3)
  commit metadata mutation through the volume's Raft group
  truncate WAL prefix; superseded chunks → GC queue
```

- WAL: segmented append log (64 MiB segments), hand-rolled, crc per record, replayed at startup.
- `fsync(fd)` / `O_SYNC` files: force flush of that file's dirty chunks and **wait for replica quorum** before returning — this is the durability line (§10.1).

### 7.3 Metadata Raft (per volume)

- One `openraft` group per volume, 3 members (1 in standalone). Leader colocated with the owning game server.
- All metadata mutations (create/unlink/rename/layout-commit/snapshot ops/refcount) are Raft log entries applied to `redb`.
- Log size is tiny (metadata ops only; data bypasses Raft entirely).
- Raft log compaction reuses the snapshot machinery (§9): metadata checkpoint = redb savepoint = Raft snapshot. One mechanism, three uses.
- Follower reads: proxy to leader (default) — keeps semantics linearizable for metadata; stale follower reads offered later as an optimization flag.
- Failover: leader loss ⇒ election < 1 s ⇒ new leader applies log tail ⇒ volume available on another machine. Registry records current leader per volume.

### 7.4 Re-replication & repair

- Degraded sources: quorum shortfall, `Stat` version mismatch, node-down broadcast, scrubber findings.
- Client-local degraded queue → background repair: read healthy replica → `Put` to a fresh node → `Delete` stale copy. Rate-limited (30 MB/s/disk default), parallel across all clients by construction (each owns its namespace).

---

## 8. POSIX Semantics via FUSE (`fuser`, FUSE3)

### 8.1 Fully supported

open/read/write/close · create/unlink/mkdir/rmdir · rename (same volume, atomic via Raft txn) · fsync/fdatasync (real quorum durability) · truncate · symlink/hardlink · chmod/chown/utimens · statfs (Registry-aggregated) · xattr `user.*` (≤4 KiB, inline in metadata) · flock/fcntl advisory locks (**local scope, documented**).

### 8.2 Documented degradations

| Operation | Behavior |
|---|---|
| rename across volumes | `EXDEV` |
| mmap(WRITE) | `ENODEV`; mmap(READ) supported |
| atime | relatime semantics |
| O_DIRECT | accepted → routed through WAL (durability never bypassed); documented |
| cross-client write to same file | serialized via Registry metadata lease; last-writer-wins otherwise (out of scope) |

POSIX scope (v0.2 ruling): a volume's **single-writer mount** provides the
§8.1 semantics. Cross-client access is an eventually-consistent **read-only
view**: foreign readers resolve the owner through the registry and read
committed state via the owner's MetaOps endpoint (≤ 1 s in practice); no
multi-mount POSIX coherence is promised. The registry lease nominates a new
writer, but fencing is committed in the volume's own raft group
(`MetaOp::ClaimWriter` writer epoch); CAS enforcement on every mutating op
lands with the shared-meta architecture.

### 8.3 Mount layout

```
/mnt/gamefs/
  save/    assets/    logs/
  .snapshots/<volume>/<name>/...    # read-only snapshot views
```

---

## 9. Snapshots

- **Volume-level, read-only, crash-consistent**; game side calls `fsync` before snapshot for app-level consistency.
- **Create** (O(1), <1 s): flush WAL → metadata write-barrier (~100 ms) → redb savepoint + file copy → row insert → release barrier → background upload of checkpoint to arenas (disaster-recovery copy).
- **Read**: `.snapshots` prefix resolves through the checkpoint DB; **shares the SSD cache** (same `(chunk_id, version)`), so snapshot reads of unchanged data are near-free.
- **Rollback**: implicit `pre-rollback-<ts>` snapshot first (always reversible) → atomic pointer swap → diff layouts → orphaned chunks to GC.
- **Delete**: remove row + checkpoint copies; exclusively-referenced chunks become collectable.
- **GC** (rate-limited, idempotent): skip if `refcount>0`, or referenced by any snapshot, or open; else version-checked `Delete` on replicas → hole punch frees space (§6.4).
- **Scheduler** (toml): e.g. `save`: hourly×48 + daily×30; `assets`: manual via deploy pipeline.

---

## 10. Durability & Failure Model

### 10.1 Contract

| Event | un-`fsync`'d writes | `fsync`'d writes | metadata |
|---|---|---|---|
| game process crash | safe (WAL) | safe | safe |
| client daemon crash | safe (WAL replay) | safe | safe |
| client machine lost | **lost** (local WAL only) | safe (quorum) | **safe (Raft quorum)** ← v0.2 fix |
| one data node lost | safe | safe | safe |
| RF data nodes lost | lost | lost | safe if ≤1 meta member lost |
| 2 of 3 meta members lost | — | — | unavailable until restore from arena checkpoint |

### 10.2 Failure handling table

| Failure | Detection | Reaction |
|---|---|---|
| Data node down | Registry lease ~5 s | read failover; degraded queue; rate-limited re-replication |
| Slow/flaky node | client RTT tracking | avoid on read; placement quarantine |
| Meta leader down | Raft heartbeat | election <1 s; clients reconnect via Registry |
| Client machine dead | lease expiry | volume claimable elsewhere; meta follower promotes |
| Bit rot | crc on read + weekly scrub | repair from healthy replica |
| GC vs re-Put race | version-checked Delete | stale Delete rejected |

### 10.3 Scrubber

Weekly `List`-driven walk per arena, crc verify, rate-limited. Mandatory on HDD (bit rot), cheap on our layout (sequential slot scan).

---

## 11. Performance Budget

10 machines × 4 HDD ≈ 6,000 raw random IOPS. With ≥90% client cache hit: HDD tier <10% loaded. Governing metric: **client SSD cache hit rate**. Dashboard (Prometheus): cache hit rate · WAL backlog · per-disk queue depth.

---

## 12. Implementation Plan — 20 Steps

Each step is a mergeable, testable increment. Steps 1–6 are single-machine; standalone mode is usable at step 10; cluster features complete at 16; hardening through 20.

| # | Step | Deliverable & acceptance |
|---|---|---|
| 1 | **Repo & skeleton** | cargo workspace: `gamefs-common` (proto/types), `gamefs-arena`, `gamefs-data`, `gamefs-meta`, `gamefs-client`, `gamefs` (bin). CI: fmt/clippy/test. |
| 2 | **ChunkArena core** | mkfs on sparse img; superblock+bitmap+slot R/W; proptest: random Put/Get/Delete then reopen ⇒ state consistent. |
| 3 | **Arena crash consistency** | startup header scan + bitmap reconcile; kill -9 mid-Put fuzz ⇒ no false-allocated slots; `criterion` baseline. |
| 4 | **Space reclamation** | hole punch / BLKDISCARD on Delete; acceptance: write 10 GiB → delete → `du` returns to baseline. |
| 5 | **Data node gRPC** | 5 RPCs over ChunkArena; idempotent Put / version-checked Delete proven by proptest. |
| 6 | **WAL** | segmented log, group-commit fsync, replay; proptest: crash at random byte offsets ⇒ prefix-consistent replay. |
| 7 | **Metadata state machine** | redb schema (ino/dentry/layout/chunkref/snap/gc); apply-op API; proptest on layout math. |
| 8 | **Single-node Raft + client core** | openraft group (1 member) driving the state machine; write path §7.2 end-to-end in-process. |
| 9 | **FUSE mount** | fuser wiring: create/read/write/unlink/mkdir/rename/fsync/statfs; `fio` randrw passes; real game asset dir loads. |
| 10 | **Standalone GA** | `gamefs standalone` one-command; acceptance: full demo on one machine — mount, write saves, kill daemon, remount, data intact. |
| 11 | **SSD read cache** | persistent catalog + LRU + Stat-validation + read lease; acceptance: warm-start asset load within 1.2× of local SSD; hit-rate metric exported. |
| 12 | **3-node metadata Raft** | multi-member groups, failover <1 s; acceptance: kill leader mid-workload, volume continues on follower. |
| 13 | **Registry** | membership, leases, node stats, pub/sub; cluster mode boots: 3 clients × 3 data nodes. |
| 14 | **Replication** | RF=2/3 placement (random + failure-domain), quorum flush, degraded marking; acceptance: kill one data node — reads unaffected, self-heal visible in metrics. |
| 15 | **Snapshots** | create/list/delete + `.snapshots` view; acceptance: snapshot → delete/modify half the tree → original fully readable. |
| 16 | **Rollback + GC + scheduler** | reversible rollback; refcount GC with snapshot awareness (hole punches reclaim space); retention policies run on schedule. |
| 17 | **Re-replication & scrubber** | background repair workers; weekly scrub; acceptance: inject bit rot (flip bytes in a slot) → detected → repaired. |
| 18 | **Cross-client sync** | pub/sub merge of foreign volumes; metadata lease escape hatch; acceptance: two clients, file written on A visible on B ≤1 s. |
| 19 | **turmoil fault-injection soak** | deterministic partitions/crashes/delays across all paths; property: zero acknowledged-write loss, zero metadata divergence. 72 h simulated soak. |
| 20 | **Production hardening** | Prometheus dashboard (3 graphs), panic-safe FUSE loop, upgrade/format-version policy, runbook, chaos drill vs. §10.2 table; fio + game-workload benchmark report. |

Estimated velocity (1 dev + AI): steps 1–10 ≈ 5–6 weeks; 11–16 ≈ 5 weeks; 17–20 ≈ 3–4 weeks.

---

## 13. Open Questions

1. **Remote WAL spill** (close the last un-`fsync` data-loss window): stream WAL segments to a peer for +1 RTT on the write fast path. Decision deferred; `fsync` path already covers save-critical data.
2. **Automatic volume takeover** on client death (v1: manual command; v2: lease-based auto).
3. Chunk size (1 MiB) / inline threshold (64 KiB): validate against the real file-size histogram at step 9.
4. mTLS + per-volume tokens: v2 security milestone.

---

*Definition of done: destroy one storage machine and fat-finger-delete a directory — full recovery in minutes via replicas and snapshots, zero data loss.*
