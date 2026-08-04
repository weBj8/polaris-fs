# polaris-fs migration documentation

This directory holds the planning and audit documents for the `mfs-rust-rewrite`
effort: incrementally replacing the c2rust machine-translated unsafe Rust
(MooseFS 4.59.2 C sources) with idiomatic safe Rust, without changing wire
protocol, on-disk layout, or observable behavior at any merge point.

## Provenance

The document set and its structure are adapted from the audit artifacts of
Bun PR #30412 (the AI-orchestrated Zig→Rust port, merged 2026-05-14), as
recovered and analyzed by Michael Lady:

- <https://gist.github.com/michaellady/7e63223d5d72d9ad18a03efa1f376aae> — broadly applicable lessons
- <https://gist.github.com/michaellady/7d552137fb1e37ab9bf637e450016c25> — full investigation (8 documents)

The methodology is Bun's; the facts are polaris-fs's. Each document below
names the Bun artifact it adapts.

## Reading order

| Document | Adapts | Role |
| --- | --- | --- |
| [methodology.md](methodology.md) | `LESSONS_LEARNED.md`, `BROADLY_APPLICABLE_LESSONS.md` | What worked / what didn't in the Bun port, and the binding rules this migration adopts. Read first. |
| [rewrite-plan.md](rewrite-plan.md) | `rust-rewrite-plan.md` | Architecture and phase plan: why, what, constraints, crate map, phases P0–P6 with caps and exit criteria. The plan we execute against. |
| [porting.md](porting.md) | `PORTING.md` | The coding standard for converting one transpiled module to safe Rust. Read before writing any code in a phase. |
| [deployment.md](deployment.md) | — (ours) | Container quickstart, multi-host Docker/Podman deployment, safe upgrades, and rollback. |
| [verified-claims.md](verified-claims.md) | `rust-rewrite-verified-claims.md` | The fact corpus. Every claim is cited to `file:line` and survives adversarial review. The plan is a derived artifact of these facts. |
| [divergence-audit.md](divergence-audit.md) | `ZIG_RUST_DIVERGENCE_AUDIT.md` | Known C→transpiled-Rust semantic divergence classes, risk-rated, worked as a punch-list. |
| [aliasing-hunt.md](aliasing-hunt.md) | `NOALIAS_HUNT_REPORT.md`, `NOALIAS_SUSPECTS.md` | Protocol and punch-list for the UB hunt: `static mut` aliasing, `&mut`-from-raw-pointer, re-entrant callbacks. |
| [p1-retrospective.md](p1-retrospective.md) | — (ours) | P1 pilot calibration: what the gates caught/missed, canonical shim layout, adjustments adopted for P2+. |
| [facts/](facts/) | `LIFETIMES.tsv` | Generated fact artifacts: `OWNERSHIP.tsv` (static-mut pre-classification), `dedup-map.md`, `ffi-boundaries.md`. Regenerate with `tools/gen_*.py` / `tools/gen_*.sh`. |

## Current status

P2 is complete. `plfsmetalogger` and `plfsgui` now use Rust-owned config,
daemon lifecycle, master protocol, and HTTP/CGI state. Their remaining unsafe
code is confined to documented libc, socket, process, and variadic ABI seams.
The `plfscommon::charts` format migration is explicitly transferred to P5,
where `plfsmaster::chartsdata` owns the writer-side compatibility proof.

P4 remains partial. Current mount/client ownership and thread waves are recorded
in [rust-native-plfsmount-audit.md](rust-native-plfsmount-audit.md). Commit
The latest completed source waves migrate `mfs_fuse` dirbuf and finfo ownership
to typed Rust registries with `Arc`, `Mutex`, and `Condvar`; mount now has 96
lib tests plus 2 bin tests. Production runs verified image
`45ff7343310a` per [deployment.md](deployment.md). Remaining work: typed
collections for backend intrusive lists and the separate bdev data path.

## House rules (from methodology.md, binding on all docs here)

1. **Facts before plans.** `verified-claims.md` is written and reviewed before
   any phase plan that depends on it.
2. **Nothing in this directory is deleted.** Closed items are struck through,
   not removed. The closed-item history is the evidence the methodology ran.
3. **Default-deny.** A claim, a migration, or a fix is `confirmed=false`
   unless verified against source at `file:line`.
4. **IOUs are explicit.** `// MIGRATION-IOU: blocked_on: <crate>::<item>` in
   code; the ledger lives in `verified-claims.md`. No silent accumulation.
