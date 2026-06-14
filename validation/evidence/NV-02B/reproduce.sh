#!/bin/bash
set -euo pipefail; GATE="NV-02B"; BASELINE="90f4993"
echo "=== AmunChain ${GATE} Reproduction ==="
CURRENT=$(git rev-parse --short HEAD); [ "$CURRENT" != "$BASELINE" ] && echo "ERROR: Baseline mismatch" && exit 1
cargo build --release --bin test_multi_height_determinism
TMPDIR=$(mktemp -d)
./target/release/test_multi_height_determinism "$TMPDIR"
