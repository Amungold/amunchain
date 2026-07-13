# Replay Determinism Law v1.0

## Article I: State Machine Replay

Given the same initial state and the same sequence of blocks,
the final state MUST be BIT-IDENTICAL on all platforms.

## Article II: Journal Determinism

The Write-Ahead Log guarantees:
- State transitions journaled BEFORE execution
- Journal entries are append-only and immutable
- Replay reproduces exact state
- Checksums verify journal integrity

## Article III: Execution Determinism

Block execution must be deterministic:
- execute(state, block) is a PURE FUNCTION
- Same (state, block) on all platforms produces identical output
- Gas metering is deterministic
- Event ordering is canonical

## Article IV: Cross-Platform Equivalence

Before any release:
- Full test suite on x86_64-unknown-linux-gnu
- Full test suite on aarch64-unknown-linux-gnu
- Full test suite on wasm32-unknown-unknown
- All KAT hashes must match BIT-FOR-BIT across platforms

## Article V: Memory Determinism

Memory layout MUST NOT affect consensus outputs:
- Allocator choice must not change state roots
- Pointer addresses must never be serialized
- Padding bytes must be canonicalized (zero-filled before hashing)
- Struct field ordering must be explicit (repr(C) where needed)
- Collection iteration order must be canonical

## Article VI: Forbidden Non-Determinism Sources

FORBIDDEN in all consensus paths:
- Wall clock access
- Random number generation
- HashMap/HashSet iteration
- Float operations
- Thread spawning
- Async I/O
- Signal handlers
- Environment variables
- Filesystem timestamps
- Pointer-to-integer casts
- Allocator-dependent behavior
