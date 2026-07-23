# Consensus Audit V2

**Status:** Complete  
**Baseline:** Commitment Layer V1 + State Commitment V2

## Safety

| Property | Mechanism | Status |
|----------|-----------|--------|
| Quorum Certificate | try_form_qc with >2/3 weighted voting | ✅ |
| Finality | FinalityCertificate chain via history_root | ✅ |
| Fork Safety | finality_chain + history_root chaining | ✅ |
| Double-Vote | EquivocationProof detection + slashing | ✅ |

## Liveness

| Property | Mechanism | Status |
|----------|-----------|--------|
| Proposer Rotation | Pacemaker with timeout-based round advance | ✅ |
| View Change | Pacemaker advances rounds on timeout | ✅ |
| Catch-up | SyncRuntime with needs_catchup flag | ✅ |

## Byzantine Resilience

| Scenario | Defense | Status |
|----------|---------|--------|
| Equivocation | DoubleVoteEvidence → slashing | ✅ |
| Invalid Signatures | Ed25519 verification on all votes | ✅ |
| Suspended Validators | ValidatorStatusRegistry | ✅ |
| Vote Binding | N109.9 enforcement | ✅ |

## Recommendations

1. Formal TLA+ model of safety/liveness for Mainnet readiness
2. Long-running Byzantine testnet with adversarial validators
3. Fuzz testing of consensus message handling
