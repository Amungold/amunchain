# ADR-021: Consensus Observability Layer

**Status:** Accepted  
**Date:** 2026-07-21  
**Author:** Eng. Mohamed Attia  
**Branch:** `feature/adr-021-consensus-observer`

---

# 1. Context

The AmunChain consensus engine (`ConsensusEngine`) is a deterministic core component and must remain free from diagnostics, logging, metrics, and debugging logic.

Previous debugging approaches relied on temporary `println!` and `eprintln!` statements directly inside consensus execution paths. Although useful during investigation, this approach introduced several architectural problems:

- Polluted production consensus code with diagnostic logic.
- Required source code modification for every debugging session.
- Prevented reusable instrumentation.
- Made observability tightly coupled to consensus implementation.
- Increased long-term maintenance cost.

A permanent observability architecture was therefore required while preserving deterministic consensus behavior.

---

# 2. Decision Drivers

The solution must satisfy the following architectural goals:

- Preserve deterministic consensus execution.
- Keep production consensus code completely independent from diagnostics.
- Support multiple concurrent observers.
- Allow future replay, metrics, auditing, and tracing without modifying consensus.
- Introduce zero observer overhead when disabled.
- Be suitable for production deployments as well as debugging environments.

---

# 3. Decision

A dedicated crate named:

```
crates/amun-consensus-observer
```

was introduced.

The crate provides an independent observability layer that can subscribe to consensus runtime events without introducing any dependency from the consensus engine toward diagnostics.

The design consists of:

- `ConsensusObserver` trait
- `ObserverHub`
- `RuntimeObserver`
- `RoundTracer`
- `RuleEngine`
- Future observer implementations

All runtime integration points are protected by:

```rust
#[cfg(feature = "consensus-observer")]
```

ensuring that observer logic is completely removed from production builds when the feature is disabled.

---

# 4. Architecture

```
ConsensusRuntime
        │
        │ #[cfg(feature = "consensus-observer")]
        ▼
RuntimeObserver
        │
        ▼
ObserverHub
        │
 ┌──────┴───────────────────────────┐
 │                                  │
 ▼                                  ▼
RoundTracer                     RuleEngine
 │                                  │
 ▼                                  ▼
Future Metrics              Future Replay Analyzer
```

The consensus engine remains unaware of any observer implementation.

---

# 5. Event Lifecycle

```
ValidatorRegistered
        │
        ▼
LeaderSelected
        │
        ▼
RoundStarted
        │
        ▼
ProposalCreated
        │
        ▼
ProposalReceived
        │
        ▼
VoteSent
        │
        ▼
VoteReceived
        │
        ▼
QuorumCertificateFormed
        │
        ▼
BlockFinalized
```

Additional event families may be added without modifying consensus logic.

---

# 6. Event Model

Every emitted event carries deterministic metadata:

- `event_id: u64`
  - Monotonically increasing unique identifier.

- `parent_event_id: Option<u64>`
  - Links causally related events.

- `round_correlation_id: u64`
  - Correlates all events belonging to the same consensus round, including retries and view changes.

- `sequence: u64`
  - Preserves deterministic replay ordering.

- `logical_height: u64`
  - Blockchain height associated with the event.

This metadata enables deterministic replay and timeline reconstruction.

---

# 7. Core Components

## ConsensusObserver

Defines the observer interface.

Characteristics:

- Uses `&self`
- Compatible with shared ownership through `Arc`
- Thread-safe
- Extensible without modifying consensus

---

## ObserverHub

Responsibilities:

- Registers observers.
- Dispatches events.
- Uses:

```rust
Arc<dyn ConsensusObserver>
```

allowing multiple observers to coexist simultaneously.

---

## RuntimeObserver

Provides a thin façade between `ConsensusRuntime` and `ObserverHub`.

Responsibilities:

- Translate runtime activity into observer events.
- Keep consensus runtime independent of observer implementations.

---

## RoundTracer

Provides:

- Ring-buffer event storage
- Monotonic timestamps
- Timeline reconstruction
- Low-overhead tracing

Implementation:

```
VecDeque
```

