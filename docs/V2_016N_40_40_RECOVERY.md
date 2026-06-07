# V2-016N: Complete 40/40 Partition Recovery

**Date:** 2026-05-30
**Status:** Closed - Final

## Summary
After a 20|20 symmetric network partition, AmunChain achieves full 40/40 validator
commitment after healing. The key insight: a periodic "commit_proof" broadcast
from committed nodes ensures lagging validators learn about the finalized state
even if they missed the initial sync_status exchange.

## Key Results
- **Partition safety:** 0 commits during 20|20 split (no quorum).
- **Rejoin without protocol:** 0 commits (step divergence blocks progress).
- **With sync_status (unconditional reset):** 39/40 commits (one lagging validator).
- **With periodic commit_proof:** 40/40 commits (lagging validator catches up).

## Final Architecture
The complete partition recovery protocol now consists of:
1. **sync_status broadcast:** Exchange round/step/commit state among all validators.
2. **Unconditional vote-state reset:** Clear local prevote/precommit/proposal flags.
3. **Periodic commit_proof:** Committed validators periodically announce their
   finalized state so lagging nodes can adopt the commit directly.

## Conclusion
AmunChain's consensus core is partition-safe and achieves full liveness recovery
after network healing through autonomous constitutional reconciliation.
