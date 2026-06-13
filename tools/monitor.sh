#!/bin/bash
# AmunChain Multi‑VPS monitor — polls metrics from every node
set -euo pipefail

VPS_LIST=(
  "root@10.0.0.1"
  "root@10.0.0.2"
  "root@10.0.0.3"
  "root@10.0.0.4"
)
SSH_KEY="$HOME/.ssh/id_rsa"
REMOTE_DIR="/opt/amunchain"
INTERVAL="${1:-10}"

echo "=== AmunChain Cluster Monitor (refresh every ${INTERVAL}s) ==="
echo ""

while true; do
  clear
  echo "=== AmunChain Cluster Status — $(date) ==="
  echo ""
  printf "%-4s %-16s %-10s %-8s %-8s %-8s\n" "V" "HOST" "HEIGHT" "ROUNDS" "SPREAD" "STATUS"
  printf "%-4s %-16s %-10s %-8s %-8s %-8s\n" "---" "----------------" "----------" "--------" "--------" "------"
  
  HEIGHTS=()
  for i in "${!VPS_LIST[@]}"; do
    HOST="${VPS_LIST[$i]}"
    METRICS=$(ssh -i "$SSH_KEY" "$HOST" "tail -1 $REMOTE_DIR/data/validator.log 2>/dev/null | grep -oP 'qcs:\K[0-9]+|final:\K[0-9]+|votes:\K[0-9]+'" 2>/dev/null || echo "")
    H=$(echo "$METRICS" | head -1)
    [[ -z "$H" ]] && H="?"
    HEIGHTS+=("$H")
    
    if ssh -i "$SSH_KEY" "$HOST" "pgrep -f 'bin/validator' > /dev/null" 2>/dev/null; then
      STATUS="RUNNING"
    else
      STATUS="STOPPED"
    fi
    
    printf "%-4s %-16s %-10s %-8s %-8s %-8s\n" "$i" "$HOST" "$H" "-" "-" "$STATUS"
  done
  
  # Calculate spread
  VALID_H=()
  for h in "${HEIGHTS[@]}"; do
    [[ "$h" != "?" ]] && VALID_H+=("$h")
  done
  if [[ ${#VALID_H[@]} -gt 1 ]]; then
    MIN=$(printf '%s\n' "${VALID_H[@]}" | sort -n | head -1)
    MAX=$(printf '%s\n' "${VALID_H[@]}" | sort -n | tail -1)
    SPREAD=$((MAX - MIN))
    echo ""
    echo "Height spread: $SPREAD (min=$MIN max=$MAX)"
  fi
  
  sleep "$INTERVAL"
done
