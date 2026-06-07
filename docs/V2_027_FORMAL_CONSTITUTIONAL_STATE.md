# V2-027: Formal Constitutional State Transfer

**Date:** 2026-05-31
**Status:** Closed - Final

## Summary
The constitutional reconciliation protocol now operates on full certified evidence:
Commit QC + Lock QC + Height + Round + Validator Set Hash.

## Results
- **27|13 partition:** 27 majority committed, 0 minority committed.
- **Formal evidence transfer:** All 40 nodes converged on height=1.
- **Adversarial evidence:** Fake evidence with invalid validator set was rejected.

## Constitutional Authority Levels
| Level | Evidence | Authority |
|-------|----------|-----------|
| 4 | Commit QC present | Highest |
| 3 | Lock QC present | High |
| 2 | Round > 0 | Medium |
| 1 | Height > 0 | Low |
| 0 | No evidence | None |

## Conclusion
The AmunChain constitutional layer now decides truth based on cryptographic
certificates, not mere height claims. This completes the transition from
"state synchronization" to "constitutional state arbitration."
