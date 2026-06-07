# N48.5-C — Constitutional Contract State Model

## 1. Scope and Dependency

This specification defines the state model for AmunChain constitutional
contracts.  It depends directly on the Constitutional Resource Model (N48.5-B)
and the Constitutional Programs Specification (N48.5-A).  It must be read
alongside both documents.

The state model answers one question: given that every value in AmunChain is a
first-class constitutional resource with linear ownership, cryptographic
provenance, and mandatory lineage tracking, how does a contract organise,
persist, and prove the correctness of its internal state across executions?

This specification does not define the virtual machine, which is deferred to
N48.5-D, or the execution runtime, which is deferred to N48.5-E.  It defines
the data model that both must implement.  The relationship between these
specifications is deliberately sequential: the state model constrains the VM's
design, and the VM constrains the runtime's implementation.

## 2. Design Constraints

Three constraints from N48.5-B drive every decision in this model.

**C1 — Every state element is a resource.**  There are no naked integers,
strings, or booleans in contract state.  Every value is either a primitive
resource (Asset, Evidence, Claim, Certificate, ConstitutionalAsset) or a
structured collection of resources.  This means that state queries, updates,
and proofs all operate on resources with well-defined algebraic semantics.

**C2 — State transitions are resource transformations.**  A contract execution
does not update a database in the traditional sense.  It consumes input
resources and produces output resources, obeying the algebraic laws of split,
merge, transform, consume, archive, and revoke defined in N48.5-B Section 9.
The runtime tracks every operation and records the resulting ResourceLineage for
each new resource.  No resource is ever modified in-place.  Every state change
produces a new resource version and consumes the previous version.

**C3 — State is independently verifiable.**  After every execution, a
third-party verifier must be able to reconstruct the pre-state from a
TransitionProof, re-execute the transition, and confirm that the post-state
matches — without access to the contract's internal memory or storage.  This is
the foundation for L1+ replay verification and for the constitutional evidence
chain that feeds N47.

## 3. The State as a Resource Forest

A contract's state is not a flat key-value store.  It is a forest of resource
trees, where each tree is rooted at a genesis resource and grows through
successive derivations.  This structure directly mirrors the resource lineage
model defined in N48.5-B.

Consider a governance contract with a treasury and a voting system.  Its state
forest would contain two trees.  The Treasury tree has an Asset\<Token\> genesis
resource that has been split into two active child assets, plus a
ConstitutionalAsset\<TreasuryCap\> that remains in its original genesis form.
The Governance tree has a ConstitutionalAsset\<VotingPower\> that was consumed
and transformed into a new version, along with a Claim that was consumed when
it was aggregated into a Verdict, which in turn produced an archived Certificate.

A resource tree is a linked structure where each node points to its parent via
ResourceLineage.parent_resource_ids.  The root of every tree is a genesis
resource with derivation equal to Genesis.  The contract's state root is the
Merkle root of all active resource IDs across all trees.

This forest structure has several important properties.  First, it makes
resource provenance explicit — walking from any leaf to its root reveals the
complete history of that resource.  Second, it enables efficient inclusion
proofs for any individual resource without traversing the entire state.  Third,
it isolates resource trees from each other, so that a transformation in the
Treasury tree does not require rehashing the Governance tree.

## 4. Contract Storage Layout

### 4.1 Logical Storage

Contract storage is organised into four logical sections: active resources,
historical resources, tree roots, and the state root.  Active resources are
those in the Active state and participate in the state root computation.
Historical resources are those in Consumed, Archived, Revoked, or
TransferredOut state and are retained for lineage verification and audit
purposes.  Tree roots map a human-readable tree identifier to the ResourceId
of the genesis resource at the root of that tree.  The state root is the Merkle
root of all active resource IDs.

### 4.2 Physical Storage Per Contract

Each contract maintains its own storage region, isolated from all other
contracts.  The physical storage is a materialised view of the subset of the
global ConstitutionalResourceRegistry that belongs to this contract.  When a
resource is created, it is inserted into both the contract's local storage and
the global registry.  When a resource is consumed, both the local and global
copies are updated.

Cross-contract resource movement requires a CrossContractTransferProof as
defined in N48.5-B Section 11.  The source contract's resource enters
TransferredOut state, and the target contract creates a new resource with
CrossContractSuccessor derivation that references the original resource's ID
in its lineage.

### 4.3 Global Registry Synchronisation

The ConstitutionalResourceRegistry maintains a global index of all resources
across all contracts.  Each contract's local storage is a materialised view of
the registry subset that belongs to that contract.  Synchronisation between
local and global storage is atomic at the block level: all resource operations
within a block are committed to both local and global storage as a single
atomic batch.