with bounded capacity.

---

## RuleEngine

Consumes events and evaluates consensus correctness.

Supports:

- Safety rules
- Liveness rules
- Future invariant verification
- Audit reporting

---

# 8. Integration Pattern

Runtime integration follows the pattern:

```rust
#[cfg(feature = "consensus-observer")]
if let Some(observer) = &self.runtime_observer {
    observer.round_started(
        height,
        round,
        validator_id,
        my_index,
        proposer_index,
    );
}
```

Consensus code emits semantic events only.

Observers decide how those events are consumed.

---

# 9. Verified Integration Points

The following runtime integration points were validated during ADR-021 development:

- Leader selection
- Round start
- Proposal creation
- Vote submission
- Quorum certificate formation

Temporary instrumentation confirmed the complete consensus flow:

```
LEADER_SELECTED
        │
        ▼
ROUND_STARTED
        │
        ▼
PROPOSAL_CREATED
        │
        ▼
VOTE_SENT
        │
        ▼
QC_FORMED
```

The temporary instrumentation was removed after validation.

No diagnostic statements remain in production code.

---

# 10. Alternatives Considered

## Embedded println!/eprintln!

Rejected.

Reasons:

- Pollutes production code.
- Difficult to maintain.
- Not reusable.
- Requires repeated code modifications.

---

## Direct callbacks inside ConsensusEngine

Rejected.

Reasons:

- Couples diagnostics to consensus implementation.
- Makes extending observability increasingly expensive.

---

## Global Singleton Observer

Rejected.

Reasons:

- Hidden global dependency.
- Poor testability.
- Reduced modularity.
- Harder lifecycle management.

---

# 11. Performance Budget

Design goals:

- Observer dispatch performs read-only access.
- Observer registration occurs only during initialization.
- `RoundTracer` uses bounded `VecDeque`.
- No observer-related allocations or dispatch occur when the feature is disabled.
- No observer synchronization exists in production builds with the feature disabled.

---

# 12. Feature Flag

```toml
[features]
default = []

consensus-observer = []
```

When disabled:

- No observer code executes.
- No observer objects are constructed.
- No observer dispatch occurs.

Production behavior remains identical to builds without ADR-021.

---

# 13. Consequences

## Positive

- Clean separation between consensus and observability.
- Reusable observer infrastructure.
- Supports multiple simultaneous observers.
- Zero observer overhead when disabled.
- Easy extension without consensus modification.
- Suitable for diagnostics, replay, metrics, and auditing.

## Neutral

- Shared ownership requires `Arc`.
- Observer implementations use internal synchronization (`Mutex`, `RwLock`, etc.) where necessary.

## Negative

- Slightly larger architecture due to the additional observability layer.
- New observer implementations require explicit registration with `ObserverHub`.

---

# 14. Future Extensions

Potential future observers include:

- Consensus metrics exporter
- Prometheus integration
- OpenTelemetry tracing
- Persistent WAL replay observer
- Catch-up path observer
- Replay observer
- Snapshot observer
- Storage observer
- Networking observer

A future categorization may introduce:

```rust
enum ConsensusEventCategory {
    Consensus,
    Networking,
    Catchup,
    Replay,
    Snapshot,
    Storage,
    Metrics,
}
```

without affecting the existing event model.

---

# 15. Validation

ADR-021 was validated by:

- Building the complete workspace successfully.
- Dedicated unit tests (`6/6` passing).
- Temporary runtime instrumentation.
- Verification of the complete consensus round lifecycle.
- Restoration of all modified production files after validation.

Final repository state:

- No temporary `println!` or `eprintln!` statements.
- Clean production consensus implementation.
- Observer crate isolated and production-ready.

---

# 16. Final Decision

ADR-021 establishes **Consensus Observability** as an independent architectural layer.

The consensus engine remains deterministic and free from diagnostic concerns.

All future tracing, auditing, replay analysis, metrics collection, and synchronization diagnostics shall be implemented through the observer framework rather than modifying consensus execution logic.

**ADR-021 is accepted and approved for permanent inclusion in the AmunChain architecture.**
