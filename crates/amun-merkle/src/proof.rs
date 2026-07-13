use crate::tree::MerkleTree;
use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, WriteResult};
use amun_failure::AmunResult;
use amun_kernel_types::PublicHash32;
use heapless::Vec;

pub const MAX_PROOF_DEPTH: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerkleProof {
    pub siblings: Vec<PublicHash32, MAX_PROOF_DEPTH>,
    pub directions: Vec<Direction, MAX_PROOF_DEPTH>,
}

impl MerkleProof {
    pub fn new() -> Self {
        Self {
            siblings: Vec::new(),
            directions: Vec::new(),
        }
    }

    pub fn add_sibling(
        &mut self,
        sibling: PublicHash32,
        direction: Direction,
    ) -> Result<(), &'static str> {
        self.siblings.push(sibling).map_err(|_| "Proof too deep")?;
        self.directions
            .push(direction)
            .map_err(|_| "Proof too deep")?;
        Ok(())
    }

    pub fn verify(&self, leaf: &PublicHash32, root: &PublicHash32) -> bool {
        let mut current = *leaf;
        for i in 0..self.siblings.len() {
            let (left, right) = match self.directions[i] {
                Direction::Left => (&self.siblings[i], &current),
                Direction::Right => (&current, &self.siblings[i]),
            };
            current = MerkleTree::internal_hash(left, right);
        }
        current == *root
    }
}

impl Default for MerkleProof {
    fn default() -> Self {
        Self::new()
    }
}

impl CanonicalEncode for MerkleProof {
    const MAX_ENCODED_SIZE: usize = 128;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        (self.siblings.len() as u8).encode_to_writer(writer)?;
        for sibling in &self.siblings {
            sibling.encode_to_writer(writer)?;
        }
        for dir in &self.directions {
            match dir {
                Direction::Left => writer.write_bytes(&[0x00])?,
                Direction::Right => writer.write_bytes(&[0x01])?,
            }
        }
        Ok(())
    }
}

impl CanonicalDecode for MerkleProof {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.is_empty() {
            return Err(amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::BufferTooSmall,
                0x0005,
                0x0001,
            ));
        }
        let (depth, len1) = u8::decode(&input[..1])?;
        let depth = depth as usize;
        if depth > MAX_PROOF_DEPTH {
            return Err(amun_failure::FailureContext::new(
                amun_failure::ConstitutionalFault::CapacityExceeded,
                0x0005,
                0x0002,
            ));
        }
        let mut pos = len1;
        let mut siblings = Vec::new();
        for _ in 0..depth {
            let (sib, len) = PublicHash32::decode(&input[pos..])?;
            pos += len;
            siblings.push(sib).map_err(|_| {
                amun_failure::FailureContext::new(
                    amun_failure::ConstitutionalFault::CapacityExceeded,
                    0x0005,
                    0x0003,
                )
            })?;
        }
        let mut directions = Vec::new();
        for _ in 0..depth {
            if pos >= input.len() {
                return Err(amun_failure::FailureContext::new(
                    amun_failure::ConstitutionalFault::BufferTooSmall,
                    0x0005,
                    0x0004,
                ));
            }
            let dir = match input[pos] {
                0x00 => Direction::Left,
                0x01 => Direction::Right,
                _ => {
                    return Err(amun_failure::FailureContext::new(
                        amun_failure::ConstitutionalFault::MalformedEncoding,
                        0x0005,
                        0x0005,
                    ))
                }
            };
            directions.push(dir).map_err(|_| {
                amun_failure::FailureContext::new(
                    amun_failure::ConstitutionalFault::CapacityExceeded,
                    0x0005,
                    0x0006,
                )
            })?;
            pos += 1;
        }
        Ok((
            Self {
                siblings,
                directions,
            },
            pos,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_codec::CanonicalEncode;

    #[test]
    fn test_proof_verify() {
        let l1 = MerkleTree::leaf_hash(b"a");
        let l2 = MerkleTree::leaf_hash(b"b");
        let root = MerkleTree::compute_root(&[l1, l2]);
        let mut proof = MerkleProof::new();
        proof.add_sibling(l2, Direction::Right).unwrap();
        assert!(proof.verify(&l1, &root));
    }

    #[test]
    fn test_proof_wire_freeze_depth_1() {
        let l1 = MerkleTree::leaf_hash(b"a");
        let l2 = MerkleTree::leaf_hash(b"b");
        let mut proof = MerkleProof::new();
        proof.add_sibling(l2, Direction::Right).unwrap();
        let mut buf = [0u8; 128];
        let len = proof.encode(&mut buf).unwrap();
        let decoded = MerkleProof::decode_exact(&buf[..len]).unwrap();
        assert!(decoded.verify(&l1, &MerkleTree::compute_root(&[l1, l2])));
    }

    #[test]
    fn test_proof_decode_exact_rejects_trailing() {
        let l2 = MerkleTree::leaf_hash(b"b");
        let mut proof = MerkleProof::new();
        proof.add_sibling(l2, Direction::Right).unwrap();
        let mut buf = [0u8; 128];
        let len = proof.encode(&mut buf).unwrap();
        assert!(MerkleProof::decode_exact(&buf[..len + 1]).is_err());
    }

    #[test]
    fn test_proof_rejects_excessive_depth() {
        let mut buf = [0u8; 128];
        buf[0] = 33; // > MAX_PROOF_DEPTH
        assert!(MerkleProof::decode(&buf).is_err());
    }

    #[test]
    fn test_proof_rejects_invalid_direction() {
        let mut buf = [0u8; 128];
        buf[0] = 1;
        for i in 0..32 {
            buf[1 + i] = 0xAA;
        }
        buf[33] = 0xFF;
        assert!(MerkleProof::decode(&buf).is_err());
    }
}
