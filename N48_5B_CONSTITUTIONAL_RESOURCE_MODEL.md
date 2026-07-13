# N48.5-B — Constitutional Resource Model, Capability System, Authorities & Lineage

## Abstract

This specification defines the Constitutional Resource Model for AmunChain
smart contracts.  Unlike traditional blockchains where assets are represented as
mutable entries in a global map, AmunChain treats all state-carrying entities as
first-class resources with compile-time enforced capabilities, cryptographic
provenance, mandatory verification lineages, and explicit constitutional
authorities.

A resource in AmunChain is not merely a value.  It is a constitutional entity
that carries origin, lineage, capabilities, certified authorities, an evidence
chain, transfer rules, consumption state, and cross-contract boundary proofs.

This model eliminates entire categories of smart-contract vulnerabilities —
double-spending, reentrancy, unauthorized minting, certificate forgery,
invariant bypass, authority forgery, and cross-contract resource leaks — by
making them compile-time errors rather than runtime exploits.

## 1. The Five Resource Archetypes

Every value manipulated by an AmunChain constitutional program belongs to
exactly one of five resource archetypes.  Each archetype has hardcoded
behavioural constraints that cannot be overridden by contract code.

### 1.1 Asset\<T\>

A fungible or non-fungible value owned by an account.

| Property         | Value                                          |
|------------------|------------------------------------------------|
| Ownable          | Yes                                            |
| Transferable     | Yes (requires TransferCapability)              |
| Replayable       | No                                             |
| Certifiable      | No                                             |
| Evidence-Backed  | No                                             |
| Cloneable        | No (must use split / merge for fungibles)      |
| Drop             | Requires explicit destroy or consume           |
| Consumption      | Can be consumed to produce derived resources   |

Token is not a separate archetype.  It is a payload carried inside Asset\<T\>.
This keeps all resources under a unified ownership and capability model.

### 1.2 Evidence\<T\>

A cryptographic proof that a specific execution produced a specific outcome.

| Property         | Value                                                |
|------------------|------------------------------------------------------|
| Ownable          | No                                                   |
| Transferable     | No                                                   |
| Replayable       | Yes                                                  |
| Certifiable      | Yes (can be referenced by a Claim)                   |
| Evidence-Backed  | Self-evident                                         |
| Cloneable        | No                                                   |
| Drop             | Requires explicit archive or reject                  |
| Consumption      | Once archived, state becomes Archived                |

### 1.3 Claim\<T\>

A constitutional assertion about the system state, backed by evidence.

| Property         | Value                                                   |
|------------------|---------------------------------------------------------|
| Ownable          | No                                                      |
| Transferable     | No                                                      |
| Replayable       | No                                                      |
| Certifiable      | Yes (feeds into Verdict)                                |
| Evidence-Backed  | Yes (must reference at least one Evidence\<T\>)         |
| Cloneable        | No                                                      |
| Drop             | Requires explicit withdraw or dismiss                   |
| Consumption      | Consumed when aggregated into a Verdict                 |

### 1.4 Certificate\<T\>

A constitutional verdict that is immutable, non-transferable, and permanently
archived.

| Property         | Value                                           |
|------------------|-------------------------------------------------|
| Ownable          | No (owned by the Constitution itself)           |
| Transferable     | No                                              |
| Replayable       | Yes                                             |
| Certifiable      | Self-certifying                                 |
| Evidence-Backed  | Yes (derived from Verdict)                      |
| Cloneable        | No                                              |
| Drop             | Requires explicit archive (permanent)           |
| Consumption      | Terminal — cannot be consumed further           |

### 1.5 ConstitutionalAsset\<T\>

A non-transferable, evidence-backed asset that represents a constitutional
right, identity, or credential.

| Property         | Value                               |
|------------------|-------------------------------------|
| Ownable          | Yes                                 |
| Transferable     | No                                  |
| Replayable       | Yes                                 |
| Certifiable      | Yes                                 |
| Evidence-Backed  | Yes                                 |
| Cloneable        | No                                  |
| Drop             | Requires explicit revoke            |
| Consumption      | Revoked assets become Revoked       |

