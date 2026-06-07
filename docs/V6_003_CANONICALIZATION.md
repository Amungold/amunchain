# CCS v0.6 – V6-003: Canonicalization

**Date:** 2026-05-31
**Status:** Final Axiom – The Core of CCS

---

## The Question of V6-003

> Is authority the path itself, or the constitutional selection of a path
> from within a larger space of possibilities?

V6-001 established authority as a position.
V6-002 established authority as a unique path from genesis.

But experimental evidence from AmunChain (V3-007A) shows that
constitutional amendments can temporarily create multiple legitimate
proposals. The constitutional graph `G` contains forks. Yet the
system converges to a single canonical history.

This means authority is not merely the path. It is the process of
**selecting** which path becomes canonical.

---

## 1. The Two-Layer Model

### Layer 1: Constitutional Possibility Graph (G)
`G = (V, E)` is a directed graph where:
- `V`: All contexts that could possibly be reached.
- `E`: All transitions that are constitutionally valid at the time
  they are proposed.

`G` may contain forks, branches, and temporary ambiguities.
It represents **what is possible**.

### Layer 2: Constitutional Authority Chain (Λ)
`Λ ⊂ G` is a **directed path** from `P₀` to the current canonical
context.

`Λ` contains exactly one path. It represents **what is chosen**.

---

## 2. The Canonicalization Function

`Canonicalize : G → Λ`

This function selects, from all possible paths in `G`, a single
path that becomes the constitutional authority chain.

`Canonicalize` is:
- **Deterministic:** For a given `G`, `Λ` is uniquely determined.
- **Constitutional:** The selection follows the rules of the
  constitutional context (quorum, epoch, validator set).
- **Monotonic:** Once a context enters `Λ`, it cannot be removed.
- **Conservative:** `Λ` always begins at `P₀`.

---

## 3. Central Definition

**Constitutional Authority** is the canonical path `Λ` selected by
the constitutional rules from the possibility graph `G`.

`Authority = Canonicalize(G)`

---

## 4. Axioms (Complete)

### Axiom 1: Foundational Root
`∃! P₀ ∈ V(G)` with in-degree 0.

### Axiom 2: Constitutional Determinism (Final Form)
`∀ G : ∃! Λ = Canonicalize(G)`

For any constitutional possibility graph, there exists exactly one
canonical authority chain.

### Axiom 3: Path Conservation
`Λ ⊆ G` and `Λ` begins at `P₀`.

Every canonical context is reachable from genesis.

### Axiom 4: Constitutional Monotonicity
If `Λₜ` is the canonical chain at time `t`, and `Λₜ₊₁` at time `t+1`,
then `Λₜ` is a prefix of `Λₜ₊₁`.

`Λₜ ⊑ Λₜ₊₁`

Canonical history only grows; it never regresses.

---

## 5. Consequences

### 5.1 Legitimacy
A context `C` is legitimate iff it appears in `Λ`.

`Legitimate(C) ⇔ C ∈ Λ`

### 5.2 Conflicting Proposals
Multiple conflicting proposals may exist in `G`. But only one can
enter `Λ`. The others are valid possibilities that were not chosen.

This matches AmunChain experimental behavior:
- V3-006B: Conflicting QCs are in `G` but not in `Λ`.
- V3-006D: Foreign QCs are not even in `G` (invalid evidence).
- V3-007A: Constitutional amendments create forks in `G`.
- V3-007B/C: Epoch transitions advance `Λ`.

### 5.3 Recovery
Recovery (V3-007D) reconstructs `Λ` from evidence. It does not create
a new path; it rediscovers the canonical one.

---

## 6. The Final Picture

CCS is not a theory of consensus.
CCS is not a theory of cryptographic validity.
CCS is not a theory of blockchain.

CCS is a theory of **canonicalization**.

Given a space of possibilities `G`, constitutional rules select exactly
one canonical history `Λ`. This selection process is what we call
**constitutional authority**.

---

## 7. Conclusion

The constitutional constitution does not create authority.
It does not create paths.
It **chooses** which path becomes the legitimate history.

The central mathematical object of CCS is therefore:

`(G, Canonicalize)`

A possibility space and a deterministic selection function.
All other concepts — legitimacy, monotonicity, conservation, recovery,
uniqueness — are consequences of this structure.

This is the final form of the CCS foundational axioms.
