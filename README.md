# polaris-fs

## Goal

polaris-fs aims to build a memory-safe clustered filesystem with the same core
functionality as MooseFS. The project starts from a complete c2rust (0.22.1)
translation of the MooseFS 4.59.2 C sources, then gradually replaces unsafe,
machine-translated C code with idiomatic, memory-safe Rust.

This is an incremental migration rather than a from-scratch rewrite. Similar
to Bun's gradual migration of systems components from Zig to Rust, each module
is replaced behind existing behavior boundaries, verified, and merged before
the next module is changed. Reference tests, migration gates, and end-to-end
cluster smoke tests keep behavior stable at each merge point while unsafe code
is removed step by step.

Compatibility here means functional parity with MooseFS: polaris-fs should
provide the same filesystem capabilities and, during the current migration,
interoperate with MooseFS 4.59.2 through its wire protocol and on-disk formats.
It is not a promise that future polaris-fs APIs, ABIs, command-line interfaces,
or management interfaces will remain compatible with MooseFS. Those interfaces
may evolve as the Rust implementation becomes safer and more idiomatic.

The current transpiled daemons speak the MooseFS wire protocol and interoperate
with stock MooseFS 4.59.2 nodes. The current on-disk layout includes
`chunk_<id>_<version>.mfs` and MooseFS metadata images.

**Migration methodology, plan, and live audit documents: [docs/](docs/README.md)**
(reading order: `docs/methodology.md` → `docs/rewrite-plan.md`).

## Migration status

| Phase | Scope | State |
| --- | --- | --- |
| P0 | facts (`docs/facts/`), CI gates, dedup of byte-identical `plfscommon`/`plfsclient` copies into shared crates | **done** — unsafe 8,727 → 7,105 by duplicate elimination |
| P1 | `plfscommon` shared crate → safe Rust | **done** — 12/14 modules safe-ified (336 → 244 unsafe sites), 11 unit tests, `charts` IOU'd to P2/P5, see `docs/p1-retrospective.md` |
| P2 | `plfsmetalogger`, `plfsgui` | next |
| P3 | `plfschunkserver` | |
| P4 | `plfsmount` + `plfsbdev` (shared `plfsclient` crate) | |
| P5 | `plfsmaster` | |
| P6 | nightly-feature removal, stable toolchain, `plfsnetdump` | |

Transpile baseline verified working end-to-end (2026-07-29, x86_64 Linux,
fuse3 3.18): master serves a full cluster, chunkserver stores chunks,
metalogger replicates, FUSE mount passes md5-verified write/read +
mkdir/ln/mv/rm/symlink/df. Not yet ported: admin/tool binaries (`mfseattr`,
`mfsquota`, …), `mfstests`, Python CGI/CLI (`plfscgi`).

## Workspace layout

```
plfscommon/     shared crate: 14 plfscommon modules (deduped P0, migrated P1)
plfsclient/     shared crate: 12 client modules common to plfsmount/plfsbdev
plfsmaster/     daemon crate (own plfscommon divergent files + plfsmaster/*.rs)
plfschunkserver/  likewise
plfsmetalogger/  likewise
plfsmount/       likewise (links fuse3, z)
plfsbdev/        likewise
plfsgui/         likewise
plfsnetdump/     standalone (links pcap)
vendor/         patched c2rust-bitfields-derive (load-bearing, README §patches)
tools/          gates, smoke test, fact generators, dedup scripts
docs/           methodology, plan, porting guide, audit + facts corpus
```

c2rust transpiles each C TU as a *standalone* module wired at link time by
symbol (`#[no_mangle] extern "C"`), so shared modules are consumed as path
dependencies whose symbols the daemons' own extern blocks resolve against.

## Building

Requires recent Rust **nightly** (`c_variadic`, `core_intrinsics`; selected by
the root `rust-toolchain.toml`). The workspace disables debug overflow checks
because the translated C code relies on wrapping arithmetic.

```sh
cargo build                    # builds the full workspace -> target/debug/
cargo build --release --locked # production binaries -> target/release/
```

Native deps: zlib, libpcap (plfsnetdump only), and **libfuse3 ≥ 3.17** for
plfsmount (`fuse_session_new_versioned`). Distros shipping older fuse: CI
builds 3.18 from source; locally the same build lives under
`target/fuse318/` (see CI workflow). Link with
`RUSTFLAGS="-L <fuse-lib-dir>"` if the system one is too old.

## Testing and gates

```sh
bash tools/smoke_test.sh       # full cluster gate: master+chunkserver+metalogger
                               # + FUSE mount, md5-verified ops (root or unshare -rm)
bash tools/gates/check.sh      # unsafe/IOU/SAFETY/wrapping counters vs frozen baseline
cargo test -p plfscommon        # unit tests for migrated shared modules
```

CI (`.github/workflows/release.yml`) runs the counter gates, builds all
daemons, and runs the cluster smoke on every push to `mfs-rust`.

## Documentation

- [docs/README.md](docs/README.md) — doc-set index and house rules
- [docs/methodology.md](docs/methodology.md) — binding rules (adapted from the Bun PR #30412 audit)
- [docs/rewrite-plan.md](docs/rewrite-plan.md) — phases P0–P6, exit criteria
- [docs/porting.md](docs/porting.md) — the c2rust-unsafe → safe Rust coding standard
- [docs/verified-claims.md](docs/verified-claims.md) — fact corpus + IOU ledger
- [docs/facts/](docs/facts/) — OWNERSHIP.tsv, dedup map, FFI boundaries
- [docs/p1-retrospective.md](docs/p1-retrospective.md) — pilot calibration results

House rule (methodology #7): these documents are never deleted; closed items
are struck through, migration PRs use merge commits not squashes.

## Regenerating the transpile

Scratch data lives under `target/mfs-port/` (never `/tmp`). The full recipe
is in git history (`port/README` era): build MooseFS C out-of-tree for
`compile_commands.json`, split per binary with `tools/split_cc.py`, run
c2rust per binary, `tools/postprocess_stable.py`, then re-apply the local
patches (vendored bitfields derive, `_Atomic` statics, link libs) **and**
re-run the dedup extraction (`tools/extract_plfscommon.sh` + the plfsclient
equivalent) — regeneration restores duplicated copies.

Known local patches after transpiling:

1. `vendor/c2rust-bitfields-derive` — `Ident::new_raw` for keyword bitfield
   names. All crates use the vendored copy via path dependency.
2. Re-added `_Atomic` statics c2rust drops (plfsmount/plfsbdev
   `mastercomm.rs`, `readdata.rs`, `writedata.rs`; `ponytail:` comments).
3. Link libs via `build.rs`: plfsmaster/plfschunkserver `z`, plfsmount
   `fuse3`+`z`, plfsnetdump `pcap`.
4. Groups-cache UAF fix in `plfsmount/src/fuse_client/getgroups.rs`
   (latent MooseFS bug; commit `35f00f4`).
5. Edition-2024/nightly adjustments: `&raw const` for the log sink,
   `c_variadic`/`core_intrinsics` feature attrs, `VaList::arg` → `next_arg`.

## Known caveats

- Unmigrated modules are unsafe machine-translated C; expect UB-preserving
  semantics, not Rust safety. Migrated modules carry `// SAFETY:`-annotated
  FFI shims at their C ABI boundary.
- Verification is behavioral (cluster smoke) + module unit tests;
  `cargo test` is meaningful only for migrated crates (plfscommon today).
- The transpile reflects MooseFS 4.59.2 build 2106 with the reference
  build's configure options (`HAVE_FUSE3`, etc.).
