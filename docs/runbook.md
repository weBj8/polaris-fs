# PolarisFS Runbook (S20)

Operations guide for plfs clusters. Binding contracts: `docs/design.md` (v0.2),
`docs/format-arena.md` (v2). Honest maturity label: **production candidate**.

## Roles & topology

One binary, three roles + standalone:

```bash
plfs registry --dir /var/lib/plfs/reg --node-id 1 --listen 0.0.0.0:9501 --peers 1@a:9501,2@b:9501,3@c:9501
plfs data --arena /dev/sdX --listen 0.0.0.0:9100 --registry a:9501 --node-key data-1
plfs standalone --dir /var/lib/plfs/vol0 --mount /mnt/plfs   # single-machine
```

- Registry: 3-node raft group (quorum 2). Lose one member → keeps serving.
- Data nodes: ChunkArena over a raw block device or sparse image; register +
  heartbeat (5 s liveness lease) with the registry.
- Volumes: a client-side volume dir (meta.redb + WAL + cache). A volume has
  exactly ONE writer at a time (single-writer model, design §8.2).

## Metrics to watch (design §11 dashboard)

Exposed on the standalone/client Prometheus endpoint (`--metrics-listen`):

| Metric | Meaning | Alarm when |
|---|---|---|
| `plfs_cache_hits_total` / `plfs_cache_misses_total` | SSD read-cache hits/misses — hit rate is the governing metric | hit rate < 90% |
| `plfs_wal_backlog_records` | Unflushed WAL records on the writer | grows without draining (flush stuck) |
| `plfs_arena_queue_depth` | Data-node arena actor queue depth | sustained growth (disk saturated) |

## Failure response (design §10.2)

| Symptom | What happens | Operator action |
|---|---|---|
| Data node down | Lease expires ~5 s; reads fail over; repair worker re-replicates at 30 MB/s/disk | Replace node; scrub verifies afterwards |
| Slow/flaky node | Writes degrade (quorum still acks); reads fail over | Check disk; drain & replace if persistent |
| Registry member down | Quorum of 3 continues | Restart it; it catches up via raft |
| Client machine dead | Volume claimable elsewhere (registry lease); fsync'd data is safe | Remount the volume on another host |
| Bit rot | crc on scrub (weekly `PLFS_SCRUB_INTERVAL_SECS`, default 7 d) repairs from a healthy replica | Investigate the drive if recurring |
| All else | `scripts/chaos-drill.sh` re-verifies the whole table | Run after any incident |

## Snapshots & rollback (design §9)

- Create/delete via the client API; `.snapshots/<name>/` is the read-only
  FUSE view (unchanged data is served from the shared SSD cache).
- Rollback: `snapshot_rollback(id)` takes an implicit `pre-rollback-*`
  snapshot first — always reversible.
- Retention: drop `snapshots.toml` in the volume dir:
  `interval_secs = 3600, keep = 48, name_prefix = "auto"`.

## Upgrade & format-version policy

- **On-disk formats are versioned and binding** (`docs/format-arena.md`,
  proto packages under `plfs.*.v1`). Contract first: edit the doc, bump the
  version, THEN change code.
- **Arena v1 → v2** introduced the checkpoint region (bounded boot). v1
  arenas are rejected (`UnsupportedVersion`): re-mkfs and re-replicate
  (clients re-upload chunks; the repair worker heals the rest). A fast boot
  (checkpoint) requires a clean shutdown — SIGTERM, not kill -9.
- **Dependencies are exact-version pinned** via the committed `Cargo.lock`;
  bump deliberately, rerun `scripts/ci.sh` + `scripts/chaos-drill.sh`.
- Storage adapters are single-module boundaries: redb lives only in
  `plfs-meta/src/state.rs`; the arena is only reachable through the
  ChunkStore gRPC surface.

## Soak & drill knobs

```bash
ITERS=200 ./scripts/soak-turmoil.sh        # long fault-injection soak
PROPTEST_CASES=2048 ./scripts/ci.sh        # deep property tests
./scripts/chaos-drill.sh                   # the §10.2 failure table, end to end
./scripts/bench-matrix.sh                  # benchmark report → target/bench-report.md
```