## 2. Capability System

A resource's behaviour is not determined by boolean flags, but by capabilities
that must be held to perform restricted operations.  This is a compile-time
security guarantee.

### 2.1 Core Capabilities

TransferCapability, DestroyCapability, CertifyCapability, ReplayCapability,
ArchiveCapability, ExecuteCapability, ConsumeCapability, and
CrossContractCapability are all zero-sized types that serve as compile-time
proofs of authorisation.

### 2.2 Capability Assignment Matrix

| Resource              | Transfer | Destroy | Certify | Replay | Archive | Execute | Consume | CrossContract |
|-----------------------|----------|---------|---------|--------|---------|---------|---------|---------------|
| Asset\<T\>            | Owner    | Owner   | —       | —      | —       | Owner   | Owner   | Owner         |
| Evidence\<T\>         | —        | —       | Runtime | Runtime| Runtime | Runtime | Runtime | Runtime       |
| Claim\<T\>            | —        | —       | Runtime | —      | Runtime | Runtime | Runtime | Runtime       |
| Certificate\<T\>      | —        | —       | Self    | Verifier| Verifier| —       | —       | —             |
| ConstitutionalAsset   | —        | Issuer  | Issuer  | Runtime| Runtime | Issuer  | Issuer  | Issuer        |

A dash means the capability does not exist for that resource type.  Attempting
to use it produces a compile-time error.

## 3. Certified Constitutional Authorities

Every resource creation or privileged operation requires a Constitutional
Authority — and authorities themselves must be certified by the constitution
to prevent forgery.

### 3.1 The Authority Forgery Problem

If authorities were plain types, any developer could forge one.  AmunChain
prevents this by wrapping every authority in a CertifiedAuthority\<T\> that
carries a constitutional certificate proving the bearer's right to hold it.

### 3.2 Authority Types

ExecutionAuthority, GovernanceAuthority, ValidatorAuthority, TreasuryAuthority,
CertificationAuthority, RegistryAuthority, and CrossContractAuthority are all
zero-sized types implementing the AuthorityType trait.  They are wrapped in
CertifiedAuthority\<T\> which carries a Certificate\<AuthorityGrant\> and
a ResourceId for lineage tracking.

### 3.3 Authority Issuance

Only the constitutional certification runtime can construct a
CertifiedAuthority.  The constructor is pub(crate), and the struct contains
a private field that prevents external construction.  The Certificate wrapped
inside proves that the grant was legitimate.

### 3.4 Authority Binding

Executing a contract transition requires CertifiedAuthority\<ExecutionAuthority\>.
Minting tokens requires CertifiedAuthority\<TreasuryAuthority\>.  Issuing a
constitutional certificate requires CertifiedAuthority\<CertificationAuthority\>.
Transferring resources across contracts requires
CertifiedAuthority\<CrossContractAuthority\>.

## 4. Resource Identity

Every resource is globally identifiable through a cryptographic ResourceId
derived from its origin and type.  ResourceId is a Hash256 computed from the
transaction hash, contract ID, resource type, and version.  It is not a string
or a database identifier — it is a cryptographic commitment to the resource's
genesis.

## 5. Resource Provenance

Every resource carries a ResourceOrigin that records the block height,
transaction hash, contract ID, creator address, and timestamp of its creation.
This origin is set at creation and cannot be modified afterward.

Resources form a linear provenance chain: Execution produces Evidence, which
supports Claims, which are evaluated into Verdicts, which are certified into
Certificates.  Each resource holds the ResourceOrigin of its immediate
predecessor, allowing any verifier to walk from a Certificate back to the
original Execution.

## 6. Resource Consumption and Lifecycle State

Resources move through a lifecycle where consumption is explicit, tracked, and
irreversible.  ResourceState can be Active, Consumed (with the consuming
resource's ID), Archived (with the archive block height), Revoked (with the
revoking authority and reason), or TransferredOut (with the target contract
and transfer proof).

