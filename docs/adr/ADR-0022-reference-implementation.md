# ADR-0022: Reference Implementation

- Status: Accepted
- Date: 2026-07-01

## Context

Explorer is the first official application built on the
Amun Design Platform.

Future products will follow its architecture.

## Decision

Explorer SHALL serve as the official reference implementation.

Every architectural pattern introduced in Explorer SHOULD
be reusable by Wallet, Studio, Governance, Faucet, and
future products.

## Requirements

Explorer MUST demonstrate:

- Correct package usage.
- Proper layer separation.
- Accessibility compliance.
- Performance compliance.
- Security compliance.
- Documentation completeness.

Explorer SHALL avoid application-specific shortcuts that
cannot be reused elsewhere.

## Benefits

- Consistent ecosystem.
- Easier onboarding.
- Faster product development.
- Higher software quality.

## Consequences

Explorer becomes both a production application and the
canonical implementation for the entire Amun Design Platform.
