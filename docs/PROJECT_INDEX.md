# AmunChain Documentation Index

## Overview

AmunChain is a constitutional blockchain architecture written in Rust.

The system is organized around deterministic replayable constitutional computation.

Every subsystem is designed to be:

- deterministic
- replayable
- cryptographically verifiable
- constitutionally constrained

---

# Core Philosophy

AmunChain is not a general-purpose blockchain.

It is a constitutional computational system where:

- determinism is mandatory
- replayability is mandatory
- constitutional validity is mandatory
- serialization is frozen
- domain separation is frozen
- state interpretation is frozen

Every node must independently verify reality.

---

# Current Status

| Property | Value |
|---|---|
| Phase | 92 |
| Crates | 32 |
| Audit Layers | 16 |
| Tests | 258 passing |
| Rust | 1.85+ |

---

# Repository Layout

| Path | Purpose |
|---|---|
| crates/ | Constitutional crates |
| docs/ | Technical documentation |
| constitution/ | Constitutional specifications |
| tools/ | Verification scripts |
| fixtures/ | Replay and snapshot fixtures |
| audit_reports/ | Audit reports and validation logs |

---

# Documentation Structure

## Constitutional Specifications

| File | Purpose |
|---|---|
| constitution/GENESIS_SPECIFICATION.md | Genesis constitutional rules |
| constitution/FREEZE_LEVEL0.md | Level 0 constitutional freeze |
| constitution/LEVEL0_CONSTITUTIONAL_FREEZE.md | Freeze semantics |
| constitution/PHASE3A_ATOMIC_RUNTIME.md | Runtime architecture |
| constitution/PHASE3B_PERSISTENT_STORAGE.md | Persistent storage semantics |
| constitution/PHASE4_CONSENSUS_ENGINE.md | Consensus engine specification |
| constitution/PHASE6_NETWORK_LAYER_AND_BYZANTINE_HARDENING.md | Byzantine networking rules |
| constitution/PHASE5_COMPLETE.md | Phase 5 consolidation |
| constitution/storage_kernel.md | Storage kernel internals |

---

## Consensus Law

| File | Purpose |
|---|---|
| constitution/consensus/CONSENSUS_ERROR_MODEL.md | Consensus error semantics |
| constitution/consensus/CONSENSUS_SAFETY_LAW.md | Safety guarantees |
| constitution/consensus/FORK_CHOICE_LAW.md | Fork selection rules |
| constitution/consensus/NETWORK_ADVERSARY_LAW.md | Hostile network assumptions |
| constitution/consensus/REPLAY_DETERMINISM_LAW.md | Replay guarantees |

---

## Core Documentation

| File | Purpose |
|---|---|
| docs/CRATE_ARCHITECTURE.md | Crate taxonomy |
| docs/DEVELOPER_GUIDE.md | Development workflow |
| docs/CONSTITUTIONAL_MODEL.md | Constitutional ontology |
| docs/REPLAY_MODEL.md | Replay semantics |
| docs/SECURITY_MODEL.md | Security doctrine |
| docs/AUDIT_LAYERS.md | Audit layer definitions |
| docs/BUILD_REPRODUCIBILITY.md | Reproducible builds |
| docs/CANONICAL_HASH.md | Hashing model |
| docs/CANONICAL_SERIALIZATION.md | Serialization guarantees |
| docs/REPLAY_LAW.md | Replay legal semantics |

---

## Architecture Documents

| File | Purpose |
|---|---|
| docs/architecture/DEPENDENCY_CONSTITUTION.md | Dependency law |
| docs/architecture/DEPENDENCY_RULES.md | Dependency restrictions |
| docs/architecture/CRATE_CLASSIFICATION.md | Crate layer mapping |
| docs/architecture/NAMING_CONVENTION.md | Naming standards |
| docs/architecture/TIMING_AUDIT.md | Timing audit |
| docs/architecture/SECRETS_AUDIT.md | Secrets audit |
| docs/architecture/constitution.yaml | Constitutional architecture map |

---

## Protocol Documents

| File | Purpose |
|---|---|
| docs/protocol/FREEZE_CERTIFICATE_v1.md | Freeze witness protocol |
| docs/protocol/lineage_law_v1.md | Civilization lineage law |
| docs/protocol/replay_physics_v1.md | Replay physics |

---

## Constitutional Evolution

| File | Purpose |
|---|---|
| docs/constitutional/phase84_freeze.md | Phase 84 freeze |
| docs/constitutional/phase85_seal.md | Phase 85 seal |
| docs/constitutional/topology.md | Constitutional topology |

---

## Formal Models

| File | Purpose |
|---|---|
| docs/tla/AmunConsensus.tla | Formal TLA+ model |

---

# Constitutional Principles

## Constitutional Authority

The runtime is not the authority.

The constitution is the authority.

---

## Replayability

All state transitions must be replayable independently.

---

## Determinism

All nodes must converge to identical results.

---

## Constitutional Identity

If constitutional semantics change,
the civilization identity changes.

---

# Verification Pipeline

The project validation pipeline includes:

```bash
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets
cargo deny check licenses advisories bans sources
cargo udeps --workspace
```

---

# Audit Layers

AmunChain uses 16 constitutional audit layers.

The layers validate:

- physics
- geometry
- snapshots
- replay
- byzantine resistance
- crash recovery
- mutation resistance
- fuzzing safety
- temporal consistency

---

# Build Requirements

Recommended environment:

- Linux x86_64
- Rust 1.85+
- clang
- lld

Recommended command:

```bash
cargo build --workspace --locked
```

---

# Long-Term Goal

AmunChain aims to become a constitutional substrate for deterministic civilizations.
