#!/usr/bin/env bash
# S17 gate (design doc §10.3): flip payload bytes in one replica's slot →
# the scrub detects the crc mismatch and repairs from a healthy replica
# (reads stay byte-exact); a chunk written straight to a data node with no
# metadata reference is swept as an orphan.
set -euo pipefail
cd "$(dirname "$0")/.."

DIR="$(pwd)/target/gate-scrub"
REG_PORTS=(19701 19702 19703)
DATA_PORTS=(19711 19712 19713 19714)
PEERS="1@127.0.0.1:${REG_PORTS[0]},2@127.0.0.1:${REG_PORTS[1]},3@127.0.0.1:${REG_PORTS[2]}"

cargo build -p plfs >/dev/null
cargo build -p plfs-client --example scrub_driver >/dev/null
rm -rf "$DIR"
mkdir -p "$DIR"

declare -a pids
cleanup() { for p in "${pids[@]:-}"; do kill -9 "$p" 2>/dev/null || true; done; }
trap cleanup EXIT

for i in 1 2 3; do
    mkdir -p "$DIR/r$i"
    target/debug/plfs registry --dir "$DIR/r$i" --node-id "$i" \
        --listen "127.0.0.1:${REG_PORTS[$((i-1))]}" --peers "$PEERS" \
        > "$DIR/r$i.log" 2>&1 &
    pids+=($!)
done
sleep 3

for i in 1 2 3 4; do
    mkdir -p "$DIR/d$i"
    target/debug/plfs mkfs --dir "$DIR/d$i" --size $((2 << 30)) >/dev/null 2>&1
    target/debug/plfs data --arena "$DIR/d$i/arena.img" \
        --listen "127.0.0.1:${DATA_PORTS[$((i-1))]}" \
        --registry "127.0.0.1:${REG_PORTS[0]}" --node-key "data-$i" \
        > "$DIR/d$i.log" 2>&1 &
    pids+=($!)
done
sleep 4

target/debug/examples/scrub_driver "127.0.0.1:${REG_PORTS[0]}" "$DIR/client0" write

# Bit rot: flip 4 payload bytes inside the 0xAB chunk on the first node
# that holds it.
for i in 1 2 3 4; do
    IMG="$DIR/d$i/arena.img"
    OFF=$(python3 - "$IMG" <<'PY'
import sys
pat = b"\xab" * 4096
with open(sys.argv[1], "rb") as f:
    off = 0
    while True:
        chunk = f.read(1 << 26)
        if not chunk:
            print(-1); break
        i = chunk.find(pat)
        if i >= 0:
            print(off + i); break
        off += len(chunk) - 4096
        f.seek(off)
PY
)
    if [ "$OFF" != "-1" ]; then
        printf '\xde\xad\xbe\xef' | dd of="$IMG" bs=1 seek="$OFF" count=4 conv=notrunc status=none
        echo "bit rot injected into d$i at $OFF"
        break
    fi
done

target/debug/examples/scrub_driver "127.0.0.1:${REG_PORTS[0]}" "$DIR/client0" scrub "127.0.0.1:${DATA_PORTS[0]}"
echo "SCRUB_GATE_OK"
