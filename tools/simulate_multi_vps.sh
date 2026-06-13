#!/bin/bash
# Simulate Multi-VPS on localhost using different ports and isolated data dirs
set -euo pipefail

VPS_COUNT=4
BASE_PORT=9900
declare -a PIDS=()

echo "=== Simulating $VPS_COUNT VPS on localhost ==="

# Build
cargo build --release --bin validator --bin evidence_service 2>&1 | tail -3

# Clean previous
pkill -f "validator\|evidence_service" 2>/dev/null || true
sleep 2

# Start validators (each simulating a different VPS)
for i in $(seq 0 $((VPS_COUNT - 1))); do
    DATA_DIR="/tmp/amun-vps-$i"
    rm -rf "$DATA_DIR"
    mkdir -p "$DATA_DIR/evidence"
    
    cargo run --release --bin validator -- $i $BASE_PORT > "/tmp/vps_v${i}.log" 2>&1 &
    PIDS+=($!)
    echo "  VPS $i: PID ${PIDS[$i]} (port $((BASE_PORT + i)))"
done

sleep 10

# Start evidence services
for i in $(seq 0 $((VPS_COUNT - 1))); do
    PEERS=""
    for j in $(seq 0 $((VPS_COUNT - 1))); do
        [[ $j -eq $i ]] && continue
        PEERS="$PEERS 127.0.0.1:$((29900 + j))"
    done
    cargo run --release --bin evidence_service \
        $((29900 + i)) "/tmp/amun-vps-$i" $PEERS > "/tmp/vps_evidence_${i}.log" 2>&1 &
    echo "  Evidence VPS $i: port $((29900 + i))"
done

echo ""
echo "=== Cluster running. Monitor with: ==="
echo "  watch -n 5 'tail -3 /tmp/vps_v0.log /tmp/vps_v1.log /tmp/vps_v2.log /tmp/vps_v3.log'"
echo "  bash tools/monitor_local.sh"
echo ""
echo "=== Inject test evidence ==="
sleep 30
echo '{"type":"Equivocation","validator":"0101010101010101010101010101010101010101010101010101010101010101","height":42,"block_a":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","block_b":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb","timestamp":1719000000}' > /tmp/amun-vps-0/evidence/equivocation_42.json
echo "Evidence injected into VPS 0. Check propagation in 30s."
