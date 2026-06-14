# NV-02A — State Mutation Determinism — Result
Gate: NV-02A | Risk: CRITICAL | Baseline: 90f4993
## PASS Condition
Applying 3 identical resources to 4 independent stores produces identical non-zero state roots.
## Result
| Validator | Initial Root | Final Root |
| :--- | :--- | :--- |
| 0 | `000...000` | `afe120e7...` |
| 1 | `000...000` | `afe120e7...` |
| 2 | `000...000` | `afe120e7...` |
| 3 | `000...000` | `afe120e7...` |
## Decision: PASS
Date: 2026-06-14
