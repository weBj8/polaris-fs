#!/usr/bin/env bash
# Old-vs-new metalogger comparison against one live master.
# Phase 1 builds real metadata (master + FUSE client, file operations,
# clean master stop saves metadata.mfs). Phase 2 restarts the master and
# runs the reference metalogger and the candidate side by side as two
# metaloggers of that master while the client keeps changing the
# namespace; both are restarted once (exercises the changelog_ml.0.back
# version scan via the master's rotation files) and stopped cleanly.
# They must download byte-identical metadata / changelog backups, record
# identical changelog streams over the common version range, and log the
# same lines (timings and per-dir paths normalized).
# Needs: target/release/{plfsmaster,plfschunkserver,plfsmount}, /dev/fuse,
# root or `unshare -rm`, LD_LIBRARY_PATH for libfuse >= 3.17 if needed.
# usage: tools/metalogger_diff.sh <old-plfsmetalogger> [new-plfsmetalogger]
set -euo pipefail
cd "$(dirname "$0")/.."
OLD=$(realpath "$1")
NEW=$(realpath "${2:-target/release/plfsmetalogger}")
BIN=$(realpath target/release)
W=$(realpath -m target/mldiff)
ML_PORT=19519 CS_PORT=19520 CL_PORT=19521 CSS_PORT=19522
HOST_IP=$(ip -4 route get 1.1.1.1 2>/dev/null | sed -n 's/.*src \([0-9.]*\).*/\1/p')
HOST_IP=${HOST_IP:-127.0.0.1}
rm -rf "$W"; mkdir -p "$W"/{master,old,new,cs/hdd,mnt}
MPID="" CPID="" OPID="" NPID=""
cleanup() {
  set +e
  for p in $OPID $NPID $CPID $MPID; do kill "$p" 2>/dev/null; done
  umount "$W/mnt" 2>/dev/null
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
waitfor() { for _ in $(seq 1 60); do eval "$1" && return 0; sleep 0.5; done; echo "timeout: $1"; return 1; }

printf 'MFSM NEW' >"$W/master/metadata.mfs"
printf '* / rw,alldirs,admin,maproot=0:0\n' >"$W/master/mfsexports.cfg"
mkcfg "$W/master" "EXPORTS_FILENAME = $W/master/mfsexports.cfg
MATOML_LISTEN_PORT = $ML_PORT
MATOCS_LISTEN_PORT = $CS_PORT
MATOCL_LISTEN_PORT = $CL_PORT"
mkcfg "$W/cs" "MASTER_HOST = $HOST_IP
MASTER_PORT = $CS_PORT
CSSERV_LISTEN_PORT = $CSS_PORT
HDD_CONF_FILENAME = $W/cs/mfshdd.cfg"
echo "$W/cs/hdd" >"$W/cs/mfshdd.cfg"
for v in old new; do
  mkcfg "$W/$v" "MASTER_HOST = 127.0.0.1
MASTER_PORT = $ML_PORT
BACK_LOGS = 5"
done

start_master() { "$BIN/plfsmaster" -f -c "$W/master/mfs.cfg" >>"$W/master.log" 2>&1 & MPID=$!; sleep 1; }
stop_master() { kill -TERM "$MPID"; wait "$MPID" 2>/dev/null || true; MPID=""; }
start_ml() { "$OLD" -f -c "$W/old/mfs.cfg" >>"$W/old.log" 2>&1 & OPID=$!; "$NEW" -f -c "$W/new/mfs.cfg" >>"$W/new.log" 2>&1 & NPID=$!; }
stop_ml() { kill -TERM "$OPID" "$NPID"; wait "$OPID" "$NPID" 2>/dev/null || true; OPID="" NPID=""; }

# client workload, run as root or inside a user+mount namespace
cat >"$W/client.sh" <<EOF
set -euo pipefail
"$BIN/plfsmount" -f -H 127.0.0.1 -P $CL_PORT "$W/mnt" >>"$W/mount.log" 2>&1 &
P=\$!
trap 'kill \$P 2>/dev/null || true; wait \$P 2>/dev/null || true; umount "$W/mnt" 2>/dev/null || true' EXIT
for i in \$(seq 1 30); do mountpoint -q "$W/mnt" && break; sleep 0.5; done
mountpoint -q "$W/mnt"
for i in \$(seq 1 \$1); do
  mkdir -p "$W/mnt/d\$2"; echo "x\$i" >"$W/mnt/d\$2/f\$i"; ln -s "f\$i" "$W/mnt/d\$2/s\$i"
  mv "$W/mnt/d\$2/f\$i" "$W/mnt/d\$2/g\$i"
done
rm -f "$W/mnt/d\$2"/s*
EOF
client() { if [ "$(id -u)" = 0 ]; then bash "$W/client.sh" "$@"; else unshare -rm bash "$W/client.sh" "$@"; fi; }

# phase 1: real metadata on disk
start_master
"$BIN/plfschunkserver" -f -c "$W/cs/mfs.cfg" >"$W/cs.log" 2>&1 & CPID=$!
waitfor "grep -q 'register end' '$W/master.log'"
client 20 1
kill -TERM "$CPID"; wait "$CPID" 2>/dev/null || true; CPID=""
stop_master

# phase 2: two metaloggers on the restarted master, workload in between
start_master
"$BIN/plfschunkserver" -f -c "$W/cs/mfs.cfg" >>"$W/cs.log" 2>&1 & CPID=$!
start_ml
waitfor "[ -e '$W/old/changelog_ml_back.1.mfs' ] && [ -e '$W/new/changelog_ml_back.1.mfs' ]"
client 30 2
sleep 2
stop_ml
# restart: both scan changelog_ml.0.back (absent -> full re-download)
start_ml
waitfor "grep -c 'changelog_1 downloaded' '$W/old.log' | grep -q 2 && grep -c 'changelog_1 downloaded' '$W/new.log' | grep -q 2"
client 10 3
sleep 2
stop_ml
kill -TERM "$CPID"; wait "$CPID" 2>/dev/null || true; CPID=""
stop_master

fail=0
for f in metadata_ml.mfs.back metadata_ml.mfs.back.1 changelog_ml_back.0.mfs changelog_ml_back.1.mfs; do
  if [ -e "$W/old/$f" ] || [ -e "$W/new/$f" ]; then
    cmp -s "$W/old/$f" "$W/new/$f" && echo "same: $f ($(wc -c <"$W/old/$f") B)" || { echo "DIFF: $f"; fail=1; }
  fi
done
[ -s "$W/old/metadata_ml.mfs.back" ] || { echo "no metadata downloaded"; fail=1; }
# changelog streams: the metaloggers connect a few ms apart; compare the
# common version range.
o="$W/old/changelog_ml.0.mfs"; n="$W/new/changelog_ml.0.mfs"
if [ -s "$o" ] && [ -s "$n" ]; then
  so=$(head -1 "$o" | cut -d: -f1); sn=$(head -1 "$n" | cut -d: -f1)
  eo=$(tail -1 "$o" | cut -d: -f1); en=$(tail -1 "$n" | cut -d: -f1)
  start=$(( so > sn ? so : sn )); end=$(( eo < en ? eo : en ))
  sel() { awk -F: -v s="$start" -v e="$end" '$1+0>=s && $1+0<=e' "$1"; }
  if diff <(sel "$o") <(sel "$n") >/dev/null; then
    echo "same: changelog_ml.0.mfs versions $start..$end ($(sel "$o" | wc -l) lines)"
  else
    echo "DIFF: changelog_ml.0.mfs"; fail=1
  fi
else
  echo "DIFF/EMPTY: changelog_ml.0.mfs"; fail=1
fi
norm() {
  sed -E -e 's/[0-9]+\.[0-9]+s \([0-9.]+ MB\/s\)/T/' \
         -e 's/monotonic clock speed: .*/monotonic clock speed: N/' \
         -e "s#$W/(old|new)#DIR#" "$1"
}
diff <(norm "$W/old.log") <(norm "$W/new.log") >"$W/log.diff" && echo "same: log lines ($(wc -l <"$W/old.log"))" || { echo "DIFF: logs"; cat "$W/log.diff"; fail=1; }
[ $fail = 0 ] && echo "METALOGGER DIFF OK"
exit $fail
