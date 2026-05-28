# Amun Crate Classification & Naming Convention

## Core (Layer 0-3)
These crates are **constitutional** — they define protocol truth.
- `amun-kernel`
- `amun-state-root`
- `amun-execution-receipt`
- `amun-stf`
- `amun-consensus`
- `amun-consensus-laws`
- `amun-wal`

## Infrastructure (Layer 4-5)
These crates support the protocol but are not truth-defining.
- `amun-network`
- `amun-gossip`
- `amun-storage`
- `amun-tls`

## Interface (Layer 6-7)
- `amun-rpc`
- `amun-cli`
- `amun-sdk`

## Test & Verification
- `amun-constitutional-tests`
- `amun-byzantine-tests`
- `amun-cluster-harness`
- `amun-replay-certification`
- `amun-determinism-tests`

## Naming Convention
- `amun-<domain>` for sovereign crates
- `amun-<domain>-law` for constitutional law modules
- `amun-<domain>-tests` for test harnesses
- No duplicate semantic domains (e.g., avoid both `replay-cert` and `replay-certificate`)
