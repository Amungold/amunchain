# Constitutional Signature Format v1

## Format Specification

Every constitutional signature MUST include:

1. Artifact Type (manifest, amendment, federation, freeze)
2. Artifact Hash (the cryptographic hash of the artifact)
3. Specification Hash (the constitutional identity at signing)
4. Genesis Hash (lineage root)
5. Authority Key Fingerprint
6. Timestamp (constitutional time)
7. Signature (cryptographic signature over fields 1-6)

---

## Signature Schema

```
ConstitutionalSignature {
    artifact_type: String,
    artifact_hash: Hash,
    specification_hash: Hash,
    genesis_hash: Hash,
    authority_fingerprint: Fingerprint,
    timestamp: ConstitutionalTimestamp,
    signature: CryptographicSignature
}
```

---

## Verification Rules

To verify a constitutional signature:
1. Reconstruct signature payload from fields 1-6
2. Verify cryptographic signature against authority key
3. Verify specification hash matches current constitutional identity
4. Verify genesis hash matches lineage root
5. Verify timestamp is chronologically valid
6. Verify authority is legitimate at timestamp

---

## Invalid Signatures

A signature is constitutionally invalid if:
- Any field is missing
- Cryptographic verification fails
- Specification hash does not match
- Genesis hash does not match lineage root
- Authority is revoked at timestamp
- Timestamp is in the future
