# AMUN DESIGN PLATFORM

# ARCHITECTURE

Version: 1.0

Status: FROZEN

Effective Date: 2026-07-01

---

# PURPOSE

This document defines the official
software architecture of the
Amun Design Platform.

It describes:

- Layers
- Packages
- Dependencies
- Data Flow
- Build Flow
- Runtime Flow

This document evolves over time.

Unlike CONSTITUTION.md,
ARCHITECTURE.md MAY change
as the platform evolves.

---

# HIGH LEVEL ARCHITECTURE

The platform is organized into
independent architectural layers.

Applications

↓

Protocol UI

↓

Core UI

↓

Theme

↓

Icons

↓

Design Tokens

Every dependency SHALL point
downward only.

Circular dependencies are prohibited.

---

# REPOSITORY STRUCTURE

amun-design-platform/

packages/

apps/

docs/

tools/

scripts/

.github/

Only packages/ contains reusable code.

Applications SHALL consume packages.

Packages SHALL NOT depend
on applications.

---

# PACKAGE LAYERS

Layer 1

amun-tokens

Purpose

Design values only.

Contains

- Colors
- Typography
- Radius
- Shadows
- Motion
- Spacing
- Z-index
- Breakpoints

Dependencies

None.

---

Layer 2

amun-theme

Purpose

Theme Engine.

Contains

- Dark Theme
- Light Theme
- High Contrast
- Theme Switching

Depends on

amun-tokens

---

Layer 3

amun-icons

Purpose

SVG Icon Library.

Contains

- SVG Sprite
- Icon Components
- Icon Registry

Depends on

amun-tokens

---

Layer 4

amun-ui

Purpose

General UI Components.

Contains

- Buttons
- Cards
- Tables
- Inputs
- Dialogs
- Toasts
- Tooltips
- Layout Components

Depends on

amun-theme

amun-icons

amun-tokens

---

Layer 5

amun-charts

Purpose

Reusable Chart Components.

Contains

- Line Charts

- Area Charts

- Bar Charts

- Pie Charts

- Timeline Charts

Depends on

amun-ui

---

Layer 6

amun-protocol-ui

Purpose

Blockchain-aware Components.

Contains

- Address

- Hash

- Validator

- Block

- Transaction

- QC

- Constitution

- Proof

Depends on

amun-ui

amun-charts

---

Layer 7

amun-devtools

Purpose

Developer Experience.

Contains

- Playground

- Documentation

- Theme Preview

- Component Catalog

Depends on

Every reusable package.

---

Layer 8

amun-testing

Purpose

Testing Infrastructure.

Contains

- Test Helpers

- Visual Tests

- Accessibility Tests

- Component Tests

Depends on

Testing libraries only.

---

# APPLICATION LAYER

Applications reside inside apps/.

Examples

Explorer

Wallet

Studio

Governance

Faucet

Applications SHALL depend only
on packages.

Applications SHALL NEVER
share code directly.

---

# DEPENDENCY RULES

Allowed

Application

↓

Protocol UI

↓

UI

↓

Theme

↓

Tokens

Forbidden

Tokens

↓

UI

Forbidden

UI

↓

Application

Forbidden

Application

↓

Application

---

# DATA FLOW

Backend

↓

API Layer

↓

Stores

↓

Event Bus

↓

Components

↓

User

Components SHALL NEVER
fetch data directly.

---

# STATE MANAGEMENT

Shared State SHALL live
inside Stores.

Examples

NetworkStore

ValidatorStore

BlockStore

TransactionStore

ConstitutionStore

Stores publish Events.

Components subscribe.

---

# EVENT BUS

Every cross-module communication
SHALL pass through
the Event Bus.

Example

block:created

↓

Event Bus

↓

Dashboard

↓

Charts

↓

Explorer

No direct communication.

---

# ROUTING

Applications SHALL use
History API.

Routes SHALL support

Deep Linking

Bookmarks

Browser Navigation

Lazy Loading

---

# BUILD SYSTEM

Development

TypeScript

↓

Vite

↓

Static Assets

↓

Nginx

Production artifacts SHALL contain

HTML

CSS

JavaScript

Fonts

Icons

Images

Nothing else.

---

# THEMING

Themes SHALL be built from

Design Tokens

↓

Theme Engine

↓

Components

↓

Applications

Components SHALL NEVER
hardcode colors.

---

# DESIGN TOKENS

Every visual value SHALL
originate from Design Tokens.

Forbidden

Magic Numbers

Hardcoded Colors

Random Spacing

---

# COMPONENT MODEL

Every Component SHALL contain

Template

Style

Logic

Documentation

Tests

Version

No Component SHALL
omit documentation.

---

# API LAYER

Applications communicate only with

REST

RPC

WebSocket

Future protocols MAY be added.

The UI SHALL remain
protocol independent.

---

# REAL-TIME ARCHITECTURE

Preferred order

WebSocket

↓

Server Sent Events

↓

Polling

Automatic fallback
is mandatory.

---

# SECURITY MODEL

Applications SHALL use

HTTPS

WSS

Strict CSP

Sanitized Rendering

Trusted APIs

Unsafe rendering is prohibited.

---

# OBSERVABILITY

Applications SHALL expose

Performance Metrics

Error Metrics

Memory Metrics

Rendering Metrics

These metrics SHALL support
future monitoring.

---

# TESTING ARCHITECTURE

Every package SHALL include

Unit Tests

Component Tests

Accessibility Tests

Performance Tests

Visual Regression Tests

Testing is mandatory.

---

# PACKAGE ISOLATION

Every package SHALL be

Independent

Versioned

Documented

Reusable

Replaceable

---

# EXTENSIBILITY

Future packages MAY include

amun-wallet-ui

amun-governance-ui

amun-nft-ui

amun-defi-ui

amun-mobile-ui

Without modifying
existing architecture.

---

# ARCHITECTURE EVOLUTION

Architectural changes SHALL require

ADR

Architecture Review

Governance Approval

Breaking architectural changes
require a MAJOR release.

---

Version: 1.0

Status: FROZEN

Effective Date: 2026-07-01

Repository:

amun-design-platform

Document:

ARCHITECTURE.md

Copyright © 2026 AmunChain Project.

All Rights Reserved.

