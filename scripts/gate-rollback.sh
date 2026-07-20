#!/usr/bin/env bash
# S16 gate (design doc §9): rollback restores byte-exact state and is itself
# reversible; deleting a snapshot re-enqueues exclusively-referenced chunks
# (du drops); the retention scheduler creates and prunes on schedule.
set -euo pipefail
cd "$(dirname "$0")/.."

DIR="$(pwd)/target/gate-rollback"
cargo build -p plfs-client --example rollback_driver >/dev/null
rm -rf "$DIR"
mkdir -p "$DIR"

OUT=$(target/debug/examples/rollback_driver "$DIR/vol1" rollback)
echo "$OUT"
C=$(echo "$OUT" | sed -n 's/.*c=\([0-9]*\).*/\1/p')

DU1=$(du -B1 "$DIR/vol1/arena.img" | cut -f1)
target/debug/examples/rollback_driver "$DIR/vol1" reclaim "$C"
DU2=$(du -B1 "$DIR/vol1/arena.img" | cut -f1)
echo "arena du: $DU1 -> $DU2 (reclaimed $((DU1 - DU2)) B)"
test $((DU1 - DU2)) -gt 30000000

target/debug/examples/rollback_driver "$DIR/vol2" schedule
echo "ROLLBACK_GATE_OK"
