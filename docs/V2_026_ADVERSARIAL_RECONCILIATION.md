# V2-026: Adversarial Reconciliation – Constitutional State Arbitration

**Date:** 2026-05-31
**Status:** Closed

## Summary
AmunChain's constitutional state transfer protocol successfully rejects adversarial
states (fake QCs, old QCs, invalid QCs with missing certificates) and only adopts
legitimate certified states from the majority.

## Results
| Adversarial State | Result |
|-------------------|--------|
| Fake QC (height=0 with commit_qc) | REJECTED (verify failed) |
| Old QC (height=0 with commit_qc) | REJECTED (verify failed) |
| Invalid QC (height=5, no commit_qc) | REJECTED (verify failed) |
| Legitimate state (height=1 with commit_qc) | ADOPTED |
| Final convergence | 40/40 validators at height=1 |

## Constitutional Rules Validated
1. A state without a commit certificate is invalid and rejected.
2. A state with commit certificate but height=0 is invalid and rejected.
3. A state with higher height but no commit certificate is rejected (height alone is not authority).
4. Only a state with valid commit certificate and height > current is adopted.
5. Equal or lower states are never adopted (prevents downgrade).

## Conclusion
The constitutional arbitration layer correctly distinguishes between legitimate
and adversarial states. Certificate-based authority (commit_qc) has been
established as the constitutional source of truth, overriding mere height claims.
