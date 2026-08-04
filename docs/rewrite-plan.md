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

**P2 exception:** `plfsmetalogger` and `plfsgui` share the migrated `cfg`,
`strerr`, and daemon lifecycle implementations. This is confined to these two
P2 daemons: their compile-time differences are represented by daemon-local
configuration/callback tables, while exported symbols remain daemon-local.
Behavior was verified separately for both binaries. This recorded exception
supersedes the general P6-only cross-crate-dedup rule for these modules only;
other divergent convenience-library modules remain per-crate.

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

### Metalogger and GUI complete (2026-08-04)

`plfsmetalogger` and `plfsgui` migrated their divergent `cfg`, `strerr`, and
`main` modules plus `masterconn` and `mfsgui`. Rust now owns configuration
entries, callback registries, timers, connection packets, metadata-download
state, HTTP requests, route data, and CGI child tracking. Safe protocol cores
deny unsafe code; exported C signatures and unavoidable libc/socket/process/
variadic seams remain narrow and SAFETY-documented.

Behavior was checked against MooseFS 4.59.2 source and vectors: config parsing
and libc numeric edges; LIFO lifecycle callbacks, timer catch-up, daemon failure
status, lock-owner handoff, and `LOCK_MEMORY`; fragmented/bad master packets and
registration frames; GET/HEAD, malformed methods/paths, redirects, CGI,
conditional 304, fragmented requests, and live reload. Each daemon crate has
17 release tests. Workspace release build, cluster/FUSE smoke, and migration
gates pass. After review remediation, unsafe ceilings are 46 for metalogger, 97
for GUI, and 449 for `plfscommon`; wrapping floors are 17, 13, and 854.
C-semantic operations remain explicit. Active ownership rows for these daemons
contain no `UNKNOWN`.

Review remediation restored daemon OOM protection, core-dump eligibility,
open-file-limit fallback, bootstrap error forwarding, lock/config diagnostics,
and high-value master-connection warnings. Accepted edge divergences remain:
high-byte garbage config lines warn instead of being ignored; malformed ETags
do not fall through to mtime matching; changelog tail scanning uses a fixed
229376-byte window; `poll` retries `EINTR`; malformed metachange payloads retain
embedded NUL bytes; default-file MD5 handling follows Rust byte-I/O semantics;
`strerr` storage is thread-local; `main_time_refresh` also stamps `USEC_NOW`;
malformed/overflowing numeric IDs fail closed after C-style numeric prefixes;
relative explicit config paths are anchored before the daemon changes directory.
The full legacy warning/error/operational log inventory and callback-name
long-call diagnostics are not reproduced. Omitted logs include some GUI request/
CGI failures and metalogger DNS/socket/connect/timeout, metadata-validation,
download offset/write/CRC/fsync/rename failures. Restored events are daemon
control/startup/resource errors plus metalogger packet rejection, disconnect,
version-gap, old-master, download-close failure, and successful-download summary.
OOM regression vectors lock the enabled-by-default setting and both Linux
procfs fallback targets/values.

`plfscommon::charts` remains verbatim because changing its binary format needs
writer-side proof. Its IOU is transferred to P5
(`plfsmaster::chartsdata`); this wave verified the unchanged GUI reader integration.

### P4 historical checkpoint — initial `plfsmount` sub-scope (2026-07-30)

Historical checkpoint: the `plfsmount` crate (FUSE frontend, 53.3k
transpiled LOC) had 17 modules migrated to safe cores with C ABI boundaries
preserved; 2 modules were documented as annotated unsafe boundaries. Later P4
waves continued this work and changed the current status below.

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

Verification at that checkpoint: 79 unit tests (77 lib + 2 bin), SMOKE OK at
every merge point, gates green (baselines re-frozen per module), and
OWNERSHIP.tsv refreshed. Two real bugs were caught by tests during that work
and fixed before merge (probe-cursor hash clobber in dirblob_name_index;
portable_usleep cross-module private symbol failing clean LTO builds).

P4 overall remains **PARTIAL**. Remaining scope at that checkpoint: migrate the 12-module shared
`plfsclient` crate, then the divergent `plfsbdev` modules. Subsequent waves
have migrated typed ownership/thread pieces in shared `plfsclient` and
`plfsmount`; current residuals are recorded in
[`docs/rust-native-plfsmount-audit.md`](rust-native-plfsmount-audit.md).

Note (post-P4): the mount-only FUSE modules moved from
`plfsmount/src/plfsclient/` to `plfsmount/src/fuse_client/` to stop the
name collision with the shared `plfsclient` crate. Pure rename — no
behavior change; paths in the entries above predate it.

Current-source correction: `dirblob_name_index.rs` and
`dirblob_node_index.rs` were later folded into the owned Rust map implementation
in `fuse_client/dirattrcache.rs`; they are historical module names, not current
source files. The current source map is generated in
[`docs/facts/dedup-map.md`](facts/dedup-map.md).

### P4 continuation — current status (2026-08-02)

Completed in current waves: direct Rust APIs for `inoleng`, `stats`, `csdb`,
`chunksdatacache`, `chunkrwlock`, and `extrapackets`; typed `Arc`/`Weak` and
owned blobs for mount `dirattrcache`; owned `FdEntry` for `fdcache`; Rust
`Arc<Mutex<Sinfo>>` state for stats/params handles; Rust worker threads for
mount and shared helper paths. Current verification: mount lib 72 tests, mount
bin 2 tests, client 12 tests, workspace release build, gates, and cluster/FUSE
smoke all pass at commit `7e4742d`.

Current continuation: `mfs_fuse` dirbuf and finfo raw registries plus pthread
lock/condition protocols have been replaced by typed Rust ownership. Not
complete: pcqueue boundary ownership, shared readdata/writedata backends, and
remaining bdev data-path migration. P4 remains
partial.

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
