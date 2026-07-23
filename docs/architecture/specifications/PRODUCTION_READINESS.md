# AmunChain Production Readiness

**Status:** Core Verified — Campaigns Pending  
**Date:** Post ADR-030 era

## Verified (Core Architecture)

| Layer | Status | Evidence |
|-------|--------|----------|
| Consensus Engine | ✅ | QC, weighted voting, finality chain, equivocation detection |
| Bootstrap + Identity | ✅ | Auto-discovery, cert generation, peer registration |
| Block Lifecycle | ✅ | 684 blocks, 4/4 consensus, 0 unknown, 0 panics |
| Determinism | ✅ | BTreeMap ordering, no RNG in consensus, SystemTime only informational |
| Storage/WAL | ✅ | Offset+length framing, flush, duplicate detection, recovery |
| Networking | ✅ | 1MB limit, WouldBlock handling, retry logic |

## Pending (Production Campaigns)

| Campaign | Priority | Duration | Goal |
|----------|----------|----------|------|
| Byzantine Fault | P0 | 1 week | Equivocation, invalid QC, double proposal — network survives |
| Network Partition | P0 | 1 week | 2/2 split → reconnect → automatic recovery, no fork |
| Crash Consistency | P0 | 1 week | Kill during append/finalize/QC → restart → consistent state |
| Long Soak | P0 | 72 hours | Memory, FDs, CPU, locks — no leaks, no degradation |
| Scale Test | P0 | 2 weeks | 4→16→32→64→100 validators, monitor QC latency, memory, bandwidth |

## P2 Optimizations

| Item | Impact |
|------|--------|
| Persistent connections (connection pool) | Reduces TCP handshake overhead |
| Peer health tracking (RTT, failures) | Better peer selection |
| Explicit fsync for WAL | Crash durability |
| Per-record checksum | Corruption detection |
| Dynamic peer discovery | Operational flexibility |

## Decision

**AmunChain is Core Verified. Production readiness requires completion of the 5 campaigns above.**
