# Constitutional Computational Systems (CCS)
## Core Specification v1.1 – Final Foundational Axioms

**Date:** 2026-05-31
**Status:** Theory Nucleus – Independent Foundational Axioms

---

## 1. Purpose and Scope

CCS is a theory of computational authority.

A CCS is a distributed system in which the authority to perform state
transitions is not derived from consensus alone, but from a deterministic
function of a **constitutional context**. Consensus serves as a mechanism
to produce verifiable **evidence** of that context.

This specification defines CCS without reference to any specific
implementation, consensus protocol, or cryptographic primitive.

---

## 2. Ontology (What Exists)

### 2.1 Constitutional Context
A **constitutional context** `C` is the fundamental entity of CCS.

`C = (E, V, H)`

where:
- `E` (Epoch): A totally ordered value representing constitutional time.
- `V` (Validator Set): A finite set of participants authorized to produce evidence.
- `H` (Finalized State Hash): A cryptographic commitment to the current state.

The set of all constitutional contexts is denoted `ℂ`.

### 2.2 Authority Object
An **authority object** `A` encapsulates the right to authorize state
transitions within a constitutional context.

`A = (E, V, Q, H)`

where `E`, `V`, `H` are as defined in the context, and `Q` (Quorum Evidence)
is a verifiable proof that at least a threshold number of members of `V` have
witnessed and attested to `H`.

The function `Authority : ℂ → A` maps each valid constitutional context
to its unique authority object. Authority is now **derived entirely** from
the context, eliminating any circularity in its definition.

### 2.3 Constitutional Evidence
**Constitutional evidence** is a verifiable proof that:
1. A specific set of validators `V` has attested to a state `H`.
2. The attestation was produced within epoch `E`.
3. The attestation meets the quorum threshold defined for `V`.

Evidence is the mechanism through which consensus contributes to CCS,
but it does not constitute authority in itself.

### 2.4 Constitutional Transition
A **constitutional transition** `τ : C₁ → C₂` is a morphism that maps one
constitutional context to another. A transition is legitimate iff it is
justified by constitutional evidence:

`∃ evidence : Justifies(evidence, τ)`

---

## 3. Axioms (Foundational Assumptions of the Theory)

### Axiom 1: Context Dominance
`∀ Q, C : ValidEvidence(Q) ⇏ Authority(Q, C)`

Cryptographic validity of evidence is necessary but not sufficient for
authority. Authority requires a valid constitutional context.

### Axiom 2: Constitutional Determinism (Foundational Axiom)
`∀ C : ValidContext(C) ⇒ ∃! A = Authority(C)`

For every valid constitutional context, there exists exactly one authority
object. Authority is not selected from alternatives; it is uniquely
determined by the context.

**This is the foundational axiom of CCS, not a proven theorem.**

### Axiom 3: Constitutional Recoverability
`∃ F : F(Evidence, Context) = Authority`

Authority can be reconstructed from constitutional evidence and context.
It is not stored locally; it is derived.

---

## 4. Dynamics (How the System Evolves)

### 4.1 Constitutional Context Validity
A context `C = (E, V, H)` is valid iff:

`ValidContext(C) iff ValidEpoch(E) ∧ ValidValidatorSet(V) ∧ ConsistentState(H)`

where:
- `ValidEpoch(E)`: `E` is the successor of a previously valid epoch.
- `ValidValidatorSet(V)`: `V` is a non-empty finite set.
- `ConsistentState(H)`: `H` is a fixed point of the state transition
  function authorized by `Authority(C)`.

### 4.2 Legitimacy
A claimed authority `A` is **legitimate** in context `C` iff:

`Legitimate(A, C) iff A = Authority(C) ∧ ValidContext(C)`

Legitimacy is not a matter of voting. It is a matter of identity with the
canonical authority determined by the context.

### 4.3 Constitutional Monotonicity
`C₁ → C₂ ⇒ Authority(C₁) ≤ Authority(C₂)`

Constitutional authority does not regress across legitimate transitions.
The ordering `≤` reflects epoch ordering and state advancement.

### 4.4 Constitutional Conservation
For every legitimate transition `τ : C₁ → C₂`, there exists a conserved
quantity: **Constitutional Continuity**.

Every legitimate context must be reachable from the initial constitutional
context `C₀` through a finite chain of legitimate transitions.

`∃ C₀, τ₁, ..., τₙ : C₀ → ... → Cₙ = C`

**Constitutional authority cannot appear spontaneously.**
It must have a traceable constitutional origin. This is the first
Conservation Law of CCS.

---

## 5. Theorems (Proven Consequences of the Axioms)

### Theorem 1: Authority Uniqueness
`∀ C : ValidContext(C) ⇒ (∃! A = Authority(C) ∧ ∀ A' ≠ A : ¬Legitimate(A', C))`

Within the same constitutional context, at most one authority can be
legitimate. Dual authority is impossible. This follows directly from
Axiom 2 (Constitutional Determinism).

---

## 6. Relationship to Instantiations

A CCS-compliant system must:
1. Provide concrete implementations of `Context`, `Authority`, `Evidence`,
   and `Transition`.
2. Satisfy all axioms.
3. Maintain Constitutional Monotonicity and Constitutional Conservation.

One such instantiation is the AmunChain protocol.

---

## 7. Conclusion

CCS is a theory of computational authority. It asserts that authority is not
produced by consensus, but is uniquely determined by constitutional context.
Consensus provides evidence; the constitution provides authority.
