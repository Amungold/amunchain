# AmunChain: Evidence-Backed Finality and Replay-Backed Consensus

## Abstract

We present AmunChain, a framework for evidence-backed finality where consensus
attests not merely to agreement on state, but to the verifiable correctness of
execution.  Every transaction produces a TransitionProof — a portable,
replayable record binding execution context, state transition, gas consumption,
and constitutional evidence into a single cryptographic commitment.  Validators
do not vote on state roots alone; they vote on five independent roots (State,
Proof, Replay, Evidence, QC), producing a ConstitutionalFinalityCertificate
that any third party can independently verify without access to validator
state.  We formalise the resource model, the replay-backed consensus protocol,
and the five-root finality construction, proving replay soundness, finality
soundness, and Byzantine safety under standard BFT assumptions.  A Rust
reference implementation across 45+ crates demonstrates 788K TPS on
microbenchmarks with sub-millisecond replay verification and 11/11 Byzantine
attack vectors defeated at the protocol level.

## 1. Introduction

Blockchain consensus protocols converge on a model where validators vote on
state roots.  A quorum certificate attests that a supermajority agrees on the
outcome of a set of transactions.  This provides safety and liveness under
Byzantine fault assumptions.

It does not provide evidence that the transactions were executed correctly.
A quorum certificate over a state root tells a verifier that N validators
claim the root is correct.  It does not tell the verifier why it is correct,
nor provide the means to independently confirm correctness without
re-executing the entire block from genesis.

This gap has practical consequences.  Light clients trust validator majorities.
Audit firms re-execute from scratch.  Cross-chain bridges rely on trusted
relays.  In each case, the lack of portable, verifiable execution evidence
forces reliance on trusted third parties.

AmunChain addresses this gap by making execution evidence a first-class citizen
of the consensus protocol.  Every transaction produces a TransitionProof.
Validators replay and verify every proof before voting.  The resulting finality
certificate binds five independent roots into a single cryptographic commitment
that any third party can verify.

The contribution of this work is threefold:

1. **A framework for evidence-backed finality** where consensus attests to
   execution correctness rather than state agreement alone.

2. **A replay-backed consensus protocol** where deterministic replay is a
   precondition for voting, transforming replay from an audit mechanism into
   a consensus requirement.

3. **A constitutional resource model** with formal laws (R1–R6) governing
   resource identity, lineage, consumption, and cross-contract transfer,
   enforced at runtime by the VM and verified at block boundaries by a
   constitutional verifier.

## 2. System Model

### 2.1 Network and Adversary

We assume a partially synchronous network with a Byzantine adversary controlling
at most f < n/3 validators.  The adversary may delay, reorder, or drop messages
arbitrarily before GST (Global Stabilization Time).  After GST, messages between
honest validators are delivered within a known bound Δ.  The adversary cannot
forge signatures of honest validators, invert cryptographic hash functions, or
violate the determinism of the execution engine.

### 2.2 Cryptographic Assumptions

Blake3 provides collision resistance and preimage resistance.  Ed25519 provides
existential unforgeability under chosen-message attack.  Merkle trees inherit
collision resistance from the underlying hash function.  Resource identifiers
are derived from transaction hashes, contract identifiers, type tags, and
version counters, making collision attacks equivalent to finding Blake3
preimages.

### 2.3 Execution Determinism (Assumption 1)

For identical PreState and Transaction, all honest validators produce identical
PostState and identical TransitionProof.  The VM has no access to non-
deterministic inputs — no wall-clock time, no random number generator, no
external oracle.  The execution function is a pure function of its inputs.
Blake3 is deterministic.  The TransitionProof is a deterministic function of
the execution output.

## 3. Constitutional Resource Model

Every state element in AmunChain belongs to one of five resource archetypes:
Asset\<T\>, Evidence\<T\>, Claim\<T\>, Certificate\<T\>, and
ConstitutionalAsset\<T\>.  Each archetype carries hardcoded behavioural
constraints enforced at compile time.  Resources are linear — they cannot be
cloned.  Every derivation consumes the parent and produces a new resource with
a cryptographic lineage.

Six formal laws govern the resource graph:

- **R1 (Uniqueness)**: No two active resources share the same identifier.
- **R2 (Unusability)**: A consumed resource cannot be used in any operation.
- **R3 (Parental Consumption)**: Every derived resource's parents must be in
  Consumed state, with consumed_by pointing to the child.
- **R4 (Certificate Terminality)**: Certificates are always in Archived state
  and can never be parents.
- **R5 (Cross-Contract Uniqueness)**: No resource may be simultaneously held
  by two different contracts.
- **R6 (Version Monotonicity)**: Version numbers increase by exactly 1 at each
  derivation, with no gaps permitted.

