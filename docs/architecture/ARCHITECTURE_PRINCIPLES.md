# AmunChain Architecture Principles

**Status:** Active
**Version:** 1.1
**Last Updated:** ADR-024 era

This document defines the architectural principles that govern all
AmunChain development. Every ADR, PR, and design decision must be
consistent with these principles.

---

## 1. Single Source of Truth

Every protocol concept MUST have exactly one reference implementation.

| Concept | Crate | Domain |
|---------|-------|--------|
| HistoryRoot | `amun-history` | Chain commitment |
| Merkle Trees | `amun-merkle` | Cryptographic proofs |
| State Root | `amun-smt` | World state commitment |
| Block Hash | `amun-block-builder` | Block identity |
| Signatures | `amun-crypto` | Ed25519 operations |

**Rule:** No duplicate implementations of the same algorithm.
If a crate already provides a protocol function, all other crates
must depend on it rather than reimplementing.

---

## 2. Layer Independence

Crates are organized in layers. Dependencies must flow downward:

```text
Networking
▲
Consensus
▲
Execution
▲
History / Identity / Crypto
```

**Rule:** A lower-layer crate MUST NOT depend on a higher-layer crate.
For example, `amun-history` must never import from `amun-consensus-network`.
Violating this creates circular dependencies and prevents reuse.

---

## 3. Specification Before Implementation

Any change that affects the following MUST have an approved ADR
before implementation begins:

- Block Header format
- Consensus rules (QC, voting, finality)
- Serialization format (canonical encoding)
- Cryptographic primitives or hash algorithms
- Storage format (FinalizedChainRecord, ChainStore)

**Rule:** No ADR, no implementation. The ADR must contain:
problem statement, scope, specification, affected components,
implementation plan, test requirements, and acceptance criteria.

---

## 4. Backward Compatibility

Any change to the following MUST explicitly state a compatibility
strategy chosen from:

| Strategy | When to use |
|----------|-------------|
| **Versioning** | New format coexists with old (e.g., version byte) |
| **Migration** | Old data is converted on startup |
| **Hard Fork** | Old and new are incompatible; requires coordinated upgrade |

Applies to:

- Block format
- FinalizedChainRecord
- Snapshots
- RPC protocol

**Rule:** Compatibility strategy must be stated in the ADR.
"Do nothing" is not an acceptable strategy.

---

## 5. Determinism First

Any code that enters the consensus-critical path MUST be:

- Fully deterministic
- Time-independent (no `SystemTime::now()` in consensus logic)
- Memory-order-independent (no iteration over `HashMap` without sorting)
- Platform-independent (same result on x86_64 and ARM)
- Replayable (same input → same output, always)

**Rule:** Non-deterministic code (logging, metrics, diagnostics)
must be clearly separated from consensus logic and annotated with
`// NON-DETERMINISTIC:` comments.

---

## 6. Security by Design

Security-sensitive code MUST:

- Fail closed rather than fail open
- Validate all external inputs at trust boundaries
- Avoid implicit trust between modules
- Keep cryptographic verification separate from business logic

Security reviews are required for changes affecting:

- Consensus
- Networking
- Cryptography
- Identity
- Storage integrity

**Rule:** Security assumptions must be documented.
"Trust me" is not a security assumption.

---

## 7. ADR Discipline

Every ADR MUST contain, at minimum:

| Section | Content |
|---------|---------|
| Problem | What is broken or missing |
| Goal | What this ADR intends to achieve |
| Out of Scope | What is explicitly NOT changed |
| Specification | Implementation-agnostic algorithm definition |
| Affected Components | Which crates/modules change |
| Implementation Plan | Ordered steps |
| Test Requirements | Minimum tests before acceptance |
| Acceptance Criteria | How to verify completion |

Accepted ADRs SHOULD NOT be modified except for editorial corrections.
Behavioral or architectural changes require either a superseding ADR
or a versioned amendment.

---

## 8. Performance Budget

Protocol-critical changes MUST document:

- Expected asymptotic complexity
- Memory impact
- Network impact (messages per round, bytes per message)
- Storage impact (bytes per block, growth rate)

Performance regressions require explicit justification in the ADR.

---

## 9. Observability

All long-running services SHOULD expose:

- Metrics (counters, histograms)
- Structured logs (JSON format)
- Health checks (liveness/readiness)

Observability code MUST NOT affect consensus determinism.
Metrics collection must be separable from consensus logic.

---

## 10. Crate Design Rules

Every new crate must:

1. Have a single, clearly defined responsibility
2. Contain its own unit tests
3. Not depend on higher layers
4. Export only its public API (minimize `pub` surface)
5. Include a `README.md` stating its purpose and layer

---

## Revision History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | ADR-024 era | Initial architecture governance policy |
| 1.1 | ADR-024 era | Added Security by Design, Performance Budget, Observability principles; relaxed ADR immutability rule |

