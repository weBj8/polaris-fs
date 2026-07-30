# AGENTS.md — guide for coding agents working on polaris-fs

polaris-fs is the c2rust machine translation of **MooseFS 4.59.2** (~498k
LOC unsafe Rust) being migrated to idiomatic safe Rust, module by module,
behavior frozen at every merge point. License: GPLv2 (see LICENSE).

## Layout

| Path | What |
| --- | --- |
| `mfscommon/` | shared crate: safe-migrated common modules (P1) |
| `mfsclient/` | shared crate: transpiled client backend modules (dedup P0) |
| `mfsmount/` | FUSE frontend (P4: safe cores + 2 annotated boundaries) |
| `mfsmaster/` `mfschunkserver/` `mfsmetalogger/` `mfsgui/` `mfsbdev/` `mfsnetdump/` | daemons, still transpiled c2rust output |
| `docs/` | migration methodology, plan, verified-claims, facts (OWNERSHIP.tsv, dedup-map, ffi-boundaries) |
| `tools/gates/` | regression gates (check.sh + baseline.tsv) |
| `tools/smoke_test.sh` | end-to-end cluster + FUSE smoke test |

## Build / verify (always in this order)

```bash
cd mfsmount && cargo test --release --lib          # unit tests (78)
RUSTFLAGS="-L $PWD/target/fuse318/fuse-3.18.2/build/lib" ./build-all.sh
LD_LIBRARY_PATH=$PWD/target/fuse318/fuse-3.18.2/build/lib bash tools/smoke_test.sh   # SMOKE OK
bash tools/gates/check.sh                          # exit 0
```

- mfsmount needs libfuse ≥ 3.17; the prebuilt one lives at
  `target/fuse318/fuse-3.18.2/build/lib` (RUSTFLAGS at build,
  LD_LIBRARY_PATH at runtime).
- mfsnetdump needs `libpcap-dev` (missing locally; CI has it).
- Baselines: after an intentional metrics change, re-freeze
  `tools/gates/baseline.tsv` in the SAME commit.

## Migration pattern (established, follow it)

- Port from the C original in `/tmp/moosefs-ref` (if present), NOT from
  the transpile — c2rust output is ~60-80% macro-expansion noise.
- Safe core in `#[deny(unsafe_code)] mod imp`; C ABI exports stay as thin
  boundary fns with SAFETY comments. Consumers link by symbol — never
  change exported signatures.
- C quirks are behavior: preserve exact overflow/wrapping, threshold
  comparisons (`<` vs `<=`), timestamp stamping, and refcount protocols.
  Write reference tests for parsers/serializers against C semantics.
- Genuine C-only modules (kernel callbacks, process bootstrap) become
  **annotated boundaries**: module-header justification, not forced
  rewrites (see mfs_fuse.rs, mfsmount.rs, mfscommon/sockets.rs).

## Traps learned the hard way (don't repeat)

1. **Stale cargo caches lie.** A passing incremental build does not prove
   a clean build passes. Before pushing: `cargo clean` the crate and
   rebuild. (P4 CI failure: `portable_usleep` was declared extern but
   only defined as a *private* fn in another module — invisible under
   LTO on clean builds. Per-TU private symbols are NOT linkable;
   c2rust's model gives each module its own copy. Mirror that or use
   `#[unsafe(no_mangle)]`.)
2. **Tests in the mfsmount *binary* target** (`src/fuse_client/mfsmount.rs`
   is the bin root, not the lib) only run with `cargo test` /
   `--bins`, not `--lib`.
3. **Probe cursors ≠ content keys.** When porting hash tables that cache
   hashes, never let the probe-walk variable alias the stored hash
   (P4 dirblob_name_index bug: stored probe-mutated hash → index
   corruption on first rehash).
4. **`unused unsafe` cleanup:** `tools/strip_unused_unsafe.py` strips
   c2rust's blanket `unsafe {}` wrappers per rustc JSON diagnostics.
5. Scratch data under `target/`, never `/tmp` (CI runners share /tmp).

## Git flow

- Work on `pN/<phase>` branches; merge to `mfs-rust` with **merge
  commits** (`--no-ff`, never squash — docs/history must survive).
- Push both the phase branch and `mfs-rust`; CI (Build workflow) must be
  green on mfs-rust before the phase is called done.
- Docs are never deleted; closed items are struck through.

## Methodology docs

Start at `docs/README.md` → `methodology.md`, `rewrite-plan.md`
(phase status log), `verified-claims.md` (VC ledger + IOUs),
`docs/facts/` (OWNERSHIP.tsv static-mut inventory).
