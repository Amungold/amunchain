#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."

echo "=== ARCHITECTURAL ENTROPY SCANNER ==="

# Scan for duplicate semantic domains
echo ""
echo "--- Potential semantic overlaps ---"

# Check for crates with similar names
find crates -maxdepth 1 -name "amun-*" -type d | sed 's|crates/||' | sort | while read crate; do
    # Strip semantic suffixes to find base domain
    base=$(echo "$crate" | sed 's/-law$//; s/-rule$//; s/-model$//; s/-simulator$//; s/-constitution$//; s/-tests$//; s/-engine$//; s/-certificate$//')
    echo "$base"
done | sort | uniq -d | while read dup; do
    echo "  ⚠ Duplicate domain: $dup"
    find crates -maxdepth 1 -name "amun-${dup}*" -type d | sed 's|crates/|    |'
done

# Count crates per category
echo ""
echo "--- Crate counts ---"
total=$(find crates tests -maxdepth 1 -name "amun-*" -type d | wc -l)
echo "  Total crates + test harnesses: $total"

echo ""
echo "--- Sovereignty breakdown ---"
sovereign=$(grep -c "sovereign = true" constitution/registry.toml 2>/dev/null || echo "0")
echo "  Sovereign crates: $sovereign"
echo "  Supporting crates: $((total - sovereign))"

echo ""
echo "--- Criticality distribution ---"
for level in kernel consensus deterministic persistence interface; do
    count=$(grep -c "criticality = \"$level\"" constitution/registry.toml 2>/dev/null || echo "0")
    echo "  $level: $count"
done

echo ""
echo "--- Freeze boundaries ---"
grep "freeze_boundary = true" constitution/registry.toml 2>/dev/null | while read line; do
    echo "  ✓ Frozen"
done
