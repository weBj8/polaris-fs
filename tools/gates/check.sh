#!/usr/bin/env bash
# CI gate check — compares live tree metrics against tools/gates/baseline.tsv.
# Metrics:
#   unsafe_sites     unsafe fn/{/impl count per crate src (le)
#   unsafe_no_safety unsafe sites without a SAFETY comment within 2 lines above (le)
#   migration_iou    MIGRATION-IOU markers tree-wide (le; ledger in verified-claims.md)
#   wrapping_ops     wrapping_* count per crate (ge — removal = behavior change,
#                    requires baseline update with claim ID, divergence-audit M3)
# Exit 1 on any violation, printing a diff-style report. Run from repo root.
set -uo pipefail
cd "$(dirname "$0")/../.."

crates="mfsmaster mfschunkserver mfsmetalogger mfsmount mfsbdev mfsgui mfsnetdump"

count_unsafe()      { grep -rE 'unsafe (fn|\{|impl)' "$1/src" --include='*.rs' | wc -l; }
count_wrapping()    { grep -rn 'wrapping_' "$1/src" --include='*.rs' | wc -l; }
# unsafe site with no "SAFETY" in the 2 preceding lines (cheap heuristic —
# ponytail: line-window heuristic, upgrade to an AST check if it false-passes)
count_no_safety() {
  grep -rEB2 'unsafe (fn|\{|impl)' "$1/src" --include='*.rs' \
    | awk '/unsafe (fn|\{|impl)/ { if ($0 !~ /SAFETY/ && prev !~ /SAFETY/) n++ } { prev=$0 } END { print n+0 }'
}
count_iou() { grep -rn 'MIGRATION-IOU' --include='*.rs' . --exclude-dir=vendor --exclude-dir=target | wc -l; }

fail=0
printf '%-16s %-14s %-4s %8s %8s %s\n' metric crate dir current baseline status
while IFS=$'\t' read -r metric crate dir base; do
  [[ "$metric" =~ ^#.*$ || -z "$metric" ]] && continue
  case "$metric" in
    unsafe_sites)     cur=$(count_unsafe "$crate") ;;
    unsafe_no_safety) cur=$(count_no_safety "$crate") ;;
    wrapping_ops)     cur=$(count_wrapping "$crate") ;;
    migration_iou)    cur=$(count_iou) ;;
    *) echo "unknown metric $metric"; fail=1; continue ;;
  esac
  if [ "$dir" = le ]; then ok=$(( cur <= base )); else ok=$(( cur >= base )); fi
  status=ok; [ "$ok" = 1 ] || { status=VIOLATION; fail=1; }
  printf '%-16s %-14s %-4s %8s %8s %s\n' "$metric" "$crate" "$dir" "$cur" "$base" "$status"
done < tools/gates/baseline.tsv

exit $fail
