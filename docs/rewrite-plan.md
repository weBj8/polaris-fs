# Incrementally Rewriting polaris-fs in Safe Rust

*Adapts `rust-rewrite-plan.md` from the Bun PR #30412 audit set. Where that
document planned a Zig→Rust port, this one plans the second half of ours: the
C→unsafe-Rust machine translation is **done** (c2rust 0.22.1, MooseFS 4.59.2,
verified working end-to-end); what remains is replacing unsafe transpiled code
with idiomatic safe Rust, module by module.*

---

## Why

The transpile preserves C semantics, including C's bug classes. The code
compiles as Rust but is not *safe* Rust: every memory bug the MooseFS C code
could have, this code can still have — plus new ones introduced by translation
(see [divergence-audit.md](divergence-audit.md)).

Measured baseline of what we're carrying (see
[verified-claims.md](verified-claims.md) for citations):

| Metric | Count | Note |
| --- | ---: | --- |
| Total `.rs` LOC across 7 crates | 498,286 | machine-translated |
| `unsafe fn` / `unsafe {}` / `unsafe impl` sites | 8,727 | grep baseline, frozen in P0 |
| `static mut` declarations | 1,786 | aliasing-mutability class |
| `extern "C"` occurrences | 5,151 | intra-crate C ABI seams |
| `wrapping_*` arithmetic ops | 11,498 | C overflow semantics, machine-inserted |
| Raw-pointer casts in `plfsmaster/src` alone | 12,664 | `as *mut` / `as *const` |
| Nightly-only features | `core_intrinsics`, `c_variadic` in all 7 crates | blocks stable toolchain |

And one existence proof that the baseline ships real memory bugs: the
groups-cache use-after-free in `plfsmount` — a latent MooseFS bug that crashed
`opendir` under root workloads, fixed in commit `35f00f4`. It survived
translation, smoke tests, and deployment. It is the kind of bug the borrow
checker makes unrepresentable.

Without stronger compile-time guarantees, hardening this codebase stays a
cat-and-mouse game. The proposal is to remove the largest bug classes
structurally rather than fix instances indefinitely.

## What

A strangler-fig migration of ~498K LOC of c2rust-transpiled unsafe Rust to
idiomatic safe Rust. Unlike Bun's port, there is **no second language and no
FFI seam to flip** — the whole system is already one Rust process per daemon.
The unit of migration is the **module** (one `src/**/*.rs` file, occasionally
a small cluster). Each migrated module:

