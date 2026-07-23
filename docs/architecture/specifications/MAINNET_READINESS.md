# AmunChain Mainnet Readiness

**Status:** P0 Complete — P1 Pending

---

# Completed Milestones

| Item | Status | Reference |
|------|--------|-----------|
| ADR-023 Runtime Foundation | ✅ Complete | Runtime baseline established |
| ADR-024 History Commitment | ✅ Complete | History root commitment verified |
| ADR-025 Parent Hash | ✅ Complete | Parent hash commitment verified |
| ADR-026 Transactions Root | ✅ Complete | Transactions commitment verified |
| ADR-027 Receipts Root | ✅ Complete | Receipts commitment verified |
| ADR-028 Protocol Specification | ✅ Complete | Normative protocol specification |
| P0.1 Canonical Codec Migration | ✅ Complete | Canonical encoding adopted |
| P0.2 Protocol Invariant Tests | ✅ Complete | I1–I8 verified |

---

# Current Conformance Status

- Canonical codec implemented for protocol commitments
- Zero `serde_json::to_vec` usage in protocol paths
- Deterministic transaction hashing
- Deterministic receipt hashing
- Deterministic block commitments
- Protocol invariants I1–I8 verified
- Workspace builds successfully
- Clippy clean
- All tests passing

---

# Remaining Roadmap

| Priority | Item | Status |
|----------|------|--------|
| P1 | Security Review | Pending |
| P2 | Add evidence_root to Block Header | Pending |
| P3 | Add validator_set_root | Pending |
| P3 | Add governance_root | Pending |
| P3 | Final Mainnet Audit | Pending |

---

# P1 Security Review

## 1. Cryptography

Acceptance Criteria:

- Signature verification audited
- Domain separation verified
- Replay protection verified
- Hash prefixes documented
- Key handling reviewed

Deliverable:

- SECURITY_CRYPTO.md

---

## 2. Serialization

Acceptance Criteria:

- Canonical codec used in every protocol path
- Golden vectors verified
- Round-trip tests verified
- Allocation limits enforced
- Malformed input rejected

Deliverable:

- SECURITY_SERIALIZATION.md

---

## 3. Consensus

Acceptance Criteria:

- Double-vote protection verified
- Equivocation handling verified
- Fork safety reviewed
- QC validation verified
- Finality invariants verified

Deliverable:

- SECURITY_CONSENSUS.md

---

## 4. Networking

Acceptance Criteria:

- Message validation verified
- Peer reputation reviewed
- Resource limits verified
- DoS resistance reviewed

Deliverable:

- SECURITY_NETWORK.md

---

## 5. Storage

Acceptance Criteria:

- Snapshot integrity verified
- WAL recovery verified
- Crash recovery verified
- Historical proofs verified

Deliverable:

- SECURITY_STORAGE.md

---

## 6. ADR Traceability

| ADR | Verification |
|-----|--------------|
| ADR-024 | History commitment tests |
| ADR-025 | Parent hash tests |
| ADR-026 | Transaction root tests |
| ADR-027 | Receipt root tests |
| ADR-028 | Protocol invariant tests |

---

# P1 Deliverables

- SECURITY_REVIEW.md
- SECURITY_CRYPTO.md
- SECURITY_SERIALIZATION.md
- SECURITY_CONSENSUS.md
- SECURITY_NETWORK.md
- SECURITY_STORAGE.md
- SECURITY_FINDINGS.md
- SECURITY_DECISIONS.md
- SECURITY_SIGNOFF.md

---

# Exit Criteria for P1

P1 is considered complete only when:

- All security review documents are completed.
- All critical findings are resolved.
- No unresolved High severity issues remain.
- Security checklist reaches 100%.
- Workspace passes:
  - cargo fmt
  - cargo check --workspace
  - cargo clippy --workspace --tests -- -D warnings
  - cargo test --workspace

---

Status after P0:

Commitment Layer V1 ............. COMPLETE
Canonical Codec ................ COMPLETE
Protocol Conformance ............ COMPLETE
Security Review ................ NEXT
