<p align="center">
  <img src="docs/assets/polaris-fs.svg" width="128" alt="polaris-fs logo">
</p>
<h1 align="center">polaris-fs</h1>
<p align="center">
  Memory-safe Rust migration of MooseFS 4.59.2
</p>
<p align="center">
  <a href="https://github.com/weBj8/polaris-fs/actions/workflows/release.yml"><img alt="Build" src="https://img.shields.io/github/actions/workflow/status/weBj8/polaris-fs/release.yml?branch=mfs-rust&style=flat-square&label=build"></a>
  <a href="rust-toolchain.toml"><img alt="Rust nightly" src="https://img.shields.io/badge/rust-nightly-E05D44?style=flat-square&logo=rust&logoColor=white"></a>
  <a href="LICENSE"><img alt="GPL-2.0 license" src="https://img.shields.io/badge/license-GPL--2.0-2F855A?style=flat-square"></a>
  <a href="docs/rewrite-plan.md"><img alt="Migration P4 partial" src="https://img.shields.io/badge/migration-P4%20partial-2B6CB0?style=flat-square"></a>
</p>

polaris-fs incrementally replaces a complete c2rust translation of MooseFS
4.59.2 with idiomatic safe Rust. C ABI boundaries, wire protocol, disk format,
and observable behavior stay frozen at each merge point.

Current daemons form a working cluster and interoperate with MooseFS 4.59.2.
Reference tests, migration counters, and an end-to-end FUSE smoke test guard
each migrated module. See [migration docs](docs/README.md) for methodology and
evidence.

## Migration status

| Phase | Scope | State |
| --- | --- | --- |
| P0 | Facts, gates, shared-crate deduplication | **Done** |
| P1 | Shared `plfscommon` | **Done** — 12/14 modules migrated |
| P2 | `plfsmetalogger`, `plfsgui` | Pending |
| P3 | `plfschunkserver` | Pending |
| P4 | `plfsmount`, shared `plfsclient`, `plfsbdev` | **Partial** — `plfsmount` done: 17 safe cores, 2 annotated boundaries, 79 tests; `plfsclient` and `plfsbdev` remain |
| P5 | `plfsmaster` | Pending |
| P6 | Stable toolchain and remaining boundaries | Pending |

Detailed scope, exit criteria, and status history live in
[rewrite-plan.md](docs/rewrite-plan.md).

## Build and verify

Requires x86_64 Linux, Rust nightly, zlib, libpcap, and libfuse 3.17 or newer.
Set `FUSE_LIB` only when libfuse is outside system search paths:

```bash
FUSE_LIB=/path/to/libfuse3/lib
cargo test --release -p plfsmount --lib
RUSTFLAGS="-L $FUSE_LIB" cargo build --release --workspace --locked
LD_LIBRARY_PATH=$FUSE_LIB bash tools/smoke_test.sh
bash tools/gates/check.sh
```

CI runs workspace build, migration gates, executable checks, and full cluster +
FUSE smoke test on every push to `mfs-rust`.

## Workspace

| Path | Role |
| --- | --- |
| [`plfscommon/`](plfscommon/) | Shared common modules; P1 safe migration |
| [`plfsclient/`](plfsclient/) | Shared client backend; deduplicated, migration pending |
| [`plfsmount/`](plfsmount/) | FUSE frontend; safe cores plus two annotated boundaries |
| [`plfsbdev/`](plfsbdev/) | Block-device frontend |
| [`plfsmaster/`](plfsmaster/) | Metadata master |
| [`plfschunkserver/`](plfschunkserver/) | Chunk storage daemon |
| [`plfsmetalogger/`](plfsmetalogger/) | Metadata replication daemon |
| [`plfsgui/`](plfsgui/) | Web UI daemon |
| [`tools/`](tools/) | Regression gates and cluster smoke test |

## Documentation

- [Methodology](docs/methodology.md): binding migration rules
- [Rewrite plan](docs/rewrite-plan.md): phases, status, and exit criteria
- [Porting guide](docs/porting.md): c2rust-to-safe-Rust module pattern
- [Verified claims](docs/verified-claims.md): sourced facts and IOU ledger
- [Ownership facts](docs/facts/): unsafe ownership and FFI inventories
- [P1 retrospective](docs/p1-retrospective.md): pilot results and calibration

## License

GPL-2.0. See [LICENSE](LICENSE).
