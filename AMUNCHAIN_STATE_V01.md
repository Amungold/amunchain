# AmunChain v0.1 — Project State Document

**Date**: 2026-06-06
**Version**: v0.1 (N55 Complete)
**Status**: Constitutional Core Certified — Ready for Network Phase

---

## 1. Executive Summary

AmunChain v0.1 represents the completion of the constitutional core — the
foundational layer that distinguishes AmunChain from traditional blockchain
architectures. The system implements **Evidence-Backed Finality** and
**Replay-Backed Consensus**, where validators do not merely vote on state
agreement but certify that execution was correct, deterministic, and auditable.

Every transaction produces a **TransitionProof** — a portable, replayable
cryptographic record binding execution context, state transition, gas
consumption, and constitutional evidence. Validators replay and verify every
proof before voting. The resulting **ConstitutionalFinalityCertificate** binds
five independent roots (State, Proof, Replay, Evidence, QC) into a single
cryptographic commitment that any third party can verify without access to
validator state.

The implementation comprises **45+ Rust crates** organized into **19
implementation waves**, passing **hundreds of unit, integration, adversarial,
and replay tests** with zero failures and zero Clippy warnings. Performance
benchmarks demonstrate **788K TPS** on microbenchmarks with sub-millisecond
replay verification. Microbenchmark throughput is measured under synthetic
workloads and is not representative of end-to-end network throughput.

AmunChain v0.1 should be regarded as a research-grade constitutional blockchain
prototype. While the constitutional execution, replay, evidence, and finality
layers are complete, real-world deployment requires completion of state
synchronization, production networking, cryptographic hardening, and independent
security review.

---

## 2. Architecture Overview

### 2.1 Layered Architecture

