#!/bin/bash
# Local multi‑VPS monitor (no SSH needed)
INTERVAL="${1:-10}"

while true; do
    clear
    echo "=== AmunChain Local VPS Monitor — $(date) ==="
    echo ""
    printf "%-4s %-10s %-12s %-10s\n" "VPS" "HEIGHT" "FINALIZED" "STATUS"
    printf "%-4s %-10s %-12s %-10s\n" "---" "----------" "------------" "------"
    
    for i in 0 1 2 3; do
        LOG="/tmp/vps_v${i}.log"
        if pgrep -f "validator.*$i 9900" > /dev/null 2>&1; then
            METRICS=$(tail -1 "$LOG" 2>/dev/null | grep -oP 'qcs:\K[0-9]+|final:\K[0-9]+' || echo "")
            H=$(echo "$METRICS" | head -1)
            F=$(echo "$METRICS" | tail -1)
            [[ -z "$H" ]] && H="-"
            [[ -z "$F" ]] && F="-"
            printf "%-4s %-10s %-12s %-10s\n" "$i" "$H" "$F" "RUNNING"
        else
            printf "%-4s %-10s %-12s %-10s\n" "$i" "-" "-" "STOPPED"
        fi
    done
    
    echo ""
    echo "Evidence registry counts:"
    for i in 0 1 2 3; do
        COUNT=$(ls /tmp/amun-vps-$i/evidence/ 2>/dev/null | wc -l)
        echo "  VPS $i: $COUNT proofs"
    done
    
    sleep "$INTERVAL"
done
