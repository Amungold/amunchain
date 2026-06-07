# V2-003 Discovery: Event-Driven Execution Exposes Synchronization Dependency

**Date:** 2026-05-30
**Baseline:** f47ec95

## Hypothesis
A deterministic event scheduler can replace the implicit global phase scheduler of v0.1 and reproduce the f47ec95 baseline under packet loss.

## Experiment
Created an `EventDrivenSim` using a priority queue of events (`ProposalBroadcast`, `PrevoteBroadcast`, `PrecommitBroadcast`, `ProcessInbox`, `CheckTimeout`). Ran a 700-trial campaign across 0-30% packet loss.

## Results
| Loss Rate | f47ec95 (Global Phases) | V2-003 (Event Scheduler) |
|:---------:|:-----------------------:|:------------------------:|
| 0%        | 100% success (40/40)    | 100% success (39/40)     |
| 5%        | 100%                    | 100%                     |
| 10%       | 100%                    | 73%                      |
| 15%       | 100%                    | 8%                       |
| 20%       | 98%                     | 0%                       |
| 25%       | 36%                     | 0%                       |
| 30%       | 0%                      | 0%                       |

## Conclusion
The event scheduler does not reproduce the f47ec95 baseline. The transition from a globally synchronized phase execution model to an event-driven model results in significantly degraded liveness under message loss. The experimental data demonstrate that the original baseline performance depends on execution properties provided by the global phase scheduler.

However, the data do not yet isolate the exact cause. Several variables remain incompletely controlled for:

- Self-delivery of proposals to the leader is not yet resolved (39/40 at 0% loss).
- Recovery rounds are not a faithful reproduction of the original implementation.
- Proposal retries are not identically integrated.
- Timeout semantics differ from the baseline.
- Global event ordering differs fundamentally from the original test harness.

Therefore, the observed discrepancy does not yet constitute a proof of a "structural dependency on synchronization" in the consensus logic itself. It is a strong signal that requires further investigation under conditions where the above variables are fully normalized.

## New Research Question (V2-004)
After restoring self-delivery, proposal retries, and recovery rounds from the f47ec95 baseline, does the event-driven model converge toward the original baseline results? If degradation persists, a structural dependency on synchronized execution becomes the leading hypothesis for the first architectural discovery of v0.2.

## Decision
Proceed to V2-004: Normalize self-delivery, proposal retries, recovery rounds, and timeout semantics to match f47ec95 as closely as possible within the event-driven framework, then re-evaluate at 10%, 15%, and 20% loss.
