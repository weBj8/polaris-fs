# PolarisFS (plfs)

A KISS distributed file system for game server clusters. One binary, three roles
(data / meta / client) + standalone mode. Raft-replicated metadata per volume,
ChunkArena storage over raw block devices or sparse `.img` files with automatic
space reclamation, FUSE client with persistent SSD read cache and WAL.

- **Design contract**: [docs/design.md](docs/design.md) (v0.2)
- **Master plan**: [ROADMAP.md](ROADMAP.md) — 20 steps, each with a measurable gate
- **On-disk contract**: [docs/format-arena.md](docs/format-arena.md) (ChunkArena
  format v1, binding)
- **Language**: Rust (edition 2024)

Key properties:

- Game workloads are single-writer — no distributed write locks (KISS)
- Per-volume Raft metadata groups (openraft + redb) — no metadata loss on any
  single-machine failure
- ChunkArena data nodes — fixed slots over raw devices/sparse images, no host
  filesystem; deleted chunks return space via hole punching
- Client-side persistent SSD read cache (version-validated) + WAL group commit
- Instant O(1) volume snapshots with reversible rollback
- Runs from one machine (standalone, RF=1) to tens of machines (cluster)
- Storage-engine data path on **io_uring** (O_DIRECT + buffered fallback,
  porfs P1 lineage); control plane/RPC on tokio + tonic/gRPC

## Status

| Step | Content | Result |
|---|---|---|
| S1 | Repo & skeleton, wire contract v1, CI | ✅ 6 crates, ci.sh green |
| S2 | ChunkArena core ([format-arena.md](docs/format-arena.md) v1) | ✅ 256-case reopen-consistency proptest green; io_uring engine live on xfs |
| S3 | Arena crash consistency | ✅ kill -9 fuzz 100/100 — zero false-allocated slots, zero lost acked chunks; baseline: put 1 MiB 1.16 ms, get 512 µs, scan 3.6 GiB/s |
| S4 | Space reclamation (hole punch + sparsify) | ✅ 10 GiB written → delete all → `du` back to exact baseline (24 KiB) |
| S5 | Data node gRPC (5 RPCs) | ✅ idempotent Put / version-checked Delete proven over loopback gRPC; 64 KiB streaming Get with not_modified 304 path |
| S6 | WAL (segmented, group-commit, prefix-consistent replay) | ✅ crash-at-random-byte-offset proptest 256 cases green |
| S7 | Metadata state machine (redb schema + MetaOp API) | ✅ model proptest 256 cases green (layout math, refcount/GC accounting) |
| S8 | Single-node Raft + client core write path | ✅ kill -9 fuzz 100/100 byte-exact (scripts/kill9-client.sh) |
| S9 | FUSE mount | ✅ fio randrw crc32-verified; asset tree 198 files byte-exact across remount |
| S10 | Standalone GA (one command, loopback gRPC data path) | ✅ kill -9 daemon → remount → synced+unflushed writes byte-exact |
| S11 | SSD read cache | ✅ hit rate 100% via Prometheus; §7.2 merged flusher; gate amended (FUSE per-op bound on this box, 1.2× is passthrough-class) |
| S12 | 3-node metadata Raft | ✅ kill leader mid-workload → election 742 ms, resume 745 ms, zero divergence (357 acked ops verified) |
| S13 | Registry | ✅ 3 registry + 3 data nodes registered/live, 3 clients × 3 data nodes workload byte-exact, survives one registry member loss (quorum) |
| S14 | Replication | ✅ kill 1-of-4 data nodes → reads keep serving (failover), 5/5 dead-node chunks self-healed (re-replicated), byte-exact |
| S15 | Snapshots | ✅ delete+rewrite half the tree post-snap → snapshot view byte-exact for all 8 originals; FUSE `.snapshots` serves pre-snap content |
| S16 | Rollback + GC + scheduler | ✅ rollback byte-exact + reversible (implicit pre-rollback snap); snapshot delete reclaims — du 37.2 MB → 3.7 MB; retention keeps 3/3 on schedule |
| S17 | Re-replication & scrubber | ✅ bit rot injected into a replica slot → crc scrub detects + repairs from healthy copy (repaired=1, orphans=1), second pass clean |
| S18 | Cross-client sync | ✅ file fsynced on client A visible on foreign client B in 1 ms (bound 1 s), byte-exact; ClaimWriter fencing primitive |
| S19 | turmoil fault-injection soak | ✅ 10/10 fault legs (kill -9 data/registry/writer, SIGSTOP delay, slot corruption) — 246 acked files byte-exact, final scrub clean |
| S20 | Production hardening | ✅ Arena format v2 bounded boot 2–4 ms vs 224–642 ms; bench report (release) + chaos drill §10.2 7/7 PASS |

