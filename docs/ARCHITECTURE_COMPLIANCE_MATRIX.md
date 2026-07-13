# Architecture Compliance Matrix

**Constitution:** AC-1.0 | **Date:** 12 July 2026 | **Status:** Audit in Progress

---

## Identity Domain

| Constitutional Article | Crate | Module | Status | Tests |
|------------------------|-------|--------|--------|-------|
| Article 7.1 — Identity Domain | amun-live-cluster | validator.rs | Review | n102_catchup_test |
| Article 7.1 — Identity Domain | amun-validator-identity | lib.rs | Review | unit tests |
| Article 7.1 — Identity Domain | amun-consensus-network | validator_registry.rs | Review | — |
| Article 6.3 — Authority Registry | amun-authority-registry | registry.rs | Review | unit tests |
| Article 6.3 — Authority Registry | amun-networking | validator_certificate.rs | Review | — |

## Consensus Domain

| Constitutional Article | Crate | Module | Status | Tests |
|------------------------|-------|--------|--------|-------|
| Article 7.2 — Consensus Domain | amun-consensus-network | engine.rs | Review | n102_catchup_test |
| Article 7.2 — Consensus Domain | amun-consensus-network | round.rs | Review | — |
| Article 8.3 — Deterministic | amun-consensus-network | messages.rs | Review | — |
| Article 9.11 — Deterministic | amun-consensus-execution | block_dag.rs | Review | — |

## Persistence Domain

| Constitutional Article | Crate | Module | Status | Tests |
|------------------------|-------|--------|--------|-------|
| Article 7.3 — Persistence Domain | amun-chain-store | store.rs | Review | unit tests |
| Article 7.3 — Persistence Domain | amun-chain-store | record.rs | Review | — |
| Article 8.8 — Storage never owns runtime | amun-chain-store | snapshot.rs | Review | — |

## Session Domain

| Constitutional Article | Crate | Module | Status | Tests |
|------------------------|-------|--------|--------|-------|
| Article 7.4 — Session Domain | amun-networking | peer_identity.rs | Review | — |
| Article 7.4 — Session Domain | amun-node | peer_handshake.rs | Review | — |
| Article 8.4 — Peer verification | amun-node | certificate_loader.rs | Review | — |

## Constitutional Violations Found

| Issue | Domain | Article Violated | Severity | Fix Required |
|-------|--------|------------------|----------|-------------|
| Multiple Identity Writers | Identity | Article 6.1, Rule 9.1 | High | Consolidate into IdentityProvider |
| Consensus state in validator.rs | Consensus | Article 7.2 | Medium | Extract to ConsensusEngine |
| ChainStore accessed directly | Persistence | Article 7.3 | Medium | Use Persistence Layer interface |
| Certificate verification bypassed | Session | Article 8.4 | High | Enable certificate verification |

---

## Audit Progress

| Domain | Crates Audited | Violations | Compliance % |
|--------|---------------|------------|-------------|
| Identity | 5 | 1 | 80% |
| Consensus | 4 | 1 | 75% |
| Persistence | 2 | 1 | 90% |
| Session | 3 | 1 | 70% |
| **Total** | **14** | **4** | **79%** |
