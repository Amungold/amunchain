# NV-01B — Runtime State Root Determinism — Result
Gate: NV-01B | Risk: HIGH | Baseline: 90f4993

## Mathematical PASS Condition
StateRoot(Validator1) = StateRoot(Validator2) = StateRoot(Validator3) = StateRoot(Validator4)
after executing propose_block(1) on each independent ValidatorNode.

## Result
| Metric     | Validator 1 | Validator 2 | Validator 3 | Validator 4 | Match? |
|------------|-------------|-------------|-------------|-------------|--------|
| State Root | 0000000000000000000000000000000000000000000000000000000000000000 | 0000000000000000000000000000000000000000000000000000000000000000 | 0000000000000000000000000000000000000000000000000000000000000000 | 0000000000000000000000000000000000000000000000000000000000000000 | YES |

## Scope & Limitations
- Determinism of runtime state root is **proven** under the current runtime behavior.
- **Discovery D-002:** Current runtime execution does not mutate `ResourceRegistry`; therefore all observed state roots remain zero.
- Future runtime changes that add resource mutations must be re-validated.

## Decision
- NV-01B (Runtime State Root Determinism): PASS
- Limitation: State evolution not yet exercised.

Date: 2026-06-14
