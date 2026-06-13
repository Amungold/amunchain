#!/bin/bash
# AmunChain Environment Cleaner — kill all processes and free test ports
set -euo pipefail

echo "=== Cleaning AmunChain test environment ==="

pkill -9 -f "target/release/validator" 2>/dev/null || true
pkill -9 -f "target/release/evidence_service" 2>/dev/null || true
pkill -9 -f "cargo run.*validator" 2>/dev/null || true
pkill -9 -f "cargo run.*evidence_service" 2>/dev/null || true

for port in 9900 9901 9902 9903 29900 29901 29902 29903; do
    fuser -k ${port}/tcp 2>/dev/null || true
done

rm -rf /tmp/amun-test-validator-*
rm -rf /tmp/amun-vps-*
rm -f /tmp/vps_v*.log /tmp/vps_evidence_*.log /tmp/evidence_*.log

sleep 2

echo "=== Verifying clean state ==="
ACTIVE=$(ss -tlnp 2>/dev/null | grep -E '990[0-3]|2990[0-3]' || true)
if [ -z "$ACTIVE" ]; then
    echo "  All ports free — environment clean."
else
    echo "  WARNING: Ports still in use:"
    echo "$ACTIVE"
fi
