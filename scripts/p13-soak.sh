#!/usr/bin/env bash
# P13 production soak supervisor (gate: 4 weeks, zero SEV1/SEV2 incidents —
# taxonomy in docs/p13-soak.md). Runs the dogfood cluster on this box:
# 4 loopback chunkservers (ports 9201-9204, racks r1/r1/r2/r2) + a
# replicas=2 cluster mount + a continuous rebuildable-data workload with
# per-cycle verification and incident journaling.
#
# Usage:
#   scripts/p13-soak.sh start     build, boot cluster, populate dataset, start workload
#   scripts/p13-soak.sh stop      stop workload, unmount, stop chunkservers
#   scripts/p13-soak.sh status    liveness + last workload/journal lines
#   scripts/p13-soak.sh verify    one full-manifest verification pass
#   scripts/p13-soak.sh workload-loop   (internal: the churn/verify loop)
#
# State: ${PORFS_SOAK_DIR:-$HOME/.local/share/porfs-soak} (real disk —
# /tmp is tmpfs). Logs: $SOAK_DIR/logs/. Journal: $SOAK_DIR/journal.md.
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PORFS="$REPO_ROOT/target/release/porfs"
PORFSADM="$REPO_ROOT/target/release/porfsadm"
SELF="$REPO_ROOT/scripts/p13-soak.sh"
SOAK_DIR="${PORFS_SOAK_DIR:-$HOME/.local/share/porfs-soak}"
MNT="$SOAK_DIR/mnt"
LOGS="$SOAK_DIR/logs"
PIDS="$SOAK_DIR/pids"
INCIDENTS="$SOAK_DIR/incidents"
JOURNAL="$SOAK_DIR/journal.md"
MANIFEST="$SOAK_DIR/manifest.txt"
CHUNKS="127.0.0.1:9201@r1,127.0.0.1:9202@r1,127.0.0.1:9203@r2,127.0.0.1:9204@r2"
CYCLE_SECS="${SOAK_CYCLE_SECS:-300}"
FULL_EVERY=12
CHURN_FILES=20
SAMPLE_SIZE=100

ts() { date '+%Y-%m-%d %H:%M:%S'; }
log() { echo "[$(ts)] $*" | tee -a "$LOGS/supervisor.log"; }
jnote() { echo "- $(ts) — $*" >> "$JOURNAL"; }

incident() { # $1=SEVn $2=what happened
    local sev="$1" what="$2"
    local f="$INCIDENTS/INCIDENT-$(date +%Y%m%d-%H%M%S).txt"
    {
        echo "SEVERITY: $sev"
        echo "TIME: $(ts)"
        echo "WHAT: $what"
        echo "--- supervisor.log tail ---"
        tail -20 "$LOGS/supervisor.log" 2>/dev/null
        echo "--- workload.log tail ---"
        tail -20 "$LOGS/workload.log" 2>/dev/null
    } > "$f"
    log "INCIDENT $sev: $what (details: $f)"
    jnote "INCIDENT $sev: $what"
    echo "P13 SOAK INCIDENT $sev: $what" >&2
}

need_binary() {
    if [ ! -x "$PORFS" ] || [ "$REPO_ROOT/Cargo.toml" -nt "$PORFS" ]; then
        cargo build --release --manifest-path "$REPO_ROOT/Cargo.toml" || {
            echo "release build failed" >&2
            exit 1
        }
    fi
}

start_chunkserver() { # $1=index
    local i="$1"
    if [ -f "$PIDS/cs$i.pid" ] && kill -0 "$(cat "$PIDS/cs$i.pid")" 2>/dev/null; then
        return 0
    fi
    nohup "$PORFS" chunkserver --device "$SOAK_DIR/dev$i.img" --size 64GiB \
        --listen "127.0.0.1:920$i" --metrics-listen "127.0.0.1:992$i" \
        >> "$LOGS/cs$i.log" 2>&1 < /dev/null &
    echo $! > "$PIDS/cs$i.pid"
    for _ in $(seq 1 100); do
        grep -q "porfs-chunkserver: serving" "$LOGS/cs$i.log" 2>/dev/null && return 0
        sleep 0.1
    done
    log "chunkserver $i failed to start (see $LOGS/cs$i.log)"
    return 1
}

wait_mount() {
    for _ in $(seq 1 100); do
        findmnt -rn "$MNT" >/dev/null 2>&1 && return 0
        sleep 0.1
    done
    log "mount did not come up (see $LOGS/mount.log)"
    return 1
}

