# Consensus Error Model v1.0

## Article I: Fault Classification

### Recoverable Faults (Node Continues)
- Timeout expiry (advance round, do not halt)
- Invalid block received (reject, do not propagate)
- Duplicate message (ignore, already processed)
- Short network partition (wait for reconnect)

### Poison Faults (Node Halts Participation)
- Equivocation detected (self or peer)
- Unsafe invariant violation in consensus core
- Constitutional violation in consensus path
- Journal corruption (checksum mismatch, unrecoverable)

### Halt Faults (Chain Halts)
- Quorum failure (insufficient validators)
- Epoch exhaustion (all epochs consumed)
- Constitutional hash mismatch (fork detected)

## Article II: Escalation Path

Timeout (retry) -> Timeout escalation (longer wait) -> Round skip ->
Proposer change -> Epoch boundary (if persistent)

## Article III: Invalid State Recovery

- Invalid block: reject, continue with preferred chain
- Invalid QC: ignore, wait for valid QC
- Stale state: request sync from peers
- Corrupt journal: rebuild from snapshot + replay

## Article IV: Equivocation Response

1. Detect equivocation (Level 0 Evidence)
2. Broadcast evidence to all validators
3. Slash equivocating validator at epoch boundary
4. Remove from active validator set
5. Freeze stake for slashability window

## Article V: Error Boundaries

Consensus errors must NOT:
- Corrupt the Level 0 kernel state
- Bypass the constitutional linter
- Leak non-determinism into execution
- Cause panic in production paths

All errors flow through AmunResult<T> from Level 0.
