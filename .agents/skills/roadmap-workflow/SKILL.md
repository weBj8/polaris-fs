---
name: roadmap-workflow
description: Roadmap-driven step workflow used by PolarisFS (plfs) — numbered steps with measurable gates, contract-first format docs, delegated implementation, evidence-based verification. Use when working on any plfs step (S1..S20), or when the user says "next step", "干 S<n>", or asks how this project is run.
---

# Roadmap-driven step workflow

This project runs on a strict step cycle. ROADMAP.md is the master plan (20 steps,
each with a measurable **gate**). Follow this loop for every step.

## The step loop

1. **Read the contract first.** ROADMAP.md (step definition + gate), the design
   contract `docs/design.md`, and, if the step touches on-disk or wire formats,
   `docs/format-arena.md` or the proto file under `crates/plfs-common/proto/`.
   The format doc is the binding contract: change the doc and bump the version
   BEFORE changing code. A contradiction in the contract = stop and report,
   never silently redesign.
2. **Delegate implementation** to a `deep` or `unspecified-high` agent with a
   six-section prompt (below). If delegation is unavailable (quota), implement
   directly with the same discipline.
3. **Verify with evidence, personally**: `cargo fmt --check`,
   `cargo clippy --workspace --all-targets` (zero warnings),
   `cargo test --workspace` (all green), LSP diagnostics clean, plus the step's
   specific gate evidence (bench numbers, soak runs) run by YOU, not only reported.
4. **Judge the gate honestly** (rules below). Pass → mark the step done in
   ROADMAP.md with measured numbers. Fail → fix or report; never move the gateposts
   silently, never delete a failing test to pass.
5. **Commit only when the user asks** (`S<n>: <what>`), then start the next step.

## Delegation prompt structure (all six sections, exhaustive)

```
1. TASK: one atomic goal (this step's deliverable)
2. EXPECTED OUTCOME: verifiable criteria (tests green, clippy clean, bench numbers, ...)
3. REQUIRED TOOLS: explicit tool whitelist
4. MUST DO: every requirement — API shapes, invariants, test list, quality bar
5. MUST NOT DO: forbidden actions (no contract edits, no new deps, no commits, ...)
6. CONTEXT: repo state, machine facts, prior-step numbers, downstream consumers
```

- Continue the same subagent session (`task(task_id="ses_...")`) for follow-ups and
  fixes — it keeps full context and saves most of the re-exploration cost.
- Line count is a reference, not a ceiling. The only code-size rule is:
  use a mature library where one exists; never reinvent wheels (see ROADMAP §0.1).

## Gate honesty rules

- Benchmark comparisons must be apples-to-apples. A CRC-verified read competes
  against a verifying baseline, not against fio with verification off. When a gate
  was mis-specified (wrong baseline), amend the gate text in ROADMAP.md with the
  reasoning recorded — do not just claim or waive the miss.
- "The engine is healthy" and "the end-to-end number is low" can both be true;
  measure and report both layers.
- Soak-type gates (kill -9, fault injection) ship as runnable scripts under
  `scripts/` so the evidence is reproducible by anyone.

## Test-artifact hygiene

- `/tmp` is tmpfs (RAM, 16 GiB on this box). Iterative tests must delete
  per-iteration device files inside the loop — accumulating them OOMed a 30GB
  machine at iteration ~500.
- Prefer `CARGO_TARGET_TMPDIR` (real disk) for device files.

## Commit conventions

- One step = one commit (squash intermediate agent commits if any):
  `S<n>: <deliverables>; <format version if bumped>`.
- Never commit `.omo/`, `target/`, or `*.img`.
