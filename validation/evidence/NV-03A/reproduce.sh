#!/bin/bash
set -euo pipefail; GATE="NV-03A"; BASELINE="90f4993"
echo "=== AmunChain ${GATE} Reproduction ==="
CURRENT=$(git rev-parse --short HEAD); [ "$CURRENT" != "$BASELINE" ] && echo "ERROR: Baseline mismatch" && exit 1
cargo build --release --bin test_validator_cluster_determinism
TMPDIR=$(mktemp -d)
./target/release/test_validator_cluster_determinism "$TMPDIR"
