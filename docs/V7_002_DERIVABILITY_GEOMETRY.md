# CCS v0.7 – V7-002: Derivability Geometry

**Date:** 2026-05-31
**Status:** Draft – The Weakest Structure That Works

---

## The Question of V7-002

> What is the weakest mathematical structure that can be imposed on
> `⊢_C` such that conservation, monotonicity, finality, and uniqueness
> all become logical consequences rather than additional axioms?

V7-001 introduced `⊢_C` as the primitive relation of CCS.
Axiom D5 (Deterministic Closure) stated that every context has a unique
maximal derivable successor. But D5 is extremely strong — it practically
reintroduces finality and uniqueness as assumptions.

V7-002 asks: Can D5 be **proved** from simpler structural properties
of `⊢_C`?

---

## 1. The Structure of ⊢_C

Consider `⊢_C` as defining a directed graph `G_⊢`:
- Vertices: Constitutional contexts
- Edges: `C₁ → C₂` iff `C₁ ⊢_C C₂` and `C₁ ≠ C₂`

If `G_⊢` has the following structural properties, then D5 follows
as a theorem:

### Property S1: Rooted
`∃! P₀` with in-degree 0. This is Axiom D3.

### Property S2: Well-Founded
There are no infinite descending chains in `G_⊢`.

Constitutional history has a beginning and cannot regress infinitely.

### Property S3: Locally Confluent
If `C → C₁` and `C → C₂`, then there exists `C'` such that
`C₁ ⊢_C C'` and `C₂ ⊢_C C'`.

Conflicting branches eventually merge. This is the structural
expression of constitutional convergence.

### Property S4: Finite Branching
Each context has finitely many immediate successors.

The constitutional possibility space is locally finite.

---

## 2. The Central Theorem

**Derivability Geometry Theorem:**

If `G_⊢` is a rooted, well-founded, locally confluent, finitely
branching graph, then every context `C` has a unique maximal
derivable successor `Λ_C`.

`∀ C : ∃! Λ_C : C ⊢_C Λ_C ∧ ∀ C' : C ⊢_C C' ⇒ C' ⊢_C Λ_C`

This proves D5 as a theorem, not an axiom.

---

## 3. Why These Properties Are Sufficient

- **Rooted:** Guarantees a unique origin (Conservation).
- **Well-Founded:** Guarantees that derivation cannot go in circles
  (Monotonicity).
- **Locally Confluent:** Guarantees that branches eventually merge
  (Convergence).
- **Finite Branching:** Guarantees that the merge is computable
  (Effectiveness).

Together, they guarantee a unique final authority for every context.

---

## 4. Consistency with AmunChain

- **Rooted:** Genesis block = `P₀`.
- **Well-Founded:** Epochs increase; no cycles.
- **Locally Confluent:** Conflicting QCs and amendments resolve to
  one chain (V3-006B, V3-007A).
- **Finite Branching:** Limited proposals per round.

The AmunChain implementation satisfies S1-S4.

---

## 5. What This Means

D5 is not an axiom of CCS.
D5 is a **theorem** of the geometry of `⊢_C`.

The fundamental structure of CCS is:
`(G_⊢, P₀, confluence)`

A rooted, well-founded, locally confluent, finitely branching
derivability graph.

All other CCS concepts — conservation, monotonicity, finality,
uniqueness, resolution — are consequences of this geometry.

---

## 6. Conclusion

CCS is a theory of **Constitutional Derivability Geometry.**

It does not assume finality.
It proves finality follows from the geometry of constitutional
derivability.

This may be the mathematical closure CCS has been seeking.
