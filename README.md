<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/assets/polaris-fs-logo-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/assets/polaris-fs-logo.svg">
    <img src="docs/assets/polaris-fs-logo.svg" width="420" alt="PolarisFS">
  </picture>
</p>
<p align="center">
  Distributed storage moving from machine-translated C to memory-safe Rust.
</p>
<p align="center">
  <a href="https://github.com/weBj8/polaris-fs/actions/workflows/release.yml"><img alt="Build" src="https://img.shields.io/github/actions/workflow/status/weBj8/polaris-fs/release.yml?branch=mfs-rust&style=flat-square&label=build"></a>
  <a href="https://github.com/weBj8/polaris-fs/pkgs/container/polaris-fs"><img alt="Container image" src="https://img.shields.io/github/actions/workflow/status/weBj8/polaris-fs/release.yml?branch=mfs-rust&style=flat-square&label=container&logo=docker"></a>
  <a href="rust-toolchain.toml"><img alt="Rust nightly" src="https://img.shields.io/badge/rust-nightly-E05D44?style=flat-square&logo=rust&logoColor=white"></a>
  <a href="LICENSE"><img alt="GPL-2.0 license" src="https://img.shields.io/badge/license-GPL--2.0-16815D?style=flat-square"></a>
  <a href="docs/rewrite-plan.md"><img alt="Migration P4 partial" src="https://img.shields.io/badge/migration-P4%20partial-078B99?style=flat-square"></a>
</p>
<p align="center">
  <a href="docs/deployment.md">Deploy</a> ·
  <a href="docs/README.md">Migration docs</a> ·
  <a href="docs/verified-claims.md">Verified claims</a> ·
  <a href="https://github.com/weBj8/polaris-fs/pkgs/container/polaris-fs">Container images</a>
</p>

PolarisFS is a working distributed filesystem undergoing an incremental
memory-safety migration. Each merge replaces part of the c2rust baseline with
idiomatic safe Rust while freezing C ABI boundaries, wire protocol, disk
format, and observable behavior.

Current daemons form a working cluster and remain wire- and disk-compatible
with MooseFS 4.59.2. Reference tests, migration counters, and an end-to-end
FUSE smoke test guard each migrated module.

## Quickstart

Single-host Docker Compose deployment: master, metalogger, chunkserver, FUSE
mount, and GUI. Set `PLFS_MASTER_HOST` to this host's LAN IP, not loopback:

```bash
export PLFS_MASTER_HOST=192.168.1.10
export PLFS_IMAGE=ghcr.io/webj8/polaris-fs:latest
sudo install -d /var/lib/polarisfs/{master,metalogger,chunkserver,chunks,gui} /mnt/plfs
mountpoint -q /mnt/plfs || sudo mount --bind /mnt/plfs /mnt/plfs
sudo mount --make-rshared /mnt/plfs
sudo -E docker compose -f docker/docker-compose.single.yml up -d
findmnt /mnt/plfs
```

Files appear at `/mnt/plfs`; GUI listens on `http://$PLFS_MASTER_HOST:9425`.
Default exports allow read-write access, so use this only on a trusted network
until exports are restricted. Multi-host Docker/Podman setup, immutable tags,
backup, upgrade, and rollback: [deployment guide](docs/deployment.md).

## Migration status

| Phase | Scope | State |
| --- | --- | --- |
| P0 | Facts, gates, shared-crate deduplication | **Done** |
| P1 | Shared `plfscommon` | **Done** — 12/14 modules migrated |
| P2 | `plfsmetalogger`, `plfsgui` | Pending |
| P3 | `plfschunkserver` | Pending |
| P4 | `plfsmount`, shared `plfsclient`, `plfsbdev` | **Partial** — mount/client ownership and thread waves migrated; mount has 72 lib tests + 2 bin tests; `mfs_fuse` dirbuf/finfo locks, groups/queue boundaries, and bdev data path remain |
| P5 | `plfsmaster` | Pending |
| P6 | Stable toolchain and remaining boundaries | Pending |

Detailed scope, exit criteria, and status history live in
[rewrite-plan.md](docs/rewrite-plan.md).

## Build and verify

Requires x86_64 Linux, pinned Rust nightly from `rust-toolchain.toml`, zlib,
libpcap, and libfuse 3.17 or newer.
Set `FUSE_LIB` only when libfuse is outside system search paths:

```bash
FUSE_LIB=/path/to/libfuse3/lib
cargo test --release -p plfsmount --lib
RUSTFLAGS="-L $FUSE_LIB" cargo build --release --workspace --locked
LD_LIBRARY_PATH=$FUSE_LIB bash tools/smoke_test.sh
bash tools/gates/check.sh
```

CI runs migration gates, workspace tests, and release build in parallel. After
the release artifact is ready, container build and full cluster + FUSE smoke
run in parallel; GHCR publish waits for every gate. Successful runs publish
`latest` and commit tags to `ghcr.io/webj8/polaris-fs`:

```bash
docker pull ghcr.io/webj8/polaris-fs:latest
```

## Workspace

| Path | Role |
| --- | --- |
| [`plfscommon/`](plfscommon/) | Shared common modules; P1 safe migration |
| [`plfsclient/`](plfsclient/) | Shared client backend; deduplicated, partial Rust-native migration |
| [`plfsmount/`](plfsmount/) | FUSE frontend; migrated cache/thread cores plus annotated FUSE state-machine boundaries |
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
