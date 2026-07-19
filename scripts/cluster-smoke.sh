#!/usr/bin/env bash
# P12.5 cluster gate: a real FUSE mount whose file data lives on four
# loopback chunkservers (wire v3, replicas=2 across racks r1/r2), then
#   1. the P6 workloads on the cluster mount: git clone (+ fsck --strict),
#      busybox build, sqlite WAL stress — zero data errors;
#   2. a full-tree sha256 manifest;
#   3. kill -9 one chunkserver (127.0.0.1:9103, deterministic):
#      (a) the manifest re-verifies 100% — every byte fails over to the
#          surviving copy;
#      (b) a bounded-time sqlite/fsync-heavy op FAILS LOUDLY (strict
#          sync broadcast: one dead server -> EIO; a hang is a gate FAIL);
#   4. restart 9103 on the same device:
#      (c) fresh write+fsync succeeds, a sweep of writes ALL succeeds
#          (the restarted server rejoined placements), and the manifest
#          is still byte-exact — zero confirmed-data loss;
#   5. clean unmount + cluster mds-check (no --data).
#
# Stage 3a timing note: the data plane's circuit breaker (plane.rs)
# skips a dead server after its first timeout, so failover reads no
# longer pay a 2s penalty per file; the residual cost is the single
# worker thread's per-file FUSE overhead, which the parallel --check
# splits across nproc readers. `timeout` remains as hang protection.
# Usage: scripts/cluster-smoke.sh
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PORFS="$REPO_ROOT/target/release/porfs"
BASE="${CARGO_TARGET_TMPDIR:-$HOME/.cache}"
mkdir -p "$BASE"
WORK="$(mktemp -d "$BASE/porfs-cluster-smoke.XXXXXX")"
MNT="$WORK/mnt"
BBVER="1.36.1"
PASS=0; FAIL=0
declare -a CS_PIDS=()
MPID=""
CHUNKS="127.0.0.1:9101@r1,127.0.0.1:9102@r1,127.0.0.1:9103@r2,127.0.0.1:9104@r2"

cleanup() {
    set +e
    fusermount3 -u "$MNT" 2>/dev/null || fusermount3 -u -z "$MNT" 2>/dev/null
    [ -n "$MPID" ] && kill "$MPID" 2>/dev/null
    for pid in "${CS_PIDS[@]:-}"; do kill "$pid" 2>/dev/null; done
    sleep 0.5
    [ -n "$MPID" ] && kill -9 "$MPID" 2>/dev/null
    for pid in "${CS_PIDS[@]:-}"; do kill -9 "$pid" 2>/dev/null; done
    wait 2>/dev/null
    if [ "$FAIL" -ne 0 ]; then
        echo "cluster-smoke: ---- mount.log tail ----"
        tail -20 "$WORK/mount.log" 2>/dev/null
        for i in 1 2 3 4; do
            echo "cluster-smoke: ---- cs$i.log tail ----"
            tail -5 "$WORK/cs$i.log" 2>/dev/null
        done
    fi
    rm -rf "$WORK"
}
trap cleanup EXIT

step() { echo; echo "==== $* ===="; }
ok() { echo "cluster-smoke: PASS: $*"; PASS=$((PASS+1)); }
bad() { echo "cluster-smoke: FAIL: $*"; FAIL=$((FAIL+1)); }

start_chunkserver() { # $1=index  $2=create(1|0)
    local i="$1" create="$2"
    local args=(chunkserver --device "$WORK/dev$i.img" --listen "127.0.0.1:910$i")
    if [ "$create" = 1 ]; then
        args+=(--size 2GiB)
    fi
    "$PORFS" "${args[@]}" > "$WORK/cs$i.log" 2>&1 < /dev/null &
    CS_PIDS[$((i-1))]=$!
}

