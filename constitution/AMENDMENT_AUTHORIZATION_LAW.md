# Amendment Authorization Law

## Principle

Constitutional amendments require cryptographic authorization.
Unsigned amendments are constitutionally void.

---

## Authorization Chain

### Proposal Authorization
- Signed by amendment authority
- Bound to current specification hash
- Must specify target specification hash

### Activation Authorization
- Signed by quorum authority
- Threshold signatures required
- Bound to proposal hash and specification hash

### Migration Authorization
- Signed by genesis authority (or delegated)
- Certifies replay compatibility
- Certifies lineage continuity

---

## Amendment Legitimacy

An amendment is legitimate if:
1. Proposal is signed by amendment authority
2. Activation meets quorum threshold
3. Migration is certified by genesis authority
4. New specification hash is canonical
5. Lineage continuity is preserved

---

## Invalid Amendments

An amendment is void if:
- Any signature is missing
- Quorum threshold is not met
- Frozen properties are violated
- Lineage continuity is broken
- Specification hash is not canonical
