# ADR-0011: Documentation First

- Status: Accepted
- Date: 2026-07-01

## Context

Documentation is a constitutional requirement.

A feature without documentation is considered incomplete.

## Decision

Every package SHALL be documented before being released.

Documentation SHALL be maintained alongside the source code.

## Requirements

Each package MUST provide:

- Overview
- Installation
- Public API
- Examples
- Architecture
- Version compatibility
- Changelog

Each component MUST include:

- Purpose
- Properties
- Events
- Slots
- CSS Parts
- Accessibility notes
- Usage examples

Documentation MUST work offline.

## CI Rules

Documentation completeness SHALL be verified automatically.

Broken links or missing documentation SHALL fail CI.

## Benefits

- Faster onboarding.
- Better maintainability.
- Easier collaboration.
- Higher quality.

## Consequences

Documentation becomes part of the product rather than an afterthought.
