# CCS v0.7 – V7-009: Why Must Closure Occur?

**Date:** 2026-05-31
**Status:** Open Problem – The Final Theoretical Question

---

## The Question of V7-009

> Is Closure (C1-C3) a fundamental axiom of CCS, or can it be derived
> from deeper structural properties of the derivability and exclusion
> space?

V7-008 established the three-layer constitutional algebra:
(⊢_C, ⇍_C, Closure). But Closure was added as an axiom set (C1-C3).
It guarantees that every possibility is eventually decided, that a
canonical authority exists, and that it is unique.

This raises the deepest question of CCS:
**Why must closure occur?**

If closure is an axiom, CCS is a consistent framework.
If closure is a theorem, CCS is an explanatory theory.

V7-009 explores the conditions under which closure is forced.

---

## 1. Candidate Structural Properties

### Property S1: Well-Foundedness of ⊢_C
There are no infinite ascending chains of derivability.
Every derivability sequence reaches a maximal element in finitely
many steps.

### Property S2: Finite Branching
For any context `C`, the set `{ C' : C ⊢_C C' }` is finite.
Only finitely many possibilities are generated at each step.

### Property S3: Evidence Monotonicity
If `C₁ ⊢_C C₂`, then `Evidence(C₂) ≥ Evidence(C₁)`.
Evidence only grows along derivability chains.

### Property S4: Exclusion Completeness
If `C₁ ⇍_C C₂`, the exclusion is justified by a finite set of
constitutional evidence rules.
There is a decision procedure for exclusion.

---

## 2. The Derivability Conjecture

**Constitutional Derivability Conjecture:**

If a CCS system satisfies S1-S4, then Closure (C1-C3) follows
as a theorem.

That is:
- **Well-Foundedness** prevents infinite ambiguity.
- **Finite Branching** makes the possibility space manageable.
- **Evidence Monotonicity** ensures that competing paths cannot
  coexist indefinitely (one eventually dominates in evidence).
- **Exclusion Completeness** ensures that the decision procedure
  terminates.

Under these conditions, the system must converge to a unique
canonical authority.

---

## 3. The Minimal Counterexample Challenge

To test the conjecture, one must attempt to construct a system
satisfying S1-S4 but violating Closure.

A minimal counterexample would require:
- An infinite set of legitimate contexts
- No infinite ascending chain (well-founded)
- Finitely many successors per context (finite branching)
- Growing evidence along chains (evidence monotonicity)
- Exclusion rules that are decidable (exclusion completeness)

Yet somehow, permanent ambiguity persists.

If no such counterexample exists, Closure is a theorem.
If one does exist, Closure is indeed an independent axiom.

---

## 4. Experimental Evidence

AmunChain v0.3 provides empirical support for the conjecture:
- Competing proposals eventually see one dominate in evidence.
- Stale and foreign evidence is excluded by finite rules.
- Epoch transitions are well-founded (epochs increase).
- Validator sets are finite, so branching is finite.
- Recovery reconstructs the unique chain from finite evidence.

No experiment has produced permanent ambiguity.

---

## 5. What This Means

If the Derivability Conjecture holds, CCS achieves theoretical
completion:

- **⊢_C** and **⇍_C** are the primitive relations.
- **S1-S4** are the structural properties that govern them.
- **Closure** is not an axiom; it is a consequence of S1-S4.
- **Authority** emerges as the inevitable result.

CCS becomes a theory of **why constitutional systems converge**
rather than merely describing that they do.

---

## 6. Next Steps

1. Formalize S1-S4 in TLA+ or Coq.
2. Attempt to model-check or prove the Derivability Conjecture.
3. Search for minimal counterexamples.
4. If the conjecture holds, CCS achieves closure as a theorem.
5. If a counterexample exists, identify the missing structural
   property that forces closure.

---

## 7. Conclusion

V7-009 does not close CCS.
It opens the final investigation.

The question is no longer "What is authority?"
It is "Why must possibilities collapse into authority?"

The answer to this question will determine whether CCS is a
framework for describing constitutional systems, or a theory
that explains their deepest properties.
