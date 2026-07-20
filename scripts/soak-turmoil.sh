#!/usr/bin/env bash
# S19 gate: deterministic fault-injection soak (design doc §10). Five legs
# rotate deterministically — kill -9 a data node, SIGSTOP/SIGCONT a data
# node (delay/partition analog; dm-flakey needs real root, out of reach
# here), slot corruption (bit rot → scrub repair), kill -9 the writer
# (WAL replay), kill -9 a registry member (quorum) — with a full acked-set
# verification observed after every leg. ITERS legs (default 10 ≈ 2 min);
# the 72 h simulated soak is this script with ITERS scaled up.
set -euo pipefail
cd "$(dirname "$0")/.."

ITERS="${ITERS:-10}"
DIR="$(pwd)/target/soak"
REG_PORTS=(19901 19902 19903)
DATA_PORTS=(19911 19912 19913 19914)
PEERS="1@127.0.0.1:${REG_PORTS[0]},2@127.0.0.1:${REG_PORTS[1]},3@127.0.0.1:${REG_PORTS[2]}"

cargo build -p plfs >/dev/null
cargo build -p plfs-client --example soak_driver >/dev/null
rm -rf "$DIR"
mkdir -p "$DIR"

declare -a pids
cleanup() { for p in "${pids[@]:-}"; do kill -9 "$p" 2>/dev/null || true; done; }
trap cleanup EXIT

DATA_PID=("" "" "" "" "")
REG_PID=("" "" "" "")
start_registry() {
    target/debug/plfs registry --dir "$DIR/r$1" --node-id "$1" \
        --listen "127.0.0.1:${REG_PORTS[$(( $1 - 1 ))]}" --peers "$PEERS" \
        > "$DIR/r$1.log" 2>&1 &
    REG_PID[$1]=$!
    pids+=($!)
}
start_data() {
    target/debug/plfs data --arena "$DIR/d$1/arena.img" \
        --listen "127.0.0.1:${DATA_PORTS[$(( $1 - 1 ))]}" \
        --registry "127.0.0.1:${REG_PORTS[0]}" --node-key "data-$1" \
        > "$DIR/d$1.log" 2>&1 &
    DATA_PID[$1]=$!
    pids+=($!)
}
start_work() {
    target/debug/examples/soak_driver "127.0.0.1:${REG_PORTS[0]}" "$DIR/client0" work \
        >> "$DIR/work.log" 2>&1 &
    WORK_PID=$!
    pids+=($!)
}

for i in 1 2 3; do mkdir -p "$DIR/r$i"; start_registry "$i"; done
sleep 3
for i in 1 2 3 4; do
    mkdir -p "$DIR/d$i"
    target/debug/plfs mkfs --dir "$DIR/d$i" --size $((2 << 30)) >/dev/null 2>&1
    start_data "$i"
done
sleep 4
start_work
sleep 3

wait_new_verify() {  # $1 = previous VERIFY_OK count
    for _ in $(seq 1 100); do
        n=$(grep -c VERIFY_OK "$DIR/work.log" 2>/dev/null || true)
        [ "${n:-0}" -gt "$1" ] && return 0
        sleep 0.2
    done
    echo "no VERIFY_OK after leg (was $1)"; tail -5 "$DIR/work.log"; return 1
}

corrupt_canary() {
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
        j = chunk.find(pat)
        if j >= 0:
            print(off + j); break
        off += len(chunk) - 4096
        f.seek(off)
PY
)
        if [ "$OFF" != "-1" ]; then
            printf '\xde\xad\xbe\xef' | dd of="$IMG" bs=1 seek="$OFF" count=4 conv=notrunc status=none
            echo "leg: bit rot into d$i at $OFF"
            return 0
        fi
    done
    echo "leg: canary not found (skipped)"; return 1
}

for leg in $(seq 1 "$ITERS"); do
    prev=$(grep -c VERIFY_OK "$DIR/work.log" 2>/dev/null || true)
    prev=${prev:-0}
    case $((leg % 5)) in
        0)
            v=$((1 + leg % 4)); echo "leg $leg: kill -9 data-$v"
            kill -9 "${DATA_PID[$v]}" 2>/dev/null || true; sleep 2
            start_data "$v"; sleep 2 ;;
        1)
            v=$((1 + leg % 4)); echo "leg $leg: SIGSTOP/SIGCONT data-$v"
            kill -STOP "${DATA_PID[$v]}"; sleep 3; kill -CONT "${DATA_PID[$v]}"; sleep 2 ;;
        2)
            echo "leg $leg: corrupt canary"
            corrupt_canary; sleep 3 ;;
        3)
            echo "leg $leg: kill -9 workload"
            kill -9 "$WORK_PID" 2>/dev/null || true; sleep 1
            start_work; sleep 2 ;;
        4)
            v=$((1 + leg % 3)); echo "leg $leg: kill -9 registry r$v"
            kill -9 "${REG_PID[$v]}" 2>/dev/null || true; sleep 2
            start_registry "$v"; sleep 2 ;;
    esac
    wait_new_verify "$prev"
done

kill -9 "$WORK_PID" 2>/dev/null || true
sleep 1
target/debug/examples/soak_driver "127.0.0.1:${REG_PORTS[0]}" "$DIR/client0" verify
echo "SOAK_OK legs=$ITERS"
