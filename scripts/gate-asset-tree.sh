#!/usr/bin/env bash
# S9 gate: a game-asset-style directory tree round-trips a real FUSE mount
# (bulk write → unmount → remount → byte-exact manifest verify).
# Mount and workloads run inside one user+mount namespace per phase.
set -euo pipefail
cd "$(dirname "$0")/.."

DIR="$(pwd)/target/gate-asset-tree"
FILES="${FILES:-200}"
PHASE_TIMEOUT="${PHASE_TIMEOUT:-120}"

cargo build -p plfs >/dev/null
rm -rf "$DIR"
mkdir -p "$DIR/vol" "$DIR/mnt"

target/debug/plfs mkfs --dir "$DIR/vol" --size $((4 << 30)) >/dev/null 2>&1

# --- phase 1: mount, write the tree, manifest, unmount --------------------
unshare -rm bash -c "
set -euo pipefail
target/debug/plfs standalone --dir '$DIR/vol' --mount '$DIR/mnt' >'$DIR/mount.log' 2>&1 &
pid=\$!
trap 'kill -9 \$pid 2>/dev/null || true' EXIT
for _ in \$(seq 1 150); do grep -q '$DIR/mnt fuse ' /proc/mounts && break; sleep 0.2; done
grep -q '$DIR/mnt fuse ' /proc/mounts || { echo 'mount never came up' >&2; exit 1; }
timeout $PHASE_TIMEOUT python3 - '$DIR/mnt' '$FILES' <<'PYEOF'
import os, random, sys
root, count = sys.argv[1], int(sys.argv[2])
rng = random.Random(20260719)
layout = {
    'packs': (64 * 1024, 4 * 1024 * 1024),
    'textures': (256 * 1024, 2 * 1024 * 1024),
    'configs': (512, 64 * 1024),
}
for group, (lo, hi) in layout.items():
    os.makedirs(f'{root}/{group}', exist_ok=True)
    for i in range(count // 3):
        size = rng.randint(lo, hi)
        with open(f'{root}/{group}/asset_{i:04d}.bin', 'wb') as f:
            f.write(rng.randbytes(size))
PYEOF
(cd '$DIR/mnt' && find . -type f -print0 | sort -z | xargs -0 sha256sum) > '$DIR/manifest.txt'
kill -INT \$pid
for _ in \$(seq 1 30); do kill -0 \$pid 2>/dev/null || exit 0; sleep 1; done
"
echo "wrote $(wc -l < "$DIR/manifest.txt") files; remounting..."

# --- phase 2: remount, verify byte-exact ----------------------------------
unshare -rm bash -c "
set -euo pipefail
target/debug/plfs standalone --dir '$DIR/vol' --mount '$DIR/mnt' >'$DIR/remount.log' 2>&1 &
pid=\$!
trap 'kill -9 \$pid 2>/dev/null || true' EXIT
for _ in \$(seq 1 150); do grep -q '$DIR/mnt fuse ' /proc/mounts && break; sleep 0.2; done
grep -q '$DIR/mnt fuse ' /proc/mounts || { echo 'remount never came up' >&2; exit 1; }
(cd '$DIR/mnt' && timeout $PHASE_TIMEOUT sha256sum -c '$DIR/manifest.txt' --quiet)
kill -INT \$pid
for _ in \$(seq 1 30); do kill -0 \$pid 2>/dev/null || exit 0; sleep 1; done
"
echo "ASSET_TREE_OK files=$(wc -l < "$DIR/manifest.txt")"
rm -rf "$DIR"
