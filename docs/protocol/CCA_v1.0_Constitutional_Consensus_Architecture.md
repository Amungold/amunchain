# Constitutional Consensus Architecture (CCA) v1.0
## AmunChain Protocol Reference
**Document Status:** Frozen
**Version:** 1.0
**Equivalent Task:** N137.0
**Classification:** Core Protocol Specification

---

## Primitives

`Hash32` is defined as a fixed-size array of 32 bytes: `[u8; 32]`.

The base hash function used throughout this specification is `BLAKE3`, producing outputs of 32 bytes unless otherwise stated.

All integer values are encoded as big-endian. Multi-byte structures are serialized in network byte order. Concatenation is denoted by `||` and implies strict sequential appending of byte arrays with no separators.

---

## Table of Contents

1. Root Ownership
2. Calculation Timing
3. Merkle Tree Specification
4. Constitutional Root Construction
5. Constitutional Commitment Structure & Serialization
6. State Root Integration
7. AppHash Trace
8. RPC Interface
9. Integration Tests
10. Consensus Rules
11. Protocol Upgrade Rules
12. Consensus Invariants
13. EconomicTree Self-Consistency
14. External Audit Package
15. Development Phases

---

## 1. Root Ownership

| Root | Owner Module |
|:---|:---|
| `identity_root` | `amun-authority-registry` |
| `evidence_root` | `amun-evidence-root` |
| `governance_root` | Governance Subsystem (Constitutional Proof) |
| `economic_root` | `amun-tokenomics-ledger` |

## 2. Calculation Timing

All roots are calculated exclusively during the `EndBlock` phase. This phase runs after all transactions have been processed and before `AppHash` is computed. This guarantees that all state changes have been finalized and the computation is strictly deterministic across all nodes.

## 3. Merkle Tree Specification

### 3.1 EconomicTree

**Schema Identifier:** `ECONOMIC_SCHEMA_VERSION: u16 = 1`

**Base Hash Function:** BLAKE3

#### Domain Separation

| Context | Domain Separator |
|:---|:---|
| Economic Leaf | `AMUN_ECON_LEAF_V1` |
| Economic Internal Node | `AMUN_ECON_NODE_V1` |
| Identity Leaf | `AMUN_ID_LEAF_V1` |
| Identity Internal Node | `AMUN_ID_NODE_V1` |
| Evidence Leaf | `AMUN_EV_LEAF_V1` |
| Evidence Internal Node | `AMUN_EV_NODE_V1` |
| Governance Leaf | `AMUN_GOV_LEAF_V1` |
| Governance Internal Node | `AMUN_GOV_NODE_V1` |

#### Leaf Computation

    LeafHash = BLAKE3(domain_separator || leaf_index_u16_be || encoded_value)

Where `leaf_index` is the 0-based index of the leaf in the canonical leaf order, encoded as a 2-byte big-endian unsigned integer.

#### Internal Node Computation

    ParentHash = BLAKE3(domain_separator || left_child_hash || right_child_hash)

#### Odd Node Strategy

When a level of the tree has an odd number of nodes, the last node is duplicated. This is the duplicate-last strategy.

#### EconomicTree Leaves (Canonical Order V1)

| Leaf Index | Leaf Name | Canonical Encoding |
|:---|:---|:---|
| 0 | TotalSupply | u64 Big-Endian |
| 1 | TreasuryBalance | u64 Big-Endian |
| 2 | ValidatorRewardPool | u64 Big-Endian |
| 3 | EcosystemPool | u64 Big-Endian |
| 4 | BurnedSupply | u64 Big-Endian |
| 5 | IssuedSupply | u64 Big-Endian |
| 6 | StakedSupply | u64 Big-Endian |
| 7 | CirculatingSupply | u64 Big-Endian |

Any change to this ordering or addition of new leaves requires incrementing the Schema Identifier to `ECONOMIC_SCHEMA_VERSION = 2` and a formal protocol upgrade.

#### CirculatingSupply Definition

    CirculatingSupply = TotalSupply - BurnedSupply - StakedSupply - TreasuryBalance

