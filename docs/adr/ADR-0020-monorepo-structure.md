# ADR-0020: Monorepo Structure

- Status: Accepted
- Date: 2026-07-01

## Context

The Amun Design Platform contains multiple packages and
applications that evolve together.

Managing them independently would increase maintenance
cost and reduce consistency.

## Decision

The platform SHALL use a single Monorepo.

Packages and applications SHALL coexist inside one
repository.

## Structure

packages/

- amun-tokens
- amun-theme
- amun-icons
- amun-ui
- amun-protocol-ui
- amun-charts
- amun-devtools
- amun-testing

apps/

- explorer
- wallet
- studio
- governance
- faucet

docs/

Architecture, Constitution, ADRs, Guidelines.

## Benefits

- Shared tooling.
- Shared CI.
- Unified version control.
- Easier refactoring.
- Consistent architecture.

## Consequences

Cross-package changes become significantly easier while
preserving package independence.
