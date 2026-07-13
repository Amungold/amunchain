# N18–N19 — Constitutional Rejoin Protocol
## Overview
The N18–N19 milestone series introduces the constitutional rejoin protocol of AmunChain. While previous milestones established how a fresh node joins the network (bootstrap) and how the network operates (consensus), the rejoin protocol establishes how a node that has crashed or been disconnected returns to active participation through a constitutionally governed lifecycle. A node must progress through four states — Bootstrapping, CatchingUp, Verifying, and Active — with cryptographic verification at each transition. No node may participate in consensus until it has completed this lifecycle.
## Architectural Objectives
The rejoin protocol must provide: (1) A constitutional lifecycle governing node participation, (2) Cryptographic verification of all recovered state, (3) Prevention of premature consensus participation, (4) Height synchronization with the advancing network, (5) Defense against Byzantine rejoin attacks, (6) Integration with the checkpoint system for state recovery, (7) Foundation for dynamic membership and validator rotation.
## N18 — Node Lifecycle
### Purpose
Establish a constitutional state machine governing every node's participation in consensus. A node is not merely "connected" or "disconnected." It exists in one of four constitutionally defined states, and may only participate in consensus from the Active state.
### Lifecycle States
Bootstrapping is the initial state of a fresh or recovering node. The node has no verified state and may not propose or vote. CatchingUp is the state where the node is downloading checkpoints and synchronizing its height with the network. The node has a trusted root but has not yet imported all data. Verifying is the state where the node has imported checkpoint data and is cryptographically verifying it against the trusted root. The node may still not participate in consensus. Active is the final state. The node has completed verification and is permitted to propose blocks and vote in consensus.
### Lifecycle API
begin_bootstrap() enters the bootstrap phase. begin_catchup() starts downloading checkpoints. begin_verification() initiates verification of downloaded state. activate() transitions to active consensus participation after successful verification. is_active() returns whether the node may currently propose or vote.
### Constitutional Invariants
A Bootstrapping node cannot propose blocks. A node cannot activate directly from Bootstrapping or CatchingUp; it must pass through Verifying first. Height acquired during catch-up is preserved after activation. Only Active nodes may participate in consensus. These invariants are enforced by the propose() method which gates on lifecycle state, and by the activate() method which requires the correct predecessor state.
## N18 — Sync Protocol
### Purpose
Enable a recovering node to request and receive checkpoint data from active peers. The protocol is minimal by design: a single request message and a single response message.
### Messages
SyncRequest carries a from_height field indicating the height from which the requesting node needs data. SyncResponse carries a latest_height field indicating the network's current tip and a checkpoints vector containing the CheckpointCertificates needed for catch-up.
### Catch-up Mechanism
import_checkpoint_height() synchronizes the node's internal height and consensus state machine with the network. It sets current_height, consensus.state.height, and consensus.last_committed_height in a single atomic operation. After import, the node's height matches the network and it may proceed to verification.
## N18 — Full Rejoin Scenario
The complete rejoin scenario exercises all components. Step 1: A four-node network commits blocks. Step 2: Node 3 crashes and is removed. Step 3: The remaining three nodes continue committing blocks, advancing the network height. Step 4: Node 3 restarts as Bootstrapping with height zero. Step 5: Node 3 discovers a peer and requests the latest checkpoint. Step 6: The peer responds with a SyncResponse containing the latest height and checkpoint certificates. Step 7: Node 3 verifies the checkpoint bundles against its trusted root via BootstrapSession. Step 8: Node 3 imports the checkpoint height and transitions through CatchingUp and Verifying. Step 9: Node 3 activates and resumes consensus participation at the network's current height.
## N19 — Adversarial Rejoin
### Purpose
Byzantine attack scenarios validating that the rejoin protocol correctly rejects malicious inputs. Where N18 proved the happy path, N19 proves the path is constitutionally defended.
### Attack Vectors
Wrong trusted root attack presents a checkpoint rooted in a different trust anchor. The inclusion proof fails and the bundle is rejected. Checkpoint rollback attack sends a checkpoint at a lower height than the node has already verified. The node detects regression and preserves its current height. Checkpoint gap attack provides non-consecutive checkpoints with missing ranges between them. The gap is detected during continuity verification. Mixed valid/invalid stream attack presents a valid checkpoint followed by a tampered one. The tampered bundle fails verification and the entire sync is rejected. Byzantine rejoin source attack uses a malicious peer that provides a forged checkpoint with a wrong root. The constitutional verification rejects it.
### Defensive Properties
Every Byzantine attack is detected before state is modified. No invalid checkpoint enters the node's verified state. No regression to a lower height is permitted. No gap in checkpoint continuity is accepted. The rejoin protocol is constitutionally safe against malicious data sources.
## Integration Architecture
The rejoin protocol integrates four subsystems. The Node Lifecycle governs when a node may participate. The Sync Protocol provides the message format for data exchange. The Checkpoint System provides the data format and verification mechanism. The Bootstrap Session enforces the constitutional rules governing which data is accepted. This four-layer architecture ensures that rejoin is cryptographically grounded, network-independent, and constitutionally governed.
## Test Coverage
| Component | Tests |
|-----------|-------|
| Lifecycle Enforcement | 6 |
| Catch-up | 6 |
| Checkpoint Sync | 3 |
| Full Rejoin | 3 |
| Adversarial Rejoin | 5 |
Total Verified Coverage: 23 Tests. All passing.
## Milestone Result
Status: COMPLETE. Outcome: A constitutional rejoin protocol enabling crashed or disconnected nodes to return to active consensus participation through a governed lifecycle with cryptographic verification at each transition. The protocol is defended against Byzantine attacks including wrong roots, checkpoint rollback, checkpoint gaps, mixed valid/invalid streams, and malicious rejoin sources. The foundation is laid for dynamic membership and validator rotation.
