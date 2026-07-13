# ADR-0005

# Adopt Event-Driven UI Architecture

Status: Accepted

Date: 2026-07-01

---

# Context

The Amun Design Platform contains
many independent UI modules that
must cooperate without becoming
tightly coupled.

Examples include:

- Dashboard
- Explorer
- Validators
- Transactions
- Network Monitor
- Constitution Viewer
- Charts

Direct communication between
Components creates unnecessary
dependencies and reduces
maintainability.

---

# Decision

The platform SHALL adopt
an Event-Driven Architecture.

Components SHALL communicate through
an Event Bus.

Components SHALL NOT directly
invoke methods on other Components.

---

# Rationale

An Event-Driven Architecture provides:

- Loose coupling
- Better scalability
- Easier testing
- Better modularity
- Independent Components
- Cleaner architecture

This aligns with the
Amun Constitution.

---

# Event Flow

Producer

↓

Event Bus

↓

Subscribers

No knowledge of subscribers
is required by producers.

---

# Event Naming

Events SHALL use
domain-oriented names.

Examples:

network:connected

network:disconnected

block:created

block:finalized

transaction:received

validator:online

validator:offline

constitution:verified

constitution:failed

theme:changed

search:submitted

---

# Event Rules

Events SHALL:

- Be immutable
- Be descriptive
- Carry only required data
- Avoid unnecessary payload

Events SHALL NOT:

- Modify global state directly
- Assume subscriber existence
- Depend on execution order

---

# Component Responsibilities

Components MAY:

- Publish Events
- Subscribe to Events
- Ignore unrelated Events

Components SHALL NOT:

- Access internal state
  of other Components

- Call private APIs
  of other Components

---

# Global State

Shared State SHALL be managed
through Stores.

Stores MAY emit Events
when state changes.

Components SHALL react
to Events instead of polling.

---

# Benefits

Expected benefits include:

- Cleaner separation
- Easier maintenance
- Better scalability
- Better testability
- Improved extensibility

---

# Consequences

Positive

- Lower coupling
- Better architecture
- Easier feature additions

Negative

- Additional Event Bus layer
- Event debugging complexity

These drawbacks are acceptable.

---

# Compliance

All official UI packages SHALL
communicate using the shared
Event Bus architecture.

Direct Component-to-Component
communication is prohibited
except for parent-child
composition relationships.

---

# References

CONSTITUTION.md

ARCHITECTURE.md

ADR-0002-web-components.md

