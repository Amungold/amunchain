# ADR-0014: Security Policy

- Status: Accepted
- Date: 2026-07-01

## Context

Security is a constitutional requirement of the
Amun Design Platform.

## Decision

Security SHALL be enforced by default throughout every
official package and application.

## Requirements

Applications MUST:

- Use HTTPS/WSS only.
- Apply strict Content Security Policy.
- Sanitize untrusted input.
- Escape rendered text.
- Validate external data.
- Protect against XSS.
- Protect against clickjacking.
- Avoid unsafe inline scripts.

Packages MUST NOT:

- Store secrets in browser storage.
- Embed credentials.
- Trust external input.
- Disable browser security features.

Dependencies SHALL be reviewed regularly.

Security vulnerabilities SHALL block releases until resolved
or formally accepted through governance.

## Benefits

- Reduced attack surface.
- Safer applications.
- Higher trust.
- Better long-term maintenance.

## Consequences

Security becomes a mandatory engineering practice rather
than an optional review.
