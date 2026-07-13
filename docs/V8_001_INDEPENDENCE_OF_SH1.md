# CCS v0.8 – V8-001: Independence of SH1

**Date:** 2026-05-31
**Status:** Active Investigation – The Counterexample Program

---

## The Question of V8-001

> Does SH1 (Single Constitutional History) follow from
> (D1-D2, E1-E4), or is it an independent axiom?

The current minimal core of CCS is the triple (⊢_C, ⇍_C, SH1).
But SH1 is qualitatively different from the other two:
- ⊢_C and ⇍_C are **relations** on contexts.
- SH1 is a **global constraint** on the entire constitutional space.

V8-001 attempts to determine whether SH1 is derivable from the
relational axioms, or whether it must be taken as primitive.

The method: search for a **minimal counterexample** — a model
satisfying D1-D2 and E1-E4, but violating SH1.

---

## 1. The Minimal Counterexample Challenge

Construct the smallest possible CCS model that:

1. Satisfies D1-D2 (derivability is reflexive and transitive).
2. Satisfies E1-E4 (exclusion is irreversible, closes subtrees,
   no self-exclusion, disjoint from derivability).
3. Violates SH1 (has two distinct, legitimate, final contexts).

---

## 2. Candidate: The 3-Context Model

Consider a constitutional space with exactly three contexts:

**Contexts:** `{ P₀, A, B }`

### Derivability (⊢_C):
- `P₀ ⊢_C P₀` (reflexive)
- `P₀ ⊢_C A`
- `P₀ ⊢_C B`
- `A ⊢_C A` (reflexive)
- `B ⊢_C B` (reflexive)
- No other derivations.

`A` and `B` are both derivable from genesis.
Neither derives the other.
Both are maximal (no further derivations from them).

### Exclusion (⇍_C):
- No exclusions at all. `⇍_C = ∅`.

### Legitimacy:
- `Legitimate(P₀)`: derivable from P₀, not excluded.
- `Legitimate(A)`: derivable from P₀, not excluded.
- `Legitimate(B)`: derivable from P₀, not excluded.

### Finality:
- `Final(A)`: legitimate, no further derivation from A.
- `Final(B)`: legitimate, no further derivation from B.

### SH1:
- `Legitimate(A) ∧ Legitimate(B) ∧ A ≠ B`
- `Final(A) ∧ Final(B)`
- But A and B are **not Resolvable**: neither derives the other,
  and they have no common derivable descendant.
- **SH1 is violated.** Two distinct, final, legitimate authorities
  coexist.

---

## 3. Verification of Axioms

### D1: Reflexivity ✅
`P₀ ⊢_C P₀`, `A ⊢_C A`, `B ⊢_C B`

### D2: Transitivity ✅
`P₀ ⊢_C A` and `A ⊢_C A` gives `P₀ ⊢_C A`.
`P₀ ⊢_C B` and `B ⊢_C B` gives `P₀ ⊢_C B`.
No other compositions are possible.

### E1: Irreversible exclusion ✅
No exclusions exist, so the condition holds vacuously.

### E2: Exclusion closes subtrees ✅
Vacuously true.

### E3: No self-exclusion ✅
Vacuously true.

### E4: Disjointness of ⊢_C and ⇍_C ✅
Vacuously true.

### SH1: ❌
Two distinct, legitimate, final contexts (A and B) coexist
permanently.

---

## 4. What This Means

**SH1 is independent of D1-D2 and E1-E4.**

The 3-context model is a valid CCS model (satisfying all relational
axioms) that exhibits permanent constitutional plurality.

Therefore, the minimal core of CCS cannot be reduced to (⊢_C, ⇍_C)
alone. SH1 is an **additional, independent axiom** — or it must be
derived from additional structural properties beyond D1-D2 and
E1-E4.

---

## 5. Implications for CCS

1. **SH1 is not a theorem of pure derivability + exclusion.**
   It is a separate commitment of CCS.

2. **CCS must either accept SH1 as primitive, or find additional
   structural properties that force it.**

3. **The 3-context model represents the simplest possible form of
   constitutional plurality.** Any real constitutional system that
   avoids this must have additional constraints.

4. **The boundary between convergent and divergent constitutional
   systems is now precisely characterized:**
   - Convergent systems satisfy SH1.
   - Divergent systems admit models like the 3-context example.

---

## 6. Next Steps

### Option A: Accept SH1 as Primitive
- CCS has three irreducible primitives: (⊢_C, ⇍_C, SH1).
- The theory is complete but includes a global constraint.

### Option B: Derive SH1 from Structural Properties
- Add properties to eliminate the 3-context model.
- Candidates: Comparability, Evidence Monotonicity, Epoch
  Supremacy, Constitutional Decidability.
- Each candidate must be tested against the 3-context model.

### Option C: Study the Space of Models
- Characterize all models satisfying D1-D2, E1-E4.
- Classify them into Convergent (SH1-holding) and Plural
  (SH1-violating).
- Identify the precise property that distinguishes them.

---

## 7. Conclusion

The 3-context model is a valid CCS model that violates SH1.
Therefore, SH1 is independent of the relational axioms.

This does not weaken CCS. It clarifies it.

CCS is not merely a theory of derivability and exclusion.
It is a theory of derivability, exclusion, **and the constitutional
impossibility of permanent plurality.**

SH1 is the axiom that makes CCS what it is.
