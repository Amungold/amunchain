#!/bin/bash
# Restart a single validator by ID
set -euo pipefail
VPS_ID=${1:?Usage: restart_validator.sh <vps_id>}
VPS_HOSTS=("VPS0_IP" "VPS1_IP" "VPS2_IP" "VPS3_IP")
HOST="${VPS_HOSTS[$VPS_ID]}"

echo "Restarting validator $VPS_ID on $HOST..."
ssh root@$HOST "pkill -f 'amun-node' || true"
sleep 3
ssh root@$HOST "cd /opt/amunchain && nohup ./bin/amun-node --config config/vps/vps${VPS_ID}.toml > data/validator.log 2>&1 &"
echo "Validator $VPS_ID restarted."
