use crate::state_tree::Key256;
pub struct TraversalLaw;
impl TraversalLaw {
    pub const MAX_DEPTH: usize = 256;
    pub fn direction(key: &Key256, depth: usize) -> u8 { key.bit(depth) }
}
