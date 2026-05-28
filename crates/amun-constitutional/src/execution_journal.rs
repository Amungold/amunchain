use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntry {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,
    pub entry_id: u64,
    pub entry_hash: [u8; 32],
    pub event_schema_id: Option<u16>,
    pub event_digest: Option<[u8; 32]>,
    pub event_data: Option<Vec<u8>>,
    pub context_hash: [u8; 32],
    pub previous_entry_hash: Option<[u8; 32]>,
    pub sequence_number: u64,
    pub active_boundary_hash: Option<[u8; 32]>,
}

impl ConstitutionalIdentity for JournalEntry {
    fn schema_id(&self) -> u16 {
        self.schema_id
    }
    fn schema_version(&self) -> u16 {
        self.schema_version
    }
    fn constitutional_revision(&self) -> u32 {
        self.constitutional_revision
    }
    fn replay_revision(&self) -> u32 {
        self.replay_revision
    }
}

impl ConstitutionalObject for JournalEntry {
    fn constitutional_hash(&self) -> [u8; 32] {
        let mut h = ConstitutionalHasher::new(crate::hash_domains::DOMAIN_JOURNAL_ENTRY);
        h.update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.entry_id)
            .update_optional_u64(self.event_schema_id.map(|x| x as u64))
            .update_optional_hash(self.event_digest.as_ref())
            .update_bytes(&self.context_hash)
            .update_optional_hash(self.previous_entry_hash.as_ref())
            .update_u64(self.sequence_number)
            .update_optional_hash(self.active_boundary_hash.as_ref());
        h.finalize()
    }
    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x000B || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.entry_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid entry",
            ));
        }
        Ok(())
    }
    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.entry_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.entry_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Hash mismatch",
            ));
        }
        Ok(())
    }
    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.entry_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision",
            ));
        }
        Ok(())
    }
}

impl JournalEntry {
    pub fn new(
        entry_id: u64,
        context_hash: [u8; 32],
        previous: Option<[u8; 32]>,
        sequence_number: u64,
        event_schema_id: Option<u16>,
        event_digest: Option<[u8; 32]>,
        event_data: &[u8],
    ) -> Self {
        let mut e = Self {
            schema_id: 0x000B,
            schema_version: 1,
            constitutional_revision: 1,
            replay_revision: 1,
            entry_id,
            entry_hash: [0; 32],
            event_schema_id,
            event_digest,
            event_data: Some(event_data.to_vec()),
            context_hash,
            previous_entry_hash: previous,
            sequence_number,
            active_boundary_hash: None,
        };
        e.entry_hash = e.constitutional_hash();
        e
    }
    pub fn with_revision(mut self, constitutional: u32, replay: u32) -> Self {
        self.constitutional_revision = constitutional;
        self.replay_revision = replay;
        self.entry_hash = self.constitutional_hash();
        self
    }
    pub fn with_boundary(mut self, h: [u8; 32]) -> Self {
        self.active_boundary_hash = Some(h);
        self.entry_hash = self.constitutional_hash();
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionJournal {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,
    pub journal_id: u64,
    pub journal_hash: [u8; 32],
    pub context_hash: [u8; 32],
    pub entry_count: u64,
    pub last_entry_hash: Option<[u8; 32]>,
    pub entries_root: Option<[u8; 32]>,
}

impl ConstitutionalIdentity for ExecutionJournal {
    fn schema_id(&self) -> u16 {
        self.schema_id
    }
    fn schema_version(&self) -> u16 {
        self.schema_version
    }
    fn constitutional_revision(&self) -> u32 {
        self.constitutional_revision
    }
    fn replay_revision(&self) -> u32 {
        self.replay_revision
    }
}

impl ConstitutionalObject for ExecutionJournal {
    fn constitutional_hash(&self) -> [u8; 32] {
        let mut h = ConstitutionalHasher::new(crate::hash_domains::DOMAIN_EXECUTION_JOURNAL);
        h.update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.journal_id)
            .update_bytes(&self.context_hash)
            .update_u64(self.entry_count)
            .update_optional_hash(self.last_entry_hash.as_ref())
            .update_optional_hash(self.entries_root.as_ref());
        h.finalize()
    }
    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x0008 || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.journal_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid journal",
            ));
        }
        Ok(())
    }
    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.journal_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.journal_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Hash mismatch",
            ));
        }
        Ok(())
    }
    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.journal_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision",
            ));
        }
        Ok(())
    }
}

