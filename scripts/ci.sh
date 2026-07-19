#!/usr/bin/env bash
set -euo pipefail

echo "== cargo fmt --all -- --check =="
cargo fmt --all -- --check

echo "== cargo clippy --workspace --all-targets -- -D warnings =="
cargo clippy --workspace --all-targets -- -D warnings

echo "== cargo test --workspace =="
cargo test --workspace
