# Amun Constitutional Architecture — Layer Stratification

## Layer 0: Constitutional Kernel
**Responsibility**: Determinism, canonical encoding, domain separation, protocol versioning.
- `amun-kernel`

## Layer 1: Truth & Evidence
**Responsibility**: State roots, execution receipts, replay proofs, snapshot sealing.
- `amun-state-root`
- `amun-execution-receipt`

## Layer 2: Deterministic Execution
**Responsibility**: State transition function, deterministic scheduling, execution semantics.
- `amun-stf`
- `amun-execution`
- `amun-deterministic-scheduler`

## Layer 3: Consensus & Finality
**Responsibility**: Consensus engine, pacemaker, QC, validator registry, finality laws.
- `amun-consensus`
- `amun-consensus-laws` (lock, timeout, finality, unlock)
- `amun-pacemaker`
- `amun-validator-registry`
- `amun-quorum-certificate`

## Layer 4: Persistence & Recovery
**Responsibility**: WAL, storage engine, snapshots, crash recovery.
- `amun-wal`
- `amun-storage`
- `amun-snapshot-engine`
- `amun-crash-recovery`

## Layer 5: Network & Civilization
**Responsibility**: Gossip, transport, TLS, peer reputation, network simulation.
- `amun-network`
- `amun-gossip`
- `amun-tls`
- `amun-websocket`
- `amun-network-simulator`

## Layer 6: Governance & Economics
**Responsibility**: Staking, governance, economic laws, upgrade protocol.
- `amun-staking`
- `amun-governance`
- `amun-economics`
- `amun-economic-law`
- `amun-upgrade`

## Layer 7: External Interfaces
**Responsibility**: RPC, CLI, SDK, bindings.
- `amun-rpc`
- `amun-cli`
- `amun-sdk`
- `amun-bindings`

## Dependency Rules
- **Downward only**: Any layer may depend on layers *below* it, but NEVER on layers above.
- **Kernel purity**: Layer 0 has zero Amun dependencies beyond `sha2` and `hex`.
- **No cyclic sovereignty**: The dependency graph must remain a DAG.

## Forbidden Patterns
- `network → state-root`
- `rpc → kernel internals`
- `consensus → cli`
- `any layer → layer above`
