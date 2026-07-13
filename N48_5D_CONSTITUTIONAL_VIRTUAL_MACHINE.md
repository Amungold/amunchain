# N48.5-D — Constitutional Virtual Machine

## 1. Scope and Relationship to Prior Specifications

This specification defines the Constitutional Virtual Machine — the execution
layer that enforces all resource laws, lineage rules, transformation legality
constraints, and invariant checks defined in the four preceding specifications:

- N48.5-A: Constitutional Programs Specification (capability levels, program
  interface, compiler obligations)
- N48.5-B: Constitutional Resource Model (resource archetypes, capabilities,
  authorities, provenance, consumption, formal laws R1–R6)
- N48.5-C: Contract State Model (resource forests, state roots, transition
  proofs, tree organisation)
- N48.5-C1: Lineage Integrity, Identity & Transformation Legality (derivation
  indices, lineage rules L1–L5, transformation matrix T1, pruning rules P1,
  anchor stability A1)

The VM is the first component in the N48.5 series that is an executable program
rather than a specification.  Its purpose is to guarantee that no contract,
regardless of its capability level or the complexity of its internal logic, can
violate any of the constitutional laws that protect resource integrity.  If a
contract attempts a violation — consuming an already-consumed resource, deriving
a Certificate from an Asset, creating a lineage cycle, or bypassing an invariant
check — the VM must detect the violation, refuse the operation, and record
constitutional evidence of the attempt.

The VM does not define a new bytecode format, instruction set, or gas model.
Those are implementation details deferred to the runtime specification
(N48.5-E).  The VM defines the execution model that any compliant runtime must
implement.

## 2. Design Principles

Four principles govern the VM's design.

**Principle 1 — The VM is the enforcer, not the policy maker.**  All
constitutional laws are defined in N48.5-B and N48.5-C1.  The VM does not
create new laws.  It rejects any operation that would violate an existing law
and produces evidence of the rejection.

**Principle 2 — Resource operations are atomic.**  A contract execution is a
sequence of resource operations.  Each operation either succeeds completely or
fails with no side effects.  There is no partial consumption, no
half-completed split, and no resource left in an intermediate state.

**Principle 3 — The VM is deterministic.**  Given identical pre-state, identical
transaction, and identical contract bytecode, the VM produces identical
post-state, identical evidence, and identical gas consumption on every node.
There is no source of non-determinism — no wall-clock time, no random number
generator, no external oracle — accessible to contract code.

**Principle 4 — Violations are recorded, not silently suppressed.**  When the
VM rejects an operation for constitutional reasons, it does not revert the
entire transaction.  It records the violation as InvariantViolationEvidence,
commits any state changes that occurred before the violation, and returns the
evidence to the caller.  This ensures that attacks on the constitutional layer
are visible and auditable, not hidden in reverted transactions.

## 3. Execution Model

### 3.1 The Execution Loop

A contract execution proceeds through a fixed sequence of phases:

1. **Pre-validation.**  The VM verifies that the caller holds a valid
   CertifiedAuthority\<ExecutionAuthority\> for the target contract.  It loads
   the contract's pre-state from the ConstitutionalResourceRegistry and
   verifies the state root against the contract's stored commitment.

2. **Execution.**  The VM invokes the contract's execute function with the
   execution context, pre-state, and transaction.  The contract returns a
   proposed post-state and an execution output listing consumed and produced
   resources.

3. **Resource Law Verification.**  For every consumed resource in the proposed
   output, the VM verifies that the resource exists, is in Active state, and
   is owned by the contract.  For every produced resource, the VM verifies
   that its ResourceId is unique (Law R1), its lineage references valid
   consumed parents (Law L2, Law L4), its version is exactly parent version
   plus one (Law L3, Law R6), its parent hash matches the actual parent
   (Law L5), and its transformation is legal under the transformation matrix
   (Law T1).  The VM also checks for lineage cycles (Law L1).

4. **Invariant Verification.**  For L2+ contracts, the VM calls every
   invariant function declared by the contract.  Each invariant receives the
   post-state and returns a boolean.  Failures are recorded as
   InvariantViolationEvidence.

5. **Commit.**  If all resource law verifications pass (regardless of invariant
   results — invariants that fail produce evidence but do not block the
   commit), the VM updates the ConstitutionalResourceRegistry, updates the
   contract's state root, inserts evidence into the EvidenceArchive, and
   queues claims for verdict evaluation.

### 3.2 Atomicity Boundary

The atomicity boundary is a single resource operation, not the entire
transaction.  If a contract proposes five resource transformations and the
third one is illegal, operations one and two are committed, operation three is
rejected with evidence, and operations four and five are never executed.  The
contract's post-state reflects the first two transformations.  The violation
evidence records what was attempted and why it was rejected.

This design is intentional.  Rolling back the entire transaction on a
constitutional violation would make violations invisible — the attacker would
simply retry with a modified transaction until they found a path that avoided
detection.  By committing partial progress and recording the violation, the
system preserves a permanent record of the attempt.

### 3.3 Gas Accounting for Constitutional Operations

Resource law verification, invariant checking, and lineage validation consume
gas.  The gas costs are:

- ResourceId uniqueness check: O(log N) in the number of active resources.
- Parent existence and state verification: O(P) where P is the number of
  parents.
- Lineage cycle detection: O(D) where D is the depth of the lineage chain.
- Invariant evaluation: contract-defined, metered by the VM.
- Evidence generation: fixed cost per evidence record.
- Claim generation: fixed cost per claim.

The contract is charged for all gas consumed up to and including the operation
that was rejected.  This prevents denial-of-service attacks that submit
transactions designed to trigger expensive verifications and then revert.

## 4. Resource Operation Primitives

The VM exposes six resource operation primitives to contract code.  Each
primitive corresponds to an algebraic operation from N48.5-B Section 9, and
each is subject to specific constitutional checks.

### 4.1 split

