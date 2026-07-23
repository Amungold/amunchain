# Networking Hardening Audit

**Status:** Complete  
**Baseline:** Commitment Layer V1 + Consensus Audit V2  
**Scope:** `amun-networking`, `amun-peer-discovery`, `amun-live-cluster/src/runtime/`

## Findings

| Area | Status | Notes |
|------|--------|-------|
| Certificate Gossip | ✅ | Slashing propagation |
| Mempool Gossip | ✅ | Module exists |
| Peer Discovery Module | ✅ | `amun-networking/src/peer_discovery` |
| Risk Scoring Module | ✅ | `amun-networking/src/risk.rs` with `overall_trust_score` |
| Vote Size Limit | ✅ | 1MB per vote |
| Canonical Allocation | ✅ | 64MB limit |
| Peer Scoring | ⚠️ | Not identified during this audit |
| Reputation System | ⚠️ | Not identified during this audit |
| Rate Limiting | ⚠️ | Not identified during this audit |
| Connection Limits | ⚠️ | Not identified during this audit |
| Bandwidth Limits | ⚠️ | Not identified during this audit |
| Eclipse Resistance | ⚠️ | Not identified during this audit |

## Recommendations (P2 Priority)

| ID | Item | Priority |
|----|------|----------|
| NET-1 | Verify/expand peer reputation scoring | P2 |
| NET-2 | Rate limiting for votes/messages | P2 |
| NET-3 | Connection limits per peer | P2 |
| NET-4 | Eclipse/Sybil resistance review | P3 |
| NET-5 | Per-peer bandwidth throttling | P3 |
