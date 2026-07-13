# AMUNCHAIN PHASE 84 - CONSTITUTIONAL FREEZE v1.0
## Frozen Invariants for Sovereign State Replication

---

## 1. CANONICAL GEOMETRY (FROZEN)
- MAX_DEPTH: 256 (constitutional constant)
- Tree structure: Full explicit binary, no compression
- Empty ladder: Precomputed, terminal = [0u8; 32]
- Proof geometry: Exactly 256 ProofStep entries per proof

## 2. TRAVERSAL LAW (FROZEN)
- Order: Pure DFS pre-order, left-before-right
- Empty subtrees: Skipped during serialization
- Depth guard: depth >= 256 terminates traversal

## 3. SNAPSHOT FORMAT (FROZEN)
- Magic: "AMSN" (4 bytes)
- Version: SNAPSHOT_VERSION_V1 = 1
- Max chunk size: 16MB constitutional constant
- Chunk Merkle domain: AMUN_CHUNK_MERKLE_V1

## 4. COMPATIBILITY LAW (FROZEN)
- Levels: FullyCompatible > ReplayCompatible > SnapshotCompatible > ReadOnlyCompatible > Incompatible
- Constitutional mismatch + same SMT universe = ReadOnlyCompatible
- Different empty roots = Incompatible (structural universe mismatch)

## 5. PROOF LAW (FROZEN)
- Each chunk carries independent Merkle inclusion proof
- Proof generation uses positional pair tracking (pair_idx * 2)
- Proof verification rebuilds Merkle root from chunk hash + siblings

## 6. BYZANTINE SYNC LAW (FROZEN)
- Consensus key: (identity_hash, manifest_hash) composite
- Only Identical civilizations can sync state
- Quorum selects authoritative manifest

## 7. CONSTITUTIONAL IDENTITY (FROZEN)
- identity_hash binds: constitutional_hash, all protocol versions, chain_id, empty_root, max_depth, max_chunk_size
- PROTOCOL_CHAIN_ID: 32-byte aligned constant

## 8. TOPOLOGY INVARIANTS (FROZEN)
- child_depth == parent_depth + 1 for all edges
- Same NodeHash must have exactly one constitutional depth
- No orphan nodes, no dangling edges, no disconnected subgraphs

## 9. TEMPORAL ATOMICITY (FROZEN)
- Every snapshot belongs to a single constitutional instant
- snapshot_cutoff_sequence + snapshot_cutoff_root bind to exact WAL boundary
- Atomic freeze: no in-flight writes during snapshot generation

## 10. REPLAY CONTINUITY (FROZEN)
- THEOREM 11: snapshot + WAL replay == full replay
- ReplayVerifier checks every frame's state_root against reconstructed state
- Epoch/generation monotonicity enforced

---

## AMENDMENT PROCESS
These invariants are FROZEN for protocol version 1.
Changes require:
1. Constitutional amendment proposal
2. New PROOF_VERSION constant
3. Explicit migration path
4. Constitutional court ratification

## SIGNATORIES
Ratified by the AmunChain Constitutional Assembly
Phase 84 - Constitutionally Complete
Date: Constitutional Era 1, Epoch 0

## 11. HASH STABILITY LAW (FROZEN)
For protocol version 1:
- Identical logical state MUST always produce identical:
  - node hashes
  - chunk hashes
  - manifest hashes
  - constitutional identity hashes
- Hash outputs MUST be independent of:
  - platform (OS, CPU architecture)
  - memory layout
  - insertion order
  - iteration nondeterminism
- All hashing uses frozen PROTOCOL_DOMAIN_* constants
- Domain separators are immutable for protocol version 1