## 5. State Transitions as Resource Transformations

A contract execution is a function from a pre-state and a transaction to a
post-state, a set of evidence records, and a set of claims.  Internally, the
execution performs a sequence of resource algebra operations — transform,
consume, split, merge, archive, and revoke — each of which consumes one or
more active resources and produces one or more new active resources.

The runtime tracks every operation and records the resulting ResourceLineage
for each new resource.  The pre-state is the set of all active resources before
the transaction executes.  The post-state is the set of all active resources
after the transaction executes.  The difference between these two sets is
exactly the resources consumed (moved to historical) and produced (added to
active) during the transaction.

This model has a crucial property: the state transition is fully auditable from
the consumed and produced resource lists alone.  A verifier can confirm that
every consumed resource was in Active state before the transaction, that every
produced resource has a valid lineage referencing its consumed parents, and
that the post-state root is correctly computed from the remaining active
resources.

## 6. Pre-State and Post-State Proofs

### 6.1 State Root Computation

The state root is a Merkle commitment to the set of all active resources.  It
is computed by sorting all active resource IDs, computing a hash of each
resource's metadata (type, state, lineage version, and parent references), and
building a Merkle tree over these hashes.  Two contracts with identical active
resources must produce identical state roots.

### 6.2 Inclusion Proof

A verifier can prove that a specific resource exists in the contract's active
state by providing a ResourceInclusionProof containing the resource ID, its
metadata, a Merkle path of sibling hashes with direction indicators, and the
claimed state root.  The verifier hashes the metadata, walks the Merkle path,
and confirms that the resulting root matches the claimed state root.

### 6.3 Transition Proof

A verifier can replay a state transition by providing a TransitionProof
containing the pre-state root, the transaction, inclusion proofs for all
consumed resources, metadata for all produced resources, the claimed post-state
root, and any evidence records generated.  The verifier confirms that the
consumed resources exist in the pre-state, applies the transaction logic,
confirms that the produced resources match, computes the post-state root, and
compares it to the claimed value.  If all steps match, the transition is
verified.

This is the basis for L1+ replay verification: any third party can verify a
past execution without access to the contract's internal storage, using only
the TransitionProof and the contract's bytecode.

## 7. Tree Organisation and Garbage Collection

### 7.1 Tree Roots

Every genesis resource defines a new tree root.  The contract's storage
maintains a mapping from tree identifiers to the ResourceId of the genesis
resource at the root of each tree.  When a genesis resource is consumed and a
successor is produced, the tree root is updated to point to the successor.  If
the genesis resource is archived without a successor, the tree is removed from
the active set entirely.

### 7.2 Active versus Historical Resources

Only resources in Active state contribute to the state root and appear in the
active resource set.  Resources in Consumed, Archived, Revoked, or
TransferredOut state are historical and are stored separately.  Historical
resources do not affect the state root, but they are retained for lineage
verification and constitutional audit purposes.

### 7.3 Historical Resource Pruning

Historical resources may be pruned after they have been archived to the
constitutional evidence archive.  Resources in Archived state can be pruned
after the archive certificate is confirmed, which occurs one block after
archival.  Resources in Consumed state cannot be pruned if they are referenced
as a parent by any active resource — the lineage must remain traversable.
Resources in Revoked state can be pruned after the revocation evidence is
archived.  Resources in TransferredOut state can be pruned after the
CrossContractTransferProof is confirmed on the target contract.

Pruning is a storage-level operation that does not affect the state root, the
resource lineage, or any constitutional verification.  It is purely an
optimisation to bound storage growth.

## 8. Contract Initialisation and Genesis State

### 8.1 Contract Deployment

Deploying a contract creates its initial storage from the deploy transaction.
The genesis state includes an ExecutionAuthority resource for the contract
deployer, any genesis resources defined by the contract's initialiser, an empty
evidence log, and a state root computed from the genesis resources.

### 8.2 Genesis Resource Creation

Genesis resources are created with a ResourceLineage that has an empty parent
list, derivation type Genesis, version 1, and a parent hash equal to the zero
hash.  The ResourceId is computed from the deployment transaction hash, the
contract ID, the resource type, and version 1.  This ensures that genesis
resources are cryptographically bound to the deployment that created them.

### 8.3 Deterministic Initialisation

The initialisation function must be deterministic.  Given the same deploy
transaction, any node must produce identical genesis resources with identical
ResourceIds.  This is enforced by deriving ResourceIds from the transaction
hash, which is itself deterministic.

## 9. State Model and Resource Algebra Integration

