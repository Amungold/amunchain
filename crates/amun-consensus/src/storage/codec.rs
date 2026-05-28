use crate::ccbf::{CCBFEncoder, CCBFDecoder};
use crate::state_tree::{Node, NodeHash, ValueKey};

pub struct NodeCodec;

impl NodeCodec {
    pub const TAG_EMPTY: u8 = 0x00;
    pub const TAG_LEAF: u8 = 0x01;
    pub const TAG_BRANCH: u8 = 0x02;
    
    pub fn encode(node: &Node, encoder: &mut CCBFEncoder) {
        match node {
            Node::Empty { depth } => {
                encoder.write_u8(Self::TAG_EMPTY);
                encoder.write_u16(*depth as u16);
            }
            Node::Leaf { key, value_key } => {
                encoder.write_u8(Self::TAG_LEAF);
                encoder.write_fixed_hash(key);
                encoder.write_fixed_hash(&value_key.hash);
                encoder.write_u64(value_key.length);
                encoder.write_u8(value_key.type_tag);
            }
            Node::Branch { left, right } => {
                encoder.write_u8(Self::TAG_BRANCH);
                encoder.write_fixed_hash(&left.0);
                encoder.write_fixed_hash(&right.0);
            }
        }
    }
    
    pub fn decode(decoder: &mut CCBFDecoder) -> Option<Node> {
        let tag = decoder.read_u8()?;
        match tag {
            Self::TAG_EMPTY => {
                let depth = decoder.read_u16()? as usize;
                Some(Node::Empty { depth })
            }
            Self::TAG_LEAF => {
                let key = decoder.read_fixed_hash()?;
                let hash = decoder.read_fixed_hash()?;
                let length = decoder.read_u64()?;
                let type_tag = decoder.read_u8()?;
                Some(Node::Leaf {
                    key,
                    value_key: ValueKey { hash, length, type_tag },
                })
            }
            Self::TAG_BRANCH => {
                let left_bytes = decoder.read_fixed_hash()?;
                let right_bytes = decoder.read_fixed_hash()?;
                Some(Node::Branch {
                    left: NodeHash(left_bytes),
                    right: NodeHash(right_bytes),
                })
            }
            _ => None,
        }
    }
}
