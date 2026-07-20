# Agent Instructions — PolarisFS (plfs)

This file defines how AI agents work in this repository.

## Project

PolarisFS is a KISS distributed file system for game server clusters. One binary
(`plfs`), three roles (data / meta / client) + standalone mode. Raft-replicated
metadata per volume, ChunkArena storage over raw block devices or sparse images,
FUSE client with persistent SSD cache and WAL. Implementation language: Rust.

- **Design contract**: [`docs/design.md`](docs/design.md) (GameFS Design Document
  v0.2 — binding; `gamefs` spellings there map to project/binary names here).
- **Master plan**: [`ROADMAP.md`](ROADMAP.md) (20 steps, each with a measurable gate).
- **On-disk contract**: [`docs/format-arena.md`](docs/format-arena.md) (ChunkArena
  format v1 — binding; change it and bump the version BEFORE code).
- **Workflow skill**: `.opencode/skills/roadmap-workflow` — follow it for every step.

## Architecture decisions worth knowing

- **I/O split**: storage-engine data path uses **io_uring** (O_DIRECT + buffered
  fallback, porfs P1 lineage — SQ pipeline, aligned buffer pool); control plane
  and RPC use tokio + tonic/gRPC. (Owner amendment 2026-07-19, ROADMAP §0.1.)
- **The `!Send` arena never crosses threads**: it is created on and owned by a
  dedicated actor thread (see `plfs-data::spawn_actor`); async gRPC handlers
  exchange commands with it over a channel.
- **Headers are truth, the bitmap is a hint**: ChunkArena boot reconciles the
  bitmap from slot headers at every open. A bitmap-only delete can resurrect at
  reopen; `sparsify` must run in-session (format-arena.md §5–§7).

## Build & verify

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
./scripts/ci.sh            # all of the above, in order
```

## Rules for AI agents

1. **Never allow clippy warnings to remain** — fix them.
2. **Never add `#[allow(...)]` or `rustfmt::skip`** to silence diagnostics.
3. **Prefer smallest reasonable diffs** — no opportunistic refactors.
4. **Never commit** unless the user explicitly asks (one step = one commit).
5. **Contract first**: on-disk/wire format changes edit the contract doc
   (`docs/format-arena.md`, proto file) and bump the version BEFORE code.
   A contradiction in the contract = stop and report, never silently redesign.
6. **Borrow, don't build**: if a mature crate solves it (ROADMAP §0.1), don't
   hand-roll it. Hand-roll only the differentiating parts (ChunkArena, WAL,
   cache, FUSE semantics).
7. **Gates & evidence**: every step closes with measured evidence (test counts,
   bench numbers, soak results) run by the orchestrator personally, and
   ROADMAP.md is updated with the numbers.
8. **Every `cargo test` test must finish in < 5 s** (owner ruling). Deep gates
   (hundreds of proptest cases, kill -9 soaks, 10 GiB write tests) live behind
   env knobs (`PROPTEST_CASES`, `ITERS`) and runnable scripts under `scripts/`;
   CI defaults stay fast.
9. **Test hygiene**: `/tmp` is tmpfs (RAM, 16 GiB on this box). Iterative tests
   must delete per-iteration device files inside the loop; use
   `CARGO_TARGET_TMPDIR` or `target/` (real disk) for device files.
10. **Never commit** `.omo/`, `target/`, or `*.img`.

## Machine facts

- Rust 1.96+ (edition 2024), protoc 35.1, fio available, 16 cores.
- Kernel on this dev box: ≥ 6.15 (io_uring + FUSE3 fine); root fs is xfs
  (hole punching works; /dev/shm is the forced-buffered fallback test path).

## Repo layout

```
ROADMAP.md              20-step master plan, every step has a measurable gate
docs/design.md          GameFS design document v0.2 (binding design contract)
docs/format-arena.md    ChunkArena on-disk format v1 (binding on-disk contract)
crates/plfs-common      proto/types (data-plane gRPC wire contract v1)
crates/plfs-arena       ChunkArena storage engine (io_uring, O_DIRECT + fallback)
crates/plfs-data        data node gRPC service (ChunkStore over ChunkArena)
crates/plfs-meta        metadata state machine + Raft (from S7)
crates/plfs-client      FUSE client + cache + WAL (from S6)
crates/plfs             the plfs binary (one binary, three roles + standalone)
scripts/                gate scripts (ci.sh, kill9-arena, gate-* deep gates)
```

## Current phase

See ROADMAP.md §4 (single source of truth).
