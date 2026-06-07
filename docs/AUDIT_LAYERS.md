# Audit Layers

AmunChain uses 16 constitutional audit layers.

Every release must pass every layer.

---

# Layer 01 — Physics

Validates physical invariants.

Checks:

- hash determinism
- domain uniqueness
- serialization stability
- endian consistency

---

# Layer 02 — Geometry

Validates Merkle geometry.

Checks:

- proof depth
- empty ladder
- empty root consistency

---

# Layer 03 — Snapshot

Validates snapshots.

Checks:

- manifest verification
- identity determinism
- compatibility

---

# Layer 04 — Byzantine

Validates hostile behavior handling.

Checks:

- quorum logic
- identity mismatch rejection

---

# Layer 05 — Identity

Validates identity integrity.

Checks:

- tamper detection
- replay equivalence
- roundtrip stability

---

# Layer 06 — Replay

Validates deterministic replay.

Checks:

- replay equivalence
- divergence detection
- transcript consistency

---

# Layer 07 — Resources

Validates allocation safety.

Checks:

- allocation guards
- chunk limits

---

# Layer 08 — Domains

Validates domain infrastructure.

Checks:

- domain uniqueness
- versioning
- chain identity

---

# Layer 09 — Freeze

Validates frozen constants.

Checks:

- max depth
- proof version
- root stability

---

# Layer 10 — Adversarial

Validates adversarial behavior.

Checks:

- malformed proofs
- delete nonexistent
- order independence

---

# Layer 11 — Crash

Validates recovery semantics.

Checks:

- WAL truncation
- partial frames
- corruption detection

---

# Layer 12 — Fuzzing

Validates random input safety.

Checks:

- random proof decoding
- random insertion stability

---

# Layer 13 — Differential

Validates implementation equivalence.

Checks:

- canonical encoding
- domain hashes
- root consistency

---

# Layer 14 — Byzantine Mesh

Validates hostile network conditions.

Checks:

- conflicting manifests
- foreign civilization rejection

---

# Layer 15 — Temporal

Validates temporal consistency.

Checks:

- replay order stability
- temporal equivalence

---

# Layer 16 — Mutation

Validates immutability.

Checks:

- frozen depth
- frozen proofs
- frozen endianness
