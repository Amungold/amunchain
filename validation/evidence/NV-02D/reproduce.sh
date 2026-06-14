#!/bin/bash
set -euo pipefail; GATE="NV-02D"; BASELINE="90f4993"
echo "=== AmunChain ${GATE} Reproduction ==="
CURRENT=$(git rev-parse --short HEAD); [ "$CURRENT" != "$BASELINE" ] && echo "ERROR: Baseline mismatch" && exit 1
cargo build --release --bin test_replay_determinism
TMPDIR=$(mktemp -d)
./target/release/test_replay_determinism "$TMPDIR"
