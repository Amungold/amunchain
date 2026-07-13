# AmunChain Level 0 — Constitutional Freeze Documentation

---

# 1. Executive Summary

AmunChain Level 0 is a constitutional deterministic sovereign kernel substrate — not a blockchain runtime, but the formal foundation upon which a Byzantine-resilient consensus system can be built.

It enforces invariants at the type level, isolates all unsafe code to a single crate, and provides machine-auditable guarantees through automated tooling.

Level 0 establishes:
- deterministic serialization
- constitutional failure semantics
- typed state legality
- deterministic execution constraints
- reproducible build guarantees
- consensus-safe primitive infrastructure

This layer intentionally excludes networking, async runtimes, mempool logic, economic policy, and smart contract execution semantics.

---

## 1.1 Key Metrics

| Metric | Value |
|---|---|
| Crates | 8 production + 1 tool + 1 test |
| Rust source files | 53 |
| Total lines of Rust | ~2,800 |
| Unit tests | 112 (all passing) |
| Build profile | Reproducible (fat LTO, codegen-units=1, panic=abort) |
| Unsafe boundary | Single crate (amun-unsafe) |
| Panic paths in production | Zero |
| Heap allocation in Layer 0 | Zero |
| Floating point in consensus | Zero |
| Cyclic dependencies | Zero |
| Target platforms | x86_64, aarch64, wasm32 |

---

# 2. Architecture Overview

## 2.1 Crate Dependency Graph

