# P13 production v0.1 soak — plan, dataset, incident taxonomy

Gate (ROADMAP P13): run "rebuildable data" on our own cluster for **4 weeks,
zero SEV1/SEV2 incidents**. If we won't run it ourselves, the project
downgrades to a learning project. This document is the soak's contract:
what runs, what counts as an incident, and how the verdict is judged.

## Topology

Single dev box (x86_64, Ryzen 7 8845HS), all processes unprivileged:

- 4 chunkservers on `127.0.0.1:9201-9204`, sparse 64 GiB devices under
  `$SOAK_DIR` (`~/.local/share/porfs-soak`), rack tags `r1,r1,r2,r2`;
- one FUSE cluster mount (`$SOAK_DIR/mnt`), `replicas=2` — every extent
  chain-written to two rack-separated servers (P12.5 data plane, wire v3);
- one workload loop (`scripts/p13-soak.sh workload-loop`), cycle 300 s.

Loopback stands in for the LAN: the wire path is identical TCP, and every
P12.5 fault behavior (failover, honest EIO, identity guard) applies. The
compromise (no independent machine failures, no real network partitions) is
recorded honestly; a multi-box soak is the natural P13-repeat on real
hardware.

## Dataset ("rebuildable data")

- **Core**: `dataset/generated/` — a deterministic pattern tree (~1–3k
  files, 4 KiB–4 MiB, content = chained SHA-256 blocks, seed = path).
  Regenerable from scratch at any time → genuinely rebuildable.
- **Best-effort**: `dataset/busybox-1.36.1.tar.bz2` (re-downloadable
  distribution file). Network is optional; absence is not an incident.
- **Churn**: `churn/<date>/` — 20 fresh pattern files per cycle,
  write+fsync+read-back-hash verified immediately, pruned after 3 days.
  Exercises the write path, tombstones, and delete continuously.
- `manifest.txt` pins the full dataset's SHA-256 list; verification uses
  it (sampled 100 files/cycle, full parallel sweep every 12th cycle).

## Workload per cycle (300 s)

1. daemon liveness check (dead chunkserver → SEV3 incident + restart;
   dead mount → SEV2 incident + remount attempt);
2. churn write + read-back hash verify (mismatch → SEV1);
3. sqlite WAL probe, `synchronous=FULL`, 2000 rows + integrity_check
   (failure → SEV1);
4. dataset sample verify, 100 random files (mismatch → SEV1);
5. every 12th cycle: full-manifest parallel verify (mismatch → SEV1);
6. `porfsadm status` snapshot of all four chunkservers → `logs/status.log`.

## Incident taxonomy

- **SEV1 — data integrity**: any checksum mismatch, any wrong byte read
  back, sqlite integrity failure, fsck-type failure, silent write loss.
  **Gate-failing.**
- **SEV2 — availability**: mount lost or unresponsive, remount failure,
  workload stuck > 2 cycles, any hang. **Gate-failing.**
- **SEV3 — absorbed faults**: a chunkserver process died and the cluster
  kept serving as designed (replicas absorb single loss; restart logged).
  Recorded, **not** gate-failing — this is what replication is for. A SEV3
  that coincides with any data error is reclassified SEV1.

Incidents land in `$SOAK_DIR/incidents/INCIDENT-<ts>.txt` (with log
tails), the journal (`$SOAK_DIR/journal.md`), and stderr.

## Verdict

Due **2026-08-16** (start: 2026-07-19). Pass = 4 weeks with zero SEV1 and
zero SEV2. Any SEV1/SEV2 = gate fail: root-cause, fix, restart the clock
with the fix noted in ROADMAP. Weekly journal summaries record cycles
completed, full verifies passed, and SEV3 events.
