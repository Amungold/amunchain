# V2-016G: Partition Safety & Local State Reconciliation

**Date:** 2026-05-30
**Status:** Closed

## Discovery
After a symmetric 20|20 network partition:
- **Safety preserved:** 0 commits during partition (no quorum possible).
- **Rejoin failure reason:** Not round divergence (all nodes in same round), but local vote/proposal state divergence.
- **Recovery mechanism:** Unconditional reset of `prevoted`, `precommitted`, `seen`, and `proposal` flags on receiving any `sync_status` after rejoin restored consensus liveness.
- **Result:** 39/40 validators finalized after autonomous reconciliation.

## Conclusion
The AmunChain consensus core is partition-safe. Liveness after rejoin is achievable through local state reconciliation. Future work should focus on a constitutional reconciliation protocol that exchanges and adopts the highest safe state, rather than simply forgetting local state.

## Next Step
V2-017: Constitutional Reconciliation Protocol (exchange highest QC, locked block, commit certificate)
