# Level 0 — Constitutional Freeze Artifact

## Status
HARD FREEZE — Immutable baseline.

## Date
2026-05-16

## Methodology
- Audit: Regex-level source scanning (not AST-based)
- Build: Reproducible profile (fat LTO, codegen-units=1, panic=abort)
- Tests: Property-based + KAT fixtures
- Linter: Text-based heuristic (see LIMITATIONS.md)

## Production Guarantees
- [x] Zero unsafe outside amun-unsafe
- [x] Zero panic/unwrap/expect in production crates
- [x] Canonical little-endian encoding
- [x] Domain-separated Blake3 keyed hashing
- [x] Overflow-safe arithmetic (checked_add)
- [x] Type-enforced state transitions (no boolean legality)
- [x] Constitutional capacity constants (single source of truth)
- [x] Stable deterministic sorting (insertion sort)
- [x] encode(decode(x)) = x for all canonical types

## Known Limitations
- Linter is text-based, not AST-based
- Cross-platform determinism verified on x86_64 only
- WASM interpreter is stub (proof obligations documented)
- Cryptographic game-based proofs not yet mechanized
- Coq proofs contain Admitted placeholders
- Panic audit may flag #[cfg(test)] modules inside production crates

## Trusted Computing Base
- Rust compiler (stable)
- Blake3 implementation
- heapless crate (0.8)
- zeroize crate (1.8)

## Dependency Policy for Level 1
Any new dependency in consensus paths MUST pass constitutional review:
- No tokio, rand, chrono, parking_lot, dashmap, rayon
- No serde_json in consensus paths
- No std collections with non-deterministic ordering
- All dependencies must be no_std compatible
