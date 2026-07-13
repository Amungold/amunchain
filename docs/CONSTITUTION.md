# AMUNCHAIN CONSTITUTION
## Version 1.0
### Constitutional Ownership & Authority Model

**Constitution-ID:** AC-1.0 | **Language:** English | **License:** Apache-2.0 | **Applies-To:** AmunChain Protocol | **Normative:** Yes | **Status:** Ratified | **Date:** 12 July 2026

---

# NORMATIVE LANGUAGE

The key words SHALL SHALL NOT SHOULD SHOULD NOT MAY are to be interpreted as described in RFC 2119.

---

# PREAMBLE

The AmunChain Constitution defines the immutable architectural principles governing the design implementation evolution operation and long-term maintenance of the AmunChain Protocol. Its purpose is to establish a stable constitutional foundation that preserves deterministic execution explicit ownership architectural independence security boundaries and long-term maintainability. Every implementation of AmunChain SHALL conform to this Constitution. Where an implementation conflicts with this Constitution the implementation SHALL be modified or this Constitution SHALL be amended through the Constitutional Amendment Process. This document is normative.

---

# ARTICLE I — PURPOSE

Define authoritative ownership of every mutable domain. Eliminate multiple sources of truth. Guarantee deterministic execution. Separate identity from consensus. Separate runtime state from persistent storage. Prevent architectural coupling. Establish long-term architectural stability.

---

# ARTICLE II — SCOPE

Governs: Runtime ownership Identity Consensus Storage Session authentication Governance interfaces Recovery Architectural boundaries. Does NOT define: Consensus algorithms Cryptographic primitives Network protocols Performance optimizations API details unless explicitly stated.

---

# ARTICLE III — GENERAL PRINCIPLES

Principle 3.1 Single Source of Truth: Every mutable domain has exactly one authoritative runtime owner.
Principle 3.2 Explicit Ownership: Ownership SHALL never be implicit.
Principle 3.3 Separation of Concerns: Identity SHALL NOT perform consensus. Consensus SHALL NOT manage identity. Storage SHALL NOT become authority.
Principle 3.4 Least Authority: Components receive only the minimum authority required.
Principle 3.5 Determinism: All consensus decisions SHALL be deterministic.
Principle 3.6 Stable Interfaces: Subsystems communicate only through published interfaces.
Principle 3.7 Recoverability: Every authoritative runtime state SHALL define a recovery mechanism.
Principle 3.8 Backward Compatibility: Constitutional amendments SHALL preserve compatibility whenever practical.
Principle 3.9 Acyclic Dependencies: Architectural dependency cycles SHALL NOT exist between constitutional domains.

---

# ARTICLE IV — DEFINITIONS

Defined in GLOSSARY.md: Legal Owner Operational Owner Reader Projection Runtime Cache Initialization Source Recovery Source Constitutional Domain.

---

# ARTICLE V — IMPLEMENTATION INDEPENDENCE

This Constitution defines architectural responsibilities not implementation classes modules packages file names or programming language constructs. Implementations MAY evolve freely provided they remain constitutionally compliant.

---

# ARTICLE VI — CONSTITUTIONAL DOMAINS

Article 6.1 Domain Model: Each constitutional domain SHALL have exactly one Operational Owner expose only stable public interfaces and define its initialization and recovery mechanisms.

Article 6.2 Domain Map:

| Domain | Legal Owner | Operational Owner | Initialization Source | Recovery Source | Readers |
|--------|-------------|-------------------|-----------------------|-----------------|----------|
| Identity Domain | — | Implementation-defined | Implementation-defined | Implementation-defined | Consensus Domain Network RPC |
| Consensus Domain | — | Implementation-defined | Implementation-defined | Implementation-defined | Persistence Domain |
| Persistence Domain | — | Implementation-defined | Implementation-defined | Implementation-defined | RPC Explorer Snapshot |
| Session Domain | — | Implementation-defined | Implementation-defined | Implementation-defined | Consensus Domain Network |

Article 6.3 Identity Sub-Domains: Validator Identity Public Keys Certificates Voting Power View Authority Registry all under a single Operational Owner.

