# AmunChain Canonical Hash Protocol

## Fixed Hash Function

**BLAKE3** is the canonical hash function for:
- Constitutional identities
- Transcript verification
- Replay certificates
- State roots
- QC hashes

## Domain Separation Tags

| Domain | Tag | Usage |
|--------|-----|-------|
| Genesis | `AMUN_GENESIS_V1` | Identity derivation |
| Chain ID | `AMUN_CHAIN_V1` | Chain identifier |
| QC | `AMUN_QC_V3` | Quorum certificate |
| Transcript | `AMUN_TRANSCRIPT_V1` | Replay transcript |
| Byzantine Round | `AMUN_BYZANTINE_ROUND_V1` | Simulation round |
| State Root | `AMUN_STATE_ROOT_V1` | State commitment |
| Validator Set | `AMUN_VALIDATOR_SET_V1` | Epoch snapshot |
| Node | `AMUN_NODE_V1` | Lineage node |

## Reasoning

- **BLAKE3**: Fast, parallelizable, cryptographically strong
- **Fixed forever**: Protocol compatibility depends on hash stability
- **Domain separated**: Prevents cross-context collisions

## Future Compatibility

Hash function can only be changed via constitutional amendment:
- Requires 2/3 validator approval
- Requires replay certification for all existing transcripts
- Requires backward compatibility mode

## Verification

All consensus-critical structures must use canonical hash function.
Non-canonical hashing (e.g., std::hash) is forbidden in consensus path.
