N110 — Provable Economic Slashing Pipeline
==========================================

Status: COMPLETE
Commit: f628ad4
Tag:    N110_COMPLETE
Branch: feature/n105-validator-identity
Date:   2026-06-17

Overview
--------
N110 is the economic enforcement layer of AmunChain. It closes the gap
between cryptographic evidence (N109) and real stake reduction on the
validator registry. Before N110 the system could detect, record, and
score misbehavior, but it could not financially penalize a validator
in a way that was provable, auditable, and consensus-enforced.

N110 adds five capabilities that together form a complete slashing
pipeline:

1. Real staking integration — the `ValidatorRegistry` from `amun-staking`
   is wired into the consensus engine through a `StakingAdapter` trait,
   replacing the earlier simulated executor.

2. SlashingCertificate — a serialisable, hashable, verifiable
   constitutional object that carries validator identity, evidence
   references, penalty details, and resulting stake.

3. Certificate gossip — certificates are deduplicated, stored locally,
   and broadcast to peers via the transport layer so that every node
   can independently verify a slash.

4. Block inclusion and consensus verification — certificates are
   embedded in blocks and validated before a validator casts its vote,
   ensuring that no block with a forged or tampered certificate can
   receive a supermajority.

5. Finality-triggered execution — the actual stake reduction happens
   strictly after a quorum certificate is formed and the block is
   appended to the chain store, with replay protection through a
   per-validator applied-certificate set.

Architecture
------------
The pipeline follows a single linear path. Every step feeds into the
next, and each step can be independently tested.

  Misbehaviour Detected (double-vote, invalid commitment, state-root
  mismatch, etc.)
       │
       ▼
  EvidenceRecord stored in EvidenceStore .................. N109.10
       │
       ▼
  MisbehaviorRegistry accumulates score per validator .... N109.11
       │
       ▼
  Threshold crossed → SlashingCertificate created ........ N110.2
       │
       ▼
  CertificateGossip stores, deduplicates, and broadcasts . N110.3
       │
       ▼
  Transport layer sends SlashingCertificateAnnounce ...... N110.3b
       │
       ▼
  BlockBuilder includes pending certificates in block .... N110.4a
       │
       ▼
  Block.verify_slashing_certificates() before voting ..... N110.4b
       │
       ▼
  QC formed → block appended → StakingAdapter.try_slash() N110.4c
       │
       ▼
  ValidatorRegistry.stake reduced; certificate marked
  as applied in per-validator set to prevent replay

Test Inventory
--------------
Total: 103 tests (0 failures)

N109.1  – N109.6   Block propagation & validation ........ 13
N109.7             Deterministic re-execution ............  8
N109.8             Execution commitment .................. 15
N109.9             Vote binding ..........................  8
N109.10            Evidence store ........................ 11
N109.11            Misbehavior registry .................. 12
N109.12            Integrated slashing pipeline ..........  6
N109.13            Unified slashing interface ............  6
N110.1             Staking adapter (simulated) ...........  4
N110.1b            Real staking executor .................  1
N110.2             Slashing certificate ..................  4
N110.3             Certificate gossip ....................  5
N110.3b            Transport roundtrip ...................  1
N110.4a            Block inclusion (build) ...............  -
N110.4b            Consensus verification ...............  6
N110.4c            State transition after finality .......  3

Key gatekeeper tests:
  n110_1b_real_staking_slash_reduces_validator_stake
      Proves that amun_staking::ValidatorRegistry is
      mutated through the StakingAdapter.

  n110_4b_valid_certificates_pass_verification
      Proves that a block carrying valid certificates
      passes verify_slashing_certificates().

  n110_4c_slash_applied_after_finality
      Proves that stake is reduced only after the block
      is appended to the chain store.

Security Assumptions
--------------------
1. A certificate is only as trustworthy as the evidence it
   references.  The evidence store must be consistent across
   nodes.  If a node misses evidence, it will reject a
   certificate that other nodes accept.

2. Certificate deduplication relies on certificate_hash.
   Two semantically identical certificates with different
   timestamps will have different hashes and will not be
   deduplicated automatically.  The gossip layer handles
   this by checking the hash before broadcasting.

3. The applied_certificates set lives in memory.  On restart,
   it is rebuilt from the chain store by replaying blocks.
   This is correct but not optimised for very long chains.

4. ValidatorRegistry::slash() is called after every QC.
   If a validator misbehaves repeatedly, they will be slashed
   multiple times.  The registry's max_slash_count and
   is_active flag provide the final safety net.

Known Limitations
-----------------
1. PublicKey is 48 bytes in amun-kernel-types but validator
   identifiers in N109/N110 are 32 bytes.  The mapping pads
   with zeros.  A native 32-byte key or an identity registry
   that links the two should be introduced in a future phase.

2. Certificate gossip currently adds the SlashingCertificate
   variant to the consensus NetworkMessage (amun-consensus-
   network), not to the transport-level NetworkMessage
   (amun-network-transport).  The transport layer has a
   separate SlashingCertificateAnnounce struct for on-wire
   efficiency, but the integration between the two types is
   manual.  A future task can unify them.

3. The system does not yet have evidence gossip.  Evidence
   records are created locally.  If a node does not witness
   the original misbehaviour, it will reject a certificate
   that references unknown evidence IDs.  Evidence
   propagation is planned for N111.

4. There is no slash refund mechanism.  If a certificate is
   later proven invalid (e.g. the underlying evidence was
   forged), the slashed stake cannot be restored without
   governance intervention.  This is acceptable for the
   current threat model but should be revisited when
   fraud-proofs are introduced.

Conclusion
----------
N110 delivers the first complete economic slashing pipeline in
AmunChain.  The system can now detect misbehaviour, record it
permanently, score it, produce a verifiable certificate,
propagate that certificate through the network, embed it in
blocks, verify it during consensus, and execute the actual
stake reduction only after finality — with replay protection
at every level.  The pipeline is backed by 103 automated tests
that pass cleanly with no warnings in the N109/N110 modules.
