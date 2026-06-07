# AmunChain v0.1 — Constitutional Kernel
## Abstract
AmunChain is a constitutional blockchain architecture where trust is established through deterministic replay verification, cryptographic proofs, and a structured node lifecycle. Unlike traditional blockchains that prioritize consensus alone, AmunChain builds a complete trust lifecycle spanning seven domains: Create, Verify, Distribute, Bootstrap, Rejoin, Defend, and Identify. The system comprises 21 architectural phases (N1–N21), 16 formal audit layers, over 400 automated tests, and a real TCP networking stack with Ed25519 cryptographic identity verification.
## Architecture Overview
### Phase Map
| Phase | Component | Tests | Status |
|-------|-----------|-------|--------|
| N1 | State Machine | 22 | ✅ |
| N3 | Certificate Merkle | 10 | ✅ |
| N6 | Constitutional Block | 21 | ✅ |
| N7 | Inclusion Proofs | 10 | ✅ |
| N8 | Light Client | 2 | ✅ |
| N9 | Certificate Distribution | 10 | ✅ |
| N10 | Certificate Gossip | 11 | ✅ |
| N11 | Stateless Sync | 8 | ✅ |
| N12 | Checkpoint System | 15 | ✅ |
| N13 | Recursive Proof Chain | 4 | ✅ |
| N14 | Bootstrap Session | 4 | ✅ |
| N15 | Constitutional Join | 2 | ✅ |
| N16 | Adversarial Bootstrap | 7 | ✅ |
| N17 | Multi-Node Network | 5 | ✅ |
| N18 | Constitutional Rejoin | 18 | ✅ |
| N19 | Adversarial Rejoin | 5 | ✅ |
| N20 | Real Network Transport | 30 | ✅ |
| N21 | Constitutional Identity | 11 | ✅ |
### Verification Paths
1. **Full Node**: Block → Certificate → Journal → Replay → StateRoot
2. **Light Client**: Header → Certificate → InclusionProof → Accept
3. **Stateless Sync**: Headers → Certificates → Bundles → VerifyChain → Trust
4. **Checkpoint**: Block → Certificate → Checkpoint → Inclusion → Chain
5. **Bootstrap**: Root → Bundles → Verify → Join Network
6. **Network Consensus**: 4/7 nodes → Propose → Prevote → Precommit → Commit
7. **Constitutional Rejoin**: Bootstrapping → CatchingUp → Verifying → Active
8. **Adversarial Defense**: Forged/Wrong/Gap/Mixed/Byzantine → All Rejected
### Trust Lifecycle
| Domain | Phases | Description |
|--------|--------|-------------|
| Create | N1–N8 | State, Certificates, Blocks, Inclusion Proofs |
| Verify | N7, N12 | Inclusion Proofs, Checkpoint Verification |
| Distribute | N9–N11 | Certificate Distribution, Gossip, Stateless Sync |
| Bootstrap | N14–N16 | Bootstrap Session, Constitutional Join, Adversarial Defense |
| Rejoin | N18 | Node Lifecycle, Catch-up, Verification, Activation |
| Defend | N16, N19 | Adversarial Bootstrap, Adversarial Rejoin |
| Identify | N21 | Validator Certificates, Trust Anchors, Validator Registry |
## Formal Audit Suite
16 architectural audit layers providing 52 tests across: Physics (domain separation uniqueness, hash determinism, endian consistency, empty root constancy, serialization stability), Geometry (proof depth invariants, ladder terminal emptiness, insertion-deletion cycles, frozen maximum depth), Snapshot (magic bytes verification, manifest self-verification, identity determinism, self-compatibility), Byzantine (quorum detection, identity mismatch rejection), Identity (self-verification, different hash different identity, encode-decode roundtrip, tampered identity detection), Replay (equivalence verification, divergence detection, epoch regression rejection), Resources (allocation guard enforcement, frozen chunk size), Domains (all domains unique, chain ID 32 bytes, domains versioned), Freeze (maximum depth is 256, proof version is v1, empty root stability), Adversarial (malformed proof rejection, delete nonexistent no-op, insert-delete-insert cycle, random order independence), Crash (truncated WAL detection, partial frame rejection, mid-frame corruption detection), Fuzzing (proof decode random bytes, absence proof consistency, random key insertions), Differential (canonical encoding determinism, domain hash determinism, empty root consistency), Byzantine Mesh (conflicting manifests detected, foreign civilization rejection), Temporal (replay twice same root, temporal order independence), Mutation (maximum depth frozen, proof version frozen, empty root not mutable, endian invariant).
## Engineering Gates
| Gate | Status |
|------|--------|
| cargo build | ✅ |
| cargo build --release | ✅ |
| cargo test (workspace) | ✅ |
| cargo clippy | ✅ |
| cargo clippy -- -D warnings | ✅ |
## Network Stack
| Layer | Component | Status |
|-------|-----------|--------|
| Transport | TCP with length-prefixed framing | ✅ |
| Identity | Ed25519 PeerKeyPair generation and verification | ✅ |
| Messaging | Signed envelopes with directed peer routing | ✅ |
| Discovery | Peer announcements with expiration registry | ✅ |
| Bootstrap | Checkpoint sync and verification over TCP | ✅ |
| Rejoin | Crash recovery with full lifecycle enforcement | ✅ |
| Testnet | Multi-machine bind, connect, and identity exchange | ✅ |
## Constitutional Identity
| Component | Description | Status |
|-----------|-------------|--------|
| ValidatorCertificate | Authority-signed certificate binding ID to key | ✅ |
| TrustAnchorRegistry | Root of cryptographic trust in the network | ✅ |
| ValidatorRegistry | Active validators with verified certificates | ✅ |
**Identity Chain**: PeerKeyPair → ValidatorCertificate → TrustAnchor → ValidatorRegistry. A validator must prove: (1) I own a key, (2) A constitutional authority recognizes me, (3) My certificate is valid and unexpired.
## Version
AmunChain v0.1 — Constitutional Kernel. N1 → N21 Complete. All gates passing. Ready for whitepaper publication and public testnet deployment.
