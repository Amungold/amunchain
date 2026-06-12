# N100 — Node Lifecycle & Rejoin Architecture

## Overview
This phase implements and validates the complete node lifecycle: discovery, handshake, catch-up, admission, and rejoin after failure.

## Milestones

| Milestone | Status | Description |
|-----------|--------|-------------|
| N100.1 | ✅ PASS | Handshake Protocol (HELLO/WELCOME) |
| N100.2 | ✅ PASS | Consensus Rejoin (full catch-up + rejoin) |
| N100.3 | ✅ PASS | Admission State Machine (NodeState) |
| N100.4 | ✅ PASS | Rejoin Stress Test (5/5 cycles) |

## Capabilities Verified
- Node discovery via handshake
- Multi-peer fallback for handshake
- Fast-forward to network height
- Historical gap detection and filling
- Full store equivalence after rejoin
- Repeated kill/rejoin cycles (5/5 passed)
- Consensus participation after rejoin

## Tags
- N100.1_REJOIN_PROTOCOL_PASS
- N100.2_CONSENSUS_REJOIN_PASS
- N100.3_ADMISSION_STATE_MACHINE_PASS
- N100.4_REJOIN_STRESS_PASS

## Known Limitations
- Rejoin requires at least one live peer
- Block sync uses sync endpoint on port+10000
- Historical backfill may be slow for very large gaps
