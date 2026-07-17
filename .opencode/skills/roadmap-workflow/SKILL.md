---
name: roadmap-workflow
description: Roadmap-driven phase workflow used by PolarisFS — numbered phases with measurable gates, contract-first format docs, delegated implementation, evidence-based verification. Use when working on any porfs phase (P1..P40), or when the user says "next phase", "干 P<n>", or asks how this project is run.
---

# Roadmap-driven phase workflow

This project runs on a strict phase cycle. ROADMAP.md is the master plan (40 phases,
each with a measurable **gate**). Follow this loop for every phase.

## The phase loop

1. **Read the contract first.** ROADMAP.md (phase definition + gate) and, if the
   phase touches on-disk or wire formats, `docs/format.md`. The format doc is the
   binding contract: change the doc and bump the version BEFORE changing code.
   A contradiction in the contract = stop and report, never silently redesign.
2. **Delegate implementation** to a `deep` or `unspecified-high` agent with a
   six-section prompt (below). If delegation is unavailable (quota), implement
   directly with the same discipline.
3. **Verify with evidence, personally**: `cargo fmt --check`,
   `cargo clippy --workspace --all-targets` (zero warnings),
   `cargo test --workspace` (all green), LSP diagnostics clean, plus the phase's
   specific gate evidence (bench numbers, soak runs) run by YOU, not only reported.
4. **Judge the gate honestly** (rules below). Pass → mark the phase done in
   ROADMAP.md with measured numbers. Fail → fix or report; never move the gateposts
   silently, never delete a failing test to pass.
5. **Commit only when the user asks** (`P<n>: <what>`), then start the next phase.

## Delegation prompt structure (all six sections, exhaustive)

```
1. TASK: one atomic goal (this phase's deliverable)
2. EXPECTED OUTCOME: verifiable criteria (tests green, clippy clean, bench numbers, ...)
3. REQUIRED TOOLS: explicit tool whitelist
4. MUST DO: every requirement — API shapes, invariants, test list, quality bar
5. MUST NOT DO: forbidden actions (no contract edits, no new deps, no commits, ...)
6. CONTEXT: repo state, machine facts, prior-phase numbers, downstream consumers
```

- Continue the same subagent session (`task(task_id="ses_...")`) for follow-ups and
  fixes — it keeps full context and saves most of the re-exploration cost.
- Line count is a reference, not a ceiling. The only code-size rule is:
  use a mature library where one exists; never reinvent wheels (see ROADMAP §0.4).

## Gate honesty rules

- Benchmark comparisons must be apples-to-apples. A CRC-verified read competes
  against a verifying baseline, not against fio with verification off. When a gate
  was mis-specified (wrong baseline), amend the gate text in ROADMAP.md with the
  reasoning recorded — do not just claim or waive the miss.
- "The engine is healthy" and "the end-to-end number is low" can both be true;
  measure and report both layers (e.g. P1: pure pipeline = 111% of fio read;
  verified read = 79%; the gap is checksum+copy bus traffic on a bandwidth-poor APU).
- Soak-type gates (kill -9, fault injection) ship as runnable scripts under
  `scripts/` so the evidence is reproducible by anyone.

## Test-artifact hygiene

- `/tmp` is tmpfs (RAM). Iterative tests must delete per-iteration device files
  inside the loop — accumulating them OOMed a 30GB machine at iteration ~500.
- Prefer `CARGO_TARGET_TMPDIR` (real disk) for device files.

## Commit conventions

- One phase = one commit (squash intermediate agent commits if any):
  `P<n>: <deliverables>; <format version if bumped>`.
- Never commit `.omo/`, `target/`, or `*.img`.
