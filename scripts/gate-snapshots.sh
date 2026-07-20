#!/usr/bin/env bash
# S15 gate (design doc §9): snapshot over 8 files → delete half, rewrite the
# other half → the snapshot view is still byte-exact for all 8 originals,
# and the FUSE `.snapshots` tree serves the pre-snapshot sentinel.
set -euo pipefail
cd "$(dirname "$0")/.."

DIR="$(pwd)/target/gate-snapshots"
cargo build -p plfs >/dev/null
cargo build -p plfs-client --example snapshot_driver >/dev/null
rm -rf "$DIR"
mkdir -p "$DIR" "$DIR/mnt"
target/debug/examples/snapshot_driver "$DIR/vol0" --keep

# The FUSE `.snapshots` view (mount+checks in one namespace; this kernel
# blocks fusermount3 outside one).
unshare -rm bash -s <<EOF
set -euo pipefail
cd "$(pwd)"
target/debug/plfs mount --dir "$DIR/vol0" "$DIR/mnt" > "$DIR/mount.log" 2>&1 &
for i in \$(seq 1 50); do grep -q "$DIR/mnt" /proc/mounts && break; sleep 0.1; done
grep -q "$DIR/mnt" /proc/mounts
ls "$DIR/mnt/.snapshots/gate-s1/" | grep -q sentinel.txt
test "\$(cat "$DIR/mnt/.snapshots/gate-s1/sentinel.txt")" = "snapshot-gate-sentinel-v1"
test "\$(cat "$DIR/mnt/sentinel.txt")" = "snapshot-gate-sentinel-v2"
echo FUSE_SNAPSHOT_VIEW_OK
EOF

echo "SNAPSHOT_GATE_OK"