wait_chunkserver() { # $1=index
    local i="$1" n
    for n in $(seq 1 100); do
        if grep -q "porfs-chunkserver: serving" "$WORK/cs$i.log" 2>/dev/null; then
            return 0
        fi
        kill -0 "${CS_PIDS[$((i-1))]}" 2>/dev/null || break
        sleep 0.1
    done
    echo "cluster-smoke: chunkserver $i failed to start; log:"
    cat "$WORK/cs$i.log" 2>/dev/null || true
    return 1
}

step "0. release binary + 4 chunkservers (127.0.0.1:9101-9104)"
cargo build --release --manifest-path "$REPO_ROOT/Cargo.toml" 2>&1 | tail -1
mkdir -p "$MNT"
for i in 1 2 3 4; do
    start_chunkserver "$i" 1
    wait_chunkserver "$i"
done
echo "4 chunkservers up (racks r1,r1,r2,r2)"

step "1. format cluster fs + mount (replicas=2)"
"$PORFS" mount --meta "$WORK/meta.redb" --mountpoint "$MNT" \
    --format --chunks "$CHUNKS" --replicas 2 \
    --no-default-permissions > "$WORK/mount.log" 2>&1 < /dev/null &
MPID=$!
for _ in $(seq 1 100); do findmnt -rn "$MNT" >/dev/null 2>&1 && break; sleep 0.1; done
findmnt -rn "$MNT" >/dev/null 2>&1 || { echo "cluster-smoke: mount failed"; cat "$WORK/mount.log"; exit 1; }
echo "mounted at $MNT"

step "2. git clone (+ fsck --strict)"
if git clone --quiet --depth 1 https://github.com/sqlite/sqlite.git "$MNT/sqlite-src" 2> "$WORK/git.log"; then
    if git -C "$MNT/sqlite-src" fsck --strict >> "$WORK/git.log" 2>&1; then
        ok "git clone (objects: $(cat "$MNT/sqlite-src/.git/objects/pack/"*.pack 2>/dev/null | wc -c) pack bytes), fsck --strict clean"
    else
        bad "git fsck reported corruption (see $WORK/git.log)"
    fi
else
    bad "git clone failed (see $WORK/git.log)"
fi

step "3. busybox build (Kconfig + defconfig, ~700 compile units)"
BBDIR="$MNT/busybox-$BBVER"
if curl -fsSL "https://busybox.net/downloads/busybox-$BBVER.tar.bz2" -o "$MNT/bb.tar.bz2" \
    && tar -xjf "$MNT/bb.tar.bz2" -C "$MNT"; then
    rm -f "$MNT/bb.tar.bz2"
    # busybox 1.36.1's tc applet no longer compiles against kernel >= 6.16
    # headers (legacy CBQ structs removed); drop that one applet, keep the
    # other ~700 compile units as the workload.
    if make -C "$BBDIR" defconfig > "$WORK/bbuild.log" 2>&1 \
        && sed -i 's/^CONFIG_TC=y$/# CONFIG_TC is not set/' "$BBDIR/.config" \
        && { yes "" | make -C "$BBDIR" oldconfig >> "$WORK/bbuild.log" 2>&1 || [ $? -eq 141 ]; } \
        && make -C "$BBDIR" -j"$(nproc)" >> "$WORK/bbuild.log" 2>&1 \
        && [ -s "$BBDIR/busybox" ]; then
        ok "busybox build (binary: $(du -h "$BBDIR/busybox" | cut -f1); smoke: $("$BBDIR/busybox" echo busybox-works))"
    else
        bad "busybox build failed (see $WORK/bbuild.log)"
    fi
else
    bad "busybox tarball download/extract failed"
fi

step "4. sqlite stress (WAL, synchronous=FULL)"
if python3 - "$MNT" > "$WORK/sqlite.log" 2>&1 << 'SQLITE_EOF'
import os, sqlite3, sys

mnt = sys.argv[1]
db = os.path.join(mnt, "stress.db")
con = sqlite3.connect(db)
cur = con.cursor()
cur.execute("PRAGMA journal_mode=WAL")
cur.execute("PRAGMA synchronous=FULL")
cur.execute("CREATE TABLE t (id INTEGER PRIMARY KEY, v INTEGER)")

