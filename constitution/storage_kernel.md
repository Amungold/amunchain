# AmunChain Storage Kernel Constitution

## 1. Canonical SMT Law
No insertion order dependency. Root MUST be deterministic.

## 2. WAL Integrity Law
Every WAL entry is hash-chained and verified during replay.

## 3. Atomic Storage Law
All writes are atomic via temp+rename+fsync.

## 4. GC Reachability Law
Only unreachable nodes are deleted.

## 5. Concurrency Law
Lock order: WAL → NodeStore → ValueStore → SMT.

## 6. Compaction Law
WAL can be truncated after safe checkpoint.

## 7. Recovery Law
Recovery restores last consistent state from WAL + manifest.

## 8. Version Manifest Law
Every state root is recorded with lineage and epoch.

## 9. Proof Soundness Law
Proofs verify inclusion/absence deterministically.

## 10. Deterministic Encoding Law
All canonical serialization uses CCBF with fixed endianness.
