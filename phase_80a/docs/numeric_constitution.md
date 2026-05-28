# AmunChain Numeric Constitution v1.0

## Core Constants
- SCALE = 1,000,000
- FIXED_E = 2.718281
- MAX_COUPLING = 1.5

## Operations
- Addition/Subtraction: Saturating
- Multiplication: Saturating after scaling
- Division: Saturating, no division by zero

## Transcendental
- exp: Taylor, 24 terms, 1e-5 error
- sqrt: Newton, 30 iterations, 1e-6 error

## State Hashing
- Canonical JSON, sorted keys, SHA-256

## Amendment
- 2/3 validator vote required