populate() { # deterministic generated dataset + best-effort busybox mirror
    [ -f "$SOAK_DIR/.populated" ] && return 0
    log "populating dataset (generated tree + optional busybox mirror)"
    if ! python3 - "$MNT" << 'PYEOF'
import hashlib, os, random, sys

mnt = sys.argv[1]
rng = random.Random(20260719)
root = os.path.join(mnt, "dataset", "generated")
count, total = 0, 0
for a in range(16):
    for b in range(8):
        d = os.path.join(root, f"{a:02x}", f"{b:02x}")
        os.makedirs(d, exist_ok=True)
        for n in range(rng.randint(8, 24)):
            size = rng.choice([4096, 65536, 262144, 1 << 20, 4 << 20])
            seed = f"{a:02x}/{b:02x}/{n}".encode()
            block = hashlib.sha256(seed).digest()
            path = os.path.join(d, f"g-{n:03d}-{size}.bin")
            with open(path, "wb") as fh:
                written = 0
                while written < size:
                    take = min(len(block), size - written)
                    fh.write(block[:take])
                    written += take
                    block = hashlib.sha256(block).digest()
            count += 1
            total += size
print(f"generated: {count} files, {total} bytes")
PYEOF
    then
        log "dataset generation failed"
        return 1
    fi
    # Best-effort rebuildable mirror (network optional; never fatal).
    if curl -fsSL --max-time 60 "https://busybox.net/downloads/busybox-1.36.1.tar.bz2" \
        -o "$MNT/dataset/busybox-1.36.1.tar.bz2" 2>> "$LOGS/populate.log"; then
        log "busybox mirror tarball stored"
    else
        log "busybox mirror skipped (offline is fine — generated set is the core)"
    fi
    if ! ( cd "$MNT/dataset" && find . -type f -print0 | sort -z | xargs -0 sha256sum ) > "$MANIFEST"; then
        log "dataset manifest build failed"
        return 1
    fi
    [ -s "$MANIFEST" ] || { log "dataset manifest is empty"; return 1; }
    sync
    touch "$SOAK_DIR/.populated"
    log "dataset manifest: $(wc -l < "$MANIFEST") files"
}

sample_verify() { # random sample of the dataset manifest
    local sample
    sample=$(mktemp "$SOAK_DIR/sample.XXXXXX")
    shuf -n "$SAMPLE_SIZE" "$MANIFEST" > "$sample" 2>/dev/null || head -n "$SAMPLE_SIZE" "$MANIFEST" > "$sample"
    local rc=0
    ( cd "$MNT/dataset" && sha256sum --check --quiet "$sample" ) >> "$LOGS/workload.log" 2>&1 || rc=1
    rm -f "$sample"
    return "$rc"
}

full_verify() {
    local splitdir rc=0 pid pids=()
    splitdir=$(mktemp -d "$SOAK_DIR/fullverify.XXXXXX")
    split -n "l/$(nproc)" "$MANIFEST" "$splitdir/part_"
    for part in "$splitdir"/part_*; do
        ( cd "$MNT/dataset" && sha256sum --check --quiet "$part" ) >> "$LOGS/workload.log" 2>&1 &
        pids+=($!)
    done
    for pid in "${pids[@]}"; do wait "$pid" || rc=1; done
    rm -rf "$splitdir"
    return "$rc"
}

churn() { # write+fsync fresh pattern files, verify them back, prune old
    local day_dir="$MNT/churn/$(date +%F)"
    mkdir -p "$day_dir"
    python3 - "$day_dir" "$CHURN_FILES" << 'PYEOF'
import hashlib, os, sys

d, n = sys.argv[1], int(sys.argv[2])
seed0 = os.path.basename(d).encode()
for i in range(n):
    size = 4096 + (i % 5) * 65536
    block = hashlib.sha256(seed0 + b"/" + str(i).encode()).digest()
    path = os.path.join(d, f"churn-{i:03d}.bin")
    with open(path, "wb") as fh:
        written = 0
        while written < size:
            take = min(len(block), size - written)
            fh.write(block[:take])
            written += take
            block = hashlib.sha256(block).digest()
    with open(path, "rb") as fh:
        digest = hashlib.sha256(fh.read()).hexdigest()
    with open(path + ".sha", "w") as fh:
        fh.write(digest + "\n")
    fd = os.open(path, os.O_RDONLY)
    os.fsync(fd)
    os.close(fd)
print(f"churn: {n} files written+hashed in {d}")
PYEOF
    # read-back verification of today's churn files
    python3 - "$day_dir" << 'PYEOF'
import hashlib, os, sys

d = sys.argv[1]
bad = 0
for name in sorted(os.listdir(d)):
    if not name.endswith(".bin"):
        continue
    path = os.path.join(d, name)
    with open(path, "rb") as fh:
        digest = hashlib.sha256(fh.read()).hexdigest()
    with open(path + ".sha") as fh:
        want = fh.read().strip()
    if digest != want:
        print(f"MISMATCH: {path}")
        bad += 1
sys.exit(1 if bad else 0)
PYEOF
    find "$MNT/churn" -mindepth 1 -maxdepth 1 -type d -mtime +3 -exec rm -rf {} + 2>/dev/null
}

