use crate::log::{MessageEntry, MessageLog};
use crate::trace::StateTrace;
use amun_chain_position::{ChainPosition, EpochSeal};
use amun_protocol_event::ProtocolEvent;
use amun_snapshot_constitution::{export_snapshot, import_snapshot, CanonicalSnapshot};
use amun_state_transition::{StateMachine, StorageJournal, TransitionOutput};
use std::collections::BTreeSet;

const EXECUTION_VERSION: u64 = 1;

pub struct TruthEngine {
    message_log: MessageLog,
    state_trace: StateTrace,
    genesis_root: [u8; 32],
    live_state: StateMachine,
    live_journal: StorageJournal,
    current_position: ChainPosition,
    epoch_seals: Vec<EpochSeal>,
    sealed_epochs: BTreeSet<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayError {
    DivergenceAt {
        position: ChainPosition,
        expected: [u8; 32],
        actual: [u8; 32],
    },
    InvalidTransition {
        at_position: ChainPosition,
    },
    ChunkReadError {
        position: ChainPosition,
    },
    JournalContinuityBroken {
        at_position: ChainPosition,
    },
    JournalAppendError {
        position: ChainPosition,
    },
    PositionOverflow,
    EpochAlreadySealed {
        epoch: u64,
    },
    PositionMismatch {
        expected: ChainPosition,
        actual: ChainPosition,
    },
}

impl TruthEngine {
    pub fn new(genesis_root: [u8; 32]) -> Self {
        Self {
            message_log: MessageLog::new(),
            state_trace: StateTrace::new(),
            genesis_root,
            live_state: StateMachine::new(genesis_root, EXECUTION_VERSION),
            live_journal: StorageJournal::new(genesis_root),
            current_position: ChainPosition::genesis(),
            epoch_seals: Vec::new(),
            sealed_epochs: BTreeSet::new(),
        }
    }

    fn prepare(&self, payload: &[u8]) -> Result<(ChainPosition, Vec<u8>), &'static str> {
        if self.sealed_epochs.contains(&self.current_position.epoch) {
            return Err("epoch sealed");
        }
        let np = self.current_position.next_sequence().ok_or("overflow")?;
        Ok((np, payload.to_vec()))
    }

