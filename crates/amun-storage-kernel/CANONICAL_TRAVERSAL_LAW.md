# AMUNCHAIN CANONICAL TRAVERSAL LAW v1.0
## Formal specification of deterministic state traversal

---

## ARTICLE I: TRAVERSAL PRIMACY

**Section 1.1: Purpose**
This Law defines the canonical order for traversing, serializing, and
committing state. All implementations SHALL produce identical byte
sequences when traversing identical logical state.

**Section 1.2: Scope**
This Law governs:
- Snapshot serialization order
- Chunk boundary determination
- Proof step ordering within Merkle proofs
- WAL entry ordering
- State root computation ordering
- Any operation producing deterministic output from state

---

## ARTICLE II: DEPTH-FIRST TRAVERSAL

**Section 2.1: Primary Order - Depth**
State traversal SHALL be depth-first, starting from depth 0 (root)
and descending to depth 256 (leaf level).

**Section 2.2: Branch Traversal**
At each Branch node:
1. Traverse left child subtree completely
2. Traverse right child subtree completely

**Section 2.3: Empty Subtree**
Empty subtrees (canonical empty ladder hashes) SHALL be skipped
during snapshot serialization. They are reconstructible from the
canonical empty ladder without explicit storage.

---

## ARTICLE III: LEXICOGRAPHIC ORDERING

**Section 3.1: Same-Depth Ordering**
When multiple nodes exist at the same depth, they SHALL be ordered
by NodeHash lexicographically (unsigned byte-by-byte comparison,
index 0 most significant).

**Section 3.2: Node Type Ordering**
At the same depth with the same parent:
- Leaf nodes SHALL precede Branch nodes
- This ensures leaves (terminal) are serialized before their
  containing branches when doing bottom-up traversal

**Section 3.3: Key Ordering Within Leafs**
Leafs at the same depth SHALL be further ordered by key_hash
lexicographically when NodeHash alone is insufficient (different
keys with different values produce different NodeHashes, so
NodeHash ordering is sufficient).

---

## ARTICLE IV: SERIALIZATION ORDER

**Section 4.1: Snapshot Order**
For snapshot serialization:
1. Collect all nodes reachable from the state root
2. Order by depth (0 to 256)
3. Within each depth, order by NodeHash lexicographically
4. Within same NodeHash (impossible per Article V of Constitution),
   order by key_hash lexicographically
5. Serialize each node in order

**Section 4.2: Chunk Determination**
After ordering, chunk boundaries are determined by cumulative
serialized size. A new chunk begins when adding the next node
would exceed MAX_CHUNK_SIZE (16MB).

**Section 4.3: Proof Step Order**
Merkle proof steps are collected top-down during traversal and
stored bottom-up (reversed). Step 0 is closest to leaf (depth 255
sibling), step 255 is closest to root (depth 0 sibling).

---

## ARTICLE V: DETERMINISM GUARANTEES

**Section 5.1: Platform Independence**
This traversal order SHALL produce identical results on all:
- CPU architectures (x86, ARM, RISC-V)
- Operating systems (Linux, macOS, Windows)
- Rust compiler versions (stable channel)
- Filesystem types

**Section 5.2: History Independence**
Traversal order SHALL depend only on the current logical state,
not on insertion history, deletion history, or any prior state.

**Section 5.3: Verification**
Two independent implementations traversing the same state root
SHALL produce bit-identical output. Any deviation indicates a
constitutional violation.
