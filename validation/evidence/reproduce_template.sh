#!/bin/bash
set -euo pipefail
GATE="[TO_BE_FILLED]"
BASELINE="90f4993"
EVIDENCE_DIR="validation/evidence/${GATE}"
echo "=== AmunChain Evidence Reproduction ==="
echo "Gate: ${GATE} | Baseline: ${BASELINE}"
CURRENT=$(git rev-parse HEAD)
[ "$CURRENT" != "$BASELINE" ] && echo "ERROR: Checkout $BASELINE first" && exit 1
[ ! -f "${EVIDENCE_DIR}/genesis.json" ] && echo "ERROR: genesis.json missing" && exit 1
EXPECTED=$(awk '{print $1}' "${EVIDENCE_DIR}/genesis.sha256")
ACTUAL=$(sha256sum "${EVIDENCE_DIR}/genesis.json" | awk '{print $1}')
[ "$EXPECTED" != "$ACTUAL" ] && echo "ERROR: Genesis hash mismatch" && exit 1
cargo build --release
# [TO BE FILLED: Test procedure]
# [TO BE FILLED: Verification]
echo "=== Reproduction Complete ==="
