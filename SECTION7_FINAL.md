# 7. Security Evaluation and Experimental Validation

## 7.1 Overview

This section evaluates the security and performance properties of the
proposed evidence-backed finality architecture.

The evaluation has two objectives. First, we investigate whether
replay-backed consensus successfully prevents invalid state transitions
from participating in the consensus process. Second, we measure the
computational cost of constitutional verification, replay validation,
lineage analysis, and evidence generation to determine whether the
proposed framework remains practical for real-world deployment.

All experiments were performed using the reference AmunChain
implementation described in previous sections. Measurements were
collected using repeated executions under controlled conditions on a
single server (AMD EPYC 9634, 8 vCPUs, 16 GB RAM, NVMe storage, Ubuntu
24.04, Rust 1.85, release profile with LTO). Every figure is reported
together with a 95% confidence interval computed over 20–30 independent
runs after warmup. The specific commit hash, Rust toolchain version,
and build configuration used are documented in the repository's
reproducibility manifest.

## 7.2 Security Objectives

The architecture is designed to satisfy four primary security goals:

- **G1 — Execution Correctness.** Every finalized state transition must
  correspond to a valid deterministic execution.

- **G2 — Replay Verifiability.** Any verifier possessing a
  TransitionProof must be capable of independently reconstructing and
  validating the execution outcome without access to validator state.

- **G3 — Evidence Integrity.** Every detected constitutional violation
  must generate cryptographically committed evidence that becomes part
  of the finality process.

- **G4 — Byzantine Resilience.** Malicious validators must be unable to
  finalize invalid state transitions without violating quorum
  assumptions.

The experiments presented in this section evaluate these objectives
both analytically and empirically, using the microbenchmark workloads
described in Table 7.1.

**Table 7.1 — Benchmark Workloads**

| Workload | Description | Opcodes |
|----------|-------------|---------|
| Halt | Minimal contract — baseline overhead | `Halt` |
| Push10 | Stack operations — compute overhead | `Push(1)` … `Push(10)`, `Halt` |
| Transform | Single resource transformation | `Transform(handle=0)`, `Halt` |
| Split | One asset split into five children | `Push(0)`, `Split(handle=0, count=5)`, `Halt` |

## 7.3 State-Scale Independence of Replay Verification

One of the primary design goals of replay-backed consensus is to
eliminate dependence on global validator state during verification
(G2). If replay verification were to scan or traverse the active state
set, its cost would grow with chain state, undermining scalability for
light clients and external auditors.

To evaluate this property, replay verification was measured while
increasing the active resource set from 10³ to 10⁶ resources. For each
configuration, a Halt-program TransitionProof was generated and then
replayed on a fresh registry initialised with the corresponding number
of active genesis resources. Only the replay step was timed; state
construction was performed outside the measurement window.

**Table 7.2 — Replay Verification Time vs Active State Size**

| Active Resources | Replay Time (µs) | 95% CI |
|------------------|------------------|--------|
| 1,000 | 1.73 | ±0.04 |
| 10,000 | 1.76 | ±0.03 |
| 100,000 | 1.71 | ±0.02 |
| 1,000,000 | 2.30 | ±0.03 |

The results demonstrate that replay verification remains effectively
constant despite a three-order-of-magnitude increase in state size. The
observed replay time varies only between 1.71 µs and 2.30 µs.

*Observation.* Replay verification complexity is independent of global
state size and depends primarily on proof size rather than chain state.
This property supports the design objective of stateless verification
(G2) and enables efficient validation by light clients and external
auditors without requiring them to synchronise the full validator state.

## 7.4 Replay Verification versus Full Execution

A central claim of the framework is that replay verification is
substantially cheaper than re-executing transactions (G1, G2). If
replay were as expensive as execution, replay-backed consensus would
double validator workload and offer no practical benefit over
traditional state-root voting.

To evaluate this claim, execution and replay costs were measured for
all four workloads at a fixed state size of 10,000 active resources.
Execution cost includes the full constitutional pipeline: VM
interpretation, gas metering, resource law verification, invariant
checking, evidence generation, and TransitionProof construction. Replay
cost includes witness reconstruction, registry initialisation,
re-execution, and result comparison. Each measurement was repeated 30
times after 5 warmup iterations.

**Table 7.3 — Execution vs Replay Cost**

| Workload | Execution (µs) | Replay (µs) | Speedup |
|----------|---------------|-------------|---------|
| Halt | 9,006.55 ±205.83 | 1.76 | 5,121× |
| Push10 | 9,025.53 ±263.26 | 3.51 | 2,570× |
| Transform | 8,660.81 ±97.05 | 2.48 | 3,495× |
| Split | 8,997.06 ±241.83 | 4.71 | 1,908× |

