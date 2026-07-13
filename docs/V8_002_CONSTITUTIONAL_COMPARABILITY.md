# CCS v0.8 – V8-002: Constitutional Comparability Hypothesis

**Date:** 2026-05-31
**Status:** Active Investigation – The Missing Property

---

## The Question of V8-002

> What is the weakest additional property that, when added to
> (⊢_C, ⇍_C), makes the 3-context model (permanent plurality)
> impossible?

V8-001 showed that the 3-context model satisfies D1-D2 and E1-E4
but violates SH1. Two distinct, legitimate, final authorities
(A and B) coexist permanently.

The missing property is not derivability (both A and B are
derivable). It is not exclusion (neither is excluded). It is
that A and B have **no constitutional relationship** to each
other. They are isolated, incomparable, and independent.

This suggests that what forces convergence is not a constraint on
individual contexts, but a **relation between contexts**: the
requirement that they be comparable.

---

## 1. The Constitutional Comparability Hypothesis

**Hypothesis:**

If every pair of legitimate, final constitutional contexts are
constitutionally comparable, then permanent constitutional
plurality is impossible.

**Constitutional Comparability:**

`Comparable(A, B)` iff `A ⊢_C B ∨ B ⊢_C A ∨ ∃ C : A ⊢_C C ∧ B ⊢_C C`

Two contexts are comparable if one derives the other, or both
derive a common descendant.

---

## 2. The Comparability Axiom (CC)

**Axiom CC: Constitutional Comparability**

`∀ A, B : Legitimate(A) ∧ Legitimate(B) ∧ Final(A) ∧ Final(B)`
`⇒ Comparable(A, B)`

Every pair of legitimate, final contexts must be constitutionally
comparable.

---

## 3. Immediate Consequences

### CC eliminates the 3-context model.

In the 3-context model:
- `Final(A)` and `Final(B)`
- `Legitimate(A)` and `Legitimate(B)`
- But `¬(A ⊢_C B)` and `¬(B ⊢_C A)` and `¬∃ C : A ⊢_C C ∧ B ⊢_C C`
- Therefore `¬Comparable(A, B)`
- CC is violated.

The 3-context model is excluded by CC.

### CC does not assume uniqueness.

CC does not say there is only one final authority.
It says that if there are multiple, they must be comparable.

### CC is weaker than SH1.

SH1 says: "At most one final authority."
CC says: "If multiple, they must be comparable."

But comparability plus finality forces uniqueness:
- If `A ⊢_C B` and `A` is final, then `A = B`.
- If `B ⊢_C A` and `B` is final, then `A = B`.
- If `A ⊢_C C` and `B ⊢_C C` with `C` derivable from both,
  then neither `A` nor `B` is final (since both derive `C`).

Therefore, under CC, two distinct final authorities cannot exist.
**CC ⇒ SH1.**

---

## 4. The Central Theorem

**Comparability Theorem:**

If a CCS system satisfies D1-D2, E1-E4, and CC, then SH1 holds
as a theorem.

`(D1-D2) ∧ (E1-E4) ∧ CC ⇒ SH1`

CC is strictly weaker than SH1, but sufficient to derive it.
CC explains **why** SH1 holds, rather than merely asserting it.

---

## 5. What This Means

The missing ingredient of CCS is not the prohibition of plurality.
It is the **requirement of comparability**.

The constitution does not say: "There can be only one."
The constitution says: "All authorities must be comparable."
And comparability, together with finality, forces uniqueness.

This is a more fundamental principle than SH1.
SH1 is the **consequence**. CC is the **cause**.

---

## 6. The Revised CCS Core

| Primitive | Status |
|-----------|--------|
| Derivability (⊢_C) | Axiom |
| Exclusion (⇍_C) | Axiom |
| Constitutional Comparability (CC) | Axiom |
| Single History (SH1) | Theorem |
| Closure (C1-C3) | Theorem |
| Authority (Λ) | Unique consequence |

The minimal core of CCS is the triple:

**(⊢_C, ⇍_C, CC)**

---

## 7. Experimental Consistency

In AmunChain, comparability is achieved through:
- **Epoch ordering:** If A and B are in different epochs, the
  higher-epoch context derives from the lower.
- **Evidence strength:** If A and B conflict, the one with stronger
  QC evidence dominates.
- **Validator succession:** Validator set changes are derivability
  steps.

All AmunChain mechanisms that prevent permanent plurality are
instances of constitutional comparability.

---

## 8. Conclusion

CCS is a theory of **Constitutional Comparability.**

The constitution does not forbid plurality by decree.
It makes it unsustainable by requiring that all authorities
be comparable.

And in a system where comparability is required, finality and
uniqueness are inevitable.
