pub mod engine;
pub mod messages;
pub mod misbehavior;
pub mod network_consensus;
pub mod slashing;
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
pub mod evidence_store;
pub mod execution_commitment;
pub mod execution_receipt;
pub mod integrated_slashing;
pub mod metrics;
pub mod misbehavior_registry;
pub mod re_executor;
pub mod real_staking_adapter;
pub mod slashing_certificate;
pub mod slashing_certificate_builder;
pub mod multi_signer_certificate;
pub mod finality_gate;
pub mod slashing_ledger;
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
pub use integrated_slashing::{IntegratedSlashingPipeline, PipelineResult};
pub use messages::{
    BlockProposal, ConsensusVote, N109BlockProposal, N109ConsensusVote, NetworkMessage,
};
pub use messages::{MissingEvidenceRequest, MissingEvidenceResponse};
pub use metrics::ConsensusMetrics;
pub use misbehavior_registry::{
    MisbehaviorRecord, MisbehaviorRegistry, MisbehaviorThresholds, ValidatorAction, ValidatorStatus,
};
pub use re_executor::verify_block_execution;
pub use real_staking_adapter::RealStakingExecutor;
pub use slashing_certificate::{CertificateResultingStatus, EvidenceCount, SlashingCertificate};
pub use slashing_certificate_builder::SlashingCertificateBuilder;
pub use multi_signer_certificate::MultiSignerCertificate;
pub use finality_gate::{is_certificate_finalized, execute_if_finalized};
pub use slashing_ledger::{SlashingLedger, ExecutedSlash, certificate_id};
pub use staking_adapter::{SlashResult, SlashingExecutor, StakingAdapter};
pub use vote_binding::verify_vote_binding;
