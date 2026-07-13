# ADR-0019: Build System

- Status: Accepted
- Date: 2026-07-01

## Context

The Amun Design Platform requires a fast, deterministic,
and maintainable build pipeline.

The build system should remain lightweight while supporting
modern web standards.

## Decision

The platform SHALL use Vite as the official build system.

TypeScript SHALL be compiled through Vite.

ES Modules SHALL be preserved whenever practical.

## Requirements

The build system MUST provide:

- Fast incremental builds.
- Production optimization.
- Tree shaking.
- Code splitting.
- Asset hashing.
- Source maps.
- CSS optimization.
- Static asset bundling.

The build MUST be reproducible.

Given identical inputs, identical outputs SHALL be produced.

## CI

Every commit SHALL execute:

- Build
- Type Check
- Lint
- Tests

Build failures SHALL block merging.

## Benefits

- Fast development.
- Predictable builds.
- Smaller bundles.
- Easier maintenance.

## Consequences

All official packages share a consistent build process.
