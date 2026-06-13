#!/bin/bash
# Test TCP connectivity between all VPS pairs
set -euo pipefail

VPS_HOSTS=("VPS0_IP" "VPS1_IP" "VPS2_IP" "VPS3_IP")
VPS_PORTS=(9000 9001 9002 9003)
FAILED=0
TOTAL=0

echo "=== Connectivity Test ==="
for i in 0 1 2 3; do
    for j in 0 1 2 3; do
        if [ $i -ne $j ]; then
            TOTAL=$((TOTAL + 1))
            if timeout 5 bash -c "echo >/dev/tcp/${VPS_HOSTS[$j]}/${VPS_PORTS[$j]}" 2>/dev/null; then
                echo "  VPS$i -> VPS$j: PASS"
            else
                echo "  VPS$i -> VPS$j: FAIL"
                FAILED=$((FAILED + 1))
            fi
        fi
    done
done

echo ""
echo "Result: $((TOTAL - FAILED))/$TOTAL passed"
if [ $FAILED -gt 0 ]; then
    echo "FAILED: $FAILED connection(s) unreachable"
    exit 1
else
    echo "PASS: 100% connectivity"
fi