impl ExecutionJournal {
    pub fn new(journal_id: u64, context_hash: [u8; 32]) -> Self {
        let mut j = Self {
            schema_id: 0x0008,
            schema_version: 1,
            constitutional_revision: 1,
            replay_revision: 1,
            journal_id,
            journal_hash: [0; 32],
            context_hash,
            entry_count: 0,
            last_entry_hash: None,
            entries_root: None,
        };
        j.journal_hash = j.constitutional_hash();
        j
    }

    pub fn append(&self, entry: &JournalEntry) -> Result<Self, ConstitutionalFailure> {
        entry.verify()?;
        if entry.constitutional_revision != self.constitutional_revision {
            return Err(ConstitutionalFailure::new(
                self.journal_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::REPLAY,
                severity::FATAL_FAILURE,
                "Constitutional revision mismatch",
            ));
        }
        if entry.replay_revision != self.replay_revision {
            return Err(ConstitutionalFailure::new(
                self.journal_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::REPLAY,
                severity::FATAL_FAILURE,
                "Replay revision mismatch",
            ));
        }
        match (self.last_entry_hash, entry.previous_entry_hash) {
            (None, None) => {}
            (Some(last), Some(prev)) if last == prev => {}
            _ => {
                return Err(ConstitutionalFailure::new(
                    self.journal_id,
                    failure_type::REPLAY_DIVERGENCE,
                    failure_domain::REPLAY,
                    severity::HARD_FAILURE,
                    "Linearity violation",
                ))
            }
        }
        if entry.sequence_number != self.entry_count {
            return Err(ConstitutionalFailure::new(
                self.journal_id,
                failure_type::REPLAY_DIVERGENCE,
                failure_domain::REPLAY,
                severity::HARD_FAILURE,
                "Sequence gap",
            ));
        }
        let mut updated = self.clone();
        updated.entry_count += 1;
        updated.last_entry_hash = Some(entry.entry_hash);
        updated.journal_hash = updated.constitutional_hash();
        Ok(updated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_context::ExecutionContext;
    fn mc() -> ExecutionContext {
        ExecutionContext::new(1, [0xAB; 32], 0)
    }
    fn me(ctx: &ExecutionContext, id: u64, seq: u64, prev: Option<[u8; 32]>) -> JournalEntry {
        JournalEntry::new(
            id,
            ctx.context_hash,
            prev,
            seq,
            Some(1),
            Some([0xAA; 32]),
            b"data",
        )
    }
    #[test]
    fn t_entry_verify() {
        assert!(me(&mc(), 1, 0, None).verify().is_ok());
    }
    #[test]
    fn t_append() {
        let ctx = mc();
        let j = ExecutionJournal::new(1, ctx.context_hash);
        let e1 = me(&ctx, 1, 0, None);
        let j1 = j.append(&e1).unwrap();
        assert_eq!(j1.entry_count, 1);
        let e2 = me(&ctx, 2, 1, Some(e1.entry_hash));
        let j2 = j1.append(&e2).unwrap();
        assert_eq!(j2.entry_count, 2);
    }
    #[test]
    fn t_revision_mismatch() {
        let ctx = mc();
        let j = ExecutionJournal::new(1, ctx.context_hash);
        let e = me(&ctx, 1, 0, None).with_revision(2, 1);
        assert!(j.append(&e).is_err());
    }
    #[test]
    fn t_seq_gap() {
        let ctx = mc();
        let j = ExecutionJournal::new(1, ctx.context_hash);
        let e = me(&ctx, 1, 5, None);
        assert!(j.append(&e).is_err());
    }
    #[test]
    fn t_chain_break() {
        let ctx = mc();
        let j = ExecutionJournal::new(1, ctx.context_hash);
        let e1 = me(&ctx, 1, 0, None);
        let j1 = j.append(&e1).unwrap();
        let e2 = me(&ctx, 2, 1, Some([0xFF; 32]));
        assert!(j1.append(&e2).is_err());
    }
}
