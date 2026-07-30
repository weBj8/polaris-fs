# P1 pilot retrospective — mfscommon migration

*2026-07-30, branch `p1/mfscommon-safe`. Scope: 14 shared modules, 16,088
LOC, 336 unsafe sites. Outcome: 12/14 modules safe-ified, 2 classified as
annotated boundaries (1 with IOU). 11 unit tests added. SMOKE + gates green
throughout.*

## What the pipeline cost and caught

### Facts-first paid off

- `OWNERSHIP.tsv` pre-classification was right for every module: scalar
  globals → atomics (mfslog), buffer/state globals → Mutex (cpuusage,
  conncache, delayrun), init-once → `LazyLock` (crc tables), pointer-heavy →
  boundary (processname, sockets).
- The dedup map prevented the worst trap: 3 of the 14 modules are used by
  daemon pairs with *different* per-daemon variants elsewhere — knowing
  exactly which files were byte-identical made "migrate once, link
  everywhere" safe.

### The shim pattern (key calibration result)

`#[unsafe(no_mangle)]` is itself `unsafe_code` in edition 2024. Consequence:
**no file that exports a C ABI symbol can live under a module-level
`deny(unsafe_code)`**. The working layout, now canonical for P2+:

- safe logic in a `#[deny(unsafe_code)] mod imp` (or top-level safe fns),
- C ABI exports at file top level, *outside* any deny scope,
- `unsafe fn` shims deref pointers → safe cores take `&mut`/slices,
- `unsafe extern "C" fn` → safe `extern "C" fn` wherever params are POD.

Consumers needed zero changes — c2rust's linker-wiring means symbol ABI is
the only contract. Two modules (`delay_heap_sort_*`) had exported symbols
referenced by nobody; dropping them was verified by grep first. Always
verify-before-drop: `mfs_log_priority_strings` was *also* unreferenced, but
it's a `static`, so it went private instead of being deleted.

### What the gates caught

1. **The wrapping gate fired on the dedup PR itself** (counts moved crates).
   Baseline re-freeze worked as designed — the move is an auditable edit in
   `baseline.tsv` with a justification comment. Lesson: gate baselines must
   be per-crate, and any crate-splitting PR updates them in the same commit.
2. **SAFETY-comment gate caught a doc-format miss** (sockets pass wrote
   `# Safety` without the literal token). Cheap heuristic, real catch.

### What the gates missed

1. **`cargo test` broke on cross-crate symbols** (strerr lives in divergent
   per-daemon copies). Fix was a `#[cfg(test)]` stub — fine — but the gate
   set had no "does the crate still link standalone" check. `cargo test`
   per crate now covers it; keep running it per module.
2. **Behavioral oracle is thin for non-cluster modules.** SMOKE exercises
   clocks/crc/sockets/mfslog/conncache heavily, but labelparser and
   timeparser only get exercised on sclass config paths. The unit tests
   (round-trip vectors) filled the gap — **unit tests with reference
   vectors are the oracle for parser/codec modules**; SMOKE is the oracle
   for runtime modules. Both are needed; neither suffices alone.
3. **My own test vectors were wrong twice** (timeparser `~1m`, labelparser
   `@4+2,*`) — the C original's actual behavior was the spec, and I had
   mis-modeled it. Default-deny applied to *my* expectations: compute the
   expected value from the original code, not from intuition.

### IOU discipline worked

charts.rs (5.2k LOC binary-format I/O) was IOU'd with named consumers
(P2/P5) instead of being rushed. Gate baseline moved 0→1 with a ledger
entry — the mechanism did what methodology.md said it should: bounded the
phase without silent accumulation.

## Numbers

| Metric | Before P1 | After |
| --- | ---: | ---: |
| mfscommon unsafe sites | 336 | 244 (sockets wrappers + charts) |
| — of which without SAFETY note | 336 | 175 |
| mfscommon `static mut` (TSV classes) | all `auto:type-shape` | resolved per module (atomics/Mutex/LazyLock/boundary) |
| Modules fully safe-ified | 0/14 | 12/14 |
| Unit tests | 0 | 11 |
| Variadic leaves confined | — | mfslog (2 fns) |
| MIGRATION-IOU | 0 | 1 (charts → P2/P5) |

## Adjustments adopted for P2+

1. Shim layout (imp + top-level exports) is the canonical pattern.
2. Parser/codec modules get reference-vector unit tests in the same PR;
   runtime modules get SMOKE. Both run per module, not per phase.
3. Expected values in tests are derived from the original's code path, not
   intuition.
4. Baseline re-freeze rides in the same commit as the change that moves a
   counter (learned the easy way in P0 dedup).
5. `cargo test` (standalone link) is a per-module gate, not just phase-end.
