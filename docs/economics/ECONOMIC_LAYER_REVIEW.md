# AmunChain Economic Layer Review

**Date:** 2026-06-25
**Status:** IN PROGRESS
**Objective:** Audit all economic components before completing the economic execution engine.

---

## 1. What Exists Today

### 1.1 Economic Data Structures (crates/amun-tokenomics, crates/amun-tokenomics-ledger)
| Component | Status | Location |
|:---|:---|:---|
| EpochEconomics | Exists | `amun-tokenomics/src/lib.rs` |
| EconomicLedger | Exists | `amun-tokenomics-ledger/src/lib.rs` |
| EconomicTree (CCA) | Exists | `amun-constitutional-commitment/src/economic_tree.rs` |
| EconomicSnapshot | Exists | `amun-constitutional-commitment/src/economic_snapshot.rs` |
| Total Supply | Defined | NTR constants |
| Treasury Allocation | Defined | NTR constants |
| Staking Allocation | Defined | NTR constants |
| Ecosystem Allocation | Defined | NTR constants |
| Inflation Rate | Defined | NTR constants |

### 1.2 Economic Roots (crates/amun-constitutional-commitment)
| Component | Status | Location |
|:---|:---|:---|
| economic_root in Block | YES | `amun-block-builder/src/lib.rs` |
| economic_root in FinalizedChainRecord | YES | `amun-chain-store/src/record.rs` |
| economic_root in RPC | YES | `amun-rpc/src/constitutional.rs` |
| ConstitutionalRoot from economic_root | YES | CCA spec v1.0 |

### 1.3 Economic Modules
| Module | Exists | Wired to Execution |
|:---|:---|:---|
| Dynamic Fees | Partial | Not verified |
| Burn Account | Partial | Not verified |
| Treasury | Partial | Not verified |
| Gharmeen Fund | Partial | Not verified |
| Validator Bonds | Partial | Not verified |
| Reward Distribution | Partial | Not verified |
| Slashing Economics | Partial | Not verified |
| Supply Conservation | Partial | Not verified |
| Effective Supply | Partial | Not verified |

---

## 2. Key Questions to Answer

### 2.1 State Storage
- Where is economic state stored? (`AccountStore`, separate `EconomicLedger`, or both?)
- Is it persisted with every block?
- Is it included in snapshots and recovery?

### 2.2 State Mutation
- What triggers economic state changes? (Transactions? EndBlock? Both?)
- Are fees deducted from senders?
- Are rewards distributed to validators?
- Is treasury updated automatically?

### 2.3 Root Computation
- Who computes `economic_root`? (BlockBuilder? EndBlock? Both?)
- Is it computed from live state or cached values?
- Is it recomputed during block validation?

### 2.4 Validation
- Does every validator recompute `economic_root` on block receipt?
- Is `economic_root` checked against the block header?
- Does mismatch cause block rejection?

### 2.5 Unused Components
- Are there economic constants defined but never used?
- Are there economic functions defined but never called?
- Are there economic fields stored but always zero?

---

## 3. Gap Analysis

### Phase I: Economic Roots Integration
| Task | Status | Priority |
|:---|:---|:---|
| economic_root in Block | DONE | — |
| economic_root in FinalizedChainRecord | DONE | — |
| economic_root in RPC | DONE | — |
| economic_root in Block Hash | DONE | — |

### Phase II: Economic Execution Engine
| Task | Status | Priority |
|:---|:---|:---|
| Fee deduction on transfer | UNKNOWN | HIGH |
| Fee distribution to treasury | UNKNOWN | HIGH |
| Fee distribution to burn | UNKNOWN | HIGH |
| Reward calculation per epoch | UNKNOWN | HIGH |
| Reward distribution to validators | UNKNOWN | HIGH |
| Slashing amount deduction | UNKNOWN | HIGH |

### Phase III: Economic Merkle State
| Task | Status | Priority |
|:---|:---|:---|
| Economic Merkle Tree builder | EXISTS | — |
| Live EconomicSnapshot from state | UNKNOWN | HIGH |
| economic_root recomputed in EndBlock | UNKNOWN | CRITICAL |

### Phase IV: Economic Validation
| Task | Status | Priority |
|:---|:---|:---|
| Validator recomputes economic_root | UNKNOWN | CRITICAL |
| economic_root mismatch causes rejection | UNKNOWN | CRITICAL |

### Phase V: Constitutional Economy
| Task | Status | Priority |
|:---|:---|:---|
| Supply Conservation as constitutional rule | UNKNOWN | MEDIUM |
| Burn limit as constitutional rule | UNKNOWN | MEDIUM |
| Reward compliance as constitutional rule | UNKNOWN | MEDIUM |
| Fee change only via governance | UNKNOWN | MEDIUM |

---

## 4. Next Steps

1. **Audit `amun-tokenomics` and `amun-tokenomics-ledger`** — identify all defined vs. used components.
2. **Trace `economic_root` computation** — find exactly where it is computed and from what data.
3. **Trace transaction execution** — verify fees, balances, rewards are actually updated.
4. **Identify the gap** — find the exact line where economic state SHOULD be updated but ISN'T.
5. **Implement the missing link** — wire economic execution into the block production pipeline.