    fn commit_transition(
        &mut self,
        position: ChainPosition,
        payload: &[u8],
        output: TransitionOutput,
    ) -> Result<(), &'static str> {
        self.state_trace
            .append(
                output.receipt.from_root,
                output.new_root,
                output.receipt.hash(),
            )
            .map_err(|_| "trace failed")?;
        self.live_journal
            .append(output.receipt.clone())
            .map_err(|_| "journal failed")?;
        self.message_log.append(position, payload)?;
        self.current_position = position;
        self.live_state
            .apply_overlay(output.overlay, output.new_root);
        Ok(())
    }

    pub fn execute_live(
        &mut self,
        tx_data: &[u8],
        gas_limit: u64,
    ) -> Result<([u8; 32], ProtocolEvent), &'static str> {
        let (position, payload) = self.prepare(tx_data)?;
        let (receipt, overlay) = StateMachine::execute_transition(
            &self.live_state.state,
            self.live_state.execution_version,
            self.live_state.epoch_seal_hash,
            position,
            &payload,
            gas_limit,
        );
        let new_root = receipt.to_root;
        let output = TransitionOutput {
            receipt,
            overlay,
            new_root,
        };
        self.commit_transition(position, &payload, output)?;
        let event = ProtocolEvent::ExecuteTransaction {
            position: self.current_position,
            payload: tx_data.to_vec(),
            expected_root: new_root,
        };
        Ok((new_root, event))
    }

    pub fn seal_and_advance_epoch(&mut self) -> Result<(EpochSeal, ProtocolEvent), &'static str> {
        let cur = self.current_position.epoch;
        let sp = self.current_position.next_sequence().ok_or("overflow")?;
        let seal = EpochSeal::new(
            cur,
            self.live_state.live_root(),
            self.live_journal.chain_hash(),
            self.compute_chain_root_until(self.current_position)
                .map_err(|_| "replay failed")?,
        );
        self.message_log.append_seal(sp, seal.seal_hash)?;
        self.live_state.set_epoch_seal(seal.seal_hash);
        self.sealed_epochs.insert(cur);
        self.epoch_seals.push(seal);
        self.current_position = sp.next_epoch().ok_or("overflow")?;
        self.live_journal.set_epoch(self.current_position.epoch);
        let event = ProtocolEvent::SealEpoch {
            position: sp,
            epoch: cur,
            seal_hash: seal.seal_hash,
            expected_root: self.live_state.live_root(),
        };
        Ok((seal, event))
    }

    pub fn apply_event(&mut self, event: &ProtocolEvent) -> Result<[u8; 32], &'static str> {
        let expected_next = self
            .current_position
            .next_sequence()
            .ok_or("position overflow")?;
        if event.position() != expected_next {
            return Err("position mismatch");
        }
        match event {
            ProtocolEvent::ExecuteTransaction {
                payload,
                expected_root,
                ..
            } => {
                let (position, payload) = self.prepare(payload)?;
                let (receipt, overlay) = StateMachine::execute_transition(
                    &self.live_state.state,
                    self.live_state.execution_version,
                    self.live_state.epoch_seal_hash,
                    position,
                    &payload,
                    1_000_000,
                );
                let new_root = receipt.to_root;
                if new_root != *expected_root {
                    return Err("execute: root mismatch");
                }
                let output = TransitionOutput {
                    receipt,
                    overlay,
                    new_root,
                };
                self.commit_transition(position, &payload, output)?;
                Ok(new_root)
            }
            ProtocolEvent::SealEpoch {
                position,
                epoch,
                seal_hash,
                expected_root,
            } => {
                let sp = *position;
                let seal = EpochSeal {
                    epoch: *epoch,
                    seal_hash: *seal_hash,
                    epoch_root: self.live_state.live_root(),
                    journal_root: self.live_journal.chain_hash(),
                    replay_root: self
                        .compute_chain_root_until(self.current_position)
                        .map_err(|_| "replay failed")?,
                };
                self.message_log.append_seal(sp, seal.seal_hash)?;
                self.live_state.set_epoch_seal(seal.seal_hash);
                self.sealed_epochs.insert(*epoch);
                self.epoch_seals.push(seal);
                self.current_position = sp.next_epoch().ok_or("overflow")?;
                self.live_journal.set_epoch(self.current_position.epoch);
                if self.live_state.live_root() != *expected_root {
                    return Err("seal: root mismatch");
                }
                Ok(self.live_state.live_root())
            }
            ProtocolEvent::CreateSnapshot { expected_root, .. } => Ok(*expected_root),
        }
    }

    pub fn export_snapshot(
        &self,
    ) -> Result<CanonicalSnapshot, amun_snapshot_constitution::SnapshotError> {
        let sealed: Vec<u64> = self.sealed_epochs.iter().copied().collect();
        export_snapshot(
            &self.live_state,
            self.current_position,
            self.current_position.epoch,
            self.genesis_root,
            self.live_state.epoch_seal_hash,
            self.live_state.execution_version,
            sealed,
        )
    }

    pub fn import_snapshot(&mut self, snapshot: &CanonicalSnapshot) -> Result<(), &'static str> {
        // Constitutional: reject foreign-chain snapshots
        if snapshot.context.genesis_root != self.genesis_root {
            return Err("snapshot genesis mismatch - foreign chain rejected");
        }

        let ctx = import_snapshot(&mut self.live_state, snapshot)?;
        self.current_position = ctx.current_position;
        self.live_state.epoch_seal_hash = ctx.epoch_seal_hash;
        self.sealed_epochs.clear();
        for epoch in &ctx.sealed_epochs {
            self.sealed_epochs.insert(*epoch);
        }
        self.live_journal.set_epoch(ctx.current_epoch);
        Ok(())
    }

    pub fn compute_chain_root_until(&self, until: ChainPosition) -> Result<[u8; 32], ReplayError> {
        if until.is_genesis() {
            return Ok(self.genesis_root);
        }
        let mut sm = StateMachine::new(self.genesis_root, EXECUTION_VERSION);
        for entry in self.message_log.entries() {
            if entry.position() > until {
                break;
            }
            match &entry.transcript {
                crate::log::TranscriptEntry::EpochSeal { seal_hash, .. } => {
                    sm.set_epoch_seal(*seal_hash)
                }
                crate::log::TranscriptEntry::Transaction { .. } => {
                    let p = self
                        .message_log
                        .payload(entry)
                        .ok_or(ReplayError::ChunkReadError {
                            position: entry.position(),
                        })?;
                    let (r, o) = sm.execute(entry.position(), p, 1_000_000);
                    sm.apply_overlay(o, r.to_root);
                }
            }
        }
        Ok(sm.live_root())
    }

    pub fn compute_chain_root(&self, target_tx_count: u64) -> Result<[u8; 32], ReplayError> {
        if target_tx_count == 0 {
            return Ok(self.genesis_root);
        }
        let mut sm = StateMachine::new(self.genesis_root, EXECUTION_VERSION);
        let mut tc: u64 = 0;
        for entry in self.message_log.entries() {
            match &entry.transcript {
                crate::log::TranscriptEntry::EpochSeal { seal_hash, .. } => {
                    sm.set_epoch_seal(*seal_hash)
                }
                crate::log::TranscriptEntry::Transaction { .. } => {
                    let p = self
                        .message_log
                        .payload(entry)
                        .ok_or(ReplayError::ChunkReadError {
                            position: entry.position(),
                        })?;
                    let (r, o) = sm.execute(entry.position(), p, 1_000_000);
                    sm.apply_overlay(o, r.to_root);
                    tc += 1;
                    if tc >= target_tx_count {
                        break;
                    }
                }
            }
        }
        Ok(sm.live_root())
    }

    pub fn record_message(&mut self, payload: &[u8]) -> Result<MessageEntry, &'static str> {
        let (position, payload) = self.prepare(payload)?;
        let entry = self.message_log.append(position, &payload)?;
        self.current_position = position;
        Ok(entry)
    }

    pub fn live_root(&self) -> [u8; 32] {
        self.live_state.live_root()
    }
    pub fn genesis_root(&self) -> [u8; 32] {
        self.genesis_root
    }
    pub fn current_position(&self) -> ChainPosition {
        self.current_position
    }
    pub fn epoch_seals(&self) -> &[EpochSeal] {
        &self.epoch_seals
    }
    pub fn live_journal(&self) -> &StorageJournal {
        &self.live_journal
    }
}
