use super::node::NodeHash;
use super::tree::MAX_DEPTH;
use crate::canonical::{Decoder, Encoder};

pub const PROOF_VERSION_V1: u8 = 0x01;

pub fn canonical_empty_root() -> [u8; 32] {
    super::tree::SparseMerkleTree::canonical_empty_root()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofStep {
    SiblingOnLeft(NodeHash),
    SiblingOnRight(NodeHash),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofType {
    Inclusion {
        value_hash: [u8; 32],
        version: u64,
    },
    Absence {
        divergence_depth: usize,
        leaf_key_hash: [u8; 32],
        leaf_value_hash: [u8; 32],
        leaf_version: u64,
    },
    EmptyTree,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleProof {
    pub proof_version: u8,
    pub key_hash: [u8; 32],
    pub proof_type: ProofType,
    pub steps: Vec<ProofStep>,
}

impl MerkleProof {
    pub fn new_inclusion(
        key_hash: [u8; 32],
        value_hash: [u8; 32],
        version: u64,
        steps: Vec<ProofStep>,
    ) -> Self {
        Self {
            proof_version: PROOF_VERSION_V1,
            key_hash,
            proof_type: ProofType::Inclusion {
                value_hash,
                version,
            },
            steps,
        }
    }
    pub fn new_absence(
        key_hash: [u8; 32],
        divergence_depth: usize,
        leaf_key_hash: [u8; 32],
        leaf_value_hash: [u8; 32],
        leaf_version: u64,
        steps: Vec<ProofStep>,
    ) -> Self {
        Self {
            proof_version: PROOF_VERSION_V1,
            key_hash,
            proof_type: ProofType::Absence {
                divergence_depth,
                leaf_key_hash,
                leaf_value_hash,
                leaf_version,
            },
            steps,
        }
    }
    pub fn new_empty_tree_absence(key_hash: [u8; 32], steps: Vec<ProofStep>) -> Self {
        Self {
            proof_version: PROOF_VERSION_V1,
            key_hash,
            proof_type: ProofType::EmptyTree,
            steps,
        }
    }

    fn direction(key: &[u8; 32], depth: usize) -> u8 {
        (key[depth / 8] >> (7 - (depth % 8))) & 1
    }

    pub fn verify(&self, root: [u8; 32]) -> bool {
        if self.proof_version != PROOF_VERSION_V1 {
            return false;
        }
        if self.steps.len() != MAX_DEPTH {
            return false;
        }
        match &self.proof_type {
            ProofType::Inclusion {
                value_hash,
                version,
            } => {
                let leaf = super::node::Node::Leaf {
                    key_hash: self.key_hash,
                    value_hash: *value_hash,
                    version: *version,
                };
                let mut current = leaf.hash();
                for step in &self.steps {
                    current = match step {
                        ProofStep::SiblingOnLeft(s) => super::node::Node::Branch {
                            left: *s,
                            right: current,
                        }
                        .hash(),
                        ProofStep::SiblingOnRight(s) => super::node::Node::Branch {
                            left: current,
                            right: *s,
                        }
                        .hash(),
                    };
                }
                current.0 == root
            }
            ProofType::Absence {
                divergence_depth,
                leaf_key_hash,
                leaf_value_hash,
                leaf_version,
            } => {
                let leaf = super::node::Node::Leaf {
                    key_hash: *leaf_key_hash,
                    value_hash: *leaf_value_hash,
                    version: *leaf_version,
                };
                let mut current = leaf.hash();
                for step in &self.steps {
                    current = match step {
                        ProofStep::SiblingOnLeft(s) => super::node::Node::Branch {
                            left: *s,
                            right: current,
                        }
                        .hash(),
                        ProofStep::SiblingOnRight(s) => super::node::Node::Branch {
                            left: current,
                            right: *s,
                        }
                        .hash(),
                    };
                }
                if current.0 != root {
                    return false;
                }
                let actual_div = (0..MAX_DEPTH)
                    .find(|&d| {
                        Self::direction(&self.key_hash, d) != Self::direction(leaf_key_hash, d)
                    })
                    .unwrap_or(MAX_DEPTH);
                actual_div == *divergence_depth && self.key_hash != *leaf_key_hash
            }
            ProofType::EmptyTree => {
                let empty_root = canonical_empty_root();
                if root != empty_root {
                    return false;
                }
                let mut current = NodeHash::ZERO;
                for step in &self.steps {
                    current = match step {
                        ProofStep::SiblingOnLeft(s) => super::node::Node::Branch {
                            left: *s,
                            right: current,
                        }
                        .hash(),
                        ProofStep::SiblingOnRight(s) => super::node::Node::Branch {
                            left: current,
                            right: *s,
                        }
                        .hash(),
                    };
                }
                current.0 == root
            }
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut enc = Encoder::new();
        enc.write_u8(self.proof_version);
        enc.write_bytes(&self.key_hash);
        match &self.proof_type {
            ProofType::Inclusion {
                value_hash,
                version,
            } => {
                enc.write_u8(0x01);
                enc.write_bytes(value_hash);
                enc.write_u64(*version);
            }
            ProofType::Absence {
                divergence_depth,
                leaf_key_hash,
                leaf_value_hash,
                leaf_version,
            } => {
                enc.write_u8(0x02);
                enc.write_u64(*divergence_depth as u64);
                enc.write_bytes(leaf_key_hash);
                enc.write_bytes(leaf_value_hash);
                enc.write_u64(*leaf_version);
            }
            ProofType::EmptyTree => {
                enc.write_u8(0x03);
            }
        }
        enc.write_u64(self.steps.len() as u64);
        for step in &self.steps {
            match step {
                ProofStep::SiblingOnLeft(h) => {
                    enc.write_u8(0x00);
                    enc.write_bytes(&h.0);
                }
                ProofStep::SiblingOnRight(h) => {
                    enc.write_u8(0x01);
                    enc.write_bytes(&h.0);
                }
            }
        }
        enc.into_bytes()
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut dec = Decoder::new(data);
        let proof_version = dec.read_u8()?;
        if proof_version != PROOF_VERSION_V1 {
            return None;
        }
        let key_hash = dec.read_bytes()?;
        if key_hash.len() != 32 {
            return None;
        }
        let mut kh = [0u8; 32];
        kh.copy_from_slice(&key_hash);
        let tag = dec.read_u8()?;
        let proof_type = match tag {
            0x01 => {
                let vh = dec.read_bytes()?;
                let ver = dec.read_u64()?;
                if vh.len() != 32 {
                    return None;
                }
                let mut v = [0u8; 32];
                v.copy_from_slice(&vh);
                ProofType::Inclusion {
                    value_hash: v,
                    version: ver,
                }
            }
            0x02 => {
                let dd = dec.read_u64()? as usize;
                let lk = dec.read_bytes()?;
                let lv = dec.read_bytes()?;
                let lver = dec.read_u64()?;
                if lk.len() != 32 || lv.len() != 32 {
                    return None;
                }
                let mut lkh = [0u8; 32];
                let mut lvh = [0u8; 32];
                lkh.copy_from_slice(&lk);
                lvh.copy_from_slice(&lv);
                ProofType::Absence {
                    divergence_depth: dd,
                    leaf_key_hash: lkh,
                    leaf_value_hash: lvh,
                    leaf_version: lver,
                }
            }
            0x03 => ProofType::EmptyTree,
            _ => return None,
        };
        let step_count = dec.read_u64()? as usize;
        if step_count != MAX_DEPTH {
            return None;
        }
        let mut steps = Vec::with_capacity(step_count);
        for _ in 0..step_count {
            let st = dec.read_u8()?;
            let hash_bytes = dec.read_bytes()?;
            if hash_bytes.len() != 32 {
                return None;
            }
            let mut h = [0u8; 32];
            h.copy_from_slice(&hash_bytes);
            steps.push(if st == 0x00 {
                ProofStep::SiblingOnLeft(NodeHash(h))
            } else {
                ProofStep::SiblingOnRight(NodeHash(h))
            });
        }
        if !dec.is_finished() {
            return None;
        }
        Some(MerkleProof {
            proof_version,
            key_hash: kh,
            proof_type,
            steps,
        })
    }
}

pub struct ProofVerifier;
impl ProofVerifier {
    pub fn verify(proof: &MerkleProof, root: [u8; 32]) -> bool {
        proof.verify(root)
    }
}
