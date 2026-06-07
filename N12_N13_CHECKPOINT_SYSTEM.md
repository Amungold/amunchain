# N12–N13 — Checkpoint Architecture
## Overview
The N12–N13 milestone series introduces the constitutional checkpoint system of AmunChain. While previous milestones enabled verification of individual blocks and certificates, the checkpoint architecture enables the compression of large historical segments into cryptographically verifiable checkpoints. This allows nodes to synchronize, bootstrap, recover, and verify long blockchain histories without replaying every block individually. The checkpoint system serves as the constitutional memory layer of the network.
## Architectural Objectives
The checkpoint architecture must provide: (1) Historical compression, (2) Verifiable state continuity, (3) Efficient synchronization, (4) Trustless bootstrap support, (5) Recursive proof composition, (6) Long-term chain preservation, (7) Byzantine-resistant history verification.
## N12A — Checkpoint Certificate
### Purpose
A Checkpoint Certificate represents a cryptographic commitment to a contiguous range of finalized blocks. Instead of verifying each block independently, nodes may verify a checkpoint that summarizes the entire range. A checkpoint commits to start height, end height, state commitment, evidence commitment, certificate commitment, and block range integrity. A checkpoint always represents [start_height, end_height] inclusive. A checkpoint must represent a continuous history. Any gap invalidates the checkpoint.
### Deterministic Hashing
Checkpoint identifiers are generated using AMUN_CHECKPOINT_V1 domain-separated hashing. This guarantees deterministic output, collision isolation, and future versioning compatibility.
### Security Properties
Continuity Enforcement: Block ranges must be continuous. Discontinuities are rejected. Tamper Detection: Modification of height range, state commitment, evidence commitment, or certificate commitment changes the checkpoint hash. Deterministic Reproducibility: Independent nodes always generate identical checkpoint identifiers.
## N12B — Checkpoint Merkle Layer
### Purpose
Multiple checkpoints are aggregated into a Merkle commitment. This creates a compact representation of large historical collections. Input: Checkpoint Certificates. Output: Checkpoint Merkle Root. Checkpoint trees use AMUN_CHECKPOINT_MERKLE_DOMAIN to prevent cross-domain collisions. Identical checkpoint sets always produce identical roots.
### Inclusion Proof System
Each checkpoint can prove membership within the checkpoint tree. An inclusion proof contains the checkpoint hash, sibling hashes, and traversal path. The verifier recomputes the root. Matching root proves inclusion. The system rejects forged checkpoints, altered proofs, root mismatches, and fabricated members.
## N12C — Checkpoint Bundle
### Purpose
A Checkpoint Bundle combines a checkpoint with its inclusion proof. This creates a self-contained verification artifact. A CheckpointBundle contains a Checkpoint Certificate and a Checkpoint Inclusion Proof. Verification Procedure: Checkpoint Verify plus Inclusion Proof Verify yields Bundle Accepted. A bundle contains everything necessary to verify membership. No additional network queries are required.
## N12D — Light Checkpoint Verification
### Purpose
Allow verification of long historical ranges without replaying individual blocks. Required inputs: Trusted Checkpoint Root and Checkpoint Bundle Sequence. Verification Process: Bundle → Checkpoint Verification → Inclusion Verification → Root Verification → Acceptance. Trust is rooted in Trusted Root rather than Historical Replay. Thousands of blocks can be represented by a small number of checkpoint bundles.
## N13 — Recursive Checkpoint Chains
### Purpose
Create proof-carrying historical compression across arbitrarily long chains. As blockchain history grows, block verification cost increases, storage cost increases, and synchronization cost increases. Recursive checkpoints solve this problem.
### Recursive Model
Individual Blocks → Checkpoints → Checkpoint Merkle Tree → Recursive Chain. A RecursiveCheckpointProof represents multiple checkpoints under a shared trusted root with verifiable continuity. Every checkpoint must connect to the next: previous.end_height + 1 must equal next.start_height. Any violation invalidates the chain.
### Compression Benefits
Instead of verifying 100,000 blocks, nodes may verify 100 checkpoints or fewer. The recursive checkpoint chain acts as the constitutional memory of the network. It preserves finalized history, state continuity, and verification lineage without requiring complete historical replay.
### Verification Paths
Checkpoint Verification: Checkpoint → Hash Verification → Acceptance. Inclusion Verification: Checkpoint → Inclusion Proof → Merkle Root → Acceptance. Bundle Verification: Checkpoint plus Proof yields Bundle. Recursive Verification: Checkpoint Chain → Continuity Validation → Root Validation → Acceptance.
### Byzantine Resistance
The checkpoint architecture rejects forged checkpoints, invalid inclusion proofs, discontinuous histories, root mismatches, replayed foreign checkpoints, and tampered checkpoint chains.
### Synchronization Advantages
Compared to full historical replay, checkpoint verification provides lower CPU usage, lower storage requirements, lower synchronization latency, and faster recovery.
### Long-Term Network Role
The checkpoint system becomes the foundation for Bootstrap Sessions, Constitutional Join, Crash Recovery, Rejoin Protocols, and Long-Term Archival Verification. All future synchronization mechanisms depend upon checkpoint verification.
## Test Coverage
| Component | Tests |
|-----------|-------|
| Checkpoint Certificate | 7 |
| Checkpoint Merkle Layer | 5 |
| Checkpoint Bundle | 1 |
| Light Checkpoint Verification | 2 |
| Recursive Checkpoint Chain | 4 |
Total Verified Coverage: 19 Tests. All passing.
## Milestone Result
Status: COMPLETE. Outcome: A constitutional checkpoint architecture capable of compressing large blockchain histories into cryptographically verifiable checkpoint chains while preserving continuity, auditability, and trustless verification. The checkpoint architecture becomes the permanent historical backbone of AmunChain and enables efficient bootstrap, synchronization, recovery, and long-term chain preservation.
