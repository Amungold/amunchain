# Constitutional Topology: Core Definitions
## AmunChain Constitutional Consensus Protocol v1.0

## 1. Authority Root (`authority_root`)

### Definition
A cryptographic commitment (Blake3) representing **who constrains constitutional authority**, not which node claims authority.

### Properties
- **Topology-centric**: Different nodes with identical authority dependencies produce the SAME `authority_root`
- **Pure constraint hash**: Excludes node identity, includes only authority relationships
- **Deterministic**: Canonical encoding + sorted dependencies prevents ordering attacks

### Constitutional Meaning
- `authority_root == origin_root` → Constitutional topology unchanged
- `authority_root != origin_root` but `origin_present` → Shadowing
- `origin_present == false` → Suffocation

## 2. Lineage Root (`lineage_root`)

### Definition
A cryptographic commitment representing the **interpretive derivation chain**: `lineage = blake3(parent_lineage || semantic_root)`

### Properties
- **Recursive**: Each node commits to entire ancestry
- **Collision-resistant**: Cryptographic hashing prevents path collisions
- **Independent**: Depends only on semantics, not authority

## 3. Suffocation Indicator

### Values
| Value | State | Meaning |
|-------|-------|---------|
| 0 | Healthy | Origin fully constraining |
| 50 | Shadowed | Authority shared with interpretations |
| 100 | Suffocated | Origin no longer constraining |

## 4. Decision Matrix

| State | Admissible | Suffocation | Alert |
|-------|------------|-------------|-------|
| Admissible | YES | 0 | NO |
| Warning | YES | 25 | YES |
| Shadowed | YES | 50 | YES |
| Suffocated | NO | 100 | YES |
| Necromancy | NO | 100 | YES |
