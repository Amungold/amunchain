pub mod engine;
pub mod lifecycle;
pub mod messages;
pub mod misbehavior;
pub mod network_consensus;
pub mod proposal;
pub mod round;
pub mod slashing;
pub mod validator_registry;
pub mod validator_status;

// N109 modules
pub mod certificate_evidence_validation;
pub mod certificate_gossip;
pub mod evidence_gossip;
pub mod evidence_push;
pub mod evidence_push_processor;
pub mod validator_identity;

pub use evidence_push::{EvidencePush, EvidencePushConfig};
pub use evidence_push_processor::{process_incoming_evidence_push, PushProcessResult};
pub use validator_identity::{ValidatorIdentity, ValidatorIdentityRegistry};
pub mod engine_metrics;
pub mod evidence_store;
pub mod execution_commitment;
pub mod execution_receipt;
pub mod finality_gate;
pub mod integrated_slashing;
pub mod metrics;
pub mod misbehavior_registry;
pub mod multi_signer_certificate;
pub mod re_executor;
pub mod real_staking_adapter;
pub mod slashing_certificate;
pub mod slashing_certificate_builder;
pub mod slashing_fraud_proof;
pub mod slashing_inclusion_proof;
pub mod slashing_ledger;
pub mod slashing_merkle;
pub mod slashing_state;
pub mod staking_adapter;
pub mod validation;
pub mod vote_binding;

pub use certificate_evidence_validation::{
    build_missing_evidence_request, process_evidence_response, validate_certificate_evidence,
    EvidenceValidationResult,
};
pub use certificate_gossip::CertificateGossip;
pub use evidence_gossip::{EvidenceAnnouncement, EvidenceGossip};
pub use evidence_store::{EvidenceRecord, EvidenceStatus, EvidenceStore, EvidenceType};
pub use execution_commitment::ExecutionCommitment;
pub use execution_receipt::ExecutionReceipt;
pub use finality_gate::{execute_if_finalized, is_certificate_finalized};
pub use integrated_slashing::{IntegratedSlashingPipeline, PipelineResult};
pub use messages::{
    BlockProposal, ConsensusVote, N109BlockProposal, N109ConsensusVote, NetworkMessage,
};
pub use messages::{MissingEvidenceRequest, MissingEvidenceResponse};
pub use metrics::ConsensusMetrics;
pub use misbehavior_registry::{
    MisbehaviorRecord, MisbehaviorRegistry, MisbehaviorThresholds, ValidatorAction, ValidatorStatus,
};
pub use multi_signer_certificate::MultiSignerCertificate;
pub use re_executor::verify_block_execution;
pub use real_staking_adapter::RealStakingExecutor;
pub use slashing_certificate::{CertificateResultingStatus, EvidenceCount, SlashingCertificate};
pub use slashing_certificate_builder::SlashingCertificateBuilder;
pub use slashing_fraud_proof::SlashingFraudProof;
pub use slashing_inclusion_proof::{build_inclusion_proof, SlashingInclusionProof};
pub use slashing_ledger::{certificate_id, ExecutedSlash, SlashingLedger};
pub use slashing_merkle::merkle_root;
pub use slashing_state::SlashingState;
pub use staking_adapter::{SlashResult, SlashingExecutor, StakingAdapter};
pub use vote_binding::verify_vote_binding;