N = 20000
for base in range(0, N, 1000):
    with con:
        cur.executemany("INSERT INTO t(v) VALUES (?)", [(i,) for i in range(base, base + 1000)])

# v == i, id == i+1 throughout.
expect = N * (N - 1) // 2
with con:
    cur.execute("UPDATE t SET v = v * 2 WHERE id % 3 = 0")
expect += sum(i for i in range(N) if i % 3 == 2)
with con:
    cur.execute("DELETE FROM t WHERE id % 10 = 0")
expect -= sum(2 * i if i % 3 == 2 else i for i in range(N) if i % 10 == 9)

rows, total = cur.execute("SELECT COUNT(*), COALESCE(SUM(v), 0) FROM t").fetchone()
assert total == expect, f"SUM(v)={total} != expected {expect}"
assert rows == N - N // 10, f"rows={rows} != expected {N - N // 10}"
ic = cur.execute("PRAGMA integrity_check").fetchone()[0]
assert ic == "ok", f"integrity_check: {ic}"
con.close()
assert not os.path.exists(db + "-wal"), "WAL file left behind after clean close"
print(f"rows={rows} sum={total} integrity_check=ok")
SQLITE_EOF
then
    ok "sqlite stress ($(tail -1 "$WORK/sqlite.log"))"
else
    bad "sqlite stress failed (see $WORK/sqlite.log)"
fi

step "5. full-tree sha256 manifest"
sync
( cd "$MNT" && find . -type f -print0 | sort -z | xargs -0 sha256sum ) > "$WORK/manifest.txt"
MANIFEST_COUNT=$(wc -l < "$WORK/manifest.txt")
echo "manifest: $MANIFEST_COUNT files"

step "6. kill -9 chunkserver 127.0.0.1:9103"
kill -9 "${CS_PIDS[2]}"
wait "${CS_PIDS[2]}" 2>/dev/null || true
echo "9103 down"

step "7. (a) manifest re-verify with 9103 down (failover reads every byte)"
VERIFY_START=$SECONDS
SPLITDIR="$WORK/verify-split"
mkdir -p "$SPLITDIR"
split -n "l/$(nproc)" "$WORK/manifest.txt" "$SPLITDIR/part_"
parallel_check() { # $1=dir with part_* manifest chunks; rc: 0 ok, 1 mismatch
    local dir="$1" pid rc=0 pids=()
    for part in "$dir"/part_*; do
        ( cd "$MNT" && sha256sum --check --quiet "$part" ) &
        pids+=($!)
    done
    for pid in "${pids[@]}"; do wait "$pid" || rc=1; done
    return "$rc"
}
parallel_check "$SPLITDIR" &
CHECK_PID=$!
VERIFY_RC=0
if timeout 900 tail --pid="$CHECK_PID" -f /dev/null; then
    wait "$CHECK_PID" || VERIFY_RC=1
else
    VERIFY_RC=124
    kill -9 "$CHECK_PID" 2>/dev/null
    pkill -f "sha256sum --check --quiet $SPLITDIR/part_" 2>/dev/null || true
fi
if [ "$VERIFY_RC" = 0 ]; then
    ok "manifest re-verified: $MANIFEST_COUNT files, 100% byte-exact with 9103 down ($((SECONDS-VERIFY_START))s)"
elif [ "$VERIFY_RC" = 124 ]; then
    bad "manifest verify TIMED OUT with 9103 down (hang = gate failure)"
else
    bad "manifest verify found mismatches/missing files with 9103 down"
fi

step "8. (b) bounded sqlite/fsync-heavy op must FAIL LOUDLY with 9103 down"
if timeout 300 python3 - "$MNT" > "$WORK/postkill.log" 2>&1 << 'POSTKILL_EOF'
import os, sqlite3, sys

mnt = sys.argv[1]
db = os.path.join(mnt, "postkill.db")
try:
    con = sqlite3.connect(db)
    cur = con.cursor()
    cur.execute("PRAGMA journal_mode=WAL")
    cur.execute("PRAGMA synchronous=FULL")
    cur.execute("CREATE TABLE t (id INTEGER PRIMARY KEY, v INTEGER)")
    for base in range(0, 20000, 100):
        with con:
            cur.executemany("INSERT INTO t(v) VALUES (?)", [(i,) for i in range(base, base + 100)])
