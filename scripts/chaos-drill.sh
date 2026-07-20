#!/usr/bin/env bash
# S20 chaos drill (design doc §10.2): every row of the failure-handling
# table, executed against the real gates.
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target/drill

declare -a results
run() {
    local row="$1"; shift
    if "$@" > "target/drill/$(echo "$row" | tr -cd 'a-z0-9' | head -c 24).log" 2>&1; then
        results+=("PASS  $row")
    else
        results+=("FAIL  $row")
    fi
}

run "data node down → read failover + re-replication" ./scripts/gate-replication.sh
run "meta leader down → election, zero divergence" ./scripts/gate-meta-failover.sh
run "bit rot → crc scrub repairs from replica" ./scripts/gate-scrub.sh
run "client machine dead → WAL/quorum durability" ./scripts/gate-standalone.sh
run "slow node, crashes, partitions (soak legs)" env ITERS=5 ./scripts/soak-turmoil.sh
run "GC vs re-Put race → version-checked Delete" cargo test -p plfs-data --test proptest_grpc
run "snapshot/rollback churn" ./scripts/gate-rollback.sh

printf '%s\n' "${results[@]}"
if printf '%s\n' "${results[@]}" | grep -q FAIL; then
    echo "CHAOS_DRILL_FAILED"
    exit 1
fi
echo "CHAOS_DRILL_OK"
