# N106 & N107: Constitutional Authority and Governance

## Overview

The N106 and N107 phases transform AmunChain's authority model from a static, hardcoded
trust anchor into a fully constitutional, versioned, and governed authority system. Before
these phases, the consensus layer relied on a single hardcoded test authority key derived
from the seed `0x42` repeated 32 times. After N106 and N107, the authority is loaded from
a genesis file, supports multiple versions, rotation, retirement, transition windows,
on-chain governance proposals, voting, execution, snapshot persistence, and WAL-based
crash recovery.

These phases build directly on N105, which established cryptographic validator identities
and signature verification. N106 replaces the hardcoded authority with a genesis-based
constitutional authority. N107 extends this into a full lifecycle management system with
governance, persistence, and recovery.

## Objectives

The phases achieve the following engineering goals. Remove all hardcoded cryptographic
material from the runtime, replacing it with genesis-loaded authority configuration.
Establish a constitutional authority as a first-class object with identity, versioning,
and lifecycle state. Create an authority registry that supports multiple concurrent
authority versions. Implement certificate epoch binding so every certificate records
which authority version issued it. Support authority rotation with activation heights
and grace periods. Enable on-chain governance proposals for adding, retiring, and
transitioning authorities. Build a voting registry with quorum detection and majority
determination. Create a governance execution engine that mutates the authority registry
only after successful votes. Add snapshot persistence for governance state so it
survives restarts. Implement WAL-based replay for crash recovery. Provide unified
snapshot-plus-WAL recovery for complete state restoration.

## Scope

The phases include the following concrete deliverables. The amun-authority-registry crate
containing ConstitutionalAuthority, AuthorityRegistry, GovernanceProposal, ProposalVotes,
ExecutionJournal, GovernanceState, GovernanceWal, and GovernanceRecoveryEngine types
and all associated logic. Modifications to amun-live-cluster to load the authority from a
genesis file, build an AuthorityRegistry at startup, and route all certificate verification
through the registry. Modifications to ValidatorCertificate to carry authority version and
authority ID fields, with a new issue_v2 constructor and updated signing payload. A
genesis_authority.json file stored in the live-cluster crate containing the authority
public key and version. Updates to the test cluster constructors and benchmark binaries
to use the genesis authority. A complete governance pipeline from proposal submission
through voting, quorum checking, execution, and journaling. Snapshot and WAL persistence
for governance state with deterministic recovery.

## Architecture

### Authority Bootstrap Flow

The system initializes authority trust through a well-defined bootstrap sequence. On
startup, the LiveValidator loads the genesis authority from a JSON file at the path
resolved by concat with CARGO_MANIFEST_DIR. The file contains the authority public key
and version number. From this, a ConstitutionalAuthority object is created and used to
seed an AuthorityRegistry. The registry is queried for its active authority, whose
public key is used for all certificate verification during the session.

### Certificate Epoch Binding

Every validator certificate now carries two additional fields: authority_version and
authority_id. The authority_version indicates which version of the authority issued the
certificate. The authority_id is a cryptographic hash of the authority public key,
providing a stable, collision-resistant identity. The signing payload includes both
fields, so a certificate signed by Authority V1 cannot be presented as if signed by
Authority V2. Verification uses registry.by_version to retrieve the correct authority
public key, rather than assuming the active authority.

### Authority Lifecycle

The authority registry supports a full lifecycle. An authority can be registered with a
version number. The active version is tracked and used by default. An authority can be
retired, marking it revoked so it cannot issue new certificates, while preserving its
public key for historical verification of existing certificates. A transition can be
scheduled between two versions, specifying an activation height and a grace period.
During the grace period, both the old and new authorities are valid, preventing network
splits during rotation.

### Governance Pipeline

The governance system introduces on-chain proposals. A GovernanceProposal is created
with a specific action: AddAuthority, ScheduleTransition, or RetireAuthority. The
proposal has a deterministic ID based on the proposer, action, and creation height.
Validators vote on proposals through GovernanceVote records stored in ProposalVotes.
Votes support Approve, Reject, and Abstain options. Quorum requires two-thirds of
validators to participate. Approval requires more approvals than rejections among
participating validators. The GovernanceExecutor enforces these rules and mutates the
registry only when a proposal passes. An ExecutionJournal prevents replay of previously
executed proposals.

