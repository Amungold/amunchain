use crate::crypto::types::NodeHash;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsensusAliveness { Living, Stable, Weakened, Inert, Critical, Dead }
#[derive(Debug, Clone)]
pub struct AlivenessAttestation { pub node_hash: NodeHash, pub aliveness: ConsensusAliveness, pub epoch: u64, pub validator_id: u64, pub signature: crate::crypto::types::SignatureBytes }
