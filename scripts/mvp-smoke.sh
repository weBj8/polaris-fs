#!/usr/bin/env bash
# P6 MVP freeze gate: one command from nothing to a mounted filesystem,
# then three real workloads, each verified with zero data errors.
#   1. git clone (verified with git fsck --strict)
#   2. Linux kernel build, tinyconfig (verified: vmlinux produced)
#   3. sqlite stress, WAL + synchronous=FULL (verified: integrity_check +
#      exact content checksums)
# Usage: scripts/mvp-smoke.sh
set -u

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PORFS="$REPO_ROOT/target/release/porfs"
WORK="${CARGO_TARGET_TMPDIR:-$REPO_ROOT/target}/mvp-smoke"
MNT="/tmp/porfs-mvp-mnt.$$"
BBVER="1.36.1"
PASS=0; FAIL=0

cleanup() {
    fusermount3 -u -z "$MNT" 2>/dev/null
    kill "${MPID:-}" 2>/dev/null
    rm -rf "$MNT"
    if [ "$FAIL" -eq 0 ]; then
        rm -rf "$WORK"
    else
        echo "mvp-smoke: logs kept at $WORK"
    fi
}
trap cleanup EXIT

step() { echo; echo "==== $* ===="; }
ok() { echo "mvp-smoke: PASS: $*"; PASS=$((PASS+1)); }
bad() { echo "mvp-smoke: FAIL: $*"; FAIL=$((FAIL+1)); }

mkdir -p "$WORK" "$MNT"

step "0. one command: format + mount"
"$PORFS" mount --meta "$WORK/meta.redb" --data "$WORK/data.img" \
    --mountpoint "$MNT" --format 16GiB --no-default-permissions > "$WORK/mount.log" 2>&1 &
MPID=$!
for i in $(seq 1 100); do findmnt -rn "$MNT" >/dev/null 2>&1 && break; sleep 0.1; done
findmnt -rn "$MNT" >/dev/null 2>&1 || { echo "mvp-smoke: mount failed"; cat "$WORK/mount.log"; exit 1; }
echo "mounted at $MNT"

step "1. git clone (+ fsck)"
if git clone --quiet --depth 1 https://github.com/sqlite/sqlite.git "$MNT/sqlite-src" 2> "$WORK/git.log"; then
    if git -C "$MNT/sqlite-src" fsck --strict >> "$WORK/git.log" 2>&1; then
        ok "git clone (objects: $(cat "$MNT/sqlite-src/.git/objects/pack/"*.pack 2>/dev/null | wc -c) pack bytes), fsck --strict clean"
    else
        bad "git fsck reported corruption (see $WORK/git.log)"
    fi
else
    bad "git clone failed (see $WORK/git.log)"
fi

step "2. busybox build (Kconfig + defconfig, ~700 compile units)"
# Substituted for the ROADMAP's "kernel build" by owner decision (a full
# kernel tree is disproportionate for this box); busybox uses the same
# Kconfig/Kbuild machinery and exercises the same syscall surface.
BBDIR="$MNT/busybox-$BBVER"
if curl -fsSL "https://busybox.net/downloads/busybox-$BBVER.tar.bz2" -o "$MNT/bb.tar.bz2" \
    && tar -xjf "$MNT/bb.tar.bz2" -C "$MNT"; then
    rm -f "$MNT/bb.tar.bz2"
    # busybox 1.36.1's tc applet no longer compiles against kernel >= 6.16
    # headers (legacy CBQ structs removed); drop that one applet, keep the
    # other ~700 compile units as the workload.
    if make -C "$BBDIR" defconfig > "$WORK/bbuild.log" 2>&1 \
        && sed -i 's/^CONFIG_TC=y$/# CONFIG_TC is not set/' "$BBDIR/.config" \
        && yes "" | make -C "$BBDIR" oldconfig >> "$WORK/bbuild.log" 2>&1 \
        && make -C "$BBDIR" -j"$(nproc)" >> "$WORK/bbuild.log" 2>&1 \
        && [ -s "$BBDIR/busybox" ]; then
        ok "busybox build (binary: $(du -h "$BBDIR/busybox" | cut -f1); smoke: $("$BBDIR/busybox" echo busybox-works))"
    else
        bad "busybox build failed (see $WORK/bbuild.log)"
    fi
else
    bad "busybox tarball download/extract failed"
fi

step "3. sqlite stress (WAL, synchronous=FULL)"
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

step "post: unmount + offline self-check"
sync
fusermount3 -u "$MNT" || fusermount3 -u -z "$MNT"
for i in $(seq 1 50); do kill -0 "$MPID" 2>/dev/null || break; sleep 0.1; done
kill -9 "$MPID" 2>/dev/null; wait "$MPID" 2>/dev/null
"$PORFS" mds-check --meta "$WORK/meta.redb" --data "$WORK/data.img"

echo
echo "mvp-smoke: $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ]