In CCA v1.0, `TreasuryBalance` is excluded from circulating supply. If governance subsequently votes to release treasury funds into circulation, this action is recorded as a burn from `TreasuryBalance` and an issuance into `EcosystemPool` or `CirculatingSupply`, changing `economic_root` under full consensus. The final classification of treasury as liquid supply is deferred to `EconomicTreeSchemaV2`.

## 4. Constitutional Root Construction

    constitutional_root = BLAKE3(
        "AMUN_CONSTITUTIONAL_ROOT_V1" ||
        identity_root ||
        evidence_root ||
        governance_root ||
        economic_root
    )

The order of concatenation is fixed. Any change constitutes a consensus-breaking protocol modification.

## 5. Constitutional Commitment Structure & Serialization

### Structure

    struct ConstitutionalCommitment {
        version: u16,
        identity_root: Hash32,
        evidence_root: Hash32,
        governance_root: Hash32,
        economic_root: Hash32,
        constitutional_root: Hash32,
    }

### Canonical Serialization V1

    [version]             = u16 big-endian (2 bytes)
    [identity_root]       = Hash32       (32 bytes)
    [evidence_root]       = Hash32       (32 bytes)
    [governance_root]     = Hash32       (32 bytes)
    [economic_root]       = Hash32       (32 bytes)
    [constitutional_root] = Hash32       (32 bytes)

    Total serialized length: 162 bytes

### Commitment Root Computation

    serialized_bytes = CanonicalSerialization(ConstitutionalCommitment)
    constitutional_commitment_root = BLAKE3("AMUN_CONSTITUTIONAL_COMMITMENT_V1" || serialized_bytes)

## 6. State Root Integration

We distinguish between two governance roots:

- `governance_state_root`: The full state root of the governance subsystem, containing all proposals, votes, and parameters stored in the `StateStore`.
- `governance_root`: The constitutional proof root of governance, included inside `ConstitutionalCommitment` and contributing to `constitutional_root`.

The final `state_root` is computed as:

    state_root = MerkleRoot(
        accounts_root,
        staking_root,
        governance_state_root,
        constitutional_commitment_root
    )

This ensures governance contributes to `state_root` once as full state and once as a constitutional proof root, with no double-counting.

## 7. AppHash Trace

    EconomicTree (Binary Merkle, SchemaV1, Domain-Separated)
            |
            v
    economic_root
            |
            v
    ConstitutionalRoot = BLAKE3(DOMAIN || identity || evidence || governance || economic)
            |
            v
    ConstitutionalCommitment (V1)
            |
            v
    Canonical Serialization
            |
            v
    constitutional_commitment_root = BLAKE3(DOMAIN || serialized)
            |
            v
    state_root = MerkleRoot(accounts, staking, governance_state, commitment_root)
            |
            v
    AppHash = state_root
            |
            v
    Block Commit (Tendermint/CometBFT)

## 8. RPC Interface

**Endpoint:** `GET /constitutional/status?height=7990`

**Response:**

    {
        "height": 7990,
        "version": 1,
        "identity_root": "0x...",
        "evidence_root": "0x...",
        "governance_root": "0x...",
        "economic_root": "0x...",
        "constitutional_root": "0x...",
        "app_hash": "0x..."
    }

## 9. Integration Tests

### Test A: Economic State Change

- Increment `TreasuryBalance` by 1.
- Assert: `economic_root` changes.
- Assert: `constitutional_root` changes.
- Assert: `state_root` changes.
- Assert: `AppHash` changes.

### Test B: Evidence State Change

- Add a new evidence record to the Evidence Module.
- Assert: `evidence_root` changes.
- Assert: `constitutional_root` changes.
- Assert: `AppHash` changes.

### Test C: Cross-Node Determinism

- Deploy Node A and Node B with identical `genesis.json`.
- Submit identical transactions to both nodes up to block height N.
- Assert: `economic_root` is identical on both nodes.
- Assert: `constitutional_root` is identical on both nodes.
- Assert: `state_root` is identical on both nodes.
- Assert: `AppHash` is identical on both nodes.

## 10. Consensus Rules

1. Any block that produces a `constitutional_root` different from the value computed by the majority of validators for the same height and same transactions is invalid.
2. Any difference in `economic_root` (or any of its constituent leaves) leads to a mismatched `constitutional_root`, which leads to a mismatched `AppHash`. This constitutes a consensus failure for the offending node.
3. All roots are calculated in `EndBlock`. No external observer, daemon, or sidecar process is authorized to modify these roots.

