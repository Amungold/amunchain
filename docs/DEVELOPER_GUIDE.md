# Developer Guide

## Getting Started

Requirements:

- Rust 1.85+
- cargo
- clang
- lld

Clone and build:

```bash
git clone <repository>
cd amunchain
cargo build --workspace
```

---

# Daily Workflow

Recommended commands:

```bash
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets
cargo deny check licenses
cargo udeps --workspace
```

---

# Creating a New Crate

```bash
cargo new --lib crates/amun-new-crate
```

Then add the crate to the workspace Cargo.toml.

---

# Testing

## Unit Tests

Place unit tests inside modules using:

```rust
#[cfg(test)]
```

## Integration Tests

Place integration tests inside:

```text
tests/
```

---

# Constitutional Rules

All code must obey:

- deterministic execution
- replay equivalence
- canonical serialization
- stable hashing
- explicit invariants

---

# Forbidden Behavior

The following are forbidden:

- hidden randomness
- nondeterministic ordering
- time-based execution semantics
- machine-specific serialization
- mutable constitutional constants

---

# Audit Layers

Every subsystem should belong to one or more audit layers.

Examples:

- replay
- physics
- geometry
- mutation
- byzantine
- crash recovery

---

# CI Pipeline

The CI pipeline executes:

```bash
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets
cargo deny check licenses advisories bans sources
cargo udeps --workspace
```

---

# Tooling

Useful scripts:

| Script | Purpose |
|---|---|
| tools/verify.sh | Full verification |
| tools/check_constitution.sh | Constitutional verification |
| tools/firewall.sh | Dependency firewall |
| tools/audit-deps.sh | Dependency audit |

---

# Philosophy

AmunChain treats software engineering as constitutional engineering.
