# AmunChain Dependency Constitution v2.1

## Preamble
Enforceable constitutional laws governing inter-crate dependencies.
Violations fail CI. This document is part of the protocol.

## Article I: Layer Architecture (8 Layers)

### Layer 0 — Cryptographic Kernel
**Crates**: `amun-kernel`, `amun-kernel-types`
**Allowlist**: `sha2`, `serde`, `ed25519-dalek`, `blake3`, `zeroize`, `subtle`, `rand_core`, `signature`, `digest`

**Law 0.1**: Layer 0 MUST NOT import any amun-* crate.
**Law 0.2**: Only cryptographic, serialization, and constant-time utility crates.
**Law 0.3**: Single source of canonical encoding truth.

### Layer 0.5 — Constitutional Semantics
**Crates**: `amun-constitution`, `amun-constitution-core`, `amun-invariants`

**Law 0.5.1**: MAY import Layer 0. MUST NOT import Layers 1-4.
**Law 0.5.2**: Governance semantics are NOT kernel primitives.

### Interface Layer — Architectural Boundaries
**Crates**: `amun-execution-interface`, `amun-storage-interface`, `amun-network-interface`

**Law I.1**: Traits ONLY. No concrete service implementations.
**Law I.2**: Std/core impls (Debug, Display, Error, From, Clone) permitted.
**Law I.3**: Service impls (Tcp, Http, Ws, Network, Store, Runtime, Tokio) FORBIDDEN.
**Law I.4**: `Send + Sync` MUST be documented with `@constitutional-runtime-contract`.
**Law I.5**: NO async runtime dependencies.

### Layer 1 — Consensus Core
**Crates**: `amun-consensus`, `amun-pacemaker`, `amun-fork-choice-law`,
`amun-qc-canonical`, `amun-view-change`, `amun-quorum-certificate`

**Law 1.1**: MUST NOT import `amun-runtime`, `amun-storage`, `amun-mempool`.

### Layer 2 — Deterministic Execution
**Crates**: `amun-execution`, `amun-runtime`, `amun-stf`, `amun-state-transition`

**Law 2.1**: MUST be deterministic and replayable.

### Layer 3 — Persistence
**Crates**: `amun-wal`, `amun-storage`, `amun-snapshot`, `amun-state-root`

**Law 3.1**: MUST NOT depend on consensus logic.

### Layer 4 — Network
**Crates**: `amun-network`, `amun-gossip`, `amun-http`, `amun-websocket`

**Law 4.1**: Primary host for tokio/hyper. MUST NOT contain consensus logic.

### Layer 5 — Governance
**Crates**: `amun-governance`, `amun-staking`, `amun-economics`, `amun-upgrade`

**Law 5.1**: Transitions MUST be replay-deterministic.

### Layer 6 — Testing
**Crates**: All `*-tests`, `*-simulation`, `*-audit`, `*-formal`

**Law 6.1**: No production crate MAY depend on a test crate.

## Article II: Async Runtime Laws

**Law Async.1**: tokio/hyper MAY exist outside Layer 4 ONLY behind deterministic interfaces.
**Law Async.2**: Async runtimes MUST NOT alter deterministic protocol behavior.
**Law Async.3**: Optional async deps require constitutional review.

## Article III: Prohibited Dependencies

| From | Must NOT Import | Reason |
|---|---|---|
| amun-consensus | amun-runtime | Consensus contamination |
| amun-consensus | amun-storage | Storage coupling |
| amun-consensus | amun-mempool | Mempool coupling |
| amun-kernel | any amun-* | Kernel purity |
| amun-wal | amun-consensus | Persistence isolation |

## Article IV: Merge Strategy (Phased)

| Phase | Crates to Merge | Timing |
|---|---|---|
| A | finality-law, unlock-law, timeout-law → parent crates | Immediate |
| B | snapshot-engine + snapshot-constitution → amun-snapshot | After Phase A |
| C | runtime-law → amun-runtime | After Phase B |
| D | Network isolation audit | After Phase C |

## Article V: Enforcement
- `tools/audit-deps.sh` — grep-based audit
- `constitutional-linter` — graph-based analysis
- `.github/workflows/constitutional-audit.yml` — CI enforcement
- Any violation = merge blocked
