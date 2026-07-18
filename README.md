# PolarisFS (porfs)

An open-source, modern GPFS. We copy GPFS's principles and full feature set — not its
technology stack. No kernel module, no 1990s assumptions, no closed source.

- **Client**: FUSE (no kmod) + io_uring + passthrough — modern kernel features, not mmfs
- **Data plane**: io_uring end-to-end (extent store / RPC / bench)
- **Language**: Rust for everything, Zig for hot-path components (P20+)
- **Differentiator**: CRC32C end-to-end checksums + built-in scrub — the data hygiene
  that GPFS, BeeGFS, and Lustre all lack
- **Target**: shared POSIX namespace for 10–50 node clusters; NVMe speed, NFS-grade ops

## Status

| Phase | Content | Result |
|---|---|---|
| P1 | On-disk format v0 + io_uring extent store + bench | ✅ write = 94% of raw fio, verified read = 79% of raw fio (fio does no checksums), 34 tests green |
| P2 | Checkpoint fast-mount + group commit + kill -9 crash recovery | ✅ **1000 kill -9 iterations, zero corruption**, 100% checkpoint mounts, 63s |
| P3 | MDS v0: namespace + inodes + file→extent map (redb, transactional) | ✅ crash-survival loops + self-check, 92 workspace tests green |
| P4 | FUSE client v0: real mount, basic POSIX ops | ✅ 11 real-mount integration tests, 103 workspace tests green |
| P5 | POSIX completion I: xattr, symlink, mknod, sparse, fsync/fdatasync, rename matrix; hash-ordered big-directory index | ✅ 114 tests green; 1M-entry dir lists correctly (`ls -f` 2.0s release vs xfs 0.2s); known-issue: non-mounter-uid EACCES |
| P6 | MVP freeze: one-command format+mount; real-workload smoke | ✅ git clone/fsck + busybox build + sqlite WAL stress, all zero data errors on a real mount |
| P7 | RPC + chunkserver: wire protocol v1, extent I/O as a service | ✅ kill -9 keeps confirmed extents over TCP; reconnect semantics documented; 128 tests green |
| P8 | Striped parallel read: CRUSH-style rendezvous placement, per-server pipelined reads | ✅ 4-client striped aggregate = 77–84% of measured pool capacity (scripts/stripe-bench.sh) |
| P9 | Parallel write + 2-way chain replication | ✅ primary-to-secondary durability acknowledgement and survivor reads |
| P10 | Failure groups + placement policy | ✅ rack-separated replicas; simulated full-rack loss stays readable |
| P11–P40 | See [ROADMAP.md](ROADMAP.md) — 40 phases to full GPFS feature parity | not started |

## Quickstart

```bash
cargo build --release

./target/release/porfs mkfs --device demo.img --size 4GiB
./target/release/porfs info --device demo.img
./target/release/porfs bench --device demo.img --size 1GiB --extent-size 1MiB --queue-depth 32
```

## Production usage (current state, P10)

**Format a filesystem and mount it** (one command; both files live on the
machine you mount on):

```bash
cargo build --release
mkdir -p /mnt/porfs
./target/release/porfs mount \
    --meta /var/lib/porfs/meta.redb \
    --data /var/lib/porfs/data.img \
    --mountpoint /mnt/porfs \
    --format 4TiB                 # formats the pair only on first run
# runs in the foreground; unmount with: fusermount3 -u /mnt/porfs
# useful flags: --allow-other (multi-user), --attr-ttl/--entry-ttl (cache),
#               --no-default-permissions (turn kernel perm checks off)
```

The pair is a sparse extent device (`data.img`, grows as data lands) plus a
redb metadata file (`meta.redb`). Put the device on your fastest NVMe; both
must be on the **same machine as the mount** today.

**Run a chunkserver on another LAN machine** (the data plane IS a network
service since P7):

```bash
# on server01 (any machine on the LAN, no porfs metadata needed):
./target/release/porfs chunkserver \
    --device /var/lib/porfs/chunk0.img --size 16TiB \
    --listen 0.0.0.0:9100
# extents are then readable/writable over TCP from any client using the
# porfs-rpc / porfs-cluster client libraries (wire protocol v2,
# docs/protocol.md — length-prefixed frames, CRC-verified reads,
# two-way chain replication, write-id idempotency, exponential-backoff reconnect).
```

**Mounting from another machine on the LAN: not yet.** The FUSE client
embeds the metadata server (MDS) in-process today, so a mount must be
local to the MDS files; chunkservers are the only piece that is already a
network service. The metadata RPC service lands in Act 2 (P16 wraps the
MDS protocol over the same seam the chunkserver uses). The P9 cluster library
supports remote striped writes, but the FUSE mount does not use it yet. Until
metadata RPC lands, the supported
topologies are: (a) format + mount on one machine, (b) chunkservers on
LAN machines serving extent I/O to client-library users (e.g. the striped
reader, replicated writer, `scripts/stripe-bench.sh`). For replicated
layouts, construct cluster membership with real rack/chassis tags; P10 forces
the two copies onto different racks.

## Tests and gates

```bash
cargo test --workspace            # all unit + integration tests
cargo clippy --workspace --all-targets
./scripts/kill9-soak.sh           # P2 durability gate: 1000 kill -9 soak (~65s)
./scripts/stripe-bench.sh         # P8 gate: 4-client striped aggregate read
./scripts/mvp-smoke.sh            # P6 gate: git clone + busybox build + sqlite stress
./scripts/bigdir-bench.sh         # P5: 1M-entry directory create/ls/find vs baselines
./scripts/pjdfstest.sh            # pjdfstest suite (root mode: sudo; rootless via userns)
```

## Requirements

- Linux ≥ 6.9 (FUSE passthrough); sweet spot ≥ 6.15 (FUSE over io_uring)
- Rust 1.96+ (edition 2024); Zig 0.16+ (from P20 on)

## Repository layout

```
ROADMAP.md            40-phase master plan, every phase has a measurable gate
docs/format.md        on-disk format contract — change it FIRST, bump FORMAT_VERSION
crates/porfs-format   on-disk structs and constants
crates/porfs-store    io_uring extent store (P1–P2)
crates/porfs-cli      the porfs command line (mkfs / info / bench)
scripts/              gate scripts (kill9 soak, ...)
docs/design-v2-dase-archive.md  archived design exploration (reference only)
```

## Documentation

- [ROADMAP.md](ROADMAP.md) — 40-phase plan to GPFS feature parity
- [docs/format.md](docs/format.md) — on-disk format v2 contract
