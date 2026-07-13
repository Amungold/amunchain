# Amun Constitutional Taxonomy

## Layer Definitions

| Layer | Name | Sovereign Boundary |
|-------|------|--------------------|
| 0 | Kernel | Canonical encoding, domain tags, protocol version |
| 1 | Truth & Evidence | State roots, execution receipts, replay proofs, snapshot sealing |
| 2 | Execution | Deterministic state transitions, scheduler |
| 3 | Consensus | Consensus engine, QC, validator registry, pacemaker |
| 4 | Persistence | WAL, storage, snapshots, crash recovery |
| 5 | Network | Gossip, transport, peer management |
| 6 | Governance & Economics | Staking, governance, economic laws |
| 7 | Interfaces | RPC, CLI, SDK, bindings |

## Semantic Types

| Type | Meaning | Example |
|------|---------|---------|
| `kernel` | Protocol foundation; catastrophic if corrupted | `amun-kernel` |
| `law` | Immutable protocol invariant | `amun-finality-law` |
| `rule` | Operational consensus predicate | `amun-lock-rule` |
| `policy` | Configurable governance parameter | `amun-governance` |
| `proof` | Cryptographic verification object | `MerkleProof`, `ReplayEquivalenceProof` |
| `transcript` | Deterministic execution record | `ReplayTranscript` |
| `certificate` | Signed attestation with quorum | `ReplayCertificate` |
| `seal` | Quorum-signed immutable commitment | `SnapshotSeal` |
| `model` | Simulation or formal abstraction | `amun-network-model` |
| `simulator` | Test harness for adversarial scenarios | `amun-network-simulator` |
| `constitution` | Layer-level invariants | `amun-network-constitution` |
| `interface` | External API boundary | `amun-rpc`, `amun-cli`, `amun-sdk` |
| `persistence` | Durable storage engine | `amun-wal`, `amun-storage` |
| `truth` | Constitutional state commitment | `amun-state-root` |
| `receipt` | Execution evidence artifact | `amun-execution-receipt` |
| `execution` | State transition engine | `amun-stf` |
| `consensus` | Consensus engine core | `amun-consensus` |
| `quorum_math` | Quorum weight computation | `amun-qc-weight` |
| `canonicalization` | Deterministic ordering logic | `amun-qc-canonical` |

## Criticality Levels

| Level | Meaning | Impact if corrupted |
|-------|---------|---------------------|
| `kernel` | Protocol foundation | Catastrophic — all layers fail |
| `consensus` | Chain safety | Chain split, double-spend |
| `deterministic` | Replay integrity | Divergent state roots |
| `persistence` | Recovery capability | Data loss, unrecoverable state |
| `interface` | External access | Service disruption only |

## Dependency Rules
- **Downward only**: Any layer may depend on layers below, never above.
- **Kernel purity**: Layer 0 has zero Amun dependencies.
- **No cyclic sovereignty**: The dependency graph must remain a DAG.
- **Sovereign crates**: May not be merged; they protect distinct truth boundaries.

## Freeze Boundaries
- `amun-kernel`: CANONICAL_VERSION = 1 (frozen wire format)
- `amun-state-root`: SNAPSHOT domain tag (frozen seal format)
- `amun-wal`: FRAME_MAGIC, FOOTER_MAGIC (frozen disk format)
- `amun-block`: wire size freeze tests active
- `amun-codec`: exact decode tests active
