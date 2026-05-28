#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."

echo "=== CONSTITUTIONAL PROVENANCE CHAIN ==="

if [ ! -d "constitution/history" ]; then
    echo "No history directory found."
    exit 0
fi

echo ""
echo "--- Snapshot History ---"
for snap in $(ls -1t constitution/history/*.json 2>/dev/null); do
    timestamp=$(grep -o '"timestamp": "[^"]*"' "$snap" 2>/dev/null | head -1 | sed 's/"timestamp": "//; s/"//')
    commit=$(grep -o '"commit": "[^"]*"' "$snap" 2>/dev/null | head -1 | sed 's/"commit": "//; s/"//')
    count=$(grep -o '"crate_count": [0-9]*' "$snap" 2>/dev/null | grep -o '[0-9]*')
    sovereign=$(grep -o '"sovereign_count": [0-9]*' "$snap" 2>/dev/null | grep -o '[0-9]*')
    echo "  $timestamp | commit: ${commit:0:8} | crates: $count | sovereign: $sovereign"
done

echo ""
echo "--- Governance Integrity ---"
snap_count=$(ls -1 constitution/history/*.json 2>/dev/null | wc -l)
if [ "$snap_count" -gt 0 ]; then
    echo "✅ Provenance chain exists ($snap_count snapshots)"
else
    echo "⚠ No provenance data — run Phase 43 snapshot capture"
fi
