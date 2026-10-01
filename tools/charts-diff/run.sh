#!/usr/bin/env bash
# Differential test: plfscommon::charts vs MooseFS 4.59.2 mfscommon/charts.c.
# The same C drivers (harness.c fixed scenario, fuzz.c seeded fuzz) are
# linked once against the compiled C original and once against the Rust
# port; every observable output is compared record by record.
# usage: tools/charts-diff/run.sh <moosefs-src> [fuzz-seeds]   (needs gcc, zlib)
# Fuzz PNGs may differ only past the image: C deflates an uninitialized
# malloc'd rawchart tail after re-init (classify.py checks decoded pixels).
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
R="$(cd "$1" && pwd)/mfscommon"
seeds="${2:-30}"
w="$here/../../target/charts-diff"
mkdir -p "$w/out"
(cd "$here/rs" && CARGO_TARGET_DIR="$w/rs-target" cargo build --release -q)
lib="$w/rs-target/release/libchartsdiff.a"
for d in harness fuzz; do
  gcc -O2 -D_REENTRANT -I"$R" -o "$w/c_$d" "$here/$d.c" "$R/charts.c" "$R/crc.c" "$here/stubs.c" -lz -lpthread
  gcc -O2 -I"$R" -o "$w/rs_$d" "$here/$d.c" "$lib" -lz -lpthread -ldl -lm
done
cd "$here"
fail=0
for tz in UTC0 Europe/Warsaw Asia/Kolkata America/St_Johns Pacific/Chatham; do
  "$w/c_harness" "$w/out" c "$tz"; "$w/rs_harness" "$w/out" rs "$tz"
  res=$(python3 cmp.py "$w/out/c.blob" "$w/out/rs.blob" | tail -1)
  echo "harness $tz: $res"; [ "$res" = "mismatched records 0" ] || fail=1
done
for s in $(seq 1 "$seeds"); do
  "$w/c_fuzz" "$w/out" c "$s" 2>"$w/out/c.err"; "$w/rs_fuzz" "$w/out" rs "$s" 2>"$w/out/rs.err"
  res=$(python3 classify.py "$w/out/c.fblob" "$w/out/rs.fblob" | tail -1)
  if [ "${res##* }" != 0 ] || ! cmp -s "$w/out/c.err" "$w/out/rs.err"; then echo "seed $s: $res"; fail=1; fi
done
echo "fuzz seeds 1..$seeds done"
[ $fail = 0 ] && echo "CHARTS DIFF OK"
exit $fail
