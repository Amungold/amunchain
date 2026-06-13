#!/bin/bash
# Run the full N102.3 test suite
set -euo pipefail

echo "=== N102.3 Multi-VPS Test Suite ==="
echo ""

echo "Phase A: Connectivity"
bash tools/check_connectivity.sh || exit 1

echo ""
echo "Phase B: Deploy + Start Cluster"
bash tools/deploy.sh
bash tools/start_cluster.sh
sleep 30

echo ""
echo "Phase C: Consensus Check (10 min)"
bash tools/monitor.sh 10

echo ""
echo "Phase D: Evidence Gossip"
echo "Injecting proof on VPS0..."
ssh root@VPS0_IP "echo '{\"type\":\"Equivocation\",\"validator\":\"test\",\"height\":99}' > /opt/amunchain/data/vps0/evidence/test.json"
sleep 60
for i in 0 1 2 3; do
    HOST="VPS${i}_IP"
    COUNT=$(ssh root@$HOST "ls /opt/amunchain/data/vps${i}/evidence/ 2>/dev/null | wc -l")
    echo "  VPS$i: $COUNT evidence files"
done

echo ""
echo "Phase E: Crash-Rejoin"
bash tools/restart_validator.sh 2
sleep 60
bash tools/monitor.sh 1

echo ""
echo "Phase F: Full Restart"
bash tools/stop_cluster.sh
sleep 10
bash tools/start_cluster.sh
sleep 30
bash tools/monitor.sh 3

echo ""
echo "=== N102.3 Suite Complete ==="
