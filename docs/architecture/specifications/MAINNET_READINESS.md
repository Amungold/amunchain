# AmunChain Mainnet Readiness

**Status:** Pre-Mainnet Review Complete  
**Date:** Post ADR-029 era

## Gate Status

| Gate | Requirement | Status |
|------|-------------|--------|
| G1 | Protocol Specification (ADR-028) | ✅ |
| G2 | Determinism (canonical codec) | ✅ |
| G3 | Correctness (8/8 invariants) | ✅ |
| G4 | Static Security Audit | ✅ |
| G5 | Dynamic Security Tests | ✅ |
| G6 | Property & Robustness Tests | ✅ |
| G7 | Final Security Signoff | ✅ |

## Audit Coverage

| Audit | Status |
|-------|--------|
| Commitment Layer V1 | ✅ |
| State Commitment V2 (ADR-029) | ✅ |
| Consensus Audit V2 | ✅ |
| Networking Hardening | ✅ |
| Storage/WAL Audit | ✅ |
| Performance Benchmark | ✅ |

## Pre-Mainnet Checklist

| Item | Status | Notes |
|------|--------|-------|
| Protocol spec | ✅ | ADR-028 normative |
| Block commitments | ✅ | 7 roots in header |
| Canonical codec | ✅ | Zero serde_json in protocol |
| SMT for state | ✅ | amun-smt with proofs |
| Consensus safety | ✅ | QC, finality, double-vote |
| Networking gossip | ✅ | Certificate + mempool |
| Storage recovery | ✅ | WAL + snapshot verify |
| Benchmark framework | ✅ | Criterion with targets |
| Workspace: check | ✅ | Clean |
| Workspace: clippy | ✅ | Zero warnings |
| Workspace: test | ✅ | All passing |
| Documentation | ✅ | 15+ audit docs |

## Deferred to P2/P3

| Item | Priority |
|------|----------|
| Peer reputation scoring | P2 |
| Rate limiting | P2 |
| Per-record checksum | P2 |
| Explicit fsync audit | P2 |
| Formal TLA+ model | P3 |
| libFuzzer integration | P3 |
| Eclipse/Sybil resistance | P3 |

## Decision

**Pre-Mainnet Review: PASSED**

All critical gates (G1-G7) cleared. Workspace clean.
Deferred items documented with priority and ownership.
Ready for testnet deployment and external security audit.
