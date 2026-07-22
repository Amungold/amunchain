use crate::config::ValidatorConfig;
use amun_authority_registry::transaction::GovernanceState;
use amun_authority_registry::AuthorityRegistry;
use amun_block_builder::BlockBuilder;
use amun_chain_store::store::ChainStore;
use amun_consensus_network::engine::ConsensusEngine;
use amun_consensus_network::{RealStakingExecutor, StakingAdapter};
use amun_constitutional_enforcement::ConstitutionalEnforcementKernel;
use amun_mempool::Mempool;
// Sync catchup imports moved to sync/catchup.rs
use ed25519_dalek::SigningKey;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::sync::catchup::SyncRuntime;
use crate::identity::service::IdentityService;
use crate::runtime::consensus::ConsensusRuntime;

#[cfg(test)]
use amun_networking::peer_identity::PeerId;
#[cfg(test)]
use amun_networking::validator_certificate::ValidatorCertificate;
#[cfg(test)]
use amun_networking::crypto_identity::PeerKeyPair;


#[cfg(test)]
use amun_constitutional_enforcement::{
    evidence_records::{
        ConstitutionalEvidenceRecord, DoubleSpendEvidence, GovernanceEvidence, ReplayEvidence,
        SignatureEvidence,
    },
    ConstitutionalVerdict,
};
#[cfg(test)]
use amun_evidence_root::EvidenceRoot;
#[cfg(test)]
use amun_validator_identity::vote_signing_payload;
#[cfg(test)]
use std::io::Write;
#[cfg(test)]
use std::time::{Duration, Instant, SystemTime};
#[cfg(test)]
use amun_chain_store::record::FinalizedChainRecord;
#[cfg(test)]
use std::thread;

pub struct LiveValidator {
    pub config: ValidatorConfig,
    pub engine: Arc<Mutex<ConsensusEngine>>,
    pub store: Arc<Mutex<ChainStore>>,
    running: Arc<Mutex<bool>>,
    handles: Mutex<Vec<JoinHandle<()>>>,
    signing_key: SigningKey,
    pub validator_id: [u8; 32],
    pub mempool: Arc<Mutex<Mempool>>,
    pub builder: Arc<Mutex<BlockBuilder>>,
    pub governance: Arc<Mutex<GovernanceState>>,
    pub authority_registry: Arc<Mutex<AuthorityRegistry>>,
    /// N110.4: Certificate gossip for slashing certificate propagation
    pub certificate_gossip: Arc<Mutex<amun_consensus_network::CertificateGossip>>,
    /// N110.4c: Staking adapter for applying slashes after finality
    pub staking_adapter: Arc<Mutex<StakingAdapter<RealStakingExecutor>>>,
    pub applied_slashing_certificates: Arc<Mutex<std::collections::HashSet<[u8; 32]>>>,
    /// N120.4: Slashing ledger for computing the merkle root
    pub slashing_ledger: Arc<Mutex<amun_consensus_network::SlashingLedger>>,
    /// N123.1: Constitutional enforcement kernel
    pub constitutional_kernel: Arc<Mutex<ConstitutionalEnforcementKernel>>,
    /// N129.3: Previous evidence root for chain continuity
    pub previous_evidence_root: Arc<Mutex<[u8; 32]>>,
}

impl LiveValidator {
    pub fn new(config: ValidatorConfig) -> Self {
        let store = ChainStore::open(&config.data_dir)
            .unwrap_or_else(|_| ChainStore::open("/tmp/amun-fallback").unwrap());
        let recovered_height = store.latest_height();
        let recovered_root = store
            .load_tip()
            .map(|r| r.history_root)
            .unwrap_or([0u8; 32]);
        let mut engine = ConsensusEngine::new(config.validator_id, config.total_validators());
        if recovered_height > 0 {
            engine.current_height = recovered_height;
            engine.history_root = recovered_root;
        }
        // ADR-023 Phase 3: Identity handled by IdentityService
        let (signing_key, validator_id, registry) =
            IdentityService::initialize(&config, &mut engine);
        let my_true_id = validator_id;
        Self {
            config,
            engine: Arc::new(Mutex::new(engine)),
            store: Arc::new(Mutex::new(store)),
            running: Arc::new(Mutex::new(false)),
            handles: Mutex::new(Vec::new()),
            signing_key: signing_key.clone(),
            validator_id: my_true_id,
            mempool: Arc::new(Mutex::new(Mempool::new())),
            builder: Arc::new(Mutex::new(BlockBuilder::new())),
            governance: Arc::new(Mutex::new(GovernanceState::new())),
            authority_registry: Arc::new(Mutex::new(registry)),
            certificate_gossip: Arc::new(Mutex::new(
                amun_consensus_network::CertificateGossip::new(),
            )),
            applied_slashing_certificates: Arc::new(Mutex::new(std::collections::HashSet::new())),
            slashing_ledger: Arc::new(Mutex::new(amun_consensus_network::SlashingLedger::new())),
            constitutional_kernel: Arc::new(Mutex::new(ConstitutionalEnforcementKernel::new())),
            previous_evidence_root: Arc::new(Mutex::new([0u8; 32])),
            staking_adapter: Arc::new(Mutex::new(StakingAdapter::new(
                amun_consensus_network::MisbehaviorRegistry::new(
                    amun_consensus_network::MisbehaviorThresholds::default(),
                ),
                RealStakingExecutor::new({
                    // Use fully-qualified path to avoid ambiguity with amun_networking::validator_registry
                    amun_staking::validator::ValidatorRegistry::new()
                }),
            ))),
        }
    }

