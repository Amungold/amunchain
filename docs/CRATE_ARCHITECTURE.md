# Crate Architecture

## Overview

AmunChain contains 32 crates organized into constitutional layers.

Each layer depends only on layers below it.

Circular dependencies are constitutionally forbidden.

---

# Layer Taxonomy

## Layer 0 — Foundation

Primitive infrastructure.

### amun-kernel-types

Defines:

- hash types
- block heights
- identity types
- domain separators

### amun-failure

Unified failure handling.

All crates depend on this layer.

---

## Layer 1 — Serialization

Deterministic encoding and hashing.

### amun-canonical-codec

Canonical binary serialization.

### amun-constitutional

The constitutional authority layer.

Contains:

- schema registries
- replay certificates
- freeze witnesses
- specification hashes
- constitutional invariants

### amun-constitutional-hasher

Domain-separated hashing infrastructure.

---

## Layer 2 — Storage

Persistent state infrastructure.

### amun-storage-kernel

Persistent SMT storage.

Contains:

- WAL
- snapshots
- recovery
- canonical traversal

### amun-smt

Sparse Merkle tree implementation.

---

## Layer 3 — Consensus

Validator agreement infrastructure.

### amun-consensus

Consensus engine.

### amun-consensus-math

Deterministic fixed-point arithmetic.

### amun-pacemaker

Logical timeout coordination.

---

## Layer 4 — State

State transition infrastructure.

### amun-state-machine

Deterministic state transitions.

### amun-state-stream

Streaming state updates.

### amun-stf

State transition function.

---

## Layer 5 — Constitutional

Constitutional enforcement infrastructure.

### amun-constitutional-geometry

Geometric constitutional modeling.

### amun-constitutional-verifier

Cross-layer constitutional verification.

### amun-constitutional-treaties

Inter-civilization treaty semantics.

### amun-constitutional-semantics

Constitutional semantic interpretation.

### amun-constitutional-linter

Static constitutional analysis.

---

## Layer 6 — Networking

Peer communication.

### amun-networking

Peer management and synchronization.

### amun-light-client

Merkle-based light verification.

### amun-network-interface

Networking abstraction traits.

---

## Layer 7 — Governance

Civilization evolution infrastructure.

### amun-governance

Proposal and voting lifecycle.

### amun-amendments

Constitutional amendment handling.

### amun-evolution

Civilizational evolution semantics.

---

## Layer 8 — Protection

Defensive constitutional infrastructure.

### amun-self-preservation

Legitimacy and continuity enforcement.

### amun-audit

16 constitutional audit layers.

---

## Layer 9 — Replay

Deterministic historical reconstruction.

### amun-replay-engine

Replay equivalence and divergence detection.

### amun-lineage-law

Lineage and compatibility rules.

### amun-lineage

Lineage serialization and validation.

### amun-replay-semantics

Replay semantic interpretation.

---

## Layer 10 — Interfaces

External adapters.

### amun-storage-interface

Storage abstraction traits.

### amun-runtime-adapter

Runtime environment adapters.

### amun-test-mocks

Mock implementations.

---

# Dependency Direction

Foundation
-> Serialization
-> Storage
-> Consensus
-> State
-> Constitutional
-> Networking
-> Governance
-> Protection
-> Replay
-> Interfaces

No crate may depend upward.
