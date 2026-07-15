#!/usr/bin/env bash
set +e

CRATES=(
  amun-validator-api
  amun-validator-runtime
  amun-validator-identity
  amun-validator-bootstrap
)

PASS=0
FAIL=0

run() {
    NAME="$1"
    CMD="$2"

    echo -n "[CHECK] $NAME ... "

    OUT=$(mktemp)

    eval "$CMD" >"$OUT" 2>&1
    RC=$?

    if [ $RC -eq 0 ]; then
        echo "PASS"
        PASS=$((PASS+1))
    else
        echo "FAIL"
        FAIL=$((FAIL+1))
        echo "--------------------------------------------------"
        grep -E "error\[|error:|warning:|failed|FAIL|cannot find|unresolved|panicked" "$OUT" | head -20
        echo "--------------------------------------------------"
    fi

    rm -f "$OUT"
}

echo "========================================================"
echo "        AMUNCHAIN VALIDATOR PLATFORM AUDIT"
echo "========================================================"

run "Workspace Check" \
"cargo check $(printf ' -p %s' "${CRATES[@]}")"

run "Clippy" \
"cargo clippy $(printf ' -p %s' "${CRATES[@]}") -- -D warnings"

run "Tests" \
"cargo test $(printf ' -p %s' "${CRATES[@]}")"

run "Formatting" \
"cargo fmt --all -- --check"

run "Dependency Audit" \
"cargo audit"

run "License Audit" \
"cargo deny check"

run "Unused Dependencies" \
"cargo machete"

run "Release Build" \
"cargo build --release $(printf ' -p %s' "${CRATES[@]}")"

echo
echo "==================== FINAL REPORT ===================="

TOTAL=$((PASS+FAIL))

echo "Checks : $TOTAL"
echo "Passed : $PASS"
echo "Failed : $FAIL"

if [ $FAIL -eq 0 ]; then
    echo
    echo "STATUS : ✅ PLATFORM VERIFIED"
else
    echo
    echo "STATUS : ❌ PLATFORM HAS ISSUES"
fi

echo "======================================================"