These laws are enforced by the Constitutional Resource Registry at runtime and
verified by the N47 Verdict Engine at every block.  A transformation legality
matrix restricts which archetypes may derive into which others, preventing
illegal paths such as Evidence → Asset or Certificate → anything.

## 4. Transition Proofs and Replay Verification

A TransitionProof π is a portable cryptographic record: (tx_hash, contract_id,
block_height, pre_state_root, post_state_root, consumed_resources,
produced_resources, operation_log, evidence, gas_used, proof_hash).  The
proof_hash binds all other fields via Blake3.

A replay witness W contains the consumed resource metadata, produced resource
metadata, and Merkle proofs sufficient to reconstruct the pre-state.  Given
(π, W, bytecode), any third party can independently replay the transaction,
verify the state transition, confirm invariant compliance, and validate all
evidence — without access to the contract's internal storage.

## 5. Replay-Backed Consensus

Validators must replay and verify every TransitionProof before voting on a
block.  A ReplayBackedQC is valid iff (a) it carries at least 2f+1 distinct
validator signatures, and (b) for every π in the block, the replay verification
record confirms state_root_match, proof_hash_match, and gas_used_match.

A block is replay-backed final if there exists a valid ReplayBackedQC over it,
and every signing validator has independently replayed and verified every
TransitionProof in the block.

**Theorem 4 (Replay-Backed Safety).**  Assuming standard BFT quorum
intersection (any two 2f+1 quorums intersect in at least one honest validator),
no equivocation (honest validators sign at most one block per height), and
replay-before-vote (Definition 10), no two conflicting blocks can both become
replay-backed final.

## 6. Five-Root Finality

The ConstitutionalFinalityCertificate binds five independent roots: StateRoot
(post-execution state), ProofRoot (Merkle root of all π.proof_hash values),
ReplayRoot (Merkle root of all replay verification records), EvidenceRoot
(Merkle root of all constitutional evidence IDs), and QCRoot (hash of the
quorum certificate).

ProofRoot attests to the existence and cryptographic integrity of the
TransitionProofs.  ReplayRoot attests that those proofs were independently
re-executed by the validators and matched the deterministic execution function.
The two roots capture distinct properties: proof existence versus proof
verification.  A block may carry valid proofs that have not yet been replayed
(ProofRoot valid, ReplayRoot empty), but only blocks where both roots are
valid and the QC threshold is met become final.

**Theorem 5 (Evidence-Backed Finality Soundness).**  If C is a valid
ConstitutionalFinalityCertificate with Five-Root Binding, any verifier with C
can independently confirm that the block was executed, the execution was
proved, the proofs were replayed, the evidence was archived, and consensus was
reached.

A light client receiving C can verify the block's finality without syncing the
full state.  It needs only C, the TransitionProofs for transactions it cares
about, their replay witnesses, and the contract bytecode.  It replays each
proof, confirms the state transition, and verifies that the proof root matches
C.proof_root.  If all checks pass, the client knows the block was finalised by
a quorum of validators who independently verified the same execution.

## 7. Byzantine Resilience

We evaluated AmunChain against 11 Byzantine attack vectors (see Appendix C for
the full evaluation matrix).  All 11 were defeated at the protocol level.
Tampered proofs fail integrity verification.  Double transfers violate Law X1
and are rejected by the TransferProofRegistry.  Lineage cycles, version
regressions, and hash forgeries are caught by the Resource Registry.  Deep
lineage (2,000-depth) and wide fanout (10,000 genesis resources) do not crash
the system.  Insufficient quorum fails QC validation.

## 8. Performance

Microbenchmarks on a single machine (release build, Rust 1.85, Blake3 hardware
acceleration).  Full experimental methodology in Appendix B.

| Operation | Throughput |
|-----------|-----------|
| Halt program (minimal) | 788,875 TPS |
| Push5 program | 591,866 TPS |
| Replay verification | 905,045 replays/sec |
| Archive operations | 3,959,690 ops/sec |
| State root (10K active) | 3.0 ms |
| 50K resource lookups | 1.48M lookups/sec |
| Cycle detection (depth 5000) | 0.24 ms |

These results indicate that the constitutional overhead — resource lineage
tracking, invariant checking, evidence generation, and transition proof
construction — does not dominate execution cost.  The bottleneck in a deployed
network will be consensus message complexity and network latency, not
constitutional verification.

## 9. Related Work

| System | Resource Model | Execution Proofs | Replay Required | Evidence Finality |
|--------|---------------|------------------|-----------------|-------------------|
| Ethereum | Partial (ERC) | No | No | No |
| Solana | No | No | No | No |
| Aptos/Sui | Yes (Move) | No | No | No |
| Cosmos/Tendermint | No | No | No | No |
| Algorand | No | No | No | No |
| **AmunChain** | **Yes (5 archetypes)** | **Yes (TransitionProof)** | **Yes (QC precondition)** | **Yes (5-root certificate)** |

