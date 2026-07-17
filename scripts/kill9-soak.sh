#!/usr/bin/env bash
# P2 durability gate: 1000 kill -9 iterations against the crash harness
# (release mode; debug io_uring is too slow for the soak). Exits nonzero on
# the first failure via `set -e`. Progress is logged every 100 iterations by
# the harness itself.
set -euo pipefail
cd "$(dirname "$0")/.."

ITERS="${PORFS_CRASH_ITERS:-1000}"
export PORFS_CRASH_ITERS="$ITERS"

echo "[kill9-soak] $(date +%T) building release crash harness"
cargo test --release -p porfs-store --test crash --no-run

echo "[kill9-soak] $(date +%T) running $ITERS kill -9 iterations"
cargo test --release -p porfs-store --test crash -- --exact crash_kill9_loop --nocapture

echo "[kill9-soak] $(date +%T) OK: $ITERS iterations, zero corruption"
