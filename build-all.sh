#!/usr/bin/env bash
# One-click build of all c2rust-transpiled MooseFS binaries.
# Output: dist/ at the repo root.
set -euo pipefail

cd "$(dirname "$0")"
TOOLCHAIN=nightly

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
    cp "$crate/target/release/$bin" "dist/$crate"
done

echo ">> done:"
ls -la dist/
