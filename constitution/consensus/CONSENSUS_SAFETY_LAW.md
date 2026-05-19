# Consensus Safety Law v1.0

## Article I: Finality Safety

**Axiom 1 (No Conflicting Finality):**
No two conflicting blocks may be finalized at the same height.
A block B1 conflicts with B2 if neither is an ancestor of the other.

**Axiom 2 (Finality Monotonicity):**
Once a block is finalized, it remains finalized forever.
The finalized prefix can only grow, never shrink.

**Axiom 3 (Finality Requires Quorum):**
A block is finalized only if there exists a Quorum Certificate
with signatures from >= 2f+1 distinct validators from the
correct epoch validator set.

## Article II: Locking Rules

**Rule 1 (Lock Monotonicity):**
A validator's locked round must monotonically increase.
It must never decrease, even across epoch transitions.

**Rule 2 (Lock Inheritance):**
When voting for a block B at round r, the validator inherits
the lock from the highest QC known. The new lock must be >= the old lock.

**Rule 3 (Lock Release):**
A validator may release its lock only when:
- A new QC at a higher round justifies a different chain, OR
- An epoch boundary forces validator set rotation.

## Article III: Voting Rules

**Rule 1 (One Vote Per Round):**
A validator must not cast more than one vote per round.
Casting two different votes for the same round is EQUIVOCATION.

**Rule 2 (Extend Preferred Chain):**
A validator must only vote for blocks that extend its preferred chain.
The preferred chain is determined by the fork-choice rule.

**Rule 3 (Valid Block Required):**
A validator must not vote for a block whose execution root
does not match the deterministic execution of its transactions.

## Article IV: Quorum Certificate Rules

**Rule 1 (Distinct Signers):**
A Quorum Certificate must contain signatures from DISTINCT validators.
Duplicate signatures invalidate the QC.

**Rule 2 (Correct Epoch):**
Signatures must be from validators active in the QC's epoch.
Signatures from validators not in the epoch's validator set are rejected.

**Rule 3 (Canonical Aggregation Order):**
Signatures must be aggregated in canonical order (sorted by validator ID).
The ordering must be deterministic across all implementations.

## Article V: Fork Choice Rules

**Rule 1 (Deterministic Choice):**
Given identical block DAGs, all validators must select the same
preferred chain. The fork choice function must be PURE.

**Rule 2 (Heaviest Subtree):**
Among competing chains, the one with the most accumulated votes
(by stake weight) is preferred.

**Rule 3 (Tie-Breaking):**
In case of equal vote weight, the chain whose head has the
lexicographically smaller block hash wins. This is a TOTAL ORDER.

**Rule 4 (Preferred Chain Stability):**
If no new blocks or QCs arrive, fork_choice() must return
the same result. The preferred chain must not change without new input.

## Article VI: Epoch Transition Rules

**Rule 1 (Quorum Continuity):**
Validator set transitions require overlap between old and new sets:
overlap_stake >= max(f_old, f_new) + 1

**Rule 2 (Activation Delay):**
New validators become active after activation_delay rounds from
the epoch boundary block.

**Rule 3 (Lock Preservation):**
Locks survive epoch transitions. A validator locked at epoch e
remains locked at epoch e+1 until the lock is released by rules above.