```text
amun-unsafe ─────────────────────────────────────────────┐
amun-failure ────────────────────────────────────────────┤
amun-kernel-types ← amun-failure ────────────────────────┤
amun-codec ← amun-failure, amun-kernel-types ────────────┤
amun-state-types ────────────────────────────────────────┤
amun-constitution ← amun-failure, amun-kernel-types, amun-codec ─┤
amun-evidence ← amun-failure, amun-kernel-types, amun-codec ─────┤
amun-execution ← amun-failure, amun-kernel-types, amun-codec, amun-constitution

---

2.2 Crate Responsibilities

Crate| Responsibility| Key Types
amun-unsafe| Sole unsafe boundary| RawSlot<T>, InitGuard<T>, StableLayout
amun-failure| Failure taxonomy| ConstitutionalFault, FailureContext, AmunResult<T>, KernelState
amun-kernel-types| Primitive types + capacity constants| Epoch, Round, Hash32, ValidatorId
amun-codec| Canonical deterministic serialization| CanonicalEncode, CanonicalDecode, CanonicalHash
amun-state-types| Typed legality model| State<T, S, D, C>
amun-constitution| Governance + quorum mathematics| ProtocolCapacities, QuorumTransitionParameters
amun-evidence| Byzantine evidence| Evidence, EquivocationProof
amun-execution| Deterministic WASM execution profile| DeterministicWasmProfile
constitutional-linter| Automated constitutional audit| Heuristic rule scanner
amun-determinism-tests| Cross-platform determinism verification| KAT fixtures

---

3. Constitutional Guarantees

3.1 Machine-Verified Invariants

#| Invariant| Enforcement
1| Zero unsafe outside amun-unsafe| workspace forbid + linter
2| Zero unwrap/expect/panic! in production| clippy + audit
3| Canonical little-endian encoding| CanonicalEncode + tests
4| Domain-separated Blake3 hashing| HashDomain
5| Overflow-safe arithmetic| checked_add
6| Type-enforced legality| State<T,S,D,C>
7| Frozen capacity constants| constitutional_capacity
8| Stable deterministic sorting| insertion sort
9| decode(encode(x)) == x| property tests

---

3.2 Forbidden Behaviors

The following are constitutionally forbidden inside consensus paths:

- f32 / f64 floating point types
- std::time::SystemTime
- randomness sources
- HashMap iteration
- std::collections in deterministic paths
- async runtimes inside consensus logic
- thread spawning inside consensus
- heap allocation in Layer 0
- memory.grow in deterministic WASM
- non-deterministic IO

---

3.3 Consensus Separation Principle

Consensus logic must remain transport-independent.

Networking, async runtimes, sockets, timers, disk IO, and external system interactions are forbidden inside consensus state transitions.

Consensus must operate as a pure deterministic replicated state machine.

Transport, mempool, and peer management layers are external systems.

---

4. Threat Model

4.1 Assumptions

Level 0 assumes:

- Byzantine validators bounded by f < n/3
- Trusted compiler/toolchain
- Secure cryptographic primitives
- Deterministic execution environment
- Deterministic memory layout assumptions
- No malicious hardware fault injection
- No side-channel resistance guarantees
- No OS-level compromise protections

---

4.2 Explicit Non-Goals

Level 0 intentionally does NOT implement:

- networking
- P2P transport
- mempool
- signature cryptography
- smart contract runtime
- parallel execution
- sharding
- economic policy
- staking economics
- validator incentives
- async runtime
- persistence engine
- distributed storage

---

5. Test Suite

5.1 Test Distribution

Crate| Tests
amun-unsafe| 15
amun-failure| 12
amun-kernel-types| 20
amun-codec| 25
amun-state-types| 8
amun-constitution| 4
amun-evidence| 2
amun-execution| 8
amun-determinism-tests| 6
constitutional-linter| 12
TOTAL| 112

---

5.2 Test Categories

Category| Description
Unit tests| Behavioral correctness
Property tests| Roundtrip invariants
KAT tests| Deterministic hash verification
Rejection tests| Overflow/trailing/short-input
Audit tests| Linter correctness
Determinism tests| Canonical equivalence

---

6. Audit Methodology

6.1 Automated Audits

Audit| Method
Unsafe boundary| regex scan
Panic paths| regex scan
Float detection| constitutional-linter
Heap allocation| constitutional-linter
Build integrity| cargo build
Test integrity| cargo test

---

6.2 Known Limitations

Limitation| Impact
Regex linter| false positives/negatives
No AST parsing| incomplete semantic analysis
x86_64 deterministic verification only| platform divergence risk
WASM interpreter stub| no actual execution
Incomplete Coq proofs| formal verification incomplete

---

6.3 Trusted Computing Base

Component| Role
Rust compiler| compilation
Blake3| hashing
heapless| stack collections
zeroize| secret clearing

---

7. Memory Model

7.1 Allocation Policy

Layer 0 forbids heap allocation in consensus-critical paths.

Allowed:

- stack allocation
- fixed-capacity containers
- heapless collections

Forbidden:

- std::Vec
- String
- Box
- Rc
- Arc
- dynamic allocation

---

7.2 Interior Mutability

Interior mutability is restricted to constitutionally reviewed primitives.

UnsafeCell usage must remain confined to amun-unsafe.

---

7.3 Recursion Policy

Recursive consensus logic is forbidden.

Consensus state transitions must remain bounded and iteration-based.

---

8. Build & Reproducibility

8.1 Compiler Profile

[profile.reproducible]
inherits = "release"
lto = "fat"
codegen-units = 1
panic = "abort"
strip = "symbols"
debug = 0
opt-level = 3

---

8.2 Build Commands

cargo build --workspace
cargo build --workspace --profile reproducible
cargo test --workspace
cargo run -p constitutional-linter -- crates tests

---

9. Stability Classification

Component| Stability
Primitive types| Frozen
Encoding format| Frozen
State legality model| Frozen
Failure taxonomy| Frozen
Capacity constants| Frozen
Deterministic sorting| Frozen
WASM execution profile| Experimental
Formal proofs| Incomplete
Linter engine| Experimental

---

10. Dependency Policy for Level 1

All new dependencies introduced into consensus paths must satisfy:

- no_std compatibility
- deterministic behavior
- pinned versions
- reproducible builds
- constitutional review

Forbidden dependencies:

- tokio
- rand
- chrono
- parking_lot
- dashmap
- rayon
- serde_json in consensus paths

---

11. Future Proof Obligations

Required before production consensus deployment:

1. AST-based constitutional linter
2. Verified deterministic WASM runtime
3. Cross-platform determinism CI
4. Machine-checked quorum proofs
5. Cryptographic reduction proofs
6. Coq deep embedding
7. Consensus replay simulator
8. Byzantine adversarial testing
9. Deterministic benchmark suite

---

12. Artifact Inventory

Artifact| Path
Freeze document| constitution/FREEZE_LEVEL0.md
Build environment| constitution/BUILD_ENVIRONMENT.md
Project tree| constitution/PROJECT_TREE.txt
Cargo metadata| constitution/CARGO_METADATA.json
Cargo dependency tree| constitution/CARGO_TREE.txt
Build log| constitution/BUILD_LOG.txt
Test log| constitution/TEST_LOG.txt
Invariants| constitution/INVARIANTS.md
Unsafe law| constitution/UNSAFE_LAW.md
Allocation law| constitution/ALLOCATION_LAW.md

---

13. Verification Disclaimer

Level 0 is constitutionally constrained and extensively tested, but NOT formally verified end-to-end.

Current guarantees are:

- architectural
- deterministic
- type-level
- test-backed
- audit-backed

Formal machine-checked proofs remain incomplete.

---

14. Sign-Off

████████████████████████████████████████████████████████████████████████████████
██                                                                          ██
██   AMUNCHAIN LEVEL 0 — CONSTITUTIONAL HARD FREEZE                         ██
██                                                                          ██
██   Date:        2026-05-16                                                ██
██   Status:      HARD FREEZE — Immutable baseline                          ██
██   Build:       PASSED                                                    ██
██   Tests:       112/112 PASSED                                            ██
██   Linter:      CLEAN                                                     ██
██   Unsafe:      Isolated to amun-unsafe                                   ██
██   Panics:      Zero in production paths                                  ██
██                                                                          ██
██   READY FOR LEVEL 1 — CONSENSUS PROTOCOL LAYER                           ██
██                                                                          ██
████████████████████████████████████████████████████████████████████████████████

