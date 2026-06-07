# CCS v0.6 – V6-007: Constitutional Preference Principle

**Date:** 2026-05-31
**Status:** Draft – From Merge to Selection

---

## The Question of V6-007

> Is canonical authority the result of merging paths, or the result of
> selecting the **preferred** path among alternatives?

V6-006 proposed Constitutional Convergence as a join-semilattice of paths.
But experimental evidence from AmunChain (V3-006B, V3-007A) shows that
the system does not **merge** conflicting histories.
It **selects** one and discards the other.

This suggests a deeper principle: **Constitutional Preference.**

---

## 1. The Principle

**Constitutional Preference Principle:**

For any set of conflicting legitimate paths, there exists a total
constitutional preference order `≺` that selects exactly one path
as the canonical authority.

Formally:
`∀ π₁, π₂ ∈ LegitimatePaths(G) : (π₁ ≺ π₂) ∨ (π₂ ≺ π₁)`

The canonical chain `Λ` is the **maximum** element under `≺`.

---

## 2. From Merge (⊔) to Select (max)

| Merge Model (V6-006) | Select Model (V6-007) |
|---|---|
| `π₁ ⊔ π₂` creates a new path | `π₁ ≺ π₂` chooses one |
| Requires join-semilattice | Requires total order |
| Both paths contribute | Only one survives |
| Convergence through inclusion | Convergence through preference |

---

## 3. The Core Theorem (Conjecture)

**Constitutional Preference Theorem:**

The set of legitimate constitutional paths `(Paths(G), ≺)` is a
**totally ordered set** under constitutional preference.

The canonical chain `Λ` is the unique maximum:

`Λ = max_{≺} LegitimatePaths(G)`

---

## 4. Constitutional Preference Order (≺)

`≺` is a total order satisfying:

### Axiom P1: Foundational Minimum
`∀ π : P₀ ≺ π`

The genesis path is the minimum element.

### Axiom P2: Epoch Priority
If `π₁` and `π₂` share a common prefix, and `π₂` reaches a higher
epoch, then `π₁ ≺ π₂`.

### Axiom P3: Evidence Priority
If `π₁` and `π₂` reach the same epoch, but `π₂` has stronger evidence
(larger quorum, more valid signatures), then `π₁ ≺ π₂`.

### Axiom P4: Lexicographic Consistency
If epochs and evidence are equal, preference is determined by
lexicographic comparison of state hashes.

---

## 5. Consistency with AmunChain Experiments

- V3-006B (Conflicting QC): Preference selects one QC; the other dies.
- V3-007A (Amendment forks): Preference selects one amendment chain.
- V3-007B/C (Epoch transitions): Higher epoch dominates.
- V3-007D (Recovery): Reconstructs the maximum preference path from
  available evidence.

---

## 6. What This Means

CCS is a theory of **Constitutional Preference.**

The fundamental structure is a totally ordered set of legitimate paths.
The canonical chain is the maximum element under constitutional preference.
Legitimacy is the property of being comparable under `≺`.
Authority is the maximum element of `≺`.

This is simpler, stronger, and more consistent with experimental
evidence than the convergence/merge model.

---

## 7. Conclusion

If V6-006 asked "Do all paths converge?", V6-007 answers:
**"No. The constitution prefers one."**

The central structure of CCS is therefore:
`(LegitimatePaths(G), ≺, max)`

A totally ordered set with a unique maximum — the canonical authority chain.
