# ADR-0008: Accessibility First

- Status: Accepted
- Date: 2026-07-01

## Context

Accessibility is a constitutional requirement of the
Amun Design Platform.

It must not be treated as an optional enhancement or
added after implementation.

## Decision

All official applications and packages MUST comply with
WCAG 2.2 AA as the minimum accessibility standard.

Accessibility SHALL be integrated into every phase of
design, development, testing, and review.

## Requirements

Every component MUST:

- Support keyboard navigation.
- Be fully usable without a mouse.
- Include proper ARIA roles where applicable.
- Expose accessible names and descriptions.
- Preserve visible focus indicators.
- Support screen readers.
- Respect reduced-motion preferences.
- Maintain sufficient color contrast.

Interactive elements MUST:

- Be reachable using Tab.
- Support Enter and Space activation.
- Never trap keyboard focus.
- Restore focus after dialogs close.

Forms MUST:

- Provide labels.
- Identify required fields.
- Associate validation messages correctly.
- Expose errors programmatically.

Tables MUST:

- Use semantic HTML.
- Provide headers.
- Support screen readers.

Charts MUST:

- Include textual summaries.
- Never rely on color alone.
- Provide accessible legends.

## Testing

Accessibility SHALL be verified through:

- Automated testing.
- Manual keyboard testing.
- Screen reader verification.
- Visual inspection.
- CI accessibility checks.

Accessibility failures SHALL block release.

## Benefits

- Inclusive user experience.
- Regulatory compliance.
- Better usability.
- Higher quality.
- Improved maintainability.

## Consequences

Accessibility becomes a mandatory engineering requirement
rather than an optional feature.
