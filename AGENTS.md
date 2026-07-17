# AGENTS.md — rules for AI agents working in this repo

## Build & verify (must ALL pass before reporting done)

```bash
cargo fmt
cargo clippy --workspace --all-targets   # zero warnings
cargo test --workspace                   # all green
```

- Evidence discipline: no passing run = work not complete. Bench-type gates must
  include measured numbers (and the fio baseline they are compared against).
- Keep LSP diagnostics clean on changed files.

## Project rules

1. **The format is the contract.** Before changing the on-disk or wire format, edit
   `docs/format.md` first and bump `FORMAT_VERSION`; only then touch code. If you find
   a real contradiction in the contract, STOP and report — never silently redesign.
2. **Gate discipline.** Each ROADMAP phase has a gate; do not start the next phase
   before the gate passes. Never delete or weaken a failing test to "pass". Benchmark
   comparisons must be apples-to-apples (verified reads vs verifying baselines).
3. **Libraries first.** If a mature open-source crate already solves ~80% of a
   subsystem, use it instead of writing our own (see the selection table in
   ROADMAP.md §0.4). Line count is a reference, not a ceiling — never shuffle code
   around just to hit a LOC budget.
4. **Crash recovery is the product.** After substantial storage changes, run
   `./scripts/kill9-soak.sh` (1000 kill -9 iterations; ~65s in release).
5. **No commits or pushes unless the user explicitly asks.** Commit message format:
   `P<n>: <what>` (e.g. `P2: checkpoint fast-mount + ...`). Do not commit `.omo/`.

## Environment notes

- `/tmp` is tmpfs (RAM): large test device files belong in `CARGO_TARGET_TMPDIR` or
  under `/home`, and per-iteration artifacts must be deleted inside the loop —
  accumulating them OOMs the machine (this actually happened).
- `*.img` is gitignored; never commit device images.
- Dev machine: x86_64, kernel 7.1.3, AMD Ryzen 7 8845HS (mobile APU — single-core
  DRAM bandwidth ~7GB/s; interpret bench numbers accordingly).

## Current phase

See ROADMAP.md §4 ("Current actions"). Completed: P1, P2. Next: P3 (MDS v0).
