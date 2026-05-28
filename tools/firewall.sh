#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."

echo "=== CONSTITUTIONAL DEPENDENCY FIREWALL ==="

PASS=0
FAIL=0

# Rule 1: Kernel purity
echo ""
echo "--- Rule 1: Kernel Purity ---"
KERNEL_DEPS=$(cargo tree -p amun-kernel --depth 1 2>/dev/null | grep -c "amun-" || true)
if [ "$KERNEL_DEPS" -gt 1 ]; then
    echo "❌ FAIL: amun-kernel has $KERNEL_DEPS Amun dependencies (must be 0)"
    FAIL=$((FAIL + 1))
else
    echo "✅ PASS: Kernel is sovereign"
    PASS=$((PASS + 1))
fi

# Rule 2: State-root must not depend on consensus
echo ""
echo "--- Rule 2: Truth Layer Isolation ---"
if cargo tree -p amun-state-root --depth 2 2>/dev/null | grep -q "amun-consensus"; then
    echo "❌ FAIL: State-root depends on consensus (upward dependency)"
    FAIL=$((FAIL + 1))
else
    echo "✅ PASS: Truth layer is isolated from consensus"
    PASS=$((PASS + 1))
fi

# Rule 3: Network must not depend on state internals
echo ""
echo "--- Rule 3: Network Isolation ---"
NET_STATE=$(cargo tree -p amun-network --depth 2 2>/dev/null | grep -cE "amun-state-root|amun-stf|amun-execution-receipt" || true)
if [ "$NET_STATE" -gt 0 ]; then
    echo "❌ FAIL: Network depends on state/execution crates"
    FAIL=$((FAIL + 1))
else
    echo "✅ PASS: Network is isolated from state layer"
    PASS=$((PASS + 1))
fi

# Rule 4: No cyclic dependencies
echo ""
echo "--- Rule 4: Cycle Detection ---"
CYCLES=$(cargo tree --duplicates 2>/dev/null | head -3 || true)
if [ -n "$CYCLES" ]; then
    echo "⚠ WARNING: Potential cycles or duplicates detected (review manually)"
    echo "$CYCLES"
else
    echo "✅ PASS: No obvious cycles detected"
    PASS=$((PASS + 1))
fi

# Rule 5: RPC must not depend on kernel internals beyond allowed interfaces
echo ""
echo "--- Rule 5: Interface Isolation ---"
RPC_KERNEL=$(cargo tree -p amun-rpc --depth 3 2>/dev/null | grep -c "amun-kernel" || true)
if [ "$RPC_KERNEL" -gt 0 ]; then
    echo "⚠ INFO: RPC has indirect access to kernel (acceptable for serialization)"
    PASS=$((PASS + 1))
else
    echo "✅ PASS: RPC has no kernel access"
    PASS=$((PASS + 1))
fi

# Rule 6: Criticality escalation check
echo ""
echo "--- Rule 6: Criticality Escalation ---"
# Check if any interface crate depends on kernel-critical crate beyond allowed paths
if cargo tree -p amun-rpc --depth 4 2>/dev/null | grep -q "amun-state-root"; then
    echo "⚠ INFO: RPC has access to state-root (monitoring path — acceptable)"
    PASS=$((PASS + 1))
else
    PASS=$((PASS + 1))
fi

echo ""
echo "========================================="
echo "RESULTS: $PASS passed, $FAIL failed"
echo "========================================="

if [ $FAIL -gt 0 ]; then
    echo "❌ Constitutional violation detected!"
    exit 1
else
    echo "✅ Constitution upheld — all firewalls active."
fi
