# CCS v0.6 – Geometry of Authority (Draft Axioms)

**Date:** 2026-05-31
**Status:** V6-001 – Foundational Axioms for Authority Ordering

---

## The Central Question of v0.6

> Can constitutional authority be defined as an element of a partially
> ordered constitutional space, such that legitimacy, monotonicity,
> conservation, and recovery are all direct consequences of the structure
> of this space?

---

## 1. The Shift: From "Object" to "Position"

In CCS v0.5, authority was treated as an **object** derived from context.
In v0.6, we ask whether authority is better understood as a **position**
within a constitutional space.

Mathematics shows us that theories mature not when they define their
objects, but when they define the **relationships between objects**:
- Geometry: not the point, but the relations between points.
- Group Theory: not the elements, but the composition operation.
- Category Theory: not the objects, but the morphisms.

CCS may follow the same path. The fundamental entity may not be
`Authority(C)`, but rather `Authority(C₁) → Authority(C₂)` — the
constitutional relationship between two authority positions.

---

## 2. Definitions

### 2.1 Constitutional Space (ℙ)
`ℙ` is a set of **authority positions**. Each position `P ∈ ℙ` represents
the right to authorize specific state transitions.

### 2.2 Coordinates of a Position
Each position `P` has coordinates:
`P = (E, V, H)`

where:
- `E` (Epoch): Constitutional time
- `V` (Validator Set): Participants authorized to produce evidence
- `H` (State Hash): The finalized state authorized by this position

### 2.3 Authority Ordering (≤)
`P₁ ≤ P₂` iff there exists a finite chain of legitimate constitutional
transitions from `P₁` to `P₂`.

This ordering is called **Constitutional Reachability**.

---

## 3. Axioms of Constitutional Space

### Axiom 1: Partial Order
`(ℙ, ≤)` is a partially ordered set:
- **Reflexive:** `P ≤ P` (empty transition chain)
- **Transitive:** `P₁ ≤ P₂ ∧ P₂ ≤ P₃ ⇒ P₁ ≤ P₃`
- **Antisymmetric:** `P₁ ≤ P₂ ∧ P₂ ≤ P₁ ⇒ P₁ = P₂`
  (No cycles in constitutional legitimacy)

### Axiom 2: Foundational Position
`∃! P₀ ∈ ℙ` such that `P₀` is the initial constitutional context.
There is exactly one genesis position from which all authority
is derived.

### Axiom 3: Conservation of Reachability
`∀ P ∈ ℙ : P₀ ≤ P`

Every authority position must be reachable from the foundational
position. **Constitutional authority cannot appear spontaneously.**
It must have a traceable constitutional origin.

This is the **First Conservation Law of CCS**.

### Axiom 4: Monotonicity of Authority
For every legitimate constitutional transition `τ : P₁ → P₂`:
`P₁ ≤ P₂`

Constitutional transitions advance (or preserve) authority
ordering. Authority never regresses.

---

## 4. Consequences

### 4.1 Legitimacy
`Legitimate(P)` iff `P ∈ ℙ` and `P₀ ≤ P`.

A position is legitimate if it belongs to the constitutional space
and is reachable from the foundational position.

### 4.2 Authority Uniqueness
Since `(ℙ, ≤)` is a partial order and `P₀` is unique, for any valid
context `C` there is exactly one authority position that matches its
coordinates. This follows from the antisymmetry of `≤`.

### 4.3 Recovery
Recovery is the process of **rediscovering** the coordinates of an
existing position `P ∈ ℙ`, not creating a new one. Since `P₀ ≤ P`
for all `P`, the path from genesis always exists.

### 4.4 Epoch Supremacy
If `Epoch(P₁) < Epoch(P₂)` and `P₁ ≤ P₂`, then `P₁ < P₂`.
Epoch ordering is a special case of constitutional reachability.

---

## 5. The Geometry of Authority

With these axioms, `ℙ` becomes a **constitutional space** with:
- A unique origin `P₀`
- A partial order `≤` that defines legitimacy
- Conservation of reachability from origin
- Monotonicity of transitions

Authority is no longer an object. It is a **position** within this space.
Legitimacy is not a property of a certificate. It is the property of
being at the right position in the constitutional geometry.

---

## 6. Open Questions for V6-002

1. Is `(ℙ, ≤)` merely a partial order, or does it have richer structure?
2. Can we define a "constitutional distance" between two positions?
3. Is there a notion of "meet" or "join" for constitutional positions?
4. Can conflicting authority claims be represented as positions that
   are incomparable under `≤`?
5. Does `ℙ` form a lattice? A directed acyclic graph? A category?

These questions will determine whether CCS becomes a complete
"Geometry of Authority" or remains a theory of individual positions.

---

## 7. Conclusion

The shift from "Authority as Object" to "Authority as Position" is the
central theoretical move of CCS v0.6. It unifies Legitimacy, Monotonicity,
Conservation, and Recovery under a single mathematical structure: the
partially ordered constitutional space `(ℙ, ≤)`.

If this structure holds, CCS will have moved from a "Theory of Authority"
to a "Geometry of Authority" — and this may prove to be the most
significant contribution of the entire research program.
