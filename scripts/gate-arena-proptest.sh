#!/usr/bin/env bash
# S2 deep gate: 256-case arena proptest (CI default is 8 cases for speed).
set -euo pipefail
cd "$(dirname "$0")/.."
PROPTEST_CASES="${PROPTEST_CASES:-256}" cargo test -p plfs-arena --test proptest_arena "$@"