## 11. Protocol Upgrade Rules

- `ConstitutionalCommitment.version` is part of the consensus state.
- When upgrading from V1 to V2:
    - For `Height < UpgradeHeight`: V1 rules apply exclusively.
    - For `Height >= UpgradeHeight`: V2 rules apply exclusively.
- No node is permitted to compute V1 on one height while another node computes V2 on the same height. This constitutes a consensus fork.
- Upgrades are coordinated via the standard protocol upgrade mechanism with a clearly specified `UpgradeHeight`.

## 12. Consensus Invariants

The following invariants must hold at the end of every `EndBlock` execution. Failure of any invariant results in immediate block validation failure before `Commit`.

**Invariant 1:** `constitutional_root == BLAKE3("AMUN_CONSTITUTIONAL_ROOT_V1" || identity_root || evidence_root || governance_root || economic_root)`

**Invariant 2:** `constitutional_commitment_root == BLAKE3("AMUN_CONSTITUTIONAL_COMMITMENT_V1" || CanonicalSerialization(ConstitutionalCommitment))`

**Invariant 3:** `state_root` must include `constitutional_commitment_root` as a leaf in its Merkle tree computation.

**Invariant 4:** The `AppHash` MUST commit to `state_root`. For CCA v1.0, this is defined as `AppHash = state_root`. Future versions may introduce additional roots (such as `execution_root` or `receipt_root`) into the `AppHash` computation, in which case the invariant will be updated to reflect the new hash composition while preserving the requirement that `state_root` remains a committed input.

**Invariant 5:** Given identical state transition inputs (same genesis, same transactions up to height N), all validators must compute identical values for all roots defined in this specification.

Any invariant violation triggers an immediate `BlockValidationFailure`, preventing the block from being committed. This ensures that no invalid state can ever become part of the canonical chain.

## 13. EconomicTree Self-Consistency

Before finalizing `economic_root` in `EndBlock`, the following mandatory check is performed:

    ComputedCirculatingSupply = TotalSupply - BurnedSupply - StakedSupply - TreasuryBalance

    If ComputedCirculatingSupply != CirculatingSupply:
        EconomicRoot is INVALID
        Block is REJECTED

This prevents silent accounting errors from becoming part of the committed state. The check is performed on every block without exception.

## 14. External Audit Package

Upon completion of CCA-IMPL-1 through CCA-IMPL-5, an independent audit package shall be published under the name:

**AmunChain Constitutional Audit Package v1.0**

This package shall contain:

- CCA v1.0 Specification (this document)
- Root Derivation Specification (mathematical derivation of all roots)
- EconomicTree Specification (Merkle tree construction, leaf encoding, domain separation)
- Canonical Serialization Specification (byte-level serialization formats)
- Determinism Test Results (Test C output from at least two independent node implementations)
- AppHash Trace Examples (worked examples for specific block heights showing intermediate values at each step)
- Cross-Node Proof Reports (evidence that multiple nodes converge on identical state)

This package is designed to allow a third-party auditor to verify the cryptographic integrity of AmunChain without reading the full source code of the node implementation.

## 15. Development Phases

The development roadmap beyond the initial feature-complete milestone is organized into named architectural phases:

| Phase | Name | Scope |
|:---|:---|:---|
| I | Sovereign Kernel | Core consensus engine, validator identity, basic governance |
| II | Consensus Hardening | Slashing framework, evidence system, recovery and snapshots |
| III | Constitutional Security | Constitutional proof system, identity and evidence roots |
| IV | Economic Sovereignty | Tokenomics engine, economic ledger, reward distribution |
| V | Constitutional Consensus Architecture | CCA v1.0 specification, root commitment, Merkle proofs |
| VI | Mainnet Readiness & External Audit | Audit package publication, cross-node verification, stress testing |
| VII | Public Mainnet Launch | Genesis ceremony, validator onboarding, public network launch |

The current phase (V) represents the transition from a network that possesses constitutional and economic features to a protocol where the economy and constitution are cryptographically committed to consensus in every block. This is the final core architectural milestone before external audit and mainnet readiness.
