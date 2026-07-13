# ADR-0001

# Adopt Monorepo Architecture

Status: Accepted

Date: 2026-07-01

---

# Context

The Amun Design Platform contains multiple
packages and applications that evolve together.

These include:

- amun-tokens
- amun-theme
- amun-icons
- amun-ui
- amun-protocol-ui
- amun-charts
- amun-devtools
- amun-testing

Applications include:

- Explorer
- Wallet
- Studio
- Governance
- Faucet

Managing these projects in separate repositories
would increase maintenance cost,
duplicate tooling,
and complicate version synchronization.

---

# Decision

The platform SHALL use a single Monorepo.

Repository structure:

packages/

Independent reusable packages.

apps/

Applications built from packages.

docs/

Architecture and documentation.

tools/

Development tooling.

scripts/

Automation scripts.

.github/

Continuous Integration.

---

# Rationale

A Monorepo provides:

- Shared tooling
- Shared CI
- Shared standards
- Shared documentation
- Atomic refactoring
- Easier dependency management
- Consistent releases

It also simplifies onboarding
for future contributors.

---

# Alternatives Considered

Multiple repositories.

Rejected because:

- Duplicate configuration
- Duplicate CI
- Dependency drift
- Harder refactoring
- More release overhead

Git submodules.

Rejected because:

- Operational complexity
- Poor developer experience

---

# Consequences

Positive

- Easier maintenance
- Unified architecture
- Shared testing
- Better consistency
- Simpler releases

Negative

- Larger repository
- More CI work
- Stronger governance required

These drawbacks are acceptable.

---

# Compliance

All official packages SHALL reside
inside this Monorepo.

External repositories MAY exist only
for experiments and prototypes.

Official releases SHALL originate
from this repository.

---

# References

CONSTITUTION.md

ARCHITECTURE.md

GOVERNANCE.md

