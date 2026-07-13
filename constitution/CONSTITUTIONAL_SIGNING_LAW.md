# Constitutional Signing Infrastructure

## Overview

Constitutional signing transforms frozen identity into cryptographically sovereign identity.

Signatures bind constitutional authority to semantic artifacts:
- Specification hashes
- Genesis hashes
- Constitutional manifests
- Lineage transitions
- Amendment semantics

---

## Constitutional Key Classes

### Genesis Authority Keys
Sign genesis artifacts. Establish origin authority.
Permanent. Cannot be rotated.

### Quorum Authority Keys
Sign constitutional amendments. Can be rotated.
Require threshold signatures.

### Amendment Authority Keys
Sign amendment proposals and activations.
Derived from quorum authority.

### Federation Authority Keys
Sign federation treaties and compatibility proofs.
Can be rotated per federation relationship.

---

## Key Properties

All constitutional keys must be:
- Cryptographically strong (Ed25519 or equivalent)
- Deterministically generated
- Verifiably replayable
- Lineage-traceable

---

## Signature Binding

Signatures are cryptographically bound to:
- The artifact being signed
- The signing authority
- The constitutional timestamp
- The specification hash at time of signing

Signatures that do not include specification hash binding
are constitutionally invalid.

---

## Signature Replay Law

Constitutional signatures are replayable.
Any node can verify:
- Signature validity
- Authority legitimacy
- Temporal consistency
- Lineage continuity

---

## Authority Rotation

Genesis authority is permanent.
Quorum authority may rotate through:
- Constitutional amendment
- Quorum vote
- Signed rotation certificate

---

## Revocation

Authority may be revoked by:
- Quorum vote
- Constitutional amendment
- Rotation certificate

Revocation is itself a constitutional artifact.
