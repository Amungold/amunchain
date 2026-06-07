# N17 — Multi-Node Constitutional Network
## Overview
The N17 milestone marks the transition of AmunChain from a single-node verification system into a multi-node BFT consensus network. This is the first phase where the consensus engine, networking layer, and constitutional verification stack operate together across multiple independent nodes. Four-node and seven-node networks achieve Byzantine Fault Tolerant commit under simulated network conditions. Multiple sequential blocks are committed and verified by all participating nodes.
## Architectural Significance
All prior milestones established individual components: state machine, certificates, checkpoints, bootstrap, distribution. N17 is the first milestone where these components function as an integrated system. The consensus engine produces actions. The network layer delivers them. The verification stack validates them. The node lifecycle governs participation. This integration proves that the constitutional architecture is not merely a collection of libraries but a functioning distributed protocol.
## NetworkNode Architecture
NetworkNode is the integration point between three subsystems: the consensus engine (RoundStateMachine) which implements BFT state machine replication, the network transport (MockTransport) which simulates peer-to-peer message delivery, and the constitutional verification stack which validates blocks, certificates, and checkpoints. NetworkNode drains pending_actions from the consensus engine and converts each action into a network envelope for transmission to peers. It also receives incoming envelopes, deserializes them into proposals and votes, feeds them into the consensus engine, and monitors for commit events.
## Consensus Flow
The BFT consensus protocol proceeds through four phases. Propose phase: the leader creates a block proposal and emits BroadcastProposal. All nodes receive the proposal and call accept_proposal which advances their state and emits BroadcastPrevote. Prevote phase: votes are collected through process_vote. When a quorum of prevotes is collected, a Quorum Certificate is formed and each node emits BroadcastPrecommit. Precommit phase: precommits are collected. When a quorum is reached, a second QC is formed and each node emits Commit. The block is finalized.
## Consensus Actions
The consensus engine produces five action types. BroadcastProposal carries a BlockProposal from the leader to all peers. BroadcastPrevote carries a validator's vote on a proposed block. BroadcastPrecommit carries a validator's commitment to an accepted block. Commit carries a QuorumCertificate proving the block is finalized. AdvanceRound signals a round transition when timeouts occur.
## Safety Properties
Double-commit prevention ensures that block finalization occurs exactly once. The Commit action emitted by the consensus engine records the event for audit, but the actual state transition is handled by finalize_commit which is called only when is_committed returns true. Mutation-during-iteration prevention uses std::mem::take to drain pending_actions safely without the risk of new actions being emitted during iteration. Lifecycle-gated proposal ensures only nodes in the Active lifecycle state may propose blocks. Validator set consistency ensures that when a node crashes and is removed, the validator set is rebuilt from the remaining live nodes so quorum calculations remain correct.
## Network Simulation
The Network struct manages a collection of NetworkNode instances connected through a message passing system. Each tick cycle executes four phases. Phase 1: the leader proposes and all nodes flush outgoing actions to their transport outbox. Phase 2: all outbox messages are collected. Phase 3: each message is delivered to all other nodes. Phase 4: each node processes its incoming messages. This cycle repeats until the target number of commits is reached or the maximum tick count is exhausted.
## Crash Recovery
When a node crashes, it is removed from the network. The validator set is rebuilt from the remaining live nodes. The network continues operation with the reduced validator set. When the crashed node restarts, it must bootstrap from a trusted checkpoint before rejoining consensus. This bootstrap requirement is enforced by the NodeLifecycle state machine.
## Test Coverage
| Scenario | Nodes | Result |
|----------|-------|--------|
| First commit | 4 | ✅ |
| First commit | 7 | ✅ |
| Multiple sequential blocks | 4 | ✅ |
| Crash and recovery | 4 | ✅ |
| Bootstrap trusted root persists | — | ✅ |
| Rejoin after crash | — | 🔒 N18 |
Total: 5 passed, 1 deferred to N18.
## Milestone Result
Status: COMPLETE. Outcome: AmunChain operates as a multi-node BFT consensus network. Blocks are proposed, voted upon, and committed through the full consensus cycle. The network survives node crashes and continues operation. The integration between consensus engine, network transport, and constitutional verification stack is proven. The foundation is laid for real TCP networking and constitutional rejoin protocols.
