# ADR-0007: Error Handling

- Status: Accepted
- Date: 2026-07-01

## Context

Explorer must remain operational even when individual components fail.
Errors should never cause a complete application failure.

## Decision

All UI errors SHALL be isolated through Error Boundaries.

Every major page and feature module MUST provide its own
Error Boundary.

## Rules

Components MUST:

- Fail gracefully.
- Display a meaningful fallback UI.
- Log diagnostic information.
- Never expose stack traces to end users.

Applications MUST:

- Continue operating after localized failures.
- Preserve user state whenever possible.
- Retry recoverable operations automatically.

## Logging

Errors SHOULD include:

- Component name.
- Package version.
- Browser information.
- Timestamp.
- Correlation ID.
- Error category.

## Benefits

- Better resilience.
- Easier debugging.
- Improved user experience.
- Production stability.

## Consequences

Localized failures no longer affect the rest of the application.
