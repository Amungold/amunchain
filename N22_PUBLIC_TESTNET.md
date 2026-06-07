# N22 — Public Testnet
## Overview
The N22 milestone defines the specification for the first public testnet deployment of AmunChain. While the constitutional kernel, network transport, identity layer, and distributed consensus have been built and verified within a controlled development environment (N1–N21), N22 provides the architectural blueprint for transitioning from internal verification to public operation. This document serves as the deployment specification to be executed when the operational prerequisites are met.
## Architectural Objectives
The public testnet must demonstrate: (1) Multi-machine consensus over real TCP across the public internet, (2) Independent validator operation by separate parties, (3) Bootstrap of new validators from the live network, (4) Crash recovery and rejoin under real network conditions, (5) Checkpoint generation and verification on a live chain, (6) Constitutional identity enforcement with real validator certificates, (7) Network resilience under extended operation.
## Deployment Architecture
### Node Configuration
Each validator operates an independent AmunChain node process. The node binds to a public TCP address and listens for connections from peers. A peer list provides initial discovery seeds. The node loads its Ed25519 key pair from secure storage. The node loads its validator certificate issued by a constitutional authority. The node loads the trust anchor registry containing the public keys of all recognized authorities.
### Network Topology
Validators are geographically distributed across multiple machines and networks. Each validator maintains persistent TCP connections to a subset of peers. Messages are routed point-to-point using directed peer messaging. Peer discovery enables new validators to find and connect to the existing network without manual configuration of every peer.
### Genesis Configuration
The genesis block is collectively agreed upon before network launch. The initial validator set is defined in the genesis configuration. The initial trust anchor registry is embedded in the genesis block. The initial checkpoint root is established at genesis. All validators share an identical genesis configuration ensuring a common constitutional starting point.
## Operational Workflows
### Network Launch
All genesis validators start their nodes simultaneously. Peer discovery enables them to find each other. Consensus begins at height zero with the genesis validator set. Blocks are proposed, voted upon, and committed. The chain advances.
### Validator Addition
A new validator generates an Ed25519 key pair. A constitutional authority issues a validator certificate binding the validator's identity to their public key. The certificate is distributed to the network. The new validator starts a node configured with its key pair and certificate. The node discovers peers and bootstraps from the latest checkpoint. The node transitions through the constitutional lifecycle: Bootstrapping → CatchingUp → Verifying → Active. The validator joins consensus and begins participating.
### Crash Recovery
A validator crashes due to process failure, machine restart, or network disconnection. The remaining validators continue consensus and advance the chain. The crashed validator restarts its node process. The node bootstraps from the latest checkpoint via the rejoin protocol. Height is synchronized with the network. The node transitions through the constitutional lifecycle. The validator resumes participation at the current network height.
## Security Considerations
All messages are Ed25519-signed. All certificates are authority-verified. All checkpoints are cryptographically validated against the trusted root. No validator may participate without a valid certificate from a trusted authority. No node may propose or vote without completing the constitutional lifecycle. The network resists Byzantine attacks validated in N16 and N19. The constitutional identity layer prevents unauthorized participation.
## Success Criteria
The testnet is considered successful when: (1) At least four independent validators operate continuously for 72 hours, (2) At least 10,000 blocks are committed, (3) A new validator successfully bootstraps from the live network, (4) A validator crashes, recovers, and rejoins without manual intervention, (5) Checkpoints are generated and verified by all nodes, (6) No constitutional safety violation is detected.
## Milestone Status
Status: SPECIFICATION COMPLETE — DEPLOYMENT PENDING. Dependencies: N1–N21 complete and verified. Infrastructure required: At least four independent machines with public IP addresses. Configuration required: Genesis block, initial validator set, trust anchor registry, and validator certificates. This milestone is architecturally designed and documented. Execution awaits operational infrastructure provisioning.
