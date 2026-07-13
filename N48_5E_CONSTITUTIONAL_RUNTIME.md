# N48.5-E — Constitutional Runtime

## 1. Scope

This specification defines the Constitutional Runtime — the implementation
layer that executes the Constitutional Virtual Machine defined in N48.5-D.
Where the VM specification defines what must be enforced and in what order,
the runtime specification defines how enforcement is achieved within the
constraints of a production blockchain: gas economics, storage organisation,
proof generation, and the bytecode representation of constitutional programs.

The runtime is the bridge between the constitutional specifications
(N48.5-A through N48.5-D) and the operating AmunChain node.  It translates
resource operations into executable instructions, manages the pending execution
buffer, coordinates with the ConstitutionalResourceRegistry, and produces the
TransitionProofs that enable third-party verification.

## 2. Evidence Taxonomy

The runtime produces three distinct categories of evidence, corresponding to
three distinct categories of execution outcome.

### 2.1 ExecutionFailureEvidence

Produced when the runtime cannot complete a transaction due to resource
exhaustion, VM panic, stack overflow, handle leak, or any other condition that
prevents normal execution from reaching a clean termination.  ExecutionFailureEvidence contains the failure reason (out of gas, stack overflow, runtime abort,
unreachable resource), the contract ID, the block height, the transaction hash,
and the gas consumed before failure.  ExecutionFailureEvidence is always
accompanied by a full transaction revert.  No state changes are committed.
Execution failures are NOT constitutional violations — they are operational
failures that do not indicate an attempt to violate resource laws.

### 2.2 ConstitutionalViolationEvidence

Produced when the VM rejects a resource operation during Phase 3 because the
operation would violate a resource law (R1–R6, L1–L5, T1).  The evidence
contains the violated law identifier, the resource IDs involved, the contract
ID, the block height, and the transaction hash.  ConstitutionalViolationEvidence
is always accompanied by a full transaction revert.  ConstitutionalViolationEvidence is only produced when Phase 3 completes in full — an
interrupted Phase 3 produces only ExecutionFailureEvidence, because a partial
verification cannot determine with certainty that a violation existed.

### 2.3 InvariantViolationEvidence

Produced when a contract's declared invariant returns false during Phase 5.
The evidence contains the invariant's obligation ID, the contract ID, the
block height, the transaction hash, and the state root at the time of the
violation.  InvariantViolationEvidence is not accompanied by a state revert —
the resource operations were constitutionally valid, and the violation is
adjudicated by the N47 Verdict Engine.

### 2.4 Evidence Routing

All three evidence types are routed to the EvidenceArchive during Phase 6.
ExecutionFailureEvidence and ConstitutionalViolationEvidence are produced by
the runtime itself.  InvariantViolationEvidence is produced by the runtime
when an invariant fails.  Evidence\<Archived\> and Evidence\<Revoked\> are
produced as outputs of the archive and revoke primitives and are a fourth
category — operational evidence of resource lifecycle transitions.

## 3. Bytecode Representation

### 3.1 Constitutional Bytecode Format

Every constitutional program compiles to AmunBytecode — a stack-based
instruction format extended with resource-aware opcodes.  The bytecode is
organised into sections:

- **Header**: capability level, invariant count, entry point offset.
- **Code**: the compiled contract logic as a sequence of instructions.
- **Invariant Table**: offsets to each invariant function.
- **Type Table**: resource types used by the contract, with their archetypes
  and legal transformation targets.
- **Metadata Table**: contract ID placeholder, compiler version, source hash.

### 3.2 Resource-Aware Opcodes

The runtime extends a standard stack-based instruction set with opcodes that
operate on constitutional resources.  Each opcode that consumes or produces a
resource carries a resource handle — an index into the transaction's local
resource table — rather than embedding the resource data directly.