Article 6.4 Domain Boundaries: A Domain Owner SHALL NOT Mutate another domain Assume authority over another domain Bypass published interfaces Depend on another domain internal state.

---

# ARTICLE VII — DOMAIN RESPONSIBILITIES

Article 7.1 Identity Domain: Owns Validator Identity Certificates Signing Keys Voting Power View Authority Registry Cache. SHALL NOT Execute consensus Commit blocks Validate chain state. Authority Registry is a runtime cache and SHALL NEVER become authoritative.

Article 7.2 Consensus Domain: Owns Current Height Current View Locked QC High QC History Root Canonical Chain Progression Commit Decisions. SHALL NOT Manage certificates Load validator identities Authenticate peers.

Article 7.3 Persistence Domain: Owns persistent projection only Store finalized blocks Recover runtime state Serve RPC Support snapshots. SHALL NEVER become canonical authority.

Article 7.4 Session Domain: Owns authenticated peer sessions Authentication Session lifecycle Secure transport binding. SHALL NOT Participate in consensus Validate blocks Decide chain state.

---

# ARTICLE VIII — ARCHITECTURAL INVARIANTS

| Invariant | Description |
|-----------|-------------|
| 8.1 | Exactly one Operational Owner exists per mutable domain |
| 8.2 | Every persistent state derives from authoritative runtime state |
| 8.3 | Consensus decisions are deterministic |
| 8.4 | Authenticated peers are verified before participating |
| 8.5 | Identity and Consensus remain independent |
| 8.6 | Caches never become authority |
| 8.7 | Projections never become authority |
| 8.8 | Persistent storage never owns runtime state |
| 8.9 | Cross-domain communication uses published interfaces only |
| 8.10 | Recovery SHALL restore runtime ownership without changing authority |

---

# ARTICLE IX — CONSTITUTIONAL RULES

| Rule | Description |
|------|-------------|
| 9.1 | Only the Operational Owner may mutate a domain |
| 9.2 | Bootstrap initializes runtime ownership once |
| 9.3 | Readers never modify data |
| 9.4 | Caches never become authoritative |
| 9.5 | Projections never become authoritative |
| 9.6 | Consensus never owns identity |
| 9.7 | Identity never owns consensus |
| 9.8 | Storage never owns runtime state |
| 9.9 | Every mutable domain has one Operational Owner |
| 9.10 | Interfaces define all cross-domain communication |
| 9.11 | Consensus is deterministic |
| 9.12 | Canonical chain progression originates exclusively from consensus |
| 9.13 | Authority SHALL NOT be inferred from cached or projected data |
| 9.14 | Recovery SHALL restore state without changing ownership |
| 9.15 | Architectural ownership SHALL remain explicit |

---

# ARTICLE X — PUBLIC INTERFACES

All cross-domain communication SHALL occur exclusively through stable public interfaces. Components SHALL NOT Access another component internal state directly Mutate data owned by another domain Assume the role of another Operational Owner Depend on implementation details of another component.

---

# ARTICLE XI — ARCHITECTURE FLOW

Governance Legal Authority to Identity Domain Operational Authority to Session Domain and Consensus Domain. Consensus Domain to Commit Pipeline to Persistence Domain to RPC Explorer Indexer Snapshot. Session Domain to Secure Network.

Constraints: Governance defines policy but SHALL NOT execute runtime operations. Identity Domain authenticates identities but SHALL NOT execute consensus. Session Domain authenticates communication but SHALL NOT validate blockchain state. Consensus Domain alone determines canonical chain progression. Persistence Domain stores authoritative projections but SHALL NEVER become authoritative.

---

# ARTICLE XII — ACS COMPLIANCE

ACS documents SHALL Reference the Constitution version Identify Constitutional Articles implemented Declare dependencies Define implementation guidance without modifying constitutional ownership. ACS documents SHALL NOT Contradict this Constitution Modify constitutional ownership Introduce multiple Operational Owners Override constitutional invariants. If an ACS conflicts with this Constitution the Constitution SHALL prevail.

