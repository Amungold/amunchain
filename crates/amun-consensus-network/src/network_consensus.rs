use crate::engine::ConsensusEngine;
use crate::messages::{ConsensusVote, FinalityCertificate};
use amun_validator_api::NetworkProvider;
use amun_validator_identity::verify_ed25519;
use amun_validator_registry::ValidatorRead;
use std::sync::{Arc, Mutex};

/// Networked consensus: validators communicate via NetworkProvider.
///
/// N143: Replaces legacy TcpStream with `Arc<dyn NetworkProvider>`.
/// N143.4: Validates senders via ValidatorRead before processing votes.
///
/// N144 note: Still uses ConsensusVote internally.
/// RuntimeVote migration is deferred until engine.rs is ready.
pub struct NetworkConsensus {
    pub engine: Arc<Mutex<ConsensusEngine>>,
    pub validator_id: [u8; 32],
    pub network: Arc<dyn NetworkProvider>,
    pub validator_registry: Arc<dyn ValidatorRead>,
}

impl NetworkConsensus {
    pub fn new(
        validator_id: [u8; 32],
        total_validators: usize,
        network: Arc<dyn NetworkProvider>,
        validator_registry: Arc<dyn ValidatorRead>,
    ) -> Self {
        Self {
            engine: Arc::new(Mutex::new(ConsensusEngine::new(
                validator_id,
                total_validators,
            ))),
            validator_id,
            network,
            validator_registry,
        }
    }

    pub fn connect_to_peer(&self, address: &str) -> Result<[u8; 32], String> {
        let peer_id = self
            .network
            .connect_to_peer(address)
            .map_err(|e| format!("Connect failed: {}", e))?;
        Ok(*peer_id.as_bytes())
    }

    pub fn run_round(
        &self,
        height: u64,
        block_hash: [u8; 32],
        state_root: [u8; 32],
        history_root: [u8; 32],
    ) -> Result<FinalityCertificate, String> {
        let proposer_idx = {
            let engine = self.engine.lock().unwrap();
            engine.proposer_for(height)
        };

        let is_proposer = self.validator_id[0] as usize == proposer_idx + 1;

        if is_proposer {
            let mut engine = self.engine.lock().unwrap();
            engine.record_proposal(height, self.validator_id, block_hash, state_root);
        }

        let my_vote = ConsensusVote {
            voter_id: self.validator_id,
            height,
            block_hash,
            state_root,
            approve: true,
            signature: [0u8; 64],
            timestamp: 1000,
            commitment: None,
        };

        {
            let mut engine = self.engine.lock().unwrap();
            if engine.has_proposal(height) {
                engine.process_vote(&my_vote)?;
            } else {
                engine.record_proposal(
                    height,
                    [(proposer_idx + 1) as u8; 32],
                    block_hash,
                    state_root,
                );
                engine.process_vote(&my_vote)?;
            }
        }

        let encoded = postcard::to_stdvec(&my_vote).map_err(|e| e.to_string())?;
        self.network
            .broadcast(&encoded)
            .map_err(|e| format!("Broadcast failed: {}", e))?;

        let mut engine = self.engine.lock().unwrap();
        engine
            .finalize_round(height, history_root)
            .ok_or_else(|| "Failed to form QC".into())
    }

    /// Process an incoming vote from the network.
    /// Verifies sender is an active validator before passing to engine.
    pub fn process_incoming_vote(&self, data: &[u8]) -> Result<(), String> {
        let vote: ConsensusVote =
            postcard::from_bytes(data).map_err(|e| format!("Vote decode: {}", e))?;

        // N143.4: Verify sender is an active validator
        let validator_id = amun_validator_registry::ValidatorId(vote.voter_id);
        if !self.validator_registry.is_active(&validator_id) {
            return Err(format!(
                "Rejected vote from inactive validator {:?}",
                &vote.voter_id[..4]
            ));
        }

        // Verify vote signature
        let payload = amun_validator_identity::vote_signing_payload(
            &vote.voter_id,
            amun_validator_identity::signature::DEFAULT_CHAIN_ID,
            vote.height,
            0,
            &vote.block_hash,
        );
        if !verify_ed25519(&vote.voter_id, &payload, &vote.signature) {
            return Err(format!(
                "Invalid signature from validator {:?}",
                &vote.voter_id[..4]
            ));
        }

        let mut engine = self.engine.lock().unwrap();
        engine.process_vote(&vote)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_validator_api::types::id::PublicKey;
    use amun_validator_identity::authority_store::AuthorityStore;
    use amun_validator_identity::certificate_store::CertificateStore;
    use amun_validator_identity::key_store::KeyStore;
    use amun_validator_identity::IdentityService;
    use amun_validator_network::{NetworkConfig, NetworkService};
    use std::sync::Arc;

    struct MockValidatorRegistry {
        active: bool,
    }
    impl ValidatorRead for MockValidatorRegistry {
        fn get_public_key(&self, _id: &amun_validator_registry::ValidatorId) -> Option<[u8; 32]> {
            Some([1u8; 32])
        }
        fn get_voting_power(&self, _id: &amun_validator_registry::ValidatorId) -> u64 {
            1
        }
        fn is_active(&self, _id: &amun_validator_registry::ValidatorId) -> bool {
            self.active
        }
        fn total_voting_power(&self) -> u64 {
            1
        }
        fn validator_count(&self) -> usize {
            1
        }
    }

    fn make_network() -> Arc<dyn NetworkProvider> {
        let k = Arc::new(KeyStore::generate());
        let c = Arc::new(CertificateStore::new(
            CertificateStore::load_from_file("x").unwrap(),
        ));
        let a = Arc::new(AuthorityStore::from_single(
            [0u8; 32],
            PublicKey([0u8; 32]),
            1,
        ));
        Arc::new(NetworkService::new(
            Arc::new(IdentityService::new(c, k, a)),
            NetworkConfig::default(),
        ))
    }

    #[test]
    fn n68_network_consensus_single_validator() {
        let mut engine = ConsensusEngine::new([1u8; 32], 1);
        engine.record_proposal(1, [1u8; 32], [0xAA; 32], [0xBB; 32]);
        engine
            .process_vote(&ConsensusVote {
                voter_id: [1u8; 32],
                height: 1,
                block_hash: [0xAA; 32],
                state_root: [0xBB; 32],
                approve: true,
                signature: [0u8; 64],
                timestamp: 1000,
                commitment: None,
            })
            .unwrap();
        let cert = engine.finalize_round(1, [0xCC; 32]).unwrap();
        assert_eq!(cert.height, 1);
    }

    #[test]
    fn n143_reject_inactive_validator() {
        let nc = NetworkConsensus::new(
            [1u8; 32],
            4,
            make_network(),
            Arc::new(MockValidatorRegistry { active: false }),
        );
        let vote = ConsensusVote {
            voter_id: [1u8; 32],
            height: 1,
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            approve: true,
            signature: [0u8; 64],
            timestamp: 1000,
            commitment: None,
        };
        let encoded = postcard::to_stdvec(&vote).unwrap();
        assert!(nc.process_incoming_vote(&encoded).is_err());
    }
}
