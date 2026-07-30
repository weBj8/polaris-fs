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
  impl` sites across 498,286 `.rs` LOC in 7 crates: `mfsnetdump` 44 (2,711
  LOC), `mfsmetalogger` 469 (13,480), `mfsgui` 475 (14,178), `mfschunkserver`
  1,288 (97,856), `mfsbdev` 1,403 (103,038), `mfsmount` 1,764 (121,524),
  `mfsmaster` 3,284 (145,499).
- RUST: this is the frozen CI baseline for the unsafe counter gate; the count
  may only decrease.
- GATE: CI job runs the same grep and diffs against the frozen numbers.
- SRC: `grep -rE 'unsafe (fn|\{|impl)' <crate>/src --include='*.rs' | wc -l`
  per crate; `cat <crate>/src/**/*.rs | wc -l` (run 2026-07-30, repo state
  commit `3660e51`).

### VC-02 — mutable-global and FFI-surface scale

- FACT: the tree contains 1,786 `static mut` declarations, 5,151 `extern "C"`
  occurrences, 11,498 `wrapping_*` arithmetic ops, and 12,664 raw-pointer
  casts (`as *mut`/`as *const`) in `mfsmaster/src` alone.
  **Correction (v2):** an earlier revision of this claim said 1,898. That
  count came from a shell glob (`mfs*/src mfsgui/src mfsnetdump/src`) that
  passed `mfsgui/src` and `mfsnetdump/src` twice, double-counting 112 hits.
  The unique-declaration count is 1,786 (668 scalar → STATIC, 621 buffer/struct
  → BUF, 497 pointer → UNKNOWN; per-crate: mfsmaster 727, mfsmount 345,
  mfschunkserver 291, mfsbdev 213, mfsgui 103, mfsmetalogger 98, mfsnetdump 9
  — `tools/gen_ownership.py`).
- RUST: `static mut` is the largest single ownership class; every one must be
  classified in `docs/facts/OWNERSHIP.tsv` (P0). `wrapping_*` sites are
  C-semantics-preserving and must NOT be "simplified" to panicking arithmetic
  ([porting.md](porting.md) type map).
- GATE: P0 exit = zero unclassified `static mut`. CI grep counts reproduced.
- SRC: `grep -rn 'static mut' --include='*.rs' . --exclude-dir=vendor | wc -l`
  = 1,786 (unique; `grep -rn 'static mut' --include='*.rs' . --exclude-dir=vendor
  --exclude-dir=target | wc -l`); `grep -rn 'extern "C"' …` = 5,151; `grep -rn 'wrapping_' …` =
  11,498; `grep -rc 'as \*mut\|as \*const' mfsmaster/src | awk '{s+=$1} END
  {print s}'` = 12,664 (run 2026-07-30).

### VC-03 — nightly feature dependencies

- FACT: every crate root (`<crate>/lib.rs`) and each daemon's main TU
  (`src/mfscommon/main.rs` or the daemon main file) carries
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

### VC-05 — mfscommon duplication map

- FACT: `mfscommon/` exists as per-crate transpiled copies in 6 crates (all
  but `mfsnetdump`). 15 files are **byte-identical** across every crate that
  has them — `clocks.rs`, `crc.rs`, `md5.rs`, `mfslog.rs`, `processname.rs`,
  `sockets.rs`, `timeparser.rs` (6 copies each); `charts.rs`, `cpuusage.rs`,
  `memusage.rs` (2); `conncache.rs`, `labelparser.rs`, `lwthread.rs` (3);
  `delayrun.rs` (2). 5 files **diverge per daemon** — `cfg.rs` (4 distinct /
  4 present), `main.rs` (4/4), `pcqueue.rs` (2 distinct / 4), `random.rs`
  (2/2), `strerr.rs` (2/6). Cause: MooseFS compiles its convenience library
  per-binary with different configure defines, and the transpile ran per
  binary (`README.md` §Regenerating, steps 3–4).
- RUST: P0 de-duplicates the byte-identical files into a shared `mfscommon`
  crate (mechanical, zero behavior risk). Divergent files stay per-crate and
  migrate with their owning daemon.
- GATE: post-dedup, `md5sum` of the shared file equals the md5 of every copy
  it replaced; daemons build and pass smoke unchanged.
- SRC: `md5sum */src/mfscommon/<file>` matrix (run 2026-07-30; full table in
  `docs/facts/dedup-map.md`, to be generated in P0).

### VC-06 — mfsclient duplication map (mfsmount vs mfsbdev)

- FACT: `mfsclient/` is transpiled into both `mfsmount` and `mfsbdev`. 12
  files are byte-identical (`mastercomm.rs`, `readdata.rs`, `writedata.rs`,
  `csdb.rs`, `csorder.rs`, `chunkrwlock.rs`, `chunksdatacache.rs`,
  `extrapackets.rs`, `heapsorter.rs`, `inoleng.rs`, `stats.rs`,
  `truncate.rs`); the rest diverge (FUSE path: `mfs_fuse.rs`,
  `mfs_meta_fuse.rs`, `getgroups.rs`, cache modules — mfsmount-only or
  differing; NBD path: `mfsbdev.rs`, `mfsioint_lookupcache.rs`,
  `squeue.rs`/`workers.rs` in `mfsbdev/src/mfscommon`).
