# AmunChain Validation Gates Registry
## Gate Lifecycle
PENDING → RUNNING → PASS → CERTIFIED | PENDING → RUNNING → FAIL | PENDING → BLOCKED

## Gate Status Derivation Rules (Status derived from evidence, never set manually)
PENDING: Evidence directory empty.
RUNNING: genesis.json + genesis.sha256 exist in evidence directory.
PASS: RESULT.md shows Decision: PASS.
FAIL: RESULT.md shows Decision: FAIL + investigation report.
CERTIFIED: CERTIFICATION.md shows Decision: CERTIFIED.
BLOCKED: Authorization gate not yet CERTIFIED.

## Governance Rules: G-01, G-02, G-03, G-04 (as in BASELINE.md)

## NV-01 — Genesis Determinism [CRITICAL, AUTOMATIC, PENDING]
Knowledge Produced: Genesis initialization is deterministic across all validators.
Procedure:
1. Archive genesis.json → validation/evidence/NV-01/
2. Compute genesis.sha256
3. Start 4 validators, stop at Height 0 or 1
4. Extract Genesis Hash, State Root, Validator Set Root using ROOT_SNAPSHOT_FORMAT.md
5. Compare all values across 4 validators
6. Create RESULT.md, MANIFEST.md, MANIFEST.sha256
Expected: All 4 validators produce identical Genesis Hash, State Root, Validator Set Root.
Failure Modes: F1 Genesis Hash mismatch, F2 State Root mismatch, F3 Validator Set Root mismatch, F4 Reproduction failure.
PASS Action → Authorize NV-02 after certification.
FAIL Action → Block all gates. Open Determinism Investigation.

## NV-02 — State Determinism [CRITICAL, AUTOMATIC, BLOCKED]
Knowledge Produced: State transitions are deterministic across independent nodes.
Checkpoints: 10, 100, 1000, 5000, 10000
Expected: At every checkpoint, all 4 validators produce identical State Root, History Root, Finality Root.
Authorization: Requires NV-01 CERTIFIED.

## NV-03 — Fresh Node Join [HIGH, AUTOMATIC, BLOCKED]
Knowledge Produced: A new node can join a running network and reach full participation.
Authorization: Requires NV-02 CERTIFIED.

## NV-04 — Restart Recovery [HIGH, AUTOMATIC, BLOCKED]
Knowledge Produced: All validators can restart from persisted state and resume consensus.
Authorization: Requires NV-03 CERTIFIED.

## NV-05 — State Sync Stress [HIGH, AUTOMATIC, BLOCKED]
Knowledge Produced: State sync is reliable and correct from long chains (10,000+ blocks).
Authorization: Requires NV-04 CERTIFIED.

## NV-05.5 — Chaos Validation [HIGH, AUTOMATIC, BLOCKED]
Knowledge Produced: Network recovers from live validator kills and network partitions.
Authorization: Requires NV-05 CERTIFIED.

## NV-06 — 7-Day Continuous Run [CRITICAL, AUTOMATIC, BLOCKED]
Knowledge Produced: Network sustains continuous 168-hour operation without manual intervention, crashes, or state divergence.
Authorization: Requires NV-05.5 CERTIFIED.

## NV-07 — Network Health Visibility [MEDIUM, MANUAL, BLOCKED]
Knowledge Produced: Network health can be monitored through operational dashboards.
Authorization: Requires NV-06 CERTIFIED.

## NV-08 — RPC Stability [MEDIUM, MANUAL, BLOCKED]
Knowledge Produced: RPC endpoints are stable under sustained load.
Authorization: Requires NV-06 CERTIFIED.