- replaces unsafe bodies with safe equivalents per [porting.md](porting.md),
- gains `#![forbid(unsafe_code)]` (or the tightest feasible `deny`),
- passes the behavioral-equivalence gate (snapshot diff against the unmigrated
  daemon, per [methodology.md](methodology.md) correction #3),
- lands as its own merge-commit PR.

**No big-bang rewrite. No behavior change at any merge point.** Wire protocol
(master↔chunkserver↔metalogger↔client, MooseFS 4.59.2 protocol) and on-disk
layout (`chunk_<id>_<version>.mfs`, metadata image format) are frozen — a
migrated daemon must interoperate with unmigrated ones and read existing data.

## Constraints

The philosophy that got the transpile working stays:

- **Behavior is the spec.** The transpiled code is the reference
  implementation. Where its behavior is surprising (wrapping arithmetic, C
  integer promotion, sentinel values), the safe Rust reproduces it — and the
  claim recording *why* goes into [verified-claims.md](verified-claims.md).
- **Very few new dependencies.** std first; existing workspace deps second;
  a new crate needs justification in the PR description. No async runtime —
  the daemons are poll-loop state machines and stay that way.
- **Zero performance regression on hot paths.** Chunk read/write, metadata
  operations, and the FUSE data path are gated by benchmark or by
  no-additional-allocation inspection; perf-relevant idioms carry
  `// PERF(port):` markers per [porting.md](porting.md).
- **Release-mode semantics.** The code relies on wrapping arithmetic; debug
  builds panic. All gates run release or with `overflow-checks = off`.
- **Nightly, for now.** `c_variadic` (stabilizing in Rust 1.99) and
  `core_intrinsics` are required until P6 removes them. Stable toolchain is an
  exit criterion, not a prerequisite.

## Why not stay unsafe / why not rewrite from scratch

Staying unsafe keeps every C bug class live and adds translation-induced ones
([divergence-audit.md](divergence-audit.md) — dropped `_Atomic` statics,
bitfield derive patch, variadic API drift). A from-scratch rewrite (the
parallel `plfs-*` effort, referenced in README) discards the verified-working
protocol and on-disk compatibility and re-derives a decade of MooseFS edge-case
behavior. The strangler path keeps a working cluster at every commit while
monotonically shrinking the unsafe surface. For an agent-assisted fleet, and
for reviewers, "wrong code doesn't compile" beats "wrong code might crash
under load."

## Crate map and migration order

Order principle: **leaves before core, small before large, test
infrastructure before the code it must verify.** Each crate is one cargo
package, one daemon binary.

| Phase | Crate(s) | LOC | unsafe | Why this position |
| --- | --- | ---: | ---: | --- |
| ~~P1 (pilot)~~ | ~~`plfsnetdump`~~ | ~~2,711~~ | ~~44~~ | ~~Smallest; single file; no cluster role~~ **Revised (user directive, 2026-07-30):** P1 targets the shared `plfscommon` crate first — every daemon benefits, and the shim pattern it requires (extern "C" signatures kept for symbol-linking consumers) is the pattern all later phases need. `plfsnetdump` demoted to a P6 cleanup item. |
| **P1 (revised)** | `plfscommon` (14 shared modules) | 16,088 | 336 | Shared by all daemons; calibrates the shim pattern + gates on real shared code |
| P2 | `plfsmetalogger`, `plfsgui` | 27,658 | 944 | Small, single-purpose daemons; real network protocol surface |
| P3 | `plfschunkserver` | 97,856 | 1,288 | Data path: chunk I/O, CRC, disk layout. Heavy `static mut` and buffer management |
| P4 | `plfsmount` + `plfsbdev` | 224,562 | 3,167 | **Shared `plfsclient/` tree in two copies** — must be de-duplicated into one shared crate *first* (else every fix lands twice) |
| P5 | `plfsmaster` | 145,499 | 3,284 | Metadata core, hardest, last — methodology and test infra are most mature here |
| P6 | workspace-wide | — | — | nightly-feature removal, vendor-patch upstreaming, stable toolchain |

`plfscommon/` exists per-crate as transpiled copies (measured: 15 files
byte-identical across crates — `clocks`, `crc`, `md5`, `mfslog`,
`processname`, `sockets`, `timeparser`, `charts`, `cpuusage`, `memusage`,
`conncache`, `delayrun`, `labelparser`, `lwthread`; 5 divergent — `cfg.rs`,
`main.rs`, `pcqueue.rs`, `random.rs`, `strerr.rs` differ per daemon because
MooseFS compiles the convenience library per-binary with different configure
defines). `plfsclient/` similarly: 12 files byte-identical between `plfsmount`
and `plfsbdev`, the rest diverge (FUSE vs NBD paths). **P0 therefore splits
into two tracks:** (a) mechanically de-duplicate byte-identical files into
shared `plfscommon` / `plfsclient` crates — zero behavior risk, the copies are
md5-identical; (b) divergent files stay per-crate and migrate with their
owning crate. Migrating six identical copies of `mfslog.rs` to safe Rust six
times is waste; migrating divergent `cfg.rs` into one shared file is a
behavior change. The split respects both.

## Approach per module

1. **Facts.** Write/refresh the `verified-claims.md` entries covering the
   module: ownership of every `static mut` it touches (classified in
   `docs/facts/OWNERSHIP.tsv`), every raw-pointer field, every FFI seam.
2. **Draft.** Translate per [porting.md](porting.md): same file, same fn
   names (snake_case), same control flow unless borrow-checker reshaping is
   required (marked `// PORT NOTE: reshaped for borrowck`).
3. **Verify (default-deny).** Independent verifier cites transpiled
   `file:line` + migrated `file:line` + equivalence argument; the literal
   prompt is "Default confirmed=false unless you verify against the
   transpiled source at file:line."
4. **Gate.** `forbid(unsafe_code)` in; unsafe counter down by the module's
   count; behavioral snapshot diff empty; Miri clean on the module.
5. **IOU or merge.** Unfinished sites get
   `// MIGRATION-IOU: blocked_on: <crate>::<item>` + ledger entry; else merge
   (merge commit, no squash).

### Loop bounds and IOUs

Per [methodology.md](methodology.md) #6: no unbounded loops. Caps:

| Phase | Round cap (modules per agent-run before forced review) | Wall-clock cap |
| --- | ---: | --- |
| P0 (facts + gates + dedup) | — | 2 weeks |
| P1 pilot | 10 | 2 weeks |
| P2 | 15 | 4 weeks |
| P3 | 20 | 6 weeks |
| P4 | 25 | 8 weeks |

P4 no longer de-duplicates `plfsclient/` itself — the byte-identical share
(12 files) moves into a shared crate in P0; P4 migrates the shared crate's
client modules plus each daemon's divergent files (`mfs_fuse.rs`,
`getgroups.rs`, cache modules in `plfsmount`; `mfsioint_*`, `squeue.rs`,
`workers.rs` in `plfsbdev`).
| P5 | 30 | 10 weeks |
| P6 | — | 2 weeks |

