#!/usr/bin/env bash
# bigdir bench for P5: one directory holding 1,000,000 entries.
# Measures create rate, `ls -f` (raw listing) and `find` traversal on a
# porfs mount and on the on-box baselines (xfs /home, tmpfs /tmp).
# Creates files with a python loop (no per-file exec) so the number is
# filesystem cost, not fork/exec cost.
set -u

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PORFS="$REPO_ROOT/target/release/porfs"
WORK="${CARGO_TARGET_TMPDIR:-$REPO_ROOT/target}/bigdir-work"
MNT="/tmp/porfs-bigdir-mnt.$$"
N="${1:-1000000}"
EARLY=50000          # measure the create rate on the first EARLY files
MIN_RATE="${MIN_RATE:-2000}" # below this, cap the run (report, not hours); 0 = never cap
CAP=100000

cleanup() {
    fusermount3 -u -z "$MNT" 2>/dev/null
    rm -rf "$MNT" "$WORK/meta.redb" "$WORK/data.img" /tmp/porfs-bigdir-tmpfs.$$ "$WORK/xfs-baseline"
    kill "${MPID:-}" 2>/dev/null
}
trap cleanup EXIT

create_files() { # $1=dir $2=count $3=start
    python3 - "$1" "$2" "$3" << 'PYEOF'
import os, sys, time
d, n, start = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
os.makedirs(d, exist_ok=True)
t0 = time.monotonic()
for i in range(start, start + n):
    fd = os.open(f"{d}/f{i:07d}", os.O_CREAT | os.O_WRONLY, 0o644)
    os.close(fd)
dt = time.monotonic() - t0
print(f"{n} creates in {dt:.1f}s = {n/dt:.0f}/s")
PYEOF
}

elapsed() { # $1=label; rest=command
    local label="$1"; shift
    local t0 t1 out
    t0=$(date +%s.%N)
    out=$("$@")
    t1=$(date +%s.%N)
    printf "%-10s %6.1fs  (%s entries)\n" "$label" \
        "$(awk -v a="$t0" -v b="$t1" 'BEGIN{printf "%.1f", b-a}')" \
        "$(echo "$out" | wc -l)"
}

measure() { # $1=label $2=dir
    local label="$1" dir="$2" out rate rest
    echo "== $label =="
    out=$(create_files "$dir" "$EARLY" 0)
    echo "$out"
    rate=$(echo "$out" | sed -n 's/.*= \([0-9]*\)\/s/\1/p')
    rest=$((N - EARLY))
    if [ "$rest" -lt 0 ]; then rest=0; fi
    if [ "$MIN_RATE" -gt 0 ] && [ "${rate:-0}" -lt "$MIN_RATE" ]; then
        echo "create rate ${rate}/s < ${MIN_RATE}/s: capping at $CAP entries (report, not hours)"
        rest=$((CAP - EARLY))
    fi
    create_files "$dir" "$rest" "$EARLY"
    sync
    echo "entries: $(find "$dir" -maxdepth 1 -type f | wc -l)"
    elapsed "ls -f" ls -f "$dir"
    elapsed "find" find "$dir" -maxdepth 1 -name 'f*' -type f
}

mkdir -p "$WORK" "$MNT"
rm -f "$WORK/meta.redb" "$WORK/data.img"
"$PORFS" mkfs --device "$WORK/data.img" --meta "$WORK/meta.redb" --size 2GiB >/dev/null || exit 1
"$PORFS" mount --meta "$WORK/meta.redb" --data "$WORK/data.img" --mountpoint "$MNT" --no-default-permissions > "$WORK/mount.log" 2>&1 &
MPID=$!
for i in $(seq 1 100); do findmnt -rn "$MNT" >/dev/null 2>&1 && break; sleep 0.1; done
findmnt -rn "$MNT" >/dev/null 2>&1 || { echo "bigdir: mount failed" >&2; cat "$WORK/mount.log" >&2; exit 1; }

measure "porfs" "$MNT/big"
fusermount3 -u "$MNT" || fusermount3 -u -z "$MNT"
kill "$MPID" 2>/dev/null; wait "$MPID" 2>/dev/null

measure "xfs (/home)" "$WORK/xfs-baseline/big"
measure "tmpfs (/tmp)" "/tmp/porfs-bigdir-tmpfs.$$/big"
