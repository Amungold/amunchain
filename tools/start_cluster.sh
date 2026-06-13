#!/bin/bash
# AmunChain Multi‑VPS cluster starter
set -euo pipefail

VPS_LIST=(
  "root@10.0.0.1"
  "root@10.0.0.2"
  "root@10.0.0.3"
  "root@10.0.0.4"
)
SSH_KEY="$HOME/.ssh/id_rsa"
REMOTE_DIR="/opt/amunchain"

echo "=== Starting AmunChain cluster on ${#VPS_LIST[@]} VPS ==="

for i in "${!VPS_LIST[@]}"; do
  HOST="${VPS_LIST[$i]}"
  echo "Starting validator $i on $HOST..."
  ssh -i "$SSH_KEY" "$HOST" "bash $REMOTE_DIR/run_validator.sh" &
done

wait
echo "=== All validators launched ==="
echo "=== Check status: bash tools/monitor.sh ==="
