# Phase 6 — Document Index
## Document Control
| Field | Value |
|-------|-------|
| Document ID | AMUN-PHASE6-INDEX-001 |
| Revision | 1.0.0 |
| Status | Frozen |
| Author | Engineering Team |
| Date | 2026-05-17 |
## Index of Phase 6 Documents
PHASE6_NETWORK_LAYER_AND_BYZANTINE_HARDENING.md: Full technical specification. PHASE6_SUMMARY.md: Executive summary. PHASE6_INDEX.md: This file. PHASE6_PRODUCTION_PLAN.md: Five-step production hardening plan. PHASE6_PRODUCTION_CHECKLIST.md: Step-by-step readiness checklist. PHASE6_ARCHITECTURE_DIAGRAM.md: System architecture and data flow diagrams.
## Phase 6 Quick Reference
New crates: amun-bls, amun-network, amun-gossip. Total crates: 19. Total tests: 218. Test pass rate: 100 percent. Build errors: 0. BLS constants: Secret key 32 bytes, Public key 48 bytes, Signature 96 bytes, Max signers 256. Network constants: Max peers 256, Max connections per IP 8, Handshake timeout 5s, Heartbeat interval 10s, Stale timeout 60s, Max message size 64KB, Rate limit 100/sec. Gossip constants: Fanout 6, Max rounds 3, Timeout 2s, Dedup cache 10000.
## Source Module Reference (New)
amun-bls/keygen.rs: Key generation. amun-bls/sign.rs: Message signing. amun-bls/verify.rs: Signature verification. amun-bls/aggregate.rs: Signature aggregation. amun-bls/constants.rs: Curve constants. amun-network/peer.rs: Peer identity. amun-network/discovery.rs: Peer discovery. amun-network/connection.rs: Connection management. amun-network/framing.rs: Message framing. amun-network/heartbeat.rs: Health monitoring. amun-network/rate_limit.rs: DoS protection. amun-network/handshake.rs: Protocol negotiation. amun-gossip/broadcaster.rs: Message broadcast. amun-gossip/receiver.rs: Message reception. amun-gossip/dedup.rs: Duplicate detection. amun-gossip/fanout.rs: Adaptive fan-out. amun-gossip/retry.rs: Retry logic. amun-gossip/topics.rs: Topic routing.
## Build and Test Commands
cargo build --workspace. cargo test --workspace. cargo clippy --workspace. cargo fmt --check --workspace.
## Amendment Log
2026-05-17: Revision 1.0.0, Initial freeze, Engineering Team.
End of index. Status: FROZEN. Revision: 1.0.0. Date: 2026-05-17.
