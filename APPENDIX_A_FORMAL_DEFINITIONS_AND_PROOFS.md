# Appendix A — Formal Definitions and Proof Sketches

## A.1 Resource Graph

**Definition 1 (Resource).** A resource is a tuple r = (id, type, state,
lineage, origin) where:

- id ∈ {0,1}^256 is a globally unique identifier
- type ∈ {Asset, Evidence, Claim, Certificate, ConstitutionalAsset}
- state ∈ {Active, Consumed, Archived, Revoked, TransferredOut}
- lineage = (parent_ids, derivation, version, parent_hashes)
- origin = (block_height, tx_hash, contract_id, creator, timestamp)

**Definition 2 (Resource Graph).** The resource graph G = (V, E) is a directed
graph where V is the set of all resources that have ever existed, and a
directed edge (u → v) exists iff v.lineage.parent_ids contains u.id.

**Definition 3 (Active Set).** Active(G) = { r ∈ V | r.state = Active }.

**Definition 4 (Consumed Set).** Consumed(G) = { r ∈ V | r.state = Consumed }.

**Definition 5 (Archived Set).** Archived(G) = { r ∈ V | r.state = Archived }.

## A.2 Formal Resource Laws

**Law R1 (Active Resource Uniqueness).**
∀ r1, r2 ∈ Active(G) : r1.id = r2.id ⇒ r1 = r2.

**Law R2 (Consumed Resources Are Unusable).**
∀ r ∈ Consumed(G) : ¬Executable(r) ∧ ¬Transferable(r) ∧ ¬Certifiable(r).

**Law R3 (Child Requires Consumed Parent).**
∀ v ∈ V, ∀ p ∈ v.lineage.parent_ids :
∃ u ∈ V : u.id = p ∧ u.state = Consumed ∧ u.consumed_by = v.id.

**Law R4 (Certificate Terminality).**
∀ r ∈ V : r.type = Certificate ⇒ r.state = Archived ∧ ¬∃ e ∈ E : e.source = r.id.

**Law R5 (Cross-Contract Uniqueness).**
∀ r ∈ V, ∀ c1, c2 ∈ Contracts :
holds(c1, r) ∧ holds(c2, r) ⇒ c1 = c2.

**Law R6 (Version Monotonicity).**
∀ v ∈ V : v.lineage.version > 0 ∧
(v.lineage.derivation = Genesis ⇒ v.lineage.version = 1) ∧
(v.lineage.derivation ≠ Genesis ⇒
 ∀ p ∈ v.lineage.parent_ids, ∃ u ∈ V : u.id = p ∧
 u.lineage.version + 1 = v.lineage.version).

**Law X1 (Transfer Proof Single Use).**
No CrossContractTransferProof may be consumed more than once.

## A.3 Execution and Transition Proofs

