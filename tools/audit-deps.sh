#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."

VIOLATIONS=0
check_violation() { echo "  ❌ VIOLATION: $1"; VIOLATIONS=$((VIOLATIONS + 1)); }
check_pass() { echo "  ✅ $1"; }

echo "===== AMUNCHAIN CONSTITUTIONAL AUDIT ====="
echo ""

# ─── Helpers ───────────────────────────────────────────────
crate_exists() { [ -f "crates/${1}/Cargo.toml" ]; }
has_dep() {
    local crate="$1" dep="$2"
    grep -Eq "^[[:space:]]*${dep}[[:space:]]*=[[:space:]]*(\{.*\}|\"[^\"]*\")" "crates/${crate}/Cargo.toml" 2>/dev/null
}
has_amun_dep() {
    local crate="$1"
    grep -P '^\s*amun-[a-z0-9-]+\s*=\s*(\{|")' "crates/${crate}/Cargo.toml" 2>/dev/null | grep -qv "^name"
}

# ─── Layer 0: Cryptographic Kernel ─────────────────────────
echo "--- Layer 0: Cryptographic Kernel ---"
for crate in amun-kernel amun-kernel-types; do
    if crate_exists "$crate"; then
        if has_dep "$crate" "tokio"; then check_violation "${crate} → tokio"; fi
        if has_dep "$crate" "hyper"; then check_violation "${crate} → hyper"; fi
        if has_amun_dep "$crate"; then check_violation "${crate} imports amun-*"; else check_pass "${crate} is pure"; fi
    fi
done

# ─── Interface Layer Protection ────────────────────────────
echo "--- Interface Layer Protection ---"
for iface in amun-execution-interface amun-storage-interface amun-network-interface; do
    if crate_exists "$iface"; then
        if has_dep "$iface" "tokio"; then check_violation "${iface} → tokio"; fi
        if has_dep "$iface" "hyper"; then check_violation "${iface} → hyper"; fi
        src="crates/${iface}/src/lib.rs"
        if [ -f "$src" ]; then
            trait_count=$(grep -cP '^\s*(pub\s+)?trait\s+' "$src" 2>/dev/null || echo 0)
            service_impls=$(grep -P '^\s*impl\s+.*\s+for\s+(Tcp|Http|Ws|Network|Store|Runtime|Tokio)' "$src" 2>/dev/null || true)
            [ -n "$service_impls" ] && check_violation "${iface} has service impls"
            [ "$trait_count" -gt 0 ] && check_pass "${iface} defines ${trait_count} trait(s)"
            if grep -qP '(Send\s*\+\s*Sync)' "$src" 2>/dev/null; then
                grep -qP '@constitutional-runtime-contract' "$src" 2>/dev/null || \
                    check_violation "${iface} uses Send+Sync without @constitutional-runtime-contract"
            fi
        fi
    fi
done

# ─── Layer 1: Consensus Core ───────────────────────────────
echo "--- Layer 1: Consensus Core ---"
if crate_exists "amun-consensus"; then
    for forbidden in amun-runtime amun-storage amun-mempool; do
        has_dep "amun-consensus" "$forbidden" && check_violation "consensus → $forbidden"
    done
fi

# ─── Layer 3: Persistence ──────────────────────────────────
echo "--- Layer 3: Persistence ---"
for crate in amun-wal amun-storage; do
    crate_exists "$crate" && has_dep "$crate" "amun-consensus" && check_violation "${crate} → consensus"
done

# ─── Async Penetration (Layers 0-3) ────────────────────────
echo "--- Async Penetration (Layers 0-3) ---"
LAYERS_0_3=(amun-kernel amun-kernel-types amun-constitution amun-constitution-core amun-invariants
    amun-consensus amun-pacemaker amun-fork-choice-law amun-qc-canonical amun-view-change
    amun-quorum-certificate amun-execution amun-runtime amun-stf amun-state-transition
    amun-wal amun-storage amun-snapshot amun-snapshot-engine amun-state-root amun-state-types)
for crate in "${LAYERS_0_3[@]}"; do
    if crate_exists "$crate"; then
        for async_dep in tokio hyper tungstenite; do
            has_dep "$crate" "$async_dep" && check_violation "${crate} → ${async_dep}"
        done
        if grep -qP '^\s*tokio\s*=\s*\{.*optional\s*=\s*true' "crates/${crate}/Cargo.toml" 2>/dev/null; then
            check_violation "${crate} has optional tokio (requires constitutional review)"
        fi
    fi
done

# ─── Test Dependency Direction ─────────────────────────────
echo "--- Test Dependency Direction ---"
for crate_path in crates/amun-*/Cargo.toml; do
    crate_name=$(basename "$(dirname "$crate_path")")
    [[ "$crate_name" == *-tests || "$crate_name" == *-simulation ]] && continue
    grep -qP '^\s*amun-[a-z0-9-]+-(tests|simulation)\s*=' "$crate_path" 2>/dev/null && \
        check_violation "${crate_name} depends on test crate"
done

# ─── Result ────────────────────────────────────────────────
echo ""
if [ $VIOLATIONS -eq 0 ]; then
    echo "===== ALL CONSTITUTIONAL LAWS PASS ✅ ====="
    exit 0
else
    echo "===== ${VIOLATIONS} CONSTITUTIONAL VIOLATIONS ❌ ====="
    exit 1
fi
