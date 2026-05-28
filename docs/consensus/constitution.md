# AmunChain Consensus Constitution v1.0

## Preamble

This document defines the constitutional rules for deterministic consensus execution in AmunChain. All validators MUST adhere to these rules. Deviation constitutes a consensus violation.

## Article I: Arithmetic Constitution

### Section 1.1: Fixed-Point Arithmetic

- SCALE = 1,000,000 (6 decimal digits)
- All values are represented as fixed-point integers
- NO floating point operations in consensus path

### Section 1.2: Division and Modulo

- Division uses FLOOR division (matches Python //)
- Modulo uses FLOOR modulo (matches Python %)
- Results MUST be bit-identical across all platforms

### Section 1.3: Overflow Handling

- SATURATING arithmetic (no wrapping, no panics)
- Legitimacy bounded to [0, SCALE]
- Maximum fixed value: 10,000,000,000 × SCALE

### Section 1.4: Rounding

- Banker's rounding (round-half-to-even)
- Used for all float-to-fixed conversions (display only)

## Article II: State Constitution

### Section 2.1: State Representation

- Accounts stored in BTreeMap (deterministic order)
- Each account has: balance, delegation, nonce
- State hash recomputed after each transition

### Section 2.2: Canonical Serialization

- Binary format only (NO JSON, NO protobuf)
- Big-endian byte order
- Sorted keys for all maps

### Section 2.3: Hashing

- SHA-256 only
- Canonical byte representation before hashing

## Article III: Event Ordering Constitution

### Section 3.1: Canonical Order

Events MUST be ordered by (in sequence):

1. block_height (ascending)
2. sender_id (ascending)
3. nonce (ascending)
4. event_type_priority (1=Mint,2=Reward,3=Transfer,4=Delegate,5=Undelegate,6=Slash,7=Burn)
5. event_hash (lexicographic)

### Section 3.2: Nonce Rules

- Nonces MUST increment sequentially per sender
- Duplicate nonces are FORBIDDEN
- Nonce validation is MANDATORY

## Article IV: Execution Constitution

### Section 4.1: Determinism

- Same block input => SAME execution outputs
- Outputs include: state_root, receipts_root, trace_root, snapshot_root

### Section 4.2: Failed Transitions

- Failed transitions MUST NOT mutate state
- State hash remains unchanged on failure
- Receipt MUST record failure with error code

### Section 4.3: Receipts

- Each transition produces a receipt
- Receipts form an accumulator hash (linear chain)
- Receipts include: pre/post state hashes, event hash, success, error code, gas used

## Article V: Block Constitution

### Section 5.1: Block Header

Required fields:
- version (u32)
- height (u64)
- parent_block_hash ([u8; 32])
- state_root ([u8; 32])
- receipts_root ([u8; 32])
- execution_trace_root ([u8; 32])
- event_root ([u8; 32])
- snapshot_root ([u8; 32])
- timestamp_logical (u64) - NOT wall-clock
- proposer_id (u64)

### Section 5.2: Block Body

- Events MUST be canonically ordered
- Block hash = SHA256(header_bytes || body_bytes)

## Article VI: Validator Constitution

### Section 6.1: Execution Replay

- Validators MUST replay every block before voting
- No optimistic acceptance, no trust shortcuts

### Section 6.2: Vote Requirements

Votes MUST certify:
- state_root
- receipts_root
- execution_trace_root
- snapshot_root
- replay_proof_hash

### Section 6.3: Quorum

- Byzantine fault tolerance: 2f + 1 where f = floor((N-1)/3)
- Quorum requires ALL execution outputs to be identical
- Duplicate validator votes are FORBIDDEN

## Article VII: Consensus Violations

The following are CONSENSUS VIOLATIONS:

1. Different state_root for same block
2. Different receipts_root for same block
3. Different execution_trace_root for same block
4. Non-canonical event ordering
5. Duplicate nonce from same sender
6. Non-deterministic arithmetic results
7. Voting without full replay

## Article VIII: Amendment Process

Any change to this constitution requires:
1. 2/3 validator vote
2. Hard fork coordination
3. Migration path for existing state

---

*Version: 1.0.0*
*Date: 2026-05-23*
*Status: FROZEN*
