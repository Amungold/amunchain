use crate::pattern::{FailurePattern, FailureSeverity};
use blake3::Hasher;

pub struct FailureOntology {
    patterns: Vec<FailurePattern>,
    chain_hash: [u8; 32],
    version: u64,
}

impl FailureOntology {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            chain_hash: [0u8; 32],
            version: 0,
        }
    }

    pub fn record_pattern(&mut self, pattern: FailurePattern) -> Result<(), &'static str> {
        if self.patterns.iter().any(|p| p.id == pattern.id) {
            return Err("duplicate pattern id - ontology is append-only");
        }
        if self.patterns.len() >= 256 {
            return Err("ontology capacity reached");
        }
        self.patterns.push(pattern);
        self.version += 1;
        self.recompute_chain_hash();
        Ok(())
    }

    fn recompute_chain_hash(&mut self) {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_FAILURE_ONTOLOGY_V1");
        hasher.update(&self.version.to_le_bytes());
        for pattern in &self.patterns {
            hasher.update(&pattern.id.to_le_bytes());
            hasher.update(pattern.name.as_bytes());
            hasher.update(&[pattern.provably_mitigated as u8]);
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
        self.chain_hash = out;
    }

    pub fn posture_summary(&self) -> FailurePosture {
        let total = self.patterns.len();
        let fatal = self
            .patterns
            .iter()
            .filter(|p| p.severity == FailureSeverity::Fatal)
            .count();
        let unmitigated = self.patterns.iter().filter(|p| !p.provably_mitigated).count();

        FailurePosture {
            total_patterns: total,
            fatal_patterns: fatal,
            unmitigated_patterns: unmitigated,
            all_fatal_mitigated: fatal == 0
                || (fatal > 0
                    && self
                        .patterns
                        .iter()
                        .filter(|p| p.severity == FailureSeverity::Fatal)
                        .all(|p| p.provably_mitigated)),
            ontology_version: self.version,
            ontology_hash: self.chain_hash,
        }
    }

    pub fn constitutional_hash(&self) -> [u8; 32] {
        self.chain_hash
    }
}

pub struct FailurePosture {
    pub total_patterns: usize,
    pub fatal_patterns: usize,
    pub unmitigated_patterns: usize,
    pub all_fatal_mitigated: bool,
    pub ontology_version: u64,
    pub ontology_hash: [u8; 32],
}