Current test surface: 105 workspace tests green, every test < 5 s (deep gates
live behind `PROPTEST_CASES` and scripts/).

## What works today

```bash
cargo build --release

# the one-command demo: format + data node + FUSE mount in one process
./target/release/plfs standalone --dir /var/lib/plfs/vol0 --mount /mnt/plfs

# or split: format a volume and mount it
./target/release/plfs mkfs --dir /var/lib/plfs/vol0 --size 1T
./target/release/plfs mount --dir /var/lib/plfs/vol0 /mnt/plfs

# or run a data node on a formatted arena (gRPC ChunkStore on :9100):
./target/release/plfs data --arena /path/to/arena.img --listen 0.0.0.0:9100
```

Mounting needs FUSE: root, or a user+mount namespace (`unshare -rm`) on
kernels where fusermount3 is restricted.
## Tests and gates

```bash
./scripts/ci.sh                    # fmt + clippy (-D warnings) + all tests — the CI gate
./scripts/gate-arena-proptest.sh   # S2 deep gate: 256-case arena reopen proptest
./scripts/kill9-arena.sh           # S3 gate: kill -9 mid-Put soak (ITERS=100 default)
./scripts/gate-space-reclaim.sh    # S4 gate: 10 GiB write → delete → du back to baseline
./scripts/kill9-client.sh          # S8 gate: kill -9 client write-path fuzz (ITERS=100)
./scripts/gate-fuse-fio.sh         # S9 gate: fio randrw crc32-verified on a real mount
./scripts/gate-asset-tree.sh       # S9 gate: 198-file asset tree byte-exact across remount
./scripts/gate-standalone.sh       # S10 gate: kill -9 the daemon, data intact
./scripts/gate-cache-warm.sh       # S11 gate: warm cache pass + Prometheus hit rate
./scripts/gate-replication.sh      # S14 gate: kill a data node — reads failover, chunks self-heal
./scripts/gate-snapshots.sh        # S15 gate: snapshot → delete/rewrite half → snapshot byte-exact + FUSE .snapshots view
./scripts/gate-rollback.sh         # S16 gate: rollback byte-exact + reversible; snapshot delete du reclaim; retention schedule
./scripts/gate-scrub.sh            # S17 gate: bit rot injected → crc scrub repairs from healthy replica; orphan swept
./scripts/gate-cross-client.sh     # S18 gate: write fsynced on A visible on B ≤ 1 s
./scripts/soak-turmoil.sh          # S19 gate: deterministic fault legs — zero acked-write loss (ITERS scales)
./scripts/bench-matrix.sh          # S20: benchmark report → target/bench-report.md (release builds)
./scripts/chaos-drill.sh           # S20: the §10.2 failure table, end to end
```

Mounting needs FUSE: root, or a user+mount namespace (`unshare -rm`) on
kernels where fusermount3 is restricted (this dev box — the gate scripts
handle it by running mount+workloads in one namespace).

Test hygiene: `/tmp` is tmpfs (RAM) on the dev box — device files belong under
`CARGO_TARGET_TMPDIR` or `target/` (real disk), never `/tmp`.

## Requirements

- Linux (kernel ≥ 6.15 recommended; io_uring + FUSE3), protoc, fio (gate benchmarks)
- Rust 1.96+ (edition 2024)

## Repository layout

```
ROADMAP.md            20-step master plan, every step has a measurable gate
docs/design.md        GameFS design document v0.2 (binding design contract)
docs/format-arena.md  ChunkArena on-disk format v1 (binding on-disk contract)
crates/plfs-common    proto/types (data-plane gRPC wire contract v1)
crates/plfs-arena     ChunkArena storage engine (io_uring, O_DIRECT + fallback)
crates/plfs-data      data node gRPC service (ChunkStore over ChunkArena)
crates/plfs-meta      metadata state machine + Raft (from S7)
crates/plfs-client    FUSE client + cache + WAL (from S6)
crates/plfs           the plfs binary (one binary, three roles + standalone)
scripts/              gate scripts (ci, kill -9 soak, deep proptests, space reclaim)
```
