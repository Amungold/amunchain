# AmunChain

A constitutional blockchain written in Rust.

The system is designed around one core idea:

The rules governing the chain are themselves part of the chain identity.

If the constitutional rules change,
the civilization identity changes.

---

## Philosophy

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

## Current Status

| Property | Value |
|---|---|
| Phase | 92 |
| Crates | 32 |
| Audit Layers | 16 |
| Tests | 258 passing |
| Rust | 1.85+ |

---

## Repository Layout

| Path | Purpose |
|---|---|
| crates/ | Constitutional crates |
| docs/ | Technical documentation |
| constitution/ | Constitutional specifications |
| tools/ | Verification scripts |
| fixtures/ | Replay and snapshot fixtures |
| audit_reports/ | Audit outputs |

---

## Quick Start

```bash
cargo build --workspace
cargo test --workspace
cargo deny check licenses
cargo udeps --workspace
```

---

## Core Principles

### Determinism

The same input must always produce the same output.

No hidden entropy is allowed.

### Replayability

Every state transition must be independently reproducible.

### Verifiability

Every result must be cryptographically provable.

### Constitutional Authority

The constitution defines the authority of the civilization.

The runtime is a servant of the constitution.

---

## Major Components

### amun-constitutional

The constitutional core.

Contains:

- schema registry
- domain registry
- constitutional codec
- replay certificates
- specification hashing
- freeze witnesses
- constitutional invariants

### amun-storage-kernel

Persistent SMT storage engine.

Contains:

- sparse merkle trees
- WAL persistence
- snapshots
- crash recovery

### amun-replay-engine

Deterministic replay infrastructure.

Contains:

- replay equivalence
- divergence detection
- transcript validation
- witness envelopes

### amun-networking

Constitutional peer-to-peer networking.

Contains:

- sovereignty checks
- quarantine logic
- manifest verification
- peer synchronization

---

## Constitutional Identity

Civilization identity is derived from:

- frozen constants
- registry fingerprints
- constitutional invariants
- replay semantics
- physics manifests
- specification hashes

Changing constitutional semantics produces a different civilization.

---

## Audit Layers

AmunChain uses a layered constitutional audit system.

The audit stack validates:

- physics
- geometry
- replay
- byzantine resistance
- mutation resistance
- crash recovery
- differential consistency
- fuzzing safety

See:

docs/AUDIT_LAYERS.md

---

## Build Requirements

Recommended environment:

- Linux x86_64
- Rust 1.85+
- clang
- lld

Recommended build:

```bash
cargo build --workspace --locked
```

---

## Governance

Constitutional amendments are explicit civilization transitions.

Amendments must define:

- lineage
- activation semantics
- migration semantics
- replay compatibility

---

## Security Model

The system assumes hostile environments.

The runtime must reject:

- malformed proofs
- corrupted manifests
- replay divergence
- invalid snapshots
- constitutional drift

---

## Documentation

| File | Purpose |
|---|---|
| docs/CRATE_ARCHITECTURE.md | Crate taxonomy |
| docs/DEVELOPER_GUIDE.md | Developer workflow |
| docs/AUDIT_LAYERS.md | Audit layer definitions |
| docs/CONSTITUTIONAL_MODEL.md | Constitutional ontology |
| docs/REPLAY_MODEL.md | Replay semantics |
| docs/SECURITY_MODEL.md | Security doctrine |

---

## Vision

AmunChain aims to become:

A constitutional substrate for deterministic civilizations.

---

## License

The source code is licensed under the **GNU Affero General Public License v3.0 or later** (AGPL-3.0-or-later).

Constitutional artifacts (genesis, lineage, witnesses, identity) are sovereign and protected under the **Constitutional Sovereignty Addendum**.

See:
- [LICENSE](LICENSE) — AGPLv3
- [CONSTITUTIONAL_SOVEREIGNTY.md](CONSTITUTIONAL_SOVEREIGNTY.md) — Sovereignty terms
- [TRADEMARKS.md](TRADEMARKS.md) — Trademark registry
- [genesis/FOUNDING_IDENTITY.md](genesis/FOUNDING_IDENTITY.md) — Founding declaration

Copyright (c) 2026 **Amungold Global**. All trademark rights and sovereign identity rights reserved.