sqlite_probe() {
    python3 - "$MNT" << 'PYEOF'
import os, sqlite3, sys

db = os.path.join(sys.argv[1], "churn", "probe.db")
con = sqlite3.connect(db)
cur = con.cursor()
cur.execute("PRAGMA journal_mode=WAL")
cur.execute("PRAGMA synchronous=FULL")
cur.execute("CREATE TABLE IF NOT EXISTS t (id INTEGER PRIMARY KEY, v INTEGER)")
cur.execute("DELETE FROM t")
with con:
    cur.executemany("INSERT INTO t(v) VALUES (?)", [(i,) for i in range(2000)])
total = cur.execute("SELECT SUM(v) FROM t").fetchone()[0]
assert total == 2000 * 1999 // 2, f"sqlite probe sum {total}"
ic = cur.execute("PRAGMA integrity_check").fetchone()[0]
assert ic == "ok", f"sqlite probe integrity: {ic}"
con.close()
print("sqlite probe ok")
PYEOF
}

check_daemons() { # restart dead chunkservers (SEV3), remount dead mount (SEV2)
    local i pid
    for i in 1 2 3 4; do
        pid=$(cat "$PIDS/cs$i.pid" 2>/dev/null || echo "")
        if [ -n "$pid" ] && ! kill -0 "$pid" 2>/dev/null; then
            incident "SEV3" "chunkserver $i (pid $pid) died; restarting (replicas absorb by design)"
            : > "$LOGS/cs$i.log"
            start_chunkserver "$i"
        fi
    done
    pid=$(cat "$PIDS/mount.pid" 2>/dev/null || echo "")
    if [ -n "$pid" ] && ! kill -0 "$pid" 2>/dev/null; then
        incident "SEV2" "FUSE mount (pid $pid) died; attempting remount"
        do_mount || incident "SEV2" "remount failed — cluster unavailable"
    fi
}

do_mount() {
    if [ -f "$SOAK_DIR/meta.redb" ]; then
        nohup "$PORFS" mount --meta "$SOAK_DIR/meta.redb" --mountpoint "$MNT" \
            --no-default-permissions \
            >> "$LOGS/mount.log" 2>&1 < /dev/null &
    else
        nohup "$PORFS" mount --meta "$SOAK_DIR/meta.redb" --mountpoint "$MNT" \
            --format --chunks "$CHUNKS" --replicas 2 --no-default-permissions \
            >> "$LOGS/mount.log" 2>&1 < /dev/null &
    fi
    echo $! > "$PIDS/mount.pid"
    wait_mount
}

workload_loop() {
    log "workload loop started (cycle=${CYCLE_SECS}s, full verify every $FULL_EVERY cycles)"
    jnote "workload loop started (cycle=${CYCLE_SECS}s)"
    local cycle=0
    while true; do
        cycle=$((cycle + 1))
        check_daemons
        if ! findmnt -rn "$MNT" >/dev/null 2>&1; then
            incident "SEV2" "mountpoint lost at cycle $cycle"
            sleep "$CYCLE_SECS"
            continue
        fi
        if ! churn >> "$LOGS/workload.log" 2>&1; then
            incident "SEV1" "churn write/read-back mismatch at cycle $cycle"
        fi
        if ! sqlite_probe >> "$LOGS/workload.log" 2>&1; then
            incident "SEV1" "sqlite probe failed at cycle $cycle"
        fi
        if ! sample_verify; then
            incident "SEV1" "dataset sample verification mismatch at cycle $cycle"
        fi
        if [ $((cycle % FULL_EVERY)) -eq 0 ]; then
            if ! full_verify; then
                incident "SEV1" "dataset FULL verification mismatch at cycle $cycle"
            else
                echo "[$(ts)] cycle $cycle: full verify ok ($(wc -l < "$MANIFEST") files)" >> "$LOGS/workload.log"
                jnote "cycle $cycle: full verify ok"
            fi
        fi
        for i in 1 2 3 4; do
            "$PORFSADM" status --addr "127.0.0.1:920$i" >> "$LOGS/status.log" 2>&1 || true
        done
        echo "[$(ts)] cycle $cycle done" >> "$LOGS/workload.log"
        sleep "$CYCLE_SECS"
    done
}

