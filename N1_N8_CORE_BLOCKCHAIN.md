# N1–N8 — Core Blockchain Stack
## Overview
The foundational layers establishing deterministic state, cryptographic certificates, constitutional blocks, inclusion proofs, and light client verification. These phases form the "Create" and "Verify" domains of the AmunChain trust lifecycle.
## N1 — State Machine
Constitutional state machine backed by a Sparse Merkle Tree. Every state transition produces a deterministic cryptographic state root. The state machine enforces height-sensitive hashing, pre-state sensitivity, journal-sensitive hashing, and overall state sensitivity, ensuring that any mutation in any dimension produces a detectably different state root. **Key Invariants:** Same transition, same input → same state root (determinism). Different transition, different input → different state root (sensitivity). Replay produces identical root (replay consistency). Order independence within same height (canonical ordering).
## N3 — Certificate Merkle
Merkle root computation over collections of ReplayCertificates. Even a single certificate produces a deterministic, non-empty root. Multiple certificates produce different roots than single certificates. This enables compact cryptographic commitment to entire certificate sets within block headers.
## N6 — Constitutional Block
Block structure carrying: block height, parent hash, timestamp, state root, evidence root, and replay certificate root. Full block provenance verification ensures that every block can be validated against its constituent certificates and the resulting state transition.
## N7 — Inclusion Proofs
Merkle proofs demonstrating that a specific ReplayCertificate is included within a certificate Merkle root. These proofs enable light clients to verify that a particular state transition was attested by the network without downloading the full certificate set. **Properties:** Valid proof verifies against correct root. Wrong certificate fails verification. Missing certificate returns no proof. Root mismatch detected. Tampered proof rejected.
## N8 — Light Client
Light client verification using only block headers, a single certificate, and its inclusion proof. Full state download is not required. The verification path: Block Header → Certificate → InclusionProof → Accept. All tampering scenarios (wrong block root, wrong certificate, tampered proof) are detected and rejected.
## Tests
| Component | Tests | Status |
|-----------|-------|--------|
| State Machine (N1) | 22 | ✅ |
| Certificate Merkle (N3) | 10 | ✅ |
| Constitutional Block (N6) | 21 | ✅ |
| Inclusion Proofs (N7) | 10 | ✅ |
| Light Client (N8) | 2 | ✅ |
