# AmunChain Validation Baseline
## Purpose
This file establishes the immutable reference point for the Network Validation Phase.
All subsequent evidence (NV-01 to NV-08) is linked to this exact baseline.

## Phase Transition
- **Previous Phase:** Constitutional Construction
- **Current Phase:** Network Validation
- **Transition Date:** 2026-06-14
- **Trigger:** N103.5_QUORUM_EXCLUSION_PASS

## Reference Commit
Commit Hash: 90f4993
Tag: N103.5_QUORUM_EXCLUSION_PASS

## Governance Rules (Active)
1. Consensus Feature Freeze: Allowed: determinism/crash/state corruption fixes + validation tooling. Forbidden: new consensus features (N103.6+), validator policy extensions, governance additions, reputation systems.
2. Evidence-First Rule: No Gate may be marked PASS without committed evidence.
3. Validation Framework Minimalism: The framework shall not become an independent product.

## Gate Governance Rules
- G-01 (Failure Requirement): Every gate must define at least one realistic failure mode.
- G-02 (Decision Requirement): Every gate must define a PASS action and a FAIL action.
- G-03 (Evidence Authority Chain): No gate may depend on another gate's result unless that result is backed by certified evidence.
- G-04 (Knowledge Requirement): Every gate must produce new knowledge about the network.

## Gate Lifecycle
PENDING → RUNNING → PASS → CERTIFIED
PENDING → RUNNING → FAIL
PENDING → BLOCKED

## Validation Scope
NV-01 Genesis Determinism (CRITICAL), NV-02 State Determinism (CRITICAL), NV-03 Fresh Node Join (HIGH), NV-04 Restart Recovery (HIGH), NV-05 State Sync (HIGH), NV-05.5 Chaos Validation (HIGH), NV-06 7-Day Continuous Run (CRITICAL), NV-07 Network Health Visibility (MEDIUM), NV-08 RPC Stability (MEDIUM).

## Evidence Format Version
Evidence Format Version: 1.0
