# AmunChain Block Commitments — Protocol Specification v1.0

Status: Normative

Supersedes:
- ADR-024
- ADR-025
- ADR-026
- ADR-027

Reference Implementation:
- crates/amun-history
- crates/amun-merkle
- crates/amun-block-builder

---

# 1. Scope

This document defines the canonical Block Header commitments and
Block Hash derivation for the AmunChain protocol.

It is the single source of truth for all protocol implementers.

---

# 2. Canonical Block Header

The Block Header SHALL contain exactly the following commitments,
in this canonical order:

1. parent_hash
2. state_root
3. transactions_root
4. receipts_root
5. slashing_root

parent_hash SHALL reference the canonical block_hash of the
immediately preceding finalized block.

The Genesis Block SHALL define the initial values of all commitments.

---

# 3. Canonical Encoding

All protocol-visible serialization SHALL use the Amun Canonical Codec.

Rules:

- fixed-width integers → Little Endian
- hashes → raw 32-byte values
- deterministic serialization only

---

# 4. Canonical Block Hash

block_hash =

BLAKE3(

AMUN_BLOCK_V1 ||

height ||

parent_hash ||

state_root ||

transactions_root ||

receipts_root ||

slashing_root ||

proposer ||

timestamp

)

This field order is frozen.

---

# 5. Protocol Invariants

I1 Transaction mutation changes transactions_root.

I2 Receipt mutation changes receipts_root.

I3 State mutation changes state_root.

I4 Block mutation changes history_root.

I5 Historical mutation changes every future history_root.

I6 Equal inputs produce identical block_hash.

I7 parent_hash is zero only for Genesis.

I8 Commitments are independent.

---

# 6. Conformance

A conforming implementation SHALL:

- compute commitments exactly
- produce identical block_hash values
- satisfy I1-I8
- reject invalid block headers
- pass the official conformance suite

---

# 7. Mainnet Readiness

P0
- Canonical Codec
- Invariant Tests

P1
- Security Review

P2
- evidence_root

P3
- validator_set_root
- governance_root

---

# 8. Governance

Changes to this specification require:

- new ADR, or
- versioned specification update.

Direct modification is prohibited.
