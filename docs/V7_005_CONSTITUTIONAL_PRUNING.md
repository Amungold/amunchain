# CCS v0.7 – V7-005: Constitutional Pruning

**Date:** 2026-05-31
**Status:** Draft – The Final Model

---

## The Question of V7-005

> Is the emergence of a unique final authority better described by
> **pruning** dead branches, rather than by merging divergent paths?

V7-004 attempted to model CCS as a confluent reduction system — all
paths eventually converge to the same normal form. But experimental
evidence from AmunChain (V3-006B, V3-007A) shows that losing branches
do not converge. They become **invalid** and are ignored.

The constitution does not merge possibilities.
It **eliminates** all but one.

V7-005 reformulates CCS as a **Constitutional Pruning System.**

---

## 1. The Pruning Model

Given a set of legitimate constitutional contexts at time `t`:

`L_t = { C : Legitimate(C) at time t }`

The constitutional rules do not select a winner.
They **prune** those that cannot continue.

`L_{t+1} ⊆ L_t`

Over time, the set of legitimate contexts shrinks.
Eventually, only one remains.

The surviving context is the canonical authority `Λ`.

---

## 2. Axioms of Constitutional Pruning

### Axiom P1: Foundational Root
`L₀ = { P₀ }`

Initially, only the genesis context is legitimate.

### Axiom P2: Legitimate Expansion
Legitimate contexts can produce successors that are also legitimate.

If `C ∈ L_t`, then `Successors(C) ∩ L_{t+1}` may be non-empty.

### Axiom P3: Irreversible Pruning
Once a context becomes illegitimate, it never becomes legitimate again.

`C ∉ L_t ⇒ ∀ t' > t : C ∉ L_{t'}`

### Axiom P4: Eventual Convergence
`lim_{t → ∞} |L_t| = 1`

Over constitutional time, the set of legitimate contexts converges
to a singleton.

### Axiom P5: Pruning by Evidence
A context is pruned when:
- It is superseded by a context with stronger evidence.
- It belongs to a previous epoch.
- It is attested by an outdated validator set.
- It conflicts with a context that has greater quorum support.

Pruning is not arbitrary. It follows constitutional evidence rules.

---

## 3. The Central Theorem

**Constitutional Pruning Theorem:**

If the constitutional pruning rules satisfy P1-P5, then:

1. `L_t` is non-empty for all `t`.
2. `L_t` is non-increasing: `L_{t+1} ⊆ L_t`.
3. `lim_{t → ∞} L_t = { Λ }` — exactly one context survives.

The canonical authority `Λ` is the unique survivor of constitutional
pruning.

---

## 4. Experimental Consistency

All AmunChain v0.3 results are instances of constitutional pruning:

- **V3-006B (Conflicting QCs):** One branch survives; the other is
  pruned by evidence superiority.
- **V3-007A (Amendment forks):** Competing amendments enter `L_t`;
  only one survives past `L_{t+1}`.
- **V3-007B/C (Epoch transitions):** Contexts from old epochs are
  pruned from legitimacy.
- **V3-006C/D (Stale/Foreign):** Contexts without valid evidence are
  pruned immediately.
- **V3-007D (Recovery):** The survivor `Λ` is rediscovered from
  pruned history.

---

## 5. The Pruning Operators

Pruning is not a single rule. It is a family of operators:

- **Epoch Pruning:** `C ∈ L_t` but `Epoch(C) < CurrentEpoch ⇒ C ∉ L_{t+1}`
- **Evidence Pruning:** `C₁, C₂ ∈ L_t`, `Evidence(C₂) > Evidence(C₁) ⇒ C₁ ∉ L_{t+1}`
- **Conflict Pruning:** `C₁, C₂ ∈ L_t`, `Conflicting(C₁, C₂) ⇒` only one survives
- **Validator Pruning:** `C ∈ L_t`, `ValidatorSet(C) ∉ CurrentValidatorSets ⇒ C ∉ L_{t+1}`

Together, these operators reduce `L_t` to a singleton over time.

---

## 6. What This Means

CCS is a theory of **Constitutional Pruning.**

The constitution does not create authority.
The constitution does not merge possibilities.
The constitution does not predetermine outcomes.

The constitution **eliminates** everything that cannot survive
constitutional scrutiny.

Authority is not chosen.
Authority is not built.
Authority is **what remains after everything else is pruned.**

---

## 7. Conclusion

CCS is a theory of **elimination, not construction.**

The fundamental process is:
`L₀ → L₁ → L₂ → ... → { Λ }`

A sequence of pruning operations that reduce the constitutional
possibility space to a single survivor.

This is the model that matches all experimental evidence from AmunChain.
It is simpler than convergence, stronger than selection, and more
realistic than predetermined reduction.
