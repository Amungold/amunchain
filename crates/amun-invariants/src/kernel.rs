#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvariantSeverity {
    Fatal,
    Critical,
    Degraded,
}

#[derive(Debug, Clone)]
pub struct InvariantDef {
    pub id: u32,
    pub name: &'static str,
    pub specification: &'static str,
    pub severity: InvariantSeverity,
    pub requires_formal_proof: bool,
    pub requires_runtime_check: bool,
    pub requires_replay_verification: bool,
    pub requires_attack_testing: bool,
}

pub struct IrreducibleInvariants;

impl IrreducibleInvariants {
    pub const SINGLE_TRUTH: InvariantDef = InvariantDef {
        id: 1,
        name: "Single Truth",
        specification: "forall h: |{state_roots_at_height(h)}| = 1",
        severity: InvariantSeverity::Fatal,
        requires_formal_proof: true,
        requires_runtime_check: true,
        requires_replay_verification: true,
        requires_attack_testing: false,
    };

    pub const SINGLE_FINALITY: InvariantDef = InvariantDef {
        id: 2,
        name: "Single Finality",
        specification: "forall h: |{finalized_blocks_at_height(h)}| <= 1 under f < n/3",
        severity: InvariantSeverity::Fatal,
        requires_formal_proof: true,
        requires_runtime_check: true,
        requires_replay_verification: false,
        requires_attack_testing: true,
    };

    pub const EVENTUAL_PROGRESS: InvariantDef = InvariantDef {
        id: 3,
        name: "Eventual Progress",
        specification: "diamond(new_block_finalized) under eventual synchrony",
        severity: InvariantSeverity::Critical,
        requires_formal_proof: true,
        requires_runtime_check: true,
        requires_replay_verification: false,
        requires_attack_testing: true,
    };

    pub const FAILURE_MEMORY: InvariantDef = InvariantDef {
        id: 4,
        name: "Failure Memory",
        specification: "forall f in known_failures: mitigation(f) != empty",
        severity: InvariantSeverity::Degraded,
        requires_formal_proof: false,
        requires_runtime_check: false,
        requires_replay_verification: false,
        requires_attack_testing: false,
    };

    pub fn all() -> [InvariantDef; 4] {
        [
            Self::SINGLE_TRUTH,
            Self::SINGLE_FINALITY,
            Self::EVENTUAL_PROGRESS,
            Self::FAILURE_MEMORY,
        ]
    }

    pub fn kernel_hash() -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_INVARIANT_KERNEL_V1");
        for inv in Self::all().iter() {
            hasher.update(&inv.id.to_le_bytes());
            hasher.update(inv.specification.as_bytes());
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
        out
    }
}
