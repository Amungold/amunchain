# AMUNCHAIN REPLAY LAW v1.0

### I. WAL AS SOLE AUTHORITY
The WAL is the sole authoritative record of state transitions.
Any state root not derivable from a valid WAL is constitutionally invalid.

### II. REPLAY EQUIVALENCE
For any valid WAL W: replay(W) == original_execution(W).
Divergence is classified QuarantineRequired or higher.

### III. VERIFICATION
- Full replay: reconstruct complete state, verify every frame
- Partial replay: snapshot + remaining WAL == full replay
- Violations: epoch regression, generation regression, root mismatch,
  hash chain break, sequence gap

### IV. LINEAGE
Every replayed state traceable to genesis through unbroken root chain.
Forks resolved by: longest valid chain, highest generation, external consensus.

### V. RECOVERY
- Crash: scan WAL, find last valid entry, replay, verify manifest
- Corruption: quarantine corrupted frames, replay to last valid, request missing from peers
