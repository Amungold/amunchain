# AMUN DESIGN PLATFORM

Official UI Platform for the AmunChain Ecosystem.

Status:

Blueprint Freeze v1.0

---

# OVERVIEW

The Amun Design Platform is the official
user interface platform for every
AmunChain product.

It provides a unified architecture,
shared design language,
reusable components,
protocol-aware UI,
documentation,
testing,
and tooling.

The platform is designed for
long-term maintainability,
enterprise quality,
and scientific rigor.

---

# PRODUCTS

Applications built on this platform include:

- Explorer
- Wallet
- Studio
- Governance Portal
- Faucet
- Validator Console
- Developer Portal

Future applications SHALL reuse
the same packages.

---

# REPOSITORY STRUCTURE

packages/

Reusable packages.

apps/

Applications.

docs/

Architecture and governance.

tools/

Development utilities.

scripts/

Automation.

.github/

Continuous Integration.

---

# PACKAGES

amun-tokens

Design Tokens.

---

amun-theme

Theme Engine.

---

amun-icons

SVG Icon Library.

---

amun-ui

Core UI Components.

---

amun-protocol-ui

Blockchain-aware Components.

---

amun-charts

Chart Components.

---

amun-devtools

Developer Tools.

---

amun-testing

Testing Infrastructure.

---

# PRINCIPLES

The platform follows
AMUN UI CONSTITUTION v1.0.

Highlights include:

- Zero External Dependencies
- API First
- Event Driven
- Accessibility First
- Security by Default
- Performance Budget
- Documentation Driven
- Design Before Implementation

See:

docs/CONSTITUTION.md

---

# GETTING STARTED

Clone the repository.

Install dependencies.

Run:

pnpm install

Run development:

pnpm dev

Run tests:

pnpm test

Run lint:

pnpm lint

Build:

pnpm build

---

# DOCUMENTATION

Primary documents:

docs/CONSTITUTION.md

docs/ARCHITECTURE.md

docs/GOVERNANCE.md

docs/VERSIONING.md

docs/ROADMAP.md

docs/DESIGN_LANGUAGE.md

docs/SECURITY.md

docs/PERFORMANCE.md

docs/UX.md

docs/CONTRIBUTING.md

---

# ARCHITECTURE

Applications consume Packages.

Packages are layered.

Applications SHALL NOT
depend on each other.

Packages SHALL remain reusable.

---

# QUALITY

Every official package SHALL pass:

- Build
- Lint
- Type Check
- Unit Tests
- Component Tests
- Accessibility
- Performance
- Documentation
- Security

---

# ROADMAP

Phase 0

Blueprint Freeze

Completed.

---

Phase 1

Monorepo Foundation

---

Phase 2

Design Tokens

---

Phase 3

Core UI

---

Phase 4

Protocol UI

---

Phase 5

Explorer

---

Phase 6

Studio

---

# CONTRIBUTING

Please read:

docs/CONTRIBUTING.md

before opening a Pull Request.

---

# GOVERNANCE

Project governance is defined in:

docs/GOVERNANCE.md

Architectural decisions are documented in:

docs/adr/

---

# LICENSE

See LICENSE.

---

Version:

1.0

Status:

FROZEN

Repository:

amun-design-platform

Copyright © 2026 AmunChain Project.

All Rights Reserved.

