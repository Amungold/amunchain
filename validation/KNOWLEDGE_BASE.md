# AmunChain Network Knowledge Base V1
Status: ACTIVE
Baseline: 90f4993

## Certified Knowledge Artifacts

### K-001 — Genesis Hash Determinism
- Statement: Genesis initialization is deterministic across all validators.
- Detail: All validators produce identical genesis hash: cd8167b9d20283b9a9241bf1129f73508e97f53fb4b912a77622c93fcfaf5b48
- Source Gate: NV-01A
- Evidence: validation/evidence/NV-01/
- Audit: PENDING
- Certification: PENDING AUDIT
- Status: REPRODUCED
- Date: 2026-06-14

### K-002 — Runtime State Root Determinism (Current Runtime)
- Statement: All validator runtimes produce an identical state_root after propose_block(1).
- Observed Root: 0000000000000000000000000000000000000000000000000000000000000000
- Qualification: Current runtime does not mutate ResourceRegistry (Discovery D-002).
- Source Gate: NV-01B
- Evidence: validation/evidence/NV-01B/
- Audit: PENDING
- Certification: PENDING AUDIT
- Status: REPRODUCED
- Date: 2026-06-14

### K-003 — State Mutation Determinism
- Statement: The ResourceRegistry engine produces deterministic state roots when identical mutations are applied.
- Source Gate: NV-02A
- Status: REPRODUCED

### K-004 — Multi-Height State Evolution Determinism
- Statement: State evolves deterministically across multiple heights with identical mutations.
- Source Gate: NV-02B
- Status: REPRODUCED

### K-005 — Persistence and Restart Determinism
- Statement: State roots remain identical after a full shutdown, save, and restore cycle.
- Source Gate: NV-02C
- Status: REPRODUCED

### K-006 — Replay Determinism
- Statement: Replaying a log of identical mutations on independent stores produces identical final state roots.
- Source Gate: NV-02D
- Status: REPRODUCED

### K-007 — Constitutional Multi-Block Evolution Determinism
- Statement: Constitutional programs driving multi-block state evolution produce identical, evolving state roots across all validators.
- Source Gate: NV-03C
- Status: REPRODUCED

### K-005 — Network State Convergence (Simulated)
- Statement: When consensus output is applied to identical state machines, all validators produce the same final state root.
- Observed Root: `0d82b09dbb6ec85d46c56e6c5e4f636dfdf52e204c4d205ba4fc1c84a40aa12b`
- Source Gate: NV-04B
- Evidence: validation/evidence/NV-04B/
- Status: REPRODUCED
- Date: 2026-06-14

### K-006 — Byzantine Fault Tolerance (Single Validator)
- Statement: Network tolerates Byzantine proposal injection from a single validator while maintaining consensus and state convergence among honest validators.
- Source Gate: NV-04D
- Evidence: test_byzantine_fault
- Status: REPRODUCED
- Date: 2026-06-14

### K-006 — Partition Recovery Convergence
- Statement: Validators that are partitioned and later rejoined successfully converge back to the same chain height.
- Observed Result: Final heights = [219,219,219,219] with zero spread.
- Source Gate: NV-04D
- Evidence: N75 partition recovery test
- Status: REPRODUCED
- Date: 2026-06-14
