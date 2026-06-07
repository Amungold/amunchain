# CCS v0.4 – Theoretical Nucleus (V4-002 & V4-003)

**Date:** 2026-05-31
**Status:** Draft for Peer Review

## 1. The Fundamental Shift: From State to Context

Classical distributed consensus protocols (Paxos, Raft, PBFT, HotStuff) model the
system as:

> **State × Input → State'**

The core entity is the *state machine*, and the protocol ensures that all correct
replicas transition through the same sequence of states.

**Constitutional Computational Systems (CCS)** introduce a new primitive: the
**Constitutional Context**. The transition function becomes:

> **(Context, State) × Evidence → AuthorizedTransitions**

This shift means CCS is not merely a new consensus algorithm, but a
redefinition of computational authority itself. A transition is not valid
simply because a quorum of nodes agreed; it is valid because the agreement
occurred within a specific constitutional context that authorizes that
transition.

## 2. The Constitutional Space (ℂ)

We define the constitutional space `ℂ` as a **preorder category** where:

- **Objects** are `ConstitutionalContext` tuples: `C = (E, V, Q, H)`
  - `E`: Epoch
  - `V`: Validator set with public keys
  - `Q`: A valid Quorum Certificate for the current finalized state
  - `H`: The finalized state hash

- **Morphisms** `τ : C₁ → C₂` represent constitutional transitions. A morphism
  exists iff there is cryptographic evidence `E` that justifies it:
  
  `∃ E : Justifies(E, τ)`

  Valid morphisms include:
  - Validator set amendment (V3-007A)
  - Epoch transition (V3-007B)
  - Authority recovery (V3-007D)

## 3. Axioms of CCS

### Axiom 1: Context Dominance
`∀ Q, C : ValidCrypto(Q) ⇏ Authority(Q, C)`

Cryptographic validity is necessary but not sufficient. Authority requires
constitutional membership, correct epoch, and proper context.

*Empirical support:* V3-006A (Impersonation), V3-006C (Stale), V3-006D (Foreign).

### Axiom 2: Contextual Recoverability
`∃ F : F(Evidence, Context) = Authority`

Authority can be reconstructed from constitutional evidence and the current
context. It is not stored; it is derived.

*Empirical support:* V3-007D (Authority Recovery).

### Axiom 3: Authority Uniqueness
`∀ C ∈ ℂ, ¬∃ A1, A2 : A1 ≠ A2 ∧ Valid(A1, C) ∧ Valid(A2, C)`

Within the same constitutional context, authority is unique. There cannot be
two conflicting authorized state transitions.

*Empirical support:* V3-006B (Conflicting QC).

### Axiom 4: Epoch Supremacy
`Epoch(C₂) > Epoch(C₁) ⇒ Authority(C₁) < Authority(C₂)`

Constitutional authority is partially ordered by epoch. Evidence from an
earlier epoch is invalid in a later epoch, even if cryptographically valid.

*Empirical support:* V3-007B, V3-007C (Stale rejection after transition).

## 4. Constitutional Temporal Logic

To reason about CCS properties formally, we introduce a temporal logic where
the fundamental modality is not "eventually" or "always", but:

**`Authorized(τ, C)`** — transition `τ` is authorized in context `C`.

Key theorems expressible in this logic:

1. **Safety:** `□(Authorized(τ, C) → Unique(τ, C))`
   At all times, if a transition is authorized, it is the unique authorized
   transition for that context.

2. **Liveness:** `◇(Evidence(Q, V) → Authorized(commit, C))`
   Eventually, if a valid quorum certificate exists for the current validator
   set, a commit transition is authorized.

3. **Recovery:** `□(Evidence(Q, C) → ◇Authorized(recover, C))`
   Whenever constitutional evidence exists, it is always possible to
   eventually authorize a recovery transition.

## 5. Authority Ordering Theory

We define a partial order `≤` on `ℂ`:

`C₁ ≤ C₂` iff there exists a constitutional morphism `τ : C₁ → C₂`.

This ordering reflects the evolution of constitutional authority:

- `C₁ ≤ C₂` implies that `C₂` is a constitutionally valid successor of `C₁`.
- Epoch transitions, validator amendments, and recovery are all morphisms
  that advance this ordering.
- Stale or foreign evidence cannot create valid morphisms, preserving the
  integrity of the authority chain.

## 6. Next Steps

- **V4-005:** Formal TLA+ specification of the CCS model.
- **V4-006:** Mechanized proof of the Authority Uniqueness Theorem.
- **V4-007:** Paper draft: "A Mathematical Framework for Constitutional
  Authority in Distributed Computational Systems."
