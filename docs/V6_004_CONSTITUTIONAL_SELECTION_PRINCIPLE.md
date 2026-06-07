# CCS v0.6 – V6-004: Constitutional Selection Principle

**Date:** 2026-05-31
**Status:** Draft – From Canonicalization to Emergent Authority

---

## The Question of V6-004

> Must `Canonicalize` be an axiom, or can it be derived from a deeper
> principle?

V6-003 introduced `Canonicalize : G → Λ` as the central object of CCS.
But is this function an additional assumption, or is it a consequence
of something more fundamental?

This document proposes that `Canonicalize` is not an axiom.
It is a **theorem** that follows from a single principle:
**Constitutional Continuity Maximization.**

---

## 1. The Core Principle

**Constitutional Selection Principle:**

A canonical chain is any chain that maximizes constitutional continuity.

Formally:
`Λ ∈ argmax_{π ∈ Paths(G)} ConstitutionalContinuity(π)`

where `ConstitutionalContinuity(π)` is a measure of:
- Reachability from `P₀`
- Consistency of evidence along the path
- Monotonicity of epoch progression
- Validity of all transitions in `π`

---

## 2. The Key Theorem (Conjecture)

**Constitutional Authority Theorem:**

For any constitutional possibility graph `G` rooted at `P₀`, there
exists **exactly one** path `Λ` that maximizes constitutional continuity.

If this theorem holds, then `Canonicalize` is not an axiom.
It is a consequence of the structure of `G` and the definition of
`ConstitutionalContinuity`.

The authority chain `Λ` is simply **the fittest path** in `G`.

---

## 3. What This Changes

### If `Canonicalize` is an axiom:
- CCS assumes the existence of a selection function.
- The theory is weaker; it postulates rather than derives.
- Legitimacy depends on an external rule.

### If `Canonicalize` is a theorem:
- CCS derives the canonical chain from first principles.
- The theory is stronger; it explains rather than assumes.
- Legitimacy is an **emergent property** of constitutional fitness.

---

## 4. Constitutional Continuity (Formal Conjecture)

`ConstitutionalContinuity(π)` may be defined as:

- `π` is a path from `P₀` to some context `C`.
- Every edge in `π` represents a legitimate constitutional transition.
- The epochs in `π` are non-decreasing.
- All QC evidence in `π` is valid and constitutionally bound.
- `π` cannot be extended without violating consistency.

The path that maximizes this measure is the one that:
- Reaches the farthest epoch.
- Includes the most valid transitions.
- Has no constitutional gaps or inconsistencies.

---

## 5. Experimental Consistency

The Constitutional Selection Principle is consistent with all
AmunChain v0.3 experimental results:

- V3-006A: Impersonation rejected → broken chain, not in `G`.
- V3-006B: Conflicting QCs → one survives, the other dies in `G`.
- V3-006C: Stale certificates → belong to a dead branch of `G`.
- V3-006D: Foreign authorities → not rooted at `P₀`.
- V3-007A/B: Amendments → create forks; continuity selects the winner.
- V3-007C: Epoch transition → advances the maximal path.
- V3-007D: Recovery → rediscovers the maximal path from evidence.

---

## 6. The Final Picture (v0.6)

CCS is not a theory of:
- Consensus
- Cryptography
- Blockchain
- Canonicalization functions

CCS is a theory of **Constitutional Continuity.**

Given a possibility space `G`, the constitutional rules define a measure
of continuity. The canonical authority chain `Λ` is simply the path
that maximizes this measure.

`Canonicalize` is not an axiom. It is a **theorem** of constitutional
fitness.

---

## 7. Next Steps (V6-005)

1. Formalize `ConstitutionalContinuity` as a measurable quantity.
2. Prove that exactly one path maximizes it in any valid `G`.
3. Derive all previous axioms (Uniqueness, Monotonicity, Conservation)
   from this single principle.
4. If successful, CCS achieves theoretical closure:
   all properties of constitutional authority follow from the
   maximization of constitutional continuity.

---

## 8. Conclusion

The search for the central object of CCS may be over.

It is not `Authority`.
It is not `Context`.
It is not `Canonicalize`.

It is **Constitutional Continuity.**

This is the conserved quantity that V5-002 sought.
This is the ordering principle that V6-001 and V6-002 needed.
This is the source of legitimacy, monotonicity, conservation, and
recovery.

If this principle holds, CCS has found its first law.