### Persistence and Recovery

Governance state is serializable via postcard for snapshot persistence. The
GovernanceState can be snapshotted at any block height and restored later. A
GovernanceWal records every governance transaction with its block height. On replay,
the WAL reconstructs the exact governance state. The GovernanceRecoveryEngine combines
a base snapshot with WAL entries after the snapshot height, enabling full state
restoration after a crash. The recovery is deterministic: the same inputs always produce
the same recovered state.

## Design Principles

The authority is a constitutional object, not a configuration parameter. It has
identity, version, and lifecycle state tracked in a registry. Verification is always
version-aware. Certificate verification looks up the specific authority version that
issued the certificate, not the current active authority. Governance is consensus-backed.
Authority changes require on-chain proposals, voting, and execution through the
governance pipeline. State is recoverable. Governance state can be snapshotted and
replayed from WAL, ensuring no loss of governance decisions after a crash. Identity
is cryptographically derived. Authority IDs are BLAKE3 hashes of public keys.
Validator IDs are BLAKE3 hashes of public keys. There is no dependence on arbitrary
index numbers.

## Components

### ConstitutionalAuthority

The ConstitutionalAuthority struct defines a versioned authority with an ID derived
from its public key, an activation height, and a revocation flag. The ID is computed
as BLAKE3 of the public key with a domain separator. The struct is serializable and
supports equality comparison.

### AuthorityRegistry

The AuthorityRegistry maintains a BTreeMap of authority versions to
ConstitutionalAuthority objects. It tracks the active version and an optional
scheduled transition. Methods include register, activate, retire, revoke,
schedule_transition, valid_authorities_at, and can_issue_at. The registry is
queryable by version for historical certificate verification.

### ValidatorCertificate Extensions

The certificate gained authority_version and authority_id fields. The issue_v2
constructor accepts these fields and includes them in the signing payload. The
original issue constructor sets defaults of zero for backward compatibility. The
serialize_for_signing method includes the new fields, so tampering with the
authority version invalidates the signature.

### GovernanceProposal

A GovernanceProposal is created by a validator with a specific GovernanceAction.
The proposal ID is a deterministic hash of the proposer, action, and creation
height. Actions include AddAuthority, ScheduleTransition, and RetireAuthority.

### ProposalVotes

ProposalVotes tracks all votes for a single proposal. The submit_vote method
replaces any existing vote from the same validator, so only the latest vote
counts. The tally method counts approvals, rejections, and abstentions.
The reached_quorum method checks for two-thirds participation. The is_approved
method combines quorum and majority checks.

### GovernanceExecutor

The execute_governance function applies an approved proposal to the registry.
It checks for prior execution via the journal, validates quorum and majority,
then executes the action. AddAuthority creates and registers a new
ConstitutionalAuthority. ScheduleTransition validates both versions exist and
schedules the transition. RetireAuthority marks the authority as revoked.

### ExecutionJournal

The ExecutionJournal uses a BTreeSet of executed proposal IDs to prevent replay.
The mark_executed method records a successful execution. The is_executed method
checks whether a proposal was already processed.

### GovernanceState

GovernanceState holds all proposals, votes, and the execution journal. The
apply_transaction method processes incoming GovernanceTransaction records.
The finalize_block method checks all proposals and executes any that have
reached approval, returning the list of executed proposal IDs. The snapshot
and restore methods serialize and deserialize the complete state.

### GovernanceWal

The GovernanceWal is an append-only log of GovernanceWalRecord entries. Each
record contains a block height and a transaction. The append method adds
entries. The replay method applies all entries to a fresh GovernanceState.
The entries_since method filters entries by block height for partial replay.

### GovernanceRecoveryEngine

The GovernanceRecoveryEngine provides a single recover method that takes a
base snapshot with its height, a WAL, and replays only the entries after the
snapshot height. It then finalizes any newly approved proposals and returns
the complete recovered state.

