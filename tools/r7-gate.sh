#!/bin/bash
# R7 Encapsulation Gate — CI-ready regression guard (v4 FINAL)
#
# Ensures ConsensusEngine remains the single source of truth
# for consensus state by detecting direct internal field access.
#
# Uses git-grep for speed and automatic target exclusion.
# Shows build/test output on failure for CI diagnostics.
#
# This is a TEXT-BASED regression guard, not a formal architectural
# verifier. It does not replace code review or AST-based analysis.
#
# Update the field list in check [4/4] when adding new internal
# fields to ConsensusEngine.
#
# Exit 0 = PASS, Exit 1 = FAIL

set -uo pipefail

PASS=0
FAIL=0

ENGINE_SRC="crates/amun-consensus-network/src/engine.rs"
ROUND_SRC="crates/amun-consensus-network/src/round.rs"

echo "================ R7 ENCAPSULATION GATE ================"

echo ""
echo "[1/4] ConsensusRound leaks outside engine.rs/round.rs..."
LEAKS=$(git grep -nE \
    "(\.rounds([[:space:]]|\.|\[|;|,|\)|\}|$)|rounds\.(get|get_mut|insert|remove|entry|iter|values|keys)\(|ConsensusRound([[:space:]]|\{|<))" \
    -- "crates/*.rs" "crates/**/*.rs" \
    | grep -v "$ENGINE_SRC" \
    | grep -v "$ROUND_SRC" \
    | grep -v "crates/amun-consensus-types/" \
    | grep -v "amun-networking/tests/" \
    | grep -v "amun-nft-stress/" \
    | grep -v "\.bak" || true)
if [ -z "$LEAKS" ]; then
    echo "  PASS — Zero ConsensusRound leaks"
    PASS=$((PASS+1))
else
    echo "  FAIL — Leaks found:"
    echo "$LEAKS"
    FAIL=$((FAIL+1))
fi

echo ""
echo "[2/4] cargo check --workspace..."
LOG=$(mktemp)
if cargo check --workspace >"$LOG" 2>&1; then
    echo "  PASS"
    PASS=$((PASS+1))
else
    echo "  FAIL — see tail below:"
    tail -30 "$LOG"
    FAIL=$((FAIL+1))
fi
rm -f "$LOG"

echo ""
echo "[3/4] amun-consensus-network tests..."
LOG=$(mktemp)
if cargo test -p amun-consensus-network >"$LOG" 2>&1; then
    echo "  PASS"
    PASS=$((PASS+1))
else
    echo "  FAIL — see tail below:"
    tail -30 "$LOG"
    FAIL=$((FAIL+1))
fi
rm -f "$LOG"

echo ""
echo "[4/4] Engine internal field access..."
KNOWN_FIELDS=$(git grep -nE \
    "engine\.(rounds|current_height|history_root|high_qc|blocks|meta|state|current_step)\b" \
    -- "crates/amun-live-cluster/*.rs" "crates/amun-live-cluster/**/*.rs" \
    | grep -v "RuntimeSummary\|runtime_summary" \
    | grep -v "\.bak" || true)
if [ -z "$KNOWN_FIELDS" ]; then
    echo "  PASS — No known internal field access"
    PASS=$((PASS+1))
else
    echo "  FAIL — Internal fields accessed directly:"
    echo "$KNOWN_FIELDS"
    FAIL=$((FAIL+1))
fi

echo ""
echo "================ R7 VERDICT ================"
echo "  Automated: $PASS/4 PASS  |  $FAIL/4 FAIL"
echo ""

if [ "$FAIL" -eq 0 ]; then
    echo "  R7 ENCAPSULATION: PASS"
    echo "  ConsensusEngine = SINGLE SOURCE OF TRUTH"
    exit 0
else
    echo "  R7 ENCAPSULATION: FAIL"
    exit 1
fi