Every state transition is a composition of the algebraic operations defined in
N48.5-B Section 9.  Split consumes one active asset and produces multiple active
assets with specified amounts.  Merge consumes multiple active assets of the
same type and produces one active asset.  Transform consumes one active resource
and produces one active resource through a function.  Consume is an alias for
transform.  Archive moves a resource from active to historical with Archived
state and produces evidence of archival.  Revoke moves a resource from active to
historical with Revoked state, requiring a certified RegistryAuthority.

The runtime enforces three invariants during every state transition.  First,
every consumed resource must be in Active state before the operation.  Second,
every produced resource must have a ResourceLineage referencing its consumed
parents.  Third, after all operations in a transaction complete, the state root
must be recomputed from the remaining active resources.

## 10. Interaction with the Constitutional Resource Registry

The contract's local storage and the global ConstitutionalResourceRegistry
interact at specific, well-defined touchpoints.

When a contract executes a resource transformation, the local storage is
updated immediately, and the global registry's consume_and_derive function is
called to record the consumption of the old resource and the creation of the
new one.  Both updates occur within the same transaction boundary.

When a contract initiates a cross-contract transfer, the local resource enters
TransferredOut state, and the global registry generates a
CrossContractTransferProof.  When the target contract receives the transfer,
it creates a new active resource with CrossContractSuccessor derivation, and
the global registry processes the transfer proof to validate the atomicity of
the operation.

When a contract archives a resource, the local resource moves to Archived state
and is transferred to historical storage.  The global registry records the
archival with the block height and produces Evidence\<Archived\> that is
inserted into the EvidenceArchive.

## 11. Determinism and Replay Guarantees

The state model guarantees deterministic execution through three properties.

First, Resource Idempotence.  ResourceId is derived from the transaction hash
and contract ID.  The same transaction executed twice produces resources with
the same IDs.  The runtime detects duplicate IDs and rejects the second
execution before any state changes occur.

Second, State Root Determinism.  The state root is a pure function of the
active resource set.  Two executions with identical pre-states and identical
transactions produce identical post-states and identical state roots.  There
is no source of non-determinism — no timestamps, no random number generators,
no external calls — within the state transition function.

Third, Transition Replayability.  Given a TransitionProof, any verifier can
replay the transition and confirm that the post-state root matches, without
access to the contract's internal state.  This is the foundation for L1+
replay verification and for the constitutional evidence chain that feeds N47.

## 12. The Verdict as an Evaluation Artifact

The Verdict occupies a unique position in the constitutional lifecycle.  It is
produced by the VerdictEvaluator from a set of Claims, and it serves as the
input to Certificate issuance.  However, the Verdict is not a resource in the
sense defined by N48.5-B.

A Verdict does not enter the ConstitutionalResourceRegistry.  It is not subject
to the formal resource laws.  It has no ResourceId, no ResourceOrigin, and no
ResourceLineage.  It is an evaluation artifact — a temporary, immutable record
of a constitutional assessment that exists only to be certified.

The rationale for this separation is that the resource graph should contain
only entities that have permanent constitutional significance.  A Verdict is
a judgment about a set of claims at a point in time.  The Certificate that
results from it is the permanent constitutional record.  Including Verdicts
in the resource graph would bloat the registry with intermediate artifacts
that have no independent existence beyond their role in producing Certificates.

This means that in the constitutional lifecycle — Execution to Evidence to
Claim to evaluation to Certificate — the Verdict is the only stage that does
not produce a resource.  Every other stage does.

## 13. Acceptance Tests

The state model must pass tests covering genesis resource creation with correct
version and derivation type, rejection of duplicate resource consumption,
state root changes after any resource transformation, identical state roots
from identical inputs, independent verification of TransitionProofs,
cross-contract transfer state transitions on both source and target contracts,
exclusion of archived resources from the active set, requirement of valid
CertifiedAuthority for revocation, non-impact of historical resources on state
root, tree root updates when genesis resources are consumed, and rejection of
duplicate ResourceIds.

## 14. Summary

The Constitutional Contract State Model defines a resource-oriented state
machine where contract state is a forest of resource trees rather than a flat
key-value store.  State transitions are resource transformations obeying the
algebraic laws of split, merge, transform, consume, archive, and revoke.  Every
state change produces a new resource version and consumes the previous one —
resources are never mutated in-place.  The state root is a Merkle commitment
to all active resources, enabling independent verification of any state
transition by any third party without access to contract internals.  Pre-state
and post-state proofs allow L1+ replay verification.  The model integrates
with the Constitutional Resource Registry for cross-contract transfers and
global resource tracking.  Determinism is guaranteed through resource
idempotence, state root determinism, and transition replayability.  The Verdict
is deliberately excluded from the resource graph, remaining an evaluation
artifact that feeds Certificate issuance without polluting the resource
registry with intermediate state.
