# CCS Runtime Mapping

**Date:** 2026-05-31
**Status:** Phase 84.0 – Semantic Bridge

## Purpose
Maps CCS theoretical concepts to AmunChain runtime types, establishing
a precise correspondence between theory and implementation.

## Mapping Table

| CCS Concept | Runtime Type(s) | Notes |
|-------------|-----------------|-------|
| Context `C` | `Epoch`, `ValidatorSet`, `StateRoot` | Not yet unified |
| Evidence `E` | `QuorumCertificate` | Carries position, votes, signatures |
| Derivability `⊢_C` | Proposal → Prevote → Precommit → QC | Consensus chain |
| Exclusion `⇍_C` | `VoteCollector` rejection rules | Stale, foreign, conflicting |
| Authority | `ConstitutionalCertificate` | Chain of trust from genesis |
| SH1 | Finality rules | One canonical chain survives |
| CC (Comparability) | Evidence ordering | QC weight, epoch priority |

## Gap Analysis
1. `Context` is not unified – Epoch, ValidatorSet, StateRoot are separate.
2. `ValidatorSet.epoch` is `u64` instead of `Epoch` type.
3. No standalone `ConstitutionalEvidence` module – logic scattered across
   `VoteCollector`, `QuorumCertificate`, and test files.
