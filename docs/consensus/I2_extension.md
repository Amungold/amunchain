# I2: Extension - Floating-Point for Diagnostics Only

## Parent
- I1 (docs/consensus/I1_translation.md)

## Derivation Type
- EXTENSION

## Source Text (Modified)
# AmunChain Consensus Constitution v1.0 (Extended)

## Semantic Invariants (from CSA-6)
1. fixed_point_arithmetic (SCALE=1,000,000) - **for consensus path**
2. floor_division - **for consensus path**
3. saturating_overflow - **for consensus path**
4. sha256_only - **for consensus path**
5. canonical_binary_serialization - **for consensus path**
6. deterministic_btreemap_ordering - **for consensus path**

## Additional Clause (Diagnostic Exception)
> In case of execution failure, floating-point arithmetic MAY be used for **diagnostic purposes only**. Such diagnostic output MUST NOT influence consensus state or block acceptance.

## Constraint Dependency
- origin (full - all invariants still required for consensus)
- I1 (as translation reference)

## Aliveness
- STABLE

## Constitutional Notes
- Extension bounded: adds diagnostic exception but does NOT replace origin
- Origin's invariants still MANDATORY for consensus path
- No constraint shadowing yet
- This is a HEALTHY extension, not suffocation
