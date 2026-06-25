#!/bin/bash
#=============================================================================
# AmunChain N103 Resource Audit — A18 (Memory), A19 (CPU), A20 (Disk)
# Version: v0.3.2-cca-complete  (Revised: bc check, wal/ path)
# Usage:   AMUN_DATA_DIR=/data/chain ./resource_monitor.sh <pid> <output_csv> [rpc_port]
#=============================================================================
set -euo pipefail

# --- Pre‑flight checks ----------------------------------------------------
command -v bc >/dev/null 2>&1 || { echo "ERROR: bc is required but not installed"; exit 1; }

PID="${1:?Usage: $0 <pid> <output_csv> [rpc_port]}"
OUTPUT="${2:-resource_log.csv}"
RPC_PORT="${3:-9070}"
INTERVAL=60

# --- Data directory (env override or safe default) -----------------------
DATA_DIR="${AMUN_DATA_DIR:-/root/projects/amunchain}"

# --- CPU core count for multi‑core normalisation -------------------------
CPU_COUNT=$(nproc 2>/dev/null || echo "1")

# --- Header block --------------------------------------------------------
{
    echo "# AmunChain N103 Resource Audit"
    echo "# Version: v0.3.2-cca-complete"
    echo "# Created: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "# PID: $PID   Interval: ${INTERVAL}s   RPC: 127.0.0.1:$RPC_PORT"
    echo "# Cores: $CPU_COUNT"
    echo "# Columns: timestamp,pid,rss_kb,vmswap_kb,vsz_kb,cpu_pct,threads,fds,conns,disk_data_kb,disk_wal_kb,load_1m,block_height,blocks_per_min"
    echo "#"
    echo "timestamp,pid,rss_kb,vmswap_kb,vsz_kb,cpu_pct,threads,fds,conns,disk_data_kb,disk_wal_kb,load_1m,block_height,blocks_per_min"
} > "$OUTPUT"

# --- Helper: fetch block height with flexible JSON parsing ---------------
fetch_height() {
    curl -s --max-time 5 "http://127.0.0.1:${RPC_PORT}/status" 2>/dev/null \
        | python3 -c "
import sys, json
try:
    obj = json.load(sys.stdin)
    print(obj.get('height') or obj.get('tip',{}).get('height') or 0)
except:
    print(0)
" 2>/dev/null || echo "0"
}

# --- Per‑process CPU via /proc/PID/stat vs /proc/stat --------------------
calc_cpu_pct() {
    local PROC_JIFFIES SYS_JIFFIES
    PROC_JIFFIES=$(awk '{print $14+$15}' /proc/"$PID"/stat 2>/dev/null || echo "0")
    SYS_JIFFIES=$(awk '{sum=0; for(i=2;i<=NF;i++) sum+=$i; print sum}' /proc/stat 2>/dev/null | head -1 || echo "0")
    
    if [ -n "$PREV_PROC" ] && [ -n "$PREV_SYS" ] && [ "$SYS_JIFFIES" -gt "${PREV_SYS:-0}" ] 2>/dev/null; then
        local DELTA_PROC=$((PROC_JIFFIES - PREV_PROC))
        local DELTA_SYS=$((SYS_JIFFIES - PREV_SYS))
        if [ "$DELTA_SYS" -gt 0 ] 2>/dev/null; then
            echo "scale=2; 100 * $CPU_COUNT * $DELTA_PROC / $DELTA_SYS" | bc
        else
            echo "0"
        fi
    else
        echo "0"
    fi
    PREV_PROC=$PROC_JIFFIES
    PREV_SYS=$SYS_JIFFIES
}

# --- Monitor loop --------------------------------------------------------
PREV_PROC=0
PREV_SYS=0
LAST_HEIGHT=0
LAST_TS=0

while kill -0 "$PID" 2>/dev/null; do
    TS=$(date +%s)

    # Memory (from /proc/PID/status)
    RSS=$(grep VmRSS /proc/"$PID"/status 2>/dev/null | awk '{print $2}' || echo "0")
    SWAP=$(grep VmSwap /proc/"$PID"/status 2>/dev/null | awk '{print $2}' || echo "0")
    VSZ=$(grep VmSize /proc/"$PID"/status 2>/dev/null | awk '{print $2}' || echo "0")

    # Threads
    THREADS=$(grep Threads /proc/"$PID"/status 2>/dev/null | awk '{print $2}' || echo "0")

    # File descriptors
    FDS=$(ls /proc/"$PID"/fd 2>/dev/null | wc -l)

    # Active TCP connections (portable across distros)
    CONNS=$(ss -tanp 2>/dev/null | grep -F "$PID," | wc -l)

    # CPU % (per‑process, per‑second resolution, multi‑core normalised)
    CPU_PCT=$(calc_cpu_pct)

    # Disk usage (separate data/ and wal/)
    DISK_DATA=$(du -sk "${DATA_DIR}/data" 2>/dev/null | cut -f1 || echo "0")
    DISK_WAL=$(du -sk "${DATA_DIR}/wal" 2>/dev/null | cut -f1 || echo "0")

    # System load
    LOAD=$(awk '{print $1}' /proc/loadavg)

    # Block height and blocks per minute
    HEIGHT=$(fetch_height)
    if [ "$LAST_HEIGHT" -gt 0 ] 2>/dev/null && [ "$LAST_TS" -gt 0 ] 2>/dev/null; then
        BLOCKS_DELTA=$((HEIGHT - LAST_HEIGHT))
        TIME_DELTA=$((TS - LAST_TS))
        if [ "$TIME_DELTA" -gt 0 ] 2>/dev/null; then
            BPM=$((BLOCKS_DELTA * 60 / TIME_DELTA))
        else
            BPM=0
        fi
    else
        BPM=0
    fi
    LAST_HEIGHT=$HEIGHT
    LAST_TS=$TS

    echo "${TS},${PID},${RSS},${SWAP},${VSZ},${CPU_PCT},${THREADS},${FDS},${CONNS},${DISK_DATA},${DISK_WAL},${LOAD},${HEIGHT},${BPM}" >> "$OUTPUT"

    sleep "$INTERVAL"
done

echo "# Monitor stopped: PID $PID terminated at $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$OUTPUT"