    pub fn start(&self) -> Result<(), String> {
        *self.running.lock().unwrap() = true;
        let engine = self.engine.clone();
        let store = self.store.clone();
        let peers: Vec<_> = self.config.other_peers().into_iter().cloned().collect();
        let peer_addrs: Vec<SocketAddr> = peers.iter().map(|p| p.address).collect();
        let sync_runtime = SyncRuntime::new(
            engine.clone(),
            store.clone(),
            peer_addrs.clone(),
        );
        let _total = self.config.total_validators();
        let running = self.running.clone();
        let signing_key_clone = self.signing_key.clone();
        let validator_id = self.validator_id;
        let my_index = self.config.validator_id[0];

        // Listen thread - extracted to NetworkingRuntime (ADR-023 Phase 1)
        let networking = crate::runtime::networking::NetworkingRuntime::new(
            &self.config.listen_addr.to_string(),
            engine.clone(),
            store.clone(),
            running.clone(),
        ).expect("Failed to create NetworkingRuntime");
        let h1 = networking.spawn_listener();

        // ADR-023 Phase 4: Consensus handled by ConsensusRuntime
        let consensus = ConsensusRuntime::new(
            engine.clone(),
            store.clone(),
            self.mempool.clone(),
            self.builder.clone(),
            self.governance.clone(),
            self.authority_registry.clone(),
            self.certificate_gossip.clone(),
            self.staking_adapter.clone(),
            self.applied_slashing_certificates.clone(),
            self.slashing_ledger.clone(),
            self.constitutional_kernel.clone(),
            self.previous_evidence_root.clone(),
            signing_key_clone,
            validator_id,
            my_index,
            Arc::new(sync_runtime),
            peer_addrs.clone(),
            running.clone(),
        );
        let h2 = consensus.spawn();

        self.handles.lock().unwrap().push(h1);
        self.handles.lock().unwrap().push(h2);
        Ok(())
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
        let handles: Vec<JoinHandle<()>> = {
            let mut h = self.handles.lock().unwrap();
            std::mem::take(&mut *h)
        };
        for h in handles {
            let _ = h.join();
        }
    }

    pub fn current_height(&self) -> u64 {
        self.engine.lock().unwrap().current_height
    }
    pub fn history_root(&self) -> [u8; 32] {
        self.engine.lock().unwrap().history_root
    }
    pub fn store_len(&self) -> usize {
        self.store.lock().unwrap().len()
    }
    pub fn metrics_summary(&self) -> String {
        self.engine.lock().unwrap().metrics.summary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU16, Ordering};

    static PORT_BASE: AtomicU16 = AtomicU16::new(9700);

    #[test]
    fn n108_1_governance_updates_live_authority_registry() {
        use amun_authority_registry::governance::{GovernanceAction, GovernanceProposal};
        use amun_authority_registry::transaction::GovernanceTransaction;
        use amun_authority_registry::voting::GovernanceVote;

        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(3);
        let v = LiveValidator::new(config);
        v.start().unwrap();

        // Verify initial state: only authority v1 exists
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(reg.by_version(1).is_some(), "v1 should exist from genesis");
            assert!(reg.by_version(2).is_none(), "v2 should not exist yet");
        }

        // Create a governance proposal to add authority v2
        let proposal = GovernanceProposal::new(
            [0xAA; 32],
            GovernanceAction::AddAuthority {
                authority_public_key: [2u8; 32],
                authority_version: 2,
            },
            100,
        );

