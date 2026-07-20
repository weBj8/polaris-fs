#!/usr/bin/env bash
# S8 gate: kill -9 the client core mid-workload; WAL replay + raft recovery
# must leave the volume byte-exact against the model (one-op gap tolerated
# at the kill point).
set -euo pipefail
cd "$(dirname "$0")/.."

ITERS="${ITERS:-100}"
DIR=target/kill9-client

cargo build --example client_fuzz >/dev/null
BIN=target/debug/examples/client_fuzz

mkdir -p "$DIR"
for i in $(seq 1 "$ITERS"); do
    iter_dir="$DIR/iter_$i"
    rm -rf "$iter_dir"
    mkdir -p "$iter_dir"
    acklog="$iter_dir/ack.log"
    seed=$((1000 + i))

    "$BIN" run "$iter_dir/vol" "$acklog" "$seed" &
    pid=$!
    sleep "0.$((RANDOM % 18 + 2))"
    kill -9 "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true

    if ! "$BIN" verify "$iter_dir/vol" "$acklog" "$seed"; then
        echo "kill9-client: FAILED at iteration $i (artifacts kept in $iter_dir)" >&2
        exit 1
    fi
    rm -rf "$iter_dir"
    if ((i % 10 == 0)); then
        echo "kill9-client: $i/$ITERS ok"
    fi
done
echo "KILL9_CLIENT_OK iters=$ITERS"
