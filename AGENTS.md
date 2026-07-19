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
- **Workflow skill**: `.opencode/skills/roadmap-workflow` — follow it for every step.

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
8. **Test hygiene**: `/tmp` is tmpfs (RAM, 16 GiB on this box). Iterative tests
   must delete per-iteration device files inside the loop; prefer
   `CARGO_TARGET_TMPDIR` (real disk) for device files.
9. **Never commit** `.omo/`, `target/`, or `*.img`.

## Machine facts

- Rust 1.96+ (edition 2024), protoc 35.1, fio available, 16 cores.
- Kernel on this dev box: ≥ 6.15 (FUSE3 fine).

## Repo layout

```
ROADMAP.md              20-step master plan, every step has a measurable gate
docs/design.md          GameFS design document v0.2 (binding design contract)
docs/format-arena.md    arena on-disk format contract (from S2; versioned)
crates/plfs-common      proto/types (data-plane gRPC contracts)
crates/plfs-arena       ChunkArena storage engine
crates/plfs-data        data node gRPC service
crates/plfs-meta        metadata state machine + Raft
crates/plfs-client      FUSE client + cache + WAL
crates/plfs             the plfs binary (one binary, three roles + standalone)
scripts/                gate scripts (ci.sh, soak/fuzz harnesses, ...)
```

## Current phase

See ROADMAP.md §4 (single source of truth).
