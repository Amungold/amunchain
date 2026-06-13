#!/bin/bash
# AmunChain Multi‑VPS deployer — copies release binary + config to each VPS
set -euo pipefail

# ── Config ────────────────────────────────────────────────────
VPS_LIST=(
  "root@10.0.0.1"
  "root@10.0.0.2"
  "root@10.0.0.3"
  "root@10.0.0.4"
)
SSH_KEY="$HOME/.ssh/id_rsa"
REMOTE_DIR="/opt/amunchain"
LOCAL_BIN="target/release/validator"
LOCAL_TOOLS="target/release/evidence_service"

# ── Build if needed ──────────────────────────────────────────
cargo build --release --bin validator --bin evidence_service 2>&1 | tail -3

# ── Deploy loop ──────────────────────────────────────────────
for i in "${!VPS_LIST[@]}"; do
  HOST="${VPS_LIST[$i]}"
  echo "=== Deploying to $HOST (validator $i) ==="
  
  ssh -i "$SSH_KEY" "$HOST" "mkdir -p $REMOTE_DIR/bin $REMOTE_DIR/data"
  
  scp -i "$SSH_KEY" "$LOCAL_BIN" "$HOST:$REMOTE_DIR/bin/validator"
  scp -i "$SSH_KEY" "$LOCAL_TOOLS" "$HOST:$REMOTE_DIR/bin/evidence_service" 2>/dev/null || true
  
  # Generate cluster config on the remote
  PEERS=""
  for j in "${!VPS_LIST[@]}"; do
    [[ $j -eq $i ]] && continue
    PEERS="$PEERS 127.0.0.1:$((9900+j))"   # replace with real IPs in production
  done
  
  ssh -i "$SSH_KEY" "$HOST" "cat > $REMOTE_DIR/run_validator.sh" <<< "
#!/bin/bash
cd $REMOTE_DIR
export RUST_LOG=info
nohup ./bin/validator $i 9900 > ./data/validator.log 2>&1 &
echo \$! > ./data/validator.pid
echo 'Validator $i started (PID '"\$(cat ./data/validator.pid)"')'
"
  
  ssh -i "$SSH_KEY" "$HOST" "chmod +x $REMOTE_DIR/run_validator.sh"
  echo "  Deploy complete."
done

echo "=== All VPS deployed. Run start_cluster.sh to launch. ==="
