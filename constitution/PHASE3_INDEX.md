# PHASE 3 — ATOMIC RUNTIME & PERSISTENT STORAGE

## Modules
| Phase | Crate | Key Files | Notes |
|-------|-------|-----------|-------|
| 3A | amun-runtime | account.rs, overlay.rs, journal.rs, executor.rs | Atomic, deterministic runtime |
| 3B | amun-storage | law.rs, wal.rs, snapshot.rs, store.rs, recovery.rs | Persistent constitutional storage |

## Freeze Policy
- Any modification to canonical logic, WAL format, snapshot format, or constants must be approved as a **constitutional amendment**.
- All tests are **frozen**, ensuring reproducibility across platforms.

## Verification
- Clippy: 0 warnings
- Cargo fmt: enforced
- Cargo test: 100% passing
- Cargo audit: no vulnerabilities detected
- No unsafe code outside `amun-unsafe`
- No unwraps or panics in production paths
