# Sovereignty Verification Engine

## Overview

The sovereignty verification engine validates constitutional authority
across all layers: genesis, amendments, federation, and lineage.

---

## Verification Functions

### Verify Genesis Authority
Input: genesis artifact, genesis signature
Output: valid | invalid

Checks:
- Genesis authority key is valid
- Signature is cryptographically correct
- Specification hash matches
- Genesis hash matches lineage root

### Verify Amendment Legitimacy
Input: amendment artifact, amendment signature
Output: valid | invalid

Checks:
- Amendment authority is legitimate at signing time
- Signature is cryptographically correct
- Amendment does not violate frozen properties
- Lineage continuity is preserved

### Verify Federation Trust Chain
Input: federation treaty, federation signatures from both civilizations
Output: valid | invalid

Checks:
- Both civilization authorities are valid
- Both signatures are cryptographically correct
- Treaty semantics are compatible
- Lineage of both civilizations is verified

### Verify Authority Rotation
Input: rotation certificate, signatures from quorum
Output: valid | invalid

Checks:
- Quorum threshold is met
- All signatures are valid
- Rotation does not affect genesis authority
- New authority is bound to same specification hash

---

## Invariant

Constitutional authority flows from genesis.
All authority verification chains must trace to genesis.
