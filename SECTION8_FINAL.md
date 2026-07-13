# 8. Formal Security Analysis and Correctness Arguments

## 8.1 Overview

This section presents a formal analysis of the security properties of the
AmunChain architecture. While Section 7 demonstrated empirical performance
and experimental validation, the purpose of this section is to establish the
theoretical foundations that justify the observed behavior.

The analysis focuses on four properties:

- **Execution Correctness** — every finalized state transition corresponds
  to a valid deterministic execution.
- **Replay Soundness** — an invalid state transition cannot generate a valid
  replay certificate.
- **Evidence Integrity** — every detected constitutional violation produces
  cryptographically committed evidence.
- **Byzantine Finality Safety** — conflicting blocks cannot both obtain valid
  finality certificates under standard BFT assumptions.

Together, these properties define the security guarantees provided by
evidence-backed constitutional finality.

## 8.2 System Model

We assume a partially synchronous distributed network consisting of a set of
validators V = {v₁, v₂, ..., vₙ}, where at most f validators may behave
arbitrarily (Byzantine). The protocol operates under the standard Byzantine
Fault Tolerance assumption n ≥ 3f + 1. Consensus decisions require a quorum
certificate containing signatures from at least 2f + 1 validators.

Validators execute deterministic constitutional programs and produce
TransitionProof objects that are distributed together with execution results.

## 8.3 Deterministic Execution

**Assumption 8.1 (Deterministic Runtime).** The constitutional execution
environment is deterministic. Specifically:

- No opcode may access wall-clock time, system time, or external timing
  sources.
- No opcode may invoke randomness or nondeterministic entropy sources.
- All external inputs required during execution are included within the
  ExecutionContext.
- The virtual machine execution semantics are deterministic.
- Resource registry operations are deterministic functions of their inputs.

**Lemma 8.1 (Deterministic Execution).** For any constitutional program P,
input state S, transaction T, and execution context C, Execute(P, S, T, C)
produces a unique execution result.

*Proof Sketch.* By Assumption 8.1, execution depends exclusively on
deterministic inputs: program bytecode, transaction payload, execution
context, consumed resources, and witness bundle. No source of nondeterminism
exists within the execution pipeline. Therefore all honest validators
executing the same inputs produce the same output state, evidence set,
claims, and metadata.

## 8.4 Replay Soundness

**Definition 8.1 (Replay Soundness).** Replay soundness holds if an invalid
state transition cannot generate a valid replay certificate.

**Theorem 8.1 (Replay Soundness).** Assuming collision resistance of Blake3
and correctness of witness construction, replay validation is sound.

*Proof Sketch.* A replay verifier reconstructs execution using the witness
bundle, consumed resources, produced resources, state commitments, and
operation log. Any modification to execution results changes either the
state root, witness root, proof hash, or evidence root. A forged transition
cannot pass replay validation without breaking the underlying cryptographic
assumptions.

## 8.5 Constitutional Law Enforcement

**Definition 8.2 (Constitutional Enforcement).** A constitutional resource
violation is any state transition that violates one or more constitutional
laws R1–R6 or Law X1.

**Theorem 8.2 (Pre-Commit Constitutional Enforcement).** Any attempted
violation of a constitutional resource law is detected before state
commitment.

*Proof Sketch.* Resource-law validation (Phase 3 in the VM execution
pipeline) is executed prior to state commitment (Phase 4). If a proposed
transition violates resource uniqueness (R1), lineage consistency (R2, R3),
version monotonicity (R6), transformation legality (T1), or cross-contract
transfer constraints (X1), the transition is rejected before post-state
commitment. Therefore no committed state may contain a resource-law
violation. Invariant violations detected after execution but before finality
(Phase 5) produce ConstitutionalEvidence objects and are recorded within the
evidence framework. Thus: resource-law violations are prevented; invariant
violations are detected and recorded.

## 8.6 Byzantine Finality Safety

**Theorem 8.3 (Byzantine Finality Safety).** Two conflicting constitutional
blocks cannot both obtain valid finality certificates unless more than f
validators are Byzantine.

*Proof Sketch.* Finality requires a quorum certificate containing at least
2f + 1 validator signatures. Any two valid quorums intersect in at least
f + 1 validators. Since at least one validator in the intersection must be
honest, conflicting blocks cannot both receive valid signatures. This reduces
to the standard Byzantine quorum intersection argument.

## 8.7 Replay-Backed Safety

Traditional BFT protocols guarantee agreement on proposed blocks.
Replay-backed consensus strengthens this guarantee by requiring validators
to independently verify execution correctness before voting.

**Corollary 8.3.1 (Replay-Backed Safety).** Assume: (i) Replay soundness
holds (Theorem 8.1); (ii) honest validators replay TransitionProof objects
before voting; (iii) honest validators refuse to sign transitions whose
replay result diverges from the claimed post-state. Then an invalid
execution cannot obtain a valid quorum certificate unless more than f
validators are Byzantine or replay verification itself is unsound.

*Proof Sketch.* Suppose a proposer distributes a block containing an invalid
execution. By Theorem 8.1, replay validation reconstructs the execution
result. The reconstructed result differs from the claimed post-state.
Therefore every honest validator rejects the proof and withholds its
signature. Since at most f validators are Byzantine, the proposer can
collect at most f signatures. However, finality requires at least 2f + 1
signatures. Therefore the block cannot obtain a valid quorum certificate.
Consequently, invalid executions cannot be finalized.

*Discussion.* Without replay validation, honest validators sign state-root
claims. With replay validation, honest validators sign execution correctness.
This transforms finality from agreement on state alone into agreement on
verifiable execution.

## 8.8 Evidence-Backed Finality

Traditional blockchains finalize agreement on state roots. AmunChain extends
finality by additionally committing: execution proof hash, witness bundle
root, evidence root, and replay validation outcome.

**Definition 8.3 (Evidence-Backed Finality).** Evidence-backed finality is
achieved when validators finalize both state agreement and execution
correctness evidence.

This property provides stronger guarantees than conventional state-root
consensus because finalized blocks carry independently verifiable proof of
correctness.

## 8.9 Security Guarantees Summary

Under the stated assumptions, AmunChain provides:

| Property | Guarantee |
|----------|-----------|
| Execution Correctness | Deterministic replay (Lemma 8.1) |
| Replay Soundness | Invalid proofs rejected (Theorem 8.1) |
| Evidence Integrity | Violations permanently recorded (Theorem 8.2) |
| Byzantine Safety | No conflicting finalized blocks (Theorem 8.3) |
| Replay-Backed Safety | Invalid executions cannot be finalized (Corollary 8.3.1) |
| Stateless Verification | Proof-based validation (Section 7.3) |
| Finality Correctness | Evidence-backed certificates (Definition 8.3) |

These guarantees collectively establish the security foundation of
constitutional replay-backed consensus.

## 8.10 Conclusion

The formal analysis demonstrates that AmunChain combines deterministic
execution, replay validation, constitutional law enforcement, and Byzantine
quorum safety into a unified security model. The resulting protocol provides
stronger correctness guarantees than traditional state-root voting systems
by requiring independently verifiable execution evidence as a prerequisite
for finality. Replay-backed consensus transforms finality from agreement on
state alone into agreement on verifiable execution.
