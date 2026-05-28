# AMUN REPLAY PROTOCOL FREEZE CERTIFICATE v1.0

## CONSTITUTIONAL RATIFICATION

This certifies that the Amun Replay Protocol v1.0 has been
constitutionally frozen and ratified.

### Protocol Identity
- Protocol Version: 1
- Protocol Constant: REPLAY_PROTOCOL_VERSION = 1
- Protocol Domain: AMUN_REPLAY_PROTOCOL_V1
- Canonical Encoding: big-endian
- Hash Algorithm: SHA-256

### Freeze Metadata
- Freeze Date: 2026-05-27T00:28:32Z
- Build Commit: 481dd77b5e4c058f14eae7243a626c6e64c36e3f
- Fixture Manifest Hash: 
- Specification Document: docs/protocol/replay_physics_v1.md

### Golden Fixtures
- Genesis Replay Root: fixtures/replay/genesis/replay_root.bin
- Genesis State Root: fixtures/replay/genesis/state_root.bin
- Deterministic Hash: fixtures/replay/genesis/deterministic_hash.bin
- Transcript Hash: fixtures/replay/genesis/transcript_hash.bin
- Sequence Gap Error: fixtures/replay/divergence/sequence_gap_error.txt
- Equivalence Root: fixtures/replay/equivalence/self_verify_root.bin

### Constitutional Invariants (FROZEN)
1. State root evolution: SHA-256(state_root || entry_hash || sequence || domain)
2. Sequence monotonicity: strict ordering enforced
3. Transcript hash: SHA-256(entry_hash_0 || ... || entry_hash_n)
4. Determinism: identical inputs → identical outputs
5. Golden values: immutable for protocol v1

### Amendment Process
Any change to these invariants requires:
1. Constitutional amendment proposal
2. New REPLAY_PROTOCOL_VERSION
3. Migration proof for all existing state
4. Golden fixture regeneration
5. New freeze certificate

### Signatories
Ratified by the Amun Constitutional Assembly.
This certificate is a constitutional artifact.
