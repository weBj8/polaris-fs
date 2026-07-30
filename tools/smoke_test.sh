#!/usr/bin/env bash
# Cluster smoke gate — the behavioral oracle for every migration PR.
# Boots master + chunkserver + metalogger from dist/, FUSE-mounts (as root:
# direct; unprivileged: inside `unshare -rm`), exercises write/read (md5-verified), mkdir/ln/mv/rm/symlink, df; tears down.
# Requires: dist/ binaries (./build-all.sh), /dev/fuse, unshare, and
# LD_LIBRARY_PATH pointing at a libfuse3 >= 3.17 if the system one is older.
# Usage: tools/smoke_test.sh [workdir]   (default: target/smoke)
set -euo pipefail
cd "$(dirname "$0")/.."

W="${1:-target/smoke}"
mkdir -p "$W"; W=$(realpath "$W")
BIN=$(realpath dist)
ML_PORT=19419 CS_PORT=19420 CL_PORT=19421 CSS_PORT=19422
# chunkserver refuses a loopback MASTER_HOST ("use ip address of network controller")
HOST_IP=$(ip -4 route get 1.1.1.1 2>/dev/null | sed -n 's/.*src \([0-9.]*\).*/\1/p')
HOST_IP=${HOST_IP:-127.0.0.1}

for b in mfsmaster mfschunkserver mfsmetalogger mfsmount; do
  [ -x "$BIN/$b" ] || { echo "missing $BIN/$b — run ./build-all.sh"; exit 1; }
done
[ -e /dev/fuse ] || { echo "/dev/fuse unavailable"; exit 1; }

rm -rf "$W"; mkdir -p "$W"/{master,metalogger,cs/hdd,mnt}
PIDS=""
cleanup() {
  set +e
  for p in $PIDS; do kill "$p" 2>/dev/null; done
  pkill -f "$BIN/mfsmount" 2>/dev/null
  wait 2>/dev/null
}
trap cleanup EXIT

mkcfg() {
  cat >"$1/mfs.cfg" <<EOF
WORKING_USER = $(id -un)
WORKING_GROUP = $(id -gn)
DATA_PATH = $1
$2
EOF
}

printf 'MFSM NEW' >"$W/master/metadata.mfs"
printf '* / rw,alldirs,admin,maproot=0:0\n* . rw\n' >"$W/master/mfsexports.cfg"
mkcfg "$W/master" "EXPORTS_FILENAME = $W/master/mfsexports.cfg
MATOML_LISTEN_PORT = $ML_PORT
MATOCS_LISTEN_PORT = $CS_PORT
MATOCL_LISTEN_PORT = $CL_PORT"
"$BIN/mfsmaster" -f -c "$W/master/mfs.cfg" >"$W/master.log" 2>&1 &
PIDS="$PIDS $!"

mkcfg "$W/cs" "MASTER_HOST = $HOST_IP
MASTER_PORT = $CS_PORT
CSSERV_LISTEN_PORT = $CSS_PORT
HDD_CONF_FILENAME = $W/cs/mfshdd.cfg"
echo "$W/cs/hdd" >"$W/cs/mfshdd.cfg"
"$BIN/mfschunkserver" -f -c "$W/cs/mfs.cfg" >"$W/cs.log" 2>&1 &
PIDS="$PIDS $!"

mkcfg "$W/metalogger" "MASTER_HOST = 127.0.0.1
MASTER_PORT = $ML_PORT"
"$BIN/mfsmetalogger" -f -c "$W/metalogger/mfs.cfg" >"$W/metalogger.log" 2>&1 &
PIDS="$PIDS $!"

for i in $(seq 1 30); do
  grep -q 'registered' "$W/cs.log" 2>/dev/null && break
  sleep 0.5
done

# mount + exercise: needs privilege for /dev/fuse. As real root (CI: sudo),
# libfuse mounts directly — no namespace needed. Unprivileged: wrap in a
# user+mount namespace (`unshare -rm`) where euid 0 mounts without setuid
# fusermount3. Net namespace is never unshared, so 127.0.0.1 reaches daemons.
cat >"$W/inner.sh" <<EOF
set -euo pipefail
export LD_LIBRARY_PATH="\${LD_LIBRARY_PATH:-}"
"$BIN/mfsmount" -f -H 127.0.0.1 -P $CL_PORT "$W/mnt" >"$W/mount.log" 2>&1 &
MPID=\$!
trap 'kill \$MPID 2>/dev/null || true; wait \$MPID 2>/dev/null || true; umount "$W/mnt" 2>/dev/null || true' EXIT
for i in \$(seq 1 30); do mountpoint -q "$W/mnt" && break; sleep 0.5; done
mountpoint -q "$W/mnt" || { echo "mount failed"; cat "$W/mount.log"; exit 1; }
dd if=/dev/urandom of="$W/ref.bin" bs=1M count=8 status=none
cp "$W/ref.bin" "$W/mnt/f1"
mkdir "$W/mnt/d1"; mv "$W/mnt/f1" "$W/mnt/d1/f2"
ln "$W/mnt/d1/f2" "$W/mnt/d1/hard"; ln -s d1/f2 "$W/mnt/sym"
cmp "$W/ref.bin" "$W/mnt/d1/f2"
df "$W/mnt" >/dev/null
rm "$W/mnt/d1/hard" "$W/mnt/sym" "$W/mnt/d1/f2"; rmdir "$W/mnt/d1"
echo "INNER OK"
EOF
if [ "$(id -u)" = 0 ]; then
  bash "$W/inner.sh" || { echo "inner smoke FAILED"; exit 1; }
else
  unshare -rm bash "$W/inner.sh" || { echo "inner smoke FAILED"; exit 1; }
fi

echo "SMOKE OK"
