# PHASE 3B — PERSISTENT STORAGE ENGINE

## Overview
This phase implements **constitutional persistent storage** with:
- Write-Ahead Log (WAL)
- Snapshots with authenticated checksums
- Last-valid-commit recovery
- Canonical insertion in all paths

## Crates
- `amun-storage`
- `law.rs` — Constitutional storage constants (frozen)
- `wal.rs` — Cryptographically chained WAL
- `snapshot.rs` — Authenticated state snapshots
- `store.rs` — PersistentStore with canonical insertion
- `recovery.rs` — Recovery engine (replays committed WAL only)

## Constitutional Guarantees
- [x] **WAL integrity** (previous_hash + entry_hash)
- [x] **Sequence continuity** verification
- [x] **Framing verification** (MAGIC + VERSION + LENGTH)
- [x] **Last-valid-commit recovery** (post-commit tail discarded)
- [x] **Recovered root verification** (must match committed root)
- [x] **Snapshot authenticity verification** (checksum)
- [x] **Canonical insertion in all paths** (no runtime sort)
- [x] **Recovery idempotence** (apply_replay, no WAL re-append)
- [x] **Bounds-checked snapshot decoding** (no slicing panics)

## Freeze Document
- All constants and storage formats are **immutable**.
- Changing any requires **constitutional amendment** in `STORAGE_LAW.md`.

## Tests
- WAL chain integrity and sequence continuity
- Hash linkage enforcement
- Last commit index and committed records truncation
- Snapshot roundtrip and checksum determinism
- Store set/get/delete and root determinism
- Recovery roundtrip with snapshot + WAL replay
- Recovery verifies root correctness
- Recovery rejects corrupt WAL
