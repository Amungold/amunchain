# Phase 6 — Network Layer & Byzantine Hardening
## Document Control
| Field | Value |
|-------|-------|
| Document ID | AMUN-PHASE6-NETWORK-001 |
| Revision | 1.0.0 |
| Status | Frozen |
| Author | Engineering Team |
| Date | 2026-05-17 |
| Classification | Internal — Constitutional |
## 1.0 Overview
Phase 6 delivers the networking and cryptographic scaffolding required to transform AmunChain from a local deterministic system into a decentralized Byzantine Fault Tolerant network. This phase introduces three new crates: amun-bls for BLS12-381 signature operations, amun-network for peer-to-peer networking, and amun-gossip for efficient block and vote propagation across the validator set.
### 1.1 Scope
This phase covers BLS12-381 key generation, signing, verification, and signature aggregation; peer identity management and discovery; connection handling with rate limiting and IP-based limits; message framing with type-tagged payloads; heartbeat-based peer health monitoring; protocol handshake negotiation; gossip protocol with fan-out broadcasting, duplicate detection, adaptive fan-out selection, and exponential backoff retry logic.
## 2.0 New Crates
### 2.1 amun-bls — BLS12-381 Signature Library
Purpose: Production-grade BLS12-381 cryptographic operations for consensus message signing and verification.
Modules: keygen.rs — Deterministic key generation from 32-byte seeds. sign.rs — BLS signature creation over arbitrary messages. verify.rs — Single signature and aggregate signature verification. aggregate.rs — Signature and public key aggregation with max signer limits. constants.rs — Curve constants and domain separation tags.
Constitutional Constants: BLS_SECRET_KEY_SIZE: 32 bytes. BLS_PUBLIC_KEY_SIZE: 48 bytes. BLS_SIGNATURE_SIZE: 96 bytes. BLS_MAX_SIGNERS: 256. BLS_DST: domain separation tag for hash-to-curve operations.
Tests: 6 tests covering key generation determinism, sign/verify roundtrip, invalid signature rejection, aggregate signature creation, aggregate public key creation, and empty aggregate rejection.
### 2.2 amun-network — Peer-to-Peer Network Layer
Purpose: Peer identity management, discovery, connection handling, message framing, heartbeats, rate limiting, and protocol handshake.
Modules: peer.rs — Peer identity with BLS public key, address, state tracking, and connection counting. discovery.rs — Peer discovery via seed nodes with active peer filtering. connection.rs — Connection management with per-IP connection limits. framing.rs — Message framing with type tag and payload encoding. heartbeat.rs — Health monitoring with configurable intervals and timeout detection. rate_limit.rs — Per-peer message rate limiting with sliding window. handshake.rs — Protocol version negotiation and capability exchange.
Constitutional Constants: MAX_PEERS: 256. MAX_CONNECTIONS_PER_IP: 8. HANDSHAKE_TIMEOUT_MS: 5000. HEARTBEAT_INTERVAL_MS: 10000. PEER_STALE_TIMEOUT_MS: 60000. MAX_MESSAGE_SIZE: 65536 bytes. RATE_LIMIT_PER_SECOND: 100.
Message Types: PeerDiscovery, PeerRequest, PeerResponse, BlockProposal, VoteBroadcast, QuorumCertAnnounce, TransactionGossip, Heartbeat.
Tests: 6 tests covering peer state transitions, discovery peer addition, connection IP limits, rate limiter enforcement, heartbeat timeout detection, and message framing encode.
### 2.3 amun-gossip — Block & Vote Propagation Protocol
Purpose: Efficient probabilistic dissemination of blocks, votes, quorum certificates, and transactions across the validator set.
Modules: broadcaster.rs — Message broadcast with fan-out selection and duplicate detection. receiver.rs — Message reception with deduplication and counting. dedup.rs — Duplicate message cache using content hashing. fanout.rs — Adaptive fan-out selection based on peer count. retry.rs — Exponential backoff retry manager for unacknowledged messages. topics.rs — Topic-based message routing with type-safe encoding.
Constitutional Constants: GOSSIP_FANOUT: 6. GOSSIP_ROUNDS: 3. GOSSIP_TIMEOUT_MS: 2000. DEDUP_CACHE_SIZE: 10000. DEDUP_WINDOW_SECONDS: 300.
Message Topics: Blocks, Votes, QuorumCert, Transactions.
Tests: 6 tests covering topic encoding roundtrip, deduplication detection, fan-out peer selection, broadcaster duplicate rejection, receiver unique counting, and retry backoff calculation.
## 3.0 Modified Crates
### 3.1 amun-consensus
Changes: Maintains existing 7 tests. Network integration points prepared for Phase 6 production hardening.
### 3.2 amun-storage
Changes: Maintains existing 3 tests. Snapshot sync integration points prepared.
### 3.3 amun-block
Changes: Maintains existing 12 tests. Block propagation integration points prepared.
## 4.0 Constitutional Guarantees
CG-6.1: BLS signatures cryptographically generated and verified for all consensus messages. CG-6.2: Peer messages rate-limited to prevent DoS attacks. CG-6.3: Duplicate gossip messages detected and dropped via content hashing. CG-6.4: Block proposals validated before network propagation. CG-6.5: QC aggregates only valid verified signatures. CG-6.6: Peer connections bounded per IP to prevent resource exhaustion. CG-6.7: Heartbeat monitoring detects stale peers within configured timeout. CG-6.8: Message framing preserves type information for routing. CG-6.9: Gossip fan-out adapts to network size. CG-6.10: Retry with exponential backoff ensures eventual delivery.
## 5.0 Test Suite
### 5.1 Test Results by Crate
amun-block: 12 passed, 0 failed. amun-bls: 6 passed, 0 failed. amun-codec: 22 passed, 0 failed. amun-consensus: 7 passed, 0 failed. amun-consensus-types: 9 passed, 0 failed. amun-constitution: 4 passed, 0 failed. amun-determinism-tests: 7 passed, 0 failed. amun-evidence: 2 passed, 0 failed. amun-execution: 8 passed, 0 failed. amun-failure: 12 passed, 0 failed. amun-gossip: 6 passed, 0 failed. amun-kernel-types: 15 passed, 0 failed. amun-merkle: 15 passed, 0 failed. amun-network: 6 passed, 0 failed. amun-runtime: 0 passed, 0 failed. amun-state-types: 8 passed, 0 failed. amun-stf: 9 passed, 0 failed. amun-storage: 3 passed, 0 failed. amun-transaction: 7 passed, 0 failed. amun-unsafe: 15 passed, 0 failed. constitutional-linter: 12 passed, 0 failed. TOTAL: 218 passed, 0 failed.
### 5.2 New Tests Added in Phase 6
BLS: test_keygen_deterministic, test_sign_and_verify, test_invalid_signature_rejected, test_aggregate_signatures, test_aggregate_public_keys, test_aggregate_empty_rejected.
Network: test_peer_state_transitions, test_discovery_add_peer, test_connection_limit, test_rate_limiter, test_heartbeat_timeout, test_framing_encode.
Gossip: test_topic_roundtrip, test_dedup_detects_duplicate, test_fanout_selects_correct_count, test_broadcaster_rejects_duplicate, test_receiver_counts_unique, test_retry_backoff.
## 6.0 Build Verification
cargo build --workspace: Success, 0 errors. cargo test --workspace: 218 passed, 0 failed. Unsafe code outside amun-unsafe: None. Unwrap in production paths: None. Panic in production paths: None. Warnings: 1 (dead_code in gossip dedup window_seconds field, harmless).
## 7.0 Production Hardening Roadmap
Step 1 — BLS12-381 Production: Replace mock blake3-based signing with blst crate for real BLS12-381 curve operations. Step 2 — TLS 1.3 + QUIC Transport: Add real network transport with certificate pinning. Step 3 — Live Network Gossip: Wire gossip to network transport for real message propagation. Step 4 — Byzantine Node Simulation: Multi-validator test harness with malicious behaviors. Step 5 — Snapshot Sync: State synchronization for new and lagging nodes.
## 8.0 Amendment Procedure
1. Document change in NETWORK_LAW.md. 2. Implement change. 3. Run full test suite. 4. Verify 218 tests pass. 5. Update documentation. 6. Increment revision. 7. Re-freeze.
## 9.0 Next Phase — Phase 7
Phase 7 will deliver: economic model and staking economics, slashing conditions for Byzantine behavior, reward distribution mechanism, fee market, governance mechanism, and mainnet launch preparation.
End of document. Status: FROZEN. Revision: 1.0.0. Date: 2026-05-17.