*Observation.* Across all workloads, replay verification is between
approximately 1,900× and 5,100× faster than full execution. The
speedup is largest for minimal contracts (Halt: 5,121×) and remains
above 1,900× even for contracts that perform resource operations
(Split: 1,908×).

The execution cost is dominated by state construction and registry
initialisation (approximately 9 ms for 10,000 resources), which is
performed once per transaction during full execution but is avoided
during replay because the TransitionProof carries the witness bundle.
These results indicate that replay-backed consensus can provide strong
execution correctness guarantees while imposing only a small fraction
of the computational cost of re-execution.

## 7.5 Full Pipeline Scalability

The complete constitutional pipeline includes transaction processing,
TransitionProof generation, replay verification, evidence construction,
and certificate formation. To evaluate whether the pipeline scales
predictably, end-to-end latency was measured for increasing transaction
counts. Each transaction is a Halt program executed against a fresh
registry; the pipeline includes proof generation, replay, and witness
archival for every transaction. Each measurement was repeated 20 times
after 5 warmup iterations.

**Table 7.4 — Pipeline Latency vs Transaction Count**

| Transactions | Latency (µs) | 95% CI |
|-------------|-------------|--------|
| 1 | 2.93 | ±0.01 |
| 10 | 28.84 | ±0.03 |
| 100 | 299.13 | ±6.50 |
| 1,000 | 3,269.08 | ±87.35 |

*Observation.* Pipeline latency grows approximately linearly with
transaction volume. The per-transaction overhead is approximately
2.9–3.3 µs for Halt programs. No evidence of superlinear behaviour was
observed. This suggests that constitutional verification introduces
predictable and scalable overhead suitable for block-level batching.

## 7.6 Lineage Cycle Detection

The Constitutional Resource Model prohibits cyclic lineage
relationships (Law L1). A malicious contract or validator could attempt
to construct a deep lineage chain and then close a cycle, forcing the
cycle detector to walk the entire ancestor graph. To evaluate the cost
of enforcing this invariant, cycle detection was measured across
lineage graphs of increasing depth. For each depth, a linear chain was
constructed, and a new derivation was proposed at the tip, triggering a
full ancestor walk and insertion into the ancestor cache. Each
measurement was repeated 20 times after 5 warmup iterations.

**Table 7.5 — Cycle Detection Time vs Lineage Depth**

| Depth | Detection Time (µs) | 95% CI |
|-------|---------------------|--------|
| 100 | 146.79 | ±7.83 |
| 500 | 1,066.80 | ±51.43 |
| 1,000 | 4,233.94 | ±161.19 |
| 2,000 | 13,835.47 | ±483.06 |
| 5,000 | 91,739.68 | ±17,057.80 |

*Observation.* Cycle detection cost increases with graph depth, and the
growth is superlinear — consistent with O(d²) behaviour where d is
lineage depth. This is expected because the current implementation
copies the full ancestor set at each derivation (O(d) per insertion,
repeated d times, yielding O(d²) for chain construction).

In practice, lineage depths exceeding 100 are rare (typical contract
chains have depth 1–5), and the ancestor cache ensures that query-time
cycle detection is O(1) after construction. The maximum depth is
capped at 2¹⁶ in the registry configuration. For applications requiring
frequent deep derivations, persistent set representations (hash array
mapped tries) can reduce construction cost to O(log d) per insertion.

The experiment confirms that constitutional lineage enforcement is
computationally feasible for realistic depths while preventing
replay-based lineage forgery attacks (G3, G4).

## 7.7 Witness Bundle Size Analysis

Replay verification relies on witness bundles embedded within
TransitionProofs (G2). Large witnesses would increase certificate size
and bandwidth requirements for light clients and cross-chain bridges.
To evaluate witness compactness, witness bundle sizes were measured for
Halt and Transform workloads at varying state sizes.

**Table 7.6 — Witness Bundle Size**

| Workload | State Size Range | Witness Size |
|----------|-----------------|--------------|
| Halt | 1–1,000 | 816–825 bytes |
| Transform | 1–1,000 | 924–950 bytes |

*Observation.* Witness sizes remain below one kilobyte across all
tested configurations. The Transform workload produces slightly larger
witnesses (approximately 100 bytes more than Halt) because it includes
metadata for the produced resource. Witness size is independent of
global state size — consistent with the replay independence result in
Section 7.3.

