# Performance Benchmark

**Status:** Framework Ready — Measurements Pending

**Baseline:** Commitment Layer V1

## Infrastructure

| Component | Status |
|-----------|--------|
| Benchmark crate | `amun-benchmarks` exists |
| Criterion framework | Configured |
| CI integration | `cargo bench` |

## Key Metrics

| Category | Metric | Current | Target |
|----------|--------|---------|--------|
| Throughput | TPS | TBD | >1000 |
| Throughput | Blocks/min | TBD | >60 |
| Latency | Finality time | TBD | <1 s |
| Latency | Vote propagation | TBD | <100 ms |
| Resources | Memory/validator | TBD | <2 GB |
| Resources | CPU utilization | TBD | <50% |

## Scope & Interpretation

This document defines the benchmarking framework, performance metrics, and
target objectives for AmunChain.

The values listed under **Target** are engineering objectives and must not be
interpreted as measured performance.

Actual benchmark results should be collected on dedicated hardware and appended
to this document after each benchmark campaign.

Every benchmark report should include:

- Hardware specification
- Operating system
- Rust toolchain version
- Build profile
- Validator count
- Workload description
- TPS
- Finality latency
- CPU utilization
- Memory consumption
- Network bandwidth
- Disk I/O

## Recommendations

1. Benchmark on dedicated hardware.
2. Establish a 4-validator baseline.
3. Repeat for 10, 100, 500 and 1000 validators.
4. Track regressions in CI.
