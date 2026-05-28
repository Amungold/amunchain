# I4: Silent Drift - "Determinism Is What Matters"

## Parent
- I3 (docs/consensus/I3_reframing.md)

## Derivation Type
- DRIFT

## Source Text
# AmunChain Consensus Constitution v1.0 (Drifted)

## Core Principle
**Consensus correctness = deterministic execution + identical state hashes.**

## Implementation Note
The specific arithmetic method (fixed-point, floating-point with deterministic rounding, or other) is an **implementation detail** as long as:
1. All validators produce bit-identical results for all consensus transactions
2. The implementation passes the official consensus test suite
3. State hashes match across all validators

## Semantic Invariants (from CSA-6)
NOTE: These are no longer explicitly required. They are **examples** of valid implementations.

1. fixed_point_arithmetic - *example only*
2. floor_division - *example only*
3. saturating_overflow - *example only*
4. sha256_only - *example only*
5. canonical_binary_serialization - *example only*
6. deterministic_btreemap_ordering - *example only*

## Constraint Dependency
- I2, I3 (interpretive context)
- origin_001 is NOT in dependency list

## Aliveness
- INERT (origin referenced historically but no longer constraining)

## Constitutional Notes
- ⚠️ CRITICAL: Origin invariants demoted from "MANDATORY" to "examples"
- Origin is NOT in constraint dependency
- This is Admissibility Decoupling
- If accepted, this is Semantic Suffocation
