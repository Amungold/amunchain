# CCS v0.7 – V7-007: Derivability and Exclusion

**Date:** 2026-05-31
**Status:** Final Draft – The Irreducible Dual Core

---

## The Question of V7-007

> What if the primitive core of CCS is not one relation, but two —
> one that builds, and one that eliminates?

V7-006 reduced CCS to a single primitive: exclusion (⇍_C).
But defining legitimacy purely as "not excluded" is too weak.
It makes every context legitimate by default, until exclusion
actively removes it.

What about contexts that are never reached?
What about contexts that have no constitutional derivation?

Legitimacy requires **both**:
- A positive path from genesis (derivability)
- No constitutional exclusion along that path

V7-007 defines CCS around **two primitive relations** —
Derivability (⊢_C) and Exclusion (⇍_C) — and shows that their
interaction produces all constitutional properties.

---

## 1. The Two Primitive Relations

### Derivability (⊢_C)
`C₁ ⊢_C C₂`

means: "Context `C₂` is constitutionally derivable from context `C₁`."

This is the **constructive** dimension of the constitution.
It defines what can be built.

### Exclusion (⇍_C)
`C₁ ⇍_C C₂`

means: "Context `C₂` is constitutionally excluded by context `C₁`."

This is the **eliminative** dimension of the constitution.
It defines what is forbidden.

---

## 2. Axioms

### Axiom D1: Foundational Root
`P₀ ⊢_C P₀`

The genesis context is self-derivable.

### Axiom D2: Derivability is Transitive
`C₁ ⊢_C C₂ ∧ C₂ ⊢_C C₃ ⇒ C₁ ⊢_C C₃`

Constitutional history is composable.

### Axiom E1: Exclusion is Irreversible
`C₁ ⇍_C C₂ ⇒ ∀ C₃ : C₁ ⊢_C C₃ ⇒ C₃ ⇍_C C₂`

Once excluded, always excluded. Exclusion propagates forward.

### Axiom E2: Exclusion Closes Subtrees
`C₁ ⇍_C C₂ ∧ C₂ ⊢_C C₃ ⇒ C₁ ⇍_C C₃`

If a context is excluded, all its descendants are excluded.

### Axiom E3: Self-Exclusion is Impossible
`¬(C ⇍_C C)`

No context excludes itself.

### Axiom E4: Derivability and Exclusion are Disjoint
`C₁ ⊢_C C₂ ⇒ ¬(C₁ ⇍_C C₂)`

If something is derivable, it cannot be excluded by the same context.

---

## 3. Derived Concepts

### Legitimacy
`Legitimate(C)` iff `P₀ ⊢_C C ∧ ¬(P₀ ⇍_C C)`

A context is legitimate if it is reachable from genesis AND
has not been excluded from genesis.

This is stronger than either relation alone.

### Authority
`Authority(C)` is the unique context `Λ` such that:
- `P₀ ⊢_C Λ`
- `¬(P₀ ⇍_C Λ)`
- `∀ C' : P₀ ⊢_C C' ∧ ¬(P₀ ⇍_C C') ⇒ C' ⊢_C Λ`

The canonical chain is the ultimate derivable, non-excluded context.

### Finality
`Final(C)` iff `¬∃ C' : C ⊢_C C' ∧ C ≠ C'`

No further legitimate derivation is possible.

### Continuity
Continuity is the derivability chain from `P₀` to `Λ` that
avoids exclusion at every step.

### Pruning
Pruning is the operation of `⇍_C`: removing contexts from
legitimacy.

### Convergence
Convergence follows from the interaction of `⊢_C` and `⇍_C`:
derivability creates possibilities; exclusion eliminates
all but one.

---

## 4. The Central Theorem

**Constitutional Completeness Theorem:**

For any system satisfying D1-D2 and E1-E4, there exists exactly
one context `Λ` such that:
- `P₀ ⊢_C Λ`
- `¬(P₀ ⇍_C Λ)`
- `Final(Λ)`

The canonical authority `Λ` is the unique derivable, non-excluded,
final context.

---

## 5. What This Means

CCS is a theory of **dual constitutional relations.**

The constitution has two faces:
- **⊢_C**: What can be built — the constructive constitution.
- **⇍_C**: What must be eliminated — the eliminative constitution.

From their interaction emerge:
- Legitimacy
- Authority
- Finality
- Continuity
- Pruning
- Convergence

No single relation suffices.
The dual core is the irreducible foundation of CCS.

---

## 6. Conclusion

The search for the primitive of CCS is over.

It is not Authority.
It is not Context.
It is not Derivability alone.
It is not Exclusion alone.

It is the **dual pair (⊢_C, ⇍_C)** —
Constitutional Derivability and Constitutional Exclusion.

This is the final form of CCS theory.
