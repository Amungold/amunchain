# Security Model

## Overview

AmunChain assumes hostile environments.

The system must survive:

- malformed proofs
- corrupted snapshots
- replay attacks
- hostile peers
- semantic drift

---

# Threat Classes

## Serialization Drift

Prevented through:

- canonical codec
- replay equivalence
- frozen encoding

---

## Byzantine Peers

Prevented through:

- quarantine logic
- sovereignty checks
- manifest verification

---

## Replay Divergence

Prevented through:

- canonical transcripts
- deterministic execution
- divergence detection

---

## Snapshot Corruption

Prevented through:

- manifest verification
- replay reconstruction
- state root validation

---

# Constitutional Security Principle

Invalid constitutional state must never be accepted.
