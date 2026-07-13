# AMUN DESIGN PLATFORM

# VERSIONING POLICY

Version: 1.0

Status: FROZEN

Effective Date: 2026-07-01

---

# PURPOSE

This document defines the official versioning policy
for every package and application inside the
Amun Design Platform.

All repositories SHALL comply with this policy.

---

# VERSION FORMAT

Semantic Versioning (SemVer) SHALL be used.

Format:

MAJOR.MINOR.PATCH

Example:

1.4.2

---

# VERSION DEFINITIONS

MAJOR

Increment when incompatible public changes
are introduced.

Examples:

- Breaking API
- Removed Components
- Removed Events
- Removed Design Tokens
- Removed CSS Variables

---

MINOR

Increment when functionality is added while
remaining backward compatible.

Examples:

- New Components
- New Features
- New Themes
- New Events
- New APIs

---

PATCH

Increment for compatible fixes.

Examples:

- Bug Fixes
- Performance Improvements
- Accessibility Improvements
- Documentation Corrections
- Internal Refactoring

---

# PACKAGE VERSIONING

Each package owns its own version.

Example:

packages/amun-ui

3.2.1

packages/amun-icons

1.5.0

packages/amun-protocol-ui

2.0.4

Packages SHALL NOT share version numbers.

---

# APPLICATION VERSIONING

Applications SHALL also follow SemVer.

Examples:

Explorer

1.0.0

Wallet

0.8.0

Studio

0.2.0

Applications SHALL declare supported package versions.

---

# PUBLIC API

Public APIs include:

- Web Components
- Custom Element Attributes
- Events
- Slots
- CSS Variables
- Design Tokens
- Theme Variables
- Public TypeScript Types
- Public Utility Functions

Breaking these APIs requires
a MAJOR release.

---

# DEPRECATION POLICY

Breaking changes SHALL NOT appear immediately.

The lifecycle SHALL be:

Stable

↓

Deprecated

↓

Removal Notice

↓

Major Release

↓

Removed

Migration guidance SHALL accompany
every deprecation.

---

# CHANGELOG

Every release SHALL include
a complete changelog.

Categories:

Added

Changed

Deprecated

Removed

Fixed

Security

Performance

Documentation

---

# COMPATIBILITY

Backward compatibility is the default rule.

Every package SHOULD support
at least one previous MINOR version.

Major compatibility SHALL be documented.

---

# PRE-RELEASE VERSIONS

Allowed identifiers:

-alpha

-beta

-rc

Examples:

1.0.0-alpha.1

1.0.0-beta.2

1.0.0-rc.1

Pre-release versions SHALL NOT
be considered production ready.

---

# RELEASE TAGS

Git Tags SHALL match
the released version.

Example:

v1.0.0

v2.3.4

No unofficial tag format is permitted.

---

# DEPENDENCY POLICY

Applications SHALL define
supported package ranges.

Packages SHOULD avoid
unnecessary dependency upgrades.

Dependency updates SHALL be tested.

---

# VERSION FREEZE

During Release Candidate phase:

- No new features
- No API changes
- Bug fixes only
- Documentation updates
- Performance tuning

---

# END OF LIFE

End-of-Life packages SHALL include:

- Final Version
- End-of-Life Date
- Supported Replacement
- Migration Guide

---

# VERSION AUDIT

Every release SHALL verify:

- Version Numbers
- Dependency Compatibility
- Changelog
- Documentation
- Tests
- Security
- Performance

---

# COMPLIANCE

Any package violating this policy
cannot be released as an Official
Amun Design Platform package.

---

Version: 1.0

Status: FROZEN

Effective Date: 2026-07-01

Repository:

amun-design-platform

Document:

VERSIONING.md

Copyright © 2026 AmunChain Project.

All Rights Reserved.

