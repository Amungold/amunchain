# Crate Classification

## Core Layer (Layer 0)
- amun-core - Types, traits, fundamental abstractions
- amun-crypto - Cryptographic primitives
- amun-codec - Canonical serialization
- amun-merkle - Merkle tree utilities

## Consensus Layer (Layer 1)
- amun-consensus-core - Consensus state machine
- amun-consensus-qc - Quorum certificate management
- amun-consensus-wal - Write-ahead log
- amun-consensus-pacemaker - Round/leader management

## Constitution Layer (Layer 2)
- amun-constitution-core - Constitutional axioms
- amun-constitution-lineage - Lineage tracking
- amun-constitution-authority - Authority topology

## Execution Layer (Layer 3)
- amun-execution-core - Execution primitives
- amun-execution-stf - State transition function
- amun-execution-runtime - Runtime environment

## Storage Layer (Layer 4)
- amun-storage-core - Storage primitives
- amun-storage-wal - Persistent WAL
- amun-storage-snapshot - Snapshot management

## Network Layer (Layer 5)
- amun-network-core - Network primitives
- amun-network-gossip - Gossip protocol
- amun-network-rpc - RPC interface

## Testing Layer (Layer 6)
- amun-test-mocks - Mock implementations
- amun-test-byzantine - Byzantine simulation
- amun-test-replay - Replay testing
