# AMUNCHAIN SNAPSHOT CONSTITUTION v1.0
## Constitutional framework for distributed state snapshots

---

### ARTICLE I: SNAPSHOT AUTHORITY

**Section 1.1: Definition**
A snapshot is a cryptographically verifiable, self-contained representation
of the complete state at a specific lineage point.

**Section 1.2: Authority Requirements**
A snapshot is authoritative iff:
1. Corresponds to a valid StateLineage entry
2. Root hash matches lineage's state_root
3. All nodes verifiable against root
4. Manifest is cryptographically signed

**Section 1.3: Snapshot Manifest**
Every snapshot SHALL include:
- Lineage reference (epoch, generation, state_root)
- Node count and total size
- Chunk layout (offsets, hashes)
- Merkle root of chunk index
- Creation timestamp and version

---

### ARTICLE II: CANONICAL TRAVERSAL ORDER

**Section 2.1: Deterministic Ordering**
Nodes within a snapshot SHALL be ordered deterministically:
1. Sort by depth (0 to 256)
2. Within each depth, sort by NodeHash lexicographically (unsigned byte comparison)
3. Leaf nodes before Branch nodes at same depth

**Section 2.2: Rationale**
This ordering guarantees identical snapshot bytes regardless of:
- Insertion order history
- Platform (OS, CPU architecture)
- Implementation language
- Physical storage layout

**Section 2.3: Verification**
Two snapshots of the same logical state SHALL produce identical bytes
when serialized according to this order.

---

### ARTICLE III: CHUNK CONSTITUTION

**Section 3.1: Chunk Boundaries**
- Maximum chunk size: 16MB (constitutional constant)
- Chunks SHALL NOT split a single node across boundaries
- Chunk boundaries SHALL be determined by cumulative node sizes

**Section 3.2: Chunk Hashing**
- Each chunk individually hashed with blake3
- Chunk index: Merkle tree over ordered chunk hashes
- Snapshot root: Merkle root of chunk index

**Section 3.3: Chunk Proofs**
A chunk proof demonstrates a specific chunk is part of the snapshot
via Merkle proof from chunk hash to snapshot root.

---

### ARTICLE IV: PROOF-CARRYING SNAPSHOTS

**Section 4.1: Inclusion Proof**
Proves (key, value) exists within snapshot without full download.
Combines SMT Merkle proof (256 steps) with chunk index proof.

**Section 4.2: Consistency Proof**
Proves two snapshots represent same logical state via root equality.

**Section 4.3: Lineage Proof**
Proves snapshot belongs to constitutional lineage via:
1. Snapshot root == lineage state_root
2. Lineage chain unbroken to genesis
3. Each lineage entry's prev_root matches

---

### ARTICLE V: SYNC PROTOCOL

**Section 5.1: Sync Phases**
1. Manifest exchange: request manifest from peer
2. Manifest verification: verify against trusted lineage
3. Chunk request: request chunks in order
4. Chunk verification: verify each chunk against manifest
5. State reconstruction: rebuild SMT from chunks
6. Root verification: reconstructed root == manifest root

**Section 5.2: Resumable Sync**
If sync is interrupted, resume from last verified chunk.
Chunk verification is idempotent.

**Section 5.3: Partial Sync**
Nodes MAY sync only specific state subtrees by requesting
only chunks containing those nodes plus Merkle proofs for
the remaining state.

---

### ARTICLE VI: DETERMINISTIC STREAMING

**Section 6.1: Stream Order**
Chunks SHALL be streamed in canonical traversal order (Article II).
Receivers verify each chunk before requesting the next.

**Section 6.2: Parallel Download**
Multiple chunks MAY be downloaded in parallel provided:
1. Verification order follows canonical traversal
2. Chunk index is fully received before parallel download begins

**Section 6.3: Backpressure**
Sender SHALL respect receiver's requested rate.
Receiver MAY re-request dropped or corrupted chunks.

---

### ARTICLE VII: SNAPSHOT LIFECYCLE

**Section 7.1: Creation**
Snapshots created at epoch boundaries or after constitutional
number of generations (TBD by governance).

**Section 7.2: Retention**
At least one recent snapshot + complete WAL from that snapshot
forward SHALL be retained. Older snapshots MAY be pruned.

**Section 7.3: Expiration**
Snapshots older than constitutional epoch window MAY be
deleted after successor snapshot is verified and stored.

---

### ARTICLE VIII: REPLAY EQUIVALENCE

**Section 8.1: Snapshot + WAL Replay**
replay(snapshot) + replay(WAL_after_snapshot) == replay(full_WAL)

**Section 8.2: Verification**
After syncing a snapshot and the remaining WAL, the final
reconstructed root MUST equal the lineage's current state_root.

**Section 8.3: Divergence**
If snapshot replay + WAL replay diverges from full replay,
the snapshot is constitutionally invalid and SHALL be rejected.

---

### ARTICLE IX: CONSTITUTIONAL COMPLIANCE

**Section 9.1: Proof Geometry**
All proofs within snapshots SHALL comply with Article II of
the Storage Constitution (fixed 256-step geometry).

**Section 9.2: NodeHash Uniqueness**
All nodes within snapshots SHALL satisfy Article V of the
Storage Constitution (NodeHash uniquely determines content).

**Section 9.3: Empty Root**
The canonical empty root within snapshots SHALL match the
protocol constant defined in Article I, Section 1.3.
