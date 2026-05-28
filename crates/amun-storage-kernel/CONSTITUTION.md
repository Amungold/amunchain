# AMUNCHAIN STORAGE CONSTITUTION v1.0
## Supreme Authority - Immutable Laws of State

---

### PREAMBLE
This Constitution establishes the immutable laws governing state representation,
proof geometry, replay determinism, and lineage continuity. All implementations,
specifications, and amendments MUST comply. In conflict, this Constitution prevails
over all other documents, code, or configurations.

---

### ARTICLE I: CANONICAL STATE GEOMETRY

**Section 1.1: Tree Structure**
The canonical logical state is a full explicit 256-depth Sparse Merkle Tree.
Every path from root to leaf SHALL contain exactly 256 branch nodes.
No structural compression, child promotion, or path truncation is permitted.

**Section 1.2: Node Types**
- Leaf: { key_hash: [u8; 32], value_hash: [u8; 32], version: u64 }
- Branch: { left: NodeHash, right: NodeHash }
- Empty marker: Precomputed canonical empty ladder, terminal = [0u8; 32]

**Section 1.3: Canonical Empty Root**
The canonical empty root is the hash of the full 256-depth empty ladder,
computed via recursive AMUN_BRANCH_V1 domain-separated hashing.
This is a PROTOCOL CONSTANT for version 1.

**Section 1.4: Order Independence (Law 0)**
For any set S of (key, value, version) tuples, exactly one canonical state
root R exists. The root SHALL be identical regardless of insertion order.

**Section 1.5: Delete-Reinsert Identity**
root(T.delete(k).insert(k, v, ver)) == root(T.insert(k, v, ver))
Delete of nonexistent key SHALL be no-op.
Delete of last key SHALL return canonical empty root.

---

### ARTICLE II: PROOF CONSTITUTION

**Section 2.1: Fixed Proof Geometry**
All Merkle proofs SHALL contain exactly 256 ProofStep entries.
No proof with fewer or more steps is constitutionally valid.

**Section 2.2: Proof Types**
- Inclusion: Proves (key, value) exists in state
- Absence: Proves key does not exist, lexicographically smallest witness
- EmptyTree: Proves state is canonical empty tree

**Section 2.3: Witness Selection**
The absence witness SHALL be the lexicographically smallest leaf in the
divergent subtree. This is a constitutional choice for protocol version 1.

---

### ARTICLE III: REPLAY DETERMINISM

**Section 3.1: WAL as Constitutional Timeline**
The WAL SHALL be the authoritative record of all state transitions.
Every entry SHALL be cryptographically chained to its predecessor.

**Section 3.2: Replay Equivalence**
Replaying a valid WAL from genesis SHALL produce the identical state root
as the original execution. Divergence is a constitutional violation.

**Section 3.3: Epoch and Generation**
- Epoch: Monotonic non-decreasing
- Generation: Strictly increasing within epoch, resets to 0 on epoch transition

---

### ARTICLE IV: LINEAGE CONTINUITY

**Section 4.1: State Lineage Chain**
prev_state_root of next MUST equal state_root of prev.
The lineage chain SHALL be unbroken from genesis.

---

### ARTICLE V: NODEHASH UNIQUENESS

**Section 5.1: Uniqueness Invariant**
NodeHash uniquely determines Node contents. Same hash + different content
= constitutional crisis (hash collision). System MUST halt.

---

### ARTICLE VI: LOGICAL/PHYSICAL SEPARATION

**Section 6.1: Dual Geometry**
Canonical logical geometry SHALL remain full explicit 256-depth.
Physical compression MAY operate below the canonical layer, provided
no logical root, proof, or replay outcome is altered.

---

### ARTICLE VII: AMENDMENT

Articles I-VI are immutable for protocol version 1. Changes require
new PROOF_VERSION constant and explicit migration path.

---

### RATIFICATION
Ratified upon verification of all 10 Constitutional Theorems.
Signed: AmunChain Constitutional Assembly, Version 1.0
