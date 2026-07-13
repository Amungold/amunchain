# ADR-0017: Browser Support

- Status: Accepted
- Date: 2026-07-01

## Context

Official Amun applications should provide a predictable
experience across supported browsers while avoiding
legacy constraints.

## Decision

The platform SHALL target modern evergreen browsers.

Legacy browsers are explicitly outside the supported scope.

## Supported Browsers

- Chrome (latest stable)
- Edge (latest stable)
- Firefox (latest stable)
- Safari (latest stable)

## Platform Requirements

Applications MUST support:

- ES Modules
- Custom Elements
- Shadow DOM
- CSS Custom Properties
- ResizeObserver
- IntersectionObserver

Applications SHOULD progressively enhance optional
features when browser capabilities differ.

## Unsupported Browsers

The platform does not guarantee compatibility with:

- Internet Explorer
- Legacy Edge
- Obsolete Android browsers

## Benefits

- Smaller bundles.
- Cleaner code.
- Better performance.
- Faster development.

## Consequences

Engineering effort focuses on modern web standards instead
of legacy browser compatibility.
