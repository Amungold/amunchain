# AMUN REPLAY PHYSICS v1.0 — CONSTITUTIONAL PROTOCOL FREEZE
## 1. CANONICAL ENCODING RULES (FROZEN)
### 1.1 Integer Encoding
All multibyte integers are **big-endian**. u64 serializes to exactly 8 bytes. u32 serializes to exactly 4 bytes. u16 serializes to exactly 2 bytes.
### 1.2 Hash Encoding
All ConstitutionalHash values are raw 32-byte blobs. No length prefix. No type tag. Fixed-width canonical encoding.
### 1.3 Hash Algorithm
The constitutional hash function is **SHA-256** for protocol v1.
## 2. REPLAY STATE TRANSITION LAW (FROZEN)
### 2.1 State Root Evolution
state_root' = SHA-256(state_root || entry_hash || sequence || domain)
This is the ONLY legal state transition function for protocol v1. No semantic dispatch. No event-type interpretation. Pure cryptographic continuity.
### 2.2 Ordering Invariant
sequence numbers MUST be strictly monotonic within a replay session. Any gap (expected_sequence != actual_sequence) SHALL produce ReplayFailure::OrderingViolation.
### 2.3 Determinism Guarantee
Given identical (state_root, entries), the output state_root' MUST be identical across all platforms, all implementations, all languages.
## 3. TRANSCRIPT HASH LAW (FROZEN)
### 3.1 Computation
transcript_hash = SHA-256(entry_hash_0 || entry_hash_1 || ... || entry_hash_n)
### 3.2 Equivalence
Two transcripts with identical entry hashes in identical order MUST produce identical transcript hashes.
## 4. EQUIVALENCE SEMANTICS (FROZEN)
### 4.1 Self-Verification
execute_and_self_verify() SHALL: 1. Execute trace via DeterministicExecutor, 2. Apply same entries via ReplayState, 3. Compare final roots, 4. Return EquivalenceProof.
## 5. GOLDEN VALUES (FROZEN)
Golden values are stored in fixtures/replay/ and verified by golden_verification tests. Any deviation indicates protocol violation.
## 6. AMENDMENT PROCESS
These laws are FROZEN for protocol v1. Amendment requires: 1. Constitutional amendment proposal, 2. New protocol version identifier, 3. Migration proof for all existing state, 4. Golden value regeneration.
## 7. IMPLEMENTATION NOTE
This document describes the ACTUAL implementation, not an aspirational specification. The implementation IS the constitution for protocol v1.
