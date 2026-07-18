#!/usr/bin/env bash
# P8 gate reproduction: the striped parallel read bench (median of 3,
# release build). Prints the single-server baseline, the pool's measured
# concurrent capacity, the 4-client striped aggregate, and the ratio
# (gate: aggregate >= 70% of pool capacity).
set -u
cd "$(dirname "$0")/.."
exec cargo test --release -p porfs-cluster --test stripe gate_four_client_aggregate_read -- --nocapture
