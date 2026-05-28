#!/usr/bin/env bash
set -euo pipefail

# MIRI Configuration
MIRIFLAGS=(
  "-Zmiri-disable-isolation"
  "-Zmiri-num-cpus=1" 
  "-Zmiri-tree-borrows"
)

export MIRIFLAGS="${MIRIFLAGS[*]}"

echo "🔍 Running MIRI with Tree Borrows model..."
echo "   Flags: $MIRIFLAGS"

cargo miri test -p amun-storage-kernel
echo "✅ MIRI audit complete"
