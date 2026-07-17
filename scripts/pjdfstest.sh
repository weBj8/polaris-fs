#!/usr/bin/env bash
# pjdfstest gate for P5: build pjd/pjdfstest from source and run the full
# POSIX suite against a real porfs mount, inside a user namespace so the
# privileged suites (chown/mknod) run without host root.
#
# Skips (printed, counted, never hidden):
#   - chflags: BSD-only flags (UF_IMMUTABLE & co.), no Linux VFS equivalent.
#
# Usage: scripts/pjdfstest.sh [suite ...]   (default: every suite except chflags)
set -u

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK="${CARGO_TARGET_TMPDIR:-$REPO_ROOT/target}/pjdfstest-work"
SRC="$REPO_ROOT/target/pjdfstest-src"
PORFS="$REPO_ROOT/target/debug/porfs"
SYNC_DIR="$WORK/ns-sync"
# The mountpoint must be traversable (o+x) by arbitrary uids — pjdfstest
# drops to uid 65534, and any non-traversable parent component (e.g. a
# 0700 home directory) turns every open into EACCES. /tmp qualifies.
MNT="/tmp/porfs-pjdfstest-mnt.$$"
mkdir -p "$WORK" "$SYNC_DIR" "$MNT"

cleanup() {
    fusermount3 -u -z "$MNT" 2>/dev/null
    [ -n "${NS_PID:-}" ] && kill "$NS_PID" 2>/dev/null
    rm -rf "$MNT" "$WORK/meta.redb" "$WORK/data.img" "$SYNC_DIR"
}
trap cleanup EXIT

# ---- 1. build pjdfstest (idempotent) ----
if [ ! -x "$SRC/pjdfstest" ]; then
    if [ ! -d "$SRC/.git" ]; then
        rm -rf "$SRC"
        git clone --depth 1 https://github.com/pjd/pjdfstest "$SRC" || exit 1
    fi
    (cd "$SRC" && autoreconf -ifs && ./configure && make) || exit 1
fi

# ---- 2. decide suites ----
if [ "$#" -gt 0 ]; then
    SUITES="$*"
else
    SUITES="$(cd "$SRC/tests" && ls -d */ | tr -d '/' | grep -v '^chflags$' | tr '\n' ' ')"
fi
echo "pjdfstest: suites: $SUITES"
echo "pjdfstest: skipped: chflags (BSD-only flags, no Linux VFS equivalent)"

# ---- 3. fresh MDS pair (and stale prove logs from previous runs) ----
rm -f "$WORK/meta.redb" "$WORK/data.img" "$WORK"/prove-*.log
"$PORFS" mkfs --device "$WORK/data.img" --meta "$WORK/meta.redb" --size 512MiB >/dev/null || exit 1

# ---- 4. mount + run ----
if [ "$(id -u)" -eq 0 ]; then
    # Root mode (upstream-intended): a direct mount with allow_other gives
    # full credential semantics — no environment-limited failures.
    "$PORFS" mount --meta "$WORK/meta.redb" --data "$WORK/data.img" \
        --mountpoint "$MNT" --allow-other --attr-ttl 0 --entry-ttl 0 > "$WORK/mount.log" 2>&1 &
    MPID=$!
    for i in $(seq 1 100); do
        findmnt -rn "$MNT" >/dev/null 2>&1 && break
        sleep 0.1
    done
    findmnt -rn "$MNT" >/dev/null 2>&1 || { echo "pjdfstest: mount failed" >&2; cat "$WORK/mount.log" >&2; exit 1; }
    rc=0
    (cd "$MNT" && for suite in $SUITES; do
        prove -rv "$SRC/tests/$suite" > "$WORK/prove-$suite.log" 2>&1 || rc=1
    done) || rc=1
    RC=$rc
    kill "$MPID" 2>/dev/null
    for i in $(seq 1 50); do kill -0 "$MPID" 2>/dev/null || break; sleep 0.1; done
    kill -9 "$MPID" 2>/dev/null
    wait "$MPID" 2>/dev/null
else
    # Rootless mode: user namespace with a full uid/gid range map. Two
    # kernel-forced environment limits apply (counted in the summary, not
    # hidden): device mknod is EPERM (userns FUSE mounts are nodev), and
    # requests carrying pjdfstest's uid-only credentials get kernel-side
    # EACCES where regular filesystems give EPERM — both disappear in root
    # mode (`sudo scripts/pjdfstest.sh`).
    rm -f "$SYNC_DIR/pid" "$SYNC_DIR/mapped" "$SYNC_DIR/done" "$SYNC_DIR/exit"
