# AmunChain Threat Model

## Adversary Model

- **Type**: Byzantine adversary
- **Power**: Controls up to f < n/3 validators
- **Capabilities**: Delay, reorder, drop messages before GST
- **Limitations**: Cannot forge signatures, invert hashes, violate determinism

## Attack Surface

| Layer | Attack Vector | Mitigation |
|-------|--------------|------------|
| Execution | Forged TransitionProof | Proof hash verification (Theorem 1) |
| Execution | Illegal transformation | Transformation matrix (T1) |
| Execution | Lineage cycle | Cycle detection (L1) |
| Execution | Version regression | Monotonicity check (R6) |
| State | Duplicate resource ID | Uniqueness check (R1) |
| State | Consumed resource reuse | State check (R2) |
| Consensus | Double voting | Replay-backed QC (C1) |
| Consensus | Insufficient quorum | QC threshold (C2) |
| Network | Replay attack | AntiReplayGuard (K3) |
| Network | Sybil attack | Validator certificate chain |
| Crypto | Key compromise | Key rotation (K4) |
| Crypto | Signature forgery | Ed25519 verification (K2) |

## Trust Assumptions

1. Blake3 provides collision resistance and preimage resistance.
2. Ed25519 provides existential unforgeability.
3. Honest validators constitute > 2/3 of total voting power.
4. Network is partially synchronous with known Δ after GST.
5. Execution is deterministic (Assumption 1).
