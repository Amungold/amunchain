# N21 — Constitutional Identity Layer
## Overview
The N21 milestone establishes the constitutional identity framework for AmunChain. Where N20 answered "Who sent this message?" through cryptographic identity, N21 answers "Who is allowed to participate?" through constitutional identity. Validators must present a certificate signed by a trusted constitutional authority to join the active validator set. This transforms network identity from mere cryptographic proof-of-possession to constitutional proof-of-authorization. The constitutional identity layer is the foundation for governed participation, authority delegation, and long-term network governance.
## Architectural Objectives
The constitutional identity layer must provide: (1) Authority-signed validator certificates binding identity to cryptographic keys, (2) A registry of trusted constitutional authorities forming the root of trust, (3) A validator registry accepting only certificate-verified participants, (4) Cryptographic verification of all identity claims, (5) Defense against certificate forgery and impostor authorities, (6) Temporal validity with explicit issuance and expiration, (7) Foundation for validator rotation and membership governance.
## N21.1 — Validator Certificate
### Purpose
A ValidatorCertificate is a constitutional document signed by a trusted authority that binds a validator's peer identity to their Ed25519 public key. Possession of a key pair is not sufficient for network participation. The validator must also present a certificate proving that a constitutional authority recognizes them as a legitimate participant.
### Certificate Structure
The validator_id field identifies the validator. The public_key field contains the validator's Ed25519 verifying key, cryptographically binding the certificate to the key holder. The issuer field identifies the constitutional authority that issued the certificate. The valid_from field specifies when the certificate becomes active. The valid_until field specifies when the certificate expires, with zero indicating no expiration. The authority_signature field contains an Ed25519 signature over all preceding fields, created by the issuing authority.
### Issuance Process
The issuing authority calls ValidatorCertificate::issue with the validator's identity, public key, the authority's own signing key, and the validity window. The certificate fields are serialized in a deterministic order. The authority signs the serialized fields. The signature is embedded in the certificate. The resulting certificate can be verified by any party possessing the authority's public key.
### Verification Properties
Certificate verification recomputes the serialized fields and checks the authority's signature. Tampered certificates fail verification because the signature no longer matches the modified fields. Certificates signed by an impostor authority fail verification because the signature does not match the claimed issuer's public key. Certificates used before their valid_from timestamp are rejected. Certificates used after their valid_until timestamp are rejected. Full serialization roundtrip preserves all fields and the signature remains verifiable after deserialization.
## N21.2 — Trust Anchor Registry
### Purpose
The TrustAnchorRegistry is the root of cryptographic trust in the AmunChain network. It maintains the set of constitutional authorities whose signatures are accepted for validator certificates. Only certificates issued by a registered trust anchor are accepted for validator registration.
### Registry Operations
The register operation adds a constitutional authority by recording their PeerId and Ed25519 public key. The revoke operation removes an authority, immediately invalidating all certificates issued by that authority for future registrations. The is_trusted operation checks whether a given PeerId belongs to a registered authority. The get_key operation retrieves an authority's public key for certificate verification.
### Trust Model
Trust anchors are the root of the identity chain. They are established through constitutional governance processes outside the scope of the protocol itself. The registry simply records which authorities are currently recognized. The registry is deterministic: the same set of trust anchors produces the same trust decisions on all nodes.
## N21.3 — Validator Registry
### Purpose
The ValidatorRegistry maintains the set of active validators whose certificates have been verified against the trust anchor registry. A peer may possess a key pair and a certificate, but it is not considered a validator until its certificate passes constitutional verification and is registered.
### Registration Requirements
Three conditions must be satisfied for registration. First, the certificate's issuer must be a registered trust anchor. Second, the certificate's authority signature must be valid for the issuing authority's public key. Third, the certificate must be within its validity window at the time of registration. If any condition fails, registration is rejected.
### Rejected Scenarios
Registration with an untrusted authority fails because the issuer is not in the trust anchor registry. Registration with a forged certificate fails because the impostor's signature does not match the claimed authority's public key. Registration with an expired certificate fails because the validity window has closed.
### Registry Operations
The register method performs all three verification checks and inserts the validator if they pass. The remove method removes a validator by PeerId. The is_validator method checks whether a peer is currently registered. The len method returns the count of active validators.
## Identity Chain
The complete identity chain flows from cryptographic possession to constitutional authorization. PeerKeyPair establishes that the validator possesses an Ed25519 signing key. ValidatorCertificate establishes that a constitutional authority recognizes the validator and has bound their identity to their public key. TrustAnchor establishes that the issuing authority is constitutionally recognized. ValidatorRegistry establishes that the validator has passed all verification checks and is an active participant.
A validator must prove three things. I own a key: possession of the Ed25519 signing key matching the public key in the certificate, demonstrated by signing messages. A constitutional authority recognizes me: a ValidatorCertificate signed by a trust anchor. My certificate is valid: current time within the certificate's validity window.
## Constitutional Significance
The constitutional identity layer transforms network participation from an open-permissionless model to a constitutionally governed model. Not everyone with a key pair may participate. Only those recognized by a constitutional authority may join the validator set. This enables governance over who may propose blocks, who may vote, and who may participate in consensus. The identity layer is the foundation for constitutional membership, authority delegation, validator rotation, and long-term network governance.
## Integration Architecture
The identity layer integrates with the network layer at the point of message verification. When a signed envelope arrives, the recipient verifies the Ed25519 signature. With N21, the recipient may additionally verify that the sender is a registered validator with a valid certificate from a trusted authority. This adds a constitutional authorization check on top of the cryptographic authentication check.
## Test Coverage
| Component | Tests |
|-----------|-------|
| Validator Certificate | 5 |
| Trust Anchor Registry | 3 |
| Validator Registry | 3 |
Total Verified Coverage: 11 Tests. All passing.
## Milestone Result
Status: COMPLETE. Outcome: A constitutional identity layer providing authority-signed validator certificates, a trust anchor registry forming the root of cryptographic trust, and a validator registry enforcing certificate verification before participation. Network identity is transformed from cryptographic proof-of-possession to constitutional proof-of-authorization. The foundation is laid for governed participation, authority delegation, and long-term network governance.