cat > "$WORK/inner.sh" << INNER
#!/usr/bin/env bash
echo \$\$ > "$SYNC_DIR/pid"
while [ ! -e "$SYNC_DIR/mapped" ]; do sleep 0.05; done
"$PORFS" mount --meta "$WORK/meta.redb" --data "$WORK/data.img" \
    --mountpoint "$MNT" --allow-other --attr-ttl 0 --entry-ttl 0 > "$WORK/mount.log" 2>&1 &
MPID=\$!
for i in \$(seq 1 100); do
    findmnt -rn "$MNT" >/dev/null 2>&1 && break
    sleep 0.1
done
if ! findmnt -rn "$MNT" >/dev/null 2>&1; then
    echo "pjdfstest: FUSE mount failed inside userns" >&2
    cat "$WORK/mount.log" >&2
    echo 1 > "$SYNC_DIR/exit"; touch "$SYNC_DIR/done"; exit 1
fi
cd "$MNT" || exit 1
rc=0
for suite in $SUITES; do
    prove -rv "$SRC/tests/\$suite" > "$WORK/prove-\$suite.log" 2>&1 || rc=1
done
sync
# fusermount3 cannot unmount inside the namespace (it consults the HOST
# /etc/mtab, which never contains ns-private mounts); stop the daemon
# instead — closing its fuse fd ends the connection and the mount dies
# with the namespace.
kill "\$MPID" 2>/dev/null
for i in \$(seq 1 50); do kill -0 "\$MPID" 2>/dev/null || break; sleep 0.1; done
kill -9 "\$MPID" 2>/dev/null
wait \$MPID 2>/dev/null
echo \$rc > "$SYNC_DIR/exit"; touch "$SYNC_DIR/done"
INNER
chmod +x "$WORK/inner.sh"

unshare -U -m "$WORK/inner.sh" &
NS_PID=$!
for i in $(seq 1 100); do [ -e "$SYNC_DIR/pid" ] && break; sleep 0.05; done
[ -e "$SYNC_DIR/pid" ] || { echo "pjdfstest: userns failed to start" >&2; exit 1; }
NS_INNER_PID="$(cat "$SYNC_DIR/pid")"
# Map ns uid 0 -> us, and ns 1..65535 -> our /etc/subuid range, so the
# chown/chmod suites can exercise arbitrary uids.
newuidmap "$NS_INNER_PID" 0 "$(id -u)" 1 1 100000 65535 || { echo "pjdfstest: newuidmap failed" >&2; exit 1; }
newgidmap "$NS_INNER_PID" 0 "$(id -g)" 1 1 100000 65535 || { echo "pjdfstest: newgidmap failed" >&2; exit 1; }
touch "$SYNC_DIR/mapped"
for i in $(seq 1 1200); do [ -e "$SYNC_DIR/done" ] && break; sleep 0.5; done
kill "$NS_PID" 2>/dev/null
wait "$NS_PID" 2>/dev/null
[ -e "$SYNC_DIR/exit" ] || { echo "pjdfstest: suite timed out" >&2; exit 1; }
RC="$(cat "$SYNC_DIR/exit")"
fi

# ---- 5. summary ----
total_pass=0; total_fail=0
for suite in $SUITES; do
    log="$WORK/prove-$suite.log"
    [ -f "$log" ] || { echo "pjdfstest: $suite: NO LOG"; total_fail=$((total_fail+1)); continue; }
    line="$(grep -E '^Result: |^Files=' "$log" | tail -1)"
    fails="$(grep -cE '^not ok' "$log")"
    pass="$(grep -cE '^ok' "$log")"
    total_pass=$((total_pass+pass)); total_fail=$((total_fail+fails))
    printf "pjdfstest: %-16s ok=%-6s not_ok=%-4s %s\n" "$suite" "$pass" "$fails" "$line"
done
echo "pjdfstest: TOTAL ok=$total_pass not_ok=$total_fail (skipped suite: chflags)"
[ "$RC" -eq 0 ] && [ "$total_fail" -eq 0 ]
