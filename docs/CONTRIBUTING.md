# AMUN DESIGN PLATFORM

# CONTRIBUTING GUIDE

Version: 1.0

Status: FROZEN

Effective Date: 2026-07-01

---

# PURPOSE

This document defines the official contribution
workflow for the Amun Design Platform.

Every contributor SHALL follow
these guidelines.

---

# WHO MAY CONTRIBUTE

Contributions are welcome from:

- Core Maintainers
- Package Maintainers
- External Contributors
- Community Members

All contributions are reviewed before acceptance.

---

# BEFORE WRITING CODE

Every contributor SHALL:

- Read CONSTITUTION.md
- Read GOVERNANCE.md
- Read ARCHITECTURE.md
- Read VERSIONING.md
- Read DESIGN_LANGUAGE.md

Failure to understand these documents
may result in rejection.

---

# CONTRIBUTION TYPES

Accepted contribution categories include:

- Bug Fixes
- Documentation
- Accessibility
- Performance
- Tests
- Components
- Design Tokens
- Themes
- Tooling
- Developer Experience

---

# CONTRIBUTION PROCESS

The official workflow is:

Issue

↓

Discussion

↓

Specification

↓

Implementation

↓

Tests

↓

Documentation

↓

Review

↓

Merge

No implementation SHALL bypass
this workflow.

---

# BRANCH STRATEGY

Recommended branch names:

feature/<name>

fix/<name>

docs/<name>

perf/<name>

refactor/<name>

security/<name>

test/<name>

---

# COMMIT MESSAGES

Use Conventional Commits.

Examples:

feat(ui): add button component

fix(theme): correct dark mode colors

docs(tokens): update documentation

test(card): improve coverage

refactor(layout): simplify grid

---

# PULL REQUESTS

Every Pull Request SHALL include:

- Description
- Motivation
- Screenshots (if UI)
- Tests
- Documentation
- Breaking Change Notice (if applicable)

---

# REVIEW CHECKLIST

Reviewers SHALL verify:

- Constitution compliance
- Architecture compliance
- Coding standards
- Tests
- Accessibility
- Performance
- Documentation
- Security

---

# DOCUMENTATION

Every public contribution SHALL update
documentation when required.

Missing documentation is considered
an incomplete contribution.

---

# TESTING

Every contribution SHALL pass:

- Build
- Lint
- Type Check
- Unit Tests
- Component Tests

UI contributions SHOULD include
visual regression tests.

---

# ACCESSIBILITY

Every interactive component SHALL support:

- Keyboard navigation
- Focus visibility
- Screen readers

Accessibility regressions SHALL
block merging.

---

# SECURITY

Contributors SHALL NEVER:

- Commit secrets
- Disable sanitization
- Bypass CSP
- Introduce unsafe HTML rendering

Security concerns SHALL be reported
immediately.

---

# PERFORMANCE

Contributors SHALL respect
performance budgets.

Large increases in bundle size
require explicit justification.

---

# CODE STYLE

All code SHALL follow:

- TypeScript Strict Mode
- ESLint
- Prettier
- Official naming conventions

---

# RESPECTFUL COLLABORATION

Contributors SHALL:

- Be respectful
- Be constructive
- Accept review feedback
- Document decisions

Technical discussions SHALL remain
professional.

---

# LICENSE

By contributing,
the contributor agrees that
their contribution becomes part of
the Amun Design Platform project
under the project's license.

---

Version: 1.0

Status: FROZEN

Effective Date: 2026-07-01

Repository:

amun-design-platform

Document:

CONTRIBUTING.md

Copyright © 2026 AmunChain Project.

All Rights Reserved.

