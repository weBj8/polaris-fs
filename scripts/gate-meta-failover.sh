#!/usr/bin/env bash
# S12 gate (design doc §7.3): 3-node metadata raft group, kill the leader
# mid-workload — a new leader must be elected < 1 s, writes must resume
# shortly after, and every acknowledged write must survive (zero metadata
# divergence).
set -euo pipefail
cd "$(dirname "$0")/.."

DIR="$(pwd)/target/gate-meta"
PORTS=(19311 19312 19313)
PEERS="1@127.0.0.1:${PORTS[0]},2@127.0.0.1:${PORTS[1]},3@127.0.0.1:${PORTS[2]}"
ENDPOINTS="127.0.0.1:${PORTS[0]},127.0.0.1:${PORTS[1]},127.0.0.1:${PORTS[2]}"

cargo build -p plfs >/dev/null
cargo build -p plfs-client --example meta_driver >/dev/null
DRIVER=target/debug/examples/meta_driver

rm -rf "$DIR"
mkdir -p "$DIR/n1" "$DIR/n2" "$DIR/n3"
pids=()
cleanup() {
    for p in "${pids[@]:-}"; do kill -9 "$p" 2>/dev/null || true; done
}
trap cleanup EXIT

for i in 1 2 3; do
    target/debug/plfs meta --dir "$DIR/n$i" --node-id "$i" \
        --listen "127.0.0.1:${PORTS[$((i-1))]}" --peers "$PEERS" \
        > "$DIR/n$i.log" 2>&1 &
    pids+=($!)
done

# Wait for a leader.
leader_id=""
leader_of() {
    # Chronologically latest "become leader" line across all logs: sort by
    # the ISO timestamp prefix (glob order is not time order).
    grep -h "become leader" "$DIR"/n*.log 2>/dev/null | sort | tail -1 \
        | sed -n -e 's/\x1b\[[0-9;]*m//g' -e 's/.*id=\([0-9][0-9]*\).*/\1/p' || true
}
for _ in $(seq 1 50); do
    leader_id=$(leader_of)
    [ -n "$leader_id" ] && break
    sleep 0.2
done
if [ -z "$leader_id" ]; then
    echo "META_FAILOVER_FAIL: no leader elected" >&2
    exit 1
fi
echo "leader is node $leader_id"

# Baseline workload.
"$DRIVER" workload "$ENDPOINTS" 10 > /dev/null

# Start the flood (continuous acked writes).
"$DRIVER" flood "$ENDPOINTS" 0 "$DIR/ack.log" > "$DIR/flood.log" 2>&1 &
flood_pid=$!
pids+=($flood_pid)
sleep 2
before=$(wc -l < "$DIR/ack.log" 2>/dev/null || echo 0)
echo "flood running, $before ops acked before kill"

# Kill the leader mid-workload.
kill_t=$(date +%s%N)
kill -9 "${pids[$((leader_id - 1))]}"
echo "killed node $leader_id"

# Wait for a new leader among survivors and measure election time.
new_leader=""
for _ in $(seq 1 200); do
    new_leader=$(leader_of)
    if [ -n "$new_leader" ] && [ "$new_leader" != "$leader_id" ]; then
        break
    fi
    sleep 0.02
done
elect_t=$(date +%s%N)

# Measure when acked writes resume.
for _ in $(seq 1 200); do
    now=$(wc -l < "$DIR/ack.log" 2>/dev/null || echo 0)
    [ "$now" -gt "$before" ] && break
    sleep 0.05
done
resume_t=$(date +%s%N)

kill -9 "$flood_pid" 2>/dev/null || true
elect_ms=$(( (elect_t - kill_t) / 1000000 ))
resume_ms=$(( (resume_t - kill_t) / 1000000 ))
echo "election=${elect_ms}ms resume=${resume_ms}ms"

if [ -z "$new_leader" ] || [ "$new_leader" = "$leader_id" ]; then
    echo "META_FAILOVER_FAIL: no new leader after kill" >&2
    exit 1
fi
if ((elect_ms > 1000)); then
    echo "META_FAILOVER_FAIL: election ${elect_ms}ms > 1s budget" >&2
    exit 1
fi
if ((resume_ms > 3000)); then
    echo "META_FAILOVER_FAIL: writes did not resume within 3s" >&2
    exit 1
fi

"$DRIVER" flood_verify "$ENDPOINTS" "$DIR/ack.log"
echo "META_FAILOVER_OK leader=$leader_id->$new_leader election=${elect_ms}ms resume=${resume_ms}ms"
