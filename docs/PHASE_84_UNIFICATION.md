# Phase 84: CCS Theory ↔ Runtime Unification

**Date:** 2026-05-31
**Status:** Implemented

## Goal
Bridge the gap between CCS theoretical concepts and AmunChain runtime types
by creating a dedicated `amun-constitutional-evidence` crate that embodies
the core constitutional verification function.

## Key Deliverables

### 1. Constitutional Context (`ConstitutionalContext`)
- Unifies `Epoch`, `ValidatorSet`, and `StateRoot` into a single struct.
- Represents the CCS concept `C = (E, V, H)`.

### 2. Constitutional Evidence (`ConstitutionalEvidence`)
- Wraps `QuorumCertificate` as the primary evidence type.
- Represents the CCS concept of evidence `E`.

### 3. Core Verification Function (`is_evidence_valid_for_context`)
- Implements the three CCS rules:
  1. **Epoch Supremacy (Exclusion `⇍_C`)**: Rejects stale evidence.
  2. **Constitutional Membership (Exclusion `⇍_C`)**: Rejects foreign evidence.
  3. **Evidence Strength (Comparability `CC`)**: Ensures quorum, preventing conflicting authorities.
- This single function is the runtime embodiment of CCS derivability (`⊢_C`).

## Gap Closure
This crate closes the semantic gap identified in the constitutional audit:
- **Before:** CCS theory spoke of `Context` and `Evidence`, while the runtime used `Epoch`, `ValidatorSet`, and `QC` independently.
- **After:** The runtime has a single, unified entry point (`is_evidence_valid_for_context`) that directly implements the constitutional rules, making the theory executable.

## Next Steps
1. Integrate this crate into the `amun-networking` test suite.
2. Port existing tests (e.g., V3-006A, V3-006C, V3-006D) to use this new module.
3. Replace ad-hoc evidence checks in `VoteCollector` with calls to this module.
