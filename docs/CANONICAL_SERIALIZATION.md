# AmunChain Canonical Serialization

## Constitutional Law

**All consensus-critical data MUST be serialized using these rules.**

### Endianness

**Little-endian forever.** This is a constitutional law that never changes.

### Fixed-Width Primitives

| Type | Bytes | Encoding |
|------|-------|----------|
| u64 | 8 | Direct little-endian bytes |
| u32 | 4 | Direct little-endian bytes |
| u8 | 1 | Direct |
| [u8; 32] | 33 | Length byte (0x20) + 32 bytes |

### Variable-Length Data

| Type | Encoding |
|------|----------|
| Vec<u8> | Length (u64 LE) + bytes |

### Constitutional Limits

- Maximum `Vec<u8>` length: 16,777,216 bytes (16 MB)
- Maximum transitions per witness: 1,000,000

### Rationale

- Fixed-width primitives have no length prefix (efficient, deterministic)
- Variable-length data has length prefix (previents ambiguity)
- Little-endian matches modern hardware (no conversion overhead)
- Bounds prevent memory amplification attacks

## Reference Implementation

See `amun_consensus::canonical::CanonicalEncoder` and `CanonicalDecoder`.

## Validation

Any implementation that produces different bytes from the same logical data is constitutionally invalid.
