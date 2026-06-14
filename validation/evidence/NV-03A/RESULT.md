# NV-03A — Validator Runtime Determinism — Result
Gate: NV-03A | Risk: HIGH | Baseline: 90f4993
## PASS Condition
Four independent ValidatorNode instances produce identical state roots after executing propose_block().
## Result
| Validator | Final Root | Match? |
| :--- | :--- | :--- |
| 0..3 | `000...000` | YES |
Note: State root remained zero due to no resource mutations (D-002).
## Decision: PASS
Date: 2026-06-14
