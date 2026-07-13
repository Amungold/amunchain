# V3-004: Aggregated Signature Verification Framework

**Date:** 2026-05-31
**Status:** Phase Complete – Awaiting Cryptographic Integration

## What V3-004 Proves
The constitutional evidence verification framework is structurally complete. All
verification gates are in place and operational:
1. **Height mismatch detection** – rejects QC with wrong height
2. **Validator set binding** – rejects evidence from foreign validator sets
3. **QC membership verification** – rejects evidence with mismatched signer bitmaps
4. **Signature verification** – framework detects missing aggregated signatures
5. **Epoch validation** – rejects stale evidence with invalid epochs

## Current Status
The framework correctly identifies that current QCs lack real aggregated signatures
and rejects them. This is CORRECT behavior – the system refuses to accept evidence
that cannot be cryptographically proven.

When real aggregated signatures are integrated into the QC structure, the
constitutional evidence pipeline will accept properly signed certificates and
reject forged ones.

## Path Forward
V3-005+: Integrate real aggregated signature generation and verification
using BLS or EdDSA, then re-run the full evidence verification suite.