Ethereum provides no execution evidence beyond state roots.  Solana uses
Proof of History for ordering but does not produce portable execution proofs.
Aptos/Sui implement resource-oriented execution (Move) with linear types but
do not extend the resource model to constitutional evidence or replay-backed
consensus.  Cosmos/Tendermint provides BFT consensus with state root voting
but no execution evidence layer.  Algorand provides cryptographic sortition
for consensus but does not produce per-transaction execution proofs.

AmunChain's contribution is the integration of resource-oriented execution,
constitutional invariants, portable transition proofs, replay-backed consensus,
and evidence-backed finality into a unified architecture where finality is
proof of correct execution, not merely agreement on state.

## 10. Conclusion

We have presented a framework for evidence-backed finality where consensus
attests to execution correctness rather than state agreement alone.  The
TransitionProof enables portable, independent verification.  Replay-backed
consensus makes deterministic replay a precondition for voting.  The
ConstitutionalFinalityCertificate binds five roots into a single cryptographic
commitment that any third party can verify.  The reference implementation
demonstrates that this overhead is modest and that the system withstands
Byzantine attack.  Formal definitions, proofs, experimental methodology, and
the Byzantine evaluation matrix are provided in the appendices.

## Appendix B — Experimental Methodology

All benchmarks were executed on a single server: AMD EPYC 9634, 8 vCPUs,
16 GB RAM, NVMe storage, Ubuntu 24.04, Rust 1.85, release profile
(opt-level=3, lto=true, codegen-units=1), Blake3 with x86_64 hardware
acceleration.  Each benchmark was run 5 times.  Reported figures are the
arithmetic mean.  Standard deviation was below 5% of the mean for all
measurements.  TPS figures measure the constitutional runtime in isolation
excluding network, consensus, and state sync overhead.  Replay figures measure
end-to-end replay including witness reconstruction, registry initialisation,
execution, and result comparison.

## Appendix C — Byzantine Evaluation Matrix

| # | Attack | Mechanism | Result |
|---|--------|-----------|--------|
| 1 | Forged TransitionProof | Tampered post_state_root | Rejected: integrity check fails |
| 2 | Double cross-contract transfer | Same proof used twice | Rejected: Law X1, TransferProofRegistry |
| 3 | Lineage cycle injection | a→b, b→c, attempt c→a | Rejected: cycle detection (R1) |
| 4 | Version regression | Child version = parent version | Rejected: version monotonicity (R6) |
| 5 | Parent hash forgery | Claimed parent_hash ≠ actual hash | Rejected: hash integrity (R3) |
| 6 | Illegal transformation | Evidence → Asset derivation | Rejected: transformation matrix (T1) |
| 7 | Deep lineage bomb | 2,000-depth chain | Survived: ancestor cache, no crash |
| 8 | Wide fanout flood | 10,000 genesis resources | Survived: registry scaling |
| 9 | Proof tampering | Modified gas_used after construction | Rejected: proof integrity |
| 10 | Proof replay attack | Duplicate TransferProof | Rejected: single-use registry |
| 11 | Insufficient quorum | 2/5 signatures, threshold 5 | Rejected: QC validation |

## Appendix D — Formal Definitions and Proof Sketches

See Appendix A for the complete formal model: resource graph definitions,
formal laws R1–R6 and X1, execution and TransitionProof definitions, replay
witness construction, replay soundness (Theorem 2), replay verifiability
(Theorem 3), replay-backed finality soundness (Theorem 4), five-root binding
(Definition 13), evidence-backed finality soundness (Theorem 5), replay-backed
safety (Theorem 6), and cycle detection complexity (Theorem 7).

## Threats to Validity

**Internal validity.**  The benchmarks measure the constitutional runtime in
isolation.  They do not capture the effects of concurrent transaction execution,
parallel contract invocations, or contention on the resource registry.  These
factors may reduce throughput in a multi-validator deployment.

**External validity.**  The implementation is a single Rust codebase.  Results
may not generalise to other languages or execution environments.  The Byzantine
resilience claims depend on the correctness of the Rust implementation, the
underlying cryptographic libraries, and the Blake3 and Ed25519 primitives.

**Construct validity.**  We measure TPS as the number of complete execution-
to-TransitionProof pipelines per second for a minimal (Halt) program.  Real
contracts with complex logic, large state, or many resource operations will
have lower throughput.  The TPS figures should be interpreted as an upper bound
on computational capacity, not as a prediction of production network throughput.

**Reproducibility.**  All source code, test suites, benchmark harnesses, and
attack scenarios are available in the AmunChain repository.  The specific
commit hash, Rust toolchain version, and build configuration used for the
reported measurements are documented in the repository's reproducibility
manifest.  Re-running the full benchmark suite requires approximately 5 minutes
on comparable hardware.
