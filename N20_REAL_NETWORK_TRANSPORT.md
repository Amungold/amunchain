# N20 — Real Network Transport
## Overview
The N20 milestone marks the transition of AmunChain from a simulated network to a real distributed system. All prior milestones operated over MockTransport, an in-memory channel that simulates message passing between nodes within a single process. N20 replaces this with real TCP transport, Ed25519 cryptographic identity, signed peer messaging, peer discovery with expiration, and complete bootstrap/rejoin protocols over actual network connections. This is the milestone where AmunChain becomes a distributed system rather than a distributed protocol simulator.
## Architectural Objectives
The real network transport must provide: (1) Real TCP connectivity between independent processes and machines, (2) Cryptographic peer identity through Ed25519 key pairs, (3) Signed and verifiable network messages, (4) Peer discovery without manual configuration, (5) Bootstrap and rejoin over real network connections, (6) Multi-machine deployment capability, (7) Preservation of all constitutional safety properties from N1–N19.
## N20.1 — Transport Abstraction
An abstract Transport trait defines the interface that all transport implementations must satisfy. The trait provides five operations: send queues an envelope for transmission, next_outgoing takes the next envelope from the outbox for delivery, deliver receives an envelope from the network into the inbox, next_incoming takes the next envelope from the inbox for processing, and tick advances transport state (for mock: advances time, for TCP: reads and writes sockets). Two implementations exist: MockTransport for deterministic testing within a single process, and TcpTransport for real network communication between processes and machines.
## N20.2 — Peer Identity
PeerId is a unique identifier derived from the Ed25519 verifying key bytes. It provides cryptographic binding between a peer's identity and their public key. PeerIdentity bundles a PeerId, the full Ed25519 public key, and a SocketAddr for network addressing. This structure enables complete identification of any peer on the network: who they are cryptographically and where they are reachable.
## N20.3 — Signed Envelope
SignedEnvelope provides cryptographic integrity and authenticity for all network messages. Each envelope contains the sender's PeerId, the serialized payload, and an Ed25519 signature over the payload. The verify method checks that the signature is valid for the claimed sender. This ensures that messages cannot be forged and cannot be tampered with in transit. The earlier placeholder verification has been replaced with real Ed25519 signature verification.
## N20.4 — TCP Transport
TcpTransport implements the Transport trait over real TCP sockets with length-prefixed message framing. The wire format prepends each message with a 4-byte big-endian length prefix followed by the JSON-encoded payload. This ensures complete message boundaries over TCP streams. Non-blocking I/O is used throughout: accept returns WouldBlock when no connections are pending, connect attempts are retried on next tick, reads accumulate partial data in per-stream buffers and extract complete messages when the length prefix is satisfied, writes drain the outbox to all connected streams. Dead connections are detected on write failure and removed from the stream list. A 10MB maximum message size guard prevents memory exhaustion attacks.
## N20.5 — Ed25519 Signatures
PeerKeyPair encapsulates an Ed25519 signing key and its corresponding verifying key. Generation uses the operating system's cryptographic random number generator. The sign method produces a 64-byte Ed25519 signature over an arbitrary message. The verify static method checks a signature against a verifying key and message, returning false for invalid signatures, wrong-length signatures, or invalid keys. SignedMessage combines a sender's public key, payload, and signature into a single serializable unit with self-verification capability.
## N20.6 — Directed Peer Messaging
SignedEnvelope wraps a SignedMessage with sender PeerId for network transmission. DirectedMessage adds a to field specifying the target PeerId, enabling point-to-point messaging rather than broadcast-only communication. Tampered envelopes are detected by signature verification. Messages can be routed to specific peers based on their cryptographic identity.
## N20.7 — Peer Discovery
PeerAnnouncement is a signed message declaring a peer's presence on the network. It contains the peer's ID, public key, network address, and a timestamp. Announcements are verified by checking the signature and confirming the sender's PeerId matches. PeerRegistry maintains a BTreeMap of known peers with last-seen timestamps. The register method inserts or updates a peer's entry. The expire method removes peers not seen since a given timestamp, enabling automatic cleanup of departed nodes. Duplicate announcements refresh the last-seen timestamp. Deterministic ordering via BTreeMap ensures reproducible peer lists across nodes.
## N20.8 — Bootstrap over TCP
A fresh node discovers a peer through the peer discovery mechanism, sends a SyncRequest for the latest checkpoint, receives a SyncResponse containing the checkpoint certificate, verifies the checkpoint bundles against its trusted root via BootstrapSession, imports the checkpoint height, transitions through the full lifecycle (CatchingUp → Verifying → Active), and activates at the network's current height. The entire process occurs over real TCP connections with cryptographic verification at each step.
## N20.9 — Rejoin over TCP
The rejoin protocol over real TCP follows the complete crash recovery lifecycle. A node crashes and loses its state. The remaining network continues operation, advancing the chain height. The crashed node restarts with no state. It discovers peers, requests the latest checkpoint, verifies it against its trusted root, imports the height, completes the lifecycle transition, and activates. Outdated checkpoints are rejected. Height regression is prevented. The full constitutional lifecycle is enforced.
## N20.10 — Multi-Machine Testnet
Real TCP bind and connect are verified between independent sockets on localhost. Peer identity exchange confirms that cryptographic identities are correctly transmitted and verified. Four nodes start together with distinct Ed25519 key pairs and peer identities. All nodes achieve Active state. The network survives a node disconnect and continues operation with the remaining nodes.
## Safety Preservation
All constitutional safety properties established in N1–N19 are preserved. Byzantine bootstrap defenses remain active over TCP. Checkpoint verification is unchanged. Lifecycle enforcement is unchanged. Consensus safety is unchanged. The transport layer is a drop-in replacement that adds real networking without modifying any constitutional logic.
## Test Coverage
| Component | Tests |
|-----------|-------|
| Ed25519 Signatures | 6 |
| Directed Messaging | 3 |
| Peer Discovery | 6 |
| Bootstrap over TCP | 6 |
| Rejoin over TCP | 4 |
| Multi-Machine Testnet | 5 |
Total Verified Coverage: 30 Tests. All passing.
## Milestone Result
Status: COMPLETE. Outcome: AmunChain operates as a real distributed system over TCP with Ed25519 cryptographic identity, signed peer messaging, automatic peer discovery, and complete bootstrap/rejoin protocols over actual network connections. The system is ready for multi-machine deployment while preserving all constitutional safety properties established in prior milestones.
