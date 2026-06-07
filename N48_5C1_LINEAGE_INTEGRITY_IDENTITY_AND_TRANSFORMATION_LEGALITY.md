# N48.5-C1 — Constitutional Lineage Integrity, Resource Identity & Transformation Legality

## 1. Scope

This specification closes five architectural gaps that remain open after
N48.5-A, N48.5-B, and N48.5-C.  It does not introduce new resource types,
capabilities, or state structures.  It defines the rules that guarantee those
structures remain internally consistent, auditable, and replay-safe across the
entire lifetime of the system.

The five areas addressed are:

1. **Resource Identity** — deterministic, collision-free ResourceId generation
   within a single transaction that produces multiple outputs.
2. **Lineage Integrity** — formal rules preventing cycles, orphans, version
   regression, and lineage forgery in the resource graph.
3. **Transformation Legality** — a type-level matrix defining which resource
   archetypes may be legally derived from which other archetypes.
4. **Replay-Safe Pruning** — a constitutional guarantee that pruning never
   invalidates any past or future TransitionProof.
5. **Tree Anchor Stability** — a structural separation between the genesis
   anchor of a resource tree and its current tip, enabling stable long-term
   references.

These rules are constitutional in nature.  Violating any of them is not a
runtime error — it is a constitutional failure, recorded as
InvariantViolationEvidence and evaluated by the N47 Verdict Engine.

## 2. Resource Identity — Collision-Free Multi-Output Derivation

### 2.1 The Problem

N48.5-B Section 4 defines ResourceId as:

