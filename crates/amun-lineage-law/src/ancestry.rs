use super::lineage::LineageProof;

/// ProtocolAncestry represents a node in the protocol lineage tree.
#[derive(Debug, Clone)]
pub struct ProtocolAncestry {
    pub protocol_version: u32,
    pub freeze_certificate_hash: [u8; 32],
    pub parent_version: Option<u32>,
    pub parent_freeze_hash: Option<[u8; 32]>,
    pub lineage_proof: Option<LineageProof>,
    pub descendants: Vec<u32>,
}

impl ProtocolAncestry {
    pub fn new(
        version: u32,
        freeze_hash: [u8; 32],
        parent_version: Option<u32>,
        parent_freeze_hash: Option<[u8; 32]>,
        lineage_proof: Option<LineageProof>,
    ) -> Self {
        Self {
            protocol_version: version,
            freeze_certificate_hash: freeze_hash,
            parent_version,
            parent_freeze_hash,
            lineage_proof,
            descendants: Vec::new(),
        }
    }

    /// Check if this protocol is a direct descendant of another.
    pub fn is_direct_descendant_of(&self, ancestor_version: u32) -> bool {
        self.parent_version == Some(ancestor_version)
    }

    /// Check if this protocol has a verified lineage proof.
    pub fn has_verified_lineage(&self) -> bool {
        self.lineage_proof
            .as_ref()
            .map(|p| p.is_verified)
            .unwrap_or(false)
    }
}

/// AncestryChain represents the full lineage from genesis to current protocol.
#[derive(Debug, Clone)]
pub struct AncestryChain {
    pub nodes: Vec<ProtocolAncestry>,
    pub genesis_version: u32,
    pub current_version: u32,
    pub chain_hash: [u8; 32],
}

impl AncestryChain {
    pub fn new(genesis: ProtocolAncestry) -> Self {
        let mut chain = Self {
            genesis_version: genesis.protocol_version,
            current_version: genesis.protocol_version,
            nodes: vec![genesis],
            chain_hash: [0u8; 32],
        };
        chain.chain_hash = chain.compute_hash();
        chain
    }

    pub fn add_descendant(&mut self, descendant: ProtocolAncestry) {
        if let Some(parent) = self
            .nodes
            .iter_mut()
            .find(|n| n.protocol_version == self.current_version)
        {
            parent.descendants.push(descendant.protocol_version);
        }
        self.current_version = descendant.protocol_version;
        self.nodes.push(descendant);
        self.chain_hash = self.compute_hash();
    }

    fn compute_hash(&self) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        h.update(b"AMUN_ANCESTRY_CHAIN_V1");
        for node in &self.nodes {
            h.update(node.protocol_version.to_be_bytes());
            h.update(node.freeze_certificate_hash);
        }
        h.finalize().into()
    }

    pub fn verify(&self) -> bool {
        self.compute_hash() == self.chain_hash
    }

    /// Check if a version is in the lawful ancestry chain.
    pub fn contains(&self, version: u32) -> bool {
        self.nodes.iter().any(|n| n.protocol_version == version)
    }
}
