#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityClass {
    Hostile,
    MigrationRequired,
    ReadOnly,
    SnapshotCompatible,
    ReplayCompatible,
    Full,
    Extinct,
}

impl CompatibilityClass {
    pub fn canonical_tag(&self) -> u8 {
        match self {
            CompatibilityClass::Hostile => 0x00,
            CompatibilityClass::MigrationRequired => 0x01,
            CompatibilityClass::ReadOnly => 0x02,
            CompatibilityClass::SnapshotCompatible => 0x03,
            CompatibilityClass::ReplayCompatible => 0x04,
            CompatibilityClass::Full => 0x05,
            CompatibilityClass::Extinct => 0xFF,
        }
    }

    pub fn rank(&self) -> u8 {
        match self {
            CompatibilityClass::Hostile => 0,
            CompatibilityClass::Extinct => 0,
            CompatibilityClass::MigrationRequired => 1,
            CompatibilityClass::ReadOnly => 2,
            CompatibilityClass::SnapshotCompatible => 3,
            CompatibilityClass::ReplayCompatible => 4,
            CompatibilityClass::Full => 5,
        }
    }

    pub fn min_class(a: CompatibilityClass, b: CompatibilityClass) -> CompatibilityClass {
        if a.rank() <= b.rank() {
            a
        } else {
            b
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityDeclaration {
    pub source: [u8; 32],
    pub target: [u8; 32],
    pub class: CompatibilityClass,
    pub requires_migration: bool,
    pub migration_rules_hash: Option<[u8; 32]>,
    pub declared_at_epoch: u64,
}
