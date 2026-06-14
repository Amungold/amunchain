#!/bin/bash
set -euo pipefail
GATE="NV-01B"
BASELINE="90f4993"
EVIDENCE_DIR="validation/evidence/NV-01B"

echo "=== AmunChain NV-01B Evidence Reproduction ==="
echo "Gate: ${GATE} | Baseline: ${BASELINE}"

CURRENT=$(git rev-parse --short HEAD)
[ "$CURRENT" != "$BASELINE" ] && echo "ERROR: Baseline mismatch" && exit 1
echo "[OK] Baseline verified"

if [ ! -f ./target/release/test_live_state ]; then
    echo "Building test_live_state..."
    cargo build --release --bin test_live_state
fi

TMPDIR=$(mktemp -d)
./target/release/test_live_state "$TMPDIR" | tee /tmp/nv01b_output.txt

for i in 0 1 2 3; do
    grep "^Validator ${i}:" /tmp/nv01b_output.txt | awk '{print $3}' > "${EVIDENCE_DIR}/validator_$((i+1))_state_root.txt"
done

FIRST=$(cat "${EVIDENCE_DIR}/validator_1_state_root.txt")
for i in 2 3 4; do
    CUR=$(cat "${EVIDENCE_DIR}/validator_${i}_state_root.txt")
    [ "$FIRST" != "$CUR" ] && echo "ERROR: Mismatch" && exit 1
done

echo "Reproduction successful. All roots: $FIRST"
echo "NV-01B REPRODUCTION SUCCESSFUL"
