# ADR-0015: Shared Design System

- Status: Accepted
- Date: 2026-07-01

## Context

Explorer, Wallet, Studio, Governance Portal, Faucet, and
future applications must present a unified visual identity.

Independent design systems inevitably diverge over time.

## Decision

All official applications SHALL consume the same shared
design system provided by the Amun Design Platform.

The Design System SHALL be the single source of truth for:

- Design Tokens
- Themes
- Icons
- Typography
- Components
- Layout primitives
- Charts

Applications MUST NOT redefine existing design tokens or
duplicate shared components.

## Requirements

Changes to the Design System SHALL:

- Preserve backward compatibility.
- Be documented.
- Include tests.
- Include visual regression updates.
- Follow Semantic Versioning.

## Benefits

- Visual consistency.
- Faster development.
- Lower maintenance cost.
- Better accessibility.
- Predictable behavior.

## Consequences

Every official product evolves together while maintaining
a consistent user experience.
