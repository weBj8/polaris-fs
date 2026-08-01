#!/usr/bin/env bash
# Extract byte-identical plfscommon modules into the shared plfscommon crate.
# Mechanical dedup per docs/facts/dedup-map.md / verified-claims VC-05:
# the 14 modules below are md5-identical in every crate that has them.
# c2rust files are linker-wired (no cross-module `use`), so consumers keep
# their own extern decls and resolve symbols from the shared rlib.
set -euo pipefail
cd "$(dirname "$0")/.."

MODS="charts clocks conncache cpuusage crc delayrun labelparser lwthread md5 memusage mfslog processname sockets timeparser"

# 1. shared crate skeleton
mkdir -p plfscommon/src
cat > plfscommon/Cargo.toml <<'EOF'
[package]
name = "plfscommon"
version.workspace = true
publish = false
edition = "2024"

[lib]
name = "plfscommon"
path = "src/lib.rs"
crate-type = ["rlib"]

[dependencies]
c2rust-bitfields = { path = "../vendor/c2rust-bitfields-0.22.1" }
libc = "0.2"
EOF

{
cat <<'EOF'
// Shared c2rust-transpiled MooseFS plfscommon modules (deduplicated P0).
// These files were byte-identical across all daemon crates (dedup-map.md).
#![feature(c_variadic)]
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

EOF
for m in $MODS; do echo "pub mod $m;"; done
} > plfscommon/src/lib.rs

# 2. populate shared sources from the first crate that has each file
for m in $MODS; do
  src=$(ls plfsmaster/src/plfscommon/$m.rs plfschunkserver/src/plfscommon/$m.rs plfsmount/src/plfscommon/$m.rs plfsbdev/src/plfscommon/$m.rs plfsmetalogger/src/plfscommon/$m.rs plfsgui/src/plfscommon/$m.rs 2>/dev/null | head -1 || true)
  [ -n "$src" ] || { echo "no source for $m"; exit 1; }
  cp "$src" plfscommon/src/$m.rs
done

# 3. strip extracted modules from consumer crates + add path dependency
for c in plfsmaster plfschunkserver plfsmetalogger plfsmount plfsbdev plfsgui; do
  for m in $MODS; do
    rm -f "$c/src/plfscommon/$m.rs"
  done
  for m in $MODS; do
    sed -i "/^        pub mod $m;$/d" "$c/lib.rs"
  done
  grep -q 'plfscommon = { path' "$c/Cargo.toml" || \
    sed -i 's|^libc = "0.2"|libc = "0.2"\nplfscommon = { path = "../plfscommon" }|' "$c/Cargo.toml"
done
echo "dedup done"
