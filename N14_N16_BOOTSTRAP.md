# N14–N16 — Bootstrap Architecture
## Overview
The N14–N16 milestone series introduces the constitutional bootstrap architecture of AmunChain. While previous milestones established how to create and verify cryptographic proofs, the bootstrap architecture establishes how a completely fresh node enters the network and obtains a cryptographically verified state without trusting any single peer and without replaying the entire blockchain history. This milestone transforms AmunChain from a verification-capable system into a network that can autonomously onboard new participants through constitutional means.
## Architectural Objectives
The bootstrap architecture must provide: (1) Trustless node initialization from zero state, (2) Cryptographic verification of all received data, (3) Resistance to malicious bootstrap sources, (4) No requirement for full historical replay, (5) Deterministic and reproducible bootstrap outcomes, (6) Integration with the checkpoint system as the trust anchor, (7) Foundation for crash recovery and rejoin protocols.
## N14 — Bootstrap Session
### Purpose
A BootstrapSession represents a constitutional ritual through which a fresh node establishes its initial trusted state. The node begins with a single cryptographic root obtained through an out-of-band trusted channel. From this root, the node can verify any amount of checkpoint data without trusting the peer that provides it.
### Core Concept
The trusted root serves as a cryptographic anchor. Any checkpoint bundle whose inclusion proof verifies against this root is accepted as constitutionally valid. Any bundle that fails verification is rejected regardless of its source. The peer providing the data is never trusted. Only the mathematics of the Merkle proof is trusted.
### Session Behavior
Empty bundle lists are rejected because no trust can be established from nothing. Wrong root mismatches are rejected because the provided data does not match the trusted anchor. Valid bundles are accepted and their checkpoints become part of the node's verified state. The trusted root is stored and remains retrievable for the lifetime of the session.
### Constitutional Significance
The BootstrapSession embodies the principle that trust in AmunChain derives from cryptographic verification, not from network reputation. A node does not ask "Who sent this data?" It asks "Does this data verify against my trusted root?"
## N15 — Constitutional Join
### Purpose
End-to-end integration proof that a new node can join an existing network using only checkpoint data. This is the first system-wide test connecting the checkpoint architecture with the bootstrap architecture in a complete scenario.
### Scenario Architecture
Node A represents an existing network participant that has processed blocks and produced checkpoint certificates. Node B represents a completely fresh node with no prior state. Node A exports its checkpoint chain. Node B imports the checkpoint Merkle root as its trusted anchor. Node B requests and receives checkpoint bundles from Node A. Node B verifies each bundle against its trusted root. Node B independently recomputes the state root for all blocks covered by the checkpoints. Both nodes arrive at identical final state roots.
### What This Proves
A new node can join the network without executing any transitions. The joining node arrives at the same constitutional state as an existing node. No trust was placed in the peer providing the data. The checkpoint system correctly anchors the bootstrap process. The system is ready for multi-node network deployment.
## N16 — Adversarial Bootstrap
### Purpose
Byzantine attack scenarios validating that the bootstrap process correctly rejects malicious inputs. While N14 and N15 proved the happy path, N16 proves that the path is defended.
### Attack Vectors
Forged checkpoint attacks present a checkpoint from a different chain or different history. The inclusion proof fails against the trusted root. Tampered bundle attacks alter the inclusion proof after legitimate creation. The proof no longer verifies. Chain gap attacks provide non-consecutive checkpoints that leave gaps in history. Continuity verification detects the gap. Wrong trusted root attacks provide a malicious root instead of the constitutional trusted root. No bundles verify against it. Byzantine source attacks mix valid and invalid bundles in the same stream. The invalid bundle causes the entire sync to be rejected. Empty bundle attacks provide no data at all. The session correctly rejects empty input.
### Defensive Properties
Every attack vector is detected and rejected before any state is imported. No invalid checkpoint ever enters the node's verified state. The bootstrap process is constitutionally safe against malicious data sources.
## Integration Architecture
The bootstrap architecture sits at the intersection of the checkpoint system and the network layer. Checkpoints provide the data format and verification mechanism. The network provides the transport for requesting and receiving bundles. The bootstrap session enforces the constitutional rules governing which data is accepted. This three-layer architecture ensures that trust establishment is cryptographically grounded, network-independent, and constitutionally governed.
## Test Coverage
| Component | Tests |
|-----------|-------|
| Bootstrap Session (N14) | 4 |
| Constitutional Join (N15) | 2 |
| Adversarial Bootstrap (N16) | 7 |
Total Verified Coverage: 13 Tests. All passing.
## Milestone Result
Status: COMPLETE. Outcome: A constitutional bootstrap architecture enabling fresh nodes to join the network with cryptographically verified state, without trusting any peer, and with comprehensive defense against Byzantine data sources. This milestone establishes the foundation for all node lifecycle operations including crash recovery, network rejoin, and long-term state reconstruction.
