# Signature Replay Binding

## Principle

Constitutional signatures must be replayable.
Signature verification must produce identical results on any node.

---

## Deterministic Signing

Signatures are deterministic when:
- Same artifact produces same signature payload
- Same authority produces same signature
- Same timestamp produces same binding

---

## Replay Verification

Any node can replay signature verification:
1. Load artifact
2. Compute artifact hash
3. Extract signature payload
4. Verify cryptographic signature
5. Verify authority legitimacy
6. Verify temporal consistency

All steps are deterministic.

---

## Signature Chain

Signatures form a chain:
Genesis -> Amendments -> Rotations -> Federation

Each signature references previous constitutional state.
Broken chains are detectable.

---

## Invariant

Signature verification is replayable constitutional infrastructure.
