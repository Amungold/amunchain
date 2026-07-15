#!/usr/bin/env bash
set -e

echo "==============================================================="
echo "        AMUNCHAIN ARCHITECTURE EXTRACTION AUDIT (AERA-1)"
echo "==============================================================="
echo

echo "[1/10] Workspace"
echo "---------------------------------------------------------------"
cargo metadata --format-version 1 --no-deps | grep '"name"' | head -20
echo

echo "[2/10] Crates"
echo "---------------------------------------------------------------"
find crates -maxdepth 2 -name Cargo.toml | sort
echo

echo "[3/10] Public Traits"
echo "---------------------------------------------------------------"
grep -R "^pub trait" crates || true
echo

echo "[4/10] Public Structs"
echo "---------------------------------------------------------------"
grep -R "^pub struct" crates || true
echo

echo "[5/10] Public Enums"
echo "---------------------------------------------------------------"
grep -R "^pub enum" crates || true
echo

echo "[6/10] Entry Points"
echo "---------------------------------------------------------------"
find crates -name main.rs -o -path "*/src/bin/*.rs" | sort
echo

echo "[7/10] Background Threads"
echo "---------------------------------------------------------------"
grep -R "thread::spawn\|tokio::spawn" crates || true
echo

echo "[8/10] Synchronization"
echo "---------------------------------------------------------------"
grep -R "Arc<\|Mutex<\|RwLock<" crates || true
echo

echo "[9/10] Validator Runtime"
echo "---------------------------------------------------------------"
grep -R "pub struct LiveValidator" -A40 crates || true
echo

echo "[10/10] Consensus Engine"
echo "---------------------------------------------------------------"
grep -R "pub struct ConsensusEngine" -A50 crates || true

echo
echo "==============================================================="
echo "AERA-1 PHASE 1 COMPLETE"
echo "==============================================================="
