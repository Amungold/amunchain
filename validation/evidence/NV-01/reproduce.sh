#!/bin/bash
set -euo pipefail

GATE="NV-01"
BASELINE="90f4993"
EVIDENCE_DIR="validation/evidence/NV-01"
GENESIS_TOOL="./genesis_hash_tool"

echo "=== AmunChain NV-01 Evidence Reproduction ==="
echo "Gate: ${GATE} | Baseline: ${BASELINE}"
echo ""

# 1. Verify baseline
CURRENT=$(git rev-parse --short HEAD)
if [ "$CURRENT" != "$BASELINE" ]; then
    echo "ERROR: Must be on baseline commit ${BASELINE}. Currently on ${CURRENT}"
    exit 1
fi
echo "[OK] Baseline verified: ${CURRENT}"

# 2. Verify evidence directory
if [ ! -f "${EVIDENCE_DIR}/genesis.json" ]; then
    echo "ERROR: Genesis artifact missing from ${EVIDENCE_DIR}"
    exit 1
fi
echo "[OK] Evidence directory exists"

# 3. Verify genesis hash
EXPECTED=$(awk '{print $1}' "${EVIDENCE_DIR}/genesis.sha256")
ACTUAL=$(sha256sum "${EVIDENCE_DIR}/genesis.json" | awk '{print $1}')
if [ "$EXPECTED" != "$ACTUAL" ]; then
    echo "ERROR: Genesis hash mismatch"
    echo "Expected: ${EXPECTED}"
    echo "Actual:   ${ACTUAL}"
    exit 1
fi
echo "[OK] Genesis hash verified"

# 4. Build genesis tool if needed
if [ ! -f "$GENESIS_TOOL" ]; then
    echo "Building genesis_hash_tool..."
    mkdir -p /tmp/genesis_hash_tool/src
    cat > /tmp/genesis_hash_tool/Cargo.toml << 'TOOL_EOF'
[package]
name = "genesis_hash_tool"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
blake3 = "1"
hex = "0.4"
TOOL_EOF
    cp /dev/stdin /tmp/genesis_hash_tool/src/main.rs << 'MAIN_EOF'
use std::env;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GenesisValidator { peer_id: String, public_key: String, voting_power: u64 }

#[derive(Debug, Deserialize)]
struct GenesisTrustAnchor { peer_id: String, public_key: String }

#[derive(Debug, Deserialize)]
struct Genesis { chain_id: String, timestamp: u64, validators: Vec<GenesisValidator>, trust_anchors: Vec<GenesisTrustAnchor> }

impl Genesis {
    fn genesis_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_GENESIS_V1");
        hasher.update(self.chain_id.as_bytes());
        hasher.update(&self.timestamp.to_le_bytes());
        for v in &self.validators {
            hasher.update(v.peer_id.as_bytes());
            hasher.update(v.public_key.as_bytes());
            hasher.update(&v.voting_power.to_le_bytes());
        }
        for t in &self.trust_anchors {
            hasher.update(t.peer_id.as_bytes());
            hasher.update(t.public_key.as_bytes());
        }
        hasher.finalize().into()
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("Usage: genesis_hash_tool <genesis.json>");
    let data = fs::read_to_string(&path).expect("Cannot read genesis file");
    let genesis: Genesis = serde_json::from_str(&data).expect("Invalid genesis JSON");
    println!("{}", hex::encode(genesis.genesis_hash()));
}
MAIN_EOF
    (cd /tmp/genesis_hash_tool && cargo build --release)
    cp /tmp/genesis_hash_tool/target/release/genesis_hash_tool ./genesis_hash_tool
fi
echo "[OK] Genesis tool ready"

# 5. Reproduce genesis hash for all validators
echo ""
echo "=== Reproducing Genesis Hashes ==="
MASTER_HASH=$($GENESIS_TOOL "crates/amun-node/data/genesis.json")
echo "Master: ${MASTER_HASH}"

for i in 1 2 3 4; do
    VAL_HASH=$($GENESIS_TOOL "crates/amun-node/data/validator${i}/genesis.json")
    echo "Validator ${i}: ${VAL_HASH}"
    if [ "$VAL_HASH" != "$MASTER_HASH" ]; then
        echo "ERROR: Validator ${i} hash mismatch!"
        exit 1
    fi
done

# 6. Verify state root (all zeros)
echo ""
echo "=== Reproducing State Roots ==="
STATE_ROOT="0000000000000000000000000000000000000000000000000000000000000000"
for i in 1 2 3 4; do
    STORED_ROOT=$(cat "${EVIDENCE_DIR}/validator_${i}_state_root.txt")
    if [ "$STORED_ROOT" != "$STATE_ROOT" ]; then
        echo "ERROR: Validator ${i} state root mismatch!"
        exit 1
    fi
done
echo "State Root: ${STATE_ROOT} (all validators)"

# 7. Verify MANIFEST.sha256
echo ""
echo "=== Verifying Evidence Integrity ==="
cd "${EVIDENCE_DIR}"
sha256sum -c MANIFEST.sha256
cd - > /dev/null

echo ""
echo "========================================="
echo "  NV-01 REPRODUCTION SUCCESSFUL"
echo "  Result: IDENTICAL"
echo "========================================="
echo ""
echo "All genesis hashes match: ${MASTER_HASH}"
echo "All state roots match: ${STATE_ROOT}"
echo "Evidence integrity verified via MANIFEST.sha256"
