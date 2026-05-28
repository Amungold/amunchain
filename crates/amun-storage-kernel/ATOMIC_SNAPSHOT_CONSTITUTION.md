# AMUNCHAIN ATOMIC SNAPSHOT FREEZE CONSTITUTION v1.0

## ARTICLE I: TEMPORAL ATOMICITY

### Section 1.1: Snapshot Isolation
A constitutionally valid snapshot MUST represent the exact state at a
single, well-defined WAL cutoff point. No write operation may be in-flight
during snapshot generation.

### Section 1.2: Freeze Protocol
Before snapshot generation begins:
1. All WAL writes are fenced (no new entries accepted)
2. All pending state mutations are flushed
3. The current state root and WAL sequence are atomically recorded
4. Snapshot generation begins from this frozen point
5. After completion, WAL writes resume normally

### Section 1.3: Atomic Cutoff Verification
The snapshot_cutoff_sequence and snapshot_cutoff_root in the manifest
MUST match the state at the freeze point exactly. Replay of WAL from
cutoff_sequence+1 to current MUST produce the claimed state_root
when applied to the snapshot's restored state.

## ARTICLE II: TEMPORAL INTEGRITY

### Section 2.1: No Mixed Temporal State
A snapshot SHALL NOT contain nodes from different WAL epochs or
generations unless those differences are explicitly committed in
the state at the freeze point.

### Section 2.2: Freeze Point Uniqueness
Within a single snapshot, exactly one freeze point exists.
All nodes belong to the same constitutional instant.

## ARTICLE III: IMPLEMENTATION

### Section 3.1: MVCC Snapshot Isolation
The recommended implementation uses Multi-Version Concurrency Control
(MVCC) with immutable persistent data structures. The freeze point
is simply a reference to the current root in the persistent structure.

### Section 3.2: Copy-on-Write Generation Freeze
For the SparseMerkleTree, the freeze is natural: the persistent
OrdMap already provides snapshot isolation. The freeze point is
the Arc::clone of the current root reference.

## ARTICLE IV: VERIFICATION

### Section 4.1: Atomicity Check
After snapshot generation, the verifier MUST confirm that the
snapshot's state_root matches the state at cutoff_sequence.
Any discrepancy invalidates the snapshot.

### Section 4.2: Temporal Drift Detection
If WAL writes were not properly fenced, the snapshot may contain
mixed temporal state. Implementations SHOULD detect this by
comparing the snapshot's node set against the expected node set
at the claimed cutoff point.
