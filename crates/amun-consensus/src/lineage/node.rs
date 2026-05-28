use blake3;
use crate::crypto::types::{NodeHash, GenesisIdentity, AuthorityReference};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImmutableLineageNode {
    pub node_hash: NodeHash,
    pub parent_hash: Option<NodeHash>,
    pub semantic_invariants: Vec<String>,
    pub authority_refs: Vec<AuthorityReference>,
    pub branch_nonce: u64,
    pub hash_bytes: [u8; 32],
}
impl ImmutableLineageNode {
    fn compute_node_hash(p: &Option<NodeHash>, inv: &[String], refs: &[AuthorityReference], nonce: u64) -> NodeHash {
        let mut bytes = Vec::new(); bytes.extend_from_slice(b"AMUN_NODE_V1");
        if let Some(par) = p { bytes.extend_from_slice(&par.as_bytes()); }
        for i in inv { let len = (i.len() as u32).to_be_bytes(); bytes.extend_from_slice(&len); bytes.extend_from_slice(i.as_bytes()); }
        let mut sorted = refs.to_vec(); sorted.sort_by(|a,b| a.to_canonical_bytes().cmp(&b.to_canonical_bytes()));
        for r in sorted { bytes.extend_from_slice(&r.to_canonical_bytes()); }
        bytes.extend_from_slice(&nonce.to_be_bytes());
        NodeHash::from_bytes(blake3::hash(&bytes).into())
    }
    pub fn create_genesis(id: GenesisIdentity) -> Self {
        let h = Self::compute_node_hash(&None, &[], &[AuthorityReference::Genesis(id)], 0);
        Self { node_hash: h, parent_hash: None, semantic_invariants: vec![], authority_refs: vec![AuthorityReference::Genesis(id)], branch_nonce: 0, hash_bytes: h.as_bytes() }
    }
    pub fn create_child(parent: NodeHash, inv: Vec<String>, refs: Vec<AuthorityReference>, nonce: u64) -> Self {
        let h = Self::compute_node_hash(&Some(parent), &inv, &refs, nonce);
        Self { node_hash: h, parent_hash: Some(parent), semantic_invariants: inv, authority_refs: refs, branch_nonce: nonce, hash_bytes: h.as_bytes() }
    }
    pub fn is_origin(&self) -> bool { self.parent_hash.is_none() }
    pub fn hash(&self) -> NodeHash { self.node_hash }
    pub fn as_bytes(&self) -> [u8; 32] { self.hash_bytes }
}
