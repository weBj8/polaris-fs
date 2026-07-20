#!/usr/bin/env bash
# S4 gate (docs/design.md §6.4): write >= 10 GiB into a sparse arena, delete
# all chunks, real du must return to baseline; plus the bitmap-only delete +
# sparsify maintenance path.
set -euo pipefail
cd "$(dirname "$0")/.."

DIR=target/space-reclaim
ARENA="$DIR/arena0"
GIB_FILL="${GIB_FILL:-10}"
EPSILON=$((64 << 20))  # superblock + bitmap copies + fs slack

cargo build --example space_reclaim >/dev/null
BIN=target/debug/examples/space_reclaim

mkdir -p "$DIR"
cleanup() { rm -f "$ARENA"; }
trap cleanup EXIT
rm -f "$ARENA"

"$BIN" mkfs "$ARENA" 14 >/dev/null
baseline=$("$BIN" blocks "$ARENA")

"$BIN" fill "$ARENA" "$GIB_FILL" >/dev/null
written=$("$BIN" blocks "$ARENA")
grown=$((written - baseline))
need=$(((GIB_FILL << 30) * 95 / 100))
if ((grown < need)); then
    echo "SPACE_RECLAIM_FAIL: du grew by $grown, expected >= $need" >&2
    exit 1
fi

"$BIN" delete-all "$ARENA" >/dev/null
after_delete=$("$BIN" blocks "$ARENA")
if ((after_delete > baseline + EPSILON)); then
    echo "SPACE_RECLAIM_FAIL: du after delete $after_delete > baseline+eps $((baseline + EPSILON))" >&2
    exit 1
fi

# Bitmap-only delete retains blocks; the same-session sparsify pass must
# reclaim them (cross-session the stale headers would resurrect the chunks —
# headers are truth, contract §5).
"$BIN" fill "$ARENA" 2 >/dev/null
before_reclaim=$("$BIN" reclaim-all "$ARENA" | sed -n 's/.*retained=\([0-9]*\).*/\1/p')
if ((before_reclaim < baseline + (1 << 30))); then
    echo "SPACE_RECLAIM_FAIL: bitmap-only delete did not retain blocks ($before_reclaim)" >&2
    exit 1
fi
after_sparsify=$("$BIN" blocks "$ARENA")
if ((after_sparsify > baseline + EPSILON)); then
    echo "SPACE_RECLAIM_FAIL: sparsify left $after_sparsify > baseline+eps $((baseline + EPSILON))" >&2
    exit 1
fi

echo "SPACE_RECLAIM_OK du_baseline=$baseline du_written=$written du_after_delete=$after_delete du_bitmap_only=$before_reclaim du_after_sparsify=$after_sparsify"
