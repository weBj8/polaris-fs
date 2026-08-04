# Verified claims corpus — polaris-fs migration

*Adapts `rust-rewrite-verified-claims.md` from the Bun PR #30412 audit set.
This is the fact substrate for the migration: every claim below is cited to
`file:line` or a reproducible command, and survives adversarial review before
any plan section may rely on it. The plan is a derived artifact of these
facts — [methodology.md](methodology.md) rule #1.*

## Schema

Each claim carries four fields:

- **FACT** — a verifiable statement about the current source, with evidence.
- **RUST** — the migration consequence (what the safe-Rust form must be).
- **GATE** — the testable target that proves the migration preserved the fact
  (equivalent to Bun's PERF field; here mostly behavioral, not perf).
- **SRC** — `file:line` or the exact command whose output is the evidence.

Review protocol: **default-deny.** A claim is `confirmed=false` unless the
reviewer re-ran the SRC evidence and reproduced it. Claims are never deleted;
wrong claims are struck through with a correction notice (see the v7
correction in Bun's `LESSONS_LEARNED.md` for the format).

---

## Baseline metrics

### VC-01 — unsafe baseline per crate

- FACT: the transpiled tree contains 8,727 `unsafe fn`/`unsafe {}`/`unsafe
  impl` sites across 498,286 `.rs` LOC in 7 crates: `plfsnetdump` 44 (2,711
  LOC), `plfsmetalogger` 469 (13,480), `plfsgui` 475 (14,178), `plfschunkserver`
  1,288 (97,856), `plfsbdev` 1,403 (103,038), `plfsmount` 1,764 (121,524),
  `plfsmaster` 3,284 (145,499).
- RUST: this is the frozen CI baseline for the unsafe counter gate; the count
  may only decrease.
- GATE: CI job runs the same grep and diffs against the frozen numbers.
- SRC: `grep -rE 'unsafe (fn|\{|impl)' <crate>/src --include='*.rs' | wc -l`
  per crate; `cat <crate>/src/**/*.rs | wc -l` (run 2026-07-30, repo state
  commit `3660e51`).

### VC-02 — mutable-global and FFI-surface scale

- FACT: the tree contains 1,786 `static mut` declarations, 5,151 `extern "C"`
  occurrences, 11,498 `wrapping_*` arithmetic ops, and 12,664 raw-pointer
  casts (`as *mut`/`as *const`) in `plfsmaster/src` alone.
  **Correction (v2):** an earlier revision of this claim said 1,898. That
  count came from a shell glob (`mfs*/src plfsgui/src plfsnetdump/src`) that
  passed `plfsgui/src` and `plfsnetdump/src` twice, double-counting 112 hits.
  The unique-declaration count is 1,786 (668 scalar → STATIC, 621 buffer/struct
  → BUF, 497 pointer → UNKNOWN; per-crate: plfsmaster 727, plfsmount 345,
  plfschunkserver 291, plfsbdev 213, plfsgui 103, plfsmetalogger 98, plfsnetdump 9
  — `tools/gen_ownership.py`).
- RUST: `static mut` is the largest single ownership class; every one must be
  classified in `docs/facts/OWNERSHIP.tsv` (P0). `wrapping_*` sites are
  C-semantics-preserving and must NOT be "simplified" to panicking arithmetic
  ([porting.md](porting.md) type map).
- GATE: P0 exit = zero unclassified `static mut`. CI grep counts reproduced.
- SRC: `grep -rn 'static mut' --include='*.rs' . --exclude-dir=vendor | wc -l`
  = 1,786 (unique; `grep -rn 'static mut' --include='*.rs' . --exclude-dir=vendor
  --exclude-dir=target | wc -l`); `grep -rn 'extern "C"' …` = 5,151; `grep -rn 'wrapping_' …` =
  11,498; `grep -rc 'as \*mut\|as \*const' plfsmaster/src | awk '{s+=$1} END
  {print s}'` = 12,664 (run 2026-07-30).

### VC-03 — nightly feature dependencies

- FACT: every crate root (`<crate>/lib.rs`) and each daemon's main TU
  (`src/plfscommon/main.rs` or the daemon main file) carries
  `#![feature(core_intrinsics)]` and `#![feature(c_variadic)]`.
- RUST: `core_intrinsics` uses are removed per-module as migrated (stable
  `std::hint` / `std::process::abort` equivalents); `c_variadic` retreats to
  the variadic logging leaves until Rust 1.99 stabilizes it. Both features
  are gone by P6 exit.
- GATE: P6 = stable toolchain builds all 7 daemons.
- SRC: `grep -rn '#!\[feature' --include='*.rs' . --exclude-dir=vendor` —
  14 hits, all `core_intrinsics`/`c_variadic` (run 2026-07-30).

### VC-04 — release-mode-only execution

- FACT: the transpiled code relies on wrapping arithmetic; debug builds panic
  on overflow checks.
- RUST: all test/gate infrastructure runs `--release` or with
  `overflow-checks = off`. No migration PR may "fix" a debug overflow panic
  by changing arithmetic semantics.
- GATE: CI smoke jobs set release or the flag explicitly.
- SRC: `README.md` §Building ("Release mode is required: the C code relies on
  wrapping arithmetic; debug builds panic on overflow checks").

## Code duplication facts

### VC-05 — plfscommon duplication map

- FACT: `plfscommon/` exists as per-crate transpiled copies in 6 crates (all
  but `plfsnetdump`). 15 files are **byte-identical** across every crate that
  has them — `clocks.rs`, `crc.rs`, `md5.rs`, `mfslog.rs`, `processname.rs`,
  `sockets.rs`, `timeparser.rs` (6 copies each); `charts.rs`, `cpuusage.rs`,
  `memusage.rs` (2); `conncache.rs`, `labelparser.rs`, `lwthread.rs` (3);
  `delayrun.rs` (2). 5 files **diverge per daemon** — `cfg.rs` (4 distinct /
  4 present), `main.rs` (4/4), `pcqueue.rs` (2 distinct / 4), `random.rs`
  (2/2), `strerr.rs` (2/6). Cause: MooseFS compiles its convenience library
  per-binary with different configure defines, and the transpile ran per
  binary (`README.md` §Regenerating, steps 3–4).
- RUST: ~~P0 de-duplicates the byte-identical files into a shared `plfscommon`
  crate (mechanical, zero behavior risk). Divergent files stay per-crate and
  migrate with their owning daemon.~~ **CLOSED (P0, commit `e167823`):** 14
  modules extracted to `plfscommon/`, consumers use `pub use` reachability so
  fat LTO retains the `#[no_mangle]` symbols. Smoke + gates green after
  extraction.
- GATE: post-dedup, `md5sum` of the shared file equals the md5 of every copy
  it replaced; daemons build and pass smoke unchanged.
- SRC: `md5sum */src/plfscommon/<file>` matrix (run 2026-07-30; full table in
  `docs/facts/dedup-map.md`, to be generated in P0).

### VC-06 — plfsclient duplication map (plfsmount vs plfsbdev)

- FACT: `plfsclient/` is transpiled into both `plfsmount` and `plfsbdev`. 12
  files are byte-identical (`mastercomm.rs`, `readdata.rs`, `writedata.rs`,
  `csdb.rs`, `csorder.rs`, `chunkrwlock.rs`, `chunksdatacache.rs`,
  `extrapackets.rs`, `heapsorter.rs`, `inoleng.rs`, `stats.rs`,
  `truncate.rs`); the rest diverge (FUSE path: `mfs_fuse.rs`,
  `mfs_meta_fuse.rs`, `getgroups.rs`, cache modules — plfsmount-only or
  differing; NBD path: `plfsbdev.rs`, `mfsioint_lookupcache.rs`,
  `squeue.rs`/`workers.rs` in `plfsbdev/src/plfscommon`).
- RUST: ~~P0 moves the 12 identical files into a shared `plfsclient` crate;
  divergent files migrate with their daemon in P4. Without this, every
  client-side fix would land twice.~~ **CLOSED (P0, commit `91a7221`):**
  12 modules extracted to `plfsclient/`; needed `#![feature(core_intrinsics)]`
  (inoleng). Smoke + gates green after extraction.
- GATE: same as VC-05.
- SRC: historical `cmp -s plfsmount/src/plfsclient/<f>
  plfsbdev/src/plfsclient/<f>` sweep (run 2026-07-30); current shared source
  is `plfsclient/src/` and current frontend-only files are listed in
  `docs/facts/dedup-map.md`.

## Translation-artifact facts (local patches the migration must not regress)

### VC-07 — manually re-added `_Atomic` statics

- FACT: c2rust 0.22.1 drops definitions of file statics declared inside
  `#if HAVE_ATOMICS` blocks while keeping their uses. Six were re-added
  manually: `rcnt/wcnt/fcnt/rbyt/wbyt` in `plfsclient/src/mastercomm.rs`,
  `total_bytes_rcvd` in `plfsclient/src/readdata.rs`, and `total_bytes_sent`
  in `plfsclient/src/writedata.rs`.
- RUST: these are statistics counters — the TSV class for each is
  `STATIC → AtomicU64` (relaxed ordering suffices; they are diagnostics).
  Migration of `mastercomm/readdata/writedata` must convert them, not
  re-drop them.
- GATE: `-v`/stats output still increments counters; Miri clean on the
  migrated module.
- SRC: `plfsclient/src/mastercomm.rs`, `plfsclient/src/readdata.rs`, and
  `plfsclient/src/writedata.rs`; shared-crate dependency map in
  `docs/facts/dedup-map.md`.

### VC-08 — vendored bitfields derive patch

- FACT: upstream `c2rust-bitfields-derive` panics on bitfield names that are
  Rust keywords (`type` → `r#type`); all crates use the vendored patched copy
  via path dependency (`vendor/c2rust-bitfields-derive-0.22.1`), using
  `Ident::new_raw` for getters and stripping `r#` for setters.
- RUST: bitfield structs stay on the vendored derive until P6; migrating a
  module containing bitfields means explicit mask/shift accessors or keeping
  the derive — never "upgrade to upstream" mid-phase.
- GATE: modules with bitfields compile; P6 considers upstreaming the patch.
- SRC: `docs/porting.md` bitfield type map;
  `vendor/c2rust-bitfields-derive-0.22.1/`; path dependency in the workspace
  Cargo manifests.

### VC-09 — known latent-bug class evidence (groups-cache UAF)

- FACT: commit `35f00f4` fixed a use-after-free in the groups cache crashing
  `plfsmount` on `opendir` under root workloads — a latent MooseFS bug that
  survived translation, smoke testing, and deployment.
- RUST: instance-level memory bugs exist in the baseline independent of
  translation artifacts. The cache modules of `plfsclient/` (see VC-06
  divergent list) are a priority cluster for the aliasing hunt.
- GATE: the P4 plan must include a re-audit of all `plfsclient` cache modules
  against the fixed `getgroups.rs` pattern.
- SRC: `git show 35f00f4`; `plfsmount/src/fuse_client/getgroups.rs`.

### VC-10 — variadic logging leaves

- FACT: C-variadic functions remain in: `plfscommon/mfslog.rs` (all 6 crates),
  `fuse_client/oplog.rs` (plfsmount), `plfsclient/mfsio.rs` (plfsbdev),
  `plfsmaster/changelog.rs`, `plfsgui/plfsgui.rs`.
- RUST: these are the `c_variadic` confinement leaves
  ([porting.md](porting.md) type map). Safe wrappers get fixed signatures;
  the variadic leaf survives until Rust 1.99.
- GATE: after each crate's migration, `#![feature(c_variadic)]` appears only
  in these leaves' modules.
- SRC: `grep -rln 'VaList' --include='*.rs' . --exclude-dir=vendor` (run
  2026-07-30).

## Build/CI facts

### VC-11 — CI verification pipeline

- FACT: `.github/workflows/release.yml` builds all daemons on push to
  `mfs-rust`; it builds libfuse 3.18 from source (ubuntu-24.04 ships 3.14,
  but `plfsmount` uses `fuse_session_new_versioned` >= 3.17), runs workspace
  tests, migration gates, a cluster/FUSE smoke test, and publishes a verified
  GHCR image only after those jobs pass.
- RUST: release build, tests, gates, and smoke remain required evidence for
  each migration wave; the libfuse-from-source cache steps are reused.
- GATE: current P4 wave passed all jobs at commit `7e4742d`; published image
  tag is `7e4742d21cb9`.
- SRC: `.github/workflows/release.yml`; CI run
  `30742916264`; `docs/rust-native-plfsmount-audit.md` current-wave evidence.

### VC-12 — plfsmount + plfsbdev ownership migration completion

- FACT: the rust-native ownership migration of `plfsmount` and its
  runtime-reachable `plfscommon`/`plfsclient` closure is complete through
  commit `e5769ca` (and `d51d1c7`/`e06d8e9` for plfsbdev): pthread
  mutex/cond/TLS/thread protocols replaced by std sync with guard-slot
  emulation; libc allocation surface reduced to the annotated boundary
  ledger (libfuse mfsopts protocol, processname bootstrap, password leak
  parity, daemon-consumed exports); all intrusive `next`-pointer chains
  converted to typed collections with per-list aliasing analysis and
  independent 1:1 C review.
- FACT: production runs image `e06d8e9cd0c0` (digest
  `sha256:8dc21be5f11b5cb4d840b502a7e6e95108b447c6a2d6b902bbcd6809e62446a7`),
  deployed plfsmount-only after green CI; post-deploy concurrent read/write
  smoke passed.
- SRC: `docs/rust-native-plfsmount-audit.md` (wave log, boundary ledger,
  final scans); `docs/deployment.md`; commits `395c1ca`, `0b66358`,
  `a14c52b`, `d51d1c7`, `e5769ca`.
- GATE: workspace tests (plfsclient 17, plfsmount 95+2, plfsbdev 16),
  release build, smoke, gates all green at `e06d8e9`.

### VC-13 — P2 metalogger + GUI migration completion

- FACT: `plfsmetalogger` and `plfsgui` compile one shared source each for `cfg`,
  `strerr`, and daemon lifecycle, while keeping daemon-local C symbols. These
  modules plus metalogger `masterconn` and GUI `mfsgui` use Rust-owned state.
  Safe protocol cores deny unsafe code; remaining unsafe is limited to libc,
  socket, process, MD5/CRC, stdio, and variadic ABI seams.
- FACT: exported master/GUI symbols and C allocation contracts are preserved;
  config strings and `cfg_buff` remain `free()`-compatible. Wire framing stays
  big-endian and metadata/chart formats are unchanged.
- FACT: P2 release tests cover config syntax/numeric edges, lifecycle ordering,
  daemon failure and lock handoff, master packet fragmentation/control paths,
  HTTP GET/HEAD/error/redirect/CGI/conditional/fragmented-request behavior.
  Live GUI GET returned 200 and POST returned 405.
- GATE: P2 crates have 18 tests each; mount has 95 library tests; locked release
  workspace build, cluster/FUSE smoke (`SMOKE OK`), and migration gates pass.
  Unsafe floors are metalogger 47, GUI 97, and shared `plfscommon` 432;
  wrapping floors are 15, 13, and 854 respectively.
- SRC: `target/moosefs-ref` tag `v4.59.2` (`ac106b2`);
  `plfscommon/src/p2_{cfg,daemon_main,strerr}.rs`;
  `plfsmetalogger/src/plfscommon/main.rs`;
  `plfsmetalogger/src/plfsmetalogger/{masterconn,masterconn_core}.rs`;
  `plfsgui/src/plfscommon/main.rs`;
  `plfsgui/src/plfsgui/{mfsgui,mfsgui_core}.rs`.

---

## IOU ledger

*Every `// MIGRATION-IOU: blocked_on: X::Y` in the tree has a row here.
Columns: item · blocking crate/module · reason · created (phase) · consumed
(phase) · status. Open IOUs block phase exit unless transferred.*

| item | blocked_on | reason | created | consumed | status |
| --- | --- | --- | --- | --- | --- |
| plfscommon::charts | P5 (`plfsmaster::chartsdata`) | 5.2k lines of binary chart-format I/O + rendering; P2 verified unchanged GUI integration, but migration needs writer-side format proof | P1 (2026-07-30) | P5 | transferred to P5 (2026-08-04) |
