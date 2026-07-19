#!/usr/bin/env bash
# ChunkArena kill -9 fuzz (contract §5/§7): spawn a writer on a fresh arena,
# SIGKILL it at a random time, then verify the reboot invariants:
# every acked-live chunk byte-exact, no false-allocated slots, delete-acked
# resurrects byte-exact. Fails fast on the first violation.
set -euo pipefail

ITERS="${ITERS:-100}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK="$ROOT/target/kill9-arena"
BIN="$ROOT/target/debug/examples/arena_fuzz"

mkdir -p "$WORK"
cargo build -q -p plfs-arena --example arena_fuzz --manifest-path "$ROOT/Cargo.toml"

for i in $(seq 1 "$ITERS"); do
    arena="$WORK/arena-$i.dev"
    ack="$WORK/arena-$i.acklog"
    rm -f "$arena" "$ack"

    "$BIN" write "$arena" "$ack" "$i" &
    pid=$!
    ms=$((RANDOM % 196 + 5))
    sleep "$(printf '0.%03d' "$ms")"
    kill -9 "$pid" 2>/dev/null || true
    status=0
    wait "$pid" 2>/dev/null || status=$?
    # 137 = 128 + SIGKILL (expected); anything else is a writer-side
    # contract violation (the writer panics on model/arena divergence).
    if [[ "$status" -ne 137 && "$status" -ne 0 ]]; then
        echo "KILL9_ARENA_FAIL iter=$i: writer exited with status $status (contract violation?) — artifacts kept: $arena $ack" >&2
        exit 1
    fi

    if ! "$BIN" verify "$arena" "$ack" "$i"; then
        echo "KILL9_ARENA_FAIL iter=$i — artifacts kept: $arena $ack" >&2
        exit 1
    fi
    rm -f "$arena" "$ack"

    if (( i % 10 == 0 )); then
        echo "kill9-arena: $i/$ITERS ok"
    fi
done

echo "KILL9_ARENA_OK iters=$ITERS"
