#!/usr/bin/env bash
# S14 gate (design doc §7.2, §7.4): RF=3 placement over 4 data nodes, kill
# one — reads keep serving (failover) and the background repair worker
# re-replicates the chunks that were on the dead node onto a fresh one
# (self-heal visible in the repair counter).
set -euo pipefail
cd "$(dirname "$0")/.."

DIR="$(pwd)/target/gate-replication"
REG_PORTS=(19501 19502 19503)
DATA_PORTS=(19511 19512 19513 19514)
REG_PEERS="1@127.0.0.1:${REG_PORTS[0]},2@127.0.0.1:${REG_PORTS[1]},3@127.0.0.1:${REG_PORTS[2]}"

cargo build -p plfs >/dev/null
cargo build -p plfs-client --example repair_driver >/dev/null
rm -rf "$DIR"
mkdir -p "$DIR"

declare -a pids
cleanup() {
    for p in "${pids[@]:-}"; do kill -9 "$p" 2>/dev/null || true; done
}
trap cleanup EXIT

for i in 1 2 3; do
    mkdir -p "$DIR/r$i"
    target/debug/plfs registry --dir "$DIR/r$i" --node-id "$i" \
        --listen "127.0.0.1:${REG_PORTS[$((i-1))]}" --peers "$REG_PEERS" \
        > "$DIR/r$i.log" 2>&1 &
    pids+=($!)
done
sleep 3

declare -a data_pids
for i in 1 2 3 4; do
    mkdir -p "$DIR/d$i"
    target/debug/plfs mkfs --dir "$DIR/d$i" --size $((2 << 30)) >/dev/null 2>&1
    target/debug/plfs data --arena "$DIR/d$i/arena.img" \
        --listen "127.0.0.1:${DATA_PORTS[$((i-1))]}" \
        --registry "127.0.0.1:${REG_PORTS[0]}" --node-key "data-$i" \
        > "$DIR/d$i.log" 2>&1 &
    data_pids+=($!)
    pids+=("${data_pids[-1]}")
done
sleep 4

# RF=3 client writes; it waits internally for the repair after we kill one.
target/debug/examples/repair_driver "127.0.0.1:${REG_PORTS[0]}" "$DIR/client0" 1 &
driver_pid=$!

# Let the write land, then kill one data node (the first).
sleep 6
kill -9 "${data_pids[0]}"
echo "killed data-1 (127.0.0.1:${DATA_PORTS[0]})"

wait "$driver_pid"
echo "REPLICATION_GATE_OK"
