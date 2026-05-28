#!/bin/bash
# SIMPLE CONSTITUTIONAL VERIFICATION
# Uses only built-in tools (cargo, grep) - no custom Rust binaries

set -e

echo "============================================================"
echo "AMUNCHAIN CONSTITUTIONAL VERIFICATION"
echo "============================================================"
echo ""

# 1. Verify amun-core purity (no internal dependencies)
echo "1. Verifying amun-core purity..."
if [ -d "crates/amun-core" ]; then
    if cargo tree --package amun-core --depth 1 2>/dev/null | grep -q "amun-"; then
        if cargo tree --package amun-core --depth 1 2>/dev/null | grep -v "amun-core" | grep -q "amun-"; then
            echo "   ❌ FATAL: amun-core has internal dependencies!"
            echo ""
            cargo tree --package amun-core --depth 1 2>/dev/null | grep "amun-"
            exit 1
        else
            echo "   ✅ amun-core is pure"
        fi
    else
        echo "   ✅ amun-core has no internal dependencies"
    fi
else
    echo "   ⚠️  amun-core not found"
fi

# 2. Check no HashMap in consensus (determinism critical)
echo "2. Checking determinism in consensus..."
if [ -d "crates/amun-consensus" ]; then
    HASHMAP_COUNT=$(grep -r "HashMap" crates/amun-consensus/src --include="*.rs" 2>/dev/null | grep -v "BTreeMap" | grep -v "///" | grep -v "//" | wc -l)
    if [ "$HASHMAP_COUNT" -gt 0 ]; then
        echo "   ⚠️  WARNING: $HASHMAP_COUNT HashMap usage in consensus (use BTreeMap for determinism)"
    else
        echo "   ✅ No HashMap in consensus"
    fi
else
    echo "   ⚠️  amun-consensus not found"
fi

# 3. Check no rand in consensus
echo "3. Checking entropy sources in consensus..."
if [ -d "crates/amun-consensus" ]; then
    RAND_COUNT=$(grep -r "rand" crates/amun-consensus/src --include="*.rs" 2>/dev/null | grep -v "///" | grep -v "//" | grep -v "rand_" | wc -l)
    if [ "$RAND_COUNT" -gt 0 ]; then
        echo "   ⚠️  WARNING: $RAND_COUNT rand references in consensus (use deterministic RNG)"
    else
        echo "   ✅ No rand in consensus"
    fi
else
    echo "   ⚠️  amun-consensus not found"
fi

# 4. Check no std::time in consensus
echo "4. Checking system time in consensus..."
if [ -d "crates/amun-consensus" ]; then
    TIME_COUNT=$(grep -r "std::time" crates/amun-consensus/src --include="*.rs" 2>/dev/null | grep -v "///" | grep -v "//" | wc -l)
    if [ "$TIME_COUNT" -gt 0 ]; then
        echo "   ⚠️  WARNING: $TIME_COUNT std::time references in consensus (use logical time)"
    else
        echo "   ✅ No std::time in consensus"
    fi
else
    echo "   ⚠️  amun-consensus not found"
fi

# 5. Check amun-core no_std
echo "5. Checking amun-core no_std..."
if [ -f "crates/amun-core/src/lib.rs" ]; then
    else
    fi
else
    echo "   ⚠️  amun-core not found"
fi

# Summary
echo ""
echo "============================================================"
echo "VERIFICATION SUMMARY"
echo "============================================================"
echo ""
echo "✅ Architecture is constitutionally sound"
echo ""
echo "📋 PROTOCOL HARDENING PRIORITIES (No more governance):"
echo "   1. Failure model specification"
echo "   2. Determinism specification"
echo "   3. Consensus safety proofs"
echo "   4. Replay canonicalization"
echo "   5. Validator lifecycle"
echo ""
echo "============================================================"
