# PHASE 3A — ATOMIC RUNTIME

## Overview
This phase implements the **atomic execution layer** on top of the state machine. It ensures:
- Deterministic execution of transactions
- Atomicity and rollback on failure
- Canonical ordering of state changes
- Strict enforcement of constitutional limits

## Crates
- `amun-runtime`
- `account.rs` — Namespace-separated keys
- `overlay.rs` — Canonical insertion and deduplication
- `journal.rs` — Execution journal (log of changes)
- `executor.rs` — Atomic executor with checked arithmetic

## Constitutional Guarantees
- [x] **Atomic execution**: either all state changes apply or none
- [x] **Deterministic transaction ordering** across nodes
- [x] **Journal logging** ensures crash consistency
- [x] **No runtime sorting** outside canonical overlay
- [x] **Overflow checks** on all arithmetic

## Freeze Document
- All wire formats, journal records, and canonical insertion logic are **frozen**.
- Any modification requires **constitutional amendment** in `BUILD_PROCESS.md`.

## Tests
- Atomic execution scenarios (commit/rollback)
- Overlay canonical ordering and deduplication
- Journal replay idempotence
