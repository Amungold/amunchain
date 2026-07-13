# ADR-0006: State Management

- Status: Accepted
- Date: 2026-07-01

## Context

The platform requires a predictable, lightweight, framework-independent
state management solution suitable for long-term maintenance.

## Decision

Amun Design Platform SHALL use Observable Stores.

Each store:

- Owns a single domain.
- Exposes immutable snapshots.
- Emits change events.
- Never mutates another store.

Examples:

- NetworkStore
- BlockStore
- TransactionStore
- ValidatorStore
- ConstitutionStore

## Rules

Stores MUST NOT:

- Manipulate the DOM.
- Fetch remote data directly.
- Know about UI Components.

Stores MAY:

- Subscribe to the Event Bus.
- Cache state.
- Notify subscribers.

## Benefits

- Predictable updates.
- Easy testing.
- Component independence.
- Better scalability.

## Consequences

Applications remain deterministic and framework-independent.
