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
| P11 | Close-to-open consistency | In progress: close flushes data; regular-file opens revalidate and bypass stale page-cache data |
| P12 | Observability | ✅ Prometheus chunkserver metrics, structured tracing init, `porfsadm status` over Stats RPC |
| P12.5 | FUSE data plane over chunkservers (wire v3) | ✅ mount on machine B, data on chunkservers A/C/… — replicas=2, read failover, honest EIO, kill-one-chunkserver zero confirmed-data loss (scripts/cluster-smoke.sh); 164 tests green |
| P13 | Production v0.1 rollout (4-week dogfood soak) | 🏃 in progress: soak running since 2026-07-19 (docs/p13-soak.md, scripts/p13-soak.sh); verdict due 2026-08-16 |
| P14–P40 | See [ROADMAP.md](ROADMAP.md) — 40 phases to full GPFS feature parity | not started |

## Quickstart

```bash
cargo build --release

./target/release/porfs mkfs --device demo.img --size 4GiB
./target/release/porfs info --device demo.img
./target/release/porfs bench --device demo.img --size 1GiB --extent-size 1MiB --queue-depth 32
```

## Production usage (current state, P12.5; P11 still in progress)

**Cluster mount — mount on one machine, data on others** (the P12.5
topology): run a chunkserver on each data machine, then format + mount
against them:

```bash
# on every data machine (server01..server04):
./target/release/porfs chunkserver \
    --device /var/lib/porfs/chunk0.img --size 16TiB \
    --listen 0.0.0.0:9100 \
    --metrics-listen 127.0.0.1:9900

# on the mount machine (first run formats, later runs just mount):
./target/release/porfs mount \
    --meta /var/lib/porfs/meta.redb \
    --mountpoint /mnt/porfs \
    --format --replicas 2 \
    --chunks server01:9100@rack1,server02:9100@rack1,server03:9100@rack2,server04:9100@rack2
# remount needs no flags (membership is pinned in the meta file):
./target/release/porfs mount --meta /var/lib/porfs/meta.redb --mountpoint /mnt/porfs
```

Semantics with `replicas=2`: every extent is chain-written to two
rack-separated chunkservers and acknowledged only by both. Killing one
chunkserver: all reads keep serving (failover to the surviving copy,
circuit-breaker accelerated), writes whose replica pair includes the dead
server fail with EIO, and fsync fails with EIO until the chunkserver
returns — nothing is ever silently acknowledged once. A remount requires
all chunkservers reachable and identity-verified (store UUIDs are pinned
at format; a wrong server at an address fails the mount rather than
endangering the namespace).

**Single-machine mount** (local data plane, as before):

```bash
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
must be on the **same machine as the mount** in local mode.

**Inspect a chunkserver without changing the wire protocol**:

```bash
./target/release/porfsadm status --addr 127.0.0.1:9100
curl -s http://127.0.0.1:9900/metrics | grep '^porfs_'
```

**Not yet:** multiple mounts sharing one metadata service (the MDS is still
embedded in each mount process — the metadata RPC service is P16), and
membership changes after format (static; P-later). The wire protocol is v3
(docs/protocol.md — length-prefixed frames, CRC-verified full/subrange
reads, two-way chain replication, write-id idempotency, server-identity
pinning, exponential-backoff reconnect).

### Cache and close-to-open semantics

Regular-file `open` always revalidates the inode at the MDS and uses direct
I/O, so a new open observes data written and closed by another client rather
than a stale kernel page-cache copy. `close` flushes the file through `fsync`,
making its completed writes durable before a subsequent open. Attribute and
directory-entry replies still use `--attr-ttl` and `--entry-ttl` (one second by
default), so metadata-only observations can remain stale for at most the
configured TTL. This is close-to-open consistency, not the concurrent-writer,
`mmap`, or `O_APPEND` coherence planned for P17.

## Tests and gates

```bash
cargo test --workspace            # all unit + integration tests
cargo clippy --workspace --all-targets
./scripts/kill9-soak.sh           # P2 durability gate: 1000 kill -9 soak (~65s)
./scripts/cluster-smoke.sh        # P12.5 gate: cluster mount P6 workloads + kill-one-chunkserver (~3min)
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
crates/porfs-cli      the porfs + porfsadm command lines
scripts/              gate scripts (kill9 soak, ...)
docs/design-v2-dase-archive.md  archived design exploration (reference only)
```

## Documentation

- [ROADMAP.md](ROADMAP.md) — 40-phase plan to GPFS feature parity
- [docs/format.md](docs/format.md) — on-disk format v2 contract
- [docs/observability.md](docs/observability.md) — tracing, Prometheus metrics, `porfsadm`
