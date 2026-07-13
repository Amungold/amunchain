# Constitutional Signature Registry

## Registry Purpose

The signature registry maintains the immutable record of all
constitutional signatures.

---

## Registry Entries

### Genesis Signatures

| Signature ID | Artifact | Timestamp | Status |
|---|---|---|---|
| SIG-GEN-001 | Genesis Manifest | 2026-05-28T00:56:17Z | Active |
| SIG-GEN-002 | Freeze Certificate | 2026-05-28T00:56:17Z | Active |
| SIG-GEN-003 | Lineage Root | 2026-05-28T00:56:17Z | Active |
| SIG-GEN-004 | Birth Certificate | 2026-05-28T00:56:17Z | Active |

---

## Registry Operations

### Record Signature
Add a constitutional signature to the registry.

### Verify Signature
Check that a signature exists and is valid.

### Query Authority
Find all signatures by a given authority.

### Query Artifact
Find all signatures for a given artifact.

---

## Registry Invariants

- Signatures are immutable once recorded
- Genesis signatures can never be revoked
- All signatures must trace to genesis
- Registry is replayable and deterministic
