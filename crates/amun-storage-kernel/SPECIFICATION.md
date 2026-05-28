# AMUNCHAIN STORAGE KERNEL SPECIFICATION v1.0
## Derived from the Constitution - Implementation Semantics

---

### 1. ENCODING (CCBF)
- Multibyte integers: little-endian
- Variable-length: u64 length prefix + data
- Fixed-width hashes: raw 32 bytes, no length prefix
- All messages: self-describing, exhaustively decodable

### 2. NODE ENCODING
- Leaf: 0x01 || key_hash(32) || value_hash(32) || version(u64)
- Branch: 0x02 || left(32) || right(32)

### 3. HASHING (Domain-Separated)
- Leaf: blake3("AMUN_LEAF_V1" || key_hash || value_hash || version)
- Branch: blake3("AMUN_BRANCH_V1" || left || right)
- WAL: blake3("AMUN_WAL_FRAME_V1" || "AMUNCHAIN_MAINNET_V1" || epoch || generation || sequence || op_type || tx_id || key || value || version || state_root || prev_hash)
- Lineage: blake3("AMUN_LINEAGE_V1" || epoch || generation || state_root || prev_state_root || wal_sequence)

### 4. STORAGE LAYOUT
- NodeStore: {root}/{hex[0..2]}/{hex[2..4]}/{hex[4..]}.node
- ValueStore: {root}/{hex[0..2]}/{hex[2..4]}/{hex[4..]}.val
- Atomic write: tmp -> fsync(tmp) -> rename -> fsync(parent_dir)

### 5. CONSTANTS
| Constant | Value |
|----------|-------|
| MAX_DEPTH | 256 |
| PROOF_VERSION_V1 | 0x01 |
| Terminal Empty Node | [0u8; 32] |
