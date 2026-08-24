#!/usr/bin/env bash
# Container entrypoint: generate a minimal MooseFS config from env and exec
# the daemon in the foreground.
#
# Common env:
#   ROLE            plfsmaster | plfsmetalogger | plfschunkserver | ...
#   DATA_PATH       daemon working dir inside the container (default /var/lib/plfs)
#   MASTER_HOST     master address as seen by this daemon (chunkserver/metalogger/mount)
#   EXPORTS         mfsexports.cfg content (master only; default: allow all rw)
#   HDD_PATHS       space-separated chunk dirs (chunkserver only; default /mnt/hdd)
#   EXTRA_CFG       extra lines appended to the daemon cfg
#   EXTRA_ARGS     extra daemon CLI args (master default: -a)
set -euo pipefail

ROLE="${ROLE:-plfsmaster}"
DATA_PATH="${DATA_PATH:-/var/lib/plfs}"
mkdir -p "$DATA_PATH"
CFG="$DATA_PATH/mfs.cfg"

common() {
    cat >"$CFG" <<EOF
WORKING_USER = root
WORKING_GROUP = root
DATA_PATH = $DATA_PATH
EOF
    if [ -n "${EXTRA_CFG:-}" ]; then echo "$EXTRA_CFG" >>"$CFG"; fi
}

case "$ROLE" in
plfsmaster)
    EXTRA_ARGS="${EXTRA_ARGS:--a}"  # -a: auto-restore metadata from backup
    common
    echo "EXPORTS_FILENAME = $DATA_PATH/mfsexports.cfg" >>"$CFG"
    cat >"$DATA_PATH/mfsexports.cfg" <<EOF
${EXPORTS:-"*                       /       rw,alldirs,admin,maproot=0:0
*                       .       rw"}
EOF
    # empty metadata seed on first start
    if [ ! -s "$DATA_PATH/metadata.mfs" ]; then
        printf 'MFSM NEW' >"$DATA_PATH/metadata.mfs"
    fi
    ;;
plfsmetalogger)
    : "${MASTER_HOST:?set MASTER_HOST}"
    common
    echo "MASTER_HOST = $MASTER_HOST" >>"$CFG"
    ;;
plfschunkserver)
    : "${MASTER_HOST:?set MASTER_HOST}"
    common
    echo "MASTER_HOST = $MASTER_HOST" >>"$CFG"
    echo "HDD_CONF_FILENAME = $DATA_PATH/mfshdd.cfg" >>"$CFG"
    : "${HDD_PATHS:=/mnt/hdd}"
    : >"$DATA_PATH/mfshdd.cfg"
    for d in $HDD_PATHS; do
        mkdir -p "$d"
        echo "$d" >>"$DATA_PATH/mfshdd.cfg"
    done
    ;;
plfsgui)
    common
    echo "ROOT_DIR = ${ROOT_DIR:-/usr/share/plfscgi}" >>"$CFG"
    ;;
plfsmount)
    : "${MASTER_HOST:?set MASTER_HOST}"
    : "${MOUNT_POINT:=/mnt/plfs}"
    mkdir -p "$MOUNT_POINT"
    exec plfsmount -f -o allow_other -H "$MASTER_HOST" ${MOUNT_OPTS:-} "$MOUNT_POINT"
    ;;
*)
    # other roles: run with whatever args were passed
    exec "$ROLE" "$@"
    ;;
esac

exec "$ROLE" -f ${EXTRA_ARGS:-} -c "$CFG"
