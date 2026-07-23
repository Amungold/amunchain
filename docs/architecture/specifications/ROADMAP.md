# AmunChain Roadmap

**Current Milestone:** Commitment Layer V1 Complete  
**Next Milestone:** P1 Security Hardening

---

## Completed ✅

| Item | Era |
|------|-----|
| ADR-023 Runtime Foundation | ADR-023 |
| ADR-024 History Commitment | ADR-024 |
| ADR-025 Parent Hash Semantics | ADR-025 |
| ADR-026 Transactions Root | ADR-026 |
| ADR-027 Receipts Root | ADR-027 |
| ADR-028 Protocol Specification v1.0 | ADR-028 |
| P0.1 Canonical Codec Migration | P0 |
| P0.2 Protocol Invariant Tests I1-I8 | P0 |
| P1.1 Static Security Audit | P1 |

---

## In Progress ⏳

| Priority | Item | Phase |
|----------|------|-------|
| P1.2 | Dynamic Security Tests | Security |
| P1.3 | Fuzz Testing | Security |
| P1.4 | Final Security Signoff | Security |

---

## Planned 📋

| Priority | Item | Dependencies |
|----------|------|--------------|
| P2 | `evidence_root` in Block Header | P1 complete |
| P3 | `validator_set_root` | P2 complete |
| P3 | `governance_root` | P2 complete |
| P3 | Mainnet Final Audit | P1, P2, P3 complete |

---

## Gates

| Gate | Requirement | Status |
|------|-------------|--------|
| G1: Specification | ADR-028 normative | ✅ |
| G2: Determinism | Canonical codec, no serde_json in protocol | ✅ |
| G3: Correctness | 8/8 invariants verified | ✅ |
| G4: Static Security | 5-layer audit complete | ✅ |
| G5: Dynamic Security | Malformed input, stress, edge cases | ⏳ |
| G6: Fuzz Coverage | Canonical, network, crypto fuzzing | ⏳ |
| G7: Final Signoff | External audit + signoff | ⏳ |