**Definition 6 (Execution).** An execution E is a function:
E : (PreState, Transaction) → (PostState, Evidence, Claims)
where PreState ⊆ Active(G), PostState ⊆ Active(G'), and G' is the resource
graph after applying the consumed and produced resources from the transaction.

**Assumption 1 (Execution Determinism).**
For identical PreState and Transaction, all honest validators produce identical
PostState and identical TransitionProof.  This holds because the VM has no
access to non-deterministic inputs — no wall-clock time, no random number
generator, no external oracle.  The execution function is a pure function of
its inputs, Blake3 is deterministic, and the TransitionProof is a deterministic
function of the execution output.

**Definition 7 (TransitionProof).** A TransitionProof π for execution E is a
tuple:
π = (tx_hash, contract_id, block_height, pre_state_root, post_state_root,
     consumed_resources, produced_resources, operation_log, evidence,
     gas_used, proof_hash)
where proof_hash = H(π \ {proof_hash}) and H is Blake3.

**Definition 8 (Replay Witness).** A replay witness W for TransitionProof π
is a tuple:
W = (consumed_resource_metadata, produced_resource_metadata, merkle_proofs,
     state_fragments)
where the metadata and Merkle proofs enable reconstruction of the pre-state
sufficient to re-execute the transaction.  The witness is self-contained:
it requires no access to the contract's internal storage or the validator's
state database.

**Theorem 1 (TransitionProof Integrity).**
For any TransitionProof π, if π.proof_hash = H(π \ {proof_hash}), then no
field of π has been tampered with after construction.

*Proof Sketch.*  H is collision-resistant (Blake3).  Any modification to any
field of π would change the hash with overwhelming probability.  The proof_hash
is the hash of all other fields.  Therefore, if π.proof_hash matches the
recomputed hash, no field has been modified.

## A.4 Replay Verification

**Definition 9 (Replay).** A replay R is the re-execution of a transaction
from the pre-state recorded in a TransitionProof, using a replay witness W,
producing a candidate post-state and candidate proof.  R is successful iff
the candidate post-state root equals π.post_state_root, the candidate proof
hash equals π.proof_hash, and the candidate gas_used equals π.gas_used.

**Theorem 2 (Replay Soundness).**
If a replay R is successful for TransitionProof π, then the recorded execution
is consistent with the deterministic execution function — that is, the
post_state_root, proof_hash, and gas_used recorded in π match the output of
the deterministic execution function applied to the recorded pre-state and
transaction.  Theorem 2 depends on Assumption 1.

*Proof Sketch.*  By Assumption 1, any honest execution of the same transaction
from the same pre-state produces identical output.  The replay re-executes the
transaction from π.pre_state_root using witness W and confirms that the output
matches π.  Therefore, π is consistent with the deterministic execution
function.  If π were forged, the forger would need to either produce an
execution output that matches the deterministic output without actually
executing the transaction (reducing to finding a Blake3 preimage for the
proof_hash), or violate Assumption 1 (assumed impossible).

**Theorem 3 (Replay Verifiability).**
Any third party with π, a replay witness W, and the contract bytecode can
independently verify the execution without access to the contract's internal
state.

*Proof Sketch.*  π contains pre_state_root and post_state_root.  W contains
the consumed resource metadata, produced resource metadata, and Merkle proofs
sufficient to reconstruct the pre-state.  The contract bytecode defines the
deterministic state transition function.  The verifier reconstructs the
pre-state from W, executes the bytecode against a fresh registry, applies the
transaction, and compares the resulting post_state_root to π.post_state_root.
All data required is contained in π and W.

## A.5 Replay-Backed Consensus

**Definition 10 (Replay-Backed QC).** A ReplayBackedQC over block B is valid
iff:
1. The number of distinct validator signatures ≥ quorum_threshold.
2. For every TransitionProof π in B, the replay verification record rv(π)
   confirms that state_root_match, proof_hash_match, and gas_used_match are
   all true.

**Definition 11 (Replay-Backed Finality).** A block B is replay-backed final
if there exists a valid ReplayBackedQC over B, and every validator contributing
a signature to the QC has independently replayed and verified every
TransitionProof in B.

**Theorem 4 (Replay-Backed Finality Soundness).**
If a block B is replay-backed final, then every honest validator who signed the
QC has independently verified that the execution of every transaction in B is
consistent with the deterministic execution function and produces the claimed
post-state.

*Proof Sketch.*  Follows from Definition 10 (which requires all replay
verification records to confirm all matches) and Theorem 2 (Replay Soundness).

## A.6 Evidence-Backed Finality

**Definition 12 (ConstitutionalFinalityCertificate).** A
ConstitutionalFinalityCertificate C is a tuple:
C = (block_height, block_hash, state_root, proof_root, replay_root,
     evidence_root, qc, certificate_hash)
where certificate_hash = H(C \ {certificate_hash}).

**Definition 13 (Five-Root Binding).** C binds five roots iff:
1. proof_root = MerkleRoot({π.proof_hash | π ∈ B.transitions})
2. replay_root = MerkleRoot({rv(π) | π ∈ B.transitions})
3. evidence_root = MerkleRoot({ev.evidence_id() | ev ∈ B.evidence})
4. state_root is the post-execution state root of B
5. qc is a valid ReplayBackedQC over B

**Theorem 5 (Evidence-Backed Finality Soundness).**
If C is a valid ConstitutionalFinalityCertificate with Five-Root Binding, then
any verifier with C can independently confirm that the block was executed
(state_root), the execution was proved (proof_root), the proofs were replayed
(replay_root), the evidence was archived (evidence_root), and consensus was
reached (qc).

*Proof Sketch.*  Each root is independently verifiable via the corresponding
Merkle tree.  The certificate_hash binds all five roots.  Any tampering with
any root changes the certificate_hash, which is detected by C.verify().

## A.7 Byzantine Resilience

**Theorem 6 (Replay-Backed Safety).**
Assuming:
1. Standard BFT quorum intersection (any two quorums of size 2f+1 intersect
   in at least one honest validator, with f < n/3).
2. Honest validators sign at most one block per height (no equivocation).
3. Replay verification is required before signing (Definition 10).

Then no two conflicting blocks can both become replay-backed final.

*Proof Sketch.*  If two conflicting blocks B1 and B2 both had ReplayBackedQCs
at the same height, the honest validator in the quorum intersection would have
had to sign both QCs.  But an honest validator replays every π before signing
(Assumption 3).  Since execution is deterministic (Assumption 1), replay
produces identical post_state_roots for B1 and B2 only if B1 and B2 are
identical.  If they differ, the honest validator would detect the divergence
during replay and would not sign the QC for the divergent block (Definition
10, condition 2).  This contradicts the existence of both QCs.

## A.8 Ancestor Cache Complexity

**Theorem 7 (Cycle Detection Query Complexity).**
Cycle detection query complexity is O(1) given a maintained ancestor cache.

*Proof Sketch.*  The ancestor cache for resource r is the set of all resources
on the path from r to genesis.  Cache construction has amortised O(depth)
cost per derivation.  Query complexity — checking whether a proposed new
resource would create a cycle — is O(1): it reduces to checking whether
new_id ∈ ancestor_set(parent), a single hash set lookup.

*Remark.*  Ancestor caching trades memory for query speed.  Worst-case storage
is O(|V|·D), where D is average lineage depth.  With the enforced maximum
depth of 2^16 and typical contract depths of 1–5, practical memory overhead
is modest.

## A.9 Assumptions and Limitations

**Cryptographic Assumptions.**  Blake3 provides collision resistance and
preimage resistance.  Ed25519 provides existential unforgeability under
chosen-message attack.  No novel cryptographic constructions are introduced.

**Network Assumptions.**  Partial synchrony with known bound Δ on message
delivery after GST.  Liveness depends on GST eventually arriving.  Safety
holds regardless of network timing.

**Adversary Model.**  The adversary controls at most f < n/3 validators.  The
adversary may delay, reorder, or drop messages before GST but cannot forge
signatures or invert hash functions.  The adversary may submit transactions
designed to exploit constitutional weaknesses (deep lineage, forged proofs,
illegal transformations) — all such attempts are caught by the resource law
enforcement layer.

**Limitations.**  The five-root finality certificate is as strong as its
weakest root.  A successful attack on any of the five roots would undermine
the corresponding guarantee.  The ancestor cache trades memory for speed;
deployments with extreme lineage depths should monitor cache growth.  Replay
verification requires the replay witness W; the size of W grows with the
number of resources touched by the transaction.

**Experimental Scope.**  All benchmarks were collected on a single machine
(release build, Rust 1.85, Blake3 with hardware acceleration).  They measure
computational throughput of the constitutional runtime in isolation.  Network
latency, consensus message complexity, and state synchronisation overhead are
not included in the TPS figures.
