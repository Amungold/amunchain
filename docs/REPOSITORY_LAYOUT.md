# Repository Layout

## Overview

The AmunChain repository is organized around constitutional layering.

Every directory has a specific constitutional responsibility.

The repository structure itself is part of the system architecture.

---

# Root Structure

| Path | Purpose |
|---|---|
| crates/ | Constitutional Rust crates |
| docs/ | Technical and architectural documentation |
| constitution/ | Formal constitutional specifications |
| tools/ | Verification and governance tooling |
| fixtures/ | Replay and snapshot test fixtures |
| audit_reports/ | Security and validation reports |
| .github/ | CI/CD workflows |
| Cargo.toml | Workspace manifest |
| Cargo.lock | Frozen dependency graph |
| README.md | Repository overview |
| CHANGELOG.md | Constitutional evolution history |

---

# crates/

## Purpose

Contains all Rust crates that compose the constitutional runtime.

The workspace is divided into layered constitutional domains.

---

## Foundation Layer

Primitive infrastructure.

### Examples

- amun-kernel-types
- amun-failure

Responsibilities:

- primitive types
- identifiers
- unified error handling

---

## Serialization Layer

Deterministic serialization and hashing.

### Examples

- amun-canonical-codec
- amun-constitutional
- amun-constitutional-hasher

Responsibilities:

- canonical encoding
- schema registries
- replay certificates
- specification hashing
- constitutional invariants

---

## Storage Layer

Persistent state infrastructure.

### Examples

- amun-storage-kernel
- amun-smt

Responsibilities:

- sparse merkle trees
- snapshots
- write-ahead logging
- crash recovery

---

## Consensus Layer

Validator agreement infrastructure.

### Examples

- amun-consensus
- amun-pacemaker
- amun-consensus-math

Responsibilities:

- block agreement
- validator coordination
- fork choice
- deterministic timing

---

## State Layer

Deterministic state transitions.

### Examples

- amun-state-machine
- amun-state-stream
- amun-stf

Responsibilities:

- state transitions
- replay semantics
- delta algebra

---

## Constitutional Layer

Constitutional semantic enforcement.

### Examples

- amun-constitutional-verifier
- amun-constitutional-treaties
- amun-constitutional-semantics
- amun-constitutional-linter

Responsibilities:

- constitutional validity
- treaty semantics
- semantic verification
- constitutional linting

---

## Networking Layer

Peer-to-peer infrastructure.

### Examples

- amun-networking
- amun-light-client
- amun-network-interface

Responsibilities:

- peer synchronization
- quarantine logic
- sovereignty verification

---

## Governance Layer

Civilizational evolution.

### Examples

- amun-governance
- amun-amendments
- amun-evolution

Responsibilities:

- constitutional amendments
- proposal lifecycle
- migration semantics

---

## Protection Layer

Defensive constitutional infrastructure.

### Examples

- amun-self-preservation
- amun-audit

Responsibilities:

- legitimacy verification
- audit layers
- mutation resistance

---

## Replay Layer

Deterministic historical reconstruction.

### Examples

- amun-replay-engine
- amun-lineage-law
- amun-lineage

Responsibilities:

- replay equivalence
- lineage validation
- divergence detection

---

# docs/

## Purpose

Human-readable technical documentation.

This directory explains:

- architecture
- replay semantics
- constitutional philosophy
- protocol invariants
- security doctrine

---

## Major Files

| File | Purpose |
|---|---|
| CRATE_ARCHITECTURE.md | Crate taxonomy |
| DEVELOPER_GUIDE.md | Development workflow |
| SECURITY_MODEL.md | Security doctrine |
| REPLAY_MODEL.md | Replay semantics |
| AUDIT_LAYERS.md | Audit specifications |
| CONSTITUTIONAL_MODEL.md | Constitutional ontology |

---

# constitution/

## Purpose

Formal constitutional artifacts.

This directory contains the civilization specification itself.

Unlike docs/,
these files are treated as constitutional authority.

---

## Contents

| Path | Purpose |
|---|---|
| consensus/ | Consensus laws |
| history/ | Historical constitutional snapshots |
| registry.toml | Registry specification |
| layers.toml | Constitutional layer map |
| taxonomy.md | Constitutional taxonomy |
| graph.txt | Dependency graph |
| BUILD_LOG.txt | Historical build logs |

---

# tools/

## Purpose

Verification and governance tooling.

These scripts automate constitutional validation.

---

## Examples

| Script | Purpose |
|---|---|
| verify.sh | Full verification suite |
| check_constitution.sh | Constitutional validation |
| firewall.sh | Dependency firewall |
| audit-deps.sh | Dependency auditing |

---

# fixtures/

## Purpose

Deterministic replay fixtures.

Contains:

- replay transcripts
- snapshot fixtures
- proof fixtures
- corruption fixtures

Fixtures are immutable test inputs.

---

# audit_reports/

## Purpose

Historical audit outputs.

Contains:

- security audits
- fuzzing outputs
- replay verification logs
- mutation reports

---

# Cargo.lock

## Constitutional Importance

Cargo.lock is treated as constitutional infrastructure.

Dependency drift is considered constitutional drift.

Build reproducibility depends on lock stability.

---

# Workspace Philosophy

The repository structure reflects constitutional layering.

The layout itself communicates authority boundaries.

No subsystem exists without explicit constitutional placement.
