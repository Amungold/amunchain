# ADR-0002

# Adopt Native Web Components

Status: Accepted

Date: 2026-07-01

---

# Context

The Amun Design Platform requires a UI
technology that is:

- Framework independent
- Long-term stable
- Standards based
- Lightweight
- Reusable across products

Products include:

- Explorer
- Wallet
- Studio
- Governance
- Faucet

The platform must avoid unnecessary
dependencies on third-party UI frameworks.

---

# Decision

The platform SHALL use
Native Web Components
as the primary UI technology.

Components SHALL be implemented using:

- Custom Elements
- Shadow DOM where appropriate
- ES Modules
- TypeScript
- CSS Custom Properties

---

# Rationale

Native Web Components provide:

- Browser standards
- Long-term compatibility
- Framework independence
- Component isolation
- Easy reuse
- Small runtime footprint
- Excellent interoperability

This aligns with the
Constitution principles.

---

# Alternatives Considered

React

Rejected because:

- Additional runtime
- Framework dependency
- Larger bundles

---

Vue

Rejected because:

- Framework dependency
- Runtime overhead

---

Angular

Rejected because:

- Large framework
- Complex build system

---

Svelte

Rejected because:

- Additional compiler dependency
- Less suitable as a shared platform
  for multiple products

---

# Consequences

Positive

- Long-term maintainability
- Smaller bundles
- Native browser support
- Easier embedding
- Framework independence

Negative

- Slightly more boilerplate
- Fewer ecosystem libraries

These drawbacks are acceptable.

---

# Component Model

Every Component SHALL:

- Be reusable
- Be documented
- Be independently testable
- Respect Design Tokens
- Support Themes
- Follow Accessibility rules

---

# Styling

Components SHALL use:

- CSS Custom Properties
- Design Tokens
- Theme Variables

Global CSS leakage SHALL be minimized.

---

# Public API

A Component MAY expose:

- Attributes
- Properties
- Events
- Slots
- CSS Parts

Breaking changes require
SemVer compliance.

---

# Compliance

All official UI Components
SHALL be implemented as
Native Web Components.

Framework wrappers MAY be added later
without changing the underlying
implementation.

---

# References

CONSTITUTION.md

ARCHITECTURE.md

DESIGN_LANGUAGE.md