cmd_start() {
    need_binary
    mkdir -p "$MNT" "$LOGS" "$PIDS" "$INCIDENTS"
    [ -f "$JOURNAL" ] || {
        echo "# P13 soak journal" > "$JOURNAL"
        echo "cluster: 4 loopback chunkservers (9201-9204, racks r1/r1/r2/r2), replicas=2 mount" >> "$JOURNAL"
    }
    local ok=1
    for i in 1 2 3 4; do start_chunkserver "$i" || ok=0; done
    [ "$ok" = 1 ] || { log "start failed: a chunkserver did not come up"; exit 1; }
    do_mount || { log "start failed: mount did not come up"; exit 1; }
    populate || { log "start failed: populate failed"; exit 1; }
    if [ -f "$PIDS/workload.pid" ] && kill -0 "$(cat "$PIDS/workload.pid")" 2>/dev/null; then
        log "workload already running"
    else
        nohup "$SELF" workload-loop >> "$LOGS/workload-driver.log" 2>&1 < /dev/null &
        echo $! > "$PIDS/workload.pid"
    fi
    jnote "soak (re)started: 4 chunkservers + mount + workload"
    log "soak started (mount: $MNT)"
}

cmd_stop() {
    local pid
    pid=$(cat "$PIDS/workload.pid" 2>/dev/null || echo "")
    [ -n "$pid" ] && kill "$pid" 2>/dev/null
    sleep 0.5
    fusermount3 -u "$MNT" 2>/dev/null || fusermount3 -u -z "$MNT" 2>/dev/null
    for _ in $(seq 1 50); do
        pid=$(cat "$PIDS/mount.pid" 2>/dev/null || echo "")
        [ -z "$pid" ] && break
        kill -0 "$pid" 2>/dev/null || break
        sleep 0.1
    done
    for name in workload mount cs1 cs2 cs3 cs4; do
        pid=$(cat "$PIDS/$name.pid" 2>/dev/null || echo "")
        [ -n "$pid" ] && kill "$pid" 2>/dev/null
    done
    sleep 1
    for name in workload mount cs1 cs2 cs3 cs4; do
        pid=$(cat "$PIDS/$name.pid" 2>/dev/null || echo "")
        [ -n "$pid" ] && kill -9 "$pid" 2>/dev/null
        rm -f "$PIDS/$name.pid"
    done
    jnote "soak stopped"
    log "soak stopped"
}

cmd_status() {
    local pid
    echo "== daemons =="
    for name in cs1 cs2 cs3 cs4 mount workload; do
        pid=$(cat "$PIDS/$name.pid" 2>/dev/null || echo "-")
        if [ -n "$pid" ] && [ "$pid" != "-" ] && kill -0 "$pid" 2>/dev/null; then
            echo "$name: pid $pid (alive)"
        else
            echo "$name: DOWN (pid $pid)"
        fi
    done
    echo "== mount =="
    findmnt -rn "$MNT" 2>/dev/null || echo "$MNT not mounted"
    echo "== incidents =="
    ls "$INCIDENTS" 2>/dev/null | tail -5
    echo "== workload.log tail =="
    tail -5 "$LOGS/workload.log" 2>/dev/null
    echo "== journal tail =="
    tail -5 "$JOURNAL" 2>/dev/null
}

cmd_verify() {
    [ -f "$MANIFEST" ] || { echo "no manifest yet"; exit 1; }
    if full_verify; then
        echo "full verify: OK ($(wc -l < "$MANIFEST") files)"
    else
        echo "full verify: MISMATCH"
        exit 1
    fi
}

mkdir -p "$LOGS"
case "${1:-}" in
    start) cmd_start ;;
    stop) cmd_stop ;;
    status) cmd_status ;;
    verify) cmd_verify ;;
    workload-loop) workload_loop ;;
    *)
        echo "usage: $0 start|stop|status|verify|workload-loop" >&2
        exit 2
        ;;
esac
