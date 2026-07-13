# ADR-0021: Frontend Architecture

- Status: Accepted
- Date: 2026-07-01

## Context

The Explorer is expected to evolve into a long-lived
enterprise-grade application.

The architecture must support scalability, maintainability,
and independent package evolution.

## Decision

The frontend SHALL follow a layered architecture.

Layers SHALL only communicate with adjacent layers.

## Layers

1. Design Tokens
2. Theme
3. Shared UI
4. Protocol UI
5. Application Modules
6. Application Pages

Each layer MUST remain independent.

## Rules

Applications MUST NOT bypass layers.

Business logic SHALL remain outside UI components.

Components SHALL remain presentation-focused.

## Benefits

- Clear separation of concerns.
- Easier testing.
- Better scalability.
- Lower maintenance cost.

## Consequences

Future applications can reuse the same architecture without
modification.