These results indicate that replay verification data can be transported
efficiently across network boundaries and embedded within finality
certificates without significant bandwidth overhead.

## 7.8 Constitutional Law Verification Cost

The framework enforces six constitutional resource laws (R1–R6) and a
transformation legality matrix before consensus participation (G3). To
evaluate the cost of this enforcement, state root computation was
measured as the number of active resources increased. State root
computation requires hashing every active resource and building a
Merkle tree, which exercises the full resource metadata hashing path
used during law verification. Each measurement was repeated 30 times
after 5 warmup iterations.

**Table 7.7 — Law Verification Cost vs Resource Count**

| Resources | Verification Time (µs) | 95% CI |
|-----------|------------------------|--------|
| 1 | 0.32 | ±0.01 |
| 10 | 5.14 | ±0.49 |
| 100 | 54.86 | ±1.62 |
| 1,000 | 687.47 | ±11.75 |

*Observation.* Verification cost scales approximately linearly with
resource count. Even for 1,000 resources, verification remains below
700 µs. In a production setting where blocks typically contain hundreds
to thousands of resources, the constitutional law verification step
adds sub-millisecond latency to the consensus path. This demonstrates
that constitutional law enforcement is practical within the critical
path of block finalisation.

## 7.9 Cross-System Comparison

Table 7.8 provides a qualitative comparison of security properties
across blockchain systems. The comparison focuses on properties
relevant to execution verification and evidence-backed finality.

**Table 7.8 — Cross-System Security Properties**

| Property | Ethereum | Solana | Aptos/Sui | Tendermint | AmunChain |
|----------|:---:|:---:|:---:|:---:|:---:|
| State-root voting | ✓ | ✓ | ✓ | ✓ | ✓ |
| Portable execution proofs | ✗ | ✗ | ✗ | ✗ | ✓ |
| Replay-before-vote requirement | ✗ | ✗ | ✗ | ✗ | ✓ |
| Constitutional law enforcement | ✗ | ✗ | ✗ | ✗ | ✓ |
| Evidence-backed finality | ✗ | ✗ | ✗ | ✗ | ✓ |
| Stateless verification | ✗ | ✗ | ✗ | ✗ | ✓ |

To the best of our knowledge, among the systems considered in this
comparison, AmunChain is the only system that embeds portable
per-transaction execution proofs into the consensus protocol, requires
validators to replay and verify those proofs before voting, enforces
constitutional resource laws at runtime, and binds execution evidence
into the finality certificate.

## 7.10 Security Discussion

The experiments support the central hypothesis of this paper.

Replay verification remains effectively constant with respect to global
state size, confirming that stateless verification (G2) is achievable
in practice. Execution correctness (G1) can be validated 1,900× to
5,100× faster than full transaction execution, making replay-backed
consensus computationally practical for validators.

Constitutional law enforcement (G3) introduces modest overhead while
providing strong guarantees regarding resource legality and lineage
integrity. The evidence framework ensures that detected violations
become cryptographically committed artifacts rather than transient
runtime events.

The Byzantine resilience experiments (Section 6.7, Appendix C)
demonstrate that the system defeats 11 attack vectors at the protocol
level, including forged proofs, double transfers, lineage cycles,
version regression, and parent hash forgery (G4).

Collectively, these properties enable a form of finality that commits
not only to state agreement but also to independently verifiable
execution correctness.

## 7.11 Limitations

Several limitations remain.

First, the experiments were conducted on a single machine using a
reference implementation and do not represent a fully optimised
production environment. Performance under geographically distributed
multi-validator deployment with network latency remains future work.

Second, the evaluation focuses primarily on protocol-level correctness
rather than validator economics, incentive compatibility, or
network-level attacks such as eclipse attacks or denial-of-service.

Third, the cycle detection experiments reveal O(d²) construction cost
for deep lineage chains. While practical depths are small (typically
1–5), applications requiring very deep derivations may need persistent
set representations to maintain performance.

Fourth, witness compression and proof aggregation mechanisms were not
explored and may further reduce verification costs and certificate
sizes in production deployments.

## 7.12 Conclusion

The experimental evaluation demonstrates that evidence-backed finality
is practical within the AmunChain architecture.

Replay-backed consensus provides strong execution correctness
guarantees while maintaining low verification costs. Replay
verification remains effectively independent of global state size,
witness bundles remain compact (under 1 KB), constitutional law
verification remains efficient (under 700 µs for 1,000 resources), and
execution correctness can be validated 1,900× to 5,100× faster than
full re-execution.

These results suggest that finality can be extended beyond state
agreement to include independently verifiable execution correctness
without sacrificing practical performance.
