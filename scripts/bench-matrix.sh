#!/usr/bin/env bash
# S20 bench matrix (design doc §11; review P1-6): bounded boot (checkpoint
# vs scan), 4 KiB random-overwrite write amplification, save-file sizes,
# cold/warm asset load. Emits target/bench-report.md.
set -euo pipefail
cd "$(dirname "$0")/.."

DIR="$(pwd)/target/bench"
REPORT="$(pwd)/target/bench-report.md"
cargo build --release -p plfs >/dev/null
cargo build --release -p plfs-arena --example boot_bench >/dev/null
rm -rf "$DIR"
mkdir -p "$DIR/vol" "$DIR/mnt"

B_SMALL=$(target/release/examples/boot_bench "$DIR/boot-small" 1500 4)
B_LARGE=$(target/release/examples/boot_bench "$DIR/boot-large" 8000 12)

target/release/plfs mkfs --dir "$DIR/vol" --size $((8 << 30)) >/dev/null 2>&1

unshare -rm bash -c "
set -euo pipefail
target/release/plfs standalone --dir '$DIR/vol' --mount '$DIR/mnt' >'$DIR/mount.log' 2>&1 &
pid=\$!
trap 'kill -9 \$pid 2>/dev/null || true' EXIT
for _ in \$(seq 1 150); do grep -q '$DIR/mnt fuse ' /proc/mounts && break; sleep 0.2; done
grep -q '$DIR/mnt fuse ' /proc/mounts

fio --name=w4k --directory='$DIR/mnt' --rw=randwrite --bs=4k --size=64m --direct=1 \
    --group_reporting --output='$DIR/fio-w4k.log' >/dev/null 2>&1
fio --name=saves8k --directory='$DIR/mnt' --rw=write --bs=8k --size=64m --direct=1 \
    --group_reporting --output='$DIR/fio-saves8k.log' >/dev/null 2>&1
fio --name=saves64k --directory='$DIR/mnt' --rw=write --bs=64k --size=256m --direct=1 \
    --group_reporting --output='$DIR/fio-saves64k.log' >/dev/null 2>&1
fio --name=assetw --directory='$DIR/mnt' --rw=write --bs=1m --size=1g --direct=1 \
    --group_reporting --output='$DIR/fio-assetw.log' >/dev/null 2>&1
sync
fio --name=assetr1 --directory='$DIR/mnt' --rw=read --bs=1m --size=1g --direct=1 \
    --group_reporting --output='$DIR/fio-assetr1.log' >/dev/null 2>&1
fio --name=assetr2 --directory='$DIR/mnt' --rw=read --bs=1m --size=1g --direct=1 \
    --group_reporting --output='$DIR/fio-assetr2.log' >/dev/null 2>&1
kill -INT \$pid
for _ in \$(seq 1 30); do kill -0 \$pid 2>/dev/null || break; sleep 1; done
"

DU=$(du -B1 "$DIR/vol/arena.img" | cut -f1)
bw() { grep -oP '(WRITE|READ): bw=\K[0-9.]+[KMG]?i?B/s' "$1" | head -1; }

{
  echo "# S20 benchmark report ($(date -Iseconds))"
  echo
  echo "## Bounded boot (Arena format v2, checkpoint vs full scan)"
  echo "- $B_SMALL"
  echo "- $B_LARGE"
  echo
  echo "## Write amplification workload (4 KiB random overwrite, 64 MiB logical)"
  echo "- throughput: $(bw "$DIR/fio-w4k.log")"
  echo "- arena du after matrix: $DU B"
  echo
  echo "## Save files"
  echo "- 8 KiB writes: $(bw "$DIR/fio-saves8k.log")"
  echo "- 64 KiB writes: $(bw "$DIR/fio-saves64k.log")"
  echo
  echo "## Asset load (1 GiB)"
  echo "- write: $(bw "$DIR/fio-assetw.log")"
  echo "- read pass 1: $(bw "$DIR/fio-assetr1.log")"
  echo "- read pass 2 (warm cache): $(bw "$DIR/fio-assetr2.log")"
} > "$REPORT"
cat "$REPORT"
echo "BENCH_MATRIX_OK"