---

# ARTICLE XIII — CONSTITUTIONAL COMPLIANCE

Every component SHALL document Constitutional Domain Operational Owner Public Interfaces Dependencies Initialization Source Recovery Source Constitutional Rules Satisfied. Every PR affecting constitutional domains SHALL include architectural review constitutional compliance review and documentation update when boundaries change.

---

# ARTICLE XIV — TESTABILITY

Every constitutional requirement SHALL be testable via Static analysis Architecture review Integration tests Runtime validation Property-based testing Documentation audit. Requirements that cannot be verified SHOULD be considered incomplete.

---

# ARTICLE XV — ARCHITECTURAL COMPLIANCE

Every implementation SHALL demonstrate constitutional compliance via Automated tests Compliance matrices Architecture diagrams Static verification Documentation review Formal verification where appropriate. Failure to comply SHALL be an architectural defect. Compliance SHALL be maintained throughout the project lifetime.

---

# ARTICLE XVI — CODE REVIEW REQUIREMENTS

Any modification involving Identity Consensus Storage Networking Sessions Governance SHALL be reviewed against this Constitution before merge.

---

# ARTICLE XVII — CONSTITUTIONAL AMENDMENTS

Requires Architectural Review Maintainer Consensus Version Increment Migration Strategy Backward Compatibility Assessment Updated Compliance Documentation.

---

# ARTICLE XVIII — CONSTITUTION VERSIONING

Semantic versioning Major for constitutional changes Minor for new articles without breaking Patch for editorial corrections.

---

# ARTICLE XIX — CONSTITUTIONAL STABILITY

The Constitution is intended to remain stable over time. Architectural amendments SHALL be exceptional. Performance improvements refactoring optimization and implementation changes SHOULD occur without modifying constitutional ownership.

---

# ARTICLE XX — CONSTITUTIONAL HIERARCHY

1 Constitution Supreme architectural authority. 2 ACS Standards Domain-specific architectural standards. 3 RFC Documents Protocol and interface specifications. 4 Design Documents Implementation design details. 5 Source Code Implementation. All lower documents SHALL conform to this Constitution. In case of conflict this Constitution prevails.

---

# ARTICLE XXI — COMPLIANCE MATRIX

| Domain | ACS | Tests | ADR |
|--------|-----|-------|-----|
| Identity | ACS-100 | — | — |
| Consensus | ACS-120 | — | — |
| Persistence | ACS-130 | — | — |
| Session | ACS-110 | — | — |
| Governance | ACS-160 | — | — |
| Security | ACS-140 | — | — |
| Performance | ACS-150 | — | — |
| Compliance | ACS-170 | — | — |

---

# ARTICLE XXII — FUTURE PARTS

Part II Authenticated Session Architecture. Part III Consensus State Model. Part IV Persistent State Projection. Part V Security Architecture. Part VI Performance Architecture. Part VII Governance Architecture. Part VIII Compliance Verification.

---

# ARTICLE XXIII — INTERPRETATION

Ambiguity SHALL be resolved preserving Single Source of Truth Deterministic execution Explicit ownership Separation of concerns Constitutional invariants.

---

# ARTICLE XXIV — DEPRECATION

Deprecated patterns SHALL remain documented until fully removed. Deprecation SHALL NOT violate constitutional compatibility without amendment.

---

# ARTICLE XXV — DOCUMENT LIFECYCLE

Draft to Review to Candidate to Ratified to Superseded to Archived.

---

# REFERENCES

STANDARDS.md ACS index. GLOSSARY.md Definitions.

---

# CONSTITUTIONAL GOVERNANCE

This Constitution is the supreme architectural authority of the AmunChain project. Any architectural change affecting constitutional ownership authority interfaces or invariants SHALL follow the Constitutional Amendment process defined in Article XVII. All implementations SHALL remain compliant with the latest ratified Constitution.

---

# VERSION HISTORY

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 12 July 2026 | Constitutional Ownership Authority Model Ratified |

---

Copyright 2026 AmunChain Constitutional Assembly