When an Asset is staked into a ConstitutionalAsset, the original Asset enters
Consumed state and its ResourceId is recorded in the new resource's lineage as
a parent.  The consumed asset cannot be spent again.

The lifecycle state machine allows resources to move from Genesis to Active,
and from Active to Consumed, Archived, Revoked, or TransferredOut.  Consumed
resources produce derived resources that become Active.  Archived and Revoked
states are terminal.  TransferredOut resources become Active on the receiving
contract.  Once a resource leaves Active state, it can never return.

## 7. Resource Lineage and Versioning

Resources evolve over time.  A staked asset becomes voting power.  A group of
claims becomes a verdict.  The system tracks every derivation through
ResourceLineage, which contains the resource's own ID, the parent resource IDs,
the derivation type, a monotonic version number, and a hash of the parent at
derivation time.

DerivationType covers Genesis (created from nothing), SingleAncestor (derived
from one parent), MultiAncestor (merged from multiple parents), Transformation
(transformed in-place), Split (one resource into many), Merge (many resources
into one), and CrossContractSuccessor (transferred across contracts).

Genesis resources start at version 1.  Every derivation increments the version
by exactly 1.  Immutable resources (Evidence, Certificate) freeze their lineage
at creation.  Mutable resources (Asset, ConstitutionalAsset) may have multiple
versions.  Version gaps are forbidden.

## 8. Formal Resource Laws

These laws transform the descriptive sections above into testable,
machine-verifiable predicates.  Every operation on resources must satisfy these
laws.  Violation of any law is a constitutional failure, recorded as evidence
and evaluated by the N47 Verdict Engine.

**Law R1 — Active Resource Uniqueness.**  No two distinct active resources may
share the same ResourceId.

**Law R2 — Consumed Resources Are Unusable.**  A consumed resource cannot be
the subject of any execution, transfer, or certification operation.  The runtime
must reject any transaction that attempts to operate on a consumed resource.

**Law R3 — Child Requires Consumed Parent.**  Every derived resource must have
all its parent resources in Consumed state, and each parent's consumed_by field
must point to the child.

**Law R4 — Certificates Are Terminal.**  A Certificate is always in Archived
state and can never be the parent of any derived resource.

**Law R5 — Cross-Contract Uniqueness.**  No resource can be simultaneously held
by two different contracts.  A resource transferred across contracts must enter
TransferredOut state on the source before becoming Active on the target.

**Law R6 — Lineage Version Monotonicity.**  Version numbers are strictly
monotonic.  Every non-genesis resource's version must be exactly one greater
than its parent's version.

## 9. Resource Algebra

The following algebraic operations define the formal semantics of resource
transformation.  These are the primitive operations that the Constitutional VM
must implement.

**split** consumes one Asset and produces N Assets with specified amounts.
Precondition: sum of amounts equals the original amount.  Postcondition: the
original asset is Consumed, each child is Active with Split derivation.

**merge** consumes N Assets of the same type and produces one Asset.
Postcondition: all inputs are Consumed, the output is Active with Merge
derivation.  merge(split(asset, amounts)) == asset.

**transform** consumes one resource and produces another through a function.
Postcondition: the input is Consumed, the output is Active with Transformation
derivation.

**consume** is an alias for transform that emphasises the parent becomes
Consumed.

**archive** moves a resource to Archived state and produces Evidence\<Archived\>.
Postcondition: the resource is terminal.

**revoke** moves a resource to Revoked state with a reason and requires a
certified RegistryAuthority.  Postcondition: the resource is terminal.

## 10. Constitutional Resource Registry

The registry is the global authority that enforces all formal resource laws.
No resource exists outside its scope.

The registry maintains two maps: active resources indexed by ResourceId, and
archived resources indexed by ResourceId.  Each entry carries metadata including
the resource type, state, lineage, origin, and owning contract.

