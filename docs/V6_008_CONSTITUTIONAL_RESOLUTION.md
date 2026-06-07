# CCS v0.6 – V6-008: Constitutional Resolution Principle

**Date:** 2026-05-31
**Status:** Draft – The Final Piece

---

## The Question of V6-008

> What mathematical process reduces multiple legitimate paths to exactly
> one final authority?

V6-007 showed that the constitution prefers one path over another.
V6-008 asks: **How is this preference resolved into finality?**

The answer must not assume the conclusion. We must define the resolution
rules and prove that they produce a unique final authority.

---

## 1. Two Layers of Constitutional Existence

### Layer 1: Legitimacy Space
`LegitimatePaths(G)` is the set of all paths from `P₀` that satisfy
constitutional validity rules.

This space may contain:
- Conflicting paths (V3-006B)
- Amendment forks (V3-007A)
- Multiple legitimate proposals

Legitimacy is **permissive**: it admits multiple possibilities.

### Layer 2: Finality Space
`FinalAuthority` is the unique path that has survived constitutional
resolution.

Finality is **exclusive**: only one path can be final.

---

## 2. The Resolution Function

`Resolve : LegitimatePaths(G) → FinalAuthority`

This function is not an axiom. It is defined by the constitutional
rules of conflict resolution.

### Resolution Rules

1. **Epoch Rule:** `π₁` dominates `π₂` if `Epoch(π₁) > Epoch(π₂)`.
2. **Evidence Rule:** If epochs are equal, `π₁` dominates `π₂` if
   `Evidence(π₁)` is stronger (larger quorum, more valid signatures).
3. **Lexicographic Rule:** If epochs and evidence are equal, `π₁`
   dominates `π₂` if `StateHash(π₁) < StateHash(π₂)`.
4. **Transitive Closure:** Domination is transitive.

These rules define a **total order** `π₁ ◃ π₂` ("`π₁` dominates `π₂`")
on legitimate paths.

---

## 3. Axioms of Resolution

### Axiom R1: Exclusivity
`Final(π₁) ∧ Final(π₂) ∧ Conflicting(π₁, π₂) ⇒ π₁ = π₂`

No two conflicting paths can both be final.

### Axiom R2: Stability
`Final(π) ⇒ □ Final(π)`

Once a path is final, it remains final forever.

### Axiom R3: Completeness
`∀ π ∈ LegitimatePaths(G) : (∃ π_F : Final(π_F) ∧ (π ◃ π_F ∨ Compatible(π, π_F)))`

Every legitimate path is either dominated by a final path or is
compatible with a final path.

---

## 4. The Central Theorem

**Constitutional Resolution Theorem:**

For any constitutional possibility graph `G` rooted at `P₀`, the
resolution rules define a unique final authority `Λ`.

`∃! Λ : Final(Λ)`

This theorem follows from:
1. The resolution rules define a total order `◃`.
2. Any finite set of paths with a total order has a unique maximum.
3. The maximum under `◃` is the final authority.

---

## 5. Consistency with AmunChain Experiments

- V3-006B (Conflicting QCs): `◃` selects one QC; the other dies.
- V3-007A (Amendment forks): `◃` selects one amendment chain.
- V3-007B/C (Epoch transitions): Higher epoch dominates.
- V3-007D (Recovery): Reconstructs the maximum `◃` from evidence.
- **Exclusivity:** No experiment ever produced two final authorities.
- **Stability:** No final authority was ever reversed.
- **Completeness:** Every legitimate proposal was either committed
  or superseded by a committed one.

---

## 6. What This Means

CCS is a theory of **Constitutional Resolution.**

The fundamental structure is not:
- Authority objects
- Authority positions
- Authority paths
- Canonicalization functions

It is the **resolution rules** that select one final authority from
among multiple legitimate possibilities.

---

## 7. Conclusion

The constitution does not create authority.
The constitution resolves authority.

It takes a space of legitimate possibilities and reduces it to a
single final reality.

This is the final piece of CCS theory.
All previous concepts — determinism, uniqueness, monotonicity,
conservation, recovery, preference — are consequences of
constitutional resolution.

The central object of CCS is therefore:
`Resolve : LegitimatePaths(G) → FinalAuthority`

This function is not assumed. It is defined by the resolution rules.
Its uniqueness is a theorem, not an axiom.