## Consensus Impact

The N106 changes do not alter the consensus protocol directly. The authority
public key used for certificate verification is now loaded from genesis rather
than hardcoded. The N107 changes introduce governance transactions that can be
included in blocks. When a governance proposal reaches approval, the authority
registry is mutated. This affects which certificates are accepted and which
authorities can issue new certificates. All changes are deterministic and
replayable.

## Security Considerations

The hardcoded test authority seed is removed from all runtime paths. The
genesis authority is loaded from a file that must be provisioned before startup.
If the file is missing, the node panics immediately, preventing operation with
an unknown authority. Certificate verification always uses the specific
authority version that issued the certificate, preventing cross-version forgery.
Governance proposals require two-thirds quorum and majority approval. Executed
proposals are journaled and cannot be replayed. Retired authorities cannot
issue new certificates but their existing certificates remain verifiable.

## Failure Scenarios

If the genesis authority file is missing, the node fails at startup with a
clear error message. If a certificate references an unknown authority version,
verification fails. If a governance proposal does not reach quorum, it is not
executed. If a proposal is executed once, subsequent attempts return
AlreadyExecuted. If the node crashes, the governance state can be recovered
from the latest snapshot and WAL.

## Performance Characteristics

Authority lookup is O(log N) using BTreeMap. Governance state serialization
uses postcard for compact binary representation. WAL replay is O(N) in the
number of entries since the last snapshot. All operations are deterministic
and do not depend on system time or external resources.

## Testing Strategy

The amun-authority-registry crate contains 52 unit tests covering authority
creation, registry operations, rotation, transition windows, governance
proposals, voting, execution, journaling, snapshot persistence, WAL replay,
and unified recovery. The amun-live-cluster crate contains 7 integration
tests exercising multi-validator consensus with certificate loading,
authority verification, and block finalization. The full workspace test
suite runs hundreds of additional tests ensuring no regressions.

## Compatibility

The authority_registry crate is a new addition and does not break existing
interfaces. The ValidatorCertificate changes are backward compatible: the
original issue method sets new fields to default values. The governance
system is additive and does not alter existing consensus paths. The
genesis authority file is required for new deployments; existing test
configurations generate it automatically.

## Operational Notes

The genesis authority JSON file must be provisioned at the path expected
by the concat macro. The test cluster constructors generate this file
automatically. The authority public key in the file must match the key
used to sign validator certificates. For production deployments, the
genesis authority must be generated by a secure offline process and
distributed to all nodes before startup.

## Limitations

The current implementation uses a single genesis authority with no
multi-signature support. Authority rotation requires governance proposals
rather than automatic rotation. Certificate issuance in test clusters
still uses a hardcoded authority keypair for signing, though verification
is routed through the registry. The governance WAL is in-memory and does
not yet persist to disk between runs, though the snapshot mechanism
provides equivalent durability.

## Future Extensions

Replace the test authority keypair in certificate issuance with a proper
authority signing service. Implement automatic authority rotation based
on block height. Add multi-signature authority support for M-of-N
governance. Persist the WAL to disk for durability between restarts.
Add governance transaction inclusion in the mempool and block production.
Implement authority audit trails with on-chain event logging.

## Acceptance Criteria

The amun-authority-registry crate exists and all 52 tests pass. The
live-cluster loads authority from genesis and routes verification through
the registry. No hardcoded cryptographic material remains in runtime paths.
The ValidatorCertificate carries authority version and ID. The governance
pipeline supports proposals, voting, execution, and journaling. Snapshot
and WAL persistence work correctly. Unified recovery restores state
deterministically. The full workspace builds and tests pass.

## Conclusion

N106 and N107 transform AmunChain from a system with a single hardcoded
trust anchor into a constitutionally governed, versioned authority system.
The authority is now a first-class object with lifecycle management, the
certificate infrastructure is epoch-aware, and governance decisions are
made on-chain through a complete proposal, voting, and execution pipeline.
The state is recoverable from snapshots and WAL, ensuring durability across
restarts. These phases establish the foundation for constitutional governance
of all chain parameters, not just authority keys.
