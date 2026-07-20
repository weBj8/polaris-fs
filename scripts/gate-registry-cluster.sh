#!/usr/bin/env bash
# S13 gate (design doc §4.3): 3-node registry group, 3 data nodes
# registered + heartbeating via the registry, 3 client volumes discovered
# through it serving a workload; registry survives a member loss.
set -euo pipefail
cd "$(dirname "$0")/.."

DIR="$(pwd)/target/gate-registry"
REG_PORTS=(19401 19402 19403)
DATA_PORTS=(19411 19412 19413)
REG_PEERS="1@127.0.0.1:${REG_PORTS[0]},2@127.0.0.1:${REG_PORTS[1]},3@127.0.0.1:${REG_PORTS[2]}"

cargo build -p plfs >/dev/null
cargo build -p plfs-client --example cluster_driver >/dev/null
rm -rf "$DIR"
mkdir -p "$DIR"

declare -a pids
cleanup() {
    for p in "${pids[@]:-}"; do kill -9 "$p" 2>/dev/null || true; done
}
trap cleanup EXIT

# 3-node registry group.
for i in 1 2 3; do
    mkdir -p "$DIR/r$i"
    target/debug/plfs registry --dir "$DIR/r$i" --node-id "$i" \
        --listen "127.0.0.1:${REG_PORTS[$((i-1))]}" --peers "$REG_PEERS" \
        > "$DIR/r$i.log" 2>&1 &
    pids+=($!)
done
sleep 3

# 3 data nodes (register + heartbeat into the registry).
for i in 1 2 3; do
    mkdir -p "$DIR/d$i"
    target/debug/plfs mkfs --dir "$DIR/d$i" --size $((4 << 30)) >/dev/null 2>&1
    target/debug/plfs data --arena "$DIR/d$i/arena.img" \
        --listen "127.0.0.1:${DATA_PORTS[$((i-1))]}" \
        --registry "127.0.0.1:${REG_PORTS[0]}" --node-key "data-$i" \
        > "$DIR/d$i.log" 2>&1 &
    pids+=($!)
done

# Wait for all three data nodes to be registered and live.
sleep 4

# The workload: registry discovery + 3 client volumes on 3 data nodes.
target/debug/examples/cluster_driver "127.0.0.1:${REG_PORTS[0]}" "$DIR/clients"

# Registry survives a member loss (quorum 2/3 keeps serving).
kill -9 "${pids[0]}"
sleep 2
target/debug/examples/cluster_driver "127.0.0.1:${REG_PORTS[1]}" "$DIR/clients2"

echo "REGISTRY_CLUSTER_OK data_nodes=3 clients=3 survivors=quorum"
