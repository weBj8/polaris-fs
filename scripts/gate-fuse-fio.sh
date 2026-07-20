#!/usr/bin/env bash
# S9 gate: fio randrw with crc32 verification on a real FUSE mount.
# Mount and the fio workload run inside one user+mount namespace.
set -euo pipefail
cd "$(dirname "$0")/.."

DIR="$(pwd)/target/gate-fio"
SIZE="${SIZE:-4M}"
RUNTIME="${RUNTIME:-10}"

cargo build -p plfs >/dev/null
rm -rf "$DIR"
mkdir -p "$DIR/vol" "$DIR/mnt"

target/debug/plfs mkfs --dir "$DIR/vol" --size $((4 << 30)) >/dev/null 2>&1

unshare -rm bash -c "
set -euo pipefail
target/debug/plfs standalone --dir '$DIR/vol' --mount '$DIR/mnt' >'$DIR/mount.log' 2>&1 &
pid=\$!
trap 'kill -9 \$pid 2>/dev/null || true' EXIT
for _ in \$(seq 1 150); do grep -q '$DIR/mnt fuse ' /proc/mounts && break; sleep 0.2; done
grep -q '$DIR/mnt fuse ' /proc/mounts || { echo 'mount never came up' >&2; exit 1; }
fio --name=randrw --directory='$DIR/mnt' --rw=randrw --bs=4k --size='$SIZE' \
    --numjobs=2 --runtime='$RUNTIME' --time_based --verify=crc32 \
    --verify_fatal=1 --group_reporting --output-format=terse > '$DIR/fio.log' 2>&1
kill -INT \$pid
for _ in \$(seq 1 30); do kill -0 \$pid 2>/dev/null || exit 0; sleep 1; done
"
echo "FIO_FUSE_OK size=$SIZE runtime=${RUNTIME}s"
rm -rf "$DIR"