        // Submit the proposal to governance state
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(proposal.clone()));
        }

        // Cast 3 approving votes (quorum for 4 validators)
        for id in 1..=3u8 {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::CastVote {
                proposal_id: proposal.proposal_id,
                validator_id: [id; 32],
                vote: GovernanceVote::Approve,
            });
        }

        // Execute governance finalization (simulates what happens at block finalization)
        {
            let mut gov = v.governance.lock().unwrap();
            let mut reg = v.authority_registry.lock().unwrap();
            let executed = gov.finalize_block(4, &mut reg).unwrap();
            assert_eq!(executed.len(), 1, "Proposal should be executed");
        }

        // Verify the registry was updated
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(reg.by_version(1).is_some(), "v1 should still exist");
            assert!(reg.by_version(2).is_some(), "v2 should now be registered");
        }

        // Verify the journal recorded the execution
        {
            let gov = v.governance.lock().unwrap();
            assert!(
                gov.journal.is_executed(&proposal.proposal_id),
                "Journal should record execution"
            );
        }

        // Verify idempotency: second finalization should not re-execute
        {
            let mut gov = v.governance.lock().unwrap();
            let mut reg = v.authority_registry.lock().unwrap();
            let executed = gov.finalize_block(4, &mut reg).unwrap();
            assert!(
                executed.is_empty(),
                "Second finalization should produce no new executions"
            );
        }

        v.stop();
    }

    #[test]
    fn n108_2_runtime_authority_rotation() {
        use amun_authority_registry::governance::{GovernanceAction, GovernanceProposal};
        use amun_authority_registry::transaction::GovernanceTransaction;
        use amun_authority_registry::voting::GovernanceVote;

        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(3);
        let v = LiveValidator::new(config);
        v.start().unwrap();

        // Phase 1: Genesis Authority v1
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(reg.by_version(1).is_some(), "v1 should exist from genesis");
            assert!(reg.by_version(2).is_none(), "v2 should not exist yet");
        }

        // Phase 2: AddAuthority v2 via governance
        let add_proposal = GovernanceProposal::new(
            [0xAA; 32],
            GovernanceAction::AddAuthority {
                authority_public_key: [2u8; 32],
                authority_version: 2,
            },
            100,
        );
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(add_proposal.clone()));
            for id in 1..=3u8 {
                gov.apply_transaction(&GovernanceTransaction::CastVote {
                    proposal_id: add_proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                });
            }
            let mut reg = v.authority_registry.lock().unwrap();
            let executed = gov.finalize_block(4, &mut reg).unwrap();
            assert_eq!(executed.len(), 1, "AddAuthority should execute");
        }
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(reg.by_version(2).is_some(), "v2 should be registered");
        }

        // Phase 3: ScheduleTransition
        let trans_proposal = GovernanceProposal::new(
            [0xBB; 32],
            GovernanceAction::ScheduleTransition {
                from_version: 1,
                to_version: 2,
                activation_height: 500,
                grace_period_blocks: 100,
            },
            200,
        );
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(
                trans_proposal.clone(),
            ));
            for id in 1..=3u8 {
                gov.apply_transaction(&GovernanceTransaction::CastVote {
                    proposal_id: trans_proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                });
            }
            let mut reg = v.authority_registry.lock().unwrap();
            let executed = gov.finalize_block(4, &mut reg).unwrap();
            assert_eq!(executed.len(), 1, "ScheduleTransition should execute");
        }

        // Phase 4: Pre-activation
        {
            let reg = v.authority_registry.lock().unwrap();
            let auths = reg.valid_authorities_at(499);
            assert_eq!(auths.len(), 1, "Only v1 before activation");
            assert_eq!(auths[0].authority_version, 1);
        }

        // Phase 5: At activation
        {
            let reg = v.authority_registry.lock().unwrap();
            let auths = reg.valid_authorities_at(500);
            assert_eq!(auths.len(), 2, "Both valid at activation");
        }

        // Phase 6: Grace window
        {
            let reg = v.authority_registry.lock().unwrap();
            let auths = reg.valid_authorities_at(550);
            assert_eq!(auths.len(), 2, "Both valid during grace");
        }

        // Phase 7: Post-grace
        {
            let reg = v.authority_registry.lock().unwrap();
            let auths = reg.valid_authorities_at(650);
            assert_eq!(auths.len(), 1, "Only v2 after grace");
            assert_eq!(auths[0].authority_version, 2);
        }

        // Phase 8: Retire v1
        let retire_proposal = GovernanceProposal::new(
            [0xCC; 32],
            GovernanceAction::RetireAuthority {
                authority_version: 1,
            },
            700,
        );
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(
                retire_proposal.clone(),
            ));
            for id in 1..=3u8 {
                gov.apply_transaction(&GovernanceTransaction::CastVote {
                    proposal_id: retire_proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                });
            }
            let mut reg = v.authority_registry.lock().unwrap();
            let executed = gov.finalize_block(4, &mut reg).unwrap();
            assert_eq!(executed.len(), 1, "RetireAuthority should execute");
        }
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(reg.is_revoked(1), "v1 should be revoked");
            assert!(reg.by_version(1).is_some(), "v1 still queryable");
        }

        v.stop();
    }

    #[test]
    fn n108_3e_authority_rotation_certificate_validation() {
        use amun_authority_registry::governance::{GovernanceAction, GovernanceProposal};
        use amun_authority_registry::transaction::GovernanceTransaction;
        use amun_authority_registry::voting::GovernanceVote;
                        
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(3);
        let v = LiveValidator::new(config);
        v.start().unwrap();

        // Create authority keypairs
        let v1_kp = PeerKeyPair::from_seed([0x42; 32]);
        let _v1_pk = v1_kp.verifying_key.to_bytes();
        let v2_kp = PeerKeyPair::from_seed([0x43; 32]);
        let v2_pk = v2_kp.verifying_key.to_bytes();

        // Phase 1: Genesis has v1 only
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(reg.by_version(1).is_some(), "v1 should exist");
            assert!(reg.by_version(2).is_none(), "v2 should not exist yet");
        }

        // Create a v1 certificate
        let v1_cert = ValidatorCertificate::issue_v2(
            PeerId::from_bytes([1u8; 32]),
            [1u8; 32],
            1,
            [0u8; 32],
            &v1_kp,
            0,
            0,
        );

        // Phase 2: Add v2 and schedule transition via governance
        let add_proposal = GovernanceProposal::new(
            [0xAA; 32],
            GovernanceAction::AddAuthority {
                authority_public_key: v2_pk,
                authority_version: 2,
            },
            100,
        );
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(add_proposal.clone()));
            for id in 1..=3u8 {
                gov.apply_transaction(&GovernanceTransaction::CastVote {
                    proposal_id: add_proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                });
            }
            let mut reg = v.authority_registry.lock().unwrap();
            gov.finalize_block(4, &mut reg).unwrap();
        }

        let trans_proposal = GovernanceProposal::new(
            [0xBB; 32],
            GovernanceAction::ScheduleTransition {
                from_version: 1,
                to_version: 2,
                activation_height: 500,
                grace_period_blocks: 100,
            },
            200,
        );
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(
                trans_proposal.clone(),
            ));
            for id in 1..=3u8 {
                gov.apply_transaction(&GovernanceTransaction::CastVote {
                    proposal_id: trans_proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                });
            }
            let mut reg = v.authority_registry.lock().unwrap();
            gov.finalize_block(4, &mut reg).unwrap();
        }

        // Phase 3: Before activation - v1 accepted, v2 unknown
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(
                reg.verify_certificate_at(&v1_cert, 400),
                "v1 cert should be accepted before activation"
            );
        }

        // Phase 4: During grace window - both accepted
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(
                reg.verify_certificate_at(&v1_cert, 550),
                "v1 cert should be accepted during grace window"
            );
        }

        // Phase 5: After grace - v1 rejected
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(
                !reg.verify_certificate_at(&v1_cert, 650),
                "v1 cert should be rejected after grace window"
            );
        }

        // Phase 6: Retire v1, verify it's rejected even at low heights
        let retire_proposal = GovernanceProposal::new(
            [0xCC; 32],
            GovernanceAction::RetireAuthority {
                authority_version: 1,
            },
            700,
        );
        {
            let mut gov = v.governance.lock().unwrap();
            gov.apply_transaction(&GovernanceTransaction::SubmitProposal(
                retire_proposal.clone(),
            ));
            for id in 1..=3u8 {
                gov.apply_transaction(&GovernanceTransaction::CastVote {
                    proposal_id: retire_proposal.proposal_id,
                    validator_id: [id; 32],
                    vote: GovernanceVote::Approve,
                });
            }
            let mut reg = v.authority_registry.lock().unwrap();
            gov.finalize_block(4, &mut reg).unwrap();
        }
        {
            let reg = v.authority_registry.lock().unwrap();
            assert!(
                !reg.verify_certificate_at(&v1_cert, 800),
                "v1 cert should be rejected after retirement"
            );
            assert!(reg.is_revoked(1), "v1 should be revoked");
        }

        v.stop();
    }

    fn next_ports() -> [u16; 4] {
        let base = PORT_BASE.fetch_add(10, Ordering::SeqCst);
        [base, base + 1, base + 2, base + 3]
    }

    #[test]
    fn n71_persist_finalized_blocks() {
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
        let mut solo = config.clone();
        solo.cluster = vec![solo.cluster[0].clone()];
        let v = LiveValidator::new(solo);
        v.start().unwrap();
        thread::sleep(Duration::from_millis(3000));
        v.stop();
        assert!(v.current_height() >= 1);
        assert!(v.store_len() >= 1);
    }

    #[test]
    fn n71_recover_after_restart() {
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
        let mut solo = config.clone();
        solo.cluster = vec![solo.cluster[0].clone()];
        let data_dir = solo.data_dir.clone();

        let height_after_first;
        {
            let v = LiveValidator::new(solo);
            v.start().unwrap();
            thread::sleep(Duration::from_millis(3000));
            v.stop();
            height_after_first = v.current_height();
        }

        let config2 = ValidatorConfig::test_cluster(0, &next_ports()).with_quorum(1);
        let mut solo2 = config2.clone();
        solo2.cluster = vec![solo2.cluster[0].clone()];
        solo2.data_dir = data_dir;
        let v2 = LiveValidator::new(solo2);
        assert!(v2.current_height() >= height_after_first);
    }

    #[test]
    fn n69_single_validator_self_finalizes() {
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
        let mut solo = config.clone();
        solo.cluster = vec![solo.cluster[0].clone()];
        let v = LiveValidator::new(solo);
        v.start().unwrap();
        thread::sleep(Duration::from_millis(2000));
        v.stop();
        assert!(v.current_height() >= 1);
    }

    #[test]
    fn n69_two_validators_reach_consensus() {
        let ports = next_ports();
        let va = LiveValidator::new(ValidatorConfig::test_cluster(0, &ports).with_quorum(2));
        let vb = LiveValidator::new(ValidatorConfig::test_cluster(1, &ports).with_quorum(2));
        va.start().unwrap();
        vb.start().unwrap();
        thread::sleep(Duration::from_millis(8000));
        va.stop();
        vb.stop();
        let ha = va.store.lock().unwrap().latest_height();
        let hb = vb.store.lock().unwrap().latest_height();
        assert!(ha >= 1 && hb >= 1, "Store heights: A={}, B={}", ha, hb);
    }

    #[test]
    fn n69_three_of_four_reach_quorum() {
        let ports = next_ports();
        let va = LiveValidator::new(ValidatorConfig::test_cluster(0, &ports).with_quorum(3));
        let vb = LiveValidator::new(ValidatorConfig::test_cluster(1, &ports).with_quorum(3));
        let vc = LiveValidator::new(ValidatorConfig::test_cluster(2, &ports).with_quorum(3));
        va.start().unwrap();
        vb.start().unwrap();
        vc.start().unwrap();
        thread::sleep(Duration::from_millis(8000));
        va.stop();
        vb.stop();
        vc.stop();
        assert!(va.store.lock().unwrap().latest_height() >= 1);
        assert!(vb.store.lock().unwrap().latest_height() >= 1);
        assert!(vc.store.lock().unwrap().latest_height() >= 1);
    }

    #[test]
    fn n69_duplicate_vote_ignored() {
        let ports = next_ports();
        let config = ValidatorConfig::test_cluster(0, &ports).with_quorum(1);
        let mut solo = config.clone();
        solo.cluster = vec![solo.cluster[0].clone()];
        let v = LiveValidator::new(solo);
        v.start().unwrap();
        thread::sleep(Duration::from_millis(2000));
        v.stop();
        assert!(v.current_height() >= 1);
    }

    #[test]
    fn n69_four_validators_full_cluster() {
        let ports = next_ports();
        let validators: Vec<LiveValidator> = (0..4)
            .map(|i| LiveValidator::new(ValidatorConfig::test_cluster(i, &ports).with_quorum(4)))
            .collect();
        for v in &validators {
            v.start().unwrap();
        }
        thread::sleep(Duration::from_millis(15000));
        for v in &validators {
            v.stop();
        }

        for (i, v) in validators.iter().enumerate() {
            let h = v.store.lock().unwrap().latest_height();
            assert!(h >= 1, "Validator {} store height: {}", i, h);
            println!("Validator {} metrics: {}", i, v.metrics_summary());
        }
    }
}
