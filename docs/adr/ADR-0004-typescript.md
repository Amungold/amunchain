# ADR-0004

# Adopt TypeScript Strict Mode

Status: Accepted

Date: 2026-07-01

---

# Context

The Amun Design Platform is intended to be
a long-lived enterprise codebase.

The platform requires:

- Type Safety
- Maintainability
- Predictability
- Self-documenting APIs
- Reliable Refactoring

JavaScript alone cannot provide
these guarantees.

---

# Decision

The platform SHALL use
TypeScript in Strict Mode
for every package
and every application.

The following compiler options
SHALL remain enabled:

- strict
- noImplicitAny
- strictNullChecks
- noImplicitReturns
- noFallthroughCasesInSwitch
- noUncheckedIndexedAccess
- exactOptionalPropertyTypes

---

# Rationale

Strict TypeScript provides:

- Early error detection
- Better tooling
- Reliable refactoring
- Better IDE support
- Safer APIs
- Higher maintainability

These benefits outweigh
the additional development effort.

---

# Alternatives Considered

JavaScript

Rejected because:

- Weak type guarantees
- Higher runtime error risk
- Harder maintenance

---

TypeScript (non-strict)

Rejected because:

- Allows unsafe patterns
- Weakens API guarantees
- Reduces long-term reliability

---

# Consequences

Positive

- Fewer runtime bugs
- Better code quality
- Safer refactoring
- Better documentation
- Stronger APIs

Negative

- More explicit typing
- Higher initial effort

These drawbacks are acceptable.

---

# Public APIs

Every exported API SHALL include:

- Explicit parameter types
- Explicit return types
- JSDoc where appropriate

Implicit public types are prohibited.

---

# Use of "any"

The use of "any" is prohibited.

Exceptions require:

- Clear justification
- JSDoc explanation
- Reviewer approval

Prefer:

- unknown
- generics
- discriminated unions

---

# Null Safety

Nullable values SHALL be handled explicitly.

Unsafe assumptions about
null or undefined are prohibited.

---

# Type Definitions

Shared types SHALL reside in
their owning package.

Duplicate type definitions
across packages are prohibited.

---

# Compliance

Every package SHALL compile
without TypeScript errors.

Compiler warnings SHALL be
treated as build failures
where practical.

---

# References

CONSTITUTION.md

CODING STANDARD

VERSIONING.md

