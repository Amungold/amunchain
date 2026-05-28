# AMUNCHAIN CONSTITUTIONAL DEBT REGISTER v1.0

### DEBT-001: Reachability GC (Phase 85) [CRITICAL]
Immutable OrdMap accumulates nodes indefinitely. GC must preserve canonical
determinism: nodes reachable from active lineage retained, abandoned forks collected.

### DEBT-002: Physical Compression (Phase 85) [HIGH]
Full 256-depth creates ~256 branches per leaf. Physical compression below
canonical layer required. Must not alter roots, proofs, or replay.

### DEBT-003: Snapshot Format Constitution (Phase 84) [HIGH]
Chunk ordering, hashing, proof-carrying snapshots, deterministic serialization.

### DEBT-004: Concurrency Model (Phase 84) [MEDIUM]
Locking strategy, read/write isolation, snapshot consistency for sync/replay/GC.

### DEBT-005: Fork-Aware Lineage (Phase 84) [MEDIUM]
Competing branches, rollback semantics, finalized root tracking.

### DEBT-006: Snapshot Vacuuming (Phase 85) [MEDIUM]
Pruning old WAL/state after snapshots. Balance storage vs replay/evidence retention.

### DEBT-007: Anti-Corruption Recovery (Phase 85) [LOW]
Automated bit-rot detection, filesystem error recovery, corruption classification.
