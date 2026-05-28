# AmunChain Validator Ordering Law

## Constitutional Principle

Validators are ordered by their ID for all consensus-critical operations:

- Vote aggregation
- Quorum calculation
- Slashing evidence
- Epoch transitions
- Snapshot generation

## Ordering Function

Validators are sorted by `validator_id` (u64) in ASCENDING order.

## Data Structures

All collections storing validator information MUST use:
- `BTreeMap<u64, T>` or
- `BTreeSet<u64>`

`HashMap` is FORBIDDEN in consensus path for validator data.

## Rationale

Deterministic iteration order is required for:
- Replay equivalence
- Signature aggregation
- Quorum certificate generation
- Cross-node state consistency

## Enforcement

The constitutional enforcer (`tools/verify.sh`) will reject code using `HashMap` for validator-related data in consensus crates.

## Future Amendments

Validator ordering can only be changed via constitutional amendment requiring 2/3 validator approval.
