# AMUNCHAIN CONSTITUTIONAL VALIDITY HIERARCHY v1.0
## Formal taxonomy of state validity and failure modes

---

## ARTICLE I: VALIDITY LEVELS

| Level | Name | Description |
|-------|------|-------------|
| 1 | CryptographicallyValid | Hash correctness verified |
| 2 | StateValid | Deterministic replay verified |
| 3 | ConstitutionallyValid | Epoch/generation/lineage compliance |
| 4 | SemanticallyValid | Interpretation consistency |
| 5 | SovereignlyValid | Civilization continuity |

State SHALL be accepted only at or above the required level for the operation.
Consensus operations require ConstitutionallyValid or higher.

---

## ARTICLE II: FAILURE TAXONOMY

### Level 1: Healable (Local Recovery Possible)
| Failure | Detection | Recovery |
|---------|-----------|----------|
| MissingValue | ValueStore lookup fails | Request from peers |
| StaleCache | LRU returns outdated entry | Invalidate and reload |
| TempFileLeftover | *.tmp files from crashed writes | Remove and retry |

### Level 2: QuarantineRequired (Isolate and Resync)
| Failure | Detection | Recovery |
|---------|-----------|----------|
| StateRootMismatch | ReplayVerifier divergence | Quarantine state, resync from trusted peer |
| WalChainBreak | Entry hash verification fails | Quarantine WAL, request missing frames |
| WalSequenceGap | Sequence non-monotonic | Quarantine WAL, resync |
| NonCanonicalStructure | Canonical invariant check fails | Reconstruct from lineage |
| SnapshotEpochMismatch | Snapshot root != lineage root | Reject snapshot, request valid one |
| IncompleteSnapshot | Missing required nodes | Request missing chunks |

### Level 3: FullReconstructionRequired (Rebuild from Genesis)
| Failure | Detection | Recovery |
|---------|-----------|----------|
| ManifestChainBreak | Manifest prev_hash mismatch | Full state reconstruction from genesis |
| EpochSealInvalid | Epoch seal verification fails | Rebuild lineage from WAL |
| ReplayDivergence | Replay produces different root | Investigate, rebuild from trusted source |
| NodeHashMismatch | Node content != hash expectation | Full reconstruction |

### Level 4: ByzantineEvidence (Malicious Behavior Detected)
| Failure | Detection | Recovery |
|---------|-----------|----------|
| EquivocationDetected | Two different states at same sequence | Slash validator, broadcast evidence |
| UnauthorizedStateInjection | State from unauthorized source | Reject state, ban peer, report |
| SealedEpochMutation | Attempt to modify sealed epoch | Reject, slash, permanent ban |
| SignedButUnavailable | Availability cert without data | Slash 100%, jail forever |

### Level 5: ConstitutionalCrisis (System Halt Required)
| Failure | Detection | Recovery |
|---------|-----------|----------|
| HashCollision | Two different nodes produce same NodeHash | HALT SYSTEM, human intervention required |
| ExcessiveCompression | Compression exceeds MAX_COMPRESSED_SKIP | HALT, constitutional review |
| WalFrameCorrupted | Frame fails all validation | HALT if no clean peer available |

---

## ARTICLE III: PEER TRUST DEGRADATION

| Violation | Trust Penalty |
|-----------|---------------|
| Provided Invalid Proof | Downgrade to Unknown |
| Stale Data (> staleness threshold) | Downgrade to Unknown |
| Inconsistent State | Downgrade to Unknown |
| Failed Challenge (> 3 failures) | Downgrade to Unknown |
| Equivocation Evidence | Permanent Ban + Slash |
| Unauthorized Injection | Permanent Ban + Slash |

---

## ARTICLE IV: REPLAY FALLBACK POLICY

| Scenario | Action |
|----------|--------|
| Local WAL intact | Replay locally |
| Local WAL corrupted | Request WAL from peers, verify, replay |
| No clean peer available | HALT, alert operators |
| Multiple clean peers | Select highest-trust peer first |
| Snapshot available | Use snapshot + remaining WAL |
| Genesis only | Full replay from genesis |

---

## ARTICLE V: QUARANTINE POLICY

| Violation Severity | Quarantine Duration | Release Condition |
|--------------------|---------------------|-------------------|
| Healable | None | Auto-healed |
| QuarantineRequired | Until resync complete | Verified clean state |
| FullReconstruction | Until rebuild complete | Verified root match |
| ByzantineEvidence | Permanent | Never released |
| ConstitutionalCrisis | Until human resolution | Constitutional court ruling |
