use crate::block_dag::{BlockDAG, BlockNode};
use crate::commit::CommitRule;
use crate::fork_choice::ForkChoice;
use amun_chain_position::ChainPosition;
use amun_quorum_certificate::QuorumCertificate;
use amun_wal::{AuthorityValidation, RecoveryMode, WALEntry, WriteAheadLog};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub struct PersistentConsensusState {
    wal: WriteAheadLog,
    pub fork_choice: ForkChoice,
    pub commit_rule: CommitRule,
    pub dag: BlockDAG,
    applied_sequences: BTreeSet<u64>,
    seq_payload_hashes: BTreeMap<u64, String>,
    known_blocks: BTreeSet<[u8; 32]>,
    committed_blocks_set: BTreeSet<[u8; 32]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QCEventFull {
    block_hash: String,
    round: u64,
    height: u64,
    parent_hash: String,
    vote_count: usize,
    total_weight: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommitEvent {
    block_hash: String,
    height: u64,
    commit_index: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FinalizeEvent {
    height: u64,
    finalized_block: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotCheckpoint {
    pub commit_index: u64,
    pub finalized_height: u64,
    pub snapshot_sequence: u64,
    pub snapshot_chain_hash: String,
    pub canonical_tip: String,
    pub dag_block_count: usize,
    pub locked_qc_block: Option<String>,
    pub locked_qc_round: Option<u64>,
    pub genesis_hash: String,
    pub epoch_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsensusStateDigest {
    pub commit_index: u64,
    pub finalized_height: u64,
    pub locked_qc_block: Option<String>,
    pub locked_qc_round: Option<u64>,
    pub canonical_tip: String,
    pub dag_block_count: usize,
    pub spine_length: usize,
    pub fork_choice_high_qc_count: usize,
    pub applied_sequence_count: usize,
    pub wal_segment: u64,
    pub wal_chain_hash: String,
}

impl PersistentConsensusState {
    pub fn open(wal_path: &str, genesis_hash: [u8; 32]) -> Result<Self, String> {
        let has_segments = std::fs::read_dir(
            std::path::Path::new(wal_path)
                .parent()
                .unwrap_or(std::path::Path::new(".")),
        )
        .map(|dir| {
            let prefix = std::path::Path::new(wal_path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            dir.filter_map(|e| e.ok()).any(|e| {
                e.file_name().to_string_lossy().starts_with(&prefix)
                    && e.file_name().to_string_lossy().ends_with(".wal")
            })
        })
        .unwrap_or(false);
        if has_segments {
            Self::recover_from_wal(wal_path, genesis_hash, RecoveryMode::Strict)
        } else {
            Ok(Self::from_existing_wal(
                WriteAheadLog::open(wal_path)?,
                genesis_hash,
            ))
        }
    }

    pub fn open_lenient(wal_path: &str, genesis_hash: [u8; 32]) -> Result<Self, String> {
        Self::recover_from_wal(wal_path, genesis_hash, RecoveryMode::Lenient)
    }

    fn from_existing_wal(wal: WriteAheadLog, genesis_hash: [u8; 32]) -> Self {
        let mut cr = CommitRule::new();
        cr.register_block(genesis_hash, [0; 32], 0, 0);
        let mut known = BTreeSet::new();
        known.insert(genesis_hash);
        Self {
            wal,
            fork_choice: ForkChoice::new(),
            commit_rule: cr,
            dag: BlockDAG::new(genesis_hash),
            applied_sequences: BTreeSet::new(),
            seq_payload_hashes: BTreeMap::new(),
            known_blocks: known,
            committed_blocks_set: BTreeSet::new(),
        }
    }

    fn recover_from_wal(
        wal_path: &str,
        genesis_hash: [u8; 32],
        mode: RecoveryMode,
    ) -> Result<Self, String> {
        let wal = WriteAheadLog::open_with_mode(wal_path, mode)?;
        let auth = wal.validate_authority_chain()?;
        if !auth.is_valid {
            return Err(format!(
                "Authority validation failed: {:?}",
                auth.violations
            ));
        }
        let entries = wal.read_all()?;
        let mut state = Self::from_existing_wal(wal, genesis_hash);
        for entry in &entries {
            state.apply_wal_entry(entry)?;
        }
        Ok(state)
    }

    /// Historical continuity: snapshot chain must appear in WAL, not match current head
    pub fn validate_snapshot_binding(&self, snapshot: &SnapshotCheckpoint) -> Result<(), String> {
        let entries = self
            .wal
            .read_all()
            .map_err(|e| format!("WAL read: {}", e))?;
        let found = entries.iter().any(|e| {
            e.sequence == snapshot.snapshot_sequence && e.chain_hash == snapshot.snapshot_chain_hash
        });

        if !found {
            return Err(format!(
                "Snapshot sequence {} not found in WAL with matching chain hash",
                snapshot.snapshot_sequence
            ));
        }
        if snapshot.epoch_id != WriteAheadLog::encode_hash(self.wal.epoch_id()) {
            return Err("Epoch mismatch".to_string());
        }
        Ok(())
    }

    pub fn validate_authority(&self) -> Result<AuthorityValidation, String> {
        self.wal.validate_authority_chain()
    }
    pub fn check_corruption(&self) -> Result<amun_wal::CorruptionReport, String> {
        self.wal.determine_corruption_action()
    }

    fn require_block_known(&self, bh: &[u8; 32], ctx: &str) -> Result<(), String> {
        if !self.known_blocks.contains(bh) {
            Err(format!(
                "Causal: {} not known at {}",
                Self::encode_hash(*bh),
                ctx
            ))
        } else {
            Ok(())
        }
    }
    fn require_block_committed(&self, bh: &[u8; 32], ctx: &str) -> Result<(), String> {
        if !self.committed_blocks_set.contains(bh) {
            Err(format!(
                "Causal: {} not committed at {}",
                Self::encode_hash(*bh),
                ctx
            ))
        } else {
            Ok(())
        }
    }

    fn apply_wal_entry(&mut self, entry: &WALEntry) -> Result<(), String> {
        if let Some(existing) = self.seq_payload_hashes.get(&entry.sequence) {
            if *existing != entry.payload_hash {
                return Err(format!("Idempotency violation at {}", entry.sequence));
            }
            return Ok(());
        }
        self.seq_payload_hashes
            .insert(entry.sequence, entry.payload_hash.clone());

        match entry.entry_type.as_str() {
            "QC" => {
                let e: QCEventFull =
                    serde_json::from_str(&entry.payload_json).map_err(|e| format!("QC: {}", e))?;
                let bh = Self::decode_hash_safe(&e.block_hash)?;
                let ph = Self::decode_hash_safe(&e.parent_hash)?;
                if ph != [0u8; 32] {
                    self.require_block_known(&ph, "QC")?;
                }
                self.commit_rule.register_block(bh, ph, e.height, e.round);
                self.known_blocks.insert(bh);
                let _ = self.dag.add_block(BlockNode::new(
                    bh,
                    Some(ph),
                    ChainPosition::new(0, e.height),
                    e.round,
                    None,
                    bh,
                ));

                // R1: Rebuild fork choice state from WAL metadata.
                // Both live execution (via record_qc -> update_qc) and WAL replay
                // now use the same apply_qc_core() path through ForkChoice.
                let metadata = crate::fork_choice::QcMetadata {
                    block_hash: bh,
                    parent_hash: ph,
                    round: e.round,
                    height: e.height,
                };
                self.fork_choice.apply_qc_core(&metadata, &self.dag);
            }
            "COMMIT" => {
                let e: CommitEvent =
                    serde_json::from_str(&entry.payload_json).map_err(|e| format!("C: {}", e))?;
                let bh = Self::decode_hash_safe(&e.block_hash)?;
                self.require_block_known(&bh, "COMMIT")?;
                if e.commit_index > self.commit_rule.commit_index {
                    self.commit_rule.commit_index = e.commit_index;
                }
                self.dag.commit_block(&bh);
                self.committed_blocks_set.insert(bh);
            }
            "FINALIZE" => {
                let e: FinalizeEvent =
                    serde_json::from_str(&entry.payload_json).map_err(|e| format!("F: {}", e))?;
                let fb = Self::decode_hash_safe(&e.finalized_block)?;
                self.require_block_known(&fb, "FINALIZE")?;
                self.require_block_committed(&fb, "FINALIZE")?;
                if e.height > self.commit_rule.finalized_height {
                    self.commit_rule.finalize(e.height);
                    self.dag.finalize_and_prune(e.height, fb);
                }
            }
            _ => return Err(format!("Unknown type: {}", entry.entry_type)),
        }
        Ok(())
    }

    pub fn record_qc(&mut self, qc: &QuorumCertificate) -> Result<(), String> {
        let ev = QCEventFull {
            block_hash: Self::encode_hash(qc.block_hash),
            round: qc.round,
            height: qc.position.sequence,
            parent_hash: Self::encode_hash(qc.parent_hash),
            vote_count: qc.votes.len(),
            total_weight: qc.total_weight(),
        };
        let json = serde_json::to_string(&ev).map_err(|e| format!("Ser: {}", e))?;
        let entry = self.wal.append_and_return_entry("QC", &json)?;
        self.apply_wal_entry(&entry)?;
        Ok(())
    }
    pub fn record_commit(&mut self, bh: [u8; 32], h: u64) -> Result<(), String> {
        let ev = CommitEvent {
            block_hash: Self::encode_hash(bh),
            height: h,
            commit_index: self.commit_rule.commit_index,
        };
        let json = serde_json::to_string(&ev).map_err(|e| format!("Ser: {}", e))?;
        let entry = self.wal.append_and_return_entry("COMMIT", &json)?;
        self.apply_wal_entry(&entry)
    }
    pub fn record_finalize(&mut self, h: u64, fb: [u8; 32]) -> Result<(), String> {
        let ev = FinalizeEvent {
            height: h,
            finalized_block: Self::encode_hash(fb),
        };
        let json = serde_json::to_string(&ev).map_err(|e| format!("Ser: {}", e))?;
        let entry = self.wal.append_and_return_entry("FINALIZE", &json)?;
        self.apply_wal_entry(&entry)
    }

    pub fn create_snapshot(&self) -> SnapshotCheckpoint {
        SnapshotCheckpoint {
            commit_index: self.commit_rule.commit_index,
            finalized_height: self.commit_rule.finalized_height,
            snapshot_sequence: self.applied_sequences.iter().max().copied().unwrap_or(0),
            snapshot_chain_hash: WriteAheadLog::encode_hash(self.wal.chain_hash()),
            canonical_tip: self
                .fork_choice
                .canonical_tip(&self.dag)
                .map(Self::encode_hash)
                .unwrap_or_else(|| Self::encode_hash(self.dag.genesis_hash)),
            dag_block_count: self.dag.blocks.len(),
            locked_qc_block: self
                .fork_choice
                .locked_qc
                .as_ref()
                .map(|qc| Self::encode_hash(qc.block_hash)),
            locked_qc_round: self.fork_choice.locked_qc.as_ref().map(|qc| qc.round),
            genesis_hash: Self::encode_hash(self.dag.genesis_hash),
            epoch_id: WriteAheadLog::encode_hash(self.wal.epoch_id()),
        }
    }
    pub fn write_snapshot(&self, p: &str) -> Result<(), String> {
        std::fs::write(
            p,
            serde_json::to_string_pretty(&self.create_snapshot())
                .map_err(|e| format!("Ser: {}", e))?,
        )
        .map_err(|e| format!("Write: {}", e))
    }
    pub fn load_snapshot(p: &str) -> Result<SnapshotCheckpoint, String> {
        serde_json::from_str(&std::fs::read_to_string(p).map_err(|e| format!("Read: {}", e))?)
            .map_err(|e| format!("Parse: {}", e))
    }

    pub fn state_digest(&self) -> ConsensusStateDigest {
        ConsensusStateDigest {
            commit_index: self.commit_rule.commit_index,
            finalized_height: self.commit_rule.finalized_height,
            locked_qc_block: self
                .fork_choice
                .locked_qc
                .as_ref()
                .map(|qc| Self::encode_hash(qc.block_hash)),
            locked_qc_round: self.fork_choice.locked_qc.as_ref().map(|qc| qc.round),
            canonical_tip: self
                .fork_choice
                .canonical_tip(&self.dag)
                .map(Self::encode_hash)
                .unwrap_or_else(|| Self::encode_hash(self.dag.genesis_hash)),
            dag_block_count: self.dag.blocks.len(),
            spine_length: self.dag.canonical_spine.len(),
            fork_choice_high_qc_count: self.fork_choice.high_qcs.len(),
            applied_sequence_count: self.applied_sequences.len(),
            wal_segment: self.wal.segment(),
            wal_chain_hash: WriteAheadLog::encode_hash(self.wal.chain_hash()),
        }
    }
    pub fn shutdown(&mut self) -> Result<(), String> {
        self.wal.shutdown()
    }
    fn encode_hash(h: [u8; 32]) -> String {
        h.iter().map(|b| format!("{:02x}", b)).collect()
    }
    fn decode_hash_safe(hex: &str) -> Result<[u8; 32], String> {
        if hex.len() != 64 {
            return Err("Invalid hex".into());
        }
        let mut arr = [0u8; 32];
        for i in 0..32 {
            arr[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
                .map_err(|e| format!("Hex: {}", e))?;
        }
        Ok(arr)
    }
}
