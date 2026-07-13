# ADR-0009: Performance Budget

- Status: Accepted
- Date: 2026-07-01

## Context

Performance is a constitutional requirement.

Without measurable limits, applications inevitably become
slower over time.

## Decision

Every package and application SHALL follow explicit
performance budgets enforced automatically in CI.

## Budgets

### JavaScript

- Initial bundle ≤ 50 KB (compressed) per page.
- Route chunks loaded lazily.
- Tree shaking required.

### CSS

- Initial CSS ≤ 10 KB (compressed).
- Shared design tokens only.
- No duplicated styles.

### Components

- Initial render < 5 ms.
- Re-render only on state changes.
- Lazy initialize expensive logic.

### Runtime

- Idle memory < 30 MB.
- Avoid unnecessary allocations.
- Avoid memory leaks.

### Network

- Cache immutable assets.
- Compress responses.
- Prefer streaming when appropriate.

## Monitoring

Performance SHALL be monitored through:

- Lighthouse
- Browser Performance API
- CI budgets
- Runtime telemetry

## Benefits

- Faster loading.
- Lower bandwidth usage.
- Better user experience.
- Predictable scalability.

## Consequences

Performance regressions become release blockers.
