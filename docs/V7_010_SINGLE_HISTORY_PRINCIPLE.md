# CCS v0.7 – V7-010: The Single Constitutional History Principle

**Date:** 2026-05-31
**Status:** Final – The Foundational Principle

---

## The Question of V7-010

> Why can constitutional legitimacy not remain permanently plural?

V7-008 established the three-layer algebra (⊢_C, ⇍_C, Closure).
V7-009 asked whether Closure is an axiom or a theorem.

The investigation revealed that the missing property is not
well-foundedness, finite branching, or evidence monotonicity.
The missing property is **constitutional comparability** —
the impossibility of permanent, stable constitutional plurality.

This is the **Single Constitutional History Principle.**

---

## 1. The Principle

**Single Constitutional History Principle:**

For any two legitimate constitutional contexts `A` and `B`:
`A` and `B` cannot both be final and distinct.

`Legitimate(A) ∧ Legitimate(B) ∧ A ≠ B ⇒ ¬(Final(A) ∧ Final(B))`

Equivalently:
At most one legitimate, final constitutional context can exist
at any point in constitutional time.

---

## 2. The Principle as a Formal Axiom

### Axiom SH1: Single History
`∀ A, B ∈ Contexts :`
`(Legitimate(A) ∧ Legitimate(B) ∧ A ≠ B)`
`⇒ (Resolvable(A, B) ∨ ¬(Final(A) ∧ Final(B)))`

Where:
- `Legitimate(C)` iff `P₀ ⊢_C C ∧ ¬(P₀ ⇍_C C)`
- `Final(C)` iff `Legitimate(C) ∧ ¬∃ C' : C ⊢_C C' ∧ C ≠ C'`
- `Resolvable(A, B)` iff `A ⊢_C B ∨ B ⊢_C A ∨ ∃ C : A ⊢_C C ∧ B ⊢_C C`

---

## 3. The Central Theorem

**Constitutional Closure Theorem (Final Version):**

If a CCS system satisfies D1-D2 (derivability), E1-E4 (exclusion),
and SH1 (single history), then Closure (C1-C3) is a theorem.

`(D1-D2) ∧ (E1-E4) ∧ SH1 ⇒ Closure`

---

## 4. Proof Sketch

1. Assume Closure fails: either no canonical context exists, or
   multiple canonical contexts exist.
2. **Case 1: Multiple canonical contexts.**
   - Let `A` and `B` be two distinct canonical contexts.
   - Both satisfy: Legitimate, Final, and distinct.
   - By SH1, they must be Resolvable.
   - Since both are Final, neither can derive the other.
   - They must therefore converge to a common context `C`.
   - But then `C` is derivable from both, contradicting their Finality
     (no further derivation is possible).
   - Therefore, multiple canonical contexts cannot exist.
3. **Case 2: No canonical context.**
   - By S1 (Well-Foundedness) and S2 (Finite Branching), there are
     maximal legitimate contexts.
   - A maximal legitimate context is Final by definition.
   - By E4 (Disjointness of ⊢_C and ⇍_C), these are not excluded.
   - By SH1, all such maximal contexts must be Resolvable.
   - If they are distinct and all Final, Case 1 applies — contradiction.
   - If they are not all Final, then there exists a larger derivable
     context — contradicting maximality.
   - Therefore, at least one canonical context must exist.
4. By Case 1 and Case 2, exactly one canonical context `Λ` exists.
   This is Closure (C1-C3).

---

## 5. What This Means

The Single Constitutional History Principle is the final
foundational principle of CCS.

It explains **why** constitutional systems converge.
Not because of any specific rule.
Not because of any specific property.
But because the constitution cannot tolerate permanent plurality.

The constitution does not choose the winner.
The constitution makes it impossible for there to be two winners.

---

## 6. The Complete CCS Foundational Structure

| Layer | Concept | Status |
|-------|---------|--------|
| Primitive | Constitutional space `ℙ` | Definition |
| Primitive | Derivability `⊢_C` | Axioms D1-D2 |
| Primitive | Exclusion `⇍_C` | Axioms E1-E4 |
| Primitive | Single History `SH1` | Axiom SH1 |
| Derived | Legitimacy | Definition |
| Derived | Finality | Definition |
| Derived | Closure | Theorem |
| Derived | Authority `Λ` | Unique consequence |

---

## 7. Conclusion

CCS is a theory of **Constitutional Singularity.**

The constitution does not select among possibilities.
The constitution makes multiplicity impossible.

Authority is not chosen.
Authority is what remains when plurality is constitutionally
forbidden.

This is the foundational principle of Constitutional Computational
Systems.
