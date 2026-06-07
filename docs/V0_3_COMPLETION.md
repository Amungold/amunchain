# AmunChain v0.3 – Constitutional Authority Layer – Complete

**Date:** 2026-05-31

## Summary
v0.3 establishes the constitutional authority layer on top of the BFT consensus core.
The system now determines truth through cryptographic certificates, validates
constitutional membership, rejects Byzantine attacks, and supports constitutional
amendments, epoch transitions, and authority recovery.

## Layers Completed
| Layer | Capability | Status |
|-------|-----------|--------|
| 1. BFT Consensus | Proposal → Prevote → Precommit → Commit | ✅ |
| 2. Network Partition | 20|20 safety, 27|13 liveness, recovery | ✅ |
| 3. Constitutional Evidence | QC verification, validator set binding | ✅ |
| 4. Byzantine Immunity | Impersonation, conflicting QC, stale/foreign certificates | ✅ |
| 5. Cryptographic Authority | BLS signatures, aggregated QC, key registry | ✅ |
| 6. Constitutional Governance | Validator set updates, epoch transitions, authority recovery | ✅ |

## Key Architectural Principle
**Authority = Cryptographic Proof + Validator Set Binding + Epoch Context + Constitutional Legitimacy**

## Next Phase: v0.4 – Formal Constitutional Theory
- Formal CCS model
- Mathematical authority ordering
- Mechanized safety proofs
