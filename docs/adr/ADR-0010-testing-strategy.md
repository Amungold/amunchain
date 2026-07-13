# ADR-0010: Testing Strategy

- Status: Accepted
- Date: 2026-07-01

## Context

Testing is a constitutional requirement.

Every package must demonstrate correctness before release.

## Decision

Testing SHALL be implemented at multiple levels.

## Required Tests

- Unit Tests
- Component Tests
- Integration Tests
- Visual Regression Tests
- Accessibility Tests
- Performance Tests
- End-to-End Tests

## Coverage

Minimum coverage:

- Statements ≥ 90%
- Branches ≥ 90%
- Functions ≥ 90%

Critical packages SHOULD exceed these values.

## CI Rules

Every Pull Request MUST pass:

- Build
- Lint
- Type Check
- Tests
- Accessibility
- Performance
- Security Scan

No failing test may be ignored.

## Benefits

- Higher reliability.
- Safer refactoring.
- Faster debugging.
- Stable releases.

## Consequences

Testing becomes part of development rather than a final step.
