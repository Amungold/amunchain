# Fork Choice Law v1.0

## Article I: Preferred Chain Determinism

Given identical block DAGs, all validators MUST select the same
preferred chain. The fork choice function is a PURE FUNCTION:

preferred_chain = fork_choice(dag, finalized_block, justified_block)

No external state, no randomness, no wall clock.

## Article II: Ancestor Monotonicity

If block A is finalized before block B, then A MUST remain an
ancestor of B in the preferred chain forever:

is_finalized(A) and is_finalized(B) and A.height < B.height
implies is_ancestor(A, B)

## Article III: Tie-Break Canonicalization

When two chains have equal accumulated vote weight, the chain
whose head has the lexicographically smaller block hash wins.
This is a MATHEMATICAL TOTAL ORDER over block hashes.

## Article IV: No Cycles

Every block's parent MUST have a strictly lower block height:
parent.height < child.height
This prevents cycles in the block DAG.

## Article V: Canonical Traversal

All DAG traversal operations MUST be:
- Iterative (not recursive) with bounded stack depth
- Deterministic: same DAG produces same traversal order
- Platform-independent across x86_64, aarch64, wasm32
- Traversal queues and stacks must have canonical ordering

## Article VI: Heaviest Subtree Rule

Among competing chains extending from the same finalized block,
the chain with the highest accumulated vote weight (by stake)
is preferred.
Vote weight = sum of stake of validators who voted for blocks
in the chain, weighted by the QC round.

## Article VII: Bounded Ancestry

Ancestry traversal depth MUST be bounded by MAX_ANCESTRY_DEPTH = 1000.
Any traversal exceeding this depth must return an error,
not silently truncate or loop.

## Article VIII: Pruning Safety

Blocks below the finalized height may be pruned.
No block above the finalized height may be pruned.
After pruning, the finalized block becomes the virtual genesis.
