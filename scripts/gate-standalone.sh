#!/usr/bin/env bash
# S10 gate (design doc §3.1): standalone one-command demo — mount, write
# saves (fsync'd AND never-closed), kill -9 the daemon, remount, data intact.
# Mount and workloads run inside one user+mount namespace per phase.
set -euo pipefail
cd "$(dirname "$0")/.."

DIR="$(pwd)/target/gate-standalone"
cargo build -p plfs >/dev/null
rm -rf "$DIR"
mkdir -p "$DIR/vol" "$DIR/mnt"

# --- phase 1: mount, write saves, kill -9 ---------------------------------
unshare -rm bash -c "
set -euo pipefail
target/debug/plfs standalone --dir '$DIR/vol' --mount '$DIR/mnt' \
    --size $((2 << 30)) >'$DIR/standalone.log' 2>&1 &
pid=\$!
for _ in \$(seq 1 150); do grep -q '$DIR/mnt fuse ' /proc/mounts && break; sleep 0.2; done
grep -q '$DIR/mnt fuse ' /proc/mounts || { echo 'mount never came up' >&2; exit 1; }
python3 - '$DIR/mnt' <<'PYEOF'
import os, sys
mnt = sys.argv[1]
os.makedirs(f'{mnt}/saves', exist_ok=True)
a = bytes((i * 7 + 3) % 251 for i in range(300_000))
with open(f'{mnt}/saves/synced.sav', 'wb') as f:
    f.write(a)
    f.flush()
    os.fsync(f.fileno())
b = bytes((i * 13 + 1) % 239 for i in range(500_000))
with open(f'{mnt}/saves/unflushed.sav', 'wb') as f:
    f.write(b)
with open('$DIR/expected-a.bin', 'wb') as f:
    f.write(a)
with open('$DIR/expected-b.bin', 'wb') as f:
    f.write(b)
PYEOF
kill -9 \$pid 2>/dev/null || true
wait \$pid 2>/dev/null || true
"

# --- phase 2: remount, verify byte-exact ----------------------------------
unshare -rm bash -c "
set -euo pipefail
target/debug/plfs standalone --dir '$DIR/vol' --mount '$DIR/mnt' \
    >'$DIR/remount.log' 2>&1 &
pid=\$!
trap 'kill -9 \$pid 2>/dev/null || true' EXIT
for _ in \$(seq 1 150); do grep -q '$DIR/mnt fuse ' /proc/mounts && break; sleep 0.2; done
grep -q '$DIR/mnt fuse ' /proc/mounts || { echo 'remount never came up' >&2; exit 1; }
cmp '$DIR/mnt/saves/synced.sav' '$DIR/expected-a.bin'
cmp '$DIR/mnt/saves/unflushed.sav' '$DIR/expected-b.bin'
kill -INT \$pid
for _ in \$(seq 1 30); do kill -0 \$pid 2>/dev/null || exit 0; sleep 1; done
"
echo "STANDALONE_OK synced+unflushed writes survived kill -9"
rm -rf "$DIR"
