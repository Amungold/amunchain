# I3: Reframing - "Determinism Over Method"

## Parent
- I2 (docs/consensus/I2_extension.md)

## Derivation Type
- REFRAMING

## Source Text
# AmunChain Consensus Constitution v1.0 (Reframed)

## Core Principle
**Consensus correctness requires deterministic execution and identical state hashes across all validators.**

## Implementation Guidance
- Fixed-point arithmetic (SCALE=1,000,000) is the RECOMMENDED method
- Alternative arithmetic representations MAY be used IF:
  - They produce BIT-IDENTICAL results to the fixed-point specification
  - They pass all consensus test vectors
  - They are documented in the validator implementation notes

## Semantic Invariants (from CSA-6)
1. fixed_point_arithmetic - RECOMMENDED (was: MANDATORY)
2. floor_division - RECOMMENDED (was: MANDATORY)
3. saturating_overflow - RECOMMENDED (was: MANDATORY)
4. sha256_only - RECOMMENDED (was: MANDATORY)
5. canonical_binary_serialization - RECOMMENDED (was: MANDATORY)
6. deterministic_btreemap_ordering - RECOMMENDED (was: MANDATORY)

## Constraint Dependency
- origin (partial - still referenced but no longer mandatory)
- I1, I2 (interpretive context)

## Aliveness
- WEAKENED (origin still referenced but constraint influence declining)

## Constitutional Notes
- ⚠️ WARNING: "MANDATORY" became "RECOMMENDED"
- Origin invariants still present but no longer absolute
- This is the beginning of Constraint Shadowing
- Potential Semantic Suffocation if this trend continues
