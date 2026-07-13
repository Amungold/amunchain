# ADR-0012: Package Versioning

- Status: Accepted
- Date: 2026-07-01

## Context

The platform consists of multiple independently released packages.

A consistent versioning strategy is required.

## Decision

All packages SHALL follow Semantic Versioning.

Each package SHALL maintain its own version.

## Rules

PATCH:

- Bug fixes.
- Documentation fixes.
- Internal improvements.

MINOR:

- Backward-compatible features.
- New Components.
- New APIs.

MAJOR:

- Breaking API changes.
- Removed features.
- Incompatible behavior.

## Requirements

Every release MUST include:

- Changelog
- Version tag
- Migration guide (for MAJOR)
- Release notes

Deprecated APIs SHALL remain available for at least one major release unless a security issue requires immediate removal.

## Benefits

- Predictable upgrades.
- Stable ecosystem.
- Easier dependency management.

## Consequences

Package evolution becomes structured and traceable.
