# CCS v0.7 – V7-006: Constitutional Exclusion

**Date:** 2026-05-31
**Status:** Final Draft – The Irreducible Core

---

## The Question of V7-006

> What if the fundamental operation of a constitution is not to build,
> select, merge, or prune — but to **exclude**?

Every constitutional rule in AmunChain operates by exclusion:
- Stale evidence is excluded.
- Foreign evidence is excluded.
- Conflicting evidence leads to exclusion of one branch.
- Old epochs exclude previous authorities.
- Outdated validator sets are excluded.

The common denominator is not what survives.
The common denominator is **what is constitutionally impossible.**

V7-006 defines CCS around the primitive relation of
**Constitutional Exclusion.**

---

## 1. The Primitive Relation: Constitutional Exclusion (⇍_C)

`C₁ ⇍_C C₂`

means: "Context `C₂` is constitutionally excluded by context `C₁`."

A context is legitimate if and only if it has **not** been
constitutionally excluded from the genesis context.

---

## 2. Axioms of Constitutional Exclusion

### Axiom E1: Foundational Legitimacy
`P₀` is legitimate. Nothing excludes `P₀`.

`¬(P₀ ⇍_C P₀)`

### Axiom E2: Exclusion is Permanent
If `C₁ ⇍_C C₂`, then for all future contexts `C₃` reachable from `C₁`,
`C₂` remains excluded.

Exclusion is irreversible. Dead branches stay dead.

### Axiom E3: Exclusion Propagates Forward
If `C₁ ⇍_C C₂`, then any context derivable from `C₂` is also excluded
from `C₁`.

Exclusion closes the entire subtree. The branch is pruned at the root.

### Axiom E4: Legitimacy by Non-Exclusion
`Legitimate(C)` iff `¬(P₀ ⇍_C C)`

A context is legitimate if the genesis context has not excluded it.

### Axiom E5: Eventual Convergence
`lim_{t → ∞} |{ C : ¬(P₀ ⇍_C C) }| = 1`

Over constitutional time, the set of non-excluded contexts converges
to a singleton.

---

## 3. The Central Theorem

**Constitutional Exclusion Theorem:**

If the exclusion relation `⇍_C` satisfies E1-E5, then there exists
exactly one context `Λ` that is never excluded:

`∃! Λ : ¬(P₀ ⇍_C Λ)`

This `Λ` is the canonical constitutional authority.

It is not the "strongest" or "most preferred" context.
It is simply the only one left.

---

## 4. The Constitutional Pruning Operators (Revisited)

All pruning operators from V7-005 are now expressed as exclusion
generators:

- **Epoch Exclusion:** `Epoch(C₁) < CurrentEpoch ⇒ P₀ ⇍_C C₁`
- **Evidence Exclusion:** `Evidence(C₂) > Evidence(C₁) ⇒ C₂ ⇍_C C₁`
- **Conflict Exclusion:** `Conflicting(C₁, C₂) ⇒ C₁ ⇍_C C₂ ∨ C₂ ⇍_C C₁`
- **Validator Exclusion:** `ValidatorSet(C) ∉ ValidSets ⇒ P₀ ⇍_C C`

These rules generate the exclusion relation. The constitution is
the set of exclusion generators.

---

## 5. What This Means

CCS is a theory of **Constitutional Exclusion.**

The constitution does not:
- Create authority
- Build paths
- Select winners
- Merge branches
- Prune possibilities
- Reduce contexts

The constitution **excludes** everything that is constitutionally
impossible.

What remains is authority.
What remains is legitimacy.
What remains is the final chain.

---

## 6. The Final Structure

The central object of CCS is:

**⇍_C**

Constitutional exclusion.

All other concepts — derivability, reduction, pruning, continuity,
convergence — are derived from this single relation.

The constitution is a system of exclusion rules.
The canonical authority is the unique context that survives
exclusion.

---

## 7. Conclusion

CCS is a theory of **elimination.**

It does not build the future.
It makes most futures impossible.

Authority is not constructed.
Authority is what cannot be excluded.

This is the irreducible core of CCS.