The registry provides functions for registering genesis resources,
consuming and deriving new resources, archiving, revoking, processing
cross-contract transfers, and validating all formal laws.  Every state change
in the registry is recorded as evidence, creating a complete audit trail.

The registry itself is subject to constitutional invariants that enforce
Laws R1 through R3 at the system level.

## 11. Cross-Contract Resource Semantics

Resources created by one contract may need to be consumed or referenced by
another.  AmunChain requires a CrossContractTransferProof that cryptographically
proves a resource was consumed on the source contract before it can be
materialised on the target contract.

The proof contains the consumed resource ID, source and target contract IDs,
the block height of consumption, the source state root, a Merkle proof of
consumption, and a certified CrossContractAuthority.

The transfer is atomic at the block level: Block N executes the source
contract's consumption (resource enters TransferredOut), and Block N+1 executes
the target contract's materialisation (resource becomes Active with
CrossContractSuccessor derivation).  If the target transaction fails, the
resource remains TransferredOut and can be retried.  It cannot be double-spent.

## 12. Invariant Taxonomy and Placement

LocalInvariant covers a single field, checked every transition by the contract
runtime.  StateInvariant covers a single contract's state, checked every
transition by the contract runtime.  EconomicInvariant spans multiple contracts,
checked periodically by the N47 Verdict Engine.  ConstitutionalInvariant covers
the entire system, checked every block by the N47 Verdict Engine.

Economic and Constitutional invariants belong in the N47 layer — not inside
individual contract runtimes — because they depend on state that spans multiple
contracts and can only be evaluated consistently after block execution.

## 13. Resource Lifecycle and Verification Lineage

Execution produces Evidence, which supports Claims, which are evaluated into
Verdicts, which are certified into Certificates.  No stage can be skipped.  No
stage can be reversed.  The type system enforces this at compile time:
Evidence can only be constructed from Execution, Claims only from Evidence,
and Certificates only from Verdicts with a CertificationAuthority.

## 14. Integration with N47

The resource model feeds N47 at every level.  Contract invariant declarations
populate the ObligationRegistry.  Evidence from L2+ executions fills the
EvidenceArchive.  Claims from L3+ contracts feed the VerdictEvaluator.
Aggregated verdicts become ConstitutionalVerdicts.  Certificates become
artifacts in the PublicationPackage.

## 15. Security Guarantees

No double-spending (resources are linear, cannot be cloned).  No unauthorized
transfer (TransferCapability required).  No certificate forgery (Certificate
needs Verdict plus CertificationAuthority).  No evidence fabrication (Evidence
needs Execution).  No claim without evidence (Claim constructor requires
Evidence reference).  No authority forgery (CertifiedAuthority requires
Certificate\<AuthorityGrant\>).  No privilege escalation (authorities are
compile-time types with private constructors).  No reentrancy (resources cannot
be recursively accessed).  No provenance erasure (ResourceOrigin is immutable).
No lineage gaps (ResourceLineage enforces version continuity).  No invariant
bypass (invariants checked by runtime, not contract code).  No cross-contract
leaks (CrossContractTransferProof plus TransferredOut state).  No unconsumed
derivations (parent must be Consumed before child is Active).  No duplicate
active IDs (Law R1 enforced by the registry).  No consumed resource usage
(Law R2 enforced by the runtime).

## 16. Summary

The Constitutional Resource Model transforms AmunChain smart contracts from
code that modifies a key-value store into verifiable constitutional programs
whose outputs are resources with linear ownership, cryptographic provenance,
capability-based access, certified constitutional authorities, mandatory
evidence lineages, explicit consumption semantics, versioned resource evolution,
cross-contract transfer proofs, formal resource laws, a resource algebra with
algebraic correctness guarantees, a global registry that enforces all laws,
and scoped invariants integrated with the N47 framework.

This is not an incremental improvement over existing platforms.  It is a
category shift — from assets-as-entries to assets-as-resources, from post-hoc
auditing to compile-time prevention, from contracts-as-code to
contracts-as-constitutional-entities, and from implicit permissions to explicit,
certified constitutional authorities.
