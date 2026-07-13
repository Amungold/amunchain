# CCS v0.8 – Research Proposal: From Framework to Theory

**Date:** 2026-05-31
**Status:** Proposal – The Next Phase

---

## Where We Stand

The exploratory phase of CCS (V4–V7) has reduced dozens of concepts
to a minimal core:

**(⊢_C, ⇍_C, SH1)**

- **⊢_C**: Constitutional derivability (what can be built)
- **⇍_C**: Constitutional exclusion (what must be eliminated)
- **SH1**: Single Constitutional History (plurality is impossible)

This core is:
- Logically consistent
- Experimentally grounded in AmunChain v0.3
- Independent of specific consensus protocols
- Formulated as a set of axioms and a central theorem

What remains is the transition from **framework** to **theory**.

---

## The Two Paths Forward

### Path A: Mechanized Verification

**Goal:** Prove (or falsify) the central theorem using formal methods.

1. **V8-001A: TLA+ Specification**
   - Formalize (⊢_C, ⇍_C, SH1) in TLA+
   - Model-check the Closure Theorem
   - Search for counterexamples

2. **V8-002A: Coq/Lean Formalization**
   - Encode the axioms in a dependent type theory
   - Mechanize the proof of Closure from SH1
   - Extract a verified core

3. **V8-003A: Counterexample Search**
   - Attempt to construct a model satisfying D1-D2, E1-E4, SH1
     but violating Closure
   - If found: CCS needs additional axioms
   - If not found: evidence that SH1 ⇒ Closure

**Success criteria:** A machine-checked proof or a documented
counterexample.

### Path B: Theoretical Deepening

**Goal:** Derive SH1 from deeper principles, or replace it with
a weaker axiom.

1. **V8-001B: Deriving SH1**
   - Can SH1 be derived from (⊢_C, ⇍_C) plus structural properties?
   - Investigate: Well-foundedness, Finite Branching, Evidence
     Monotonicity, Constitutional Comparability

2. **V8-002B: The Counterexample Program**
   - Attempt to construct a CCS-compliant system that has permanent
     constitutional plurality
   - If successful: SH1 is independent and CCS has a fundamental
     incompleteness
   - If unsuccessful: SH1 may be derivable

3. **V8-003B: Constitutional Model Theory**
   - Study the class of all models satisfying (⊢_C, ⇍_C)
   - Characterize which models additionally satisfy SH1
   - Find the precise boundary between convergent and divergent
     constitutional systems

**Success criteria:** Either SH1 is derived from weaker axioms,
or a precise characterization of when it holds is established.

---

## The Central Decision

The next step depends on what kind of theory CCS aims to be:

**If CCS is a foundation for verified constitutional systems:**
→ Take Path A. Build TLA+ specs, mechanized proofs.

**If CCS is an explanatory theory of constitutional authority:**
→ Take Path B. Derive SH1, find counterexamples, characterize
  the boundary between convergence and divergence.

**If CCS is both:**
→ Start with Path A to validate the current axioms, then
  proceed to Path B to deepen them.

---

## Recommendation

Begin with **V8-001A: TLA+ Specification of the Minimal Core.**

This is the fastest path to a definitive result:
- If TLC finds a counterexample, CCS is falsified — and we learn
  what is missing.
- If TLC verifies the theorem, CCS has its first mechanized
  evidence of correctness.

Either outcome advances the research program.

---

## Conclusion

The exploratory phase of CCS is complete.
The minimal core is established.
The central theorem is conjectured.

The next phase will determine whether CCS is a framework for
describing constitutional systems, or a verified theory of
constitutional authority.
