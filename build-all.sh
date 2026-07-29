#!/usr/bin/env bash
# One-click build of all c2rust-transpiled MooseFS binaries.
# Output: dist/ at the repo root.
set -euo pipefail

cd "$(dirname "$0")"
TOOLCHAIN=nightly

# one shared target dir: deps (libc, c2rust-bitfields) compile once for all 7 crates
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target/build-all}"

if ! rustup toolchain list | grep -q "^$TOOLCHAIN"; then
    echo ">> installing $TOOLCHAIN"
    rustup toolchain install "$TOOLCHAIN" --profile minimal
fi

# crate dir : binary name produced in target/release/
CRATES=(
    mfsmaster:main
    mfschunkserver:main
    mfsmetalogger:main
    mfsgui:main
    mfsmount:mfsmount
    mfsbdev:mfsbdev
    mfsnetdump:mfsnetdump
)

mkdir -p dist
for entry in "${CRATES[@]}"; do
    crate="${entry%%:*}"
    bin="${entry##*:}"
    echo ">> building $crate"
    (cd "$crate" && cargo +"$TOOLCHAIN" build --release --quiet)
    cp "$CARGO_TARGET_DIR/release/$bin" "dist/$crate"
done

echo ">> done:"
ls -la dist/
