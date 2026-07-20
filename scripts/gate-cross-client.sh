#!/usr/bin/env bash
# S18 gate (design doc §4.3, §8.2): client A owns a volume (writer epoch
# claimed, MetaOps served, registered as the volume's CLIENT); foreign
# client B discovers A through the registry and sees each committed file
# within one second.
set -euo pipefail
cd "$(dirname "$0")/.."

DIR="$(pwd)/target/gate-cross-client"
cargo build -p plfs >/dev/null
cargo build -p plfs-client --example sync_driver >/dev/null
rm -rf "$DIR"
mkdir -p "$DIR"

declare -a pids
cleanup() { for p in "${pids[@]:-}"; do kill -9 "$p" 2>/dev/null || true; done; }
trap cleanup EXIT

mkdir -p "$DIR/r1"
target/debug/plfs registry --dir "$DIR/r1" --node-id 1 \
    --listen 127.0.0.1:19801 --peers "1@127.0.0.1:19801" \
    > "$DIR/r1.log" 2>&1 &
pids+=($!)
sleep 3

for i in 1 2; do
    mkdir -p "$DIR/d$i"
    target/debug/plfs mkfs --dir "$DIR/d$i" --size $((2 << 30)) >/dev/null 2>&1
    target/debug/plfs data --arena "$DIR/d$i/arena.img" \
        --listen "127.0.0.1:1981$i" \
        --registry 127.0.0.1:19801 --node-key "data-$i" \
        > "$DIR/d$i.log" 2>&1 &
    pids+=($!)
done
sleep 4

target/debug/examples/sync_driver 127.0.0.1:19801 write "$DIR/volA" vol-a 127.0.0.1:19891 \
    > "$DIR/a.log" 2>&1 &
pids+=($!)
for i in $(seq 1 50); do grep -q FILE1_READY "$DIR/a.log" 2>/dev/null && break; sleep 0.1; done
grep -q FILE1_READY "$DIR/a.log"

target/debug/examples/sync_driver 127.0.0.1:19801 read vol-a > "$DIR/b.log" 2>&1
cat "$DIR/a.log" "$DIR/b.log"

T1=$(sed -n 's/FILE2_FSYNCED //p' "$DIR/a.log")
T2=$(sed -n 's/FILE2_SEEN //p' "$DIR/b.log")
echo "file-2 visible after $((T2 - T1)) ms"
test $((T2 - T1)) -lt 1000
echo "CROSS_CLIENT_GATE_OK"
