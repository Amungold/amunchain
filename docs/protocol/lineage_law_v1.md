# AMUN CONSTITUTIONAL LINEAGE LAW v1.0

## 1. PROTOCOL ANCESTRY (FROZEN)

### 1.1 Lineage Proof
A LineageProof cryptographically binds a child protocol to its parent.
It requires: parent freeze certificate hash, child freeze certificate hash,
and compatible golden fixtures.

### 1.2 Ancestry Chain
Protocol versions form a lawful ancestry chain. Each version must have
a verified lineage proof connecting it to its parent.

## 2. MIGRATION LAW (FROZEN)

### 2.1 Migration Witness
State migration from parent to child protocol must produce a MigrationWitness
proving: replay preservation, identity preservation, and rule compliance.

### 2.2 Migration Certificate
A MigrationCertificate authorizes state transition between protocols.
It binds a lineage proof with a migration witness.

## 3. COMPATIBILITY (FROZEN)

### 3.1 Verdicts
- Identical: same protocol version
- LawfulDescendant: direct lineage with proof
- MigrationRequired: compatible with migration
- PartialCompatibility: state only
- Incompatible: no lawful relationship

## 4. CONSTITUTIONAL INVARIANTS
- Protocol versions are immutable once frozen
- Lineage must be cryptographically provable
- Migration must preserve replay determinism
- Golden fixtures are constitutional evidence