An item that exceeds its budget becomes an IOU. Phase exit requires
unresolved IOUs = 0, or each IOU explicitly transferred to a named later
phase in the ledger.


## Phase status log

### P4 (plfsmount) — DONE (2026-07-30)

Scope delivered: the `plfsmount` crate (FUSE frontend, 53.3k transpiled
LOC). 17 modules migrated to safe cores with C ABI boundaries preserved;
2 modules documented as annotated unsafe boundaries.

Migrated (safe core in `#[deny(unsafe_code)] mod imp`, tests):
dirblob_name_index, dirblob_node_index, oplog, sustained_inodes,
symlinkcache, sustained_stats, sustained_parents, fdcache,
negentrycache, dentry_invalidator, xattrcache, getgroups, dirattrcache,
masterproxy, pcqueue, strerr, mfs_meta_fuse.

Annotated boundaries (justified in module headers):
- `mfs_fuse.rs` — libfuse kernel callback protocol (raw request/buffer
  lifetimes owned by libfuse, reply-exactly-once C contract).
- `plfsmount.rs` — process bootstrap (argv/fuse_args, daemonize, setuid,
  signal handlers, FUSE session loop); its extractable pure logic
  (comma escape/remove) is a tested safe core.

Verification: 78 unit tests (76 lib + 2 bin), SMOKE OK at every merge
point, gates green (baselines re-frozen per module), OWNERSHIP.tsv
refreshed. Two real bugs caught by tests during the work and fixed
before merge (probe-cursor hash clobber in dirblob_name_index;
portable_usleep cross-module private symbol failing clean LTO builds).

Remaining P4-named scope per the original split: `plfsbdev` (shares
`plfsclient/`) — continues as its own phase effort.

Note (post-P4): the mount-only FUSE modules moved from
`plfsmount/src/plfsclient/` to `plfsmount/src/fuse_client/` to stop the
name collision with the shared `plfsclient` crate. Pure rename — no
behavior change; paths in the entries above predate it.

## Exit criteria per phase

- Every module targeted by the phase: migrated or IOU'd-with-transfer.
- `docs/facts/OWNERSHIP.tsv` rows for the phase's crates: zero `UNKNOWN`.
- unsafe-site counter: decreased by at least the phase's migrated count; never increased.
- Divergence-audit and aliasing-hunt items in the phase's crates: struck through (closed) or re-audited against the new code ([methodology.md](methodology.md) correction #4).
- Cluster smoke green; long-soak (P3+) clean under ASAN/TSAN build.
- Scoped human review of the phase's punch-list closed items ([methodology.md](methodology.md) correction #7).

## Global exit (definition of done)

- `#![forbid(unsafe_code)]` everywhere except a small, enumerated set of
  `unsafe`-permitting modules (FFI to libfuse3/zlib/pcap, intrinsics shims),
  each with a `// SAFETY`-documented boundary and a tracked reduction plan.
- Stable toolchain builds all daemons; `c_variadic`/`core_intrinsics` gone.
- unsafe-site counter at its floor and frozen by CI.
- All audit docs in `docs/` intact, closed items struck through, none deleted.
