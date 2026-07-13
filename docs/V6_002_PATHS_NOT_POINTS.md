# CCS v0.6 – V6-002: Paths, Not Points

**Date:** 2026-05-31
**Status:** Draft Axioms – From Position to Path

---

## The Question of V6-002

> Is legitimacy preserved in the positions, or in the paths?

V6-001 established authority as a position `P ∈ ℙ` in a constitutional
space with partial order `≤`. But the Conservation Axiom (`P₀ ≤ P`)
already hints at something deeper: what makes `P` legitimate is not its
coordinates `(E, V, H)`, but the **existence of a path** from the
foundational position `P₀` to `P`.

This document explores the consequences of making the **path** the
fundamental entity of CCS.

---

## 1. The Shift: From Points to Paths

In the position-based model:
- Authority is a point in `ℙ`.
- Legitimacy is `P₀ ≤ P` (existence of a path).
- The path is a **witness** to legitimacy, not legitimacy itself.

In the path-based model:
- Authority **is** the path from `P₀` to the current context.
- Legitimacy is the path itself.
- The position is merely the endpoint.

This is not a minor change. It means:
- **Recovery** reconstructs the path, not the point.
- **Conservation** is automatic: a path cannot be created from nothing.
- **Monotonicity** is structural: paths only extend, never shrink.
- **Acyclicity** is inherent: paths define direction, cycles are impossible.

---

## 2. Definitions

### 2.1 Constitutional Graph (G)
`G = (V, E)` where:
- `V` (vertices): Constitutional contexts
- `E` (edges): Legitimate constitutional transitions

`G` is a **directed acyclic graph (DAG)** rooted at `P₀`.

### 2.2 Constitutional Path
A **constitutional path** π is a finite sequence of contexts:
`π = (P₀, P₁, ..., Pₙ)`

such that for each `i`, `(Pᵢ, Pᵢ₊₁) ∈ E` is a legitimate transition.

### 2.3 Authority as Path
The authority at context `C` is the **unique path** from `P₀` to `C`:
`Authority(C) = π₀→C`

### 2.4 Legitimacy
`Legitimate(C)` iff there exists a path `π = (P₀, ..., C)` in `G`.

Equivalently: `Legitimate(C)` iff `C` is reachable from `P₀` in `G`.

---

## 3. Axioms (Path-Based)

### Axiom 1: Foundational Root
`∃! P₀ ∈ V` with in-degree 0.

There is exactly one genesis context with no predecessor.
All constitutional authority originates from `P₀`.

### Axiom 2: Deterministic Paths
For every vertex `v ∈ V` reachable from `P₀`, the path from `P₀` to `v`
is **unique**.

`∀ v ∈ V : reachable(P₀, v) ⇒ (∃! π : P₀ →* v)`

This replaces the "Antisymmetry" axiom from the poset model.
Acyclicity is a consequence, not an additional assumption.

### Axiom 3: Path Conservation
Every legitimate context has a path from `P₀`. Paths cannot be created
from nothing; they can only be extended or reconstructed from evidence.

`Legitimate(C) ⇒ ∃ π : P₀ →* C`

### Axiom 4: Path Monotonicity
For any legitimate transition `(C₁, C₂) ∈ E`, the path to `C₂` is an
extension of the path to `C₁`:

`π₀→C₂ = π₀→C₁ ++ (C₁, C₂)`

Authority only grows; it never regresses.

---

## 4. Consequences

### 4.1 Legitimacy is Path-Dependent
A context `C` is legitimate not because of its coordinates `(E, V, H)`,
but because there exists a verifiable constitutional history leading to
it. The coordinates are evidence; the path is authority.

### 4.2 Recovery is Path Reconstruction
Recovery (V3-007D) is not the creation of a new point. It is the
**reconstruction of the path** from `P₀` to the recovered context,
using constitutional evidence.

### 4.3 Acyclicity is Inherent
Since every vertex is reachable from `P₀` via a unique path, `G` is
necessarily acyclic. Cycles would create multiple paths, violating
Axiom 2. No separate "Acyclicity Theorem" is needed.

### 4.4 Conflicting Authorities are Incomparable
If two authorities claim different paths to the same context, at most
one can be legitimate. The others are not connected to `P₀` in `G`.
This is a structural property, not a definitional trick.

---

## 5. Open Questions for V6-003

1. Can `G` be extended to a **category** where morphisms are paths?
2. Does the uniqueness of paths make `G` a **tree**?
3. If validator amendments change `V`, how is that represented in `G`?
4. Is there a notion of "path distance" that corresponds to
   constitutional time or information gain?
5. Can conflicting authorities be represented as **forks** in an
   otherwise lawful graph, and what mechanisms resolve forks?

---

## 6. Conclusion

If V6-001 asked "Where is authority?", V6-002 answers:
**Authority is the path that led here.**

Legitimacy is not a property of a position. It is the existence of a
traceable constitutional history. Conservation and Monotonicity are
not additional axioms; they are structural properties of the path model.

This may be the most significant theoretical shift in CCS since the
introduction of Constitutional Determinism in v0.4.
