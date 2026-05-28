#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."

echo "=== CONSTITUTIONAL GOVERNANCE DASHBOARD ==="
echo ""

# Current state
echo "--- Current Constitutional State ---"
echo "Timestamp: $(date -u)"
echo "Commit: $(git rev-parse HEAD 2>/dev/null || echo 'unknown')"
echo ""

# Crate statistics
echo "--- Crate Statistics ---"
total=$(find crates tests -maxdepth 1 -name "amun-*" -type d | wc -l)
sovereign=$(grep -c "sovereign = true" constitution/registry.toml 2>/dev/null || echo 0)
echo "  Total crates + test harnesses: $total"
echo "  Sovereign crates: $sovereign"
echo "  Supporting crates: $((total - sovereign))"
echo ""

# Layer distribution
echo "--- Layer Distribution ---"
for layer in 0 1 2 3 4 5 6 7; do
    count=$(grep -c "layer = $layer" constitution/registry.toml 2>/dev/null || echo 0)
    case $layer in
        0) name="Kernel";;
        1) name="Truth & Evidence";;
        2) name="Execution";;
        3) name="Consensus";;
        4) name="Persistence";;
        5) name="Network";;
        6) name="Governance & Economics";;
        7) name="Interfaces";;
    esac
    echo "  Layer $layer ($name): $count crates"
done
echo ""

# Criticality distribution
echo "--- Criticality Distribution ---"
echo "  Safety-critical: $(grep -c 'criticality_safety = true' constitution/registry.toml 2>/dev/null || echo 0)"
echo "  Determinism-critical: $(grep -c 'criticality_determinism = true' constitution/registry.toml 2>/dev/null || echo 0)"
echo "  Persistence-critical: $(grep -c 'criticality_persistence = true' constitution/registry.toml 2>/dev/null || echo 0)"
echo "  External-facing: $(grep -c 'criticality_external = true' constitution/registry.toml 2>/dev/null || echo 0)"
echo ""

# Freeze boundaries
echo "--- Freeze Boundaries ---"
grep -B1 "freeze_boundary = true" constitution/registry.toml 2>/dev/null | grep "\[crate\." | sed 's/\[crate\./  /; s/\]//' || echo "  (none documented)"
echo ""

# Historical snapshots
echo "--- Historical Snapshots ---"
if [ -d "constitution/history" ]; then
    ls -1 constitution/history/*.json 2>/dev/null | wc -l | xargs echo "  Total snapshots:"
    echo "  Latest: $(ls -1t constitution/history/*.json 2>/dev/null | head -1 | xargs basename)"
else
    echo "  No history captured yet"
fi
echo ""

# External dependencies
echo "--- External Dependencies ---"
if grep -q "\[external_policy\]" constitution/registry.toml 2>/dev/null; then
    echo "  Allowed: $(grep -A20 '\[external_policy\]' constitution/registry.toml | grep 'allowed =' | sed 's/.*\[//; s/\].*//')"
else
    echo "  No external policy defined"
fi
echo ""

# Entropy trend (simple)
echo "--- Entropy Trend ---"
if [ -d "constitution/history" ]; then
    snapshots=$(ls -1 constitution/history/*.json 2>/dev/null | wc -l)
    if [ "$snapshots" -gt 1 ]; then
        first=$(ls -1 constitution/history/*.json 2>/dev/null | head -1)
        last=$(ls -1t constitution/history/*.json 2>/dev/null | head -1)
        first_count=$(grep -o '"crate_count": [0-9]*' "$first" 2>/dev/null | grep -o '[0-9]*')
        last_count=$(grep -o '"crate_count": [0-9]*' "$last" 2>/dev/null | grep -o '[0-9]*')
        if [ -n "$first_count" ] && [ -n "$last_count" ]; then
            delta=$((last_count - first_count))
            echo "  Initial crate count: $first_count"
            echo "  Current crate count: $last_count"
            echo "  Delta: $delta"
            if [ "$delta" -gt 3 ]; then
                echo "  ⚠ Significant growth — review needed"
            else
                echo "  ✅ Growth within acceptable range"
            fi
        fi
    else
        echo "  Not enough history for trend analysis"
    fi
else
    echo "  No history available"
fi
