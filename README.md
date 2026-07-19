# PolarisFS (plfs)

A KISS distributed file system for game server clusters. One binary, three roles
(data / meta / client) + standalone mode. Raft-replicated metadata per volume,
ChunkArena storage over raw block devices or sparse `.img` files with automatic
space reclamation, FUSE client with persistent SSD read cache and WAL.

- **Design contract**: [docs/design.md](docs/design.md) (v0.2)
- **Master plan**: [ROADMAP.md](ROADMAP.md) — 20 steps, each with a measurable gate
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

## Status

| Step | Content | Result |
|---|---|---|
| S1 | Repo & skeleton, CI | in progress |
| S2–S20 | See [ROADMAP.md](ROADMAP.md) | not started |

## Quickstart

```bash
cargo build --release
# (from S10) ./target/release/plfs standalone --dir /var/lib/plfs --mount /mnt/plfs
```

## Tests and gates

```bash
./scripts/ci.sh    # fmt + clippy + tests
```

## Requirements

- Linux (kernel ≥ 6.15 recommended for FUSE3), protoc, fio (gate benchmarks)
- Rust 1.96+ (edition 2024)