- RUST: P0 moves the 12 identical files into a shared `mfsclient` crate;
  divergent files migrate with their daemon in P4. Without this, every
  client-side fix would land twice.
- GATE: same as VC-05.
- SRC: `cmp -s mfsmount/src/mfsclient/<f> mfsbdev/src/mfsclient/<f>` sweep
  (run 2026-07-30).

## Translation-artifact facts (local patches the migration must not regress)

### VC-07 — manually re-added `_Atomic` statics

- FACT: c2rust 0.22.1 drops definitions of file statics declared inside
  `#if HAVE_ATOMICS` blocks while keeping their uses. Six were re-added
  manually: `rcnt/wcnt/fcnt/rbyt/wbyt` in `mfsmount/src/mfsclient/mastercomm.rs`
  and `mfsbdev/src/mfsclient/mastercomm.rs`, `total_bytes_rcvd` in
  `readdata.rs`, `total_bytes_sent` in `writedata.rs` (both crates).
- RUST: these are statistics counters — the TSV class for each is
  `STATIC → AtomicU64` (relaxed ordering suffices; they are diagnostics).
  Migration of `mastercomm/readdata/writedata` must convert them, not
  re-drop them.
- GATE: `-v`/stats output still increments counters; Miri clean on the
  migrated module.
- SRC: `README.md` §Local patches, item 2; `grep -n 'ponytail:'
  mfsmount/src/mfsclient/mastercomm.rs`.

### VC-08 — vendored bitfields derive patch

- FACT: upstream `c2rust-bitfields-derive` panics on bitfield names that are
  Rust keywords (`type` → `r#type`); all crates use the vendored patched copy
  via path dependency (`vendor/c2rust-bitfields-derive-0.22.1`), using
  `Ident::new_raw` for getters and stripping `r#` for setters.
- RUST: bitfield structs stay on the vendored derive until P6; migrating a
  module containing bitfields means explicit mask/shift accessors or keeping
  the derive — never "upgrade to upstream" mid-phase.
- GATE: modules with bitfields compile; P6 considers upstreaming the patch.
- SRC: `README.md` §Local patches, item 1; `vendor/c2rust-bitfields-derive-0.22.1/`.

### VC-09 — known latent-bug class evidence (groups-cache UAF)

- FACT: commit `35f00f4` fixed a use-after-free in the groups cache crashing
  `mfsmount` on `opendir` under root workloads — a latent MooseFS bug that
  survived translation, smoke testing, and deployment.
- RUST: instance-level memory bugs exist in the baseline independent of
  translation artifacts. The cache modules of `mfsclient/` (see VC-06
  divergent list) are a priority cluster for the aliasing hunt.
- GATE: the P4 plan must include a re-audit of all `mfsclient` cache modules
  against the fixed `getgroups.rs` pattern.
- SRC: `git show 35f00f4`; `mfsmount/src/mfsclient/getgroups.rs`.

### VC-10 — variadic logging leaves

- FACT: C-variadic functions remain in: `mfscommon/mfslog.rs` (all 6 crates),
  `mfsclient/oplog.rs` (mfsmount), `mfsclient/mfsio.rs` (mfsbdev),
  `mfsmaster/changelog.rs`, `mfsgui/mfsgui.rs`.
- RUST: these are the `c_variadic` confinement leaves
  ([porting.md](porting.md) type map). Safe wrappers get fixed signatures;
  the variadic leaf survives until Rust 1.99.
- GATE: after each crate's migration, `#![feature(c_variadic)]` appears only
  in these leaves' modules.
- SRC: `grep -rln 'VaList' --include='*.rs' . --exclude-dir=vendor` (run
  2026-07-30).

## Build/CI facts

### VC-11 — existing CI is build-only

- FACT: `.github/workflows/release.yml` builds all daemons on push to
  `mfs-rust`; it builds libfuse 3.18 from source (ubuntu-24.04 ships 3.14,
  but `mfsmount` uses `fuse_session_new_versioned` ≥ 3.17) and runs no tests.
- RUST: the P0 gate work (unsafe counter, IOU counter, smoke test) extends
  this workflow; the libfuse-from-source cache steps are reused.
- GATE: P0 exit = all new gates green on the unmigrated baseline.
- SRC: `.github/workflows/release.yml` (libfuse comment at the cache step).

---

## IOU ledger

*Every `// MIGRATION-IOU: blocked_on: X::Y` in the tree has a row here.
Columns: item · blocking crate/module · reason · created (phase) · consumed
(phase) · status. Open IOUs block phase exit unless transferred.*

| item | blocked_on | reason | created | consumed | status |
| --- | --- | --- | --- | --- | --- |
| — | — | — | — | — | — (empty as of P0 start) |