except Exception as e:
    print(f"failed loudly as required: {e}")
    sys.exit(0)
print("sqlite op COMPLETED with a dead chunkserver: strict fsync broadcast broken", file=sys.stderr)
sys.exit(1)
POSTKILL_EOF
then
    ok "post-kill sqlite op failed loudly ($(tail -1 "$WORK/postkill.log"))"
else
    rc=$?
    if [ "$rc" = 124 ]; then
        bad "post-kill sqlite op HUNG (timeout after 300s)"
    else
        bad "post-kill sqlite op silently succeeded (see $WORK/postkill.log)"
    fi
fi

step "9. restart chunkserver 9103 on the same device"
start_chunkserver 3 0
wait_chunkserver 3
echo "9103 back up"

step "10. (c) rejoin: fresh write+fsync, all-succeed sweep, manifest byte-exact"
if python3 - "$MNT" > "$WORK/rejoin.log" 2>&1 << 'REJOIN_EOF'
import os, sys

mnt = sys.argv[1]
# Fresh write + fsync: the strict broadcast must be whole again.
path = os.path.join(mnt, "rejoin-fresh.bin")
fd = os.open(path, os.O_CREAT | os.O_WRONLY, 0o644)
os.write(fd, b"fresh-after-restart" * 1024)
os.fsync(fd)
os.close(fd)
# Sweep: every write succeeds — placements involving the restarted
# server work again (rendezvous spreads 24 files over all 4 servers).
for i in range(24):
    p = os.path.join(mnt, f"rejoin-sweep-{i}.bin")
    fd = os.open(p, os.O_CREAT | os.O_WRONLY, 0o644)
    os.write(fd, bytes([i & 0xFF]) * 16384)
    os.fsync(fd)
    os.close(fd)
print("fresh write+fsync ok; 24/24 sweep writes ok")
REJOIN_EOF
then
    ok "restarted 9103 rejoined placements ($(tail -1 "$WORK/rejoin.log"))"
else
    bad "post-restart writes failed (see $WORK/rejoin.log)"
fi

SPLITDIR2="$WORK/verify-split2"
mkdir -p "$SPLITDIR2"
split -n "l/$(nproc)" "$WORK/manifest.txt" "$SPLITDIR2/part_"
RECHECK_START=$SECONDS
parallel_check "$SPLITDIR2" &
CHECK_PID=$!
RECHECK_RC=0
if timeout 900 tail --pid="$CHECK_PID" -f /dev/null; then
    wait "$CHECK_PID" || RECHECK_RC=1
else
    RECHECK_RC=124
    kill -9 "$CHECK_PID" 2>/dev/null
    pkill -f "sha256sum --check --quiet $SPLITDIR2/part_" 2>/dev/null || true
fi
if [ "$RECHECK_RC" = 0 ]; then
    ok "manifest byte-exact after kill + restart: zero confirmed-data loss ($((SECONDS-RECHECK_START))s)"
else
    bad "manifest mismatch after restart (confirmed data lost)"
fi

step "11. unmount + cluster mds-check"
sync
fusermount3 -u "$MNT" || fusermount3 -u -z "$MNT"
for _ in $(seq 1 50); do kill -0 "$MPID" 2>/dev/null || break; sleep 0.1; done
kill -9 "$MPID" 2>/dev/null || true
wait "$MPID" 2>/dev/null || true
MPID=""
if "$PORFS" mds-check --meta "$WORK/meta.redb" > "$WORK/mds-check.log" 2>&1; then
    ok "cluster mds-check: $(grep -E 'inodes|extent_map_rows' "$WORK/mds-check.log" | tr '\n' ' ')"
else
    bad "cluster mds-check failed (see $WORK/mds-check.log)"
fi

echo
echo "cluster-smoke: $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ]
