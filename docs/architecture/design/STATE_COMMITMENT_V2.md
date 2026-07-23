# State Commitment V2

## Objective

Replace flat deterministic hashing with a Sparse Merkle Tree while preserving
determinism and protocol compatibility.

---

## Components

ExecutionEngine

↓

StateCommitmentEngine

↓

SparseMerkleTree

↓

Block Header

---

## Engine API

compute_root()

update()

generate_proof()

verify_proof()

---

## Hash Domains

AMUN_STATE_LEAF_V1

AMUN_STATE_NODE_V1

---

## Tree

256-bit sparse key space

Leaf = Account

Node = Blake3(left || right)

---

## Serialization

CanonicalEncode(Account)

No serde_json

Deterministic only

---

## Complexity

Update:

O(log n)

Proof:

O(log n)

Verification:

O(log n)

Memory:

Sparse

---

## Future Features

Light Client

Stateless Validation

Cross-chain Proofs

Fraud Proofs

Bridge Verification

