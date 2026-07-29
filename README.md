# port/ — c2rust machine translation of MooseFS 4.59.2

This directory contains a full c2rust (0.22.1) transpile of the MooseFS
4.59.2 C sources (reference clone: `/tmp/moosefs-ref`) into Rust. It is the
starting baseline of the `mfs-rust-rewrite` effort: get the complete MooseFS
feature set compiling and running as Rust first, then incrementally replace
the unsafe machine-translated code with idiomatic safe Rust.

This is a separate, parallel artifact from the hand-written `crates/plfs-*`
reimplementation. The transpiled daemons speak the genuine MooseFS wire
protocol and use the MooseFS on-disk layout (`chunk_<id>_<version>.mfs`),
not the plfs protocol documented in `docs/protocol.md`.

## Status

Verified working end-to-end (2026-07-29, x86_64 Linux, fuse3 3.18):

| Crate | C source | Build | Smoke test |
| --- | --- | --- | --- |
| `mfsmaster` | `mfsmaster/*.c` + `mfscommon` | `cargo build --release` | `-v` prints 4.59.2-1 build 2106; serves full cluster |
| `mfschunkserver` | `mfschunkserver/*.c` + `mfscommon` | ok | registers with master, stores chunks |
| `mfsmetalogger` | `mfsmetalogger/*.c` + `mfscommon` | ok | `-v` ok |
| `mfsmount` | `mfsclient/*.c` + `libmfsio` | ok (links `fuse3`, `z`) | FUSE mount: write/read md5-verified, mkdir/ln/mv/rm/symlink/df |
| `mfsbdev` | `mfsclient/mfsbdev.c` + `libmfsio` | ok | builds |
| `mfsgui` | `mfsgui/*.c` + `mfscommon` | ok | `-v` ok |
| `mfsnetdump` | `mfsnetdump/mfsnetdump.c` | ok (links `pcap`) | runs (capture needs CAP_NET_RAW) |

Not yet ported: admin/tool binaries (`mfseattr`, `mfsquota`, `mfstrashtool`,
`mfsdiagtools`, `mfsfacl`, `mfsarchive`, `mfspatadmin`, `mfsscadmin`,
`mfssclass`, `mfssnapshots`, `mfstrashretention`, `mfstrashtime`,
`mfsmetadirinfo`, `mfsmetadump`, `mfsmetasearch`, `mfssupervisor`,
`mfsstatsdump`, `mfscsstatsdump`, `mfschunkdbdump`, `mfschunktool`),
`mfstests`, and the Python CGI/CLI (`mfscgi`).

## Building

Requires a recent Rust **nightly** (the transpiled code defines C-variadic
functions, which are not stable yet — `c_variadic` is stabilizing in
Rust 1.99, after which stable Rust will suffice). Each crate carries a
`rust-toolchain.toml` (`channel = "nightly"`). Crates use edition 2024
and build with `lto = true, codegen-units = 1`.

```sh
./build-all.sh        # installs nightly via rustup if missing, builds all 7
# binaries: dist/{mfsmaster,mfschunkserver,mfsmetalogger,mfsgui,mfsmount,mfsbdev,mfsnetdump}
```

Release mode is required: the C code relies on wrapping arithmetic; debug
builds panic on overflow checks.

## Regenerating the transpile

All scratch data lives under `target/mfs-port/` (never `/tmp`):

1. Build MooseFS C out-of-tree to get `compile_commands.json`
   (`target/mfs-port/mfs-build`). Note: copy `buildno.txt` into the build
   dir and fix `config.h` (`#define BUILDNO 2106`) — MooseFS `configure.ac`
   reads `buildno.txt` from the cwd, which breaks out-of-tree builds.
2. Split the db per binary: `python3 port/tools/split_cc.py` (run from the
   build dir; parses `<name>_SOURCES` from each `Makefile.am`, prefers
   binary-prefixed objects over convenience-library ones).
3. Copy each `cc-<bin>.json` to `ccdb-<bin>/compile_commands.json`.
   **Important**: clang 22 treats `-p <file>` as a directory and silently
   falls back to auto-detecting `compile_commands.json` from the cwd,
   which picks the wrong translation-unit variant for files compiled into
   several daemons (e.g. `mfscommon/main.c` resolved to the mfsgui
   variant). Giving each binary its own directory side-steps this.
4. `c2rust transpile ccdb-<bin>/compile_commands.json --output-dir <out>
   -b <main TU> --edition 2024 --deny-unsafe-op-in-unsafe-fn`
   (`main` for daemons using `mfscommon/main.c`;
   `mfsmount`/`mfsbdev`/`mfsgui`... otherwise).
5. `python3 tools/postprocess_stable.py <out>` — drops `#![feature(...)]`
   lines and replaces extern-block opaque types with zero-variant enums.
6. Re-apply the local patches below (bitfields path dep, `_Atomic`
   statics, groups-cache UAF fix, link libs, `core_intrinsics`/`c_variadic`
   feature attrs, `VaList::arg` → `next_arg` rename, `static_mut_refs`
   raw-pointer reads).

## Local patches applied after transpiling

1. `port/vendor/c2rust-bitfields-derive` — upstream derive panics on
   bitfield names that are Rust keywords (`type` → `"r#type"` passed to
   `Ident::new`). Patched to use `Ident::new_raw` for getters and strip the
   `r#` prefix for setters (`set_type`). All crates use this vendored copy
   via path dependency.
2. Dropped `_Atomic` statics — c2rust 0.22.1 drops definitions of file
   statics declared inside `#if HAVE_ATOMICS` blocks while keeping their
   uses. Re-added manually (marked with `ponytail:` comments):
   - `mfsmount`, `mfsbdev`: `src/mfsclient/mastercomm.rs`
     (`rcnt/wcnt/fcnt/rbyt/wbyt`), `readdata.rs` (`total_bytes_rcvd`),
     `writedata.rs` (`total_bytes_sent`).
3. Link libraries via `build.rs`: `mfsmaster`/`mfschunkserver` link `z`;
   `mfsmount` links `fuse3` and `z`; `mfsnetdump` links `pcap`.
4. `mfsgui` uses `-b main` (its `main()` lives in `mfscommon/main.c`).
5. Groups-cache use-after-free fix in `mfsmount/src/mfsclient/getgroups.rs`
   (see commit history; latent MooseFS bug, crashes opendir for root).
6. Edition-2024/stable-toolchain adjustments: `mfs_log_sink` read via
   `&raw const` (static_mut_refs is a hard error in edition 2024),
   `#![feature(c_variadic, core_intrinsics)]` in crate roots,
   `VaList::arg::<T>()` → `next_arg::<T>()` (API rename on recent
   nightlies).

## Known caveats

- Generated code is unsafe, machine-translated C; expect UB-preserving
  semantics, not Rust safety. Nightly-only features (`raw_ref_op`,
  `strict_provenance`, `extern_types`).
- `cargo test` is not meaningful yet; verification is behavioral
  (run a cluster, exercise the mount).
- The transpile reflects MooseFS 4.59.2 build 2106 with the configure
  options of the reference build (`HAVE_FUSE3`, etc.).
